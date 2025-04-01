import { FheType } from './common';
import { findMinimumValueInBigIntArray, generateRandomNumber } from './utils';

/**
 * Represents a test structure with input and output types.
 *
 * @property inputs - An array of `bigint` values representing the inputs.
 * @property output - The output value, which can be a `number`, `boolean`, or `bigint`.
 */
type Test = {
  inputs: bigint[];
  output: number | boolean | bigint;
};

/**
 * Represents a collection of supported functions, where each function is identified
 * by a unique key and associated with its corresponding `SupportedFunction` definition.
 */
type SupportedFunctions = {
  [key: string]: SupportedFunction;
};

/**
 * Represents the parameters that can be used to configure supported functions.
 *
 * @property safeMin - Optional. If `true`, ensures that the minimum value is safely handled.
 * @property noScalar - Optional. If `true`, disables scalar operations.
 * @property lhsHigher - Optional. If `true`, enforces that the left-hand side value is higher.
 * @property scalarOnly - Optional. If `true`, restricts operations to scalar values only.
 * @property limit - Optional. Specifies a limit type, such as `'bits'`, to constrain operations.
 */
type SupportedFunctionParams = {
  safeMin?: boolean;
  noScalar?: boolean;
  lhsHigher?: boolean;
  scalarOnly?: boolean;
  limit?: 'bits';
};

type SupportedFunction = SupportedFunctionParams &
  (
    | {
        // Represents a binary function (e.g., addition, subtraction) that operates on two inputs.
        unary?: false;
        evalTest: (lhsNumber: bigint, rhsNumber: bigint, lhs: number, rhs: number) => number | boolean | bigint;
      }
    | {
        // Represents a unary function (e.g., negation, bitwise NOT) that operates on a single input.
        unary: true;
        evalTest: (lhs: bigint, bits: number) => number | boolean | bigint;
      }
  );

/**
 * Safely evaluates a function with given inputs, ensuring the result does not exceed a specified bit limit.
 *
 * @param fn - A function that takes four arguments: `lhsNumber`, `rhsNumber`, `lhs`, and `rhs`, and returns a `number`, `boolean`, or `bigint`.
 * @param lhsNumber - The left-hand side number as a `bigint`.
 * @param rhsNumber - The right-hand side number as a `bigint`.
 * @param lhs - The left-hand side number as a `number`.
 * @param rhs - The right-hand side number as a `number`.
 * @param safeMin - A boolean flag indicating whether to use the minimum (`Math.min`) or maximum (`Math.max`) of `lhs` and `rhs` to determine the bit limit. Defaults to `false`.
 * @returns An object containing:
 *   - `inputs`: An array with the adjusted `lhsNumber` and `rhsNumber`.
 *   - `output`: The result of the function `fn` after adjustments.
 */
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

  if (typeof result === 'number' || typeof result === 'bigint') {
    while ((result as number | bigint) > Math.pow(2, bitResults) - 1) {
      lhsNumber = lhsNumber / 2n + 1n;
      rhsNumber = rhsNumber / 2n + 1n;
      result = fn(lhsNumber, rhsNumber, lhs, rhs);
    }
  }
  return { inputs: [lhsNumber, rhsNumber], output: result };
};

export const SUPPORTED_FUNCTIONS: SupportedFunctions = {
  add: {
    safeMin: true,
    evalTest: (lhsNumber, rhsNumber) => BigInt(lhsNumber) + BigInt(rhsNumber),
  },
  sub: {
    lhsHigher: true,
    evalTest: (lhsNumber, rhsNumber) => lhsNumber - rhsNumber,
  },
  mul: {
    safeMin: true,
    evalTest: (lhsNumber, rhsNumber) => BigInt(lhsNumber) * BigInt(rhsNumber),
  },
  div: {
    evalTest: (lhsNumber, rhsNumber) => lhsNumber / rhsNumber,
    scalarOnly: true,
  },
  rem: {
    evalTest: (lhsNumber, rhsNumber) => lhsNumber % rhsNumber,
    scalarOnly: true,
  },
  le: {
    evalTest: (lhsNumber, rhsNumber) => lhsNumber <= rhsNumber,
  },
  lt: {
    evalTest: (lhsNumber, rhsNumber) => lhsNumber < rhsNumber,
  },
  ge: {
    evalTest: (lhsNumber, rhsNumber) => lhsNumber >= rhsNumber,
  },
  gt: {
    evalTest: (lhsNumber, rhsNumber) => lhsNumber > rhsNumber,
  },
  eq: {
    evalTest: (lhsNumber, rhsNumber) => lhsNumber === rhsNumber,
  },
  ne: {
    evalTest: (lhsNumber, rhsNumber) => lhsNumber !== rhsNumber,
  },
  shl: {
    limit: 'bits',
    evalTest: (lhsNumber, rhsNumber, lhs, _rhs) => {
      // Perform a left shift operation by manipulating the bit positions of the binary representation.
      const bits = `${new Array(256).fill('0').join('')}${lhsNumber.toString(2)}`.slice(-lhs).split('');
      const r = bits.map((_, index) => {
        const newIndex = Number(BigInt(index) + (rhsNumber % BigInt(lhs)));
        return newIndex >= bits.length ? '0' : bits[newIndex];
      });
      return BigInt(`0b${r.join('')}`);
    },
  },
  shr: {
    limit: 'bits',
    evalTest: (lhsNumber, rhsNumber, lhs, _rhs) => {
      const bits = `${new Array(256).fill('0').join('')}${lhsNumber.toString(2)}`.slice(-lhs).split('');
      const r = bits.map((_, index) => {
        const newIndex = Number(BigInt(index) - (rhsNumber % BigInt(lhs)));
        return newIndex < 0 ? '0' : bits[newIndex];
      });
      return BigInt(`0b${r.join('')}`);
    },
  },
  rotl: {
    limit: 'bits',
    evalTest: (lhsNumber, rhsNumber, lhs, _rhs) => {
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
    limit: 'bits',
    evalTest: (lhsNumber, rhsNumber, lhs, _rhs) => {
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
    unary: false,
    evalTest: (lhsNumber, rhsNumber) => (lhsNumber > rhsNumber ? lhsNumber : rhsNumber),
  },
  min: {
    evalTest: (lhsNumber, rhsNumber) => (lhsNumber < rhsNumber ? lhsNumber : rhsNumber),
  },
  or: {
    evalTest: (lhsNumber, rhsNumber) => lhsNumber | rhsNumber,
  },
  and: {
    evalTest: (lhsNumber, rhsNumber) => lhsNumber & rhsNumber,
  },
  xor: {
    evalTest: (lhsNumber, rhsNumber) => lhsNumber ^ rhsNumber,
  },
  not: {
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

/**
 * Generates test cases for supported functions based on the provided FHE types.
 *
 * @param fheTypes - An array of FHE types, each containing information about type, bit length, and supported operators.
 * @returns An object containing generated test cases for each supported function and FHE type combination.
 */
export const generateOverloads = (fheTypes: FheType[]) => {
  const generatedTests: any = {};
  Object.keys(SUPPORTED_FUNCTIONS).forEach((functionName: string) => {
    const test = SUPPORTED_FUNCTIONS[functionName];

    fheTypes.forEach((lhsFheType: FheType) => {
      if (lhsFheType.type.startsWith('Uint') && lhsFheType.supportedOperators.includes(functionName)) {
        if (test.unary) {
          let lhsNumber = generateRandomNumber(lhsFheType.bitLength);
          const encryptedTestName = [functionName, `e${lhsFheType.type.toLowerCase()}`].join('_');
          const encryptedTests: Test[] = [];
          encryptedTests.push({
            inputs: [lhsNumber],
            output: test.evalTest(lhsNumber, lhsFheType.bitLength),
          });
          generatedTests[encryptedTestName] = encryptedTests;
        } else {
          fheTypes.forEach((rhsFheType: FheType) => {
            if (rhsFheType.type.startsWith('Uint') && rhsFheType.supportedOperators.includes(functionName)) {
              const bitResults = Math.min(lhsFheType.bitLength, rhsFheType.bitLength);
              let lhsNumber = generateRandomNumber(lhsFheType.bitLength);
              let rhsNumber = generateRandomNumber(rhsFheType.bitLength);

              if (test.limit === 'bits') {
                // @dev We set the floor as 5 to prevent underflows since tests would use smallest - 4n.
                rhsNumber = BigInt(5 + Math.floor(Math.random() * (rhsFheType.bitLength - 1)));
              }

              const smallest = findMinimumValueInBigIntArray(lhsNumber, rhsNumber);
              const only8bits = test.limit === 'bits' && rhsFheType.bitLength === 8;

              if ((test.limit !== 'bits' || only8bits) && !test.scalarOnly) {
                const encryptedTestName = [
                  functionName,
                  `e${lhsFheType.type.toLowerCase()}`,
                  `e${rhsFheType.type.toLowerCase()}`,
                ].join('_');
                const encryptedTests: Test[] = [];
                if (!test.lhsHigher) {
                  encryptedTests.push(
                    safeEval(
                      test.evalTest,
                      lhsNumber,
                      rhsNumber,
                      lhsFheType.bitLength,
                      rhsFheType.bitLength,
                      test.safeMin,
                    ),
                  );
                  encryptedTests.push(
                    safeEval(
                      test.evalTest,
                      smallest - 4n,
                      smallest,
                      lhsFheType.bitLength,
                      rhsFheType.bitLength,
                      test.safeMin,
                    ),
                  );
                }
                encryptedTests.push(
                  safeEval(test.evalTest, smallest, smallest, lhsFheType.bitLength, rhsFheType.bitLength, test.safeMin),
                );
                encryptedTests.push(
                  safeEval(
                    test.evalTest,
                    smallest,
                    smallest - 4n,
                    lhsFheType.bitLength,
                    rhsFheType.bitLength,
                    test.safeMin,
                  ),
                );
                generatedTests[encryptedTestName] = encryptedTests;
              }

              const scalarCondition = !test.noScalar && lhsFheType.bitLength === rhsFheType.bitLength;

              if (only8bits || (test.limit !== 'bits' && scalarCondition)) {
                if (test.limit !== 'bits') {
                  rhsNumber = generateRandomNumber(bitResults);
                }
                const encryptedTestName = [
                  functionName,
                  `e${lhsFheType.type.toLowerCase()}`,
                  `uint${rhsFheType.bitLength}`,
                ].join('_');
                const encryptedTests: Test[] = [];
                if (!test.lhsHigher) {
                  encryptedTests.push(
                    safeEval(
                      test.evalTest,
                      lhsNumber,
                      rhsNumber,
                      lhsFheType.bitLength,
                      rhsFheType.bitLength,
                      test.safeMin,
                    ),
                  );
                  encryptedTests.push(
                    safeEval(
                      test.evalTest,
                      smallest - 4n,
                      smallest,
                      lhsFheType.bitLength,
                      rhsFheType.bitLength,
                      test.safeMin,
                    ),
                  );
                }
                encryptedTests.push(
                  safeEval(test.evalTest, smallest, smallest, lhsFheType.bitLength, rhsFheType.bitLength, test.safeMin),
                );
                encryptedTests.push(
                  safeEval(
                    test.evalTest,
                    smallest,
                    smallest - 4n,
                    lhsFheType.bitLength,
                    rhsFheType.bitLength,
                    test.safeMin,
                  ),
                );
                generatedTests[encryptedTestName] = encryptedTests;
              }
              if (test.limit !== 'bits' && scalarCondition && !test.scalarOnly) {
                lhsNumber = generateRandomNumber(bitResults);
                const encryptedTestName = [
                  functionName,
                  `uint${lhsFheType.bitLength}`,
                  `e${rhsFheType.type.toLowerCase()}`,
                ].join('_');
                const encryptedTests: Test[] = [];
                if (!test.lhsHigher) {
                  encryptedTests.push(
                    safeEval(
                      test.evalTest,
                      lhsNumber,
                      rhsNumber,
                      lhsFheType.bitLength,
                      rhsFheType.bitLength,
                      test.safeMin,
                    ),
                  );
                  encryptedTests.push(
                    safeEval(
                      test.evalTest,
                      smallest - 4n,
                      smallest,
                      lhsFheType.bitLength,
                      rhsFheType.bitLength,
                      test.safeMin,
                    ),
                  );
                }
                encryptedTests.push(
                  safeEval(test.evalTest, smallest, smallest, lhsFheType.bitLength, rhsFheType.bitLength, test.safeMin),
                );
                encryptedTests.push(
                  safeEval(
                    test.evalTest,
                    smallest,
                    smallest - 4n,
                    lhsFheType.bitLength,
                    rhsFheType.bitLength,
                    test.safeMin,
                  ),
                );
                generatedTests[encryptedTestName] = encryptedTests;
              }
            }
          });
        }
      }
    });
  });

  return generatedTests;
};
