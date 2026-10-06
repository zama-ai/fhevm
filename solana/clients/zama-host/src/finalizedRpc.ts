import { createDefaultRpcTransport, createRpc, createSolanaRpcApi, DEFAULT_RPC_CONFIG } from '@solana/kit';

// Kit's createSolanaRpc defaults reads without a commitment to confirmed; this client defaults to finalized.
export const createFinalizedRpc = (url: string) => createRpc({
  api: createSolanaRpcApi({ ...DEFAULT_RPC_CONFIG, defaultCommitment: 'finalized' }),
  transport: createDefaultRpcTransport({ url }),
});
