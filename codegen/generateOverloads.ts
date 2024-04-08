type Test = {
  inputs: bigint[];
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
        evalTest: (lhsNumber: bigint, rhsNumber: bigint, lhs: number, rhs: number) => number | boolean | bigint;
      }
    | {
        unary: true;
        evalTest: (lhs: bigint, bits: number) => number | boolean | bigint;
      }
  );

(BigInt as any).prototype['toJSON'] = function () {
  return this.toString();
};

const SUPPORTED_UINT = [8, 16, 32, 64, 128, 256];
const SUPPORTED_BITS = [4, 8, 16, 32, 64];

const bigIntMin = (...args: bigint[]) => {
  return args.reduce((min, e) => (e < min ? e : min), args[0]);
};

const bigIntMax = (...args: bigint[]) => {
  return args.reduce((max, e) => (e > max ? e : max), args[0]);
};

const generateNumber = (bits: number) => {
  const power = BigInt(Math.pow(2, bits) - 1);
  const maxRange = bigIntMin(power, BigInt(Number.MAX_SAFE_INTEGER));
  const substract = bigIntMax(BigInt(Math.floor(Math.random() * Number(maxRange))), 1n);
  return bigIntMax(power - substract, 1n);
};

const safeEval = (
  fn: (lhsNumber: bigint, rhsNumber: bigint, lhs: number, rhs: number) => number | boolean | bigint,
  lhsNumber: bigint,
  rhsNumber: bigint,
  lhs: number,
  rhs: number,
  safeMin: boolean = false,
) => {
  const bitResults = safeMin ? Math.min(lhs, rhs) : Math.max(lhs, rhs);
  let result = fn(lhsNumber, rhsNumber, lhs, rhs);
  const logs: any[] = [];
  if (typeof result === 'number' || typeof result === 'bigint') {
    while ((result as number | bigint) > Math.pow(2, bitResults) - 1) {
      lhsNumber = lhsNumber / 2n + 1n;
      rhsNumber = rhsNumber / 2n + 1n;
      result = fn(lhsNumber, rhsNumber, lhs, rhs);
      logs.push([lhs, rhs, lhsNumber, rhsNumber, result]);
    }
  }
  return { inputs: [lhsNumber, rhsNumber], output: result };
};

export const SUPPORTED_FUNCTIONS: SupportedFunctions = {
  add: {
    supportedBits: SUPPORTED_BITS,
    safeMin: true,
    evalTest: (lhsNumber, rhsNumber) => BigInt(lhsNumber) + BigInt(rhsNumber),
  },
  sub: {
    supportedBits: SUPPORTED_BITS,
    lhsHigher: true,
    evalTest: (lhsNumber, rhsNumber) => lhsNumber - rhsNumber,
  },
  mul: {
    supportedBits: SUPPORTED_BITS,
    safeMin: true,
    evalTest: (lhsNumber, rhsNumber) => BigInt(lhsNumber) * BigInt(rhsNumber),
  },
  div: {
    supportedBits: SUPPORTED_BITS,
    evalTest: (lhsNumber, rhsNumber) => lhsNumber / rhsNumber,
    scalarOnly: true,
  },
  rem: {
    supportedBits: SUPPORTED_BITS,
    evalTest: (lhsNumber, rhsNumber) => lhsNumber % rhsNumber,
    scalarOnly: true,
  },
  le: {
    supportedBits: SUPPORTED_BITS,
    evalTest: (lhsNumber, rhsNumber) => lhsNumber <= rhsNumber,
  },
  lt: {
    supportedBits: SUPPORTED_BITS,
    evalTest: (lhsNumber, rhsNumber) => lhsNumber < rhsNumber,
  },
  ge: {
    supportedBits: SUPPORTED_BITS,
    evalTest: (lhsNumber, rhsNumber) => lhsNumber >= rhsNumber,
  },
  gt: {
    supportedBits: SUPPORTED_BITS,
    evalTest: (lhsNumber, rhsNumber) => lhsNumber > rhsNumber,
  },
  eq: {
    supportedBits: SUPPORTED_BITS,
    evalTest: (lhsNumber, rhsNumber) => lhsNumber === rhsNumber,
  },
  ne: {
    supportedBits: SUPPORTED_BITS,
    evalTest: (lhsNumber, rhsNumber) => lhsNumber !== rhsNumber,
  },
  shl: {
    supportedBits: SUPPORTED_BITS,
    limit: 'bits',
    evalTest: (lhsNumber, rhsNumber, lhs, rhs) => {
      const bits = `${new Array(256).fill('0').join('')}${lhsNumber.toString(2)}`.slice(-lhs).split('');
      const r = bits.map((_, index) => {
        const newIndex = Number(BigInt(index) + (rhsNumber % BigInt(lhs)));
        return newIndex >= bits.length ? '0' : bits[newIndex];
      });
      return BigInt(`0b${r.join('')}`);
    },
  },
  shr: {
    supportedBits: SUPPORTED_BITS,
    limit: 'bits',
    evalTest: (lhsNumber, rhsNumber, lhs, rhs) => {
      const bits = `${new Array(256).fill('0').join('')}${lhsNumber.toString(2)}`.slice(-lhs).split('');
      const r = bits.map((_, index) => {
        const newIndex = Number(BigInt(index) - (rhsNumber % BigInt(lhs)));
        return newIndex < 0 ? '0' : bits[newIndex];
      });
      return BigInt(`0b${r.join('')}`);
    },
  },
  rotl: {
    supportedBits: SUPPORTED_BITS,
    limit: 'bits',
    evalTest: (lhsNumber, rhsNumber, lhs, rhs) => {
      const bits = `${new Array(256).fill('0').join('')}${lhsNumber.toString(2)}`.slice(-lhs).split('');
      const r = bits.map((_, index) => {
        let newIndex = Number(BigInt(index) + (rhsNumber % BigInt(lhs)));
        if (newIndex >= lhs) newIndex = newIndex % lhs;
        return bits[newIndex];
      });
      return BigInt(`0b${r.join('')}`);
    },
  },
  rotr: {
    supportedBits: SUPPORTED_BITS,
    limit: 'bits',
    evalTest: (lhsNumber, rhsNumber, lhs, rhs) => {
      const bits = `${new Array(256).fill('0').join('')}${lhsNumber.toString(2)}`.slice(-lhs).split('');
      const r = bits.map((_, index) => {
        let newIndex = Number(BigInt(index) - (rhsNumber % BigInt(lhs)));
        if (newIndex < 0) newIndex = lhs + newIndex;
        return bits[newIndex];
      });
      return BigInt(`0b${r.join('')}`);
    },
  },
  max: {
    supportedBits: SUPPORTED_BITS,
    unary: false,
    evalTest: (lhsNumber, rhsNumber) => (lhsNumber > rhsNumber ? lhsNumber : rhsNumber),
  },
  min: {
    supportedBits: SUPPORTED_BITS,
    evalTest: (lhsNumber, rhsNumber) => (lhsNumber < rhsNumber ? lhsNumber : rhsNumber),
  },
  or: {
    supportedBits: SUPPORTED_BITS,
    noScalar: true,
    evalTest: (lhsNumber, rhsNumber) => lhsNumber | rhsNumber,
  },
  and: {
    supportedBits: SUPPORTED_BITS,
    noScalar: true,
    evalTest: (lhsNumber, rhsNumber) => lhsNumber & rhsNumber,
  },
  xor: {
    supportedBits: SUPPORTED_BITS,
    noScalar: true,
    evalTest: (lhsNumber, rhsNumber) => lhsNumber ^ rhsNumber,
  },
  not: {
    supportedBits: SUPPORTED_BITS,
    unary: true,
    evalTest: (lhsNumber, bits) => {
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
    evalTest: (lhsNumber, bits) => {
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
      if (test.unary) {
        let lhsNumber = generateNumber(lhs);
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
          let lhsNumber = generateNumber(lhs);
          let rhsNumber = generateNumber(rhs);
          if (test.limit === 'bits') {
            rhsNumber = BigInt(1 + Math.floor(Math.random() * (rhs - 1)));
          }
          const smallest = bigIntMax(bigIntMin(lhsNumber, rhsNumber), 8n);
          const only8bits = test.limit === 'bits' && rhs === 8;
          const onlyEncrypted8bits = only8bits && lhs > 4;

          if ((test.limit !== 'bits' || onlyEncrypted8bits) && !test.scalarOnly) {
            const encryptedTestName = [functionName, `euint${lhs}`, `euint${rhs}`].join('_');
            const encryptedTests: Test[] = [];
            if (!test.lhsHigher) {
              encryptedTests.push(safeEval(test.evalTest, lhsNumber, rhsNumber, lhs, rhs, test.safeMin));
              encryptedTests.push(safeEval(test.evalTest, smallest - 4n, smallest, lhs, rhs, test.safeMin));
            }
            encryptedTests.push(safeEval(test.evalTest, smallest, smallest, lhs, rhs, test.safeMin));
            encryptedTests.push(safeEval(test.evalTest, smallest, smallest - 4n, lhs, rhs, test.safeMin));
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
              encryptedTests.push(safeEval(test.evalTest, smallest - 4n, smallest, lhs, rhs, test.safeMin));
            }
            encryptedTests.push(safeEval(test.evalTest, smallest, smallest, lhs, rhs, test.safeMin));
            encryptedTests.push(safeEval(test.evalTest, smallest, smallest - 4n, lhs, rhs, test.safeMin));
            tests[encryptedTestName] = encryptedTests;
          }
          if (SUPPORTED_UINT.includes(lhs) && test.limit !== 'bits' && scalarCondition && !test.scalarOnly) {
            lhsNumber = generateNumber(bitResults);
            const encryptedTestName = [functionName, `uint${lhs}`, `euint${rhs}`].join('_');
            const encryptedTests: Test[] = [];
            if (!test.lhsHigher) {
              encryptedTests.push(safeEval(test.evalTest, lhsNumber, rhsNumber, lhs, rhs, test.safeMin));
              encryptedTests.push(safeEval(test.evalTest, smallest - 4n, smallest, lhs, rhs, test.safeMin));
            }
            encryptedTests.push(safeEval(test.evalTest, smallest, smallest, lhs, rhs, test.safeMin));
            encryptedTests.push(safeEval(test.evalTest, smallest, smallest - 4n, lhs, rhs, test.safeMin));
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
