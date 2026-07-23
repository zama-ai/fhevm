import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';

const sendAndConfirm = vi.hoisted(() => vi.fn());
vi.mock('@solana/kit', async (importOriginal) => ({
  ...(await importOriginal<typeof import('@solana/kit')>()),
  sendAndConfirmTransactionFactory: () => sendAndConfirm,
}));

const certificate = vi.hoisted(() => vi.fn());
vi.mock('../actions/publicDecryptCertificate.js', () => ({ publicDecryptCertificate: certificate }));

import {
  address,
  generateKeyPairSigner,
  getBase64Encoder,
  getCompiledTransactionMessageDecoder,
  getTransactionDecoder,
  type Address,
} from '@solana/kit';
import { base58 } from '@scure/base';

import { settleBatch, type SolanaVaultSettleAccounts, type SolanaVaultSettleParameters } from './settleBatch.js';
import { burnRedemptionAddress } from './internal/batcherPdas.js';
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

function accounts(): SolanaVaultSettleAccounts {
  return {
    batcher: addr(2),
    batch: addr(3),
    joinConfidentialMint: addr(4),
    joinUnderlyingMint: addr(5),
    joinMintVaultUnderlying: addr(6),
    joinMintVaultAuthority: addr(7),
    hostConfig: addr(8),
    kmsContext: addr(9),
    vault: addr(10),
    vaultAuthority: addr(11),
    vaultTokenAccount: addr(12),
    payoutConfidentialMint: addr(13),
    payoutUnderlyingMint: addr(14),
    batchPayoutTokenAccount: addr(15),
    payoutMintVaultUnderlying: addr(16),
    payoutMintVaultAuthority: addr(17),
    payoutComputeSigner: addr(18),
    payoutTotalSupplyAuthority: addr(19),
    batchPayoutBalanceValue: addr(20),
    payoutTotalSupplyValue: addr(21),
  };
}

/** Every settle account we know up front — the set the open_batch ALT would hold (no payer, no redemption_record). */
function lookupTableAddresses(a: SolanaVaultSettleAccounts): Address[] {
  return Object.values(a);
}

function claim(cleartext: string) {
  return {
    handle: `0x${hex(BURNED_HANDLE)}`,
    abiEncodedCleartext: cleartext,
    signatures: [hex(new Uint8Array(65).fill(0x11))],
    extraData: '0x00',
    inclusionProof: { leafIndex: 0n, siblings: [] as Uint8Array[] },
  };
}

function proofServiceResponse(): Response {
  return {
    ok: true,
    status: 200,
    json: async () => ({
      mmr_proof: { leaf_index: 0, siblings: [] },
      leaf_count: 1,
      proof_slot: 1,
      verified: true,
      status: 'verified',
    }),
  } as unknown as Response;
}

async function parameters(): Promise<SolanaVaultSettleParameters> {
  const payer = await generateKeyPairSigner();
  const acc = accounts();
  return {
    rpc: {
      getLatestBlockhash: vi.fn().mockReturnValue({
        send: vi.fn().mockResolvedValue({ value: { blockhash: addr(250), lastValidBlockHeight: 1_000n } }),
      }),
      simulateTransaction: vi.fn().mockReturnValue({ send: vi.fn().mockResolvedValue({ value: { err: null } }) }),
    } as unknown as SolanaVaultSettleParameters['rpc'],
    rpcSubscriptions: {} as SolanaVaultSettleParameters['rpcSubscriptions'],
    chain: {} as never,
    runtime: {} as never,
    proofService: { proofServiceUrl: 'http://proof:8080', retryDelayMs: 0 },
    payer,
    accounts: acc,
    burnedTotalHandle: BURNED_HANDLE,
    contextId: new Uint8Array(32),
    peaks: [],
    leafCount: 1n,
    lookupTableAddress: addr(200),
    lookupTableAddresses: lookupTableAddresses(acc),
    authorityFundingLamports: 5_000_000n,
  };
}

describe('settleBatch', () => {
  beforeEach(() => {
    sendAndConfirm.mockReset().mockResolvedValue(undefined);
    certificate.mockReset();
    vi.stubGlobal('fetch', vi.fn().mockResolvedValue(proofServiceResponse()));
  });
  afterEach(() => vi.unstubAllGlobals());

  it('builds an ALT-aware v0 settle that keeps redemption_record and the fee payer static', async () => {
    certificate.mockResolvedValue(claim(cleartextHex(800n)));
    const params = await parameters();
    await expect(settleBatch(params)).resolves.toEqual(expect.any(String));

    // The proof leg fed the certificate leg: mmrProofBytes + proofSlot from the service.
    expect(certificate).toHaveBeenCalledTimes(1);
    const certParams = certificate.mock.calls[0]![1] as { proofSlot: bigint; mmrProofBytes: Uint8Array };
    expect(certParams.proofSlot).toBe(1n);
    expect(certParams.mmrProofBytes[0]).toBe(0x02); // public-decrypt transport mode

    const simulate = params.rpc.simulateTransaction as unknown as ReturnType<typeof vi.fn>;
    const wire = simulate.mock.calls[0]![0] as string;
    expect(getBase64Encoder().encode(wire).length).toBeLessThanOrEqual(1232);

    const transaction = getTransactionDecoder().decode(getBase64Encoder().encode(wire));
    const compiled = getCompiledTransactionMessageDecoder().decode(transaction.messageBytes);
    expect(compiled.version).toBe(0);

    // The v0 message references the batch's lookup table, and every derivable settle account we
    // handed it was moved into that table.
    const lookups =
      (
        compiled as {
          addressTableLookups?: { lookupTableAddress: Address; writableIndexes: number[]; readonlyIndexes: number[] }[];
        }
      ).addressTableLookups ?? [];
    expect(lookups).toHaveLength(1);
    expect(lookups[0]!.lookupTableAddress).toBe(params.lookupTableAddress);
    const moved = lookups[0]!.writableIndexes.length + lookups[0]!.readonlyIndexes.length;
    expect(moved).toBe(params.lookupTableAddresses.length);

    // redemption_record (derived from the burned handle) and the fee payer stay STATIC — they are
    // never in the table.
    const staticAccounts = compiled.staticAccounts as readonly Address[];
    const redemptionRecord = await burnRedemptionAddress(params.accounts.joinConfidentialMint, BURNED_HANDLE);
    expect(staticAccounts[0]).toBe(params.payer.address); // fee payer is always static account 0
    expect(staticAccounts).toContain(redemptionRecord);
    for (const moved of params.lookupTableAddresses) expect(staticAccounts).not.toContain(moved);

    // The certified 32-byte cleartext was decoded to the u64 settle argument. instructions[0] is
    // the prepended SetComputeUnitLimit; the settle instruction is instructions[1].
    const compiledInstructions = (compiled as unknown as { instructions: { data?: Uint8Array }[] }).instructions;
    const data = getSettleInstructionDataDecoder().decode(compiledInstructions[1]!.data!);
    expect(data.cleartextTotal).toBe(800n);
    expect(data.leafIndex).toBe(0n);
    expect(data.siblings).toEqual([]);
  });

  it('rejects a certified total that does not fit u64 before touching the RPC or sending', async () => {
    certificate.mockResolvedValue(claim(cleartextHex(1n, 0x01))); // a high byte set
    const params = await parameters();
    await expect(settleBatch(params)).rejects.toThrow('exceeds u64');
    expect(params.rpc.simulateTransaction).not.toHaveBeenCalled();
    expect(sendAndConfirm).not.toHaveBeenCalled();
  });

  it('rejects a proof-service leaf count that disagrees with the supplied lineage before any RPC or certificate leg', async () => {
    certificate.mockResolvedValue(claim(cleartextHex(800n)));
    const params = { ...(await parameters()), leafCount: 2n }; // service returns leaf_count 1
    await expect(settleBatch(params)).rejects.toThrow('does not match the lineage leaf count');
    expect(certificate).not.toHaveBeenCalled(); // the check runs before leg 2
    expect(params.rpc.getLatestBlockhash).not.toHaveBeenCalled();
    expect(params.rpc.simulateTransaction).not.toHaveBeenCalled();
    expect(sendAndConfirm).not.toHaveBeenCalled();
  });

  it('rejects a lookup table that wrongly contains redemption_record before any RPC or sending', async () => {
    certificate.mockResolvedValue(claim(cleartextHex(800n)));
    const base = await parameters();
    const redemptionRecord = await burnRedemptionAddress(base.accounts.joinConfidentialMint, BURNED_HANDLE);
    const params = { ...base, lookupTableAddresses: [...base.lookupTableAddresses, redemptionRecord] };
    await expect(settleBatch(params)).rejects.toThrow('must not contain the settle redemption_record');
    expect(params.rpc.getLatestBlockhash).not.toHaveBeenCalled();
    expect(params.rpc.simulateTransaction).not.toHaveBeenCalled();
    expect(sendAndConfirm).not.toHaveBeenCalled();
  });
});
