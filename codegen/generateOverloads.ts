type Test = {
  inputs: number[];
  output: number | boolean;
};

type SupportedFunctions = {
  [key: string]: SupportedFunction;
};

type SupportedFunctionParams = {
  supportedBits: number[];
  scalarSameType?: boolean;
  noScalar?: boolean;
  lhsHigher?: boolean;
  scalarOnly?: boolean;
  limit?: 'bits';
};

type SupportedFunction = SupportedFunctionParams &
  (
    | {
        unary?: false;
        evalTest: (lhs: number, rhs: number) => number | boolean;
      }
    | {
        unary: true;
        evalTest: (lhs: number, bits: number) => number | boolean;
      }
  );

export const SUPPORTED_UINT = [8, 16, 32, 64, 128, 256];
export const SUPPORTED_BITS = [4, 8, 16, 32, 64];

const generateNumber = (bits: number) => {
  return Math.max(Math.floor(Math.random() * (Math.pow(2, Math.min(bits, 31)) - 1)), 1);
};

const safeEval = (fn: (lhs: number, rhs: number) => number | boolean, lhs: number, rhs: number, bits: number) => {
  let result = fn(lhs, rhs);
  const logs: any[] = [];
  if (typeof result === 'number') {
    while ((result as number) > Math.pow(2, bits) - 1) {
      lhs = Math.max(Math.floor(lhs / 2), 1);
      rhs = Math.max(Math.floor(rhs / 2), 1);
      result = fn(lhs, rhs);
      logs.push([lhs, rhs, result]);
    }
  }
  return { inputs: [lhs, rhs], output: result };
};

export const SUPPORTED_FUNCTIONS: SupportedFunctions = {
  add: {
    supportedBits: SUPPORTED_BITS,
    evalTest: (lhs: number, rhs: number) => lhs + rhs,
  },
  sub: {
    supportedBits: SUPPORTED_BITS,
    lhsHigher: true,
    evalTest: (lhs: number, rhs: number) => lhs - rhs,
  },
  mul: {
    supportedBits: SUPPORTED_BITS,
    evalTest: (lhs: number, rhs: number) => lhs * rhs,
  },
  div: {
    supportedBits: SUPPORTED_BITS,
    evalTest: (lhs: number, rhs: number) => Math.floor(lhs / rhs),
    scalarOnly: true,
  },
  rem: {
    supportedBits: SUPPORTED_BITS,
    evalTest: (lhs: number, rhs: number) => lhs % rhs,
    scalarOnly: true,
  },
  le: {
    supportedBits: SUPPORTED_BITS,
    evalTest: (lhs: number, rhs: number) => lhs <= rhs,
  },
  lt: {
    supportedBits: SUPPORTED_BITS,
    evalTest: (lhs: number, rhs: number) => lhs < rhs,
  },
  ge: {
    supportedBits: SUPPORTED_BITS,
    evalTest: (lhs: number, rhs: number) => lhs >= rhs,
  },
  gt: {
    supportedBits: SUPPORTED_BITS,
    evalTest: (lhs: number, rhs: number) => lhs > rhs,
  },
  eq: {
    supportedBits: SUPPORTED_BITS,
    evalTest: (lhs: number, rhs: number) => lhs === rhs,
  },
  ne: {
    supportedBits: SUPPORTED_BITS,
    evalTest: (lhs: number, rhs: number) => lhs !== rhs,
  },
  shl: {
    supportedBits: SUPPORTED_BITS,
    limit: 'bits',
    evalTest: (lhs: number, rhs: number) => lhs >> rhs,
  },
  shr: {
    supportedBits: SUPPORTED_BITS,
    limit: 'bits',
    evalTest: (lhs: number, rhs: number) => lhs >> rhs,
  },
  max: {
    supportedBits: SUPPORTED_BITS,
    unary: false,
    evalTest: (lhs: number, rhs: number) => (lhs > rhs ? lhs : rhs),
  },
  min: {
    supportedBits: SUPPORTED_BITS,
    evalTest: (lhs: number, rhs: number) => (lhs < rhs ? lhs : rhs),
  },
  or: {
    supportedBits: SUPPORTED_BITS,
    // scalarSameType: true,
    noScalar: true,
    evalTest: (lhs: number, rhs: number) => lhs | rhs,
  },
  and: {
    supportedBits: SUPPORTED_BITS,
    noScalar: true,
    evalTest: (lhs: number, rhs: number) => lhs & rhs,
  },
  xor: {
    supportedBits: SUPPORTED_BITS,
    noScalar: true,
    evalTest: (lhs: number, rhs: number) => lhs ^ rhs,
  },
  not: {
    supportedBits: SUPPORTED_BITS,
    unary: true,
    evalTest: (lhs: number, bits: number) => (~lhs >>> 0) & (Math.pow(2, Math.min(bits, 31)) - 1),
  },
  neg: {
    supportedBits: SUPPORTED_BITS,
    unary: true,
    evalTest: (lhs: number, bits: number) => (~lhs >>> 0) & (Math.pow(2, Math.min(bits, 31)) - 1),
  },
};

export const generateTests = () => {
  const tests: any = {};
  Object.keys(SUPPORTED_FUNCTIONS).forEach((functionName: string) => {
    const test = SUPPORTED_FUNCTIONS[functionName];
    test.supportedBits.forEach((lhs: number) => {
      let lhsNumber = generateNumber(lhs);
      if (test.unary) {
        const encryptedTestName = [functionName, `euint${lhs}`].join('_');
        const encryptedTests: Test[] = [];
        encryptedTests.push({
          inputs: [lhsNumber],
          output: test.evalTest(lhsNumber, lhs),
        });
        tests[encryptedTestName] = encryptedTests;
      } else {
        test.supportedBits.forEach((rhs: number) => {
          const bitResults = Math.min(lhs, rhs);
          let rhsNumber = generateNumber(rhs);
          if (test.limit === 'bits') {
            rhsNumber = 1 + Math.floor(Math.random() * (rhs - 1));
          }
          const smallest = Math.max(Math.min(lhsNumber, rhsNumber), 8);
          const only8bits = test.limit === 'bits' && rhs === 8;
          const onlyEncrypted8bits = only8bits && lhs > 4;

          if ((test.limit !== 'bits' || onlyEncrypted8bits) && !test.scalarOnly) {
            const encryptedTestName = [functionName, `euint${lhs}`, `euint${rhs}`].join('_');
            const encryptedTests: Test[] = [];
            if (!test.lhsHigher) {
              encryptedTests.push(safeEval(test.evalTest, lhsNumber, rhsNumber, bitResults));
              encryptedTests.push(safeEval(test.evalTest, smallest - 4, smallest, bitResults));
            }
            encryptedTests.push(safeEval(test.evalTest, smallest, smallest, bitResults));
            encryptedTests.push(safeEval(test.evalTest, smallest, smallest - 4, bitResults));
            tests[encryptedTestName] = encryptedTests;
          }

          const scalarCondition =
            !test.noScalar &&
            (lhs === rhs || (!test.scalarSameType && ((rhs == 8 && lhs == 4) || (rhs == 4 && lhs == 8))));

          if (SUPPORTED_UINT.includes(rhs) && (only8bits || (test.limit !== 'bits' && scalarCondition))) {
            rhsNumber = generateNumber(bitResults);
            const encryptedTestName = [functionName, `euint${lhs}`, `uint${rhs}`].join('_');
            const encryptedTests: Test[] = [];
            if (!test.lhsHigher) {
              encryptedTests.push(safeEval(test.evalTest, lhsNumber, rhsNumber, bitResults));
              encryptedTests.push(safeEval(test.evalTest, smallest - 4, smallest, bitResults));
            }
            encryptedTests.push(safeEval(test.evalTest, smallest, smallest, bitResults));
            encryptedTests.push(safeEval(test.evalTest, smallest, smallest - 4, bitResults));
            tests[encryptedTestName] = encryptedTests;
          }
          if (SUPPORTED_UINT.includes(lhs) && test.limit !== 'bits' && scalarCondition && !test.scalarOnly) {
            lhsNumber = generateNumber(bitResults);
            const encryptedTestName = [functionName, `uint${lhs}`, `euint${rhs}`].join('_');
            const encryptedTests: Test[] = [];
            if (!test.lhsHigher) {
              encryptedTests.push(safeEval(test.evalTest, lhsNumber, rhsNumber, bitResults));
              encryptedTests.push(safeEval(test.evalTest, smallest - 4, smallest, bitResults));
            }
            encryptedTests.push(safeEval(test.evalTest, smallest, smallest, bitResults));
            encryptedTests.push(safeEval(test.evalTest, smallest, smallest - 4, bitResults));
            tests[encryptedTestName] = encryptedTests;
          }
        });
      }
    });
  });
  return tests;
};

const tests = generateTests();

const fs = require('fs');
const path = require('path');

fs.writeFileSync(`${path.resolve(__dirname)}/overloads.json`, JSON.stringify(tests));
