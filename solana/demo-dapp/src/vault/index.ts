/** Demo vault workflows composed through the public SDK and generated application clients. */

export { joinBatch, type SolanaVaultJoinParameters } from './joinBatch.js';
export { buildQuitInstruction, type SolanaVaultQuitParameters } from './quit.js';
export { buildDispatchBatchInstruction, type SolanaVaultDispatchParameters } from './dispatchBatch.js';
export { buildCancelDispatchInstruction, type SolanaVaultCancelDispatchParameters } from './cancelDispatch.js';
export { settleBatch, type SolanaVaultSettleOptions } from './settleBatch.js';
export { buildClaimInstruction, type SolanaVaultClaimParameters } from './claim.js';
export {
  buildReclaimBatchAuthorityInstruction,
  type SolanaVaultReclaimBatchAuthorityParameters,
} from './reclaimBatchAuthority.js';
export { buildCloseJoinRecordInstruction, type SolanaVaultCloseJoinRecordParameters } from './closeJoinRecord.js';
export {
  buildHarvestInstruction,
  getVaultMetrics,
  type SolanaVaultHarvestParameters,
  type SolanaVaultMetrics,
} from './harvest.js';
export { openBatch, type SolanaVaultOpenBatchParameters, type SolanaVaultOpenBatchResult } from './openBatch.js';

// One-time provisioning builders the demo seeder drives (fhevm-internal#1760). Kept on the vault
// surface — the seeder is their only caller — and shaped as thin, root-taking actions: each derives
// its encrypted store/event PDAs internally so the seeder passes semantic roots, never hand-rolled accounts.
export { buildInitializeVaultInstruction, type SolanaVaultInitializeVaultParameters } from './initializeVault.js';
export {
  buildInitializeBatcherInstruction,
  BatchDirection,
  type SolanaVaultInitializeBatcherParameters,
} from './initializeBatcher.js';
export { buildInitializeMintInstruction, type SolanaVaultInitializeMintParameters } from './initializeMint.js';
export {
  buildInitializeTokenAccountInstruction,
  getOrCreateConfidentialTokenAccountInstruction,
  needsConfidentialTokenAccountInitialization,
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
  deriveJoinRecordAddress,
  deriveSettleAccounts,
  deriveSettleLookupTableAddresses,
  settleAccountsToLookupTableAddresses,
  type VaultDemoRoots,
  type BatchAddresses,
  type SolanaVaultSettleAccounts,
} from './derive.js';
export {
  getBatcher,
  getBatchByIndex,
  getCurrentBatch,
  getEncryptedStore,
  getJoinRecord,
  type BatcherState,
  type BatchState,
  type JoinRecordState,
} from './reads.js';

export { settleTotalFromCleartext } from './internal/cleartext.js';
export {
  batchAddress,
  tokenAccountAddress,
  pendingBurnAddress,
  tokenStateAddress,
  joinStoreAddress,
} from './internal/batcherPdas.js';
export { TOKEN_PROGRAM_ADDRESS } from './internal/tokenAccounts.js';
export {
  ADDRESS_LOOKUP_TABLE_PROGRAM_ADDRESS,
  LOOKUP_TABLE_DEACTIVATION_COOLDOWN_SLOTS,
  LOOKUP_TABLE_STILL_ACTIVE,
  MAX_EXTEND_ADDRESSES_PER_TRANSACTION,
  decodeLookupTableDeactivationSlot,
  deriveAddressLookupTableAddress,
  getCreateLookupTableInstruction,
  getCloseLookupTableInstruction,
  getDeactivateLookupTableInstruction,
  getExtendLookupTableInstruction,
  getExtendLookupTableInstructions,
} from './internal/addressLookupTable.js';
export { CONFIDENTIAL_BATCHER_PROGRAM_ADDRESS } from './internal/generated/confidentialBatcher/programAddress.js';
