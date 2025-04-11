import { strict as assert } from 'node:assert';

import {
  ArgumentType,
  FheType,
  FunctionType,
  Operator,
  OperatorArguments,
  OverloadShard,
  OverloadSignature,
  ReturnType,
} from './common';
import { overloadTests } from './overloadTests';
import { getUint } from './utils';

export function generateSolidityOverloadTestFiles(operators: Operator[], fheTypes: FheType[]): OverloadSignature[] {
  const signatures: OverloadSignature[] = [];

  // Exclude types that do not support any operators.
  const adjustedFheTypes = fheTypes.filter((fheType: FheType) => fheType.supportedOperators.length > 0);

  // Generate overloads for encrypted operators with two encrypted types.
  adjustedFheTypes.forEach((lhsFheType: FheType) => {
    adjustedFheTypes.forEach((rhsFheType: FheType) => {
      operators.forEach((operator) => {
        generateOverloadsForTFHEEncryptedOperatorForTwoEncryptedTypes(lhsFheType, rhsFheType, operator, signatures);
      });
    });
  });

  // Generate overloads for scalar operators for all supported types.
  adjustedFheTypes.forEach((fheType: FheType) => {
    operators.forEach((operator) => {
      generateOverloadsForTFHEScalarOperator(fheType, operator, signatures);
    });
  });

  // Generate overloads for handle shift & rotate operators for all supported types
  adjustedFheTypes.forEach((fheType: FheType) => {
    operators.forEach((operator) => {
      generateOverloadsForTFHEShiftOperator(fheType, operator, signatures);
    });
  });

  // Generate overloads unary operators for all supported types.
  adjustedFheTypes.forEach((fheType: FheType) =>
    generateOverloadsForTFHEUnaryOperators(fheType, operators, signatures),
  );

  // TODO Add tests for conversion from plaintext and einput to all supported types (e.g., einput --> ebool, bytes memory --> ebytes64, uint32 --> euint32)
  return signatures;
}

function generateOverloadsForTFHEEncryptedOperatorForTwoEncryptedTypes(
  lhsFheType: FheType,
  rhsFheType: FheType,
  operator: Operator,
  signatures: OverloadSignature[],
) {
  if (operator.shiftOperator || operator.rotateOperator) {
    return;
  }

  if (!operator.hasEncrypted || operator.arguments != OperatorArguments.Binary) {
    return;
  }

  if (
    !lhsFheType.supportedOperators.includes(operator.name) ||
    !rhsFheType.supportedOperators.includes(operator.name)
  ) {
    return;
  }

  if (lhsFheType.type.startsWith('Uint') && rhsFheType.type.startsWith('Uint')) {
    // Determine the maximum number of bits between lhsBits and rhsBits
    const outputBits = Math.max(lhsFheType.bitLength, rhsFheType.bitLength);

    const returnTypeOverload: ArgumentType =
      operator.returnType == ReturnType.Euint ? ArgumentType.Euint : ArgumentType.Ebool;

    signatures.push({
      name: operator.name,
      arguments: [
        { type: ArgumentType.Euint, bits: lhsFheType.bitLength },
        { type: ArgumentType.Euint, bits: rhsFheType.bitLength },
      ],
      returnType: { type: returnTypeOverload, bits: outputBits },
    });
  } else if (lhsFheType.type == rhsFheType.type && rhsFheType.type.startsWith('Bytes')) {
    // TODO
  } else if (lhsFheType.type.startsWith('Int') && rhsFheType.type.startsWith('Int')) {
    throw new Error('Eint types are not supported!');
  }
}

function generateOverloadsForTFHEScalarOperator(fheType: FheType, operator: Operator, signatures: OverloadSignature[]) {
  if (operator.shiftOperator || operator.rotateOperator) {
    return;
  }

  if (operator.arguments != OperatorArguments.Binary) {
    return;
  }

  if (!operator.hasScalar) {
    return;
  }

  if (!fheType.supportedOperators.includes(operator.name)) {
    return;
  }

  const outputBits = fheType.bitLength;
  const returnTypeOverload = operator.returnType == ReturnType.Euint ? ArgumentType.Euint : ArgumentType.Ebool;

  if (fheType.type.startsWith('Uint')) {
    signatures.push({
      name: operator.name,
      arguments: [
        { type: ArgumentType.Euint, bits: outputBits },
        { type: ArgumentType.Uint, bits: outputBits },
      ],
      returnType: { type: returnTypeOverload, bits: outputBits },
    });
  }

  // lhs scalar
  if (!operator.leftScalarDisable) {
    if (fheType.type.startsWith('Uint')) {
      signatures.push({
        name: operator.name,
        arguments: [
          { type: ArgumentType.Uint, bits: outputBits },
          { type: ArgumentType.Euint, bits: outputBits },
        ],
        returnType: { type: returnTypeOverload, bits: outputBits },
      });
    }
  }
}

function generateOverloadsForTFHEShiftOperator(fheType: FheType, operator: Operator, signatures: OverloadSignature[]) {
  if (!operator.shiftOperator && !operator.rotateOperator) {
    return;
  }

  if (fheType.supportedOperators.includes(operator.name)) {
    const lhsBits = fheType.bitLength;
    const rhsBits = 8;

    const returnTypeOverload: ArgumentType = ArgumentType.Euint;

    if (fheType.type.startsWith('Uint')) {
      signatures.push({
        name: operator.name,
        arguments: [
          { type: ArgumentType.Euint, bits: lhsBits },
          { type: ArgumentType.Euint, bits: rhsBits },
        ],
        returnType: { type: returnTypeOverload, bits: fheType.bitLength },
      });
    }

    if (fheType.type.startsWith('Uint')) {
      signatures.push({
        name: operator.name,
        arguments: [
          { type: ArgumentType.Euint, bits: lhsBits },
          { type: ArgumentType.Uint, bits: rhsBits },
        ],
        returnType: { type: returnTypeOverload, bits: fheType.bitLength },
      });
    }
  }
}

function generateOverloadsForTFHEUnaryOperators(
  fheType: FheType,
  operators: Operator[],
  signatures: OverloadSignature[],
) {
  operators.forEach((op) => {
    if (op.arguments == OperatorArguments.Unary && fheType.supportedOperators.includes(op.name)) {
      if (fheType.type.startsWith('Uint')) {
        signatures.push({
          name: op.name,
          arguments: [{ type: ArgumentType.Euint, bits: fheType.bitLength }],
          returnType: { type: ArgumentType.Euint, bits: fheType.bitLength },
        });
      }
    }
  });
}

// TODO: generate automatically based on array of FheType
const stateVar = {
  ebool: 'resEbool',
  euint8: 'resEuint8',
  euint16: 'resEuint16',
  euint32: 'resEuint32',
  euint64: 'resEuint64',
  euint128: 'resEuint128',
  euint256: 'resEuint256',
  ebytes64: 'resEbytes64',
  ebytes128: 'resEbytes128',
  ebytes256: 'resEbytes256',
};

/**
 * Splits the provided overloads into multiple shards.
 *
 * @param overloads - The overloads to be split into shards.
 * @returns An array of shards containing the split overloads.
 * This is done because there's a limit on how big
 * of a smart contract you can deploy.
 */
export function splitOverloadsToShards(overloads: OverloadSignature[]): OverloadShard[] {
  const MAX_SHARD_SIZE = 90;
  const res: OverloadShard[] = [];

  let shardNo = 1;
  let accumulator: OverloadSignature[] = [];
  overloads.forEach((o) => {
    accumulator.push(o);
    if (accumulator.length >= MAX_SHARD_SIZE) {
      res.push({
        shardNumber: shardNo,
        overloads: [...accumulator],
      });
      shardNo++;
      accumulator = [];
    }
  });

  if (accumulator.length > 0) {
    res.push({
      shardNumber: shardNo,
      overloads: Object.assign([], accumulator),
    });
  }

  return res;
}

/**
 * Generates the first part of the test code for HTTPZ operations.
 *
 * This function dynamically creates TypeScript code for testing HTTPZ operations
 * based on the provided shards and index split. It imports necessary modules,
 * defines deployment functions for each shard, and sets up the test suite
 * with the appropriate contracts and instances.
 *
 * @param {OverloadShard[]} shards - An array of OverloadShard objects representing the shards to be included in the test.
 * @param {number} idxSplit - The index split value used to differentiate the test suite.
 * @returns {string} The generated introductory test code as a string.
 */
function generateIntroTestCode(shards: OverloadShard[], idxSplit: number): string {
  const intro: string[] = [];
  intro.push(`
    import { expect } from 'chai';
    import { ethers } from 'hardhat';
    import { createInstances, decrypt8, decrypt16, decrypt32, decrypt64, decrypt128, decrypt256, decryptBool } from '../instance';
    import { getSigners, initSigners } from '../signers';

  `);
  shards.forEach((os) => {
    intro.push(`
  import type { HTTPZTestSuite${os.shardNumber} } from '../../types/contracts/tests/HTTPZTestSuite${os.shardNumber}';
  `);
  });

  shards.forEach((os) => {
    intro.push(`
async function deployHTTPZTestFixture${os.shardNumber}(): Promise<HTTPZTestSuite${os.shardNumber}> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('HTTPZTestSuite${os.shardNumber}');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}
    `);
  });

  intro.push(`
    describe('HTTPZ operations ${idxSplit}', function () {
        before(async function () {
            await initSigners(1);
            this.signers = await getSigners();

  `);

  shards.forEach((os) => {
    intro.push(`
            const contract${os.shardNumber} = await deployHTTPZTestFixture${os.shardNumber}();
            this.contract${os.shardNumber}Address = await contract${os.shardNumber}.getAddress();
            this.contract${os.shardNumber} = contract${os.shardNumber};
    `);
  });

  intro.push(`
  const instances = await createInstances(this.signers);
  this.instances = instances;
        });
  `);
  return intro.join('');
}

/**
 * Generates TypeScript test code for the given overload shards.
 *
 * @param {OverloadShard[]} shards - An array of overload shards, each containing multiple overloads.
 * @param {number} numTsSplits - The number of TypeScript test files to split the generated tests into.
 * @returns {string[]} An array of strings, each representing the content of a TypeScript test file.
 *
 * The function calculates the total number of Solidity tests and splits them into the specified number of TypeScript test files.
 * It iterates over each overload shard and generates test code for each overload, ensuring that tests are defined for each overload method.
 * The generated test code includes assertions to verify the correctness of the inputs and outputs.
 * The function also ensures that there are no unused overload tests defined.
 */
export function generateTypeScriptTestCode(shards: OverloadShard[], numTsSplits: number): string[] {
  const numSolTest = shards.reduce((sum, os) => sum + os.overloads.length, 0);
  let idxTsTest = 0;

  let listRes: string[] = [];

  const sizeTsShard = Math.floor(numSolTest / numTsSplits);

  let res: string[] = [];

  const overloadUsages: { [methodName: string]: boolean } = {};
  shards.forEach((os) => {
    os.overloads.forEach((o) => {
      if (idxTsTest % sizeTsShard === 0) res.push(generateIntroTestCode(shards, idxTsTest / sizeTsShard + 1));
      const testName = `test operator "${o.name}" overload ${signatureContractEncryptedSignature(o)}`;
      const methodName = signatureContractMethodName(o);
      overloadUsages[methodName] = true;
      const tests = overloadTests[methodName] || [];

      // Ensure that there are tests defined for each overload method.
      assert(tests.length > 0, `Overload ${methodName} has no test, please add them.`);

      let testIndex = 1;
      tests.forEach((t) => {
        assert(
          t.inputs.length == o.arguments.length,
          `Test argument count is different to operator arguments, arguments: ${t.inputs}, expected count: ${o.arguments.length}`,
        );
        t.inputs.forEach((input, inputIndex) => ensureNumberAcceptableInBitRange(o.arguments[inputIndex].bits, input));
        t.inputs.forEach((input, index) => ensureNumberAcceptableInBitRange(o.arguments[index].bits, input));
        if (typeof t.output === 'number') {
          ensureNumberAcceptableInBitRange(o.returnType.bits, t.output);
        }
        const testArgs = t.inputs.join(', ');
        let numEncryptedArgs = 0;
        const testArgsEncrypted = t.inputs
          .map((v, index) => {
            if (o.arguments[index].type == ArgumentType.Euint) {
              numEncryptedArgs++;
              return `encryptedAmount.handles[${numEncryptedArgs - 1}]`;
            } else {
              return `${v}n`;
            }
          })
          .join(', ');
        const inputsAdd = t.inputs
          .map((v, index) => {
            if (o.arguments[index].type == ArgumentType.Euint) {
              return `input.add${o.arguments[index].bits}(${v}n);`;
            }
          })
          .join('\n');
        let expectedOutput = t.output.toString();
        if (typeof t.output === 'bigint') expectedOutput += 'n';

        res.push(`
                it('${testName} test ${testIndex} (${testArgs})', async function () {
                    const input = this.instances.alice.createEncryptedInput(this.contract${os.shardNumber}Address, this.signers.alice.address);
                    ${inputsAdd}
                    const encryptedAmount = await input.encrypt();
                    const tx = await this.contract${os.shardNumber}.${methodName}(${testArgsEncrypted}, encryptedAmount.inputProof);
                    await tx.wait();
                    const res = await decrypt${o.returnType.type === 1 ? o.returnType.bits : 'Bool'}(await this.contract${os.shardNumber}.res${o.returnType.type === 1 ? `Euint${o.returnType.bits}` : 'Ebool'}());
                    expect(res).to.equal(${expectedOutput});
                });
            `);
        testIndex++;
      });
      idxTsTest++;
      if (idxTsTest % sizeTsShard === 0 || idxTsTest === numSolTest) {
        res.push(`
      });
    `);
        listRes.push(res.join(''));
        res = [];
      }
    });
  });

  return listRes;
}

/**
 * Ensures that a given number or bigint falls within the acceptable range for a specified number of bits.
 *
 * @param bits - The number of bits that define the acceptable range.
 * @param input - The number or bigint to be checked.
 * @throws Will throw an error if the input is not within the range [0, 2^bits].
 */
function ensureNumberAcceptableInBitRange(bits: number, input: number | bigint) {
  assert(
    input >= 0 && input <= 2 ** bits,
    `${bits} bit number ${input} doesn't fall into expected [${0}; ${2 ** bits}] range`,
  );
}

/**
 * Generates Solidity unit test contracts for a given OverloadShard.
 *
 * This function creates a Solidity contract named `HTTPZTestSuite` followed by the shard number.
 * The contract includes several public variables of different encrypted types (ebool, euint8, euint16, euint32, euint64, euint128, euint256)
 * and a constructor that sets the FHEVM configuration using the default configuration from `HTTPZConfig`.
 * It also calls the `generateLibCallTest` function to add additional test logic to the contract.
 *
 * @param {OverloadShard} os - The overload shard for which the test contract is generated.
 * @returns {string} The generated Solidity unit test contract as a string.
 */
export function generateSolidityUnitTestContracts(os: OverloadShard): string {
  const res: string[] = [];

  res.push(`
        // SPDX-License-Identifier: BSD-3-Clause-Clear
        pragma solidity ^0.8.24;

        import "../../lib/HTTPZ.sol";
        import "../../lib/HTTPZConfig.sol";

        contract HTTPZTestSuite${os.shardNumber} {
          ebool public resEbool;
          euint8 public resEuint8;
          euint16 public resEuint16;
          euint32 public resEuint32;
          euint64 public resEuint64;
          euint128 public resEuint128;
          euint256 public resEuint256;
          ebytes64 public resEbytes64;
          ebytes128 public resEbytes128;
          ebytes256 public resEbytes256;

          constructor() {
            HTTPZ.setCoprocessor(HTTPZConfig.defaultConfig());
          }

    `);

  generateLibCallTest(os, res);

  res.push(`
        }
    `);

  return res.join('');
}

/**
 * Generates a library call test function for the given overload shard.
 *
 * @param os - The overload shard containing the overloads to generate the test for.
 * @param res - The array to which the generated test function code will be appended.
 *
 * This function iterates over the overloads in the provided overload shard and generates
 * a Solidity function for each overload. The generated function includes the necessary
 * argument processing, type casting, and the appropriate HTTPZ library call. The result
 * of the library call is then allowed and assigned to the corresponding state variable.
 */
function generateLibCallTest(os: OverloadShard, res: string[]) {
  os.overloads.forEach((o) => {
    const methodName = signatureContractMethodName(o);
    const args = signatureContractArguments(o);
    res.push(`function ${methodName}(${args}) public {`);

    const procArgs: string[] = [];
    let argumentCharCode = 97; // letter 'a' in ascii
    o.arguments.forEach((a) => {
      const arg = String.fromCharCode(argumentCharCode);
      const argProc = `${arg}Proc`;
      procArgs.push(argProc);
      res.push(`${functionTypeToString(a)} ${argProc} = ${castExpressionToType(arg, a)};`);
      res.push('\n');
      argumentCharCode++;
    });

    const tfheArgs = procArgs.join(', ');

    if (o.binaryOperator) {
      assert(o.arguments.length == 2, 'We assume two arguments for binary operator');
      res.push(`${functionTypeToEncryptedType(o.returnType)} result = aProc ${o.binaryOperator} bProc;`);
      res.push('\n');
    } else if (o.unaryOperator) {
      assert(o.arguments.length == 1, 'We assume one argument for unary operator');
      res.push(`${functionTypeToEncryptedType(o.returnType)} result = ${o.unaryOperator}aProc;`);
      res.push('\n');
    } else {
      res.push(`${functionTypeToEncryptedType(o.returnType)} result = HTTPZ.${o.name}(${tfheArgs});`);
      res.push('\n');
    }
    res.push('HTTPZ.allowThis(result);');
    res.push(`${stateVar[functionTypeToEncryptedType(o.returnType) as keyof typeof stateVar]} = result;
        }
    `);
  });
}

/**
 * Generates a unique method name for a contract method based on its signature.
 *
 * @param s - The overload signature of the contract method.
 * @returns A string representing the unique method name, composed of the method name and its argument types joined by underscores.
 */
export function signatureContractMethodName(s: OverloadSignature): string {
  const res: string[] = [];

  res.push(s.name);
  s.arguments.forEach((a) => res.push(functionTypeToString(a)));

  return res.join('_');
}

/**
 * Generates a string representation of the contract arguments for a given overload signature.
 *
 * @param s - The overload signature containing the arguments to be converted.
 * @returns A string representing the contract arguments, formatted as `type name`.
 *
 * The function iterates over the arguments of the provided overload signature,
 * converts each argument type to a calldata type, and assigns a name starting
 * from 'a' and incrementing for each subsequent argument. Additionally, it appends
 * 'bytes calldata inputProof' to the end of the arguments list.
 */
function signatureContractArguments(s: OverloadSignature): string {
  const res: string[] = [];
  let argumentCharCode = 97; // letter 'a' in ascii
  s.arguments.forEach((a) => {
    res.push(`${functionTypeToCalldataType(a)} ${String.fromCharCode(argumentCharCode)}`);
    argumentCharCode++;
  });
  res.push('bytes calldata inputProof');

  return res.join(', ');
}

/**
 * Generates a string representation of a contract function signature with encrypted return type.
 *
 * @param s - The overload signature containing the function arguments and return type.
 * @returns A string representing the function signature with encrypted return type.
 */
function signatureContractEncryptedSignature(s: OverloadSignature): string {
  const res: string[] = [];
  let argumentCharCode = 97; // letter 'a' in ascii
  s.arguments.forEach((a) => {
    res.push(`${functionTypeToString(a)}`);
    argumentCharCode++;
  });

  const joined = res.join(', ');
  return `(${joined}) => ${functionTypeToEncryptedType(s.returnType)}`;
}

/**
 * Casts an expression to a specified type.
 *
 * @param argExpr - The expression to be casted as a string.
 * @param outputType - The type to cast the expression to, represented as a FunctionType object.
 * @returns The casted expression as a string.
 *
 * The function handles the following types:
 * - `Euint`: Casts to an encrypted unsigned integer with a specified bit length.
 * - `Uint`: Returns the expression as is.
 * - `Ebool`: Casts to an encrypted boolean.
 * - `Ebytes`: Casts to encrypted bytes with a specified byte length.
 */
function castExpressionToType(argExpr: string, outputType: FunctionType): string {
  switch (outputType.type) {
    case ArgumentType.Euint:
      return `HTTPZ.asEuint${outputType.bits}(${argExpr}, inputProof)`;
    case ArgumentType.Uint:
      return argExpr;
    case ArgumentType.Ebool:
      return `HTTPZ.asEbool(${argExpr})`;
    // case ArgumentType.Ebytes:
    //  return `HTTPZ.asEbytes${outputType.bits / 8}(${argExpr})`;
  }
}

/**
 * Converts a `FunctionType` to its corresponding calldata type string.
 *
 * @param t - The `FunctionType` object to convert.
 * @returns The calldata type string corresponding to the given `FunctionType`.
 *
 * The conversion is based on the `type` property of the `FunctionType` object:
 * - `ArgumentType.Euint`: Returns "einput".
 * - `ArgumentType.Uint`: Returns the result of `getUint(t.bits)`.
 * - `ArgumentType.Ebool`: Returns "einput".
 * - `ArgumentType.Ebytes`: Returns "einput".
 */
function functionTypeToCalldataType(t: FunctionType): string {
  switch (t.type) {
    case ArgumentType.Euint:
      return `einput`;
    case ArgumentType.Uint:
      return getUint(t.bits);
    case ArgumentType.Ebool:
      return `einput`;
    // case ArgumentType.Ebytes:
    //  return `einput`;
  }
}

/**
 * Converts a given `FunctionType` to its corresponding encrypted type string.
 *
 * @param t - The `FunctionType` object to be converted.
 * @returns The encrypted type string based on the `FunctionType`.
 *
 * The conversion rules are as follows:
 * - If the type is `Euint` or `Ebytes`, it returns `ebytes` followed by the number of bytes (calculated as `t.bits / 8`).
 * - If the type is `Uint`, it returns `euint` followed by the number of bits.
 * - If the type is `Ebool`, it returns `ebool`.
 */
function functionTypeToEncryptedType(t: FunctionType): string {
  switch (t.type) {
    case ArgumentType.Euint:
    // case ArgumentType.Ebytes:
    //  return `ebytes${t.bits / 8}`;
    case ArgumentType.Uint:
      return `euint${t.bits}`;
    case ArgumentType.Ebool:
      return `ebool`;
  }
}

/**
 * Converts a `FunctionType` object to its corresponding string representation.
 *
 * @param t - The `FunctionType` object to convert.
 * @returns The string representation of the `FunctionType`.
 *
 * The conversion is based on the `type` property of the `FunctionType` object:
 * - If `t.type` is `ArgumentType.Euint`, the result is `euint` followed by the number of bits.
 * - If `t.type` is `ArgumentType.Uint`, the result is obtained from the `getUint` function with the number of bits.
 * - If `t.type` is `ArgumentType.Ebool`, the result is `ebool`.
 * - If `t.type` is `ArgumentType.Ebytes`, the result is `ebytes` followed by the number of bits.
 */
function functionTypeToString(t: FunctionType): string {
  switch (t.type) {
    case ArgumentType.Euint:
      return `euint${t.bits}`;
    case ArgumentType.Uint:
      return getUint(t.bits);
    case ArgumentType.Ebool:
      return `ebool`;
    // case ArgumentType.Ebytes:
    //  return `ebytes${t.bits}`;
  }
}
