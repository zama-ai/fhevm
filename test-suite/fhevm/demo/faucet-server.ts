// faucet-server — the `demo:faucet` entrypoint (#1760). Wires the pure faucet (`./faucet`) to a live
// validator: it reads the seeded demo-config for the RPC/WS endpoints and the mock-USDC mint, loads
// the committed mint-authority keypair, and builds a `UsdcMinter` that mints mock USDC to a
// recipient's associated token account (creating that ATA idempotently first).
//
// The SPL instructions are hand-built with `@solana/kit` primitives on purpose: the test-suite
// carries no `@solana-program/token` dependency. The ATA derivation, `CreateIdempotent`, and
// `MintTo` all come from `./tokenAccounts` (shared with the seed and the typed scenario
// provisioning), which cites their layouts inline.
//
// This process holds a live validator connection and cannot be unit-tested offline; it is exercised
// only by the `solana-e2e` workflow's demo phase (per-PR and manual dispatch), which starts it and funds the
// deposit-arc persona through it. The pure request handler it serves IS unit-tested (`faucet.test.ts`).

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

import { readDemoAllowedOriginFromEnv, readDemoAuthorizationFromEnv } from "./authorization";
import { readDemoConfig } from "./config";
import { serveFaucet, type UsdcMinter } from "./faucet";
import { DEMO_KEYPAIRS } from "./loadDemoEnv";
import { associatedTokenAddress, createIdempotentAtaInstruction, mintToInstruction } from "./tokenAccounts";

/** Builds a `UsdcMinter` that mints mock USDC to a recipient's ATA on the live validator. */
const buildUsdcMinter = async (options: {
  readonly rpcUrl: string;
  readonly wsUrl: string;
  readonly mint: Address;
  readonly mintAuthorityKeypairPath: string;
}): Promise<UsdcMinter> => {
  const rpc = createSolanaRpc(options.rpcUrl);
  const rpcSubscriptions = createSolanaRpcSubscriptions(options.wsUrl);
  const sendAndConfirm = sendAndConfirmTransactionFactory({ rpc, rpcSubscriptions });

  const keypairBytes = Uint8Array.from(JSON.parse(await fs.readFile(options.mintAuthorityKeypairPath, "utf8")) as number[]);
  const authority = await createKeyPairSignerFromBytes(keypairBytes);

  return async (recipient: Address, baseUnits: bigint): Promise<string> => {
    const ata = await associatedTokenAddress(recipient, options.mint);
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

const main = async (): Promise<void> => {
  const authorization = await readDemoAuthorizationFromEnv();
  const allowedOrigin = readDemoAllowedOriginFromEnv();
  const config = await readDemoConfig();
  const mintUsdc = await buildUsdcMinter({
    rpcUrl: config.rpcUrl,
    wsUrl: config.wsUrl,
    // Mock USDC is the deposit underlying (`mints.joinUnderlying`); the faucet drips exactly that.
    mint: address(config.mints.joinUnderlying),
    mintAuthorityKeypairPath: DEMO_KEYPAIRS.mintAuthority,
  });
  const { port } = serveFaucet({ rpcUrl: config.rpcUrl, mintUsdc, authorization, allowedOrigin });
  console.log(`demo faucet listening on http://127.0.0.1:${port} (mock USDC mint ${config.mints.joinUnderlying})`);
};

await main();
