import { describe, expect, it } from "bun:test";
import { address, createNoopSigner, type ProgramDerivedAddress } from "@solana/kit";
import * as host from "@fhevm/solana-zama-host";
import * as token from "@fhevm/confidential-token";
import fixture from "../../../../solana/test-fixtures/pda/pda_v1.json";

const input = fixture.inputs;
const key = (name: Exclude<keyof typeof input, "contextId">) => address(input[name]);

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
  it("pins available zama-host recipes through generated finders", async () => {
    expect(key("program")).toBe(token.CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS);
    expect(key("scope")).toBe(key("mint"));
    // The generic host HCU-meter finder is deferred; the Rust golden still pins that recipe.
    const { hcuBlockMeter: _deferred, ...pdas } = fixture.programs.zamaHost.pdas;
    await check({ ...fixture.programs.zamaHost, pdas }, host.ZAMA_HOST_PROGRAM_ADDRESS, {
      hostConfig: host.findHostConfigPda(),
      kmsContext: host.findKmsContextPda({ contextId: new Uint8Array(input.contextId) }),
      randNonce: host.findRandNoncePda(),
      encryptedStore: host.findEncryptedStorePda({ program: key("program"), authority: key("authority"), scope: key("scope") }),
      transientStore: host.findTransientStorePda({ payer: key("payer") }),
      delegationRecord: host.findDelegationRecordPda({ delegator: key("delegator"), delegate: key("delegate"), program: key("program"), scope: key("scope") }),
      invalidation: host.findInvalidationPda({ user: key("user") }),
      denyScopeRecord: host.findDenyScopeRecordPda({ appProgram: key("program"), scope: key("scope") }),
      hcuTrustedAppRecord: host.findHcuTrustedAppRecordPda({ appProgram: key("program"), scope: key("scope") }),
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
