import { loadLocalstackChainDefaults } from './support/chainDefaults.js';
import { ensureLocalstackReady } from './support/localstack.js';

export default async function globalSetup(): Promise<void> {
  const restart = process.env.MULTI_WASM_RESTART_LOCALSTACK === '1';
  const fhevmCliProfile = process.env.MULTI_WASM_FHEVM_CLI_PROFILE;
  const chainName = process.env.CHAIN ?? 'localstack';
  const { rpcUrl } = loadLocalstackChainDefaults(chainName);
  await ensureLocalstackReady({ restart, rpcUrl, chainName, fhevmCliProfile });
}
