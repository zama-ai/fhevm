import { createFhevmDecryptClient, hasFhevmRuntimeConfig, setFhevmRuntimeConfig } from '../../../src/ethers/index.js';
import { createLogger } from '../../fheTest/setupCommon.js';
import { createTkmsMemoryReader } from '../support/wasmMemory.js';
import type { Scenario } from './scenario.js';

// ---------------------------------------------------------------------------
// Isolates the leak surface `roundtrip` deliberately avoids
// ---------------------------------------------------------------------------
//
// `roundtrip` signs one decryption permit in its setup() and reuses it across
// every iteration — both for realism (a real app signs a permit once per
// session, not once per decrypt) and to avoid blending this leak surface into
// the encrypt/tx/decrypt cycle that scenario exists to test. This scenario is
// the deliberate opposite: one long-lived client, but a FRESH
// `generateTransportKeyPair()` + `signLegacyDecryptionPermit()` +
// `signUnifiedDecryptionPermit()` every iteration — no transaction, no
// relayer decrypt call at all. It isolates the ML-KEM transport-keypair
// generation and both EIP-712 permit-signing paths (legacy V1 and unified
// V2) on their own, on the tkms side of the WASM boundary
// (`clientChurn`/`valueChurn` only ever exercised the tfhe/encrypt side).
//
// All operations here are purely local (WASM keygen + ethers signatures) —
// none touch the relayer over the network — so this should run much faster
// per iteration than any of the relayer-bound scenarios. Each iteration also
// round-trips the transport key pair and both signed permits through
// serialize/parse, to exercise the (de)serialization path on the tkms side
// of the WASM boundary alongside keygen and signing.

export const permitChurnScenario: Scenario = {
  name: 'permitChurn',
  description:
    'One long-lived client; loops generateTransportKeyPair + signLegacyDecryptionPermit + signUnifiedDecryptionPermit every iteration, round-tripping the key pair and both permits through serialize/parse. No tx, no decrypt call.',
  defaultIterations: 9_000,
  defaultIterationsDuration: '~1 min',
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

    const client = createFhevmDecryptClient({
      chain: config.fhevmChain,
      provider: config.provider,
      options: config.moduleVersions !== undefined ? { moduleVersions: config.moduleVersions } : undefined,
    });
    await client.ready;

    const readTkmsMemory = await createTkmsMemoryReader();

    const iterate = async (): Promise<void> => {
      const transportKeyPair = await client.generateTransportKeyPair();

      const serializedTransportKeyPair = await client.serializeTransportKeyPair({ transportKeyPair });
      const parsedTransportKeyPair = await client.parseTransportKeyPair(serializedTransportKeyPair);

      const signedLegacyPermit = await client.signLegacyDecryptionPermit({
        transportKeyPair: parsedTransportKeyPair,
        contractAddresses: [config.fheTestAddress],
        durationSeconds: 24 * 3600,
        startTimestamp: Math.floor(Date.now() / 1000) - 5,
        signerAddress: config.wallet.address,
        signer: config.signer,
      });

      const serializedLegacyPermit = await client.serializeSignedDecryptionPermit({
        signedPermit: signedLegacyPermit,
      });
      await client.parseSignedDecryptionPermit({
        serializedPermit: serializedLegacyPermit,
        transportKeyPair: parsedTransportKeyPair,
      });

      const signedUnifiedPermit = await client.signUnifiedDecryptionPermit({
        transportKeyPair: parsedTransportKeyPair,
        contractAddresses: [config.fheTestAddress],
        durationSeconds: 24 * 3600,
        startTimestamp: Math.floor(Date.now() / 1000) - 5,
        signerAddress: config.wallet.address,
        signer: config.signer,
      });

      const serializedUnifiedPermit = await client.serializeSignedDecryptionPermit({
        signedPermit: signedUnifiedPermit,
      });
      await client.parseSignedDecryptionPermit({
        serializedPermit: serializedUnifiedPermit,
        transportKeyPair: parsedTransportKeyPair,
      });
    };

    return { iterate, readTkmsMemory };
  },
};
