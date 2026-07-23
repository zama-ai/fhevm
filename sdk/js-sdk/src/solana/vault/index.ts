/**
 * Isolated Solana confidential-vault module (fhevm-internal#1759): typed client actions for the
 * confidential-batcher lifecycle used by the confidential-vault demo (epic #1754).
 *
 * This module is DELIBERATELY not re-exported from `src/solana/index.ts` — the demo surface must
 * not leak into the SDK's normal Solana paths. Import it explicitly from `@fhevm/sdk/solana/vault`.
 */

export { joinBatch, type SolanaVaultJoinParameters } from './joinBatch.js';
export { buildQuitInstruction, type SolanaVaultQuitParameters } from './quit.js';
export { buildDispatchBatchInstruction, type SolanaVaultDispatchParameters } from './dispatchBatch.js';
export { settleBatch, type SolanaVaultSettleParameters, type SolanaVaultSettleAccounts } from './settleBatch.js';
export { buildClaimInstruction, type SolanaVaultClaimParameters } from './claim.js';
export { decryptPosition } from './decryptPosition.js';
export { openBatch, type SolanaVaultOpenBatchParameters, type SolanaVaultOpenBatchResult } from './openBatch.js';

export { settleTotalFromCleartext } from './internal/cleartext.js';
export {
  fetchSolanaPublicDecryptProof,
  type SolanaProofServiceConfig,
  type SolanaMmrProofResult,
} from './internal/proofService.js';
export {
  batchAddress,
  tokenAccountAddress,
  burnRedemptionAddress,
  burnedAmountLineage,
  pendingJoinLineage,
  claimAmountLineage,
  type SolanaValueLineage,
} from './internal/batcherPdas.js';
export {
  ADDRESS_LOOKUP_TABLE_PROGRAM_ADDRESS,
  deriveAddressLookupTableAddress,
  getCreateLookupTableInstruction,
  getExtendLookupTableInstruction,
} from './internal/addressLookupTable.js';
export { CONFIDENTIAL_BATCHER_PROGRAM_ADDRESS } from './internal/generated/confidentialBatcher/programAddress.js';
