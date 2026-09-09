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

import { mmrVerify, publicDecryptLeafCommitment, reconstructSolanaEncryptedValueAccount } from '@sdk-src/solana/proof.js';
import { buildBurnedAmountPublicProof, burnedAmountLeafHistory, settleBatch, type SolanaVaultSettleOptions } from './settleBatch.js';
import {
  deriveBatchAddresses,
  deriveSettleAccounts,
  deriveSettleLookupTableAddresses,
  type VaultDemoRoots,
} from './derive.js';
import type { FhevmSolanaChain } from '@sdk-src/core/types/fhevmSolanaChain.js';
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

function claim(cleartext: string) {
  return {
    handle: `0x${hex(BURNED_HANDLE)}`,
    abiEncodedCleartext: cleartext,
    signatures: [hex(new Uint8Array(65).fill(0x11))],
    extraData: '0x00',
  };
}

/** The live account state the batcher's one burn leaves behind, for the fixture batch. */
async function liveBurnedAccount(burnedHandle: Uint8Array = BURNED_HANDLE, extraLeaves = 0) {
  const addresses = await deriveBatchAddresses(roots(), 0n);
  const history = burnedAmountLeafHistory(burnedHandle, addresses.batchAuthority);
  const events = [
    ...history.events,
    ...Array.from({ length: extraLeaves }, () => ({ kind: 'markedPublic' as const, handle: burnedHandle })),
  ];
  const rebuilt = reconstructSolanaEncryptedValueAccount(base58.decode(addresses.batchBurnedAmountValue), events);
  return { addresses, state: { currentHandle: burnedHandle, leafCount: rebuilt.leafCount, peaks: rebuilt.peaks } };
}

async function options(overrides: { burnedHandle?: Uint8Array; extraLeaves?: number } = {}): Promise<{
  chain: FhevmSolanaChain;
  keeper: Awaited<ReturnType<typeof generateKeyPairSigner>>;
  opts: SolanaVaultSettleOptions;
}> {
  const keeper = await generateKeyPairSigner();
  const burnedHandle = overrides.burnedHandle ?? BURNED_HANDLE;
  const { addresses, state } = await liveBurnedAccount(burnedHandle, overrides.extraLeaves ?? 0);

  getCurrentBatch.mockResolvedValue({ index: 0n, addresses, state: { burnedTotalHandle: burnedHandle } });
  getEncryptedValueState.mockResolvedValue(state);

  const opts: SolanaVaultSettleOptions = {
    rpc: {
      getLatestBlockhash: vi.fn().mockReturnValue({
        send: vi.fn().mockResolvedValue({ value: { blockhash: addr(250), lastValidBlockHeight: 1_000n } }),
      }),
      simulateTransaction: vi.fn().mockReturnValue({ send: vi.fn().mockResolvedValue({ value: { err: null } }) }),
    } as unknown as SolanaVaultSettleOptions['rpc'],
    rpcSubscriptions: {} as SolanaVaultSettleOptions['rpcSubscriptions'],
    runtime: {} as never,
    roots: roots(),
    contextId: new Uint8Array(32),
    lookupTableAddress: addr(200),
    authorityFundingLamports: 5_000_000n,
  };
  return { chain: { id: 9223372036854788153n, fhevm: { relayerUrl: 'http://relayer:3000' } }, keeper, opts };
}

describe('the burned amount leaf history', () => {
  it('is one allow for the batch authority then the public leaf, and its proof verifies against the peaks', async () => {
    const { addresses, state } = await liveBurnedAccount();
    const proof = buildBurnedAmountPublicProof(
      addresses.batchBurnedAmountValue,
      state,
      BURNED_HANDLE,
      addresses.batchAuthority,
    );
    expect(proof.leafIndex).toBe(1n);
    expect(state.leafCount).toBe(2n);
    const commitment = publicDecryptLeafCommitment(
      base58.decode(addresses.batchBurnedAmountValue),
      proof.leafIndex,
      BURNED_HANDLE,
    );
    expect(mmrVerify(state.peaks, state.leafCount, commitment, proof)).toBe(true);
  });

  it('refuses a live account whose leaves are not the ones the batcher writes', async () => {
    const { addresses, state } = await liveBurnedAccount(BURNED_HANDLE, 1);
    expect(() =>
      buildBurnedAmountPublicProof(addresses.batchBurnedAmountValue, state, BURNED_HANDLE, addresses.batchAuthority),
    ).toThrow('do not match');
  });
});

describe('settleBatch', () => {
  beforeEach(() => {
    sendAndConfirm.mockReset().mockResolvedValue(undefined);
    certificate.mockReset();
    getCurrentBatch.mockReset();
    getEncryptedValueState.mockReset();
  });
  afterEach(() => vi.unstubAllGlobals());

  it('resolves the current batch and builds an ALT-aware v0 settle with pending_burn in the table', async () => {
    certificate.mockResolvedValue(claim(cleartextHex(800n)));
    const { chain, keeper, opts } = await options();
    await expect(settleBatch(chain, keeper, opts)).resolves.toEqual(expect.any(String));

    // The batch was resolved from chain state, not supplied.
    expect(getCurrentBatch).toHaveBeenCalledTimes(1);
    // The certificate names the handle and the account, nothing else: no proof travels.
    expect(certificate).toHaveBeenCalledTimes(1);
    const batchAddresses = await deriveBatchAddresses(opts.roots, 0n);
    expect(certificate.mock.calls[0]![1]).toEqual({
      handle: `0x${hex(BURNED_HANDLE)}`,
      contextId: opts.contextId,
      encryptedValueAccount: base58.decode(batchAddresses.batchBurnedAmountValue),
      options: undefined,
    });

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

    // The compiled message must reference every address in the provisioned list exactly once.
    // Provisioning and settlement share the same derivation helper, so this assertion checks lookup
    // index coverage; derive.test.ts separately pins the helper's field set and ordering.
    const provisioned = await deriveSettleLookupTableAddresses(opts.roots, batchAddresses);
    const usedIndexes = [...lookups[0]!.writableIndexes, ...lookups[0]!.readonlyIndexes];
    expect(Math.max(...usedIndexes)).toBeLessThan(provisioned.length);
    expect(new Set(usedIndexes.map((index) => provisioned[index]!))).toEqual(new Set(provisioned));

    // pending_burn rides in the ALT; only the fee payer (plus program /
    // event authorities outside the provisioned list) stays static.
    const accounts = await deriveSettleAccounts(opts.roots, batchAddresses);
    const staticAccounts = compiled.staticAccounts as readonly Address[];
    expect(staticAccounts[0]).toBe(keeper.address); // fee payer is always static account 0
    expect(staticAccounts).not.toContain(accounts.pendingBurn);
    expect(provisioned).toContain(accounts.pendingBurn);
    // The static set is closed at 13: the fee payer, the program ids settle passes as accounts
    // (system, SPL token, the CPI targets) and the event-CPI authorities. Pinning the count makes
    // growth a deliberate edit with a fresh look at the 1232-byte budget.
    expect(staticAccounts).toHaveLength(13);
    // And nothing is paid for twice: an address provisioned into the table must not also sit in
    // the static set.
    expect(staticAccounts.filter((address) => provisioned.includes(address))).toEqual([]);

    // The certified 32-byte cleartext was decoded to the u64 settle argument, and the locally built
    // proof of the second leaf rode along. instructions[0] is the prepended SetComputeUnitLimit; the
    // settle instruction is instructions[1].
    const compiledInstructions = (compiled as unknown as { instructions: { data?: Uint8Array }[] }).instructions;
    const data = getSettleInstructionDataDecoder().decode(compiledInstructions[1]!.data!);
    expect(data.cleartextTotal).toBe(800n);
    expect(data.leafIndex).toBe(1n);
    expect(data.siblings).toHaveLength(1);
  });

  it('rejects a batch that has not been dispatched (zero burned handle) before any phase', async () => {
    certificate.mockResolvedValue(claim(cleartextHex(800n)));
    const { chain, keeper, opts } = await options({ burnedHandle: new Uint8Array(32) });
    await expect(settleBatch(chain, keeper, opts)).rejects.toThrow('no burned total handle');
    expect(certificate).not.toHaveBeenCalled();
    expect(sendAndConfirm).not.toHaveBeenCalled();
  });

  it('rejects a certified total that does not fit u64 before touching the RPC or sending', async () => {
    certificate.mockResolvedValue(claim(cleartextHex(1n, 0x01))); // a high byte set
    const { chain, keeper, opts } = await options();
    await expect(settleBatch(chain, keeper, opts)).rejects.toThrow('exceeds u64');
    expect(opts.rpc.simulateTransaction).not.toHaveBeenCalled();
    expect(sendAndConfirm).not.toHaveBeenCalled();
  });

  it('rejects a live burned account that disagrees with the batcher history before the certificate phase', async () => {
    certificate.mockResolvedValue(claim(cleartextHex(800n)));
    const { chain, keeper, opts } = await options({ extraLeaves: 1 });
    await expect(settleBatch(chain, keeper, opts)).rejects.toThrow('do not match');
    expect(certificate).not.toHaveBeenCalled();
    expect(opts.rpc.getLatestBlockhash).not.toHaveBeenCalled();
    expect(sendAndConfirm).not.toHaveBeenCalled();
  });
});
