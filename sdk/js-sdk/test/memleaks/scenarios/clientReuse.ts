import type { ethers } from 'ethers';
import { createFhevmClient, hasFhevmRuntimeConfig, setFhevmRuntimeConfig } from '../../../src/ethers/index.js';
import { createLogger, encryptTestCases, fheTypeIdFromName } from '../../fheTest/setupCommon.js';
import { createTfheMemoryReader, createTkmsMemoryReader } from '../support/wasmMemory.js';
import type { Scenario } from './scenario.js';

// Mode A (control group): one long-lived client, looping encryptValues +
// decryptValues. Targets the per-*operation* code path — building/parsing a
// ciphertext list (`buildWithProofPacked` in
// src/core/modules/encrypt/module/api-p.ts) — which frees its wasm-bindgen
// objects in a `finally` and reads clean from the source. This scenario
// exists to prove the detector doesn't just flag everything: if this one
// grows unbounded too, the detector or the harness itself is suspect, not
// just the client-churn code path.

export const clientReuseScenario: Scenario = {
  name: 'clientReuse',
  description: 'One long-lived client; loops encryptValues + decryptValues against a stable on-chain handle.',
  defaultIterations: 200,
  defaultIterationsDuration: '~1h',
  setup: async ({ config }) => {
    // Process-wide singleton: when running multiple scenarios in one `main.ts`
    // invocation (e.g. `--scenario all`), only the first scenario's setup()
    // may call this — a later call with a fresh `createLogger()` reference
    // would throw even though the effective config is identical.
    if (!hasFhevmRuntimeConfig()) {
      setFhevmRuntimeConfig({
        auth: { type: 'ApiKeyHeader', value: config.zamaApiKey },
        logger: createLogger(console.log, config.chainName),
      });
    }

    const tfheVersion =
      config.moduleVersions !== undefined && config.moduleVersions !== 'auto' ? config.moduleVersions.tfhe : undefined;

    const client = createFhevmClient({
      chain: config.fhevmChain,
      provider: config.provider,
      options: config.moduleVersions !== undefined ? { moduleVersions: config.moduleVersions } : undefined,
    });
    await client.ready;

    // A stable, pre-existing handle (created by setupCommon's initFheTest
    // preflight for Alice) so every iteration decrypts the same real on-chain
    // value instead of needing a fresh transaction per iteration.
    const fheTest = config.fheTestContract.connect(config.signer) as ethers.Contract;
    const stableHandle: string = await fheTest.getHandleOf!(config.wallet.address, fheTypeIdFromName('euint64'));

    const transportKeyPair = await client.generateTransportKeyPair();
    const signedPermit = await client.signLegacyDecryptionPermit({
      transportKeyPair,
      contractAddresses: [config.fheTestAddress],
      durationSeconds: 24 * 3600,
      startTimestamp: Math.floor(Date.now() / 1000) - 5,
      signerAddress: config.wallet.address,
      signer: config.signer,
    });

    const [readTfheMemory, readTkmsMemory] = await Promise.all([
      createTfheMemoryReader(tfheVersion),
      createTkmsMemoryReader(),
    ]);

    const iterate = async (): Promise<void> => {
      await client.encryptValues({
        contractAddress: config.fheTestAddress,
        userAddress: config.wallet.address,
        values: encryptTestCases,
      });

      await client.decryptValues({
        encryptedValues: [stableHandle],
        contractAddress: config.fheTestAddress,
        signedPermit,
        transportKeyPair,
      });
    };

    return { iterate, readTfheMemory, readTkmsMemory };
  },
};
