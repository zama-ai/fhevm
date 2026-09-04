import { createFhevmEncryptClient, hasFhevmRuntimeConfig, setFhevmRuntimeConfig } from '../../../src/ethers/index.js';
import { createLogger } from '../../fheTest/setupCommon.js';
import { createTfheMemoryReader } from '../support/wasmMemory.js';
import type { Scenario } from './scenario.js';

// ---------------------------------------------------------------------------
// Isolates a hypothesis raised by the `roundtrip` scenario
// ---------------------------------------------------------------------------
//
// `roundtrip` (varying plaintext + real on-chain tx + decryptValues +
// decryptPublicValues, all bundled together) showed accelerating tfheMemory
// growth, while `clientReuse` and `clientChurn` (both always encrypt the same
// fixed value(s) every iteration) stayed perfectly flat. A source-level
// investigation ruled out any JS-side cache keyed by plaintext value, any
// tfhe-module involvement in `decryptPublicValues` (it's tkms-only), and any
// code path that loops back into the encrypt module after `tx.wait()` — so
// none of those three are the direct cause.
//
// This scenario strips `roundtrip` down to the one remaining, untested
// variable: does encrypting a DIFFERENT plaintext value every iteration grow
// tfhe's WASM linear memory on its own, with no transaction and no decrypt at
// all? It also runs far faster than `roundtrip`, since it skips the
// tx.wait()/decrypt relayer round-trips that dominate that scenario's wall
// time — only the input-proof relayer call remains per iteration.
//
// One leading (not yet independently verified) explanation: the WASM-side
// allocator inside the tfhe module's linear memory serves the ZK range-proof
// computation's scratch buffers. Identical plaintext -> identical allocation
// sizes every time -> the allocator's free-list reuses freed blocks perfectly
// (matching the flat 0B/iter result from clientReuse/clientChurn). Varying
// plaintext -> the proof arithmetic's allocation shape depends on the value's
// bit pattern -> the heap fragments -> `memory.grow()` gets called, which is
// monotonic and never shrinks. If that's right, growth here should
// decelerate/plateau once the value cycle repeats (clearValue = counter %
// 256, so all 256 shapes get seen within one cycle) — a genuine unbounded
// leak wouldn't care about that boundary and would keep climbing past it.

export const valueChurnScenario: Scenario = {
  name: 'valueChurn',
  description:
    'One long-lived client; loops encryptValue with a different uint8 value every iteration. No tx submission, no decrypt.',
  defaultIterations: 400,
  defaultIterationsDuration: '~25 min',
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

    const client = createFhevmEncryptClient({
      chain: config.fhevmChain,
      provider: config.provider,
      options: config.moduleVersions !== undefined ? { moduleVersions: config.moduleVersions } : undefined,
    });
    await client.ready;

    const readTfheMemory = await createTfheMemoryReader(tfheVersion);

    let counter = 0;

    const iterate = async (): Promise<void> => {
      counter += 1;
      await client.encryptValue({
        contractAddress: config.fheTestAddress,
        userAddress: config.wallet.address,
        value: { type: 'uint8', value: counter % 256 },
      });
    };

    return { iterate, readTfheMemory };
  },
};
