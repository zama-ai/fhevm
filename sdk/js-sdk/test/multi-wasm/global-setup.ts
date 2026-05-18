import { ensureLocalstackReady } from './support/localstack.js';

export default async function globalSetup(): Promise<void> {
  const restart = process.env.MULTI_WASM_RESTART_LOCALSTACK === '1';
  const env = await ensureLocalstackReady({ restart });
  process.env.MULTI_WASM_RPC_URL = env.rpcUrl;
}
