import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';

const sendAndConfirm = vi.hoisted(() => vi.fn());
vi.mock('@solana/kit', async (importOriginal) => ({
  ...(await importOriginal<typeof import('@solana/kit')>()),
  sendAndConfirmTransactionFactory: () => sendAndConfirm,
}));

const certificate = vi.hoisted(() => vi.fn());

const getCurrentBatch = vi.hoisted(() => vi.fn());
vi.mock('./reads.js', () => ({ getCurrentBatch }));

import {
  address,
  decompileTransactionMessage,
  generateKeyPairSigner,
  getCompiledTransactionMessageDecoder,
  type Address,
  type Transaction,
} from '@solana/kit';
import { base58 } from '@scure/base';

import { settleBatch, type SolanaVaultSettleOptions } from './settleBatch.js';
import { deriveBatchAddresses, type VaultDemoRoots } from './derive.js';
import { getSettleInstructionDataDecoder, parseSettleInstruction } from './internal/generated/confidentialBatcher/instructions/settle.js';
import { CONFIDENTIAL_BATCHER_PROGRAM_ADDRESS } from './internal/generated/confidentialBatcher/programAddress.js';
import { findDenyScopeRecordPda, ZAMA_HOST_PROGRAM_ADDRESS } from '@fhevm/solana-zama-host';
import { CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS } from '@fhevm/confidential-token';
import { encodedSize, testDemoClient } from '../testDemoClient';

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
    kmsContext: addr(9),
  };
}

function claim(cleartext: string, signatures = 1) {
  return {
    handle: `0x${hex(BURNED_HANDLE)}`,
    abiEncodedCleartext: cleartext,
    signatures: Array.from({ length: signatures }, () => hex(new Uint8Array(65).fill(0x11))),
    extraData: '0x00',
  };
}

async function setup(overrides: { burnedHandle?: Uint8Array } = {}) {
  const addresses = await deriveBatchAddresses(roots(), 0n);
  getCurrentBatch.mockResolvedValue({
    index: 0n,
    addresses,
    state: { burnedTotalHandle: overrides.burnedHandle ?? BURNED_HANDLE },
  });
  const opts: SolanaVaultSettleOptions = {
    roots: roots(),
    contextId: new Uint8Array(32),
    authorityFundingLamports: 5_000_000n,
  };
  return { ...testDemoClient(await generateKeyPairSigner()), opts, addresses };
}

/** The last transaction handed to Kit's send-and-confirm. */
const sent = (): Transaction => sendAndConfirm.mock.lastCall![0] as Transaction;

function messageOf(transaction: Transaction) {
  return decompileTransactionMessage(getCompiledTransactionMessageDecoder().decode(transaction.messageBytes));
}

describe('settleBatch', () => {
  beforeEach(() => {
    sendAndConfirm.mockReset().mockResolvedValue(undefined);
    certificate.mockReset();
    getCurrentBatch.mockReset();
  });
  afterEach(() => vi.restoreAllMocks());

  it('settles the current batch with its certificate as one v1 FHE transaction', async () => {
    certificate.mockResolvedValue(claim(cleartextHex(800n)));
    const { client, opts, addresses } = await setup();
    const signature = await settleBatch({ publicDecryptCertificate: certificate }, client, opts);

    // The batch was resolved from chain state, not supplied.
    expect(getCurrentBatch).toHaveBeenCalledTimes(1);
    // The certificate names the handle and the account, nothing else: no proof travels.
    expect(certificate).toHaveBeenCalledTimes(1);
    expect(certificate.mock.calls[0]![0]).toEqual({
      handle: `0x${hex(BURNED_HANDLE)}`,
      contextId: opts.contextId,
      encryptedStore: base58.decode(addresses.batchBurnedAmountStore),
      options: undefined,
    });

    expect(sendAndConfirm).toHaveBeenCalledOnce();
    const transaction = sent();
    expect(Object.keys(transaction.signatures)[0]).toBe(client.payer.address);
    expect(signature).toEqual(expect.any(String));
    const message = messageOf(transaction);
    expect(message.version).toBe(1);
    const [open, settle, close] = message.instructions;
    expect([...message.instructions].map((instruction) => instruction.programAddress)).toEqual([
      ZAMA_HOST_PROGRAM_ADDRESS,
      CONFIDENTIAL_BATCHER_PROGRAM_ADDRESS,
      ZAMA_HOST_PROGRAM_ADDRESS,
    ]);
    const parsed = parseSettleInstruction({ ...settle!, accounts: settle!.accounts!, data: settle!.data! });
    expect(parsed.accounts.batch.address).toBe(addresses.batch);
    expect(open!.accounts?.[1]?.address).toBe(parsed.accounts.transientStore.address);
    expect(close!.accounts?.[1]?.address).toBe(parsed.accounts.transientStore.address);
    // The certified 32-byte cleartext was decoded to the u64 settle argument.
    expect(getSettleInstructionDataDecoder().decode(settle!.data!).cleartextTotal).toBe(800n);
  });

  // The largest settle: a certificate at the host's maximum KMS threshold (MAX_KMS_SIGNERS = 16)
  // with the deny record and both HCU witnesses. A v1 transaction is at most 4,096 bytes and 64
  // account keys (solana-message v1::MAX_TRANSACTION_SIZE and MAX_ADDRESSES).
  it('fits one v1 transaction at the maximum KMS threshold with every witness', async () => {
    certificate.mockResolvedValue(claim(cleartextHex(800n), 16));
    const { client, opts } = await setup();
    await settleBatch({ publicDecryptCertificate: certificate }, client, {
      ...opts,
      payoutMintHcuBlockMeter: addr(40),
      payoutMintHcuTrustedAppRecord: addr(41),
      denyListEnabled: true,
    });
    const size = encodedSize(sent());
    expect(size.version).toBe(1);
    expect(size.bytes).toBeLessThanOrEqual(4096);
    expect(size.addresses).toBeLessThanOrEqual(64);
  });

  // The settle instruction's accounts as submitted.
  async function submittedSettleAccounts(total: bigint, denyListEnabled: boolean): Promise<Address[]> {
    certificate.mockResolvedValue(claim(cleartextHex(total)));
    const { client, opts } = await setup();
    await settleBatch({ publicDecryptCertificate: certificate }, client, { ...opts, denyListEnabled });
    const settle = messageOf(sent()).instructions[1]!;
    return Array.from(settle.accounts ?? [], (account) => account.address);
  }

  it('appends, under the deny list, the payout mint deny record for the wrap, and none for a zero total', async () => {
    const plain = await submittedSettleAccounts(800n, false);
    const [payoutMintRecord] = await findDenyScopeRecordPda({
      appProgram: CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS,
      scope: roots().payoutConfidentialMint,
    });
    expect((await submittedSettleAccounts(800n, true)).slice(plain.length)).toEqual([payoutMintRecord]);
    expect(await submittedSettleAccounts(0n, true)).toHaveLength(plain.length);
  });

  it('rejects a batch that has not been dispatched (zero burned handle) before any phase', async () => {
    certificate.mockResolvedValue(claim(cleartextHex(800n)));
    const { client, opts, rpcMethods } = await setup({ burnedHandle: new Uint8Array(32) });
    await expect(settleBatch({ publicDecryptCertificate: certificate }, client, opts)).rejects.toThrow('no burned total handle');
    expect(certificate).not.toHaveBeenCalled();
    expect(rpcMethods).toEqual([]);
    expect(sendAndConfirm).not.toHaveBeenCalled();
  });

  it('rejects a certified total that does not fit u64 before touching the RPC or sending', async () => {
    certificate.mockResolvedValue(claim(cleartextHex(1n, 0x01))); // a high byte set
    const { client, opts, rpcMethods } = await setup();
    await expect(settleBatch({ publicDecryptCertificate: certificate }, client, opts)).rejects.toThrow('exceeds u64');
    expect(rpcMethods).toEqual([]);
    expect(sendAndConfirm).not.toHaveBeenCalled();
  });
});
