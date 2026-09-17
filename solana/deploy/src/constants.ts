// Localnet Solana host chain id: type byte `0x01` plus cluster tag 12345.
// Fits in PostgreSQL BIGINT as a positive i64.
export const SOLANA_HOST_CHAIN_ID = 72057594037940281n;
export const SOLANA_HOST_CHAIN_ID_I64 = SOLANA_HOST_CHAIN_ID;

/**
 * Bring-up KMS context id. Same 32-byte tagged gateway uint256 as
 * `SOLANA_DEFAULT_PUBLIC_DECRYPT_CONTEXT` in `src/layout.ts` (duplicated here so the deployer
 * image bundle does not import the fhevm-cli layout module).
 */
const BRINGUP_KMS_CONTEXT_HEX = '0700000000000000000000000000000000000000000000000000000000000001';

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
