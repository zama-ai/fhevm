// seed — the `demo:seed` entrypoint (#1760). Brings a freshly-deployed demo stack to the state the
// dApp (#1761), the deposit-arc smoke and the rehearsal (#1762) expect, then writes the demo-config
// JSON that every consumer reads.
//
// STATUS: live-only, UNVERIFIED offline. It provisions real on-chain state against a running local
// validator with the two demo programs deployed (their keypairs are classifier-gated in this
// environment — see solana/scripts/demo/demo-keypairs/README). It is exercised end-to-end only by the
// `solana-demo-acceptance` workflow (manual dispatch), which deploys the programs and runs
// `demo-up.sh` (which calls this) before the smoke. The SDK provisioning surface is reached through
// the runtime dynamic-import seam (string module specifier) the rest of test-suite uses, because the
// SDK's generated `_types` are not built at `tsc` check time (see `src/solana/current-user-decrypt.ts`).
//
// Seeding sequence (writes nothing until every step has produced a real on-chain address):
//   1. create the mock-USDC SPL mint (6 decimals, the committed mint-authority as mint authority so
//      `demo:faucet` can later drip it) — hand-built SPL like `faucet-server.ts`.
//   2. `initialize_vault` (demo_vault): creates the vault, its share mint (payout underlying) and the
//      program-owned underlying token account.
//   3. `initialize_mint` ×2 (confidential_token): cUSDC wrapping mock USDC, cShares wrapping the share
//      mint. Same decimals as their underlyings.
//   3b. create each confidential mint's underlying-token escrow — the `vault_usdc` account
//      `wrap_usdc`/`redeem_burned_amount` require to pre-exist (ATA of the mint's `vault_authority`
//      PDA holding the underlying). `initialize_mint` does NOT create it; a missing escrow fails wrap
//      on-chain with 3012 AccountNotInitialized. Both directions' escrows are created up front.
//   4. `initialize_batcher` ×2 (confidential_batcher): the deposit batcher (join cUSDC → payout
//      cShares) and the redeem batcher (the reverse), each with a slot-denominated min batch age.
//   5. `open_batch` ×2 (via `openBatchForBatcher`): opens each batcher's first batch and stands up its
//      per-batch settle Address Lookup Table; the derived table address goes into the config.
//   6. fund the personas (keeper/alice/bob) — and the deployer payer — with SOL for fees.
//   7. derive host/kms roots and write the demo-config JSON (`writeDemoConfig`, which re-parses).

import fs from "node:fs/promises";

import {
  AccountRole,
  appendTransactionMessageInstructions,
  assertIsTransactionWithBlockhashLifetime,
  createKeyPairSignerFromBytes,
  createSolanaRpc,
  createSolanaRpcSubscriptions,
  createTransactionMessage,
  generateKeyPairSigner,
  getAddressEncoder,
  getProgramDerivedAddress,
  getSignatureFromTransaction,
  lamports,
  sendAndConfirmTransactionFactory,
  setTransactionMessageFeePayerSigner,
  setTransactionMessageLifetimeUsingBlockhash,
  signTransactionMessageWithSigners,
  type AccountMeta,
  type Address,
  type Instruction,
  type TransactionSigner,
} from "@solana/kit";

import { resolveEnv } from "../e2e/harness/loadEnv";
import { until } from "../e2e/harness/until";
import { buildVaultUnderlyingEscrowAtaInstruction } from "./tokenAccounts";
import { DEMO_KEYPAIRS } from "./loadDemoEnv";
import {
  resolveDemoConfigPath,
  writeDemoConfig,
  type SolanaDemoConfig,
  type VaultDemoRoots,
} from "./config";

// Well-known program ids (the same literals `faucet-server.ts` and the SDK's `derive.ts` use).
const SPL_TOKEN_PROGRAM_ADDRESS = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA" as Address;
const SYSTEM_PROGRAM_ADDRESS = "11111111111111111111111111111111" as Address;
const COMPUTE_BUDGET_PROGRAM_ADDRESS = "ComputeBudget111111111111111111111111111111" as Address;

const MOCK_USDC_DECIMALS = 6;
const SPL_MINT_ACCOUNT_SPACE = 82n; // SPL Token `Mint` account length.
// The confidential-token instructions emit FHE-handle CPIs; the default 200k CU limit is too low, so
// every provisioning transaction requests the same generous ceiling the SDK's live actions use.
const PROVISIONING_COMPUTE_UNIT_LIMIT = 800_000;
// ~10s live window before a batch may dispatch, at ~400ms/slot on the local validator.
const DEMO_MIN_BATCH_AGE_SLOTS = 25n;
// Lamports the batch authority is funded with (from the payer) to cover its owner-charged rent.
const BATCH_AUTHORITY_FUNDING_LAMPORTS = 100_000_000n;
// Host KMS-context id used at bring-up (context 1). The demo runs against this single context; the
// gateway/user-decrypt context id is a separate value carried in `userDecryptContextId`.
const BRINGUP_KMS_CONTEXT_ID = 1n;
// Addresses per ALT extend transaction. The full settle table is 32 addresses; a single extend of
// all 32 is ~1274 bytes of instruction data alone and overflows the 1232-byte transaction wire
// limit (even before the create instruction and signatures), so the extend is chunked. 20 keeps the
// first chunk — which rides in the same transaction as the table create — comfortably under the
// limit; 32 addresses then split into two chunks (20 + 12).
const SETTLE_ALT_EXTEND_CHUNK_SIZE = 20;

const addressEncoder = getAddressEncoder();
const encodeAddress = (value: Address): Uint8Array => new Uint8Array(addressEncoder.encode(value));

/** Splits `items` into consecutive slices of at most `size`, preserving order. */
const chunk = <T>(items: readonly T[], size: number): T[][] => {
  const chunks: T[][] = [];
  for (let index = 0; index < items.length; index += size) chunks.push(items.slice(index, index + size));
  return chunks;
};

/**
 * A signer account meta: the `signer` field rides along at runtime so `signTransactionMessageWithSigners`
 * produces the signature, while the meta stays typed as a plain `AccountMeta` (same shape the SDK's
 * lookup-table builder uses). Used for the hand-built SPL `CreateAccount`, whose new-account keypair
 * must sign its own creation.
 */
const signerMeta = (signer: TransactionSigner, role: AccountRole): AccountMeta =>
  ({ address: signer.address, role, signer }) as unknown as AccountMeta;

// Structural view of the provisioning members the seed uses from `@fhevm/sdk/solana/vault`. The
// import itself is untyped (runtime dynamic-import seam); this interface restates only what the seed
// calls, so a signature drift on the SDK side surfaces as a local type error here rather than an
// opaque runtime failure. Keep it in lockstep with the vault subpath exports.
type BatchDirectionValue = number;
type OpenBatchResult = {
  readonly instructions: readonly Instruction[];
  readonly lookupTableAddress: Address;
  readonly lookupTableAddresses: readonly Address[];
};
type VaultProvisioning = {
  buildInitializeVaultInstruction(parameters: {
    payer: TransactionSigner;
    vault: TransactionSigner;
    underlyingMint: Address;
  }): Promise<Instruction>;
  buildInitializeMintInstruction(parameters: {
    authority: TransactionSigner;
    mint: TransactionSigner;
    underlyingMint: Address;
    hostConfig: Address;
  }): Promise<Instruction>;
  buildInitializeBatcherInstruction(parameters: {
    payer: TransactionSigner;
    batcher: TransactionSigner;
    joinConfidentialMint: Address;
    payoutConfidentialMint: Address;
    vault: Address;
    minBatchAgeSlots: number | bigint;
    direction: BatchDirectionValue;
  }): Instruction;
  BatchDirection: { readonly Deposit: BatchDirectionValue; readonly Redeem: BatchDirectionValue };
  openBatchForBatcher(parameters: {
    roots: VaultDemoRoots;
    batchIndex: bigint;
    payer: TransactionSigner;
    recentSlot: bigint;
    authorityFundingLamports: number | bigint;
  }): Promise<OpenBatchResult>;
  getExtendLookupTableInstruction(parameters: {
    lookupTable: Address;
    authority: TransactionSigner;
    payer: TransactionSigner;
    addresses: readonly Address[];
  }): Instruction;
  DEMO_VAULT_PROGRAM_ADDRESS: Address;
  CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS: Address;
  ZAMA_HOST_PROGRAM_ADDRESS: Address;
  CONFIDENTIAL_BATCHER_PROGRAM_ADDRESS: Address;
};

/** Loads the vault provisioning surface through the runtime dynamic-import seam (untyped by construction). */
const loadVaultModule = async (): Promise<VaultProvisioning> => {
  const vaultModule = "@fhevm/sdk/solana/vault";
  return (await import(vaultModule)) as unknown as VaultProvisioning;
};

/** ComputeBudget `SetComputeUnitLimit` (tag 2): raises the per-tx CU ceiling for the FHE-heavy CPIs. */
const setComputeUnitLimitInstruction = (units: number): Instruction => {
  const data = new Uint8Array(5);
  data[0] = 2;
  new DataView(data.buffer).setUint32(1, units, true);
  return { programAddress: COMPUTE_BUDGET_PROGRAM_ADDRESS, data };
};

/** SystemProgram `CreateAccount` (tag 0), signed by both the payer and the new account's keypair. */
const createAccountInstruction = (parameters: {
  readonly payer: TransactionSigner;
  readonly newAccount: TransactionSigner;
  readonly lamports: bigint;
  readonly space: bigint;
  readonly owner: Address;
}): Instruction => {
  const data = new Uint8Array(4 + 8 + 8 + 32);
  const view = new DataView(data.buffer);
  view.setUint32(0, 0, true); // instruction index 0 = CreateAccount
  view.setBigUint64(4, parameters.lamports, true);
  view.setBigUint64(12, parameters.space, true);
  data.set(encodeAddress(parameters.owner), 20);
  return {
    programAddress: SYSTEM_PROGRAM_ADDRESS,
    accounts: [
      signerMeta(parameters.payer, AccountRole.WRITABLE_SIGNER),
      signerMeta(parameters.newAccount, AccountRole.WRITABLE_SIGNER),
    ],
    data,
  };
};

/** SPL Token `InitializeMint2` (tag 20): sets decimals + mint authority, no freeze authority. */
const initializeMint2Instruction = (parameters: {
  readonly mint: Address;
  readonly decimals: number;
  readonly mintAuthority: Address;
}): Instruction => {
  const data = new Uint8Array(1 + 1 + 32 + 1);
  data[0] = 20;
  data[1] = parameters.decimals;
  data.set(encodeAddress(parameters.mintAuthority), 2);
  data[34] = 0; // freeze authority COption::None
  return {
    programAddress: SPL_TOKEN_PROGRAM_ADDRESS,
    accounts: [{ address: parameters.mint, role: AccountRole.WRITABLE }],
    data,
  };
};

/** Loads a 64-byte Solana keypair file into a kit `TransactionSigner`. */
const loadSigner = async (keypairPath: string): Promise<TransactionSigner> => {
  const bytes = Uint8Array.from(JSON.parse(await fs.readFile(keypairPath, "utf8")) as number[]);
  return createKeyPairSignerFromBytes(bytes);
};

const LAMPORTS_PER_SOL = 1_000_000_000n;

const main = async (): Promise<void> => {
  const env = resolveEnv();
  const configPath = resolveDemoConfigPath();

  // Remove any prior config up front: it is written only at the very end (step 7), so its presence
  // means a completed prior seed. Deleting it now guarantees that if this run crashes mid-seed, no
  // stale config survives pointing consumers at half-provisioned / now-defunct roots — the absence
  // of the file is the honest signal that no usable stack was seeded.
  await fs.rm(configPath, { force: true });

  const rpc = createSolanaRpc(env.rpcUrl);
  const rpcSubscriptions = createSolanaRpcSubscriptions(env.wsUrl);
  const sendAndConfirm = sendAndConfirmTransactionFactory({ rpc, rpcSubscriptions });

  /** Signs `instructions` with `payer` (fee payer) plus any account-embedded signers, then confirms. */
  const send = async (payer: TransactionSigner, instructions: readonly Instruction[]): Promise<void> => {
    const { value: latestBlockhash } = await rpc.getLatestBlockhash().send();
    const base = setTransactionMessageFeePayerSigner(payer, createTransactionMessage({ version: 0 }));
    const withLifetime = setTransactionMessageLifetimeUsingBlockhash(latestBlockhash, base);
    const message = appendTransactionMessageInstructions(
      [setComputeUnitLimitInstruction(PROVISIONING_COMPUTE_UNIT_LIMIT), ...instructions],
      withLifetime,
    );
    const signedTransaction = await signTransactionMessageWithSigners(message);
    assertIsTransactionWithBlockhashLifetime(signedTransaction);
    await sendAndConfirm(signedTransaction, { commitment: "confirmed" });
    void getSignatureFromTransaction(signedTransaction);
  };

  const airdrop = async (recipient: Address, sol: bigint): Promise<void> => {
    const signature = await rpc
      .requestAirdrop(recipient, lamports(sol * LAMPORTS_PER_SOL), { commitment: "confirmed" })
      .send();
    await until(
      async () => {
        const { value } = await rpc.getSignatureStatuses([signature]).send();
        const status = value[0];
        if (status?.err) throw new Error(`airdrop failed: ${JSON.stringify(status.err)}`);
        const level = status?.confirmationStatus;
        return level === "confirmed" || level === "finalized";
      },
      { description: `airdrop to ${recipient}`, timeoutMs: 30_000 },
    );
  };

  const vault = await loadVaultModule();

  // Actors. The deployer wallet pays for and signs all provisioning; the mint authority is the
  // committed key `demo:faucet` mints mock USDC from; the personas are the demo end-users + operator.
  const deployer = await loadSigner(env.roots.deployerKeypairPath);
  const mintAuthority = await loadSigner(DEMO_KEYPAIRS.mintAuthority);
  const keeper = await loadSigner(DEMO_KEYPAIRS.keeper);
  const alice = await loadSigner(DEMO_KEYPAIRS.alice);
  const bob = await loadSigner(DEMO_KEYPAIRS.bob);

  // Fund the payer + personas before provisioning so every subsequent step has fees available. The
  // mint authority is funded too: `demo:faucet` makes it the fee payer AND the ATA rent payer for
  // every /mint-usdc, so an unfunded mint authority fails the first faucet drip (and the smoke).
  await airdrop(deployer.address, 100n);
  await airdrop(mintAuthority.address, 10n);
  await airdrop(keeper.address, 10n);
  await airdrop(alice.address, 10n);
  await airdrop(bob.address, 10n);

  // Fresh accounts created by this run.
  const mockUsdcMint = await generateKeyPairSigner();
  const vaultAccount = await generateKeyPairSigner();
  const cUsdcMint = await generateKeyPairSigner();
  const cSharesMint = await generateKeyPairSigner();
  const depositBatcher = await generateKeyPairSigner();
  const redeemBatcher = await generateKeyPairSigner();

  // Deterministic host roots: the singleton host config and the bring-up KMS context PDA.
  const [hostConfig] = await getProgramDerivedAddress({
    programAddress: vault.ZAMA_HOST_PROGRAM_ADDRESS,
    seeds: [new TextEncoder().encode("host-config")],
  });
  const kmsContextId = new Uint8Array(8);
  new DataView(kmsContextId.buffer).setBigUint64(0, BRINGUP_KMS_CONTEXT_ID, true);
  // NOTE: kmsContext is DERIVED here (and recorded into the config), not confirmed on-chain — the
  // seed never fetches the account to check it exists/was initialized. It is provisioned by the host
  // bring-up (clean-e2e.sh), so its existence is that step's contract, not this seeder's. A settle
  // against a missing kms-context would fail on-chain at the boundary the smoke does not yet reach.
  const [kmsContext] = await getProgramDerivedAddress({
    programAddress: vault.ZAMA_HOST_PROGRAM_ADDRESS,
    seeds: [new TextEncoder().encode("kms-context"), kmsContextId],
  });
  // The vault's share mint (payout underlying) is a demo_vault PDA of the vault account: [b"shares", vault].
  const [shareMint] = await getProgramDerivedAddress({
    programAddress: vault.DEMO_VAULT_PROGRAM_ADDRESS,
    seeds: [new TextEncoder().encode("shares"), encodeAddress(vaultAccount.address)],
  });

  // 1. Mock-USDC SPL mint (create account + initialize), owned by the classic token program.
  const mintRent = await rpc.getMinimumBalanceForRentExemption(SPL_MINT_ACCOUNT_SPACE).send();
  await send(deployer, [
    createAccountInstruction({
      payer: deployer,
      newAccount: mockUsdcMint,
      lamports: mintRent,
      space: SPL_MINT_ACCOUNT_SPACE,
      owner: SPL_TOKEN_PROGRAM_ADDRESS,
    }),
    initializeMint2Instruction({
      mint: mockUsdcMint.address,
      decimals: MOCK_USDC_DECIMALS,
      mintAuthority: mintAuthority.address,
    }),
  ]);

  // 2. Vault (creates the share mint + program token account as PDAs).
  await send(deployer, [
    await vault.buildInitializeVaultInstruction({
      payer: deployer,
      vault: vaultAccount,
      underlyingMint: mockUsdcMint.address,
    }),
  ]);

  // 3. Confidential mints: cUSDC wraps mock USDC, cShares wraps the share mint.
  await send(deployer, [
    await vault.buildInitializeMintInstruction({
      authority: deployer,
      mint: cUsdcMint,
      underlyingMint: mockUsdcMint.address,
      hostConfig,
    }),
  ]);
  await send(deployer, [
    await vault.buildInitializeMintInstruction({
      authority: deployer,
      mint: cSharesMint,
      underlyingMint: shareMint,
      hostConfig,
    }),
  ]);

  // 3b. Underlying-token escrows. `wrap_usdc` and `redeem_burned_amount` both take the confidential
  // mint's `vault_usdc` = ATA(vault_authority(mint), underlyingMint) and require it to already exist
  // (neither instruction inits it). Create both mints' escrows up front — cUSDC/mock-USDC is exercised
  // by the wrap-leg smoke; cShares/share-mint is the redeem-direction mirror — so a later redeem does
  // not fail the same way one step past what the smoke covers.
  const cUsdcEscrow = await buildVaultUnderlyingEscrowAtaInstruction({
    payer: deployer,
    tokenProgram: vault.CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS,
    confidentialMint: cUsdcMint.address,
    underlyingMint: mockUsdcMint.address,
  });
  const cSharesEscrow = await buildVaultUnderlyingEscrowAtaInstruction({
    payer: deployer,
    tokenProgram: vault.CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS,
    confidentialMint: cSharesMint.address,
    underlyingMint: shareMint,
  });
  await send(deployer, [cUsdcEscrow.instruction, cSharesEscrow.instruction]);

  // 4. Batchers: deposit (join cUSDC → payout cShares) and redeem (the reverse).
  await send(deployer, [
    vault.buildInitializeBatcherInstruction({
      payer: deployer,
      batcher: depositBatcher,
      joinConfidentialMint: cUsdcMint.address,
      payoutConfidentialMint: cSharesMint.address,
      vault: vaultAccount.address,
      minBatchAgeSlots: DEMO_MIN_BATCH_AGE_SLOTS,
      direction: vault.BatchDirection.Deposit,
    }),
  ]);
  await send(deployer, [
    vault.buildInitializeBatcherInstruction({
      payer: deployer,
      batcher: redeemBatcher,
      joinConfidentialMint: cSharesMint.address,
      payoutConfidentialMint: cUsdcMint.address,
      vault: vaultAccount.address,
      minBatchAgeSlots: DEMO_MIN_BATCH_AGE_SLOTS,
      direction: vault.BatchDirection.Redeem,
    }),
  ]);

  const commonRoots = {
    batcherProgram: vault.CONFIDENTIAL_BATCHER_PROGRAM_ADDRESS,
    tokenProgram: vault.CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS,
    vaultProgram: vault.DEMO_VAULT_PROGRAM_ADDRESS,
    hostProgram: vault.ZAMA_HOST_PROGRAM_ADDRESS,
    vault: vaultAccount.address,
    hostConfig,
    kmsContext,
  } as const;
  const depositRoots: VaultDemoRoots = {
    ...commonRoots,
    batcher: depositBatcher.address,
    joinConfidentialMint: cUsdcMint.address,
    payoutConfidentialMint: cSharesMint.address,
    joinUnderlyingMint: mockUsdcMint.address,
    payoutUnderlyingMint: shareMint,
  };
  const redeemRoots: VaultDemoRoots = {
    ...commonRoots,
    batcher: redeemBatcher.address,
    joinConfidentialMint: cSharesMint.address,
    payoutConfidentialMint: cUsdcMint.address,
    joinUnderlyingMint: shareMint,
    payoutUnderlyingMint: mockUsdcMint.address,
  };

  // 5. Open the first batch on each batcher and stand up its settle lookup table. open_batch carries
  // ~24 accounts, so it goes in its own transaction. The table's create then rides with the FIRST
  // extend chunk; the remaining chunks follow one transaction at a time. The single 32-address extend
  // the SDK returns is rebuilt here in `SETTLE_ALT_EXTEND_CHUNK_SIZE` chunks because a lone 32-address
  // extend overflows the 1232-byte wire limit. Each extend is confirmed before the next, so the table
  // is fully populated before `settle` ever reads it.
  const openFirstBatch = async (roots: VaultDemoRoots): Promise<Address> => {
    const recentSlot = await rpc.getSlot({ commitment: "finalized" }).send();
    const opened = await vault.openBatchForBatcher({
      roots,
      batchIndex: 0n,
      payer: deployer,
      recentSlot,
      authorityFundingLamports: BATCH_AUTHORITY_FUNDING_LAMPORTS,
    });
    const [openBatchInstruction, createLookupTable] = opened.instructions;
    await send(deployer, [openBatchInstruction!]);
    const extendChunks = chunk(opened.lookupTableAddresses, SETTLE_ALT_EXTEND_CHUNK_SIZE);
    for (const [chunkIndex, addresses] of extendChunks.entries()) {
      const extendLookupTable = vault.getExtendLookupTableInstruction({
        lookupTable: opened.lookupTableAddress,
        authority: deployer,
        payer: deployer,
        addresses,
      });
      // The create must land in the same transaction that first extends the table (or immediately
      // before it); pair it with chunk 0, then send each later chunk on its own.
      await send(deployer, chunkIndex === 0 ? [createLookupTable!, extendLookupTable] : [extendLookupTable]);
    }
    return opened.lookupTableAddress;
  };
  const depositLookupTable = await openFirstBatch(depositRoots);
  const redeemLookupTable = await openFirstBatch(redeemRoots);

  // 6 + 7. Assemble and persist the demo-config. Endpoints/ids come from the resolved env; the vault
  // roots are the real addresses provisioned above. `writeDemoConfig` re-parses before persisting, so
  // a malformed assembly fails at write with a named field rather than later inside an SDK call.
  const config: SolanaDemoConfig = {
    source: "demo-config",
    chainId: env.chainId.toString(),
    rpcUrl: env.rpcUrl,
    wsUrl: env.wsUrl,
    relayerUrl: env.relayerUrl,
    proofServiceUrl: env.proofServiceUrl,
    gatewayRpcUrl: env.gatewayRpcUrl,
    aclProgram: env.aclProgram,
    userDecryptContextId: env.userDecryptContextId,
    programs: {
      batcher: vault.CONFIDENTIAL_BATCHER_PROGRAM_ADDRESS,
      token: vault.CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS,
      vault: vault.DEMO_VAULT_PROGRAM_ADDRESS,
      host: vault.ZAMA_HOST_PROGRAM_ADDRESS,
    },
    hostConfig,
    kmsContext,
    vault: vaultAccount.address,
    mints: {
      joinUnderlying: mockUsdcMint.address,
      payoutUnderlying: shareMint,
      joinConfidential: cUsdcMint.address,
      payoutConfidential: cSharesMint.address,
    },
    batchers: {
      deposit: { batcher: depositBatcher.address, lookupTable: depositLookupTable },
      redeem: { batcher: redeemBatcher.address, lookupTable: redeemLookupTable },
    },
    mintAuthority: mintAuthority.address,
    personas: { keeper: keeper.address, alice: alice.address, bob: bob.address },
  };
  await writeDemoConfig(config, configPath);
  console.log(`demo-config written to ${configPath}`);
};

await main();
