// Spike S4 (phase-1b.md §13.4): runs the scenario's sequence as a plain Node script, without
// Cucumber or the engine worker, and never calls process.exit. If the process does not end on its
// own once the work is done and everything is released, something the SDK opened keeps it alive.
//
// Usage (after `npm run build`), with the variables of test-suite.env in the environment:
//   node dist/scenarios/spikes/fhe-sdk/exit-probe.js
import { ContractFactory, HDNodeWallet, JsonRpcProvider, Network } from 'ethers';
import { readFileSync } from 'node:fs';
import path from 'node:path';

import { activeResources, createFheClient, encryptUint64, isYes, publicDecrypt, userDecrypt } from './sdk-client.js';

const env = (name: string): string => {
  const value = process.env[name];
  if (!value) throw new Error(`${name} is not set`);
  return value;
};
const address = (name: string) => env(name) as `0x${string}`;
const step = (message: string) => console.log(`[exit-probe] ${message}`);
/** Runs `work` and returns its result with how long it took. */
async function timed<T>(work: () => Promise<T>): Promise<[T, number]> {
  const started = Date.now();
  const result = await work();
  return [result, Date.now() - started];
}

const singleThread = isYes(process.env.FHE_SPIKE_SINGLE_THREAD);

async function main(): Promise<void> {
  const baseline = activeResources();
  const chainId = Number(process.env.CHAIN_ID_HOST ?? 12345);
  const provider = new JsonRpcProvider(env('RPC_URL'), Network.from(chainId), {
    staticNetwork: true,
    pollingInterval: 1_000,
  });
  const alice = HDNodeWallet.fromPhrase(env('MNEMONIC'), undefined, "m/44'/60'/0'/0/0").connect(provider);

  // The scenario folder in the source tree holds the artifact; this file runs from dist/.
  const artifactPath = path.resolve(
    import.meta.dirname,
    '../../../../scenarios/spikes/fhe-sdk/artifacts/TestInput.json',
  );
  const artifact = JSON.parse(readFileSync(artifactPath, 'utf8')) as { abi: unknown[]; bytecode: string };
  const contract = await new ContractFactory(artifact.abi as never, artifact.bytecode, alice).deploy();
  await contract.waitForDeployment();
  const contractAddress = await contract.getAddress();
  step(`TestInput deployed at ${contractAddress}`);

  step(`singleThread: ${singleThread}`);
  const [client, readyMs] = await timed(() =>
    createFheClient(
      {
        relayerUrl: env('RELAYER_URL'),
        chainId,
        gatewayChainId: Number(process.env.CHAIN_ID_GATEWAY ?? 54321),
        decryption: address('DECRYPTION_ADDRESS'),
        inputVerification: address('INPUT_VERIFICATION_ADDRESS'),
        acl: address('ACL_CONTRACT_ADDRESS'),
        inputVerifier: address('INPUT_VERIFIER_CONTRACT_ADDRESS'),
        kmsVerifier: address('KMS_VERIFIER_CONTRACT_ADDRESS'),
        protocolConfig: address('PROTOCOL_CONFIG_CONTRACT_ADDRESS'),
      },
      provider,
      singleThread,
    ),
  );
  step(`SDK client ready in ${readyMs} ms`);

  const [{ handle, inputProof }, encryptMs] = await timed(() =>
    encryptUint64(client, 7n, contractAddress, alice.address),
  );
  step(`encrypted in ${encryptMs} ms`);
  const receipt = await (await contract.getFunction('add42ToInput64')(handle, inputProof)).wait();
  step(`add42ToInput64 mined in block ${receipt?.blockNumber} with status ${receipt?.status}`);

  const result = String(await contract.getFunction('resUint64')());
  const [userValue, userMs] = await timed(() => userDecrypt(client, result, contractAddress, alice));
  step(`user decryption: ${String(userValue)} in ${userMs} ms`);
  const [publicValue, publicMs] = await timed(() => publicDecrypt(client, result));
  step(`public decryption: ${String(publicValue)} in ${publicMs} ms`);

  provider.destroy();
  step(`work done and provider closed; resources at start ${JSON.stringify(baseline)}`);
  // Printed on the next turn of the event loop, after the provider's handles are closed.
  setImmediate(() => step(`resources now ${JSON.stringify(activeResources())}; waiting for the process to exit`));
}

const started = Date.now();
process.on('exit', (code) => step(`process exited with code ${code} after ${Date.now() - started} ms`));
main().catch((error: unknown) => {
  console.error('[exit-probe] failed:', error);
  process.exitCode = 1;
});
