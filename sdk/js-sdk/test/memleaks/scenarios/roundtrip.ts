import { createFhevmClient, hasFhevmRuntimeConfig, setFhevmRuntimeConfig } from '../../../src/ethers/index.js';
import { createLogger } from '../../fheTest/setupCommon.js';
import { createTfheMemoryReader, createTkmsMemoryReader } from '../support/wasmMemory.js';
import type { Scenario } from './scenario.js';

// Full network-bound cycle: encrypt -> submit tx -> wait for receipt ->
// user-decrypt -> public-decrypt, against the real FHETest contract on
// localstack. Exercises the ethers provider/signer/contract/tx-lifecycle code
// paths that the pure client-side scenarios (clientReuse/clientChurn) never
// touch, and confirms whatever leak-or-no-leak verdict those scenarios
// produce still holds once a real transaction is in the loop. Far fewer
// iterations than the other scenarios given per-tx latency and localstack
// throughput — this is not where most of the sample budget should go.

export const roundtripScenario: Scenario = {
  name: 'roundtrip',
  description: 'Full encrypt -> submit tx -> wait receipt -> user-decrypt + public-decrypt cycle against localstack.',
  defaultIterations: 100,
  defaultIterationsDuration: '~45 min',
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

    // Signed once and reused: a real app signs a decryption permit per
    // session, not per decrypt call. Permit signing has its own KMS wasm
    // surface, but that's providerChurn/clientChurn's concern, not this one.
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

    let counter = 0;

    const iterate = async (): Promise<void> => {
      counter += 1;
      const clearValue = counter % 256;

      const encrypted = await client.encryptValue({
        contractAddress: config.fheTestAddress,
        userAddress: config.wallet.address,
        value: { type: 'uint8', value: clearValue },
      });

      const tx = await config.fheTestContract.setEuint8!(
        encrypted.encryptedValue,
        encrypted.inputProof,
        clearValue,
        true,
      );
      const receipt = await tx.wait();
      if (receipt?.status !== 1) {
        throw new Error(`FHETest.setEuint8 transaction failed: ${tx.hash}`);
      }

      const [privateValue] = await client.decryptValues({
        encryptedValues: [encrypted.encryptedValue],
        contractAddress: config.fheTestAddress,
        signedPermit,
        transportKeyPair,
      });
      assertClearValueMatches('private decrypt', privateValue?.value, clearValue);

      const [publicValue] = await client.decryptPublicValues({ encryptedValues: [encrypted.encryptedValue] });
      assertClearValueMatches('public decrypt', publicValue?.value, clearValue);
    };

    return { iterate, readTfheMemory, readTkmsMemory };
  },
};

function assertClearValueMatches(label: string, actual: unknown, expected: number): void {
  if (String(actual) !== String(expected)) {
    throw new Error(`${label} mismatch: expected ${expected}, got ${String(actual)}`);
  }
}
