// Scenario: representative fhe_execute operator rows — the operator table ported from
// `solana/scripts/e2e/full-vertical.sh` (run_binary/run_unary + the ternary/rand/composite
// phases). Operator SEMANTICS are exhaustive elsewhere (operator_conformance covers every op;
// Mollusk proves SBF admission; real_tfhe_conformance proves representative cryptographic
// execution) — this expensive live vertical keeps only the materially distinct
// network/listener/worker/decrypt WIRING shapes, one row each:
//
//   Binary enc/enc      Sub(enc 100, enc 30)            == 70
//   Binary enc/scalar   Mul(enc 6, scalar 7)            == 42
//   Unary cast          Cast(enc 42, euint8 -> euint16) == 42
//   Ternary             IfThenElse(enc 1, 42, 99)       == 42
//   Nullary random      RandBounded(128)                 < 128
//   N-ary composite     Sum(enc 10, enc 20)             == 30
//   Set composite       IsIn(enc 42, [10, 42, 100])     == 1
//   Fused composite     MulDiv(enc 6, scalar 7, / 3)    == 14
//
// Wiring (the live-client's shape, now typed): every encrypted operand is created first as its
// own persistent trivial-encrypt execution, then referenced as a `StoredValue` operand — the
// dictionary holds its current handle, the transaction carries its account read-only. The output
// binds writable to a fresh scenario-owned value. Each row ends in the full release tail:
// allow_subjects + make_handle_public, SNS commit wait, proof-service inclusion proof, and the
// KMS public-decrypt certificate — so a row passing means the listener/worker/decrypt pipeline
// materialized that operator's DAG end to end, replacing the bash's
// "result handle"/"allow_for_decryption" greps and `assert_decrypt` cleartext comparisons.

import { describe, expect, test } from "bun:test";

import {
  ExecutionDictionary,
  FHE_TYPE,
  FheBinaryOpCode,
  FheTernaryOpCode,
  FheUnaryOpCode,
  persistentOutput,
  persistentValueTarget,
  scalarBytes,
  sendFheExecute,
  type FheExecuteOutputArgs,
  type FheExecuteStepArgs,
} from "../../src/solana/fhe-execute";
import { currentHandle, paddedLabel, releaseAndExpect, trivialEncryptPersistent, type PersistentHandle } from "../../src/solana/fhe-vertical";
import { verticalSetup, type VerticalTestSetup } from "../harness/solana/vertical";

// Each row runs 1-4 operand executions + the operator execution + SNS commit wait + KMS decrypt.
const ROW_TIMEOUT_MS = 15 * 60_000;

/** A `StoredValue` reference to an already-created operand: dictionary handle + account index. */
type StoredValueRef = { __kind: "StoredValue"; handleIndex: number; encryptedValueIndex: number };

/** Creates one encrypted operand as its own persistent trivial-encrypt execution. */
const encryptOperand = (
  setup: VerticalTestSetup,
  value: bigint,
  name: string,
  fheType: number = FHE_TYPE.euint64,
): Promise<PersistentHandle> =>
  trivialEncryptPersistent(setup.context, {
    payer: setup.wallet.signer,
    value,
    label: paddedLabel(name),
    fheType,
  });

/**
 * Runs one single-step operator execution: `operands` become read-only `StoredValue` references
 * (in order), the output binds writable to a fresh value labeled `outputName`, and `buildStep`
 * assembles the step from those parts. Returns the output's persistent handle.
 */
const runOperatorStep = async (
  setup: VerticalTestSetup,
  params: {
    readonly outputName: string;
    readonly operands: readonly PersistentHandle[];
    buildStep(
      refs: readonly StoredValueRef[],
      dictionary: ExecutionDictionary,
      output: FheExecuteOutputArgs,
    ): FheExecuteStepArgs;
  },
): Promise<PersistentHandle> => {
  const { context, wallet } = setup;
  const target = await persistentValueTarget(wallet.signer.address, wallet.signer.address, paddedLabel(params.outputName));
  const dictionary = new ExecutionDictionary();
  const refs = params.operands.map<StoredValueRef>((operand, index) => ({
    __kind: "StoredValue",
    handleIndex: dictionary.intern(operand.handle),
    encryptedValueIndex: index,
  }));
  const output = await persistentOutput(context, dictionary, {
    target,
    encryptedValueIndex: params.operands.length,
    subjects: [wallet.signer.address],
  });
  await sendFheExecute(context, {
    payer: wallet.signer,
    dictionary,
    steps: [params.buildStep(refs, dictionary, output)],
    remainingAccounts: [
      ...params.operands.map((operand) => ({ address: operand.target.encryptedValue, writable: false })),
      { address: target.encryptedValue, writable: true },
    ],
  });
  return { target, handle: await currentHandle(context, target.encryptedValue) };
};

/** The release tail every row shares: allow + seal, SNS commit wait, public-decrypt comparison. */
const decryptRow = async (
  setup: VerticalTestSetup,
  result: PersistentHandle,
  expected: bigint | { readonly lessThan: bigint },
): Promise<bigint> => {
  const outcome = await releaseAndExpect(setup.context, setup.config, setup.stack, {
    payer: setup.wallet.signer,
    result,
    expect: expected,
  });
  return outcome.cleartext;
};

describe("solana fhe_execute operator wiring", () => {
  test(
    "binary enc/enc: Sub(enc 100, enc 30) == 70",
    async () => {
      const setup = await verticalSetup();
      const [lhs, rhs] = await Promise.all([
        encryptOperand(setup, 100n, "op-sub-lhs"),
        encryptOperand(setup, 30n, "op-sub-rhs"),
      ]);
      const result = await runOperatorStep(setup, {
        outputName: "op-sub-out",
        operands: [lhs, rhs],
        buildStep: ([lhsRef, rhsRef], _dictionary, output) => ({
          __kind: "Binary",
          op: FheBinaryOpCode.Sub,
          lhs: lhsRef,
          rhs: rhsRef,
          outputFheType: FHE_TYPE.euint64,
          output,
        }),
      });
      expect(await decryptRow(setup, result, 70n)).toBe(70n);
    },
    ROW_TIMEOUT_MS,
  );

  test(
    "binary enc/scalar: Mul(enc 6, scalar 7) == 42",
    async () => {
      const setup = await verticalSetup();
      const lhs = await encryptOperand(setup, 6n, "op-mul-lhs");
      const result = await runOperatorStep(setup, {
        outputName: "op-mul-out",
        operands: [lhs],
        buildStep: ([lhsRef], dictionary, output) => ({
          __kind: "Binary",
          op: FheBinaryOpCode.Mul,
          lhs: lhsRef,
          rhs: { __kind: "Scalar", valueIndex: dictionary.intern(scalarBytes(7n)) },
          outputFheType: FHE_TYPE.euint64,
          output,
        }),
      });
      expect(await decryptRow(setup, result, 42n)).toBe(42n);
    },
    ROW_TIMEOUT_MS,
  );

  test(
    "unary: Cast(enc 42, euint8 -> euint16) == 42",
    async () => {
      const setup = await verticalSetup();
      const operand = await encryptOperand(setup, 42n, "op-cast-in", FHE_TYPE.euint8);
      const result = await runOperatorStep(setup, {
        outputName: "op-cast-out",
        operands: [operand],
        buildStep: ([operandRef], _dictionary, output) => ({
          __kind: "Unary",
          op: FheUnaryOpCode.Cast,
          operand: operandRef,
          outputFheType: FHE_TYPE.euint16,
          output,
        }),
      });
      expect(await decryptRow(setup, result, 42n)).toBe(42n);
    },
    ROW_TIMEOUT_MS,
  );

  test(
    "ternary: IfThenElse(enc 1, enc 42, enc 99) == 42",
    async () => {
      const setup = await verticalSetup();
      const [control, ifTrue, ifFalse] = await Promise.all([
        encryptOperand(setup, 1n, "op-ite-ctrl", FHE_TYPE.ebool),
        encryptOperand(setup, 42n, "op-ite-true"),
        encryptOperand(setup, 99n, "op-ite-false"),
      ]);
      const result = await runOperatorStep(setup, {
        outputName: "op-ite-out",
        operands: [control, ifTrue, ifFalse],
        buildStep: ([controlRef, ifTrueRef, ifFalseRef], _dictionary, output) => ({
          __kind: "Ternary",
          op: FheTernaryOpCode.IfThenElse,
          control: controlRef,
          ifTrue: ifTrueRef,
          ifFalse: ifFalseRef,
          outputFheType: FHE_TYPE.euint64,
          output,
        }),
      });
      expect(await decryptRow(setup, result, 42n)).toBe(42n);
    },
    ROW_TIMEOUT_MS,
  );

  test(
    "nullary random: RandBounded(128) < 128",
    async () => {
      const setup = await verticalSetup();
      const result = await runOperatorStep(setup, {
        outputName: "op-rand-out",
        operands: [],
        buildStep: (_refs, _dictionary, output) => ({
          __kind: "RandBounded",
          upperBound: scalarBytes(128n),
          fheType: FHE_TYPE.euint64,
          output,
        }),
      });
      const cleartext = await decryptRow(setup, result, { lessThan: 128n });
      expect(cleartext).toBeLessThan(128n);
    },
    ROW_TIMEOUT_MS,
  );

  test(
    "n-ary composite: Sum(enc 10, enc 20) == 30",
    async () => {
      const setup = await verticalSetup();
      const [a, b] = await Promise.all([
        encryptOperand(setup, 10n, "op-sum-a"),
        encryptOperand(setup, 20n, "op-sum-b"),
      ]);
      const result = await runOperatorStep(setup, {
        outputName: "op-sum-out",
        operands: [a, b],
        buildStep: (refs, _dictionary, output) => ({
          __kind: "Sum",
          operands: [...refs],
          fheType: FHE_TYPE.euint64,
          output,
        }),
      });
      expect(await decryptRow(setup, result, 30n)).toBe(30n);
    },
    ROW_TIMEOUT_MS,
  );

  test(
    "set composite: IsIn(enc 42, [enc 10, enc 42, enc 100]) == 1",
    async () => {
      const setup = await verticalSetup();
      const [value, ...set] = await Promise.all([
        encryptOperand(setup, 42n, "op-isin-value"),
        encryptOperand(setup, 10n, "op-isin-set-0"),
        encryptOperand(setup, 42n, "op-isin-set-1"),
        encryptOperand(setup, 100n, "op-isin-set-2"),
      ]);
      const result = await runOperatorStep(setup, {
        outputName: "op-isin-out",
        operands: [value, ...set],
        buildStep: ([valueRef, ...setRefs], _dictionary, output) => ({
          __kind: "IsIn",
          value: valueRef,
          set: [...setRefs],
          fheType: FHE_TYPE.euint64,
          output,
        }),
      });
      expect(await decryptRow(setup, result, 1n)).toBe(1n);
    },
    ROW_TIMEOUT_MS,
  );

  test(
    "fused composite: MulDiv(enc 6, scalar 7, divisor 3) == 14",
    async () => {
      const setup = await verticalSetup();
      const factor1 = await encryptOperand(setup, 6n, "op-muldiv-f1");
      const result = await runOperatorStep(setup, {
        outputName: "op-muldiv-out",
        operands: [factor1],
        buildStep: ([factor1Ref], dictionary, output) => ({
          __kind: "MulDiv",
          factor1: factor1Ref,
          factor2: { __kind: "Scalar", valueIndex: dictionary.intern(scalarBytes(7n)) },
          divisor: scalarBytes(3n),
          outputFheType: FHE_TYPE.euint64,
          output,
        }),
      });
      expect(await decryptRow(setup, result, 14n)).toBe(14n);
    },
    ROW_TIMEOUT_MS,
  );
});
