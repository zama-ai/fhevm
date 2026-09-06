// Scenario: dependency-chain load smoke — the live half of the `dep-chain` specimen
// (`solana/programs/dep-chain`), and the Solana analog of the EVM suite's SlowLaneContention:
// one `extend(links = 32)` carries the host's full step ceiling as a strictly DEPENDENT chain (each
// add reads the previous step's transient result). The coprocessor cannot parallelize any of it —
// the whole chain sits in the slow lane and must be computed in order before the tail handle's
// ciphertext materializes.
//
// While the chain grinds, the same wallet's counter goes through the same decrypt tail
// concurrently: the smoke fails if the deep chain wedges the pipeline for unrelated work, if any
// intermediate link is dropped (the tail cleartext is exact), or if the SNS commit of a 32-deep
// handle outlives the shared materialization budget.
//
// The Mollusk twin (`runtime-tests/tests/dep_chain_mollusk.rs`) proves the same dependent shape
// in-process at the ceiling; this scenario sends it against the running coprocessor, which is what
// the Mollusk twin cannot exercise.

import { describe, expect, test } from "bun:test";

import { userDecryptExpect } from "../../src/solana/fhe-vertical";
import {
  extendChain,
  incrementCounter,
  initializeChain,
  initializeCounter,
  MAX_CHAIN_LINKS,
} from "../../src/solana/specimens";
import { verticalSetup } from "../harness/solana/vertical";

// One 32-step execution + a concurrent single-step write, each waiting on its own SNS commit
// (up to 240s) + KMS round-trip.
const SCENARIO_TIMEOUT_MS = 15 * 60_000;

const STEP = 1n;
const TAIL = BigInt(MAX_CHAIN_LINKS) * STEP;

const hex = (bytes: Uint8Array): string => `0x${Buffer.from(bytes).toString("hex")}`;

describe("solana dependency-chain load smoke", () => {
  test(
    "32 dependent steps in one execution -> tail == 32, with unrelated work alongside",
    async () => {
      const { stack, context, wallet, config, secretKey } = await verticalSetup();

      await initializeChain(context, wallet.signer);
      const chain = await extendChain(context, wallet.signer, { links: MAX_CHAIN_LINKS, amount: STEP });

      // The unrelated fast-lane value, written AFTER the chain is in flight.
      await initializeCounter(context, wallet.signer);
      const bystander = await incrementCounter(context, wallet.signer, 7n);

      await Promise.all([stack.waitForSnsCommit(hex(chain.handle)), stack.waitForSnsCommit(hex(bystander.handle))]);
      const [tail, count] = await Promise.all([
        userDecryptExpect(config, { encryptedValue: chain.value.encryptedValue, handle: chain.handle, secretKey, expected: TAIL }),
        userDecryptExpect(config, {
          encryptedValue: bystander.value.encryptedValue,
          handle: bystander.handle,
          secretKey,
          expected: 7n,
        }),
      ]);
      expect(tail).toBe(TAIL);
      expect(count).toBe(7n);
    },
    SCENARIO_TIMEOUT_MS,
  );
});
