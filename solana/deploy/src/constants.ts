// Gateway KMS context and epoch tags, matching host-contracts/contracts/shared/Constants.sol.
const KMS_CONTEXT_COUNTER_BASE = 0x07n << 248n;
const EPOCH_COUNTER_BASE = 0x08n << 248n;
// First gateway context, encoded as a 32-byte big-endian uint256.
export const BRINGUP_KMS_CONTEXT_HEX = (KMS_CONTEXT_COUNTER_BASE + 1n).toString(16).padStart(64, '0');

const uint256Bytes = (value: bigint): Uint8Array =>
  Uint8Array.from(value.toString(16).padStart(64, '0').match(/.{2}/g)!.map((byte) => Number.parseInt(byte, 16)));

export const BRINGUP_KMS_CONTEXT_ID = uint256Bytes(KMS_CONTEXT_COUNTER_BASE + 1n);
// The epoch the bring-up context starts with.
export const BRINGUP_KMS_EPOCH_ID = uint256Bytes(EPOCH_COUNTER_BASE + 1n);

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
