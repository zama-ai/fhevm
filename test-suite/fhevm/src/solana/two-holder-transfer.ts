import fs from "node:fs/promises";
import os from "node:os";
import path from "node:path";

import { address, getAddressEncoder } from "@solana/kit";

import {
  REPO_ROOT,
  SOLANA_ACL_PROGRAM,
  SOLANA_DEFAULT_USER_DECRYPT_CONTEXT,
} from "../layout";
import { runSolanaCurrentUserDecrypt } from "./current-user-decrypt";
import {
  createConfidentialMint,
  createProvisioningContext,
  createSplMint,
  generateSolanaKeypair,
  initializeConfidentialTokenAccount,
  mintSplTo,
  readTokenBalanceState,
  wrapUnderlying,
  type BalanceState,
} from "./provision";
import { waitForSnsCommit } from "./sns";
import { run } from "../utils/process";

export type { BalanceState };

export const SOLANA_TWO_HOLDER_TRANSFER_PROFILE = "solana-two-holder-transfer";
export const SOLANA_TWO_HOLDER_TRANSFER_DESCRIPTION =
  "Transfer an SDK-encrypted euint64 between two real Solana holders and decrypt both latest balances.";

const RPC_URL = "http://127.0.0.1:8899";
const WS_URL = "ws://127.0.0.1:8900";
const RELAYER_URL = "http://127.0.0.1:3000";
const GATEWAY_RPC_URL = "http://127.0.0.1:8546";
const ACL_PROGRAM = SOLANA_ACL_PROGRAM;
const DEFAULT_USER_DECRYPT_CONTEXT = SOLANA_DEFAULT_USER_DECRYPT_CONTEXT;
const SDK_WORKER = path.join(REPO_ROOT, "test-suite/fhevm/solana-two-holder-transfer.ts");
const CLI_DIR = path.join(REPO_ROOT, "test-suite/fhevm");
const SIGNATURE = /^[1-9A-HJ-NP-Za-km-z]{87,88}$/;
const BYTES32 = /^0x[0-9a-f]{64}$/i;

export type Holder = { owner: string; keypairPath: string; secretKey: string };
export type TwoHolderScenario = {
  mint: string;
  computeSigner: string;
  alice: Holder;
  bob: Holder;
};

/**
 * The stack endpoints and identities the real transfer arc binds to. Defaults reproduce the local
 * clean-e2e stack (the constants above); the e2e harness injects these from `loadEnv()` so the same
 * arc can target another environment without editing this file.
 */
export type TwoHolderConfig = {
  readonly rpcUrl: string;
  readonly wsUrl: string;
  readonly relayerUrl: string;
  /** Gateway RPC the decrypt step resolves the KMS trust anchor from. */
  readonly gatewayRpcUrl: string;
  readonly aclProgram: string;
  readonly userDecryptContext: string;
};

export type TwoHolderDependencies = {
  provision(): Promise<TwoHolderScenario>;
  readBalance(scenario: TwoHolderScenario, holder: Holder): Promise<BalanceState>;
  waitForHandle(handle: string): Promise<void>;
  transfer(scenario: TwoHolderScenario, alice: BalanceState, bob: BalanceState): Promise<void>;
  decrypt(scenario: TwoHolderScenario, holder: Holder, state: BalanceState, expected: bigint): Promise<bigint>;
  cleanup(scenario: TwoHolderScenario | undefined): Promise<void>;
};

const parseJsonLine = (output: string): unknown => {
  const line = output.trim().split(/\r?\n/).at(-1);
  if (!line) throw new Error("command did not emit final-line JSON");
  return JSON.parse(line) as unknown;
};

const isRecord = (value: unknown): value is Record<string, unknown> =>
  typeof value === "object" && value !== null && !Array.isArray(value);

const hasExactKeys = (value: Record<string, unknown>, keys: readonly string[]) =>
  Object.keys(value).sort().join(",") === [...keys].sort().join(",");

export const parseTransferWorkerResult = (output: string): void => {
  const value = parseJsonLine(output);
  if (
    !isRecord(value) ||
    !hasExactKeys(value, ["version", "signature", "inputHandle"]) ||
    value.version !== 1 ||
    typeof value.signature !== "string" ||
    !SIGNATURE.test(value.signature) ||
    typeof value.inputHandle !== "string" ||
    !BYTES32.test(value.inputHandle)
  ) {
    throw new Error("SDK transfer worker returned malformed versioned JSON");
  }
};

export const solanaUserDecryptContext = (
  decimal = process.env.SOLANA_UD_CONTEXT_ID ?? DEFAULT_USER_DECRYPT_CONTEXT,
): string => {
  if (!/^\d+$/.test(decimal)) throw new Error("SOLANA_UD_CONTEXT_ID must be an unsigned decimal integer");
  const value = BigInt(decimal);
  if (value >= 1n << 256n) throw new Error("SOLANA_UD_CONTEXT_ID must fit in 32 bytes");
  return `0x${value.toString(16).padStart(64, "0")}`;
};

const resolveConfig = (config: Partial<TwoHolderConfig>): TwoHolderConfig => ({
  rpcUrl: config.rpcUrl ?? RPC_URL,
  wsUrl: config.wsUrl ?? WS_URL,
  relayerUrl: config.relayerUrl ?? RELAYER_URL,
  gatewayRpcUrl: config.gatewayRpcUrl ?? GATEWAY_RPC_URL,
  aclProgram: config.aclProgram ?? ACL_PROGRAM,
  userDecryptContext: config.userDecryptContext ?? solanaUserDecryptContext(),
});

export const createRealTwoHolderDependencies = (config: Partial<TwoHolderConfig> = {}): TwoHolderDependencies => {
  const cfg = resolveConfig(config);
  const context = createProvisioningContext(cfg.rpcUrl, cfg.wsUrl);
  let scenarioDir: string | undefined;
  return {
    async provision() {
      scenarioDir = await fs.mkdtemp(path.join(os.tmpdir(), "fhevm-solana-two-holder-"));
      // Both holders are fresh keypairs written under the scenario dir: the SDK transfer worker
      // subprocess loads Alice's file, and the user-decrypt secret is the 32-byte seed.
      const createHolder = async (name: string) => {
        const { signer, bytes } = await generateSolanaKeypair();
        const keypairPath = path.join(scenarioDir!, `${name}.json`);
        await fs.writeFile(keypairPath, JSON.stringify(Array.from(bytes)));
        const holder: Holder = {
          owner: signer.address,
          keypairPath,
          secretKey: `0x${Buffer.from(bytes.subarray(0, 32)).toString("hex")}`,
        };
        return { signer, holder };
      };
      const alice = await createHolder("alice");
      const bob = await createHolder("bob");
      // Alice pays every provisioning rent + fee (mints, escrow, wrap, later the transfer itself);
      // Bob only pays his own confidential token account.
      await context.airdropSol(alice.signer.address, 10n);
      await context.airdropSol(bob.signer.address, 5n);

      // The public underlying: a fresh 9-decimals SPL mint with Alice as mint authority, funded
      // well past the 1000 base units the wrap below rotates into her confidential balance.
      const underlyingMint = await createSplMint(context, { authority: alice.signer, decimals: 9 });
      await mintSplTo(context, {
        authority: alice.signer,
        mint: underlyingMint,
        recipient: alice.signer.address,
        baseUnits: 1_000_000n,
      });
      const { mint, computeSigner } = await createConfidentialMint(context, {
        authority: alice.signer,
        underlyingMint,
      });
      await initializeConfidentialTokenAccount(context, { payer: alice.signer, owner: alice.signer.address, mint });
      await wrapUnderlying(context, { owner: alice.signer, mint, underlyingMint, amount: 1000n });
      await initializeConfidentialTokenAccount(context, { payer: bob.signer, owner: bob.signer.address, mint });
      return { mint, computeSigner, alice: alice.holder, bob: bob.holder };
    },
    async readBalance(scenario, holder) {
      return readTokenBalanceState(context, { mint: address(scenario.mint), owner: address(holder.owner) });
    },
    async waitForHandle(handle) {
      await waitForSnsCommit(handle);
    },
    async transfer(scenario, alice, bob) {
      if (alice.chainId !== bob.chainId) throw new Error("Alice and Bob balance handles disagree on chain id");
      // bun, not node: the worker imports the demo dapp's vault module (TS sources resolved
      // through tsconfig paths), which node's type-stripping cannot resolve.
      const result = await run(["bun", SDK_WORKER], {
        cwd: CLI_DIR,
        env: {
          TRANSFER_RPC_URL: cfg.rpcUrl,
          TRANSFER_WS_URL: cfg.wsUrl,
          TRANSFER_RELAYER_URL: cfg.relayerUrl,
          TRANSFER_ACL_PROGRAM: cfg.aclProgram,
          TRANSFER_CHAIN_ID: alice.chainId,
          TRANSFER_OWNER_KEYPAIR: scenario.alice.keypairPath,
          TRANSFER_OWNER: scenario.alice.owner,
          TRANSFER_RECIPIENT: scenario.bob.owner,
          TRANSFER_MINT: scenario.mint,
          TRANSFER_COMPUTE_SIGNER: scenario.computeSigner,
          TRANSFER_FROM_ACCOUNT: alice.tokenAccount,
          TRANSFER_TO_ACCOUNT: bob.tokenAccount,
          TRANSFER_FROM_BALANCE: alice.encryptedValueAccount,
          TRANSFER_TO_BALANCE: bob.encryptedValueAccount,
        },
      });
      parseTransferWorkerResult(result.stdout);
    },
    decrypt: (scenario, holder, state, expected) =>
      runSolanaCurrentUserDecrypt({
        UD_RELAYER_URL: cfg.relayerUrl,
        UD_GATEWAY_RPC_URL: cfg.gatewayRpcUrl,
        UD_CONTRACTS_CHAIN_ID: state.chainId,
        UD_HANDLE: state.currentHandle,
        UD_SECRET_KEY: holder.secretKey,
        UD_CONTEXT_ID: cfg.userDecryptContext,
        UD_ALLOWED_DOMAIN_KEYS: `0x${Buffer.from(getAddressEncoder().encode(address(scenario.mint))).toString("hex")}`,
        // The env var and the v3 request field keep the wire name `aclValueKey`; what the probe
        // reports is the encrypted value ID it derives.
        UD_ACL_VALUE_KEY: state.encryptedValueId,
        UD_EXPECTED: expected.toString(),
      }),
    async cleanup() {
      if (scenarioDir) await fs.rm(scenarioDir, { recursive: true, force: true });
      scenarioDir = undefined;
    },
  };
};

/** Runs one real two-holder transfer and proves both current balances through independent SDK decrypts. */
export const runSolanaTwoHolderTransfer = async (dependencies: TwoHolderDependencies = createRealTwoHolderDependencies()) => {
  let scenario: TwoHolderScenario | undefined;
  try {
    scenario = await dependencies.provision();
    const initialAlice = await dependencies.readBalance(scenario, scenario.alice);
    const initialBob = await dependencies.readBalance(scenario, scenario.bob);
    await dependencies.waitForHandle(initialAlice.currentHandle);
    await dependencies.waitForHandle(initialBob.currentHandle);
    await dependencies.decrypt(scenario, scenario.alice, initialAlice, 1000n);
    await dependencies.decrypt(scenario, scenario.bob, initialBob, 0n);

    await dependencies.transfer(scenario, initialAlice, initialBob);
    const finalAlice = await dependencies.readBalance(scenario, scenario.alice);
    const finalBob = await dependencies.readBalance(scenario, scenario.bob);
    if (finalAlice.currentHandle === initialAlice.currentHandle || finalBob.currentHandle === initialBob.currentHandle) {
      throw new Error("confidential transfer did not rotate both current balance handles");
    }
    await dependencies.waitForHandle(finalAlice.currentHandle);
    await dependencies.waitForHandle(finalBob.currentHandle);
    await dependencies.decrypt(scenario, scenario.alice, finalAlice, 600n);
    await dependencies.decrypt(scenario, scenario.bob, finalBob, 400n);
    console.log("[solana-two-holder-transfer] Alice=600 Bob=400");
  } finally {
    await dependencies.cleanup(scenario);
  }
};
