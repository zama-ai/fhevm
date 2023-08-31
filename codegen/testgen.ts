import { strict as assert } from 'node:assert';

import { Operator } from './common';
import { overloadTests } from './overloadTests';

export enum ArgumentType {
  Ebool,
  EUint,
  Uint,
}

export type FunctionType = {
  type: ArgumentType;
  bits: number;
};

export type OverloadSignature = {
  name: string;
  arguments: FunctionType[];
  returnType: FunctionType;
};

export type OverloadShard = {
  shardNumber: number;
  overloads: OverloadSignature[];
};

/**
 * This is done because there's a limit on how big
 * of a smart contract you can deploy
 */
export function splitOverloadsToShards(overloads: OverloadSignature[]): OverloadShard[] {
  const MAX_SHARD_SIZE = 150;
  const res: OverloadShard[] = [];

  var shardNo = 1;
  var accumulator: OverloadSignature[] = [];
  overloads.forEach((o) => {
    accumulator.push(o);
    if (accumulator.length >= MAX_SHARD_SIZE) {
      res.push({
        shardNumber: shardNo,
        overloads: Object.assign([], accumulator),
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

export function generateTestCode(shards: OverloadShard[]): string {
  const res: string[] = [];

  res.push(`
    import { expect } from 'chai';
    import { ethers } from 'hardhat';
    import { createInstances } from '../instance';
    import type { Signers } from '../types';

  `);

  shards.forEach((os) => {
    res.push(`
    import type { TFHETestSuite${os.shardNumber} } from '../../types/contracts/TFHETestSuite${os.shardNumber}';
    `);
  });

  shards.forEach((os) => {
    res.push(`
async function deployTfheTestFixture${os.shardNumber}(): Promise<TFHETestSuite${os.shardNumber}> {
  const signers = await ethers.getSigners();
  const admin = signers[0];

  const contractFactory = await ethers.getContractFactory('TFHETestSuite${os.shardNumber}');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}
    `);
  });

  res.push(`
    describe('TFHE operations', function () {
        before(async function () {
            this.signers = {} as Signers;
            const signers = await ethers.getSigners();
            this.signers.alice = signers[0];
            this.signers.bob = signers[1];
            this.signers.carol = signers[2];
            this.signers.dave = signers[3];

  `);

  shards.forEach((os) => {
    res.push(`
            const contract${os.shardNumber} = await deployTfheTestFixture${os.shardNumber}();
            this.contract${os.shardNumber}Address = await contract${os.shardNumber}.getAddress();
            this.contract${os.shardNumber} = contract${os.shardNumber};
            const instances${os.shardNumber} = await createInstances(this.contract${os.shardNumber}Address, ethers, this.signers);
            this.instances${os.shardNumber} = instances${os.shardNumber};
    `);
  });

  res.push(`
        });
  `);

  // don't allow user to add test for method that doesn't exist
  const overloadUsages: { [methodName: string]: boolean } = {};
  shards.forEach((os) => {
    os.overloads.forEach((o) => {
      const testName = `test operator "${o.name}" overload ${signatureContractEncryptedSignature(o)}`;
      const methodName = signatureContractMethodName(o);
      overloadUsages[methodName] = true;
      const tests = overloadTests[methodName] || [];
      assert(tests.length > 0, `Overload ${methodName} has no tests, please add them.`);
      var testIndex = 1;
      tests.forEach((t) => {
        assert(
          t.inputs.length == o.arguments.length,
          `Test argument count is different to operator arguments, arguments: ${t.inputs}, expected count: ${o.arguments.length}`,
        );
        t.inputs.forEach((input, index) => ensureNumberAcceptableInBitRange(o.arguments[index].bits, input));
        if (typeof t.output === 'number') {
          ensureNumberAcceptableInBitRange(o.returnType.bits, t.output);
        }
        const testArgs = t.inputs.join(', ');
        res.push(`
                it('${testName} test ${testIndex} (${testArgs})', async function () {
                    const res = await this.contract${os.shardNumber}.${methodName}(${testArgs});
                    expect(res).to.equal(${t.output});
                });
            `);
        testIndex++;
      });
    });
  });

  for (let key in overloadTests) {
    assert(overloadUsages[key], `No such overload '${key}' exists for which test data is defined`);
  }

  res.push(`
    });
  `);

  return res.join('');
}

function ensureNumberAcceptableInBitRange(bits: number, input: number) {
  switch (bits) {
    case 8:
      ensureNumberInRange(bits, input, 0x00, 0xff);
      break;
    case 16:
      ensureNumberInRange(bits, input, 0x00, 0xffff);
      break;
    case 32:
      ensureNumberInRange(bits, input, 0x00, 0xffffffff);
      break;
    default:
      assert(false, `TODO: add support for ${bits} numbers`);
  }
}

function ensureNumberInRange(bits: number, input: number, min: number, max: number) {
  assert(input >= min && input <= max, `${bits} bit number ${input} doesn't fall into expected [${min}; ${max}] range`);
}

export function generateSmartContract(os: OverloadShard): string {
  const res: string[] = [];

  res.push(`
        // SPDX-License-Identifier: BSD-3-Clause-Clear
        pragma solidity >=0.8.13 <0.8.20;

        import "../lib/TFHE.sol";
        contract TFHETestSuite${os.shardNumber} {
    `);

  os.overloads.forEach((o) => {
    const methodName = signatureContractMethodName(o);
    const args = signatureContractArguments(o);
    const retType = functionTypeToDecryptedType(o.returnType);
    res.push(`function ${methodName}(${args}) public view returns (${retType}) {`);
    res.push('\n');

    const procArgs: string[] = [];
    var argName = 97; // letter 'a' in ascii
    o.arguments.forEach((a) => {
      const arg = String.fromCharCode(argName);
      const argProc = `${arg}_proc`;
      procArgs.push(argProc);
      res.push(`${functionTypeToString(a)} ${argProc} = ${castExpressionToType(arg, a)};`);
      res.push('\n');
      argName++;
    });

    const tfheArgs = procArgs.join(', ');

    res.push(`${functionTypeToEncryptedType(o.returnType)} result = TFHE.${o.name}(${tfheArgs});`);
    res.push('\n');

    res.push(`return TFHE.decrypt(result);
        }
    `);
  });

  res.push(`
        }
    `);

  return res.join('');
}

export function signatureContractMethodName(s: OverloadSignature): string {
  const res: string[] = [];

  res.push(s.name);
  s.arguments.forEach((a) => res.push(functionTypeToString(a)));

  return res.join('_');
}

function signatureContractArguments(s: OverloadSignature): string {
  const res: string[] = [];

  var argName = 97; // letter 'a' in ascii
  s.arguments.forEach((a) => {
    res.push(`${functionTypeToDecryptedType(a)} ${String.fromCharCode(argName)}`);
    argName++;
  });

  return res.join(', ');
}

function signatureContractEncryptedSignature(s: OverloadSignature): string {
  const res: string[] = [];

  var argName = 97; // letter 'a' in ascii
  s.arguments.forEach((a) => {
    res.push(`${functionTypeToString(a)}`);
    argName++;
  });

  const joined = res.join(', ');
  return `(${joined}) => ${functionTypeToEncryptedType(s.returnType)}`;
}

function castExpressionToType(argExpr: string, outputType: FunctionType): string {
  switch (outputType.type) {
    case ArgumentType.EUint:
      return `TFHE.asEuint${outputType.bits}(${argExpr})`;
    case ArgumentType.Uint:
      return argExpr;
    case ArgumentType.Ebool:
      return `TFHE.asEbool(${argExpr})`;
  }
}

function functionTypeToDecryptedType(t: FunctionType): string {
  switch (t.type) {
    case ArgumentType.EUint:
    case ArgumentType.Uint:
      return `uint${t.bits}`;
    case ArgumentType.Ebool:
      return `bool`;
  }
}

function functionTypeToEncryptedType(t: FunctionType): string {
  switch (t.type) {
    case ArgumentType.EUint:
    case ArgumentType.Uint:
      return `euint${t.bits}`;
    case ArgumentType.Ebool:
      return `ebool`;
  }
}

function functionTypeToString(t: FunctionType): string {
  switch (t.type) {
    case ArgumentType.EUint:
      return `euint${t.bits}`;
    case ArgumentType.Uint:
      return `uint${t.bits}`;
    case ArgumentType.Ebool:
      return `ebool`;
  }
}
