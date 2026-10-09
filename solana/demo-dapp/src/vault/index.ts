/** Demo vault workflows composed through the public SDK and generated application clients. */

export { joinBatch, type SolanaVaultJoinParameters } from './joinBatch.js';
export { readHostPolicy, type HostPolicy } from './internal/hostPolicy.js';
export { buildQuitInstruction, type SolanaVaultQuitParameters } from './quit.js';
export { buildDispatchBatchInstruction, type SolanaVaultDispatchParameters } from './dispatchBatch.js';
export { buildCancelDispatchInstruction, type SolanaVaultCancelDispatchParameters } from './cancelDispatch.js';
export { settleBatch, type SolanaVaultSettleOptions } from './settleBatch.js';
export { buildClaimInstruction, type SolanaVaultClaimParameters } from './claim.js';
export {
  getCloseJoinRecordInstructionAsync,
  getInitializeBatcherInstruction,
  getReclaimBatchAuthorityInstructionAsync,
} from './internal/generated/confidentialBatcher/instructions/index.js';
export { BatchDirection } from './internal/generated/confidentialBatcher/types/batchDirection.js';
export { BatchStatus } from './internal/generated/confidentialBatcher/types/batchStatus.js';
export { JOINED_AMOUNT_KEY } from './internal/generated/confidentialBatcher/constants.js';
export { findJoinRecordPda } from './internal/generated/confidentialBatcher/pdas/index.js';
export { getInitializeVaultInstructionAsync } from './internal/generated/demoVault/instructions/initializeVault.js';
export {
  buildHarvestInstruction,
  getVaultMetrics,
  type SolanaVaultHarvestParameters,
  type SolanaVaultMetrics,
} from './harvest.js';

// One-time provisioning builders the demo seeder drives (fhevm-internal#1760). Each derives its
// encrypted store/event PDAs internally so the seeder passes semantic roots, never hand-rolled accounts.
export { buildInitializeMintInstruction, type SolanaVaultInitializeMintParameters } from './initializeMint.js';
export {
  buildInitializeTokenAccountInstruction,
  type SolanaVaultInitializeTokenAccountParameters,
} from './initializeTokenAccount.js';
export { buildWrapUsdcInstruction, type SolanaVaultWrapUsdcParameters } from './wrapUsdc.js';
export { openBatchForBatcher, type SolanaVaultOpenBatchForBatcherParameters } from './openBatchForBatcher.js';

// The confidential-token app actions (transfer + secp disclosure). These moved out of the SDK's
// protocol surface with the rest of the dapp code: they target one specific token program, not
// the host protocol.
export { confidentialTransfer, type SolanaConfidentialTransferParameters } from './actions/confidentialTransfer.js';
export { buildDiscloseSecpInstruction, type SolanaDiscloseSecpAccounts } from './actions/discloseSecp.js';

// Program ids the seeder records into the demo-config `programs` block. `CONFIDENTIAL_BATCHER_PROGRAM_ADDRESS`
// is already exported below with the batcher internals; the token/host pair comes from the
// workspace confidential-token client, and the demo vault id from its generated client.
export { CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS, ZAMA_HOST_PROGRAM_ADDRESS } from '@fhevm/confidential-token';
export { DEMO_VAULT_PROGRAM_ADDRESS } from './internal/generated/demoVault/programAddress.js';

export {
  deriveBatchAddresses,
  deriveSettleAccounts,
  type VaultDemoRoots,
  type BatchAddresses,
  type SolanaVaultSettleAccounts,
} from './derive.js';
export {
  dispatchableAt,
  getBatcher,
  getBatchByIndex,
  getBatchJoinRecords,
  getCurrentBatch,
  getEncryptedStore,
  getJoinRecord,
  settleDeadline,
  type BatcherState,
  type BatchState,
  type JoinRecordState,
} from './reads.js';

export { settleTotalFromCleartext } from './internal/cleartext.js';
export { joinStoreAddress, tokenStoreAddress } from './internal/encryptedStores.js';
export { CONFIDENTIAL_BATCHER_PROGRAM_ADDRESS } from './internal/generated/confidentialBatcher/programAddress.js';

export { findShareMintPda } from './internal/generated/demoVault/pdas/index.js';
