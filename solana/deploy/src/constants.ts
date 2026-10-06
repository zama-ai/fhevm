// Localnet Solana host chain id: type byte `0x01` plus cluster tag 12345.
// Fits in PostgreSQL BIGINT as a positive i64.
export const SOLANA_HOST_CHAIN_ID = 72057594037940281n;

// Gateway KMS context tag, matching shared/kms-context.
const KMS_CONTEXT_COUNTER_BASE = 0x07n << 248n;
// First gateway context, encoded as a 32-byte big-endian uint256.
export const BRINGUP_KMS_CONTEXT_HEX = (KMS_CONTEXT_COUNTER_BASE + 1n).toString(16).padStart(64, '0');

export const BRINGUP_KMS_CONTEXT_ID = Uint8Array.from(
  BRINGUP_KMS_CONTEXT_HEX.match(/.{2}/g)!.map((byte) => Number.parseInt(byte, 16)),
);

export const SOLANA_DEPLOY_PROGRAMS = [
  'zama_host',
  'confidential_token',
  'demo_vault',
  'confidential_batcher',
] as const;
export type SolanaDeployProgram = (typeof SOLANA_DEPLOY_PROGRAMS)[number] | 'encrypted_counter' | 'dep_chain';
