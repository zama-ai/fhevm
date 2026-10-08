// Gateway KMS context tag, matching shared/kms-context.
const KMS_CONTEXT_COUNTER_BASE = 0x07n << 248n;
// First gateway context, encoded as a 32-byte big-endian uint256.
export const BRINGUP_KMS_CONTEXT_HEX = (KMS_CONTEXT_COUNTER_BASE + 1n).toString(16).padStart(64, '0');

export const BRINGUP_KMS_CONTEXT_ID = Uint8Array.from(
  BRINGUP_KMS_CONTEXT_HEX.match(/.{2}/g)!.map((byte) => Number.parseInt(byte, 16)),
);

// The per-transaction HCU limits a new host starts with, the values `host-contracts/tasks/taskDeploy.ts`
// deploys `HCULimit` with. Admins may tune them afterwards with the setters, as on EVM.
export const HCU_LIMITS = { maxHcuDepthPerTx: 5_000_000n, maxHcuPerTx: 20_000_000n } as const;

export const SOLANA_DEPLOY_PROGRAMS = [
  'zama_host',
  'confidential_token',
  'demo_vault',
  'confidential_batcher',
] as const;
export type SolanaDeployProgram = (typeof SOLANA_DEPLOY_PROGRAMS)[number] | 'encrypted_counter' | 'dep_chain';
