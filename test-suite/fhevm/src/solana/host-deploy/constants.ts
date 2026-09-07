// RFC-021 Solana host chain id: the chain-type high bit ORed over 12345. The coprocessor DB
// stores chain ids as PostgreSQL BIGINT, so the same bit pattern reads back as the negative i64.
export const SOLANA_HOST_CHAIN_ID = 9223372036854788153n;
export const SOLANA_HOST_CHAIN_ID_I64 = SOLANA_HOST_CHAIN_ID - (1n << 64n);

/**
 * Bring-up KMS context id. Same 32-byte tagged gateway uint256 as
 * `SOLANA_DEFAULT_PUBLIC_DECRYPT_CONTEXT` in `src/layout.ts` (duplicated here so the deployer
 * image bundle does not import the fhevm-cli layout module).
 */
const BRINGUP_KMS_CONTEXT_HEX = "0700000000000000000000000000000000000000000000000000000000000001";

export const BRINGUP_KMS_CONTEXT_ID = Uint8Array.from(
  BRINGUP_KMS_CONTEXT_HEX.match(/.{2}/g)!.map((byte) => Number.parseInt(byte, 16)),
);

export const SOLANA_DEPLOY_PROGRAMS = ["zama_host", "confidential_token"] as const;
export type SolanaDeployProgram = (typeof SOLANA_DEPLOY_PROGRAMS)[number];
