import { prepareTransientStore, type TransientStore } from "@fhevm/sdk/solana";
// seed — the `demo:seed` entrypoint (#1760). Brings a freshly-deployed demo stack to the state the
// dApp (#1761), the deposit-arc smoke and the rehearsal (#1762) expect, then writes the demo-config
// JSON that every consumer reads.
//
// STATUS: live-only, UNVERIFIED offline. It provisions real on-chain state against a running local
// validator with the two demo programs deployed (their keypairs are classifier-gated in this
// environment — see solana/scripts/demo/demo-keypairs/README). It is exercised end-to-end only by the
// `solana-e2e` workflow's demo phase (per-PR and manual dispatch), which deploys the programs and runs
// `demo-up.sh` (which calls this) before the smoke. The vault provisioning surface is the demo
// dapp's own module (`solana/demo-dapp/src/vault`, fhevm-internal#1859 §6d), imported statically
// and fully typed.
//
// Seeding sequence (writes nothing until every step has produced a real on-chain address):
//   0. verify the bring-up's kms-context account exists on-chain — the seeder never creates it, it
//      only fails loudly (with remediation) when the host bring-up did not provision it.
//   1. create the mock-USDC SPL mint (6 decimals, the committed mint-authority as mint authority so
//      `demo:operator` can later drip it) — `@solana-program/token` instructions; only the System
//      `CreateAccount` comes from `@solana-program/system`.
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
//   5. `open_batch` ×2 (via `openBatchForBatcher`): opens each batcher's first batch.
//   6. fund the personas (keeper/alice/bob) — and the deployer payer — with SOL for fees.
//   7. derive host/kms roots and write the demo-config JSON (`writeDemoConfig`, which re-parses).

import fs from "node:fs/promises";
import {
  generateKeyPairSigner,
  type Instruction,
  type TransactionSigner,
} from "@solana/kit";
import { getCreateAccountInstruction } from "@solana-program/system";
import { findKmsContextPda } from "@fhevm/solana-zama-host";
import {
  TOKEN_PROGRAM_ADDRESS as SPL_TOKEN_PROGRAM_ADDRESS,
  getMintSize,
  getInitializeMint2Instruction,
} from "@solana-program/token";
import { loadEnv } from "../e2e/harness/loadEnv";
import { openProvisioning } from "../e2e/harness/solana/provisioning";
import {
  BRINGUP_KMS_CONTEXT_ID,
} from "../src/solana/addresses";
import { loadKeypairSigner } from "../src/solana/provision";
import { buildVaultUnderlyingEscrowAtaInstruction } from "../src/solana/spl";
import { ensureDemoRecoveryKey, mirrorRecoveryKeys, recoveryDirectory } from "../src/solana/recovery";
import { demoKeypairs } from "./loadDemoEnv";
import {
  resolveDemoConfigPath,
  writeDemoConfig,
  type SolanaDemoConfig,
  type VaultDemoRoots,
} from "./config";
import * as vault from "@demo-dapp/vault/index.js";

const MOCK_USDC_DECIMALS = 6;
// ~10s live window before a batch may dispatch, at ~400ms/slot on the local validator.
const DEMO_MIN_BATCH_AGE_SLOTS = 25n;
// Lamports the batch authority is funded with (from the payer) to cover its owner-charged rent.
const BATCH_AUTHORITY_FUNDING_LAMPORTS = 100_000_000n;



const main = async (): Promise<void> => {
  const env = loadEnv();
  const configPath = resolveDemoConfigPath();

  // Preserve the last inventory even if a reseed fails before publishing a replacement.
  if (env.network === "devnet") {
    try {
      await fs.copyFile(configPath, `${configPath}.${Date.now()}.previous`);
    } catch (error) {
      if ((error as NodeJS.ErrnoException).code !== "ENOENT") throw error;
    }
    for (const role of ["keeper", "alice", "bob", "mintAuthority"]) await ensureDemoRecoveryKey(role);
    await mirrorRecoveryKeys();
  }

  // The shared provisioning clients and fund closures.
  const provisioning = await openProvisioning(env);
  const { rpc } = provisioning;
  const send = async (payer: TransactionSigner, instructions: readonly Instruction[]): Promise<void> => {
    await (await provisioning.client(payer)).sendTransaction(instructions);
  };
  const sendFhe = async (
    payer: TransactionSigner,
    transientStore: TransientStore,
    instructions: readonly Instruction[],
  ): Promise<void> => {
    await (await provisioning.client(payer)).sendFheTransaction(transientStore, instructions);
  };

  // Actors. The deployer drives provisioning; the keeper pays confidential-mint account rent and
  // is the wrapper authority used by settlement/cancellation. The separate mock-USDC mint
  // authority backs the operator's faucet, and Alice/Bob are end users.
  const deployer = await loadKeypairSigner(env.roots.deployerKeypairPath);
  const mintAuthority = await loadKeypairSigner(demoKeypairs(env).mintAuthority);
  const keeper = await loadKeypairSigner(demoKeypairs(env).keeper);
  const alice = await loadKeypairSigner(demoKeypairs(env).alice);
  const bob = await loadKeypairSigner(demoKeypairs(env).bob);

  // Fund the personas before provisioning so every subsequent step has fees available. A local
  // validator airdrops the deployer first; on devnet the deployer is the funder and pays from its
  // own balance. The mint authority is funded too: `demo:operator` makes it the fee payer AND the ATA
  // rent payer for every mint-usdc, so an unfunded mint authority fails the first faucet drip.
  if (env.capabilities.faucet) await provisioning.fundSol(deployer.address, 100);
  for (const actor of [mintAuthority, keeper, alice, bob]) {
    await provisioning.fundSol(actor.address, env.funding.primarySol);
  }

  // Fresh accounts created by this run.
  const mockUsdcMint = await generateKeyPairSigner();
  const vaultAccount = await generateKeyPairSigner();
  const cUsdcMint = await generateKeyPairSigner();
  const cSharesMint = await generateKeyPairSigner();
  const depositBatcher = await generateKeyPairSigner();
  const redeemBatcher = await generateKeyPairSigner();

  // Deterministic host root: the bring-up KMS context PDA.
  const [kmsContext] = await findKmsContextPda({ contextId: BRINGUP_KMS_CONTEXT_ID });
  // The kms-context account is provisioned by the HOST BRING-UP, not by this seeder — the seeder
  // must never create it (it has neither the authority nor the key material to). But the smoke's
  // settle phase consumes it on-chain, so a missing account is a bring-up failure this seed can catch
  // NOW, at provisioning time, instead of ~15 minutes later inside the live settle. Fail loudly.
  const kmsContextAccount = await rpc.getAccountInfo(kmsContext, { encoding: "base64" }).send();
  if (kmsContextAccount.value === null) {
    throw new Error(
      `kms-context account ${kmsContext} (context id 0x${Buffer.from(BRINGUP_KMS_CONTEXT_ID).toString("hex")}) does not exist on-chain. ` +
        "It is provisioned by the host bring-up (the step that deploys the zama-host program and " +
        "initializes its config + KMS context), NOT by demo:seed. Re-run the host bring-up against " +
        "this validator and confirm it initialized KMS context 1, then re-run demo:seed.",
    );
  }
  const [shareMint] = await vault.findShareMintPda({ vault: vaultAccount.address });

  if (env.network === "devnet") {
    await fs.writeFile(`${recoveryDirectory()}/inventory-${mockUsdcMint.address}.json`, JSON.stringify({
      mints: [mockUsdcMint.address, shareMint],
      accounts: [vaultAccount.address, cUsdcMint.address, cSharesMint.address, depositBatcher.address, redeemBatcher.address],
    }), { mode: 0o600, flag: "wx" });
    await mirrorRecoveryKeys();
  }

  // 1. Mock-USDC SPL mint (create account + initialize), owned by the classic token program.
  const mintRent = await rpc.getMinimumBalanceForRentExemption(BigInt(getMintSize())).send();
  await send(deployer, [
    getCreateAccountInstruction({
      payer: deployer,
      newAccount: mockUsdcMint,
      lamports: mintRent,
      space: BigInt(getMintSize()),
      programAddress: SPL_TOKEN_PROGRAM_ADDRESS,
    }),
    getInitializeMint2Instruction({
      mint: mockUsdcMint.address,
      decimals: MOCK_USDC_DECIMALS,
      mintAuthority: mintAuthority.address,
    }),
  ]);

  // 2. Vault (creates the share mint + program token account as PDAs).
  await send(deployer, [
    await vault.getInitializeVaultInstructionAsync({
      payer: deployer,
      vault: vaultAccount,
      underlyingMint: mockUsdcMint.address,
    }),
  ]);

  const mintTransientStore = await prepareTransientStore({ payer: deployer, host: vault.ZAMA_HOST_PROGRAM_ADDRESS });
  // 3. Confidential mints: cUSDC wraps mock USDC, cShares wraps the share mint.
  await sendFhe(deployer, mintTransientStore, [
    await vault.buildInitializeMintInstruction({
      transientStore: mintTransientStore,
      authority: keeper,
      mint: cUsdcMint,
      underlyingMint: mockUsdcMint.address,
    }),
  ]);
  await sendFhe(deployer, mintTransientStore, [
    await vault.buildInitializeMintInstruction({
      transientStore: mintTransientStore,
      authority: keeper,
      mint: cSharesMint,
      underlyingMint: shareMint,
    }),
  ]);

  // 3b. Underlying-token escrows. `wrap_usdc` and `redeem_burned_amount` both take the confidential
  // mint's `vault_usdc` = ATA(vault_authority(mint), underlyingMint) and require it to already exist
  // (neither instruction inits it). Create both mints' escrows up front — cUSDC/mock-USDC is exercised
  // by the deposit-arc smoke; cShares/share-mint is the redeem-direction mirror — so a later redeem does
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
    vault.getInitializeBatcherInstruction({
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
    vault.getInitializeBatcherInstruction({
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

  // 5. Open the first batch on each batcher.
  const openFirstBatch = async (roots: VaultDemoRoots): Promise<void> => {
    const transientStore = await prepareTransientStore({ payer: keeper, host: vault.ZAMA_HOST_PROGRAM_ADDRESS });
    await sendFhe(keeper, transientStore, [
      await vault.openBatchForBatcher({
        transientStore,
        roots,
        batchIndex: 0n,
        payer: keeper,
        authorityFundingLamports: BATCH_AUTHORITY_FUNDING_LAMPORTS,
      }),
    ]);
  };
  await openFirstBatch(depositRoots);
  await openFirstBatch(redeemRoots);

  // 6 + 7. Assemble and persist the demo-config. Endpoints/ids come from the resolved env; the vault
  // roots are the real addresses provisioned above. `writeDemoConfig` re-parses before persisting, so
  // a malformed assembly fails at write with a named field rather than later inside an SDK call.
  const config: SolanaDemoConfig = {
    source: "demo-config",
    network: env.network,
    chainId: env.chainId.toString(),
    rpcUrl: env.rpcUrl,
    wsUrl: env.wsUrl,
    relayerUrl: env.relayerUrl,
    gatewayRpcUrl: env.gatewayRpcUrl,
    aclProgram: env.aclProgram,
    // The local stack runs the test FHE parameter set; the SDK reads the KMS trust from zama-host.
    fheParameter: "test",
    // Must suffice to cover the rent settle's CPIs charge to the batch authority; the open_batch
    // value is recorded as a known-good amount.
    authorityFundingLamports: BATCH_AUTHORITY_FUNDING_LAMPORTS.toString(),
    programs: {
      batcher: vault.CONFIDENTIAL_BATCHER_PROGRAM_ADDRESS,
      token: vault.CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS,
      vault: vault.DEMO_VAULT_PROGRAM_ADDRESS,
      host: vault.ZAMA_HOST_PROGRAM_ADDRESS,
    },
    vault: vaultAccount.address,
    mints: {
      joinUnderlying: mockUsdcMint.address,
      payoutUnderlying: shareMint,
      joinConfidential: cUsdcMint.address,
      payoutConfidential: cSharesMint.address,
    },
    batchers: {
      deposit: { batcher: depositBatcher.address },
      redeem: { batcher: redeemBatcher.address },
    },
    mintAuthority: mintAuthority.address,
    personas: { keeper: keeper.address, alice: alice.address, bob: bob.address },
  };
  await writeDemoConfig(config, configPath);
  console.log(`demo-config written to ${configPath}`);
};

await main();
