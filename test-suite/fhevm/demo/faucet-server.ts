// faucet-server — the `demo:faucet` entrypoint (#1760). Wires the pure faucet (`./faucet`) to a live
// validator: it reads the seeded demo-config for the RPC/WS endpoints and the mock-USDC mint, loads
// the committed mint-authority keypair, and builds a `UsdcMinter` that mints mock USDC to a
// recipient's associated token account (creating that ATA idempotently first).
//
// The SPL instructions are hand-built with `@solana/kit` primitives on purpose: the test-suite
// carries no `@solana-program/token` dependency, and the two instructions the faucet needs are
// small and stable. Layouts cited inline:
//   - SPL Token `MintTo` (tag 7): data = [7, amount:u64-le]; accounts [mint(w), destination(w), authority(s)].
//     https://github.com/solana-program/token — processor `Instruction::MintTo`.
//   - Associated-Token `CreateIdempotent` (tag 1): no data args beyond the tag; accounts
//     [payer(ws), ata(w), owner, mint, systemProgram, tokenProgram]. Idempotent = a no-op if the ATA
//     already exists, so repeat drips to the same recipient don't error.
//     https://github.com/solana-program/associated-token-account.
//
// This process holds a live validator connection and cannot be unit-tested offline; it is exercised
// only by the `solana-demo-acceptance` workflow (manual dispatch), which starts it and funds the
// deposit-arc persona through it. The pure request handler it serves IS unit-tested (`faucet.test.ts`).

import fs from "node:fs/promises";

import {
  AccountRole,
  address,
  appendTransactionMessageInstructions,
  assertIsTransactionWithBlockhashLifetime,
  createKeyPairSignerFromBytes,
  createSolanaRpc,
  createSolanaRpcSubscriptions,
  createTransactionMessage,
  getAddressEncoder,
  getProgramDerivedAddress,
  getSignatureFromTransaction,
  sendAndConfirmTransactionFactory,
  setTransactionMessageFeePayerSigner,
  setTransactionMessageLifetimeUsingBlockhash,
  signTransactionMessageWithSigners,
  type Address,
  type Instruction,
  type TransactionSigner,
} from "@solana/kit";

import { readDemoConfig } from "./config";
import { DEMO_KEYPAIRS } from "./loadDemoEnv";
import { serveFaucet, type UsdcMinter } from "./faucet";

// Well-known SPL program ids (same literals the SDK's vault `derive.ts` uses).
const SPL_TOKEN_PROGRAM_ADDRESS = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA" as Address;
const ASSOCIATED_TOKEN_PROGRAM_ADDRESS = "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL" as Address;
const SYSTEM_PROGRAM_ADDRESS = "11111111111111111111111111111111" as Address;

const addressEncoder = getAddressEncoder();
const encodeAddress = (value: Address): Uint8Array => new Uint8Array(addressEncoder.encode(value));

/** Derives a recipient's associated token account for `mint` (canonical ATA seeds). */
const associatedTokenAddress = async (owner: Address, mint: Address): Promise<Address> => {
  const [ata] = await getProgramDerivedAddress({
    programAddress: ASSOCIATED_TOKEN_PROGRAM_ADDRESS,
    seeds: [encodeAddress(owner), encodeAddress(SPL_TOKEN_PROGRAM_ADDRESS), encodeAddress(mint)],
  });
  return ata;
};

/** Associated-Token `CreateIdempotent` (tag 1): a no-op when the ATA already exists. */
const createIdempotentAtaInstruction = (params: {
  readonly payer: TransactionSigner;
  readonly ata: Address;
  readonly owner: Address;
  readonly mint: Address;
}): Instruction => ({
  programAddress: ASSOCIATED_TOKEN_PROGRAM_ADDRESS,
  accounts: [
    { address: params.payer.address, role: AccountRole.WRITABLE_SIGNER },
    { address: params.ata, role: AccountRole.WRITABLE },
    { address: params.owner, role: AccountRole.READONLY },
    { address: params.mint, role: AccountRole.READONLY },
    { address: SYSTEM_PROGRAM_ADDRESS, role: AccountRole.READONLY },
    { address: SPL_TOKEN_PROGRAM_ADDRESS, role: AccountRole.READONLY },
  ],
  data: new Uint8Array([1]),
});

/** SPL Token `MintTo` (tag 7): mints `baseUnits` to `destination`, signed by the mint authority. */
const mintToInstruction = (params: {
  readonly mint: Address;
  readonly destination: Address;
  readonly authority: TransactionSigner;
  readonly baseUnits: bigint;
}): Instruction => {
  const data = new Uint8Array(9);
  data[0] = 7;
  new DataView(data.buffer).setBigUint64(1, params.baseUnits, true);
  return {
    programAddress: SPL_TOKEN_PROGRAM_ADDRESS,
    accounts: [
      { address: params.mint, role: AccountRole.WRITABLE },
      { address: params.destination, role: AccountRole.WRITABLE },
      { address: params.authority.address, role: AccountRole.READONLY_SIGNER },
    ],
    data,
  };
};

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
  const config = await readDemoConfig();
  const mintUsdc = await buildUsdcMinter({
    rpcUrl: config.rpcUrl,
    wsUrl: config.wsUrl,
    // Mock USDC is the deposit underlying (`mints.joinUnderlying`); the faucet drips exactly that.
    mint: address(config.mints.joinUnderlying),
    mintAuthorityKeypairPath: DEMO_KEYPAIRS.mintAuthority,
  });
  const { port } = serveFaucet({ rpcUrl: config.rpcUrl, mintUsdc });
  console.log(`demo faucet listening on http://127.0.0.1:${port} (mock USDC mint ${config.mints.joinUnderlying})`);
};

await main();
