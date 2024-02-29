type Test = {
  inputs: number[];
  output: number | boolean | bigint;
};

type SupportedFunctions = {
  [key: string]: SupportedFunction;
};

type SupportedFunctionParams = {
  supportedBits: number[];
  safeMin?: boolean;
  noScalar?: boolean;
  lhsHigher?: boolean;
  scalarOnly?: boolean;
  limit?: 'bits';
};

type SupportedFunction = SupportedFunctionParams &
  (
    | {
        unary?: false;
        evalTest: (lhsNumber: number, rhsNumber: number, lhs: number, rhs: number) => number | boolean | bigint;
      }
    | {
        unary: true;
        evalTest: (lhs: number, bits: number) => number | boolean | bigint;
      }
  );

(BigInt as any).prototype['toJSON'] = function () {
  return this.toString();
};

const SUPPORTED_UINT = [8, 16, 32, 64, 128, 256];
const SUPPORTED_BITS = [4, 8, 16, 32, 64];

const generateNumber = (bits: number) => {
  return Math.max(Math.floor(Math.random() * (Math.pow(2, Math.min(bits, 28)) - 1)), 1);
};

const safeEval = (
  fn: (lhsNumber: number, rhsNumber: number, lhs: number, rhs: number) => number | boolean | bigint,
  lhsNumber: number,
  rhsNumber: number,
  lhs: number,
  rhs: number,
  safeMin: boolean = false,
) => {
  const bitResults = safeMin ? Math.min(lhs, rhs) : Math.max(lhs, rhs);
  let result = fn(lhsNumber, rhsNumber, lhs, rhs);
  const logs: any[] = [];
  if (typeof result === 'number') {
    while ((result as number) > Math.pow(2, bitResults) - 1) {
      lhsNumber = Math.max(Math.floor(lhsNumber / 2), 1);
      rhsNumber = Math.max(Math.floor(rhsNumber / 2), 1);
      result = fn(lhsNumber, rhsNumber, lhs, rhs);
      logs.push([lhsNumber, rhsNumber, result]);
    }
  }
  return { inputs: [lhsNumber, rhsNumber], output: result };
};

export const SUPPORTED_FUNCTIONS: SupportedFunctions = {
  add: {
    supportedBits: SUPPORTED_BITS,
    safeMin: true,
    evalTest: (lhsNumber: number, rhsNumber: number) => lhsNumber + rhsNumber,
  },
  sub: {
    supportedBits: SUPPORTED_BITS,
    lhsHigher: true,
    evalTest: (lhsNumber: number, rhsNumber: number) => lhsNumber - rhsNumber,
  },
  mul: {
    supportedBits: SUPPORTED_BITS,
    safeMin: true,
    evalTest: (lhsNumber: number, rhsNumber: number) => lhsNumber * rhsNumber,
  },
  div: {
    supportedBits: SUPPORTED_BITS,
    evalTest: (lhsNumber: number, rhsNumber: number) => Math.floor(lhsNumber / rhsNumber),
    scalarOnly: true,
  },
  rem: {
    supportedBits: SUPPORTED_BITS,
    evalTest: (lhsNumber: number, rhsNumber: number) => lhsNumber % rhsNumber,
    scalarOnly: true,
  },
  le: {
    supportedBits: SUPPORTED_BITS,
    evalTest: (lhsNumber: number, rhsNumber: number) => lhsNumber <= rhsNumber,
  },
  lt: {
    supportedBits: SUPPORTED_BITS,
    evalTest: (lhsNumber: number, rhsNumber: number) => lhsNumber < rhsNumber,
  },
  ge: {
    supportedBits: SUPPORTED_BITS,
    evalTest: (lhsNumber: number, rhsNumber: number) => lhsNumber >= rhsNumber,
  },
  gt: {
    supportedBits: SUPPORTED_BITS,
    evalTest: (lhsNumber: number, rhsNumber: number) => lhsNumber > rhsNumber,
  },
  eq: {
    supportedBits: SUPPORTED_BITS,
    evalTest: (lhsNumber: number, rhsNumber: number) => lhsNumber === rhsNumber,
  },
  ne: {
    supportedBits: SUPPORTED_BITS,
    evalTest: (lhsNumber: number, rhsNumber: number) => lhsNumber !== rhsNumber,
  },
  shl: {
    supportedBits: SUPPORTED_BITS,
    limit: 'bits',
    evalTest: (lhsNumber: number, rhsNumber: number, lhs: number, rhs: number) => {
      const bits = `${new Array(256).fill('0').join('')}${lhsNumber.toString(2)}`.slice(-lhs).split('');
      const r = bits.map((_, index) => {
        const newIndex = index + (rhsNumber % lhs);
        return newIndex >= bits.length ? '0' : bits[newIndex];
      });
      return parseInt(r.join(''), 2);
    },
  },
  shr: {
    supportedBits: SUPPORTED_BITS,
    limit: 'bits',
    evalTest: (lhsNumber: number, rhsNumber: number, lhs: number, rhs: number) => {
      const bits = `${new Array(256).fill('0').join('')}${lhsNumber.toString(2)}`.slice(-lhs).split('');
      const r = bits.map((_, index) => {
        const newIndex = index - (rhsNumber % lhs);
        return newIndex < 0 ? '0' : bits[newIndex];
      });
      return parseInt(r.join(''), 2);
    },
  },
  max: {
    supportedBits: SUPPORTED_BITS,
    unary: false,
    evalTest: (lhsNumber: number, rhsNumber: number) => (lhsNumber > rhsNumber ? lhsNumber : rhsNumber),
  },
  min: {
    supportedBits: SUPPORTED_BITS,
    evalTest: (lhsNumber: number, rhsNumber: number) => (lhsNumber < rhsNumber ? lhsNumber : rhsNumber),
  },
  or: {
    supportedBits: SUPPORTED_BITS,
    noScalar: true,
    evalTest: (lhsNumber: number, rhsNumber: number) => lhsNumber | rhsNumber,
  },
  and: {
    supportedBits: SUPPORTED_BITS,
    noScalar: true,
    evalTest: (lhsNumber: number, rhsNumber: number) => lhsNumber & rhsNumber,
  },
  xor: {
    supportedBits: SUPPORTED_BITS,
    noScalar: true,
    evalTest: (lhsNumber: number, rhsNumber: number) => lhsNumber ^ rhsNumber,
  },
  not: {
    supportedBits: SUPPORTED_BITS,
    unary: true,
    evalTest: (lhsNumber: number, bits: number) => {
      const val = `${new Array(256).fill('0').join('')}${lhsNumber.toString(2)}`.slice(-bits).split('');
      return BigInt(
        `0b${val
          .map((v) => {
            if (v === '1') return '0';
            return '1';
          })
          .join('')}`,
      );
    },
  },
  neg: {
    supportedBits: SUPPORTED_BITS,
    unary: true,
    evalTest: (lhsNumber: number, bits: number) => {
      const val = `${new Array(256).fill('0').join('')}${lhsNumber.toString(2)}`.slice(-bits).split('');
      return (
        BigInt(
          `0b${val
            .map((v) => {
              if (v === '1') return '0';
              return '1';
            })
            .join('')}`,
        ) + 1n
      );
    },
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
              encryptedTests.push(safeEval(test.evalTest, lhsNumber, rhsNumber, lhs, rhs, test.safeMin));
              encryptedTests.push(safeEval(test.evalTest, smallest - 4, smallest, lhs, rhs, test.safeMin));
            }
            encryptedTests.push(safeEval(test.evalTest, smallest, smallest, lhs, rhs, test.safeMin));
            encryptedTests.push(safeEval(test.evalTest, smallest, smallest - 4, lhs, rhs, test.safeMin));
            tests[encryptedTestName] = encryptedTests;
          }

          const scalarCondition = !test.noScalar && (lhs === rhs || (rhs == 8 && lhs == 4) || (rhs == 4 && lhs == 8));

          if (SUPPORTED_UINT.includes(rhs) && (only8bits || (test.limit !== 'bits' && scalarCondition))) {
            if (test.limit !== 'bits') {
              rhsNumber = generateNumber(bitResults);
            }
            const encryptedTestName = [functionName, `euint${lhs}`, `uint${rhs}`].join('_');
            const encryptedTests: Test[] = [];
            if (!test.lhsHigher) {
              encryptedTests.push(safeEval(test.evalTest, lhsNumber, rhsNumber, lhs, rhs, test.safeMin));
              encryptedTests.push(safeEval(test.evalTest, smallest - 4, smallest, lhs, rhs, test.safeMin));
            }
            encryptedTests.push(safeEval(test.evalTest, smallest, smallest, lhs, rhs, test.safeMin));
            encryptedTests.push(safeEval(test.evalTest, smallest, smallest - 4, lhs, rhs, test.safeMin));
            tests[encryptedTestName] = encryptedTests;
          }
          if (SUPPORTED_UINT.includes(lhs) && test.limit !== 'bits' && scalarCondition && !test.scalarOnly) {
            lhsNumber = generateNumber(bitResults);
            const encryptedTestName = [functionName, `uint${lhs}`, `euint${rhs}`].join('_');
            const encryptedTests: Test[] = [];
            if (!test.lhsHigher) {
              encryptedTests.push(safeEval(test.evalTest, lhsNumber, rhsNumber, lhs, rhs, test.safeMin));
              encryptedTests.push(safeEval(test.evalTest, smallest - 4, smallest, lhs, rhs, test.safeMin));
            }
            encryptedTests.push(safeEval(test.evalTest, smallest, smallest, lhs, rhs, test.safeMin));
            encryptedTests.push(safeEval(test.evalTest, smallest, smallest - 4, lhs, rhs, test.safeMin));
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
