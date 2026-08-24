// The Solana user-decrypt permit: typed form, canonical text, envelope, signature check.
//
// One entry point per concern and nothing else: `stub.js` is deliberately absent, and so is every
// helper the modules keep to themselves. The constants are exported alongside the functions because
// they are normative pins — a test that hardcodes 869 proves less than one that asserts the module
// and the vectors agree on what 869 is.

export {
  PERMIT_IDENTITY_LEN,
  PERMIT_KMS_ROUTING_LEN,
  PERMIT_KMS_ROUTING_VERSION,
  PERMIT_MAX_ACL_DOMAIN_KEYS,
  PERMIT_MAX_DURATION_SECONDS,
  PERMIT_MAX_START_TIMESTAMP,
  PERMIT_MIN_DURATION_SECONDS,
  PERMIT_SIGNATURE_LEN,
  PERMIT_TRANSPORT_KEY_LEN,
  isPermissivePermit,
} from './types.js';
export type { SolanaKmsRouting, SolanaPermitFields, SolanaPermitU64, SolanaPermitWireFields } from './types.js';

export { SolanaPermitError } from './errors.js';
export type {
  SolanaPermitIdentityField,
  SolanaPermitRejection,
  SolanaPermitRejectionCode,
  SolanaPermitU64Field,
} from './errors.js';

export { decodeSolanaPermitFields, encodeSolanaKmsRouting } from './validate.js';

export { transportKeyFingerprint } from './fingerprint.js';

export {
  PERMIT_TEXT_HEADER,
  PERMIT_TEXT_PERMISSIVE_DOMAINS_LINE,
  renderPermitTimestamp,
  renderSolanaPermitText,
} from './render.js';

export {
  PERMIT_ENVELOPE_PREAMBLE,
  PERMIT_ENVELOPE_SIGNER_COUNT,
  PERMIT_ENVELOPE_VERSION,
  buildSolanaPermitEnvelope,
  verifySolanaPermitSignature,
} from './envelope.js';

export {
  SOLANA_OFFCHAIN_MESSAGE_VERSION,
  SOLANA_SIGN_OFFCHAIN_MESSAGE_FEATURE,
  SolanaPermitChannelError,
  signSolanaPermit,
} from './channel.js';
export { solanaPermitWalletFromSecretKey } from './headlessWallet.js';
export type { SolanaPermitChannelFailure, SolanaPermitWallet, SolanaSignedPermit } from './channel.js';
// The channel speaks the Wallet Standard's own types; they are re-exported so a caller assembling
// a SolanaPermitWallet — or a test wallet — names the same contract without a second import path.
export type {
  SolanaSignOffchainMessageFeature,
  SolanaSignOffchainMessageInput,
  SolanaSignOffchainMessageOutput,
} from '@solana/wallet-standard-features';
export type { WalletAccount } from '@wallet-standard/base';

export { PERMIT_START_GRANULARITY_SECONDS, normalizeSolanaPermitStart } from './start.js';

export { PERMIT_PERMISSIVE_WARNING, PERMIT_WARN_ABOVE_DURATION_SECONDS, solanaPermitWarnings } from './warnings.js';
export type { SolanaPermitWarning } from './warnings.js';
