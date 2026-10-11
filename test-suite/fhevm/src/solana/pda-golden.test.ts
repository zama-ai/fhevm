import * as batcherPdas from "../../../../solana/demo-dapp/src/vault/internal/generated/confidentialBatcher/pdas/index.js";
import * as vaultPdas from "../../../../solana/demo-dapp/src/vault/internal/generated/demoVault/pdas/index.js";
import * as chainPdas from "./internal/generated/depChain/pdas/index.js";
import * as counterPdas from "./internal/generated/encryptedCounter/pdas/index.js";
import { CONFIDENTIAL_BATCHER_PROGRAM_ADDRESS } from "../../../../solana/demo-dapp/src/vault/internal/generated/confidentialBatcher/programAddress.js";
import { DEMO_VAULT_PROGRAM_ADDRESS } from "../../../../solana/demo-dapp/src/vault/internal/generated/demoVault/programAddress.js";
import { DEP_CHAIN_PROGRAM_ADDRESS } from "./internal/generated/depChain/programAddress.js";
import { ENCRYPTED_COUNTER_PROGRAM_ADDRESS } from "./internal/generated/encryptedCounter/programAddress.js";
import { getDispatchInstructionAsync, parseDispatchInstruction } from "../../../../solana/demo-dapp/src/vault/internal/generated/confidentialBatcher/instructions/dispatch.js";
import { getInitializeBatcherInstructionAsync, parseInitializeBatcherInstruction } from "../../../../solana/demo-dapp/src/vault/internal/generated/confidentialBatcher/instructions/initializeBatcher.js";
import { BatchDirection } from "../../../../solana/demo-dapp/src/vault/internal/generated/confidentialBatcher/types/batchDirection.js";
import { getInitializeVaultInstructionAsync, parseInitializeVaultInstruction } from "../../../../solana/demo-dapp/src/vault/internal/generated/demoVault/instructions/initializeVault.js";
import { getInitializeInstructionAsync as initializeChain, parseInitializeInstruction as parseChain } from "./internal/generated/depChain/instructions/initialize.js";
import { getInitializeInstructionAsync as initializeCounter, parseInitializeInstruction as parseCounter } from "./internal/generated/encryptedCounter/instructions/initialize.js";
import { describe, expect, it } from "bun:test";
import { address, createNoopSigner, getProgramDerivedAddress, type ProgramDerivedAddress } from "@solana/kit";
import * as host from "@fhevm/solana-zama-host";
import * as token from "@fhevm/confidential-token";
import fixture from "../../../../solana/test-fixtures/pda/pda_v1.json";

const input = fixture.inputs;
const key = (name: Exclude<keyof typeof input, "contextId" | "batchIndex">) => address(input[name]);

async function check(
  expected: { id: string; pdas: Record<string, { address: string; bump: number }> },
  id: string,
  finders: Record<string, Promise<ProgramDerivedAddress>>,
) {
  expect(id).toBe(expected.id);
  expect(Object.keys(finders).sort()).toEqual(Object.keys(expected.pdas).sort());
  for (const [name, finder] of Object.entries(finders)) {
    const pda = expected.pdas[name]!;
    const [address, bump] = await finder;
    expect({ address: address as string, bump: bump as number }, name).toEqual(pda);
  }
}

describe("program-owned PDA golden", () => {
  it("pins every zama-host recipe through generated finders", async () => {
    expect(key("program")).toBe(token.CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS);
    expect(key("scope")).toBe(key("mint"));
    await check(fixture.programs.zamaHost, host.ZAMA_HOST_PROGRAM_ADDRESS, {
      hostConfig: host.findHostConfigPda(),
      kmsContext: host.findKmsContextPda({ contextId: new Uint8Array(input.contextId) }),
      randNonce: host.findRandNoncePda(),
      encryptedStore: host.findEncryptedStorePda({ program: key("program"), authority: key("authority"), scope: key("scope") }),
      transientStore: host.findTransientStorePda({ payer: key("payer") }),
      delegationRecord: host.findDelegationRecordPda({ delegator: key("delegator"), delegate: key("delegate"), program: key("program"), scope: key("scope") }),
      invalidation: host.findInvalidationPda({ user: key("user") }),
      denyScopeRecord: host.findDenyScopeRecordPda({ appProgram: key("program"), scope: key("scope") }),
      hcuTrustedAppRecord: host.findHcuTrustedAppRecordPda({ appProgram: key("program"), scope: key("scope") }),
      hcuBlockMeter: host.findHcuBlockMeterPda({ appProgram: key("program"), scope: key("scope") }),
      pauserRecord: host.findPauserRecordPda({ pauser: key("user") }),
      eventAuthority: host.findEventAuthorityPda(),
    });
  });

  it("pins every confidential-token recipe through generated finders", async () => {
    await check(fixture.programs.confidentialToken, token.CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS, {
      tokenAccount: token.findTokenAccountPda({ mint: key("mint"), owner: key("owner") }),
      totalSupplyAuthority: token.findTotalSupplyAuthorityPda({ mint: key("mint") }),
      vaultAuthority: token.findVaultAuthorityPda({ mint: key("mint") }),
      pendingBurn: token.findPendingBurnPda({ mint: key("mint"), tokenAccount: key("tokenAccount") }),
      eventAuthority: token.findEventAuthorityPda(),
    });
  });

  it("binds generated seed encoders and builders to their finders", async () => {
    const [transientStore] = await host.findTransientStorePda({ payer: key("payer") });
    const open = await host.getOpenTransientStoreInstructionAsync({ payer: createNoopSigner(key("payer")) });
    const close = await host.getCloseTransientStoreInstructionAsync({ payer: key("payer") });
    expect(open.accounts[1]?.address).toBe(transientStore);
    expect(close.accounts[1]?.address).toBe(transientStore);
    const vault = await token.findVaultAuthorityPda({ mint: key("mint") });
    const { getProgramDerivedAddress } = await import("@solana/kit");
    expect(await getProgramDerivedAddress({ programAddress: token.CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS, seeds: token.getVaultAuthorityPdaSeeds({ mint: key("mint") }) })).toEqual(vault);
  });
  it("resolves foreign defaults through the host finders and keeps event authorities separate", async () => {
    const payer = createNoopSigner(key("payer"));
    const fromAccount = key("tokenAccount");
    const toAccount = key("authority");
    const instruction = await token.getConfidentialTransferInstructionAsync({
      owner: createNoopSigner(key("owner")), payer, mint: key("mint"),
      underlyingMint: key("scope"), fromAta: key("user"), toAta: key("delegate"),
      fromAccount, toAccount,
      transientStore: (await host.findTransientStorePda({ payer: payer.address }))[0],
      instructions: address("Sysvar1nstructions1111111111111111111111111"),
      amountAttestation: { inputHandle: new Uint8Array(32), ctHandles: [], handleIndex: 0,
        userAddress: new Uint8Array(32), contractAddress: new Uint8Array(32), contractChainId: 1n,
        extraData: new Uint8Array(), signatures: [] },
    });
    const parsed = token.parseConfidentialTransferInstruction(instruction).accounts;
    expect(parsed.fromStore.address).toBe((await host.findEncryptedStorePda({ program: token.CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS, authority: fromAccount, scope: key("mint") }))[0]);
    expect(parsed.toStore.address).toBe((await host.findEncryptedStorePda({ program: token.CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS, authority: toAccount, scope: key("mint") }))[0]);
    expect(parsed.hostConfig.address).toBe((await host.findHostConfigPda())[0]);
    expect(parsed.zamaEventAuthority.address).toBe((await host.findEventAuthorityPda())[0]);
    expect(parsed.eventAuthority.address).toBe((await token.findEventAuthorityPda())[0]);
  });
  it("keeps optional HCU accounts absent until the caller supplies them", async () => {
    const input = {
      owner: createNoopSigner(key("owner")), mint: key("mint"), tokenAccount: key("tokenAccount"),
      balanceStore: (await host.findEncryptedStorePda({ program: token.CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS, authority: key("tokenAccount"), scope: key("mint") }))[0],
      totalSupplyStore: (await host.findEncryptedStorePda({ program: token.CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS, authority: (await token.findTotalSupplyAuthorityPda({ mint: key("mint") }))[0], scope: key("mint") }))[0],
      underlyingMint: key("scope"), userUsdc: key("user"), vaultUsdc: key("authority"),
      instructions: address("Sysvar1nstructions1111111111111111111111111"),
      transientStore: (await host.findTransientStorePda({ payer: key("payer") }))[0], amount: 1n,
    };
    const omitted = token.parseWrapUsdcInstruction(await token.getWrapUsdcInstructionAsync(input));
    expect(omitted.accounts.hcuBlockMeter).toBeUndefined();
    expect(omitted.accounts.hcuTrustedAppRecord).toBeUndefined();
    const meter = address(fixture.programs.zamaHost.pdas.hcuBlockMeter.address);
    const explicit = token.parseWrapUsdcInstruction(await token.getWrapUsdcInstructionAsync({ ...input, hcuBlockMeter: meter }));
    expect(explicit.accounts.hcuBlockMeter?.address).toBe(meter);
    expect(explicit.accounts.hcuTrustedAppRecord).toBeUndefined();
  });

});


describe("demo and specimen PDA golden", () => {
  const index = BigInt(input.batchIndex);

  it("pins every confidential-batcher recipe", async () => {
    const batch = (await batcherPdas.findBatchPda({ batcher: key("batcher"), index }))[0];
    await check(fixture.programs.confidentialBatcher, CONFIDENTIAL_BATCHER_PROGRAM_ADDRESS, {
      batch: batcherPdas.findBatchPda({ batcher: key("batcher"), index }),
      batchAuthority: batcherPdas.findBatchAuthorityPda({ batch }),
      joinRecord: batcherPdas.findJoinRecordPda({ batch, user: key("user") }),
      batchJoinUnderlying: batcherPdas.findBatchJoinUnderlyingPda({ batch }),
      batchPayoutUnderlying: batcherPdas.findBatchPayoutUnderlyingPda({ batch }),
    });
  });

  it("pins every demo-vault recipe", async () => {
    await check(fixture.programs.demoVault, DEMO_VAULT_PROGRAM_ADDRESS, {
      vaultAuthority: vaultPdas.findVaultAuthorityPda({ vault: key("vault") }),
      shareMint: vaultPdas.findShareMintPda({ vault: key("vault") }),
      vaultTokenAccount: vaultPdas.findVaultTokenAccountPda({ vault: key("vault") }),
    });
  });

  it("pins every specimen recipe", async () => {
    const chain = (await chainPdas.findChainPda({ owner: key("owner") }))[0];
    await check(fixture.programs.depChain, DEP_CHAIN_PROGRAM_ADDRESS, {
      chain: chainPdas.findChainPda({ owner: key("owner") }),
      chainAuthority: chainPdas.findChainAuthorityPda({ chain }),
    });
    const counter = (await counterPdas.findCounterPda({ owner: key("owner") }))[0];
    await check(fixture.programs.encryptedCounter, ENCRYPTED_COUNTER_PROGRAM_ADDRESS, {
      counter: counterPdas.findCounterPda({ owner: key("owner") }),
      counterAuthority: counterPdas.findCounterAuthorityPda({ counter }),
    });
  });

  for (const override of [undefined, key("program")]) {
    it(`binds initialize_batcher and dispatch defaults and seed encoders to ${override ?? "the default program"}`, async () => {
      const programAddress = override ?? CONFIDENTIAL_BATCHER_PROGRAM_ADDRESS;
      const config = { programAddress };
      // A batch's authority and SPL accounts, as the batcher derives them from the batch.
      const derived = async (batch: (typeof input)["batcher"]) => ({
        authority: (await batcherPdas.findBatchAuthorityPda({ batch: address(batch) }, config))[0],
        joinUnderlying: (await batcherPdas.findBatchJoinUnderlyingPda({ batch: address(batch) }, config))[0],
        payoutUnderlying: (await batcherPdas.findBatchPayoutUnderlyingPda({ batch: address(batch) }, config))[0],
      });
      const shared = {
        payer: createNoopSigner(key("payer")),
        joinConfidentialMint: key("mint"),
        payoutConfidentialMint: key("mint"),
        joinUnderlyingMint: key("mint"),
        payoutUnderlyingMint: key("mint"),
        transientStore: key("authority"),
        instructions: address("Sysvar1nstructions1111111111111111111111111"),
        confidentialTokenEventAuthority: key("authority"),
        authorityFundingLamports: 0n,
      };

      // initialize_batcher opens batch 0.
      const batch0 = (await batcherPdas.findBatchPda({ batcher: key("batcher"), index: 0n }, config))[0];
      const initialize = parseInitializeBatcherInstruction(await getInitializeBatcherInstructionAsync({
        ...shared,
        batcher: createNoopSigner(key("batcher")), vault: key("vault"),
        batchJoinTokenAccount: key("tokenAccount"), batchJoinBalanceStore: key("authority"),
        batchPayoutTokenAccount: key("tokenAccount"), batchPayoutBalanceStore: key("authority"),
        minBatchAgeSecs: 0n, settleDeadlineSecs: 1n, direction: BatchDirection.Deposit,
      }, config));
      const batch0Derived = await derived(batch0);
      expect(initialize.programAddress).toBe(programAddress);
      expect(initialize.accounts.batch.address).toBe(batch0);
      expect(initialize.accounts.batchAuthority.address).toBe(batch0Derived.authority);
      expect(initialize.accounts.batchJoinUnderlying.address).toBe(batch0Derived.joinUnderlying);
      expect(initialize.accounts.batchPayoutUnderlying.address).toBe(batch0Derived.payoutUnderlying);

      // dispatch opens the next batch; its authority and SPL accounts derive from that batch.
      const batch = (await batcherPdas.findBatchPda({ batcher: key("batcher"), index }, config))[0];
      const next = (await batcherPdas.findBatchPda({ batcher: key("batcher"), index: index + 1n }, config))[0];
      const dispatch = parseDispatchInstruction(await getDispatchInstructionAsync({
        ...shared,
        batcher: key("batcher"), batch, nextBatch: next,
        batchAuthorityAta: key("authority"), totalSupplyAuthority: key("authority"),
        batchJoinTokenAccount: key("tokenAccount"), batchBalanceStore: key("authority"),
        totalSupplyStore: key("authority"), pendingBurn: key("authority"),
        nextBatchJoinTokenAccount: key("tokenAccount"), nextBatchJoinBalanceStore: key("authority"),
        nextBatchPayoutTokenAccount: key("tokenAccount"), nextBatchPayoutBalanceStore: key("authority"),
      }, config));
      const nextDerived = await derived(next);
      expect(dispatch.programAddress).toBe(programAddress);
      expect(dispatch.accounts.batchAuthority.address).toBe((await derived(batch)).authority);
      expect(dispatch.accounts.nextBatchAuthority.address).toBe(nextDerived.authority);
      expect(dispatch.accounts.nextBatchJoinUnderlying.address).toBe(nextDerived.joinUnderlying);
      expect(dispatch.accounts.nextBatchPayoutUnderlying.address).toBe(nextDerived.payoutUnderlying);
      expect(dispatch.accounts.hostConfig.address).toBe((await host.findHostConfigPda())[0]);
      expect(dispatch.accounts.zamaEventAuthority.address).toBe((await host.findEventAuthorityPda())[0]);
      expect(await getProgramDerivedAddress({ programAddress, seeds: batcherPdas.getBatchAuthorityPdaSeeds({ batch: next }) }))
        .toEqual(await batcherPdas.findBatchAuthorityPda({ batch: next }, config));
    });

    it(`binds vault and specimen defaults to ${override ?? "their default programs"}`, async () => {
      const programAddress = override ?? DEMO_VAULT_PROGRAM_ADDRESS;
      const config = { programAddress };
      const vault = key("vault");
      const instruction = await getInitializeVaultInstructionAsync({ payer: createNoopSigner(key("payer")), vault: createNoopSigner(vault), underlyingMint: key("mint") }, config);
      const parsed = parseInitializeVaultInstruction(instruction).accounts;
      const authority = await vaultPdas.findVaultAuthorityPda({ vault }, config);
      expect(parsed.vaultAuthority.address).toBe(authority[0]);
      expect(parsed.shareMint.address).toBe((await vaultPdas.findShareMintPda({ vault }, config))[0]);
      expect(parsed.vaultTokenAccount.address).toBe((await vaultPdas.findVaultTokenAccountPda({ vault }, config))[0]);
      expect(await getProgramDerivedAddress({ programAddress, seeds: vaultPdas.getVaultAuthorityPdaSeeds({ vault }) })).toEqual(authority);
      const specimenInput = {
        owner: createNoopSigner(key("owner")), encryptedStore: key("authority"), transientStore: key("authority"),
        instructions: address("Sysvar1nstructions1111111111111111111111111"),
      };
      const chainConfig = { programAddress: override ?? DEP_CHAIN_PROGRAM_ADDRESS };
      const chain = (await chainPdas.findChainPda({ owner: key("owner") }, chainConfig))[0];
      const chainAccounts = parseChain(await initializeChain(specimenInput, chainConfig)).accounts;
      expect(chainAccounts.chain.address).toBe(chain);
      expect(chainAccounts.chainAuthority.address).toBe((await chainPdas.findChainAuthorityPda({ chain }, chainConfig))[0]);
      expect(chainAccounts.hostConfig.address).toBe((await host.findHostConfigPda())[0]);
      expect(chainAccounts.zamaEventAuthority.address).toBe((await host.findEventAuthorityPda())[0]);
      const counterConfig = { programAddress: override ?? ENCRYPTED_COUNTER_PROGRAM_ADDRESS };
      const counter = (await counterPdas.findCounterPda({ owner: key("owner") }, counterConfig))[0];
      const counterAccounts = parseCounter(await initializeCounter(specimenInput, counterConfig)).accounts;
      expect(counterAccounts.counter.address).toBe(counter);
      expect(counterAccounts.counterAuthority.address).toBe((await counterPdas.findCounterAuthorityPda({ counter }, counterConfig))[0]);
      expect(counterAccounts.hostConfig.address).toBe((await host.findHostConfigPda())[0]);
      expect(counterAccounts.zamaEventAuthority.address).toBe((await host.findEventAuthorityPda())[0]);
    });
  }
});
