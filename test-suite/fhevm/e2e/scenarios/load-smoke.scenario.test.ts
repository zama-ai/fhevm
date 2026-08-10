// Scenario: dependency-chain load smoke — the live half of the `dep-chain` specimen
// (`solana/programs/dep-chain`), and the Solana analog of the EVM suite's SlowLaneContention:
// one `fhe_execute` carrying the host's full 32-step ceiling as a strictly DEPENDENT chain
// (TrivialEncrypt seed, then 31 adds, each reading the previous step's transient result via an
// `EarlierStep` operand). The coprocessor cannot parallelize any of it — the whole chain sits in
// the slow lane and must be computed in order before the tail handle's ciphertext materializes.
//
// While the chain grinds, an independent single-step value goes through the same release tail
// concurrently: the smoke fails if the deep chain wedges the pipeline for unrelated work, if any
// intermediate link is dropped (the tail cleartext is exact), or if the SNS commit of a
// 32-deep handle outlives the shared 240s materialization budget.
//
// The Mollusk twin (`runtime-tests/tests/dep_chain_mollusk.rs`) proves the same dependent shape
// in-process through the specimen program's CPI at the on-chain builder's 16-step ceiling
// (`zama_fhe::MAX_ON_CHAIN_EXECUTION_STEPS`, Anchor's default-heap budget); this scenario builds
// the execution OFF-chain through the raw typed client, which is what makes the host's full
// 32-step ceiling reachable — and means the live lane needs no extra program deployment.

import { describe, expect, test } from "bun:test";

import {
  ExecutionDictionary,
  FHE_TYPE,
  FheBinaryOpCode,
  persistentOutput,
  persistentValueTarget,
  scalarBytes,
  sendFheExecute,
  type FheExecuteStepArgs,
} from "../../src/solana/fhe-execute";
import { currentHandle, paddedLabel, releaseAndExpect, trivialEncryptPersistent } from "../../src/solana/fhe-vertical";
import { verticalSetup } from "../harness/solana/vertical";

// One 32-step execution + a concurrent single-step release tail, each waiting on its own SNS
// commit (up to 240s) + KMS certificate round-trip.
const SCENARIO_TIMEOUT_MS = 15 * 60_000;

// The host's per-execution step ceiling (zama_host MAX_FHE_EXECUTION_STEPS): the seed encrypt
// plus 31 dependent adds.
const CHAIN_STEPS = 32;
const SEED = 5n;
const STEP = 1n;

describe("solana dependency-chain load smoke", () => {
  test(
    "32 dependent steps in one execution -> tail == seed + 31, with unrelated work alongside",
    async () => {
      const { stack, context, wallet, config } = await verticalSetup();

      const target = await persistentValueTarget(
        wallet.signer.address,
        wallet.signer.address,
        paddedLabel("load-smoke-chain"),
      );
      const dictionary = new ExecutionDictionary();
      const output = await persistentOutput(context, dictionary, {
        target,
        encryptedValueIndex: 0,
        subjects: [wallet.signer.address],
      });
      const stepScalar = dictionary.intern(scalarBytes(STEP));
      const steps: FheExecuteStepArgs[] = [
        {
          __kind: "TrivialEncrypt",
          plaintext: scalarBytes(SEED),
          fheType: FHE_TYPE.euint64,
          output: { __kind: "Transient" },
        },
      ];
      for (let index = 1; index < CHAIN_STEPS; index += 1) {
        steps.push({
          __kind: "Binary",
          op: FheBinaryOpCode.Add,
          lhs: { __kind: "EarlierStep", producerIndex: index - 1 },
          rhs: { __kind: "Scalar", valueIndex: stepScalar },
          outputFheType: FHE_TYPE.euint64,
          output: index === CHAIN_STEPS - 1 ? output : { __kind: "Transient" },
        });
      }
      await sendFheExecute(context, {
        payer: wallet.signer,
        dictionary,
        steps,
        remainingAccounts: [{ address: target.encryptedValue, writable: true }],
      });
      const chainHandle = await currentHandle(context, target.encryptedValue);

      // The unrelated fast-lane value, created AFTER the chain is in flight.
      const bystander = await trivialEncryptPersistent(context, {
        payer: wallet.signer,
        value: 7n,
        label: paddedLabel("load-smoke-bystander"),
      });

      const [chainOutcome, bystanderOutcome] = await Promise.all([
        releaseAndExpect(context, config, stack, {
          payer: wallet.signer,
          result: { target, handle: chainHandle },
          expect: SEED + BigInt(CHAIN_STEPS - 1) * STEP,
        }),
        releaseAndExpect(context, config, stack, {
          payer: wallet.signer,
          result: bystander,
          expect: 7n,
        }),
      ]);
      expect(chainOutcome.cleartext).toBe(36n);
      expect(bystanderOutcome.cleartext).toBe(7n);
    },
    SCENARIO_TIMEOUT_MS,
  );
});
