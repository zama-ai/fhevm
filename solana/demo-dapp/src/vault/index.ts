/**
 * Solana confidential-vault dapp module (fhevm-internal#1759): typed client actions for the
 * confidential-batcher lifecycle used by the confidential-vault demo (epic #1754).
 *
 * This is dapp code, not SDK code — it lives in the demo (fhevm-internal#1859 §6d) and reaches
 * past the published package's exports via the `@sdk-src/*` alias for internals it needs. The
 * alias resolves to two different copies on purpose: inside this dapp it points at the *built*
 * package under `node_modules/@fhevm/sdk` (`_types` for tsc, `_esm` for vite), so every type has
 * one identity; the bun consumers that run without a build step (test-suite scenarios, the
 * two-holder worker) map it to the SDK *sources* instead. The SDK's protocol surface
 * (`@fhevm/sdk/solana`) carries no vault, token-app, or address-lookup-table code.
 */

export { joinBatch, type SolanaVaultJoinParameters } from './joinBatch.js';
export { buildQuitInstruction, type SolanaVaultQuitParameters } from './quit.js';
export { buildDispatchBatchInstruction, type SolanaVaultDispatchParameters } from './dispatchBatch.js';
export { buildCancelDispatchInstruction, type SolanaVaultCancelDispatchParameters } from './cancelDispatch.js';
export { settleBatch, type SolanaVaultSettleOptions } from './settleBatch.js';
export { buildClaimInstruction, type SolanaVaultClaimParameters } from './claim.js';
export {
  buildHarvestInstruction,
  getVaultMetrics,
  type SolanaVaultHarvestParameters,
  type SolanaVaultMetrics,
} from './harvest.js';
export { decryptPosition } from './decryptPosition.js';
export { openBatch, type SolanaVaultOpenBatchParameters, type SolanaVaultOpenBatchResult } from './openBatch.js';

// One-time provisioning builders the demo seeder drives (fhevm-internal#1760). Kept on the vault
// surface — the seeder is their only caller — and shaped as thin, root-taking actions: each derives
// its encrypted value account/event PDAs internally so the seeder passes semantic roots, never hand-rolled accounts.
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
export {
  buildMakeTokenAccountHandlePublicInstruction,
  type SolanaMakeTokenAccountHandlePublicParameters,
} from './actions/makeHandlePublic.js';
export { DisclosedValueKind } from './internal/generated/confidentialToken/types/disclosedValueKind.js';

// Program ids the seeder records into the demo-config `programs` block. `CONFIDENTIAL_BATCHER_PROGRAM_ADDRESS`
// is already exported below with the batcher internals; the other three come from the generated
// confidential-token / demo-vault program-address modules.
export {
  CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS,
  ZAMA_HOST_PROGRAM_ADDRESS,
} from './internal/generated/confidentialToken/programAddress.js';
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
  getEncryptedValueState,
  getJoinRecord,
  type BatcherState,
  type BatchState,
  type JoinRecordState,
} from './reads.js';

export { settleTotalFromCleartext } from './internal/cleartext.js';
export {
  fetchSolanaPublicDecryptProof,
  type SolanaProofServiceConfig,
  type SolanaMmrProofResult,
} from './internal/proofService.js';
export {
  batchAddress,
  tokenAccountAddress,
  pendingBurnAddress,
  burnedAmountValueAccount,
  pendingJoinValueAccount,
  claimAmountValueAccount,
  type SolanaEncryptedValueAccount,
} from './internal/batcherPdas.js';
// The mint's compute-signer PDA — the contract identity an input proof binds to. Exported so demo
// consumers derive it from the mint root instead of restating the `fhe-compute` seed; the other
// confidential-token encrypted-value-account derivations stay internal because every action derives them itself.
export { computeSignerAddress } from './internal/tokenValueAccount.js';
export { confidentialBalanceValueAccount } from './internal/tokenValueAccount.js';
// The canonical `EncryptedValue` PDA for an arbitrary `(domain, account, label)` triple. Exported
// for the e2e scenarios' raw fhe_execute driver, which binds persistent outputs to scenario-owned
// values rather than the token/batcher-shaped ones above.
export { encryptedValueAddress } from './internal/batcherPdas.js';
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
