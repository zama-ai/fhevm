import { INSTRUCTIONS_SYSVAR_ADDRESS, prepareTransientStore } from '@fhevm/sdk/solana';
import type { Signature } from '@solana/kit';
import { base58 } from '@scure/base';

import { bytesToHex, hexToBytes } from '@fhevm/sdk/base';
import type { FhevmSolanaPublicDecryptClient } from '@fhevm/sdk/solana';
import type { RelayerPublicDecryptOptions } from '@fhevm/sdk/types';
import {
  getSettleInstructionAsync,
  type SettleAsyncInput,
} from './internal/generated/confidentialBatcher/instructions/settle.js';
import { tokenApp, withDenyRecords, type DenyListParameters } from './internal/denyRecords.js';
import { fetchBatch } from './internal/generated/confidentialBatcher/accounts/batch.js';
import { settleTotalFromCleartext } from './internal/cleartext.js';
import { deriveBatchAddresses, deriveSettleAccounts, type BatchAddresses, type VaultDemoRoots } from './derive.js';
import { getCurrentBatch } from './reads.js';
import { ZAMA_HOST_PROGRAM_ADDRESS } from '@fhevm/confidential-token';
import type { DemoClient } from '../demoClient';
// The token client pins the host program it was compiled against; the vault module targets that pair.

const ZERO_HANDLE = new Uint8Array(32);

/** What `settleBatch` needs beyond the certificate phase and the keeper's client. */
export type SolanaVaultSettleOptions = Pick<SettleAsyncInput, 'payoutMintHcuBlockMeter' | 'payoutMintHcuTrustedAppRecord'> &
  DenyListParameters & {
  /** The batcher's demo topology; every settle account is derived from these. */
  readonly roots: VaultDemoRoots;
  /** Which batch to settle; defaults to the batcher's current (most-recently-opened) batch. */
  readonly batchIndex?: bigint | undefined;
  readonly authorityFundingLamports: bigint;
  /** Bounds and observes the relayer/KMS certificate request independently of the on-chain send. */
  readonly certificateOptions?: RelayerPublicDecryptOptions | undefined;
};

/**
 * Settles the batch's pinned burn handle with a KMS certificate. The settle program checks the
 * certificate against the burned handle pinned in the batch's `PendingBurn`. `keeperClient.payer` pays.
 */
export async function settleBatch(
  client: Pick<FhevmSolanaPublicDecryptClient, 'publicDecryptCertificate'>,
  keeperClient: DemoClient,
  options: SolanaVaultSettleOptions,
): Promise<Signature> {
  const { roots } = options;
  const { rpc } = keeperClient;

  // Resolve the batch and its created-public burned handle from chain state.
  let addresses: BatchAddresses;
  let burnedTotalHandle: Uint8Array;
  if (options.batchIndex !== undefined) {
    addresses = await deriveBatchAddresses(roots, options.batchIndex);
    const batch = await fetchBatch(rpc, addresses.batch);
    burnedTotalHandle = new Uint8Array(batch.data.burnedTotalHandle);
  } else {
    const current = await getCurrentBatch(rpc, roots);
    addresses = current.addresses;
    burnedTotalHandle = new Uint8Array(current.state.burnedTotalHandle);
  }
  if (burnedTotalHandle.every((byte, i) => byte === ZERO_HANDLE[i])) {
    throw new Error(`batch ${addresses.batch} has no burned total handle yet; dispatch it before settling`);
  }

  const accounts = await deriveSettleAccounts(roots, addresses);

  // The KMS burn certificate. The relayer request names the handle and the account, nothing else.
  const claim = await client.publicDecryptCertificate({
      handle: bytesToHex(burnedTotalHandle),
      encryptedStore: base58.decode(accounts.batchBurnedAmountStore),
      options: options.certificateOptions,
  });

  const cleartextTotal = settleTotalFromCleartext(hexToBytes(claim.abiEncodedCleartext));

  const signatures = claim.signatures.map((signature, index) => {
    const bytes = hexToBytes(signature);
    if (bytes.length !== 65) throw new Error(`certificate signature[${index}] must be 65 bytes`);
    return bytes;
  });

  const transientStore = await prepareTransientStore({ payer: keeperClient.payer, host: ZAMA_HOST_PROGRAM_ADDRESS });
  const settleWithoutDenyRecords = await getSettleInstructionAsync({
    transientStore: transientStore.address,
    instructions: INSTRUCTIONS_SYSVAR_ADDRESS,
    payer: keeperClient.payer,
    ...accounts,
    cleartextTotal,
    signatures,
    extraData: hexToBytes(claim.extraData),
    authorityFundingLamports: options.authorityFundingLamports,
    payoutMintHcuBlockMeter: options.payoutMintHcuBlockMeter,
    payoutMintHcuTrustedAppRecord: options.payoutMintHcuTrustedAppRecord,
  });
  const settleInstruction = await withDenyRecords(
    settleWithoutDenyRecords,
    options.denyListEnabled,
    cleartextTotal === 0n ? [] : [tokenApp(roots.payoutConfidentialMint)],
  );

  return (await keeperClient.sendFheTransaction(transientStore, [settleInstruction])).context.signature;
}
