// operator-server — the `demo:operator` entrypoint. Wires the pure operator (`./operator`) to a live
// stack: the seeded demo-config (RPC endpoints, mints, personas), the environment-specific keeper and
// mint-authority keypairs, the environment's SOL funder, the listener's proof endpoint and the
// relayer's key material. It binds loopback: the dapp dev server proxies the browser to it and adds
// the boot capability; `tailscale serve` may front it for direct callers on the tailnet.
//
// The SPL instructions are hand-built with `@solana/kit` primitives on purpose: the test-suite
// carries no `@solana-program/token` dependency; they come from `../src/solana/spl` (shared with
// the seed). The keeper-side vault logic is the dapp's own operator modules, imported directly.
//
// This process holds a live validator connection and is exercised by the `solana-e2e` workflow's
// demo phase (which funds the deposit-arc persona through it) and the browser-reality checks. The
// request handler it serves is unit-tested in `operator.test.ts`.

import fs from "node:fs/promises";

import {
  address,
  appendTransactionMessageInstructions,
  assertIsTransactionWithBlockhashLifetime,
  createKeyPairSignerFromBytes,
  createSolanaRpc,
  createSolanaRpcSubscriptions,
  createTransactionMessage,
  getSignatureFromTransaction,
  sendAndConfirmTransactionFactory,
  setTransactionMessageFeePayerSigner,
  setTransactionMessageLifetimeUsingBlockhash,
  signTransactionMessageWithSigners,
  type Address,
} from "@solana/kit";

import { lookupTableForBatch, prepareNextBatch } from "@demo-dapp/batchProvisioning";
import { claimBatchPayout } from "@demo-dapp/claim";
import { parseRuntimeDemoConfig } from "@demo-dapp/demoConfig";
import { harvestDemoVault, readDemoVaultMetrics, type UnderlyingMinter } from "@demo-dapp/harvestOperator";
import { dispatchVaultBatch, settleVaultBatch, type DemoOperatorSession } from "@demo-dapp/settlement";
import { openProvisioning } from "../e2e/harness/solana/provisioning";
import { DEMO_OPERATOR_PORT, solanaBatchLookupTablesPath } from "../src/layout";
import { LOCAL_SOLANA_ENDPOINTS } from "../src/solana/endpoints";
import {
  associatedTokenAddress,
  createIdempotentAtaInstruction,
  mintToInstruction,
  SPL_TOKEN_PROGRAM_ADDRESS,
} from "../src/solana/spl";
import { readDemoAllowedOriginFromEnv, readDemoAuthorizationFromEnv } from "./authorization";
import { resolveDemoConfigPath } from "./config";
import { createEncryptionKeyMaterial } from "./encryptionKeyMaterial";
import { saveRecoveryKey, mirrorRecoveryKeys } from "../src/solana/recovery";
import { demoKeypairs, loadDemoEnv } from "./loadDemoEnv";
import { createOperator, TAILSCALE_LOGIN_HEADER } from "./operator";

/** Tailscale logins (comma-separated) the operator accepts through the identity header. */
const TAILSCALE_LOGINS_ENV = "DEMO_OPERATOR_TAILSCALE_LOGINS";

/** Builds the minter that mints mock USDC to a recipient's ATA on the live cluster. */
const buildUsdcMinter = async (options: {
  readonly rpcUrl: string;
  readonly wsUrl: string;
  readonly mint: Address;
  readonly mintAuthorityKeypairPath: string;
}): Promise<UnderlyingMinter> => {
  const rpc = createSolanaRpc(options.rpcUrl);
  const rpcSubscriptions = createSolanaRpcSubscriptions(options.wsUrl);
  const sendAndConfirm = sendAndConfirmTransactionFactory({ rpc, rpcSubscriptions });
  const authority = await loadSigner(options.mintAuthorityKeypairPath);

  return async (recipient: Address, baseUnits: bigint): Promise<string> => {
    const ata = await associatedTokenAddress(recipient, options.mint, SPL_TOKEN_PROGRAM_ADDRESS);
    const { value: latestBlockhash } = await rpc.getLatestBlockhash().send();
    const base = setTransactionMessageFeePayerSigner(authority, createTransactionMessage({ version: 0 }));
    const withLifetime = setTransactionMessageLifetimeUsingBlockhash(latestBlockhash, base);
    const message = appendTransactionMessageInstructions(
      [
        createIdempotentAtaInstruction({ payer: authority, ata, owner: recipient, mint: options.mint }),
        mintToInstruction({ mint: options.mint, destination: ata, authority, baseUnits }),
      ],
      withLifetime,
    );
    const signedTransaction = await signTransactionMessageWithSigners(message);
    // The message was given a blockhash lifetime above; narrow the signed tx so the blockhash-based
    // send factory accepts it (kit's signer returns the generic lifetime union).
    assertIsTransactionWithBlockhashLifetime(signedTransaction);
    await sendAndConfirm(signedTransaction, { commitment: "confirmed" });
    return getSignatureFromTransaction(signedTransaction);
  };
};

const loadSigner = async (keypairPath: string) =>
  createKeyPairSignerFromBytes(Uint8Array.from(JSON.parse(await fs.readFile(keypairPath, "utf8")) as number[]));

const requiredEnv = (name: string): string => {
  const value = process.env[name];
  if (!value) throw new Error(`${name} is required`);
  return value;
};

const main = async (): Promise<void> => {
  const authorization = await readDemoAuthorizationFromEnv();
  const allowedOrigin = readDemoAllowedOriginFromEnv();
  const relayerUrl = process.env.DEMO_RELAYER_URL ?? LOCAL_SOLANA_ENDPOINTS.relayer;
  const proofService = { url: requiredEnv("DEMO_PROOF_URL"), apiKey: requiredEnv("DEMO_PROOF_API_KEY") };
  const tailscaleLogins = (process.env[TAILSCALE_LOGINS_ENV] ?? "")
    .split(",")
    .map((login) => login.trim())
    .filter((login) => login.length > 0);

  // The seeded network decides how SOL is dripped: airdrop on a local validator, transfer from the
  // deployer wallet on devnet.
  const configPath = resolveDemoConfigPath();
  const { env, config: seeded } = await loadDemoEnv(configPath);
  const relayerApiKey = process.env.DEMO_RELAYER_API_KEY ?? (env.network === "localnet" ? "local" : requiredEnv("DEMO_RELAYER_API_KEY"));
  const { fundSol } = await openProvisioning(env);
  const mintAuthority = await loadSigner(demoKeypairs(env).mintAuthority);
  const mintUnderlying = await buildUsdcMinter({
    rpcUrl: seeded.rpcUrl,
    wsUrl: seeded.wsUrl,
    // Mock USDC is the deposit underlying (`mints.joinUnderlying`); the faucet drips exactly that.
    mint: address(seeded.mints.joinUnderlying),
    mintAuthorityKeypairPath: demoKeypairs(env).mintAuthority,
  });
  const keeper = await loadSigner(demoKeypairs(env).keeper);
  const mintUsdc: UnderlyingMinter = async (recipient, amount) => {
    await fundSol(mintAuthority.address, 0.2);
    return mintUnderlying(recipient, amount);
  };

  // One configuration per process: restart the operator after reseeding.
  const readRuntimeConfig = async (): Promise<Record<string, unknown>> =>
    seeded as unknown as Record<string, unknown>;
  const session = async (): Promise<DemoOperatorSession> => {
    const config = parseRuntimeDemoConfig(await readRuntimeConfig(), authorization.bootId);
    if (keeper.address !== config.personas.keeper) {
      throw new Error(`keeper signer ${keeper.address} does not match seeded keeper ${config.personas.keeper}`);
    }
    await fundSol(keeper.address, 0.2);
    return { config, keeper, proofService, relayerApiKey };
  };
  const encryptionKey = createEncryptionKeyMaterial({ relayerUrl, apiKey: relayerApiKey, network: seeded.network });

  const handler = createOperator({
    authorization,
    allowedOrigin,
    tailscaleLogins,
    actions: {
      registerBurner: async (bytes) => {
        const wallet = await createKeyPairSignerFromBytes(bytes);
        if (env.network === "devnet") {
          try { await saveRecoveryKey(`browser-${wallet.address}`, bytes); }
          catch (error) { if ((error as NodeJS.ErrnoException).code !== "EEXIST") throw error; }
          await mirrorRecoveryKeys();
        }
        return wallet.address;
      },
      fundSol,
      mintUsdc,
      // The page reaches the relayer through its own origin's proxy, never a raw relayer URL.
      readConfig: async () => ({
        ...(await readRuntimeConfig()),
        relayerUrl: `${allowedOrigin}/api/relayer`,
        demoBootId: authorization.bootId,
      }),
      encryptionKeyFingerprint: encryptionKey.fingerprint,
      encryptionKey: encryptionKey.key,
      vaultMetrics: async () => readDemoVaultMetrics((await session()).config),
      prepareBatch: async (direction) => {
        const { config } = await session();
        return prepareNextBatch(config, keeper, direction, solanaBatchLookupTablesPath);
      },
      runOperator: async (request) => {
        const current = await session();
        const { position, direction } = request;
        if (request.action === "claim") return claimBatchPayout(current, position, direction, request.user);
        if (request.action === "dispatch") return dispatchVaultBatch(current, position, direction);
        const lookupTable = await lookupTableForBatch(current.config, direction, position, solanaBatchLookupTablesPath);
        return settleVaultBatch(current, position, direction, lookupTable);
      },
      harvest: async () => harvestDemoVault((await session()).config, keeper, mintUsdc),
    },
  });

  // The lifecycle (or preview.env) names this service's URL; the port comes from it, the bind stays
  // loopback: the dapp proxy and `tailscale serve` are the only intended callers.
  const listen = new URL(process.env.DEMO_OPERATOR_URL ?? LOCAL_SOLANA_ENDPOINTS.demoOperator);
  const server = Bun.serve({ port: Number(listen.port || DEMO_OPERATOR_PORT), hostname: "127.0.0.1", fetch: handler });
  console.log(
    `demo operator listening on http://127.0.0.1:${server.port} for ${allowedOrigin}` +
      ` (mock USDC mint ${seeded.mints.joinUnderlying}` +
      (tailscaleLogins.length > 0 ? `, ${TAILSCALE_LOGIN_HEADER} accepted for ${tailscaleLogins.join(", ")})` : ")"),
  );
};

await main();
