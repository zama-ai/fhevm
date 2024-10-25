import dotenv from 'dotenv';
import { log2 } from 'extra-bigint';
import * as fs from 'fs';
import { ethers } from 'hardhat';
import hre from 'hardhat';
import { Database } from 'sqlite3';

import operatorsPrices from '../codegen/operatorsPrices.json';

const parsedEnvCoprocessor = dotenv.parse(fs.readFileSync('lib/.env.exec'));
const coprocAddress = parsedEnvCoprocessor.TFHE_EXECUTOR_CONTRACT_ADDRESS;
const coprocAdd = parsedEnvCoprocessor.TFHE_EXECUTOR_CONTRACT_ADDRESS.replace(/^0x/, '')
  .replace(/^0+/, '')
  .toLowerCase();

const parsedEnvACL = dotenv.parse(fs.readFileSync('lib/.env.acl'));
const aclAddress = parsedEnvACL.ACL_CONTRACT_ADDRESS;
let chainId: number;

let firstBlockListening = 0;
let lastBlockSnapshot = 0;
let lastCounterRand = 0;
let counterRand = 0;

const contractABI = JSON.parse(fs.readFileSync('artifacts/lib/TFHEExecutor.sol/TFHEExecutor.json').toString()).abi;

const iface = new ethers.Interface(contractABI);

const functions = iface.fragments.filter((fragment) => fragment.type === 'function');

const selectors = functions.reduce((acc, func) => {
  const signature = `${func.name}(${func.inputs.map((input) => input.type).join(',')})`;
  acc[func.selector] = signature;
  return acc;
}, {});

//const db = new Database('./sql.db'); // on-disk db for debugging
const db = new Database(':memory:');

export function insertSQL(handle: string, clearText: BigInt, replace: boolean = false) {
  if (replace) {
    // this is useful if using snapshots while sampling different random numbers on each revert
    db.run('INSERT OR REPLACE INTO ciphertexts (handle, clearText) VALUES (?, ?)', [handle, clearText.toString()]);
  } else {
    db.run('INSERT OR IGNORE INTO ciphertexts (handle, clearText) VALUES (?, ?)', [handle, clearText.toString()]);
  }
}

// Decrypt any handle, bypassing ACL
// WARNING : only for testing or internal use
export const getClearText = async (handle: BigInt): Promise<string> => {
  const handleStr = '0x' + handle.toString(16).padStart(64, '0');

  return new Promise((resolve, reject) => {
    let attempts = 0;
    const maxRetries = 100;

    function executeQuery() {
      db.get('SELECT clearText FROM ciphertexts WHERE handle = ?', [handleStr], (err, row) => {
        if (err) {
          reject(new Error(`Error querying database: ${err.message}`));
        } else if (row) {
          resolve(row.clearText);
        } else if (attempts < maxRetries) {
          attempts++;
          executeQuery();
        } else {
          reject(new Error('No record found after maximum retries'));
        }
      });
    }

    executeQuery();
  });
};

db.serialize(() => db.run('CREATE TABLE IF NOT EXISTS ciphertexts (handle BINARY PRIMARY KEY,clearText TEXT)'));

enum Operators {
  fheAdd = 0,
  fheSub,
  fheMul,
  fheDiv,
  fheRem,
  fheBitAnd,
  fheBitOr,
  fheBitXor,
  fheShl,
  fheShr,
  fheRotl,
  fheRotr,
  fheEq,
  fheNe,
  fheGe,
  fheGt,
  fheLe,
  fheLt,
  fheMin,
  fheMax,
  fheNeg,
  fheNot,
  verifyCiphertext,
  cast,
  trivialEncrypt,
  fheIfThenElse,
  fheRand,
  fheRandBounded,
}

interface EvmState {
  stack: string[];
  memory: string[];
}

interface FHEVMEvent {
  eventName: string;
  args: object;
}

function extractCalldata(memory: string[], offset: number, size: number): string {
  const startIndex = Math.floor(offset / 32);
  const endIndex = Math.ceil((offset + size) / 32);
  const memorySegments = memory.slice(startIndex, endIndex);
  let calldata = '';
  for (let i = 0; i < memorySegments.length; i++) {
    calldata += memorySegments[i];
  }
  const calldataStart = (offset % 32) * 2;
  const calldataEnd = calldataStart + size * 2;
  return calldata.slice(calldataStart, calldataEnd);
}

const NumBits = {
  0: 1n, //ebool
  1: 4n, //euint4
  2: 8n, //euint8
  3: 16n, //euint16
  4: 32n, //euint32
  5: 64n, //euint64
  6: 128n, //euint128
  7: 160n, //eaddress
  8: 256n, //euint256
  9: 512n, //ebytes64
  10: 1024n, //ebytes128
  11: 2048n, //ebytes256
};

const HANDLE_VERSION = 0;

export function numberToEvenHexString(num: number) {
  if (typeof num !== 'number' || num < 0) {
    throw new Error('Input should be a non-negative number.');
  }
  let hexString = num.toString(16);
  if (hexString.length % 2 !== 0) {
    hexString = '0' + hexString;
  }
  return hexString;
}

function appendType(handle: string, type: number): string {
  return handle.slice(0, -4) + numberToEvenHexString(type) + numberToEvenHexString(HANDLE_VERSION);
}

function getRandomBigInt(numBits: number): bigint {
  if (numBits <= 0) {
    throw new Error('Number of bits must be greater than 0');
  }
  const numBytes = Math.ceil(numBits / 8);
  const randomBytes = new Uint8Array(numBytes);
  crypto.getRandomValues(randomBytes);
  let randomBigInt = BigInt(0);
  for (let i = 0; i < numBytes; i++) {
    randomBigInt = (randomBigInt << BigInt(8)) | BigInt(randomBytes[i]);
  }
  const mask = (BigInt(1) << BigInt(numBits)) - BigInt(1);
  randomBigInt = randomBigInt & mask;
  return randomBigInt;
}

async function insertHandle(
  obj2: EvmState,
  validIdxes: [number],
  blockStatus: {
    blockhash: string;
    timestamp: number;
  },
) {
  const obj = obj2.value;
  if (isCoprocAdd(obj!.stack.at(-2))) {
    const argsOffset = Number(`0x${obj!.stack.at(-4)}`);
    const argsSize = Number(`0x${obj!.stack.at(-5)}`);
    const calldata = extractCalldata(obj.memory, argsOffset, argsSize);
    const currentSelector = '0x' + calldata.slice(0, 8);
    const decodedData = iface.decodeFunctionData(currentSelector, '0x' + calldata);

    let handle;
    let clearText;
    let clearLHS;
    let clearRHS;
    let lhsType;
    let resultType;
    let shift;

    switch (selectors[currentSelector]) {
      case 'trivialEncrypt(uint256,bytes1)':
        resultType = Number(decodedData[1]);
        clearText = decodedData[0];
        handle = ethers.keccak256(
          ethers.solidityPacked(
            ['uint8', 'uint256', 'bytes1', 'address', 'uint256'],
            [Operators.trivialEncrypt, decodedData[0], decodedData[1], aclAddress, chainId],
          ),
        );
        handle = appendType(handle, resultType);
        insertSQL(handle, clearText);
        break;

      case 'trivialEncrypt(bytes,bytes1)':
        resultType = Number(decodedData[1]);
        clearText = decodedData[0];
        handle = ethers.keccak256(
          ethers.solidityPacked(
            ['uint8', 'bytes', 'bytes1', 'address', 'uint256'],
            [Operators.trivialEncrypt, decodedData[0], decodedData[1], aclAddress, chainId],
          ),
        );
        handle = appendType(handle, resultType);
        insertSQL(handle, clearText);
        break;

      case 'fheAdd(uint256,uint256,bytes1)':
        handle = ethers.keccak256(
          ethers.solidityPacked(
            ['uint8', 'uint256', 'uint256', 'bytes1', 'address', 'uint256'],
            [Operators.fheAdd, decodedData[0], decodedData[1], decodedData[2], aclAddress, chainId],
          ),
        );
        lhsType = parseInt(decodedData[0].toString(16).slice(-4, -2), 16);
        resultType = lhsType;
        handle = appendType(handle, resultType);
        clearLHS = await getClearText(decodedData[0]);
        if (decodedData[2] === '0x01') {
          clearText = BigInt(clearLHS) + decodedData[1];
          clearText = clearText % 2n ** NumBits[resultType];
        } else {
          clearRHS = await getClearText(decodedData[1]);
          clearText = BigInt(clearLHS) + BigInt(clearRHS);
          clearText = clearText % 2n ** NumBits[resultType];
        }
        insertSQL(handle, clearText);
        break;

      case 'fheSub(uint256,uint256,bytes1)':
        handle = ethers.keccak256(
          ethers.solidityPacked(
            ['uint8', 'uint256', 'uint256', 'bytes1', 'address', 'uint256'],
            [Operators.fheSub, decodedData[0], decodedData[1], decodedData[2], aclAddress, chainId],
          ),
        );
        lhsType = parseInt(decodedData[0].toString(16).slice(-4, -2), 16);
        resultType = lhsType;
        handle = appendType(handle, resultType);
        clearLHS = await getClearText(decodedData[0]);
        if (decodedData[2] === '0x01') {
          clearText = BigInt(clearLHS) - decodedData[1];
          if (clearText < 0n) clearText = clearText + 2n ** NumBits[resultType];
          clearText = clearText % 2n ** NumBits[resultType];
        } else {
          clearRHS = await getClearText(decodedData[1]);
          clearText = BigInt(clearLHS) - BigInt(clearRHS);
          if (clearText < 0n) clearText = clearText + 2n ** NumBits[resultType];
          clearText = clearText % 2n ** NumBits[resultType];
        }
        insertSQL(handle, clearText);
        break;

      case 'fheMul(uint256,uint256,bytes1)':
        handle = ethers.keccak256(
          ethers.solidityPacked(
            ['uint8', 'uint256', 'uint256', 'bytes1', 'address', 'uint256'],
            [Operators.fheMul, decodedData[0], decodedData[1], decodedData[2], aclAddress, chainId],
          ),
        );
        lhsType = parseInt(decodedData[0].toString(16).slice(-4, -2), 16);
        resultType = lhsType;
        handle = appendType(handle, resultType);
        clearLHS = await getClearText(decodedData[0]);
        if (decodedData[2] === '0x01') {
          clearText = BigInt(clearLHS) * decodedData[1];
          clearText = clearText % 2n ** NumBits[resultType];
        } else {
          clearRHS = await getClearText(decodedData[1]);
          clearText = BigInt(clearLHS) * BigInt(clearRHS);
          clearText = clearText % 2n ** NumBits[resultType];
        }
        insertSQL(handle, clearText);
        break;

      case 'fheDiv(uint256,uint256,bytes1)':
        handle = ethers.keccak256(
          ethers.solidityPacked(
            ['uint8', 'uint256', 'uint256', 'bytes1', 'address', 'uint256'],
            [Operators.fheDiv, decodedData[0], decodedData[1], decodedData[2], aclAddress, chainId],
          ),
        );
        lhsType = parseInt(decodedData[0].toString(16).slice(-4, -2), 16);
        resultType = lhsType;
        handle = appendType(handle, resultType);
        clearLHS = await getClearText(decodedData[0]);
        if (decodedData[2] === '0x01') {
          clearText = BigInt(clearLHS) / decodedData[1];
        } else {
          throw new Error('Non-scalar div not implemented yet');
        }
        insertSQL(handle, clearText);
        break;

      case 'fheRem(uint256,uint256,bytes1)':
        handle = ethers.keccak256(
          ethers.solidityPacked(
            ['uint8', 'uint256', 'uint256', 'bytes1', 'address', 'uint256'],
            [Operators.fheRem, decodedData[0], decodedData[1], decodedData[2], aclAddress, chainId],
          ),
        );
        lhsType = parseInt(decodedData[0].toString(16).slice(-4, -2), 16);
        resultType = lhsType;
        handle = appendType(handle, resultType);
        clearLHS = await getClearText(decodedData[0]);
        if (decodedData[2] === '0x01') {
          clearText = BigInt(clearLHS) % decodedData[1];
        } else {
          throw new Error('Non-scalar rem not implemented yet');
        }
        insertSQL(handle, clearText);
        break;

      case 'fheBitAnd(uint256,uint256,bytes1)':
        handle = ethers.keccak256(
          ethers.solidityPacked(
            ['uint8', 'uint256', 'uint256', 'bytes1', 'address', 'uint256'],
            [Operators.fheBitAnd, decodedData[0], decodedData[1], decodedData[2], aclAddress, chainId],
          ),
        );
        lhsType = parseInt(decodedData[0].toString(16).slice(-4, -2), 16);
        resultType = lhsType;
        handle = appendType(handle, resultType);
        clearLHS = await getClearText(decodedData[0]);
        if (decodedData[2] === '0x01') {
          clearText = BigInt(clearLHS) & decodedData[1];
          clearText = clearText % 2n ** NumBits[resultType];
        } else {
          clearRHS = await getClearText(decodedData[1]);
          clearText = BigInt(clearLHS) & BigInt(clearRHS);
          clearText = clearText % 2n ** NumBits[resultType];
        }
        insertSQL(handle, clearText);
        break;

      case 'fheBitOr(uint256,uint256,bytes1)':
        handle = ethers.keccak256(
          ethers.solidityPacked(
            ['uint8', 'uint256', 'uint256', 'bytes1', 'address', 'uint256'],
            [Operators.fheBitOr, decodedData[0], decodedData[1], decodedData[2], aclAddress, chainId],
          ),
        );
        lhsType = parseInt(decodedData[0].toString(16).slice(-4, -2), 16);
        resultType = lhsType;
        handle = appendType(handle, resultType);
        clearLHS = await getClearText(decodedData[0]);
        if (decodedData[2] === '0x01') {
          clearText = BigInt(clearLHS) | decodedData[1];
          clearText = clearText % 2n ** NumBits[resultType];
        } else {
          clearRHS = await getClearText(decodedData[1]);
          clearText = BigInt(clearLHS) | BigInt(clearRHS);
          clearText = clearText % 2n ** NumBits[resultType];
        }
        insertSQL(handle, clearText);
        break;

      case 'fheBitXor(uint256,uint256,bytes1)':
        handle = ethers.keccak256(
          ethers.solidityPacked(
            ['uint8', 'uint256', 'uint256', 'bytes1', 'address', 'uint256'],
            [Operators.fheBitXor, decodedData[0], decodedData[1], decodedData[2], aclAddress, chainId],
          ),
        );
        lhsType = parseInt(decodedData[0].toString(16).slice(-4, -2), 16);
        resultType = lhsType;
        handle = appendType(handle, resultType);
        clearLHS = await getClearText(decodedData[0]);
        if (decodedData[2] === '0x01') {
          clearText = BigInt(clearLHS) ^ decodedData[1];
          clearText = clearText % 2n ** NumBits[resultType];
        } else {
          clearRHS = await getClearText(decodedData[1]);
          clearText = BigInt(clearLHS) ^ BigInt(clearRHS);
          clearText = clearText % 2n ** NumBits[resultType];
        }
        insertSQL(handle, clearText);
        break;

      case 'fheShl(uint256,uint256,bytes1)':
        handle = ethers.keccak256(
          ethers.solidityPacked(
            ['uint8', 'uint256', 'uint256', 'bytes1', 'address', 'uint256'],
            [Operators.fheShl, decodedData[0], decodedData[1], decodedData[2], aclAddress, chainId],
          ),
        );
        lhsType = parseInt(decodedData[0].toString(16).slice(-4, -2), 16);
        resultType = lhsType;
        handle = appendType(handle, resultType);
        clearLHS = await getClearText(decodedData[0]);
        if (decodedData[2] === '0x01') {
          clearText = BigInt(clearLHS) << decodedData[1] % NumBits[resultType];
          clearText = clearText % 2n ** NumBits[resultType];
        } else {
          clearRHS = await getClearText(decodedData[1]);
          clearText = BigInt(clearLHS) << BigInt(clearRHS) % NumBits[resultType];
          clearText = clearText % 2n ** NumBits[resultType];
        }
        insertSQL(handle, clearText);
        break;

      case 'fheShr(uint256,uint256,bytes1)':
        handle = ethers.keccak256(
          ethers.solidityPacked(
            ['uint8', 'uint256', 'uint256', 'bytes1', 'address', 'uint256'],
            [Operators.fheShr, decodedData[0], decodedData[1], decodedData[2], aclAddress, chainId],
          ),
        );
        lhsType = parseInt(decodedData[0].toString(16).slice(-4, -2), 16);
        resultType = lhsType;
        handle = appendType(handle, resultType);
        clearLHS = await getClearText(decodedData[0]);
        if (decodedData[2] === '0x01') {
          clearText = BigInt(clearLHS) >> decodedData[1] % NumBits[resultType];
          clearText = clearText % 2n ** NumBits[resultType];
        } else {
          clearRHS = await getClearText(decodedData[1]);
          clearText = BigInt(clearLHS) >> BigInt(clearRHS) % NumBits[resultType];
          clearText = clearText % 2n ** NumBits[resultType];
        }
        insertSQL(handle, clearText);
        break;

      case 'fheRotl(uint256,uint256,bytes1)':
        handle = ethers.keccak256(
          ethers.solidityPacked(
            ['uint8', 'uint256', 'uint256', 'bytes1', 'address', 'uint256'],
            [Operators.fheRotl, decodedData[0], decodedData[1], decodedData[2], aclAddress, chainId],
          ),
        );
        lhsType = parseInt(decodedData[0].toString(16).slice(-4, -2), 16);
        resultType = lhsType;
        handle = appendType(handle, resultType);
        clearLHS = await getClearText(decodedData[0]);

        if (decodedData[2] === '0x01') {
          shift = decodedData[1] % NumBits[resultType];
          clearText = (BigInt(clearLHS) << shift) | (BigInt(clearLHS) >> (NumBits[resultType] - shift));
          clearText = clearText % 2n ** NumBits[resultType];
        } else {
          clearRHS = await getClearText(decodedData[1]);
          shift = BigInt(clearRHS) % NumBits[resultType];
          clearText = (BigInt(clearLHS) << shift) | (BigInt(clearLHS) >> (NumBits[resultType] - shift));
          clearText = clearText % 2n ** NumBits[resultType];
        }
        insertSQL(handle, clearText);
        break;

      case 'fheRotr(uint256,uint256,bytes1)':
        handle = ethers.keccak256(
          ethers.solidityPacked(
            ['uint8', 'uint256', 'uint256', 'bytes1', 'address', 'uint256'],
            [Operators.fheRotr, decodedData[0], decodedData[1], decodedData[2], aclAddress, chainId],
          ),
        );
        lhsType = parseInt(decodedData[0].toString(16).slice(-4, -2), 16);
        resultType = lhsType;
        handle = appendType(handle, resultType);
        clearLHS = await getClearText(decodedData[0]);

        if (decodedData[2] === '0x01') {
          shift = decodedData[1] % NumBits[resultType];
          clearText = (BigInt(clearLHS) >> shift) | (BigInt(clearLHS) << (NumBits[resultType] - shift));
          clearText = clearText % 2n ** NumBits[resultType];
        } else {
          clearRHS = await getClearText(decodedData[1]);
          shift = BigInt(clearRHS) % NumBits[resultType];
          clearText = (BigInt(clearLHS) >> shift) | (BigInt(clearLHS) << (NumBits[resultType] - shift));
          clearText = clearText % 2n ** NumBits[resultType];
        }
        insertSQL(handle, clearText);
        break;

      case 'fheEq(uint256,uint256,bytes1)':
        handle = ethers.keccak256(
          ethers.solidityPacked(
            ['uint8', 'uint256', 'uint256', 'bytes1', 'address', 'uint256'],
            [Operators.fheEq, decodedData[0], decodedData[1], decodedData[2], aclAddress, chainId],
          ),
        );
        handle = appendType(handle, 0);
        clearLHS = await getClearText(decodedData[0]);
        if (decodedData[2] === '0x01') {
          clearText = BigInt(clearLHS) === decodedData[1] ? 1n : 0n;
        } else {
          clearRHS = await getClearText(decodedData[1]);
          clearText = BigInt(clearLHS) === BigInt(clearRHS) ? 1n : 0n;
        }
        insertSQL(handle, clearText);
        break;

      case 'fheEq(uint256,bytes,bytes1)':
        handle = ethers.keccak256(
          ethers.solidityPacked(
            ['uint8', 'uint256', 'bytes', 'bytes1', 'address', 'uint256'],
            [Operators.fheEq, decodedData[0], decodedData[1], decodedData[2], aclAddress, chainId],
          ),
        );
        handle = appendType(handle, 0);
        clearLHS = await getClearText(decodedData[0]);
        if (decodedData[2] === '0x01') {
          clearText = BigInt(clearLHS) === BigInt(decodedData[1]) ? 1n : 0n;
        } else {
          clearRHS = await getClearText(decodedData[1]);
          clearText = BigInt(clearLHS) === BigInt(clearRHS) ? 1n : 0n;
        }
        insertSQL(handle, clearText);
        break;

      case 'fheNe(uint256,uint256,bytes1)':
        handle = ethers.keccak256(
          ethers.solidityPacked(
            ['uint8', 'uint256', 'uint256', 'bytes1', 'address', 'uint256'],
            [Operators.fheNe, decodedData[0], decodedData[1], decodedData[2], aclAddress, chainId],
          ),
        );
        handle = appendType(handle, 0);
        clearLHS = await getClearText(decodedData[0]);
        if (decodedData[2] === '0x01') {
          clearText = BigInt(clearLHS) !== decodedData[1] ? 1n : 0n;
        } else {
          clearRHS = await getClearText(decodedData[1]);
          clearText = BigInt(clearLHS) !== BigInt(clearRHS) ? 1n : 0n;
        }
        insertSQL(handle, clearText);
        break;

      case 'fheNe(uint256,bytes,bytes1)':
        handle = ethers.keccak256(
          ethers.solidityPacked(
            ['uint8', 'uint256', 'bytes', 'bytes1', 'address', 'uint256'],
            [Operators.fheNe, decodedData[0], decodedData[1], decodedData[2], aclAddress, chainId],
          ),
        );
        handle = appendType(handle, 0);
        clearLHS = await getClearText(decodedData[0]);
        if (decodedData[2] === '0x01') {
          clearText = BigInt(clearLHS) !== BigInt(decodedData[1]) ? 1n : 0n;
        } else {
          clearRHS = await getClearText(decodedData[1]);
          clearText = BigInt(clearLHS) !== BigInt(clearRHS) ? 1n : 0n;
        }
        insertSQL(handle, clearText);
        break;

      case 'fheGe(uint256,uint256,bytes1)':
        handle = ethers.keccak256(
          ethers.solidityPacked(
            ['uint8', 'uint256', 'uint256', 'bytes1', 'address', 'uint256'],
            [Operators.fheGe, decodedData[0], decodedData[1], decodedData[2], aclAddress, chainId],
          ),
        );
        handle = appendType(handle, 0);
        clearLHS = await getClearText(decodedData[0]);
        if (decodedData[2] === '0x01') {
          clearText = BigInt(clearLHS) >= decodedData[1] ? 1n : 0n;
        } else {
          clearRHS = await getClearText(decodedData[1]);
          clearText = BigInt(clearLHS) >= BigInt(clearRHS) ? 1n : 0n;
        }
        insertSQL(handle, clearText);
        break;

      case 'fheGt(uint256,uint256,bytes1)':
        handle = ethers.keccak256(
          ethers.solidityPacked(
            ['uint8', 'uint256', 'uint256', 'bytes1', 'address', 'uint256'],
            [Operators.fheGt, decodedData[0], decodedData[1], decodedData[2], aclAddress, chainId],
          ),
        );
        handle = appendType(handle, 0);
        clearLHS = await getClearText(decodedData[0]);
        if (decodedData[2] === '0x01') {
          clearText = BigInt(clearLHS) > decodedData[1] ? 1n : 0n;
        } else {
          clearRHS = await getClearText(decodedData[1]);
          clearText = BigInt(clearLHS) > BigInt(clearRHS) ? 1n : 0n;
        }
        insertSQL(handle, clearText);
        break;

      case 'fheLe(uint256,uint256,bytes1)':
        handle = ethers.keccak256(
          ethers.solidityPacked(
            ['uint8', 'uint256', 'uint256', 'bytes1', 'address', 'uint256'],
            [Operators.fheLe, decodedData[0], decodedData[1], decodedData[2], aclAddress, chainId],
          ),
        );
        handle = appendType(handle, 0);
        clearLHS = await getClearText(decodedData[0]);
        if (decodedData[2] === '0x01') {
          clearText = BigInt(clearLHS) <= decodedData[1] ? 1n : 0n;
        } else {
          clearRHS = await getClearText(decodedData[1]);
          clearText = BigInt(clearLHS) <= BigInt(clearRHS) ? 1n : 0n;
        }
        insertSQL(handle, clearText);
        break;

      case 'fheLt(uint256,uint256,bytes1)':
        handle = ethers.keccak256(
          ethers.solidityPacked(
            ['uint8', 'uint256', 'uint256', 'bytes1', 'address', 'uint256'],
            [Operators.fheLt, decodedData[0], decodedData[1], decodedData[2], aclAddress, chainId],
          ),
        );
        handle = appendType(handle, 0);
        clearLHS = await getClearText(decodedData[0]);
        if (decodedData[2] === '0x01') {
          clearText = BigInt(clearLHS) < decodedData[1] ? 1n : 0n;
        } else {
          clearRHS = await getClearText(decodedData[1]);
          clearText = BigInt(clearLHS) < BigInt(clearRHS) ? 1n : 0n;
        }
        insertSQL(handle, clearText);
        break;

      case 'fheMax(uint256,uint256,bytes1)':
        lhsType = parseInt(decodedData[0].toString(16).slice(-4, -2), 16);
        resultType = lhsType;
        handle = ethers.keccak256(
          ethers.solidityPacked(
            ['uint8', 'uint256', 'uint256', 'bytes1', 'address', 'uint256'],
            [Operators.fheMax, decodedData[0], decodedData[1], decodedData[2], aclAddress, chainId],
          ),
        );
        handle = appendType(handle, resultType);
        clearLHS = await getClearText(decodedData[0]);
        if (decodedData[2] === '0x01') {
          clearText = BigInt(clearLHS) > decodedData[1] ? clearLHS : decodedData[1];
        } else {
          clearRHS = await getClearText(decodedData[1]);
          clearText = BigInt(clearLHS) > BigInt(clearRHS) ? clearLHS : clearRHS;
        }
        insertSQL(handle, clearText);
        break;

      case 'fheMin(uint256,uint256,bytes1)':
        lhsType = parseInt(decodedData[0].toString(16).slice(-4, -2), 16);
        resultType = lhsType;
        handle = ethers.keccak256(
          ethers.solidityPacked(
            ['uint8', 'uint256', 'uint256', 'bytes1', 'address', 'uint256'],
            [Operators.fheMin, decodedData[0], decodedData[1], decodedData[2], aclAddress, chainId],
          ),
        );
        handle = appendType(handle, resultType);
        clearLHS = await getClearText(decodedData[0]);
        if (decodedData[2] === '0x01') {
          clearText = BigInt(clearLHS) < decodedData[1] ? clearLHS : decodedData[1];
        } else {
          clearRHS = await getClearText(decodedData[1]);
          clearText = BigInt(clearLHS) < BigInt(clearRHS) ? clearLHS : clearRHS;
        }
        insertSQL(handle, clearText);
        break;

      case 'cast(uint256,bytes1)':
        resultType = parseInt(decodedData[1]);
        handle = ethers.keccak256(
          ethers.solidityPacked(
            ['uint8', 'uint256', 'bytes1', 'address', 'uint256'],
            [Operators.cast, decodedData[0], decodedData[1], aclAddress, chainId],
          ),
        );
        clearText = BigInt(await getClearText(decodedData[0])) % 2n ** NumBits[resultType];
        handle = appendType(handle, resultType);
        insertSQL(handle, clearText);
        break;

      case 'fheNot(uint256)':
        resultType = parseInt(decodedData[0].toString(16).slice(-4, -2), 16);
        handle = ethers.keccak256(
          ethers.solidityPacked(
            ['uint8', 'uint256', 'address', 'uint256'],
            [Operators.fheNot, decodedData[0], aclAddress, chainId],
          ),
        );
        handle = appendType(handle, resultType);
        clearText = BigInt(await getClearText(decodedData[0]));
        clearText = bitwiseNotUintBits(clearText, Number(NumBits[resultType]));
        insertSQL(handle, clearText);
        break;

      case 'fheNeg(uint256)':
        resultType = parseInt(decodedData[0].toString(16).slice(-4, -2), 16);
        handle = ethers.keccak256(
          ethers.solidityPacked(
            ['uint8', 'uint256', 'address', 'uint256'],
            [Operators.fheNeg, decodedData[0], aclAddress, chainId],
          ),
        );
        handle = appendType(handle, resultType);
        clearText = BigInt(await getClearText(decodedData[0]));
        clearText = bitwiseNotUintBits(clearText, Number(NumBits[resultType]));
        clearText = (clearText + 1n) % 2n ** NumBits[resultType];
        insertSQL(handle, clearText);
        break;

      case 'verifyCiphertext(bytes32,address,bytes,bytes1)':
        try {
          await getClearText(BigInt(decodedData[0]));
        } catch {
          throw Error('User input was not found in DB');
        }
        break;

      case 'fheIfThenElse(uint256,uint256,uint256)':
        resultType = parseInt(decodedData[1].toString(16).slice(-4, -2), 16);
        handle = ethers.keccak256(
          ethers.solidityPacked(
            ['uint8', 'uint256', 'uint256', 'uint256', 'address', 'uint256'],
            [Operators.fheIfThenElse, decodedData[0], decodedData[1], decodedData[2], aclAddress, chainId],
          ),
        );
        handle = appendType(handle, resultType);
        const clearControl = BigInt(await getClearText(decodedData[0]));
        const clearIfTrue = BigInt(await getClearText(decodedData[1]));
        const clearIfFalse = BigInt(await getClearText(decodedData[2]));
        if (clearControl === 1n) {
          clearText = clearIfTrue;
        } else {
          clearText = clearIfFalse;
        }
        insertSQL(handle, clearText);
        break;

      case 'fheRand(bytes1)':
        if (validIdxes.includes(obj2.index)) {
          resultType = parseInt(decodedData[0], 16);
          const preseed = ethers.keccak256(
            ethers.solidityPacked(
              ['uint256', 'address', 'uint256', 'uint256', 'uint256'],
              [counterRand, aclAddress, chainId, blockStatus.blockhash, blockStatus.timestamp],
            ),
          );
          const seed = preseed.slice(0, 34);
          handle = ethers.keccak256(
            ethers.solidityPacked(['uint8', 'bytes1', 'bytes16'], [Operators.fheRand, decodedData[0], seed]),
          );
          handle = appendType(handle, resultType);
          clearText = getRandomBigInt(Number(NumBits[resultType]));
          insertSQL(handle, clearText, true);
          counterRand++;
        }
        break;

      case 'fheRandBounded(uint256,bytes1)':
        if (validIdxes.includes(obj2.index)) {
          resultType = parseInt(decodedData[1], 16);
          const preseed = ethers.keccak256(
            ethers.solidityPacked(
              ['uint256', 'address', 'uint256', 'uint256', 'uint256'],
              [counterRand, aclAddress, chainId, blockStatus.blockhash, blockStatus.timestamp],
            ),
          );
          const seed = preseed.slice(0, 34);
          handle = ethers.keccak256(
            ethers.solidityPacked(
              ['uint8', 'uint256', 'bytes1', 'bytes16'],
              [Operators.fheRandBounded, decodedData[0], decodedData[1], seed],
            ),
          );
          handle = appendType(handle, resultType);
          clearText = getRandomBigInt(Number(log2(BigInt(decodedData[0]))));
          insertSQL(handle, clearText, true);
          counterRand++;
        }
        break;
    }
  }
}

function bitwiseNotUintBits(value: BigInt, numBits: number) {
  if (typeof value !== 'bigint') {
    throw new TypeError('The input value must be a BigInt.');
  }
  if (typeof numBits !== 'number' || numBits <= 0) {
    throw new TypeError('The numBits parameter must be a positive integer.');
  }
  // Create the mask with numBits bits set to 1
  const BIT_MASK = (BigInt(1) << BigInt(numBits)) - BigInt(1);
  return ~value & BIT_MASK;
}

function isCoprocAdd(longString: string): boolean {
  const strippedLongString = longString.replace(/^0+/, '');
  const normalizedLongString = strippedLongString.toLowerCase();
  return normalizedLongString === coprocAdd;
}

async function processLogs(trace, validSubcallsIndexes, blockStatus) {
  for (const obj of trace.structLogs
    .map((value, index) => ({ value, index }))
    .filter((obj) => obj.value.op === 'CALL')) {
    await insertHandle(obj, validSubcallsIndexes, blockStatus);
  }
}

export const awaitCoprocessor = async (): Promise<void> => {
  chainId = (await ethers.provider.getNetwork()).chainId;
  if (process.env.HARDHAT_TFHEEXECUTOR_EVENTS !== '1') {
    const pastTxHashes = await getAllPastTransactionHashes();
    for (const txHash of pastTxHashes) {
      const trace = await ethers.provider.send('debug_traceTransaction', [txHash[0]]);
      if (!trace.failed) {
        const callTree = await buildCallTree(trace, txHash[1]);
        const validSubcallsIndexes = getValidSubcallsIds(callTree)[1];
        await processLogs(trace, validSubcallsIndexes, {
          timestamp: txHash[1].timestamp,
          blockhash: txHash[1].blockhash,
        });
      }
    }
  } else {
    // with events we can go way faster since we don't need to trace all tx subcall
    await processAllPastTFHEExecutorEvents();
  }
};

async function getAllPastTransactionHashes() {
  const provider = ethers.provider;
  const latestBlockNumber = await provider.getBlockNumber();
  let txHashes = [];

  if (hre.__SOLIDITY_COVERAGE_RUNNING !== true) {
    // evm_snapshot is not supported in coverage mode
    [lastBlockSnapshot, lastCounterRand] = await provider.send('get_lastBlockSnapshot');
    if (lastBlockSnapshot < firstBlockListening) {
      firstBlockListening = lastBlockSnapshot + 1;
      counterRand = Number(lastCounterRand);
    }
  }

  // Iterate through all blocks and collect transaction hashes
  for (let i = firstBlockListening; i <= latestBlockNumber; i++) {
    const block = await provider.getBlock(i, true);
    block!.transactions.forEach((tx, index) => {
      const rcpt = block?.prefetchedTransactions[index];
      txHashes.push([
        tx,
        { to: rcpt.to, status: rcpt.status, blockhash: block?.parentHash, timestamp: block?.timestamp },
      ]);
    });
  }
  firstBlockListening = latestBlockNumber + 1;
  if (hre.__SOLIDITY_COVERAGE_RUNNING !== true) {
    // evm_snapshot is not supported in coverage mode
    await provider.send('set_lastBlockSnapshot', [firstBlockListening]);
  }
  return txHashes;
}

async function buildCallTree(trace, receipt) {
  const structLogs = trace.structLogs;

  const callStack = [];
  const callTree = {
    id: 0,
    type: !!receipt.to ? 'TOPCALL' : 'TOPCREATE',
    revert: receipt.status === 1 ? false : true,
    to: !!receipt.to ? receipt.to : null,
    calls: [],
    indexTrace: 0,
  };
  let currentNode = callTree;
  const lenStructLogs = structLogs.length;
  let index = 1;
  for (const [i, log] of structLogs.entries()) {
    if (i < lenStructLogs - 1) {
      if (structLogs[i].depth - structLogs[i + 1].depth === 1) {
        if (!['RETURN', 'SELFDESTRUCT', 'STOP', 'REVERT', 'INVALID'].includes(structLogs[i].op)) {
          currentNode.outofgasOrOther = true;
          currentNode = callStack.pop();
        }
      }
    }

    switch (log.op) {
      case 'CALL':
      case 'DELEGATECALL':
      case 'CALLCODE':
      case 'STATICCALL':
      case 'CREATE':
      case 'CREATE2':
        if (i < lenStructLogs - 1) {
          if (structLogs[i + 1].depth - structLogs[i].depth === 1) {
            const newNode = {
              id: index,
              type: log.op,
              to: log.stack[log.stack.length - 2],
              calls: [],
              revert: true,
              outofgasOrOther: false,
              indexTrace: i,
            };
            currentNode.calls.push(newNode);
            callStack.push(currentNode);
            currentNode = newNode;
            index += 1;
          }
        }
        break;
      case 'RETURN': // some edge case probably not handled well : if memory expansion cost on RETURN exceeds the remaining gas in current subcall, but it's OK for a mocked mode
      case 'SELFDESTRUCT': // some edge case probably not handled well : if there is not enough gas remaining on SELFDESTRUCT, but it's OK for a mocked mode
      case 'STOP':
        currentNode.revert = false;
        currentNode = callStack.pop();
        break;
      case 'REVERT':
      case 'INVALID':
        currentNode = callStack.pop();
        break;
    }

    switch (log.op) {
      case 'CREATE':
      case 'CREATE2':
        currentNode.to = null;
        break;
    }
  }
  return callTree;
}

function logCallContextsTree(callContext, indent = 0) {
  const indentation = ' '.repeat(indent);
  console.log(`${indentation}id: ${callContext.id}, type: ${callContext.type}, revert: ${callContext.revert}`);

  if (callContext.calls.length > 0) {
    console.log(`${indentation}  Calls:`);
    for (const call of callContext.calls) {
      logCallContextsTree(call, indent + 4);
    }
  }
}

function getValidSubcallsIds(tree) {
  const result = [];
  const resultIndexes = [];

  function traverse(node, ancestorReverted) {
    if (ancestorReverted || node.revert) {
      ancestorReverted = true;
    } else {
      result.push(node.id);
      resultIndexes.push(node.indexTrace);
    }
    for (const child of node.calls) {
      traverse(child, ancestorReverted);
    }
  }

  traverse(tree, false);

  return [result, resultIndexes];
}

const abi = [
  'event FheAdd(uint256 lhs, uint256 rhs, bytes1 scalarByte, uint256 result)',
  'event FheSub(uint256 lhs, uint256 rhs, bytes1 scalarByte, uint256 result)',
  'event FheMul(uint256 lhs, uint256 rhs, bytes1 scalarByte, uint256 result)',
  'event FheDiv(uint256 lhs, uint256 rhs, bytes1 scalarByte, uint256 result)',
  'event FheRem(uint256 lhs, uint256 rhs, bytes1 scalarByte, uint256 result)',
  'event FheBitAnd(uint256 lhs, uint256 rhs, bytes1 scalarByte, uint256 result)',
  'event FheBitOr(uint256 lhs, uint256 rhs, bytes1 scalarByte, uint256 result)',
  'event FheBitXor(uint256 lhs, uint256 rhs, bytes1 scalarByte, uint256 result)',
  'event FheShl(uint256 lhs, uint256 rhs, bytes1 scalarByte, uint256 result)',
  'event FheShr(uint256 lhs, uint256 rhs, bytes1 scalarByte, uint256 result)',
  'event FheRotl(uint256 lhs, uint256 rhs, bytes1 scalarByte, uint256 result)',
  'event FheRotr(uint256 lhs, uint256 rhs, bytes1 scalarByte, uint256 result)',
  'event FheEq(uint256 lhs, uint256 rhs, bytes1 scalarByte, uint256 result)',
  'event FheEqBytes(uint256 lhs, bytes rhs, bytes1 scalarByte, uint256 result)',
  'event FheNe(uint256 lhs, uint256 rhs, bytes1 scalarByte, uint256 result)',
  'event FheNeBytes(uint256 lhs, bytes rhs, bytes1 scalarByte, uint256 result)',
  'event FheGe(uint256 lhs, uint256 rhs, bytes1 scalarByte, uint256 result)',
  'event FheGt(uint256 lhs, uint256 rhs, bytes1 scalarByte, uint256 result)',
  'event FheLe(uint256 lhs, uint256 rhs, bytes1 scalarByte, uint256 result)',
  'event FheLt(uint256 lhs, uint256 rhs, bytes1 scalarByte, uint256 result)',
  'event FheMin(uint256 lhs, uint256 rhs, bytes1 scalarByte, uint256 result)',
  'event FheMax(uint256 lhs, uint256 rhs, bytes1 scalarByte, uint256 result)',
  'event TrivialEncrypt(uint256 pt, bytes1 toType, uint256 result)',
  'event TrivialEncrypt(bytes pt, bytes1 toType, uint256 result)',
  'event FheNeg(uint256 ct, uint256 result)',
  'event FheNot(uint256 ct, uint256 result)',
  'event VerifyCiphertext(bytes32 inputHandle,address userAddress,bytes inputProof,bytes1 inputType,uint256 result)',
  'event Cast(uint256 ct, bytes1 toType, uint256 result)',
  'event TrivialEncrypt(uint256 pt, bytes1 toType, uint256 result)',
  'event TrivialEncryptBytes(bytes pt, bytes1 toType, uint256 result)',
  'event FheIfThenElse(uint256 control, uint256 ifTrue, uint256 ifFalse, uint256 result)',
  'event FheRand(bytes1 randType, uint256 result)',
  'event FheRandBounded(uint256 upperBound, bytes1 randType, uint256 result)',
];

async function processAllPastTFHEExecutorEvents() {
  const provider = ethers.provider;
  const latestBlockNumber = await provider.getBlockNumber();

  if (hre.__SOLIDITY_COVERAGE_RUNNING !== true) {
    // evm_snapshot is not supported in coverage mode
    [lastBlockSnapshot, lastCounterRand] = await provider.send('get_lastBlockSnapshot');
    if (lastBlockSnapshot < firstBlockListening) {
      firstBlockListening = lastBlockSnapshot + 1;
      counterRand = Number(lastCounterRand);
    }
  }

  const contract = new ethers.Contract(coprocAddress, abi, provider);

  // Fetch all events emitted by the contract
  const filter = {
    address: coprocAddress,
    fromBlock: firstBlockListening,
    toBlock: latestBlockNumber,
  };

  const logs = await provider.getLogs(filter);

  const events = logs
    .map((log) => {
      try {
        const parsedLog = contract.interface.parseLog(log);
        return {
          eventName: parsedLog.name,
          args: parsedLog.args,
        };
      } catch (e) {
        // If the log cannot be parsed, skip it
        return null;
      }
    })
    .filter((event) => event !== null);

  firstBlockListening = latestBlockNumber + 1;
  if (hre.__SOLIDITY_COVERAGE_RUNNING !== true) {
    // evm_snapshot is not supported in coverage mode
    await provider.send('set_lastBlockSnapshot', [firstBlockListening]);
  }
  events.map(async (event) => await insertHandleFromEvent(event));
}

async function insertHandleFromEvent(event: FHEVMEvent) {
  let handle;
  let clearText;
  let clearLHS;
  let clearRHS;
  let resultType;
  let shift;

  switch (event.eventName) {
    case 'TrivialEncrypt':
      clearText = event.args[0];
      handle = ethers.toBeHex(event.args[2], 32);
      insertSQL(handle, clearText);
      break;

    case 'TrivialEncryptBytes':
      clearText = event.args[0];
      handle = ethers.toBeHex(event.args[2], 32);
      insertSQL(handle, clearText);
      break;

    case 'FheAdd':
      handle = ethers.toBeHex(event.args[3], 32);
      resultType = parseInt(handle.slice(-4, -2), 16);
      clearLHS = await getClearText(event.args[0]);
      if (event.args[2] === '0x01') {
        clearText = BigInt(clearLHS) + event.args[1];
        clearText = clearText % 2n ** NumBits[resultType];
      } else {
        clearRHS = await getClearText(event.args[1]);
        clearText = BigInt(clearLHS) + BigInt(clearRHS);
        clearText = clearText % 2n ** NumBits[resultType];
      }
      insertSQL(ethers.toBeHex(handle, 32), clearText);
      break;

    case 'FheSub':
      handle = ethers.toBeHex(event.args[3], 32);
      resultType = parseInt(handle.slice(-4, -2), 16);
      clearLHS = await getClearText(event.args[0]);
      if (event.args[2] === '0x01') {
        clearText = BigInt(clearLHS) - event.args[1];
        if (clearText < 0n) clearText = clearText + 2n ** NumBits[resultType];
        clearText = clearText % 2n ** NumBits[resultType];
      } else {
        clearRHS = await getClearText(event.args[1]);
        clearText = BigInt(clearLHS) - BigInt(clearRHS);
        if (clearText < 0n) clearText = clearText + 2n ** NumBits[resultType];
        clearText = clearText % 2n ** NumBits[resultType];
      }
      insertSQL(handle, clearText);
      break;

    case 'FheMul':
      handle = ethers.toBeHex(event.args[3], 32);
      resultType = parseInt(handle.slice(-4, -2), 16);
      clearLHS = await getClearText(event.args[0]);
      if (event.args[2] === '0x01') {
        clearText = BigInt(clearLHS) * event.args[1];
        clearText = clearText % 2n ** NumBits[resultType];
      } else {
        clearRHS = await getClearText(event.args[1]);
        clearText = BigInt(clearLHS) * BigInt(clearRHS);
        clearText = clearText % 2n ** NumBits[resultType];
      }
      insertSQL(handle, clearText);
      break;

    case 'FheDiv':
      handle = ethers.toBeHex(event.args[3], 32);
      resultType = parseInt(handle.slice(-4, -2), 16);
      clearLHS = await getClearText(event.args[0]);
      if (event.args[2] === '0x01') {
        clearText = BigInt(clearLHS) / event.args[1];
      } else {
        throw new Error('Non-scalar div not implemented yet');
      }
      insertSQL(handle, clearText);
      break;

    case 'FheRem':
      handle = ethers.toBeHex(event.args[3], 32);
      resultType = parseInt(handle.slice(-4, -2), 16);
      clearLHS = await getClearText(event.args[0]);
      if (event.args[2] === '0x01') {
        clearText = BigInt(clearLHS) % event.args[1];
      } else {
        throw new Error('Non-scalar rem not implemented yet');
      }
      insertSQL(handle, clearText);
      break;

    case 'FheBitAnd':
      handle = ethers.toBeHex(event.args[3], 32);
      resultType = parseInt(handle.slice(-4, -2), 16);
      clearLHS = await getClearText(event.args[0]);
      if (event.args[2] === '0x01') {
        clearText = BigInt(clearLHS) & event.args[1];
        clearText = clearText % 2n ** NumBits[resultType];
      } else {
        clearRHS = await getClearText(event.args[1]);
        clearText = BigInt(clearLHS) & BigInt(clearRHS);
        clearText = clearText % 2n ** NumBits[resultType];
      }
      insertSQL(handle, clearText);
      break;

    case 'FheBitOr':
      handle = ethers.toBeHex(event.args[3], 32);
      resultType = parseInt(handle.slice(-4, -2), 16);
      clearLHS = await getClearText(event.args[0]);
      if (event.args[2] === '0x01') {
        clearText = BigInt(clearLHS) | event.args[1];
        clearText = clearText % 2n ** NumBits[resultType];
      } else {
        clearRHS = await getClearText(event.args[1]);
        clearText = BigInt(clearLHS) | BigInt(clearRHS);
        clearText = clearText % 2n ** NumBits[resultType];
      }
      insertSQL(handle, clearText);
      break;

    case 'FheBitXor':
      handle = ethers.toBeHex(event.args[3], 32);
      resultType = parseInt(handle.slice(-4, -2), 16);
      clearLHS = await getClearText(event.args[0]);
      if (event.args[2] === '0x01') {
        clearText = BigInt(clearLHS) ^ event.args[1];
        clearText = clearText % 2n ** NumBits[resultType];
      } else {
        clearRHS = await getClearText(event.args[1]);
        clearText = BigInt(clearLHS) ^ BigInt(clearRHS);
        clearText = clearText % 2n ** NumBits[resultType];
      }
      insertSQL(handle, clearText);
      break;

    case 'FheShl':
      handle = ethers.toBeHex(event.args[3], 32);
      resultType = parseInt(handle.slice(-4, -2), 16);
      clearLHS = await getClearText(event.args[0]);
      if (event.args[2] === '0x01') {
        clearText = BigInt(clearLHS) << event.args[1] % NumBits[resultType];
        clearText = clearText % 2n ** NumBits[resultType];
      } else {
        clearRHS = await getClearText(event.args[1]);
        clearText = BigInt(clearLHS) << BigInt(clearRHS) % NumBits[resultType];
        clearText = clearText % 2n ** NumBits[resultType];
      }
      insertSQL(handle, clearText);
      break;

    case 'FheShr':
      handle = ethers.toBeHex(event.args[3], 32);
      resultType = parseInt(handle.slice(-4, -2), 16);
      clearLHS = await getClearText(event.args[0]);
      if (event.args[2] === '0x01') {
        clearText = BigInt(clearLHS) >> event.args[1] % NumBits[resultType];
        clearText = clearText % 2n ** NumBits[resultType];
      } else {
        clearRHS = await getClearText(event.args[1]);
        clearText = BigInt(clearLHS) >> BigInt(clearRHS) % NumBits[resultType];
        clearText = clearText % 2n ** NumBits[resultType];
      }
      insertSQL(handle, clearText);
      break;

    case 'FheRotl':
      handle = ethers.toBeHex(event.args[3], 32);
      resultType = parseInt(handle.slice(-4, -2), 16);
      clearLHS = await getClearText(event.args[0]);
      if (event.args[2] === '0x01') {
        shift = event.args[1] % NumBits[resultType];
        clearText = (BigInt(clearLHS) << shift) | (BigInt(clearLHS) >> (NumBits[resultType] - shift));
        clearText = clearText % 2n ** NumBits[resultType];
      } else {
        clearRHS = await getClearText(event.args[1]);
        shift = BigInt(clearRHS) % NumBits[resultType];
        clearText = (BigInt(clearLHS) << shift) | (BigInt(clearLHS) >> (NumBits[resultType] - shift));
        clearText = clearText % 2n ** NumBits[resultType];
      }
      insertSQL(handle, clearText);
      break;

    case 'FheRotr':
      handle = ethers.toBeHex(event.args[3], 32);
      resultType = parseInt(handle.slice(-4, -2), 16);
      clearLHS = await getClearText(event.args[0]);
      if (event.args[2] === '0x01') {
        shift = event.args[1] % NumBits[resultType];
        clearText = (BigInt(clearLHS) >> shift) | (BigInt(clearLHS) << (NumBits[resultType] - shift));
        clearText = clearText % 2n ** NumBits[resultType];
      } else {
        clearRHS = await getClearText(event.args[1]);
        shift = BigInt(clearRHS) % NumBits[resultType];
        clearText = (BigInt(clearLHS) >> shift) | (BigInt(clearLHS) << (NumBits[resultType] - shift));
        clearText = clearText % 2n ** NumBits[resultType];
      }
      insertSQL(handle, clearText);
      break;

    case 'FheEq':
      handle = ethers.toBeHex(event.args[3], 32);
      resultType = parseInt(handle.slice(-4, -2), 16);
      clearLHS = await getClearText(event.args[0]);
      if (event.args[2] === '0x01') {
        clearText = BigInt(clearLHS) === event.args[1] ? 1n : 0n;
      } else {
        clearRHS = await getClearText(event.args[1]);
        clearText = BigInt(clearLHS) === BigInt(clearRHS) ? 1n : 0n;
      }
      insertSQL(handle, clearText);
      break;

    case 'FheEqBytes':
      handle = ethers.toBeHex(event.args[3], 32);
      resultType = parseInt(handle.slice(-4, -2), 16);
      clearLHS = await getClearText(event.args[0]);
      if (event.args[2] === '0x01') {
        clearText = BigInt(clearLHS) === BigInt(event.args[1]) ? 1n : 0n;
      } else {
        clearRHS = await getClearText(event.args[1]);
        clearText = BigInt(clearLHS) === BigInt(clearRHS) ? 1n : 0n;
      }
      insertSQL(handle, clearText);
      break;

    case 'FheNe':
      handle = ethers.toBeHex(event.args[3], 32);
      resultType = parseInt(handle.slice(-4, -2), 16);
      clearLHS = await getClearText(event.args[0]);
      if (event.args[2] === '0x01') {
        clearText = BigInt(clearLHS) !== event.args[1] ? 1n : 0n;
      } else {
        clearRHS = await getClearText(event.args[1]);
        clearText = BigInt(clearLHS) !== BigInt(clearRHS) ? 1n : 0n;
      }
      insertSQL(handle, clearText);
      break;

    case 'FheNeBytes':
      handle = ethers.toBeHex(event.args[3], 32);
      resultType = parseInt(handle.slice(-4, -2), 16);
      clearLHS = await getClearText(event.args[0]);
      if (event.args[2] === '0x01') {
        clearText = BigInt(clearLHS) !== BigInt(event.args[1]) ? 1n : 0n;
      } else {
        clearRHS = await getClearText(event.args[1]);
        clearText = BigInt(clearLHS) !== BigInt(clearRHS) ? 1n : 0n;
      }
      insertSQL(handle, clearText);
      break;

    case 'FheGe':
      handle = ethers.toBeHex(event.args[3], 32);
      resultType = parseInt(handle.slice(-4, -2), 16);
      clearLHS = await getClearText(event.args[0]);
      if (event.args[2] === '0x01') {
        clearText = BigInt(clearLHS) >= event.args[1] ? 1n : 0n;
      } else {
        clearRHS = await getClearText(event.args[1]);
        clearText = BigInt(clearLHS) >= BigInt(clearRHS) ? 1n : 0n;
      }
      insertSQL(handle, clearText);
      break;

    case 'FheGt':
      handle = ethers.toBeHex(event.args[3], 32);
      resultType = parseInt(handle.slice(-4, -2), 16);
      clearLHS = await getClearText(event.args[0]);
      if (event.args[2] === '0x01') {
        clearText = BigInt(clearLHS) > event.args[1] ? 1n : 0n;
      } else {
        clearRHS = await getClearText(event.args[1]);
        clearText = BigInt(clearLHS) > BigInt(clearRHS) ? 1n : 0n;
      }
      insertSQL(handle, clearText);
      break;

    case 'FheLe':
      handle = ethers.toBeHex(event.args[3], 32);
      resultType = parseInt(handle.slice(-4, -2), 16);
      clearLHS = await getClearText(event.args[0]);
      if (event.args[2] === '0x01') {
        clearText = BigInt(clearLHS) <= event.args[1] ? 1n : 0n;
      } else {
        clearRHS = await getClearText(event.args[1]);
        clearText = BigInt(clearLHS) <= BigInt(clearRHS) ? 1n : 0n;
      }
      insertSQL(handle, clearText);
      break;

    case 'FheLt':
      handle = ethers.toBeHex(event.args[3], 32);
      resultType = parseInt(handle.slice(-4, -2), 16);
      clearLHS = await getClearText(event.args[0]);
      if (event.args[2] === '0x01') {
        clearText = BigInt(clearLHS) < event.args[1] ? 1n : 0n;
      } else {
        clearRHS = await getClearText(event.args[1]);
        clearText = BigInt(clearLHS) < BigInt(clearRHS) ? 1n : 0n;
      }
      insertSQL(handle, clearText);
      break;

    case 'FheMax':
      handle = ethers.toBeHex(event.args[3], 32);
      resultType = parseInt(handle.slice(-4, -2), 16);
      clearLHS = await getClearText(event.args[0]);
      if (event.args[2] === '0x01') {
        clearText = BigInt(clearLHS) > event.args[1] ? clearLHS : event.args[1];
      } else {
        clearRHS = await getClearText(event.args[1]);
        clearText = BigInt(clearLHS) > BigInt(clearRHS) ? clearLHS : clearRHS;
      }
      insertSQL(handle, clearText);
      break;

    case 'FheMin':
      handle = ethers.toBeHex(event.args[3], 32);
      resultType = parseInt(handle.slice(-4, -2), 16);
      clearLHS = await getClearText(event.args[0]);
      if (event.args[2] === '0x01') {
        clearText = BigInt(clearLHS) < event.args[1] ? clearLHS : event.args[1];
      } else {
        clearRHS = await getClearText(event.args[1]);
        clearText = BigInt(clearLHS) < BigInt(clearRHS) ? clearLHS : clearRHS;
      }
      insertSQL(handle, clearText);
      break;

    case 'Cast':
      resultType = parseInt(event.args[1]);
      handle = ethers.toBeHex(event.args[2], 32);
      clearText = BigInt(await getClearText(event.args[0])) % 2n ** NumBits[resultType];
      insertSQL(handle, clearText);
      break;

    case 'FheNot':
      handle = ethers.toBeHex(event.args[1], 32);
      resultType = parseInt(handle.slice(-4, -2), 16);
      clearText = BigInt(await getClearText(event.args[0]));
      clearText = bitwiseNotUintBits(clearText, Number(NumBits[resultType]));
      insertSQL(handle, clearText);
      break;

    case 'FheNeg':
      handle = ethers.toBeHex(event.args[1], 32);
      resultType = parseInt(handle.slice(-4, -2), 16);
      clearText = BigInt(await getClearText(event.args[0]));
      clearText = bitwiseNotUintBits(clearText, Number(NumBits[resultType]));
      clearText = (clearText + 1n) % 2n ** NumBits[resultType];
      insertSQL(handle, clearText);
      break;

    case 'VerifyCiphertext':
      handle = event.args[0];
      try {
        await getClearText(BigInt(handle));
      } catch {
        throw Error('User input was not found in DB');
      }
      break;

    case 'FheIfThenElse':
      handle = ethers.toBeHex(event.args[3], 32);
      resultType = parseInt(handle.slice(-4, -2), 16);
      handle = ethers.toBeHex(event.args[3], 32);
      const clearControl = BigInt(await getClearText(event.args[0]));
      const clearIfTrue = BigInt(await getClearText(event.args[1]));
      const clearIfFalse = BigInt(await getClearText(event.args[2]));
      if (clearControl === 1n) {
        clearText = clearIfTrue;
      } else {
        clearText = clearIfFalse;
      }
      insertSQL(handle, clearText);
      break;

    case 'FheRand':
      resultType = parseInt(event.args[0], 16);
      handle = ethers.toBeHex(event.args[1], 32);
      clearText = getRandomBigInt(Number(NumBits[resultType]));
      insertSQL(handle, clearText, true);
      counterRand++;
      break;

    case 'FheRandBounded':
      resultType = parseInt(event.args[1], 16);
      handle = ethers.toBeHex(event.args[2], 32);
      clearText = getRandomBigInt(Number(log2(BigInt(event.args[0]))));
      insertSQL(handle, clearText, true);
      counterRand++;
      break;
  }
}

export function getFHEGasFromTxReceipt(receipt: ethers.TransactionReceipt): number {
  if (process.env.HARDHAT_TFHEEXECUTOR_EVENTS !== '1') {
    throw Error('FHEGas tracking is currently implemented only with TFHEExecutor.events.sol variant');
  }
  if (receipt.status === 0) {
    throw new Error('Transaction reverted');
  }
  const contract = new ethers.Contract(coprocAddress, abi, ethers.provider);
  const relevantLogs = receipt.logs.filter((log: ethers.Log) => {
    if (log.address.toLowerCase() !== coprocAddress.toLowerCase()) {
      return false;
    }
    try {
      const parsedLog = contract.interface.parseLog({
        topics: log.topics,
        data: log.data,
      });
      return abi.some((item) => item.startsWith(`event ${parsedLog.name}`) && parsedLog.name !== 'VerifyCiphertext');
    } catch {
      return false;
    }
  });
  const FHELogs = relevantLogs.map((log: ethers.Log) => {
    const parsedLog = contract.interface.parseLog({
      topics: log.topics,
      data: log.data,
    });
    return {
      name: parsedLog.name,
      args: parsedLog.args,
    };
  });
  let FHEGasConsumed = 0;
  for (const event of FHELogs) {
    let type;
    let handle;
    switch (event.name) {
      case 'TrivialEncrypt':
        type = parseInt(event.args[1], 16);
        FHEGasConsumed += operatorsPrices['trivialEncrypt'].types[type];
        break;

      case 'TrivialEncryptBytes':
        type = parseInt(event.args[1], 16);
        FHEGasConsumed += operatorsPrices['trivialEncrypt'].types[type];
        break;

      case 'FheAdd':
        handle = ethers.toBeHex(event.args[0], 32);
        type = parseInt(handle.slice(-4, -2), 16);
        if (event.args[2] === '0x01') {
          FHEGasConsumed += operatorsPrices['fheAdd'].scalar[type];
        } else {
          FHEGasConsumed += operatorsPrices['fheAdd'].nonScalar[type];
        }
        break;

      case 'FheSub':
        handle = ethers.toBeHex(event.args[0], 32);
        type = parseInt(handle.slice(-4, -2), 16);
        if (event.args[2] === '0x01') {
          FHEGasConsumed += operatorsPrices['fheSub'].scalar[type];
        } else {
          FHEGasConsumed += operatorsPrices['fheSub'].nonScalar[type];
        }
        break;

      case 'FheMul':
        handle = ethers.toBeHex(event.args[0], 32);
        type = parseInt(handle.slice(-4, -2), 16);
        if (event.args[2] === '0x01') {
          FHEGasConsumed += operatorsPrices['fheMul'].scalar[type];
        } else {
          FHEGasConsumed += operatorsPrices['fheMul'].nonScalar[type];
        }
        break;

      case 'FheDiv':
        handle = ethers.toBeHex(event.args[0], 32);
        type = parseInt(handle.slice(-4, -2), 16);
        if (event.args[2] === '0x01') {
          FHEGasConsumed += operatorsPrices['fheDiv'].scalar[type];
        } else {
          throw new Error('Non-scalar div not implemented yet');
        }
        break;

      case 'FheRem':
        handle = ethers.toBeHex(event.args[0], 32);
        type = parseInt(handle.slice(-4, -2), 16);
        if (event.args[2] === '0x01') {
          FHEGasConsumed += operatorsPrices['fheRem'].scalar[type];
        } else {
          throw new Error('Non-scalar rem not implemented yet');
        }
        break;

      case 'FheBitAnd':
        handle = ethers.toBeHex(event.args[0], 32);
        type = parseInt(handle.slice(-4, -2), 16);
        if (event.args[2] === '0x01') {
          FHEGasConsumed += operatorsPrices['fheBitAnd'].scalar[type];
        } else {
          FHEGasConsumed += operatorsPrices['fheBitAnd'].nonScalar[type];
        }
        break;

      case 'FheBitOr':
        handle = ethers.toBeHex(event.args[0], 32);
        type = parseInt(handle.slice(-4, -2), 16);
        if (event.args[2] === '0x01') {
          FHEGasConsumed += operatorsPrices['fheBitOr'].scalar[type];
        } else {
          FHEGasConsumed += operatorsPrices['fheBitOr'].nonScalar[type];
        }
        break;

      case 'FheBitXor':
        handle = ethers.toBeHex(event.args[0], 32);
        type = parseInt(handle.slice(-4, -2), 16);
        if (event.args[2] === '0x01') {
          FHEGasConsumed += operatorsPrices['fheBitXor'].scalar[type];
        } else {
          FHEGasConsumed += operatorsPrices['fheBitXor'].nonScalar[type];
        }
        break;

      case 'FheShl':
        handle = ethers.toBeHex(event.args[0], 32);
        type = parseInt(handle.slice(-4, -2), 16);
        if (event.args[2] === '0x01') {
          FHEGasConsumed += operatorsPrices['fheBitShl'].scalar[type];
        } else {
          FHEGasConsumed += operatorsPrices['fheBitShl'].nonScalar[type];
        }
        break;

      case 'FheShr':
        handle = ethers.toBeHex(event.args[0], 32);
        type = parseInt(handle.slice(-4, -2), 16);
        if (event.args[2] === '0x01') {
          FHEGasConsumed += operatorsPrices['fheBitShr'].scalar[type];
        } else {
          FHEGasConsumed += operatorsPrices['fheBitShr'].nonScalar[type];
        }
        break;

      case 'FheRotl':
        handle = ethers.toBeHex(event.args[0], 32);
        type = parseInt(handle.slice(-4, -2), 16);
        if (event.args[2] === '0x01') {
          FHEGasConsumed += operatorsPrices['fheRotl'].scalar[type];
        } else {
          FHEGasConsumed += operatorsPrices['fheRotl'].nonScalar[type];
        }
        break;

      case 'FheRotr':
        handle = ethers.toBeHex(event.args[0], 32);
        type = parseInt(handle.slice(-4, -2), 16);
        if (event.args[2] === '0x01') {
          FHEGasConsumed += operatorsPrices['fheRotr'].scalar[type];
        } else {
          FHEGasConsumed += operatorsPrices['fheRotr'].nonScalar[type];
        }
        break;

      case 'FheEq':
        handle = ethers.toBeHex(event.args[0], 32);
        type = parseInt(handle.slice(-4, -2), 16);
        if (event.args[2] === '0x01') {
          FHEGasConsumed += operatorsPrices['fheEq'].scalar[type];
        } else {
          FHEGasConsumed += operatorsPrices['fheEq'].nonScalar[type];
        }
        break;

      case 'FheEqBytes':
        handle = ethers.toBeHex(event.args[0], 32);
        type = parseInt(handle.slice(-4, -2), 16);
        if (event.args[2] === '0x01') {
          FHEGasConsumed += operatorsPrices['fheEq'].scalar[type];
        } else {
          FHEGasConsumed += operatorsPrices['fheEq'].nonScalar[type];
        }

      case 'FheNe':
        handle = ethers.toBeHex(event.args[0], 32);
        type = parseInt(handle.slice(-4, -2), 16);
        if (event.args[2] === '0x01') {
          FHEGasConsumed += operatorsPrices['fheNe'].scalar[type];
        } else {
          FHEGasConsumed += operatorsPrices['fheNe'].nonScalar[type];
        }
        break;

      case 'FheNeBytes':
        handle = ethers.toBeHex(event.args[0], 32);
        type = parseInt(handle.slice(-4, -2), 16);
        if (event.args[2] === '0x01') {
          FHEGasConsumed += operatorsPrices['fheNe'].scalar[type];
        } else {
          FHEGasConsumed += operatorsPrices['fheNe'].nonScalar[type];
        }
        break;

      case 'FheGe':
        handle = ethers.toBeHex(event.args[0], 32);
        type = parseInt(handle.slice(-4, -2), 16);
        if (event.args[2] === '0x01') {
          FHEGasConsumed += operatorsPrices['fheGe'].scalar[type];
        } else {
          FHEGasConsumed += operatorsPrices['fheGe'].nonScalar[type];
        }
        break;

      case 'FheGt':
        handle = ethers.toBeHex(event.args[0], 32);
        type = parseInt(handle.slice(-4, -2), 16);
        if (event.args[2] === '0x01') {
          FHEGasConsumed += operatorsPrices['fheGt'].scalar[type];
        } else {
          FHEGasConsumed += operatorsPrices['fheGt'].nonScalar[type];
        }
        break;

      case 'FheLe':
        handle = ethers.toBeHex(event.args[0], 32);
        type = parseInt(handle.slice(-4, -2), 16);
        if (event.args[2] === '0x01') {
          FHEGasConsumed += operatorsPrices['fheLe'].scalar[type];
        } else {
          FHEGasConsumed += operatorsPrices['fheLe'].nonScalar[type];
        }
        break;

      case 'FheLt':
        handle = ethers.toBeHex(event.args[0], 32);
        type = parseInt(handle.slice(-4, -2), 16);
        if (event.args[2] === '0x01') {
          FHEGasConsumed += operatorsPrices['fheLt'].scalar[type];
        } else {
          FHEGasConsumed += operatorsPrices['fheLt'].nonScalar[type];
        }
        break;

      case 'FheMax':
        handle = ethers.toBeHex(event.args[0], 32);
        type = parseInt(handle.slice(-4, -2), 16);
        if (event.args[2] === '0x01') {
          FHEGasConsumed += operatorsPrices['fheMax'].scalar[type];
        } else {
          FHEGasConsumed += operatorsPrices['fheMax'].nonScalar[type];
        }
        break;

      case 'FheMin':
        handle = ethers.toBeHex(event.args[0], 32);
        type = parseInt(handle.slice(-4, -2), 16);
        if (event.args[2] === '0x01') {
          FHEGasConsumed += operatorsPrices['fheMin'].scalar[type];
        } else {
          FHEGasConsumed += operatorsPrices['fheMin'].nonScalar[type];
        }
        break;

      case 'Cast':
        handle = ethers.toBeHex(event.args[0], 32);
        type = parseInt(handle.slice(-4, -2), 16);
        FHEGasConsumed += operatorsPrices['cast'].types[type];
        break;

      case 'FheNot':
        handle = ethers.toBeHex(event.args[0], 32);
        type = parseInt(handle.slice(-4, -2), 16);
        FHEGasConsumed += operatorsPrices['fheNot'].types[type];
        break;

      case 'FheNeg':
        handle = ethers.toBeHex(event.args[0], 32);
        type = parseInt(handle.slice(-4, -2), 16);
        FHEGasConsumed += operatorsPrices['fheNeg'].types[type];
        break;

      case 'FheIfThenElse':
        handle = ethers.toBeHex(event.args[3], 32);
        type = parseInt(handle.slice(-4, -2), 16);
        FHEGasConsumed += operatorsPrices['ifThenElse'].types[type];
        break;

      case 'FheRand':
        type = parseInt(event.args[0], 16);
        FHEGasConsumed += operatorsPrices['fheRand'].types[type];
        break;

      case 'FheRandBounded':
        type = parseInt(event.args[1], 16);
        FHEGasConsumed += operatorsPrices['fheRandBounded'].types[type];
        break;
    }
  }
  return FHEGasConsumed;
}
