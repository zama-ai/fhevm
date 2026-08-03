import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';

const sendAndConfirm = vi.hoisted(() => vi.fn());
vi.mock('@solana/kit', async (importOriginal) => ({
  ...(await importOriginal<typeof import('@solana/kit')>()),
  sendAndConfirmTransactionFactory: () => sendAndConfirm,
}));

const certificate = vi.hoisted(() => vi.fn());
vi.mock('@sdk-src/solana/actions/publicDecryptCertificate.js', () => ({ publicDecryptCertificate: certificate }));

const getCurrentBatch = vi.hoisted(() => vi.fn());
const getEncryptedValueState = vi.hoisted(() => vi.fn());
vi.mock('./reads.js', () => ({ getCurrentBatch, getEncryptedValueState }));

import {
  address,
  generateKeyPairSigner,
  getBase64Encoder,
  getCompiledTransactionMessageDecoder,
  getTransactionDecoder,
  type Address,
} from '@solana/kit';
import { base58 } from '@scure/base';

import { settleBatch, type SolanaVaultSettleOptions } from './settleBatch.js';
import {
  deriveBatchAddresses,
  deriveSettleAccounts,
  deriveSettleLookupTableAddresses,
  type VaultDemoRoots,
} from './derive.js';
import type { FhevmSolanaChain } from '@sdk-src/core/types/fhevmSolanaChain.js';
import type { SolanaProofServiceConfig } from './internal/proofService.js';
import { getSettleInstructionDataDecoder } from './internal/generated/confidentialBatcher/instructions/settle.js';

function addr(fill: number): Address {
  return address(base58.encode(new Uint8Array(32).fill(fill)));
}

function hex(bytes: Uint8Array): string {
  return Array.from(bytes, (b) => b.toString(16).padStart(2, '0')).join('');
}

/** A 32-byte big-endian uint256 carrying `value` in its low 8 bytes. */
function cleartextHex(value: bigint, highByte0 = 0): string {
  const bytes = new Uint8Array(32);
  bytes[0] = highByte0;
  new DataView(bytes.buffer).setBigUint64(24, value, false);
  return hex(bytes);
}

const BURNED_HANDLE = new Uint8Array(32).fill(0x92);

function roots(): VaultDemoRoots {
  return {
    batcherProgram: addr(30),
    tokenProgram: addr(31),
    vaultProgram: addr(32),
    hostProgram: addr(33),
    batcher: addr(2),
    vault: addr(10),
    joinConfidentialMint: addr(4),
    payoutConfidentialMint: addr(13),
    joinUnderlyingMint: addr(5),
    payoutUnderlyingMint: addr(14),
    hostConfig: addr(8),
    kmsContext: addr(9),
  };
}

function claim(cleartext: string, siblings: Uint8Array[] = []) {
  return {
    handle: `0x${hex(BURNED_HANDLE)}`,
    abiEncodedCleartext: cleartext,
    signatures: [hex(new Uint8Array(65).fill(0x11))],
    extraData: '0x00',
    inclusionProof: { leafIndex: 0n, siblings },
  };
}

function proofServiceResponse(): Response {
  return {
    ok: true,
    status: 200,
    json: async () => ({
      mmr_proof: { leaf_index: 0, siblings: [] },
      leaf_index: 0,
      leaf_count: 1,
      rpc_context_slot: 1234,
      commitment: 'confirmed',
      proof_format_version: 'v1',
      verified: true,
      status: 'verified',
    }),
  } as unknown as Response;
}

async function options(overrides: { burnedHandle?: Uint8Array; leafCount?: bigint } = {}): Promise<{
  chain: FhevmSolanaChain;
  proofConfig: SolanaProofServiceConfig;
  keeper: Awaited<ReturnType<typeof generateKeyPairSigner>>;
  opts: SolanaVaultSettleOptions;
}> {
  const keeper = await generateKeyPairSigner();
  const demoRoots = roots();
  const addresses = await deriveBatchAddresses(demoRoots, 0n);
  const burnedHandle = overrides.burnedHandle ?? BURNED_HANDLE;

  getCurrentBatch.mockResolvedValue({
    index: 0n,
    addresses,
    state: { burnedTotalHandle: burnedHandle },
  });
  getEncryptedValueState.mockResolvedValue({
    currentHandle: new Uint8Array(32),
    leafCount: overrides.leafCount ?? 1n,
    peaks: [] as Uint8Array[],
  });

  const opts: SolanaVaultSettleOptions = {
    rpc: {
      getLatestBlockhash: vi.fn().mockReturnValue({
        send: vi.fn().mockResolvedValue({ value: { blockhash: addr(250), lastValidBlockHeight: 1_000n } }),
      }),
      simulateTransaction: vi.fn().mockReturnValue({ send: vi.fn().mockResolvedValue({ value: { err: null } }) }),
    } as unknown as SolanaVaultSettleOptions['rpc'],
    rpcSubscriptions: {} as SolanaVaultSettleOptions['rpcSubscriptions'],
    runtime: {} as never,
    roots: demoRoots,
    contextId: new Uint8Array(32),
    lookupTableAddress: addr(200),
    authorityFundingLamports: 5_000_000n,
  };
  return {
    chain: { id: 9223372036854788153n, fhevm: { relayerUrl: 'http://relayer:3000', acl: { domainKeys: [] } } },
    proofConfig: { proofServiceUrl: 'http://proof:8080', retryDelayMs: 0 },
    keeper,
    opts,
  };
}

describe('settleBatch', () => {
  beforeEach(() => {
    sendAndConfirm.mockReset().mockResolvedValue(undefined);
    certificate.mockReset();
    getCurrentBatch.mockReset();
    getEncryptedValueState.mockReset();
    vi.stubGlobal('fetch', vi.fn().mockResolvedValue(proofServiceResponse()));
  });
  afterEach(() => vi.unstubAllGlobals());

  it('resolves the current batch and builds an ALT-aware v0 settle with pending_burn in the table', async () => {
    certificate.mockResolvedValue(claim(cleartextHex(800n)));
    const { chain, proofConfig, keeper, opts } = await options();
    await expect(settleBatch(chain, proofConfig, keeper, opts)).resolves.toEqual(expect.any(String));

    // The batch was resolved from chain state, not supplied.
    expect(getCurrentBatch).toHaveBeenCalledTimes(1);
    // The proof phase fed the certificate phase: mmrProofBytes + proofSlot from the service.
    expect(certificate).toHaveBeenCalledTimes(1);
    const certParams = certificate.mock.calls[0]![1] as { proofSlot: bigint; mmrProofBytes: Uint8Array };
    expect(certParams.proofSlot).toBe(1n);
    expect(certParams.mmrProofBytes[0]).toBe(0x02); // public-decrypt transport mode

    const simulate = opts.rpc.simulateTransaction as unknown as ReturnType<typeof vi.fn>;
    const wire = simulate.mock.calls[0]![0] as string;
    expect(getBase64Encoder().encode(wire).length).toBeLessThanOrEqual(1232);

    const transaction = getTransactionDecoder().decode(getBase64Encoder().encode(wire));
    const compiled = getCompiledTransactionMessageDecoder().decode(transaction.messageBytes);
    expect(compiled.version).toBe(0);

    // The v0 message references the batch's lookup table, and every derivable settle account moved
    // into it.
    const lookups =
      (
        compiled as {
          addressTableLookups?: { lookupTableAddress: Address; writableIndexes: number[]; readonlyIndexes: number[] }[];
        }
      ).addressTableLookups ?? [];
    expect(lookups).toHaveLength(1);
    expect(lookups[0]!.lookupTableAddress).toBe(opts.lookupTableAddress);

    // One table composition, bound end to end: the addresses the demo provisions into the table at
    // open_batch (openBatchForBatcher -> deriveSettleLookupTableAddresses) are exactly the accounts
    // settle then resolves through it. Both sides call settleAccountsToLookupTableAddresses, and the
    // v0 message's table indices are checked against that list here, so adding a settle account to
    // one side without the other fails this test instead of failing live with a missing table entry.
    const batchAddresses = await deriveBatchAddresses(opts.roots, 0n);
    const provisioned = await deriveSettleLookupTableAddresses(opts.roots, batchAddresses);
    const usedIndexes = [...lookups[0]!.writableIndexes, ...lookups[0]!.readonlyIndexes];
    expect(Math.max(...usedIndexes)).toBeLessThan(provisioned.length);
    expect(new Set(usedIndexes.map((index) => provisioned[index]!))).toEqual(new Set(provisioned));

    // pending_burn rides in the ALT (`burn_id = batch.key()`); only the fee payer (plus program /
    // event authorities outside the provisioned list) stays static.
    const accounts = await deriveSettleAccounts(opts.roots, batchAddresses);
    const staticAccounts = compiled.staticAccounts as readonly Address[];
    expect(staticAccounts[0]).toBe(keeper.address); // fee payer is always static account 0
    expect(staticAccounts).not.toContain(accounts.pendingBurn);
    expect(provisioned).toContain(accounts.pendingBurn);
    // The static set is closed at 13: the fee payer, the program ids settle passes as accounts
    // (system, SPL token, the CPI targets) and the event-CPI authorities. Pinning the count makes
    // growth a deliberate edit with a fresh look at the 1232-byte budget (the sibling-depth test
    // below is what that budget costs today).
    expect(staticAccounts).toHaveLength(13);
    // And nothing is paid for twice: an address provisioned into the table must not also sit in
    // the static set.
    expect(staticAccounts.filter((address) => provisioned.includes(address))).toEqual([]);

    // The certified 32-byte cleartext was decoded to the u64 settle argument. instructions[0] is the
    // prepended SetComputeUnitLimit; the settle instruction is instructions[1].
    const compiledInstructions = (compiled as unknown as { instructions: { data?: Uint8Array }[] }).instructions;
    const data = getSettleInstructionDataDecoder().decode(compiledInstructions[1]!.data!);
    expect(data.cleartextTotal).toBe(800n);
    expect(data.leafIndex).toBe(0n);
    expect(data.siblings).toEqual([]);
  });

  it('keeps a realistic-depth settle (14 MMR siblings) within the 1232-byte v0 wire limit', async () => {
    // A live settle carries a real MMR inclusion proof: one 32-byte sibling per mountain level, the
    // term that actually grows in the settle instruction data (`siblings: [...claim.inclusionProof.
    // siblings]`). The happy-path test above used an EMPTY proof, which would not have caught an ALT
    // that left the tx too full to absorb a real proof. This re-asserts the size bound with a deep
    // proof.
    //
    // 14 is the MEASURED ceiling for the current ALT design: the settle v0 message keeps the fee
    // payer and the two event-CPI authorities static plus the invoked program ids; pending_burn is
    // in the ALT. At 14 siblings the wire stays under 1232. This is far above any realistic depth
    // here — the burned_amount encrypted value account is a PER-BATCH EncryptedValue whose MMR
    // gains ~one leaf per batch (depth ~0-1), so 14 is generous headroom, not a live expectation.
    const siblings = Array.from({ length: 14 }, (_, level) => new Uint8Array(32).fill(0xa0 + level));
    certificate.mockResolvedValue(claim(cleartextHex(800n), siblings));
    const { chain, proofConfig, keeper, opts } = await options();
    await expect(settleBatch(chain, proofConfig, keeper, opts)).resolves.toEqual(expect.any(String));

    const simulate = opts.rpc.simulateTransaction as unknown as ReturnType<typeof vi.fn>;
    const wire = simulate.mock.calls[0]![0] as string;
    expect(getBase64Encoder().encode(wire).length).toBeLessThanOrEqual(1232);

    // The proof really did ride in the settle instruction (guards against the siblings being dropped,
    // which would make the size check meaningless).
    const transaction = getTransactionDecoder().decode(getBase64Encoder().encode(wire));
    const compiled = getCompiledTransactionMessageDecoder().decode(transaction.messageBytes);
    const compiledInstructions = (compiled as unknown as { instructions: { data?: Uint8Array }[] }).instructions;
    const data = getSettleInstructionDataDecoder().decode(compiledInstructions[1]!.data!);
    expect(data.siblings).toHaveLength(14);
  });

  it('rejects a batch that has not been dispatched (zero burned handle) before any phase', async () => {
    certificate.mockResolvedValue(claim(cleartextHex(800n)));
    const { chain, proofConfig, keeper, opts } = await options({ burnedHandle: new Uint8Array(32) });
    await expect(settleBatch(chain, proofConfig, keeper, opts)).rejects.toThrow('no burned total handle');
    expect(certificate).not.toHaveBeenCalled();
    expect(sendAndConfirm).not.toHaveBeenCalled();
  });

  it('rejects a certified total that does not fit u64 before touching the RPC or sending', async () => {
    certificate.mockResolvedValue(claim(cleartextHex(1n, 0x01))); // a high byte set
    const { chain, proofConfig, keeper, opts } = await options();
    await expect(settleBatch(chain, proofConfig, keeper, opts)).rejects.toThrow('exceeds u64');
    expect(opts.rpc.simulateTransaction).not.toHaveBeenCalled();
    expect(sendAndConfirm).not.toHaveBeenCalled();
  });

  it('rejects a proof-service leaf count that disagrees with the on-chain encrypted value account before the certificate phase', async () => {
    certificate.mockResolvedValue(claim(cleartextHex(800n)));
    const { chain, proofConfig, keeper, opts } = await options({ leafCount: 2n }); // service returns leaf_count 1
    await expect(settleBatch(chain, proofConfig, keeper, opts)).rejects.toThrow(
      'does not match the on-chain encrypted value account',
    );
    expect(certificate).not.toHaveBeenCalled();
    expect(opts.rpc.getLatestBlockhash).not.toHaveBeenCalled();
    expect(sendAndConfirm).not.toHaveBeenCalled();
  });
});
