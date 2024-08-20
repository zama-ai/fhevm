import dotenv from 'dotenv';
import { log2 } from 'extra-bigint';
import * as fs from 'fs';
import { ethers } from 'hardhat';
import hre from 'hardhat';
import { Database } from 'sqlite3';

const parsedEnvCoprocessor = dotenv.parse(fs.readFileSync('lib/.env.exec'));
const coprocAdd = parsedEnvCoprocessor.TFHE_EXECUTOR_CONTRACT_ADDRESS.replace(/^0x/, '')
  .replace(/^0+/, '')
  .toLowerCase();

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

function insertSQL(handle: string, clearText: BigInt, replace: boolean = false) {
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
    const maxRetries = 10;

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

const TypesBytesSize = {
  0: 1, //ebool
  1: 1, //euint4
  2: 1, //euint8
  3: 2, //euint16
  4: 4, //euint32
  5: 8, //euint64
  6: 16, //euint128
  7: 20, //eaddress
  8: 32, //euint256
  9: 64, //ebytes64
  10: 128, //ebytes128
  11: 256, //ebytes256
};

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

async function insertHandle(obj2: EvmState, validIdxes: [number]) {
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
            ['uint8', 'uint256', 'bytes1'],
            [Operators.trivialEncrypt, decodedData[0], decodedData[1]],
          ),
        );
        handle = appendType(handle, resultType);
        insertSQL(handle, clearText);
        break;

      case 'fheAdd(uint256,uint256,bytes1)':
        handle = ethers.keccak256(
          ethers.solidityPacked(
            ['uint8', 'uint256', 'uint256', 'bytes1'],
            [Operators.fheAdd, decodedData[0], decodedData[1], decodedData[2]],
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
            ['uint8', 'uint256', 'uint256', 'bytes1'],
            [Operators.fheSub, decodedData[0], decodedData[1], decodedData[2]],
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
            ['uint8', 'uint256', 'uint256', 'bytes1'],
            [Operators.fheMul, decodedData[0], decodedData[1], decodedData[2]],
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
            ['uint8', 'uint256', 'uint256', 'bytes1'],
            [Operators.fheDiv, decodedData[0], decodedData[1], decodedData[2]],
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
            ['uint8', 'uint256', 'uint256', 'bytes1'],
            [Operators.fheRem, decodedData[0], decodedData[1], decodedData[2]],
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
            ['uint8', 'uint256', 'uint256', 'bytes1'],
            [Operators.fheBitAnd, decodedData[0], decodedData[1], decodedData[2]],
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
            ['uint8', 'uint256', 'uint256', 'bytes1'],
            [Operators.fheBitOr, decodedData[0], decodedData[1], decodedData[2]],
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
            ['uint8', 'uint256', 'uint256', 'bytes1'],
            [Operators.fheBitXor, decodedData[0], decodedData[1], decodedData[2]],
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
            ['uint8', 'uint256', 'uint256', 'bytes1'],
            [Operators.fheShl, decodedData[0], decodedData[1], decodedData[2]],
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
            ['uint8', 'uint256', 'uint256', 'bytes1'],
            [Operators.fheShr, decodedData[0], decodedData[1], decodedData[2]],
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
            ['uint8', 'uint256', 'uint256', 'bytes1'],
            [Operators.fheRotl, decodedData[0], decodedData[1], decodedData[2]],
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
            ['uint8', 'uint256', 'uint256', 'bytes1'],
            [Operators.fheRotr, decodedData[0], decodedData[1], decodedData[2]],
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
            ['uint8', 'uint256', 'uint256', 'bytes1'],
            [Operators.fheEq, decodedData[0], decodedData[1], decodedData[2]],
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

      case 'fheNe(uint256,uint256,bytes1)':
        handle = ethers.keccak256(
          ethers.solidityPacked(
            ['uint8', 'uint256', 'uint256', 'bytes1'],
            [Operators.fheNe, decodedData[0], decodedData[1], decodedData[2]],
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

      case 'fheGe(uint256,uint256,bytes1)':
        handle = ethers.keccak256(
          ethers.solidityPacked(
            ['uint8', 'uint256', 'uint256', 'bytes1'],
            [Operators.fheGe, decodedData[0], decodedData[1], decodedData[2]],
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
            ['uint8', 'uint256', 'uint256', 'bytes1'],
            [Operators.fheGt, decodedData[0], decodedData[1], decodedData[2]],
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
            ['uint8', 'uint256', 'uint256', 'bytes1'],
            [Operators.fheLe, decodedData[0], decodedData[1], decodedData[2]],
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
            ['uint8', 'uint256', 'uint256', 'bytes1'],
            [Operators.fheLt, decodedData[0], decodedData[1], decodedData[2]],
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
            ['uint8', 'uint256', 'uint256', 'bytes1'],
            [Operators.fheMax, decodedData[0], decodedData[1], decodedData[2]],
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
            ['uint8', 'uint256', 'uint256', 'bytes1'],
            [Operators.fheMin, decodedData[0], decodedData[1], decodedData[2]],
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
          ethers.solidityPacked(['uint8', 'uint256', 'bytes1'], [Operators.cast, decodedData[0], decodedData[1]]),
        );
        clearText = BigInt(await getClearText(decodedData[0])) % 2n ** NumBits[resultType];
        handle = appendType(handle, resultType);
        insertSQL(handle, clearText);
        break;

      case 'fheNot(uint256)':
        resultType = parseInt(decodedData[0].toString(16).slice(-4, -2), 16);
        handle = ethers.keccak256(ethers.solidityPacked(['uint8', 'uint256'], [Operators.fheNot, decodedData[0]]));
        handle = appendType(handle, resultType);
        clearText = BigInt(await getClearText(decodedData[0]));
        clearText = bitwiseNotUintBits(clearText, Number(NumBits[resultType]));
        insertSQL(handle, clearText);
        break;

      case 'fheNeg(uint256)':
        resultType = parseInt(decodedData[0].toString(16).slice(-4, -2), 16);
        handle = ethers.keccak256(ethers.solidityPacked(['uint8', 'uint256'], [Operators.fheNeg, decodedData[0]]));
        handle = appendType(handle, resultType);
        clearText = BigInt(await getClearText(decodedData[0]));
        clearText = bitwiseNotUintBits(clearText, Number(NumBits[resultType]));
        clearText = (clearText + 1n) % 2n ** NumBits[resultType];
        insertSQL(handle, clearText);
        break;

      case 'verifyCiphertext(bytes32,address,bytes,bytes1)':
        handle = decodedData[0];
        const type = parseInt(handle.slice(-4, -2), 16);
        if (type !== 11) {
          //not an ebytes256
          const typeSize = TypesBytesSize[type];
          const idx = parseInt(handle.slice(-6, -4), 16);
          const inputProof = decodedData[2].replace(/^0x/, '');
          clearText = BigInt('0x' + inputProof.slice(2 + 2 * 53 * idx, 2 + 2 * typeSize + 2 * 53 * idx));
          insertSQL(handle, clearText);
        } else {
          const inputProof = decodedData[2].replace(/^0x/, '');
          clearText = BigInt('0x' + inputProof.slice(2, 2 + 2 * 256));
          insertSQL(handle, clearText);
        }
        break;

      case 'fheIfThenElse(uint256,uint256,uint256)':
        resultType = parseInt(decodedData[1].toString(16).slice(-4, -2), 16);
        handle = ethers.keccak256(
          ethers.solidityPacked(
            ['uint8', 'uint256', 'uint256', 'uint256'],
            [Operators.fheIfThenElse, decodedData[0], decodedData[1], decodedData[2]],
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
          handle = ethers.keccak256(
            ethers.solidityPacked(['uint8', 'bytes1', 'uint256'], [Operators.fheRand, decodedData[0], counterRand]),
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
          handle = ethers.keccak256(
            ethers.solidityPacked(
              ['uint8', 'uint256', 'bytes1', 'uint256'],
              [Operators.fheRandBounded, decodedData[0], decodedData[1], counterRand],
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

async function processLogs(trace, validSubcallsIndexes) {
  for (const obj of trace.structLogs
    .map((value, index) => ({ value, index }))
    .filter((obj) => obj.value.op === 'CALL')) {
    await insertHandle(obj, validSubcallsIndexes);
  }
}

export const awaitCoprocessor = async (): Promise<void> => {
  const pastTxHashes = await getAllPastTransactionHashes();
  for (const txHash of pastTxHashes) {
    const trace = await ethers.provider.send('debug_traceTransaction', [txHash[0]]);

    if (!trace.failed) {
      const callTree = await buildCallTree(trace, txHash[1]);
      const validSubcallsIndexes = getValidSubcallsIds(callTree)[1];
      await processLogs(trace, validSubcallsIndexes);
    }
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
      txHashes.push([tx, { to: rcpt.to, status: rcpt.status }]);
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
