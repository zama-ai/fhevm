import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';

const sendAndConfirm = vi.hoisted(() => vi.fn());
vi.mock('@solana/kit', async (importOriginal) => ({
  ...(await importOriginal<typeof import('@solana/kit')>()),
  sendAndConfirmTransactionFactory: () => sendAndConfirm,
}));

const publicProof = vi.hoisted(() => vi.fn());
vi.mock('./internal/publicProof.js', () => ({ publicProof }));

const certificate = vi.hoisted(() => vi.fn());
vi.mock('@sdk-src/solana/actions/publicDecryptCertificate.js', () => ({ publicDecryptCertificate: certificate }));

const getCurrentBatch = vi.hoisted(() => vi.fn());
const getEncryptedStore = vi.hoisted(() => vi.fn());
vi.mock('./reads.js', () => ({ getCurrentBatch, getEncryptedStore }));

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
import { getSettleInstructionDataDecoder } from './internal/generated/confidentialBatcher/instructions/settle.js';
import { CLOSE_TRANSIENT_STORE_DISCRIMINATOR } from '@sdk-src/solana/internal/generated/zamaHost/instructions/closeTransientStore.js';
import { ZAMA_HOST_PROGRAM_ADDRESS } from '@sdk-src/solana/internal/generated/zamaHost/programAddress.js';

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

async function options(overrides: { burnedHandle?: Uint8Array; extraLeaves?: number } = {}): Promise<{
  chain: FhevmSolanaChain;
  keeper: Awaited<ReturnType<typeof generateKeyPairSigner>>;
  opts: SolanaVaultSettleOptions;
}> {
  const keeper = await generateKeyPairSigner();
  const burnedHandle = overrides.burnedHandle ?? BURNED_HANDLE;
  const addresses = await deriveBatchAddresses(roots(), 0n);

  getCurrentBatch.mockResolvedValue({ index: 0n, addresses, state: { burnedTotalHandle: burnedHandle } });
  if (overrides.extraLeaves) publicProof.mockRejectedValue(new Error('public proof does not match on-chain state'));
  else publicProof.mockResolvedValue({ leafIndex: 3n, siblings: [new Uint8Array(32), new Uint8Array(32)] });

  const opts: SolanaVaultSettleOptions = {
    rpc: {
      getLatestBlockhash: vi.fn().mockReturnValue({
        send: vi.fn().mockResolvedValue({ value: { blockhash: addr(250), lastValidBlockHeight: 1_000n } }),
      }),
      simulateTransaction: vi.fn().mockReturnValue({ send: vi.fn().mockResolvedValue({ value: { err: null } }) }),
    } as unknown as SolanaVaultSettleOptions['rpc'],
    rpcSubscriptions: {} as SolanaVaultSettleOptions['rpcSubscriptions'],
    runtime: {} as never,
    proofService: { url: 'http://listener-proof-endpoint', apiKey: 'test' },
    roots: roots(),
    contextId: new Uint8Array(32),
    lookupTableAddress: addr(200),
    authorityFundingLamports: 5_000_000n,
  };
  return { chain: { id: 9223372036854788153n, fhevm: { relayerUrl: 'http://relayer:3000' } }, keeper, opts };
}

describe('settleBatch', () => {
  beforeEach(() => {
    sendAndConfirm.mockReset().mockResolvedValue(undefined);
    certificate.mockReset();
    getCurrentBatch.mockReset();
    getEncryptedStore.mockReset();
    publicProof.mockReset();
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
      encryptedStore: base58.decode(batchAddresses.batchBurnedAmountStore),
      options: undefined,
    });

    const simulate = opts.rpc.simulateTransaction as unknown as ReturnType<typeof vi.fn>;
    const wire = simulate.mock.calls[0]![0] as string;
    expect(getBase64Encoder().encode(wire).length).toBeLessThanOrEqual(1232);

    const transaction = getTransactionDecoder().decode(getBase64Encoder().encode(wire));
    const compiled = getCompiledTransactionMessageDecoder().decode(transaction.messageBytes);
    if (compiled.version !== 0) throw new Error('Expected an ALT-aware v0 transaction');

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
    // The static set is closed at 10: the fee payer, transient store, instructions sysvar,
    // compute-budget program and fixed program IDs. Pinning the count makes
    // growth a deliberate edit with a fresh look at the 1232-byte budget.
    expect(staticAccounts).toHaveLength(10);
    // And nothing is paid for twice: an address provisioned into the table must not also sit in
    // the static set.
    expect(staticAccounts.filter((address) => provisioned.includes(address))).toEqual([]);

    // The certified 32-byte cleartext was decoded to the u64 settle argument, and the locally built
    // proof of the second leaf rode along. instructions[0] is the prepended SetComputeUnitLimit; the
    // open is instructions[1] and settle is instructions[2].
    const compiledInstructions = compiled.instructions;
    expect(compiledInstructions).toHaveLength(4);
    expect(compiledInstructions.at(-1)!.data).toEqual(CLOSE_TRANSIENT_STORE_DISCRIMINATOR);
    expect(staticAccounts[compiledInstructions.at(-1)!.programAddressIndex]).toBe(ZAMA_HOST_PROGRAM_ADDRESS);
    const data = getSettleInstructionDataDecoder().decode(compiledInstructions[2]!.data!);
    expect(data.cleartextTotal).toBe(800n);
    expect(data.leafIndex).toBe(3n);
    expect(data.siblings).toHaveLength(2);
  });

  it.each([{ threshold: 1, depth: 2 }, { threshold: 7, depth: 0 }, { threshold: 7, depth: 2 }, { threshold: 7, depth: 5 }])('fits $threshold signatures and $depth proof siblings with the provisioned table', async ({ threshold, depth }) => {
    const { chain, keeper, opts } = await options();
    certificate.mockResolvedValue({
      ...claim(cleartextHex(800n)),
      signatures: Array.from({ length: threshold }, () => hex(new Uint8Array(65).fill(0x11))),
    });
    publicProof.mockResolvedValue({ leafIndex: 0n, siblings: Array.from({ length: depth }, () => new Uint8Array(32)) });
    await settleBatch(chain, keeper, opts);
    const simulate = opts.rpc.simulateTransaction as unknown as ReturnType<typeof vi.fn>;
    const bytes = getBase64Encoder().encode(simulate.mock.calls[0]![0] as string);
    expect(bytes.length).toBeLessThanOrEqual(1232);
  });

  it('rejects an oversized certificate/proof before simulation or send', async () => {
    const { chain, keeper, opts } = await options();
    certificate.mockResolvedValue({
      ...claim(cleartextHex(800n)),
      signatures: Array.from({ length: 7 }, () => hex(new Uint8Array(65).fill(0x11))),
    });
    publicProof.mockResolvedValue({ leafIndex: 0n, siblings: Array.from({ length: 6 }, () => new Uint8Array(32)) });
    await expect(settleBatch(chain, keeper, opts)).rejects.toThrow('exceeds limit of 1232 bytes');
    expect(opts.rpc.simulateTransaction).not.toHaveBeenCalled();
    expect(sendAndConfirm).not.toHaveBeenCalled();
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

  it('fetches the proof after the certificate and rejects invalid evidence before sending', async () => {
    certificate.mockResolvedValue(claim(cleartextHex(800n)));
    const { chain, keeper, opts } = await options({ extraLeaves: 1 });
    await expect(settleBatch(chain, keeper, opts)).rejects.toThrow('does not match');
    expect(certificate).toHaveBeenCalledTimes(1);
    expect(publicProof.mock.invocationCallOrder[0]).toBeGreaterThan(certificate.mock.invocationCallOrder[0]!);
    expect(opts.rpc.getLatestBlockhash).not.toHaveBeenCalled();
    expect(sendAndConfirm).not.toHaveBeenCalled();
  });
});
