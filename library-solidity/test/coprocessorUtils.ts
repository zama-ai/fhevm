import dotenv from 'dotenv';
import { log2 } from 'extra-bigint';
import * as fs from 'fs';
import { ethers } from 'hardhat';
import { Database } from 'sqlite3';

import { FheType } from '../codegen/common';
import operatorsPrices from '../codegen/operatorsPrices.json';
import { ALL_FHE_TYPES } from '../codegen/types';

const parsedEnvCoprocessor = dotenv.parse(fs.readFileSync('./fhevmTemp/addresses/.env.host'));
const coprocAddress = parsedEnvCoprocessor.FHEVM_EXECUTOR_CONTRACT_ADDRESS;

let firstBlockListening = 0;
let lastBlockSnapshot = 0;
let lastCounterRand = 0;
let counterRand = 0;
let chainId: number;

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
export const getClearText = async (handle: string): Promise<string> => {
  return new Promise((resolve, reject) => {
    let attempts = 0;
    const maxRetries = 100;

    function executeQuery() {
      db.get('SELECT clearText FROM ciphertexts WHERE handle = ?', [handle], (err, row) => {
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

interface FHEVMEvent {
  eventName: string;
  args: object;
}

const NumBits = {
  0: 1n, //ebool
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

export const awaitCoprocessor = async (): Promise<void> => {
  chainId = (await ethers.provider.getNetwork()).chainId;
  await processAllPastFHEVMExecutorEvents();
};

const abi = [
  'event FheAdd(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result)',
  'event FheSub(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result)',
  'event FheMul(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result)',
  'event FheDiv(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result)',
  'event FheRem(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result)',
  'event FheBitAnd(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result)',
  'event FheBitOr(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result)',
  'event FheBitXor(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result)',
  'event FheShl(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result)',
  'event FheShr(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result)',
  'event FheRotl(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result)',
  'event FheRotr(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result)',
  'event FheEq(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result)',
  'event FheEqBytes(address indexed caller, bytes32 lhs, bytes rhs, bytes1 scalarByte, bytes32 result)',
  'event FheNe(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result)',
  'event FheNeBytes(address indexed caller, bytes32 lhs, bytes rhs, bytes1 scalarByte, bytes32 result)',
  'event FheGe(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result)',
  'event FheGt(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result)',
  'event FheLe(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result)',
  'event FheLt(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result)',
  'event FheMin(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result)',
  'event FheMax(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result)',
  'event FheNeg(address indexed caller, bytes32 ct, bytes32 result)',
  'event FheNot(address indexed caller, bytes32 ct, bytes32 result)',
  'event VerifyCiphertext(address indexed caller, bytes32 inputHandle, address userAddress, bytes inputProof, uint8 inputType, bytes32 result)',
  'event Cast(address indexed caller, bytes32 ct, uint8 toType, bytes32 result)',
  'event TrivialEncrypt(address indexed caller, uint256 pt, uint8 toType, bytes32 result)',
  'event TrivialEncryptBytes(address indexed caller, bytes pt, uint8 toType, bytes32 result)',
  'event FheIfThenElse(address indexed caller, bytes32 control, bytes32 ifTrue, bytes32 ifFalse, bytes32 result)',
  'event FheRand(address indexed caller, uint8 randType, bytes16 seed, bytes32 result)',
  'event FheRandBounded(address indexed caller, uint256 upperBound, uint8 randType, bytes16 seed, bytes32 result)',
];

async function processAllPastFHEVMExecutorEvents() {
  const provider = ethers.provider;
  const latestBlockNumber = await provider.getBlockNumber();

  if (process.env.SOLIDITY_COVERAGE !== 'true') {
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
  if (process.env.SOLIDITY_COVERAGE !== 'true') {
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
      clearText = event.args[1];
      handle = ethers.toBeHex(event.args[3], 32);
      insertSQL(handle, clearText);
      break;

    case 'TrivialEncryptBytes':
      clearText = event.args[1];
      handle = ethers.toBeHex(event.args[3], 32);
      insertSQL(handle, BigInt(clearText));
      break;

    case 'FheAdd':
      handle = ethers.toBeHex(event.args[4], 32);
      resultType = parseInt(handle.slice(-4, -2), 16);
      clearLHS = await getClearText(event.args[1]);
      if (event.args[3] === '0x01') {
        clearText = BigInt(clearLHS) + BigInt(event.args[2]);
        clearText = clearText % 2n ** NumBits[resultType];
      } else {
        clearRHS = await getClearText(event.args[2]);
        clearText = BigInt(clearLHS) + BigInt(clearRHS);
        clearText = clearText % 2n ** NumBits[resultType];
      }

      insertSQL(ethers.toBeHex(handle, 32), clearText);
      break;

    case 'FheSub':
      handle = ethers.toBeHex(event.args[4], 32);
      resultType = parseInt(handle.slice(-4, -2), 16);
      clearLHS = await getClearText(event.args[1]);
      if (event.args[3] === '0x01') {
        clearText = BigInt(clearLHS) - BigInt(event.args[2]);
        if (clearText < 0n) clearText = clearText + 2n ** NumBits[resultType];
        clearText = clearText % 2n ** NumBits[resultType];
      } else {
        clearRHS = await getClearText(event.args[2]);
        clearText = BigInt(clearLHS) - BigInt(clearRHS);
        if (clearText < 0n) clearText = clearText + 2n ** NumBits[resultType];
        clearText = clearText % 2n ** NumBits[resultType];
      }
      insertSQL(handle, clearText);
      break;

    case 'FheMul':
      handle = ethers.toBeHex(event.args[4], 32);
      resultType = parseInt(handle.slice(-4, -2), 16);
      clearLHS = await getClearText(event.args[1]);
      if (event.args[3] === '0x01') {
        clearText = BigInt(clearLHS) * BigInt(event.args[2]);
        clearText = clearText % 2n ** NumBits[resultType];
      } else {
        clearRHS = await getClearText(event.args[2]);
        clearText = BigInt(clearLHS) * BigInt(clearRHS);
        clearText = clearText % 2n ** NumBits[resultType];
      }
      insertSQL(handle, clearText);
      break;

    case 'FheDiv':
      handle = ethers.toBeHex(event.args[4], 32);
      resultType = parseInt(handle.slice(-4, -2), 16);
      clearLHS = await getClearText(event.args[1]);
      if (event.args[3] === '0x01') {
        clearText = BigInt(clearLHS) / BigInt(event.args[2]);
      } else {
        throw new Error('Non-scalar div not implemented yet');
      }
      insertSQL(handle, clearText);
      break;

    case 'FheRem':
      handle = ethers.toBeHex(event.args[4], 32);
      resultType = parseInt(handle.slice(-4, -2), 16);
      clearLHS = await getClearText(event.args[1]);
      if (event.args[3] === '0x01') {
        clearText = BigInt(clearLHS) % BigInt(event.args[2]);
      } else {
        throw new Error('Non-scalar rem not implemented yet');
      }
      insertSQL(handle, clearText);
      break;

    case 'FheBitAnd':
      handle = ethers.toBeHex(event.args[4], 32);
      resultType = parseInt(handle.slice(-4, -2), 16);
      clearLHS = await getClearText(event.args[1]);
      if (event.args[3] === '0x01') {
        clearText = BigInt(clearLHS) & BigInt(event.args[2]);
        clearText = clearText % 2n ** NumBits[resultType];
      } else {
        clearRHS = await getClearText(event.args[2]);
        clearText = BigInt(clearLHS) & BigInt(clearRHS);
        clearText = clearText % 2n ** NumBits[resultType];
      }
      insertSQL(handle, clearText);
      break;

    case 'FheBitOr':
      handle = ethers.toBeHex(event.args[4], 32);
      resultType = parseInt(handle.slice(-4, -2), 16);
      clearLHS = await getClearText(event.args[1]);
      if (event.args[3] === '0x01') {
        clearText = BigInt(clearLHS) | BigInt(event.args[2]);
        clearText = clearText % 2n ** NumBits[resultType];
      } else {
        clearRHS = await getClearText(event.args[2]);
        clearText = BigInt(clearLHS) | BigInt(clearRHS);
        clearText = clearText % 2n ** NumBits[resultType];
      }
      insertSQL(handle, clearText);
      break;

    case 'FheBitXor':
      handle = ethers.toBeHex(event.args[4], 32);
      resultType = parseInt(handle.slice(-4, -2), 16);
      clearLHS = await getClearText(event.args[1]);
      if (event.args[3] === '0x01') {
        clearText = BigInt(clearLHS) ^ BigInt(event.args[2]);
        clearText = clearText % 2n ** NumBits[resultType];
      } else {
        clearRHS = await getClearText(event.args[2]);
        clearText = BigInt(clearLHS) ^ BigInt(clearRHS);
        clearText = clearText % 2n ** NumBits[resultType];
      }
      insertSQL(handle, clearText);
      break;

    case 'FheShl':
      handle = ethers.toBeHex(event.args[4], 32);
      resultType = parseInt(handle.slice(-4, -2), 16);
      clearLHS = await getClearText(event.args[1]);
      if (event.args[3] === '0x01') {
        clearText = BigInt(clearLHS) << BigInt(event.args[2]) % NumBits[resultType];
        clearText = clearText % 2n ** NumBits[resultType];
      } else {
        clearRHS = await getClearText(event.args[2]);
        clearText = BigInt(clearLHS) << BigInt(clearRHS) % NumBits[resultType];
        clearText = clearText % 2n ** NumBits[resultType];
      }
      insertSQL(handle, clearText);
      break;

    case 'FheShr':
      handle = ethers.toBeHex(event.args[4], 32);
      resultType = parseInt(handle.slice(-4, -2), 16);
      clearLHS = await getClearText(event.args[1]);
      if (event.args[3] === '0x01') {
        clearText = BigInt(clearLHS) >> BigInt(event.args[2]) % NumBits[resultType];
        clearText = clearText % 2n ** NumBits[resultType];
      } else {
        clearRHS = await getClearText(event.args[2]);
        clearText = BigInt(clearLHS) >> BigInt(clearRHS) % NumBits[resultType];
        clearText = clearText % 2n ** NumBits[resultType];
      }
      insertSQL(handle, clearText);
      break;

    case 'FheRotl':
      handle = ethers.toBeHex(event.args[4], 32);
      resultType = parseInt(handle.slice(-4, -2), 16);
      clearLHS = await getClearText(event.args[1]);
      if (event.args[3] === '0x01') {
        shift = BigInt(event.args[2]) % NumBits[resultType];
        clearText = (BigInt(clearLHS) << shift) | (BigInt(clearLHS) >> (NumBits[resultType] - shift));
        clearText = clearText % 2n ** NumBits[resultType];
      } else {
        clearRHS = await getClearText(event.args[2]);
        shift = BigInt(clearRHS) % NumBits[resultType];
        clearText = (BigInt(clearLHS) << shift) | (BigInt(clearLHS) >> (NumBits[resultType] - shift));
        clearText = clearText % 2n ** NumBits[resultType];
      }
      insertSQL(handle, clearText);
      break;

    case 'FheRotr':
      handle = ethers.toBeHex(event.args[4], 32);
      resultType = parseInt(handle.slice(-4, -2), 16);
      clearLHS = await getClearText(event.args[1]);
      if (event.args[3] === '0x01') {
        shift = BigInt(event.args[2]) % NumBits[resultType];
        clearText = (BigInt(clearLHS) >> shift) | (BigInt(clearLHS) << (NumBits[resultType] - shift));
        clearText = clearText % 2n ** NumBits[resultType];
      } else {
        clearRHS = await getClearText(event.args[2]);
        shift = BigInt(clearRHS) % NumBits[resultType];
        clearText = (BigInt(clearLHS) >> shift) | (BigInt(clearLHS) << (NumBits[resultType] - shift));
        clearText = clearText % 2n ** NumBits[resultType];
      }
      insertSQL(handle, clearText);
      break;

    case 'FheEq':
      handle = ethers.toBeHex(event.args[4], 32);
      resultType = parseInt(handle.slice(-4, -2), 16);
      clearLHS = await getClearText(event.args[1]);

      if (event.args[3] === '0x01') {
        clearText = BigInt(clearLHS) === BigInt(event.args[2]) ? 1n : 0n;
      } else {
        clearRHS = await getClearText(event.args[2]);
        clearText = clearLHS === clearRHS ? 1n : 0n;
      }

      insertSQL(handle, clearText);
      break;

    case 'FheEqBytes':
      handle = ethers.toBeHex(event.args[4], 32);
      resultType = parseInt(handle.slice(-4, -2), 16);
      clearLHS = await getClearText(event.args[1]);
      if (event.args[3] === '0x01') {
        clearText = BigInt(clearLHS) === BigInt(event.args[2]) ? 1n : 0n;
      } else {
        clearRHS = await getClearText(event.args[2]);
        clearText = BigInt(clearLHS) === BigInt(clearRHS) ? 1n : 0n;
      }
      insertSQL(handle, clearText);
      break;

    case 'FheNe':
      handle = ethers.toBeHex(event.args[4], 32);
      resultType = parseInt(handle.slice(-4, -2), 16);
      clearLHS = await getClearText(event.args[1]);
      if (event.args[3] === '0x01') {
        clearText = BigInt(clearLHS) !== BigInt(event.args[2]) ? 1n : 0n;
      } else {
        clearRHS = await getClearText(event.args[2]);
        clearText = BigInt(clearLHS) !== BigInt(clearRHS) ? 1n : 0n;
      }
      insertSQL(handle, clearText);
      break;

    case 'FheNeBytes':
      handle = ethers.toBeHex(event.args[4], 32);
      resultType = parseInt(handle.slice(-4, -2), 16);
      clearLHS = await getClearText(event.args[1]);
      if (event.args[3] === '0x01') {
        clearText = BigInt(clearLHS) !== BigInt(event.args[2]) ? 1n : 0n;
      } else {
        clearRHS = await getClearText(event.args[2]);
        clearText = BigInt(clearLHS) !== BigInt(clearRHS) ? 1n : 0n;
      }
      insertSQL(handle, clearText);
      break;

    case 'FheGe':
      handle = ethers.toBeHex(event.args[4], 32);
      resultType = parseInt(handle.slice(-4, -2), 16);
      clearLHS = await getClearText(event.args[1]);
      if (event.args[3] === '0x01') {
        clearText = BigInt(clearLHS) >= BigInt(event.args[2]) ? 1n : 0n;
      } else {
        clearRHS = await getClearText(event.args[2]);
        clearText = BigInt(clearLHS) >= BigInt(clearRHS) ? 1n : 0n;
      }
      insertSQL(handle, clearText);
      break;

    case 'FheGt':
      handle = ethers.toBeHex(event.args[4], 32);
      resultType = parseInt(handle.slice(-4, -2), 16);
      clearLHS = await getClearText(event.args[1]);
      if (event.args[3] === '0x01') {
        clearText = BigInt(clearLHS) > BigInt(event.args[2]) ? 1n : 0n;
      } else {
        clearRHS = await getClearText(event.args[2]);
        clearText = BigInt(clearLHS) > BigInt(clearRHS) ? 1n : 0n;
      }
      insertSQL(handle, clearText);
      break;

    case 'FheLe':
      handle = ethers.toBeHex(event.args[4], 32);
      resultType = parseInt(handle.slice(-4, -2), 16);
      clearLHS = await getClearText(event.args[1]);
      if (event.args[3] === '0x01') {
        clearText = BigInt(clearLHS) <= BigInt(event.args[2]) ? 1n : 0n;
      } else {
        clearRHS = await getClearText(event.args[2]);
        clearText = BigInt(clearLHS) <= BigInt(clearRHS) ? 1n : 0n;
      }
      insertSQL(handle, clearText);
      break;

    case 'FheLt':
      handle = ethers.toBeHex(event.args[4], 32);
      resultType = parseInt(handle.slice(-4, -2), 16);
      clearLHS = await getClearText(event.args[1]);
      if (event.args[3] === '0x01') {
        clearText = BigInt(clearLHS) < BigInt(event.args[2]) ? 1n : 0n;
      } else {
        clearRHS = await getClearText(event.args[2]);
        clearText = BigInt(clearLHS) < BigInt(clearRHS) ? 1n : 0n;
      }
      insertSQL(handle, clearText);
      break;

    case 'FheMax':
      handle = ethers.toBeHex(event.args[4], 32);
      resultType = parseInt(handle.slice(-4, -2), 16);
      clearLHS = await getClearText(event.args[1]);
      if (event.args[3] === '0x01') {
        clearText = BigInt(clearLHS) > BigInt(event.args[2]) ? clearLHS : BigInt(event.args[2]);
      } else {
        clearRHS = await getClearText(event.args[2]);
        clearText = BigInt(clearLHS) > BigInt(clearRHS) ? clearLHS : clearRHS;
      }
      insertSQL(handle, clearText);
      break;

    case 'FheMin':
      handle = ethers.toBeHex(event.args[4], 32);
      resultType = parseInt(handle.slice(-4, -2), 16);
      clearLHS = await getClearText(event.args[1]);
      if (event.args[3] === '0x01') {
        clearText = BigInt(clearLHS) < BigInt(event.args[2]) ? clearLHS : BigInt(event.args[2]);
      } else {
        clearRHS = await getClearText(event.args[2]);
        clearText = BigInt(clearLHS) < BigInt(clearRHS) ? clearLHS : clearRHS;
      }
      insertSQL(handle, clearText);
      break;

    case 'Cast':
      resultType = parseInt(event.args[2]);
      handle = ethers.toBeHex(event.args[3], 32);
      clearText = BigInt(await getClearText(event.args[1])) % 2n ** NumBits[resultType];
      insertSQL(handle, clearText);
      break;

    case 'FheNot':
      handle = ethers.toBeHex(event.args[2], 32);
      resultType = parseInt(handle.slice(-4, -2), 16);
      clearText = BigInt(await getClearText(event.args[1]));
      clearText = bitwiseNotUintBits(clearText, Number(NumBits[resultType]));
      insertSQL(handle, clearText);
      break;

    case 'FheNeg':
      handle = ethers.toBeHex(event.args[2], 32);
      resultType = parseInt(handle.slice(-4, -2), 16);
      clearText = BigInt(await getClearText(event.args[1]));
      clearText = bitwiseNotUintBits(clearText, Number(NumBits[resultType]));
      clearText = (clearText + 1n) % 2n ** NumBits[resultType];
      insertSQL(handle, clearText);
      break;

    case 'VerifyCiphertext':
      handle = event.args[1];
      try {
        await getClearText(BigInt(handle));
      } catch {
        throw Error('User input was not found in DB');
      }
      break;

    case 'FheIfThenElse':
      handle = ethers.toBeHex(event.args[4], 32);
      resultType = parseInt(handle.slice(-4, -2), 16);
      handle = ethers.toBeHex(event.args[4], 32);
      const clearControl = BigInt(await getClearText(event.args[1]));
      const clearIfTrue = BigInt(await getClearText(event.args[2]));
      const clearIfFalse = BigInt(await getClearText(event.args[3]));
      if (clearControl === 1n) {
        clearText = clearIfTrue;
      } else {
        clearText = clearIfFalse;
      }
      insertSQL(handle, clearText);
      break;

    case 'FheRand':
      resultType = parseInt(event.args[1]);
      handle = ethers.toBeHex(event.args[3], 32);
      clearText = getRandomBigInt(Number(NumBits[resultType]));
      insertSQL(handle, clearText, true);
      counterRand++;
      break;

    case 'FheRandBounded':
      resultType = parseInt(event.args[2]);
      handle = ethers.toBeHex(event.args[4], 32);
      clearText = getRandomBigInt(Number(log2(BigInt(event.args[1]))));
      insertSQL(handle, clearText, true);
      counterRand++;
      break;
  }
}

export function getTxHCUFromTxReceipt(
  receipt: ethers.TransactionReceipt,
  FheTypes: FheType[] = ALL_FHE_TYPES,
): {
  globalTxHCU: number;
  maxTxHCUDepth: number;
  HCUDepthPerHandle: Record<string, number>;
} {
  if (receipt.status === 0) {
    throw new Error('Transaction reverted');
  }

  function readFromHCUMap(handle: string): number {
    if (hcuMap[handle] === undefined) {
      return 0;
    }
    return hcuMap[handle];
  }

  let hcuMap: Record<string, number> = {};
  let handleSet: Set<string> = new Set();

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

  let totalHCUConsumed = 0;

  for (const event of FHELogs) {
    let type: string | undefined;
    let typeIndex: number;
    let handle: string;
    let handleResult: string;
    let hcuConsumed: number;

    switch (event.name) {
      case 'TrivialEncrypt':
        typeIndex = parseInt(event.args[2]);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;
        if (!type) {
          throw new Error(`Invalid FheType index: ${typeIndex}`);
        }

        hcuConsumed = (operatorsPrices['trivialEncrypt'].types as Record<string, number>)[type];
        totalHCUConsumed += hcuConsumed;
        handleResult = ethers.toBeHex(event.args[3], 32);
        hcuMap[handleResult] = hcuConsumed;
        handleSet.add(handleResult);
        break;

      case 'TrivialEncryptBytes':
        typeIndex = parseInt(event.args[2]);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;
        if (!type) {
          throw new Error(`Invalid FheType index: ${typeIndex}`);
        }

        hcuConsumed = (operatorsPrices['trivialEncrypt'].types as Record<string, number>)[type];
        totalHCUConsumed += hcuConsumed;
        handleResult = ethers.toBeHex(event.args[3], 32);
        hcuMap[handleResult] = hcuConsumed;
        handleSet.add(handleResult);
        break;

      case 'FheAdd':
        handleResult = ethers.toBeHex(event.args[4], 32);
        typeIndex = parseInt(handleResult.slice(-4, -2), 16);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;

        if (!type) {
          throw new Error(`Invalid FheType index: ${typeIndex}`);
        }

        if (event.args[3] === '0x01') {
          hcuConsumed = (operatorsPrices['fheAdd'].scalar as Record<string, number>)[type];
          hcuMap[handleResult] = hcuConsumed + readFromHCUMap(ethers.toBeHex(event.args[1], 32));
        } else {
          hcuConsumed = (operatorsPrices['fheAdd'].nonScalar as Record<string, number>)[type];
          hcuMap[handleResult] =
            hcuConsumed +
            Math.max(
              readFromHCUMap(ethers.toBeHex(event.args[1], 32)),
              readFromHCUMap(ethers.toBeHex(event.args[2], 32)),
            );
        }

        handleSet.add(handleResult);
        totalHCUConsumed += hcuConsumed;
        break;

      case 'FheSub':
        handleResult = ethers.toBeHex(event.args[4], 32);
        typeIndex = parseInt(handleResult.slice(-4, -2), 16);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;

        if (!type) {
          throw new Error(`Invalid FheType index: ${typeIndex}`);
        }

        if (event.args[3] === '0x01') {
          hcuConsumed = (operatorsPrices['fheSub'].scalar as Record<string, number>)[type];
          hcuMap[handleResult] = hcuConsumed + readFromHCUMap(ethers.toBeHex(event.args[1], 32));
        } else {
          hcuConsumed = (operatorsPrices['fheSub'].nonScalar as Record<string, number>)[type];
          hcuMap[handleResult] =
            hcuConsumed +
            Math.max(
              readFromHCUMap(ethers.toBeHex(event.args[1], 32)),
              readFromHCUMap(ethers.toBeHex(event.args[2], 32)),
            );
        }

        handleSet.add(handleResult);
        totalHCUConsumed += hcuConsumed;
        break;

      case 'FheMul':
        handleResult = ethers.toBeHex(event.args[4], 32);
        typeIndex = parseInt(handleResult.slice(-4, -2), 16);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;

        if (!type) {
          throw new Error(`Invalid FheType index: ${typeIndex}`);
        }

        if (event.args[3] === '0x01') {
          hcuConsumed = (operatorsPrices['fheMul'].scalar as Record<string, number>)[type];
          hcuMap[handleResult] = hcuConsumed + readFromHCUMap(ethers.toBeHex(event.args[1], 32));
        } else {
          hcuConsumed = (operatorsPrices['fheMul'].nonScalar as Record<string, number>)[type];
          hcuMap[handleResult] =
            hcuConsumed +
            Math.max(
              readFromHCUMap(ethers.toBeHex(event.args[1], 32)),
              readFromHCUMap(ethers.toBeHex(event.args[2], 32)),
            );
        }

        handleSet.add(handleResult);
        totalHCUConsumed += hcuConsumed;
        break;

      case 'FheDiv':
        handleResult = ethers.toBeHex(event.args[4], 32);
        typeIndex = parseInt(handleResult.slice(-4, -2), 16);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;
        if (!type) {
          throw new Error(`Invalid FheType index: ${typeIndex}`);
        }
        if (event.args[3] === '0x01') {
          hcuConsumed = (operatorsPrices['fheDiv'].scalar as Record<string, number>)[type];
          hcuMap[handleResult] = hcuConsumed + readFromHCUMap(ethers.toBeHex(event.args[1], 32));
        } else {
          throw new Error('Non-scalar div not implemented yet');
        }

        handleSet.add(handleResult);
        totalHCUConsumed += hcuConsumed;
        break;

      case 'FheRem':
        handleResult = ethers.toBeHex(event.args[4], 32);
        typeIndex = parseInt(handleResult.slice(-4, -2), 16);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;
        if (!type) {
          throw new Error(`Invalid FheType index: ${typeIndex}`);
        }
        if (event.args[3] === '0x01') {
          hcuConsumed = (operatorsPrices['fheRem'].scalar as Record<string, number>)[type];
          hcuMap[handleResult] = hcuConsumed + readFromHCUMap(ethers.toBeHex(event.args[1], 32));
        } else {
          throw new Error('Non-scalar rem not implemented yet');
        }
        handleSet.add(handleResult);
        totalHCUConsumed += hcuConsumed;
        break;

      case 'FheBitAnd':
        handleResult = ethers.toBeHex(event.args[4], 32);
        typeIndex = parseInt(handleResult.slice(-4, -2), 16);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;
        if (!type) {
          throw new Error(`Invalid FheType index: ${typeIndex}`);
        }
        if (event.args[3] === '0x01') {
          hcuConsumed = (operatorsPrices['fheBitAnd'].scalar as Record<string, number>)[type];
          hcuMap[handleResult] = hcuConsumed + readFromHCUMap(ethers.toBeHex(event.args[1], 32));
        } else {
          hcuConsumed = (operatorsPrices['fheBitAnd'].nonScalar as Record<string, number>)[type];
          hcuMap[handleResult] =
            hcuConsumed +
            Math.max(
              readFromHCUMap(ethers.toBeHex(event.args[1], 32)),
              readFromHCUMap(ethers.toBeHex(event.args[2], 32)),
            );
        }
        handleSet.add(handleResult);
        totalHCUConsumed += hcuConsumed;
        break;

      case 'FheBitOr':
        handleResult = ethers.toBeHex(event.args[4], 32);
        typeIndex = parseInt(handleResult.slice(-4, -2), 16);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;
        if (!type) {
          throw new Error(`Invalid FheType index: ${typeIndex}`);
        }
        if (event.args[3] === '0x01') {
          hcuConsumed = (operatorsPrices['fheBitOr'].scalar as Record<string, number>)[type];
          hcuMap[handleResult] = hcuConsumed + readFromHCUMap(ethers.toBeHex(event.args[1], 32));
        } else {
          hcuConsumed = (operatorsPrices['fheBitOr'].nonScalar as Record<string, number>)[type];
          hcuMap[handleResult] =
            hcuConsumed +
            Math.max(
              readFromHCUMap(ethers.toBeHex(event.args[1], 32)),
              readFromHCUMap(ethers.toBeHex(event.args[2], 32)),
            );
        }
        handleSet.add(handleResult);
        totalHCUConsumed += hcuConsumed;
        break;

      case 'FheBitXor':
        handleResult = ethers.toBeHex(event.args[4], 32);
        typeIndex = parseInt(handleResult.slice(-4, -2), 16);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;
        if (!type) {
          throw new Error(`Invalid FheType index: ${typeIndex}`);
        }
        if (event.args[3] === '0x01') {
          hcuConsumed = (operatorsPrices['fheBitXor'].scalar as Record<string, number>)[type];
          hcuMap[handleResult] = hcuConsumed + readFromHCUMap(ethers.toBeHex(event.args[1], 32));
        } else {
          hcuConsumed = (operatorsPrices['fheBitXor'].nonScalar as Record<string, number>)[type];
          hcuMap[handleResult] =
            hcuConsumed +
            Math.max(
              readFromHCUMap(ethers.toBeHex(event.args[1], 32)),
              readFromHCUMap(ethers.toBeHex(event.args[2], 32)),
            );
        }
        handleSet.add(handleResult);
        totalHCUConsumed += hcuConsumed;
        break;

      case 'FheShl':
        handleResult = ethers.toBeHex(event.args[4], 32);
        typeIndex = parseInt(handleResult.slice(-4, -2), 16);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;
        if (!type) {
          throw new Error(`Invalid FheType index: ${typeIndex}`);
        }
        if (event.args[3] === '0x01') {
          hcuConsumed = (operatorsPrices['fheShl'].scalar as Record<string, number>)[type];
          hcuMap[handleResult] = hcuConsumed + readFromHCUMap(ethers.toBeHex(event.args[1], 32));
        } else {
          hcuConsumed = (operatorsPrices['fheShl'].nonScalar as Record<string, number>)[type];
          hcuMap[handleResult] =
            hcuConsumed +
            Math.max(
              readFromHCUMap(ethers.toBeHex(event.args[1], 32)),
              readFromHCUMap(ethers.toBeHex(event.args[2], 32)),
            );
        }
        handleSet.add(handleResult);
        totalHCUConsumed += hcuConsumed;
        break;

      case 'FheShr':
        handleResult = ethers.toBeHex(event.args[4], 32);
        typeIndex = parseInt(handleResult.slice(-4, -2), 16);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;
        if (!type) {
          throw new Error(`Invalid FheType index: ${typeIndex}`);
        }
        if (event.args[3] === '0x01') {
          hcuConsumed = (operatorsPrices['fheShr'].scalar as Record<string, number>)[type];
          hcuMap[handleResult] = hcuConsumed + readFromHCUMap(ethers.toBeHex(event.args[1], 32));
        } else {
          hcuConsumed = (operatorsPrices['fheShr'].nonScalar as Record<string, number>)[type];
          hcuMap[handleResult] =
            hcuConsumed +
            Math.max(
              readFromHCUMap(ethers.toBeHex(event.args[1], 32)),
              readFromHCUMap(ethers.toBeHex(event.args[2], 32)),
            );
        }
        handleSet.add(handleResult);
        totalHCUConsumed += hcuConsumed;
        break;

      case 'FheRotl':
        handleResult = ethers.toBeHex(event.args[4], 32);
        typeIndex = parseInt(handleResult.slice(-4, -2), 16);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;
        if (!type) {
          throw new Error(`Invalid FheType index: ${typeIndex}`);
        }
        if (event.args[3] === '0x01') {
          hcuConsumed = (operatorsPrices['fheRotl'].scalar as Record<string, number>)[type];
          hcuMap[handleResult] = hcuConsumed + readFromHCUMap(ethers.toBeHex(event.args[1], 32));
        } else {
          hcuConsumed = (operatorsPrices['fheRotl'].nonScalar as Record<string, number>)[type];
          hcuMap[handleResult] =
            hcuConsumed +
            Math.max(
              readFromHCUMap(ethers.toBeHex(event.args[1], 32)),
              readFromHCUMap(ethers.toBeHex(event.args[2], 32)),
            );
        }
        handleSet.add(handleResult);
        totalHCUConsumed += hcuConsumed;
        break;

      case 'FheRotr':
        handleResult = ethers.toBeHex(event.args[4], 32);
        typeIndex = parseInt(handleResult.slice(-4, -2), 16);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;
        if (!type) {
          throw new Error(`Invalid FheType index: ${typeIndex}`);
        }
        if (event.args[3] === '0x01') {
          hcuConsumed = (operatorsPrices['fheRotr'].scalar as Record<string, number>)[type];
          hcuMap[handleResult] = hcuConsumed + readFromHCUMap(ethers.toBeHex(event.args[1], 32));
        } else {
          hcuConsumed = (operatorsPrices['fheRotr'].nonScalar as Record<string, number>)[type];
          hcuMap[handleResult] =
            hcuConsumed +
            Math.max(
              readFromHCUMap(ethers.toBeHex(event.args[1], 32)),
              readFromHCUMap(ethers.toBeHex(event.args[2], 32)),
            );
        }
        handleSet.add(handleResult);
        totalHCUConsumed += hcuConsumed;
        break;

      case 'FheEq':
        handleResult = ethers.toBeHex(event.args[4], 32);
        handle = ethers.toBeHex(event.args[1], 32);
        typeIndex = parseInt(handle.slice(-4, -2), 16);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;

        if (!type) {
          throw new Error(`Invalid FheType index: ${typeIndex}`);
        }
        if (event.args[3] === '0x01') {
          hcuConsumed = (operatorsPrices['fheEq'].scalar as Record<string, number>)[type];
          hcuMap[handleResult] = hcuConsumed + readFromHCUMap(ethers.toBeHex(event.args[1], 32));
        } else {
          hcuConsumed = (operatorsPrices['fheEq'].nonScalar as Record<string, number>)[type];
          hcuMap[handleResult] =
            hcuConsumed +
            Math.max(
              readFromHCUMap(ethers.toBeHex(event.args[1], 32)),
              readFromHCUMap(ethers.toBeHex(event.args[2], 32)),
            );
        }
        handleSet.add(handleResult);
        totalHCUConsumed += hcuConsumed;
        break;

      case 'FheEqBytes':
        handleResult = ethers.toBeHex(event.args[4], 32);
        handle = ethers.toBeHex(event.args[1], 32);
        typeIndex = parseInt(handle.slice(-4, -2), 16);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;

        if (!type) {
          throw new Error(`Invalid FheType index: ${typeIndex}`);
        }
        if (event.args[3] === '0x01') {
          hcuConsumed = (operatorsPrices['fheEq'].scalar as Record<string, number>)[type];
          hcuMap[handleResult] = hcuConsumed + readFromHCUMap(ethers.toBeHex(event.args[1], 32));
        } else {
          hcuConsumed = (operatorsPrices['fheEq'].nonScalar as Record<string, number>)[type];
          hcuMap[handleResult] =
            hcuConsumed +
            Math.max(
              readFromHCUMap(ethers.toBeHex(event.args[1], 32)),
              readFromHCUMap(ethers.toBeHex(event.args[2], 32)),
            );
        }
        handleSet.add(handleResult);
        totalHCUConsumed += hcuConsumed;
        break;

      case 'FheNe':
        handleResult = ethers.toBeHex(event.args[4], 32);
        handle = ethers.toBeHex(event.args[1], 32);
        typeIndex = parseInt(handle.slice(-4, -2), 16);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;

        if (!type) {
          throw new Error(`Invalid FheType index: ${typeIndex}`);
        }
        if (event.args[3] === '0x01') {
          hcuConsumed = (operatorsPrices['fheNe'].scalar as Record<string, number>)[type];
          hcuMap[handleResult] = hcuConsumed + readFromHCUMap(ethers.toBeHex(event.args[1], 32));
        } else {
          hcuConsumed = (operatorsPrices['fheNe'].nonScalar as Record<string, number>)[type];
          hcuMap[handleResult] =
            hcuConsumed +
            Math.max(
              readFromHCUMap(ethers.toBeHex(event.args[1], 32)),
              readFromHCUMap(ethers.toBeHex(event.args[2], 32)),
            );
        }
        handleSet.add(handleResult);
        totalHCUConsumed += hcuConsumed;
        break;

      case 'FheNeBytes':
        handleResult = ethers.toBeHex(event.args[4], 32);
        handle = ethers.toBeHex(event.args[1], 32);
        typeIndex = parseInt(handle.slice(-4, -2), 16);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;

        if (!type) {
          throw new Error(`Invalid FheType index: ${typeIndex}`);
        }
        if (event.args[3] === '0x01') {
          hcuConsumed = (operatorsPrices['fheNe'].scalar as Record<string, number>)[type];
          hcuMap[handleResult] = hcuConsumed + readFromHCUMap(ethers.toBeHex(event.args[1], 32));
        } else {
          hcuConsumed = (operatorsPrices['fheNe'].nonScalar as Record<string, number>)[type];
          hcuMap[handleResult] =
            hcuConsumed +
            Math.max(
              readFromHCUMap(ethers.toBeHex(event.args[1], 32)),
              readFromHCUMap(ethers.toBeHex(event.args[2], 32)),
            );
        }
        handleSet.add(handleResult);
        totalHCUConsumed += hcuConsumed;
        break;

      case 'FheGe':
        handleResult = ethers.toBeHex(event.args[4], 32);
        handle = ethers.toBeHex(event.args[1], 32);
        typeIndex = parseInt(handle.slice(-4, -2), 16);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;

        if (!type) {
          throw new Error(`Invalid FheType index: ${typeIndex}`);
        }
        if (event.args[3] === '0x01') {
          hcuConsumed = (operatorsPrices['fheGe'].scalar as Record<string, number>)[type];
          hcuMap[handleResult] = hcuConsumed + readFromHCUMap(ethers.toBeHex(event.args[1], 32));
        } else {
          hcuConsumed = (operatorsPrices['fheGe'].nonScalar as Record<string, number>)[type];
          hcuMap[handleResult] =
            hcuConsumed +
            Math.max(
              readFromHCUMap(ethers.toBeHex(event.args[1], 32)),
              readFromHCUMap(ethers.toBeHex(event.args[2], 32)),
            );
        }
        handleSet.add(handleResult);
        totalHCUConsumed += hcuConsumed;
        break;

      case 'FheGt':
        handleResult = ethers.toBeHex(event.args[4], 32);
        handle = ethers.toBeHex(event.args[1], 32);
        typeIndex = parseInt(handle.slice(-4, -2), 16);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;
        if (!type) {
          throw new Error(`Invalid FheType index: ${typeIndex}`);
        }
        if (event.args[3] === '0x01') {
          hcuConsumed = (operatorsPrices['fheGt'].scalar as Record<string, number>)[type];
          hcuMap[handleResult] = hcuConsumed + readFromHCUMap(ethers.toBeHex(event.args[1], 32));
        } else {
          hcuConsumed = (operatorsPrices['fheGt'].nonScalar as Record<string, number>)[type];
          hcuMap[handleResult] =
            hcuConsumed +
            Math.max(
              readFromHCUMap(ethers.toBeHex(event.args[1], 32)),
              readFromHCUMap(ethers.toBeHex(event.args[2], 32)),
            );
        }
        handleSet.add(handleResult);
        totalHCUConsumed += hcuConsumed;
        break;

      case 'FheLe':
        handleResult = ethers.toBeHex(event.args[4], 32);
        handle = ethers.toBeHex(event.args[1], 32);
        typeIndex = parseInt(handle.slice(-4, -2), 16);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;
        if (!type) {
          throw new Error(`Invalid FheType index: ${typeIndex}`);
        }
        if (event.args[3] === '0x01') {
          hcuConsumed = (operatorsPrices['fheLe'].scalar as Record<string, number>)[type];
          hcuMap[handleResult] = hcuConsumed + readFromHCUMap(ethers.toBeHex(event.args[1], 32));
        } else {
          hcuConsumed = (operatorsPrices['fheLe'].nonScalar as Record<string, number>)[type];
          hcuMap[handleResult] =
            hcuConsumed +
            Math.max(
              readFromHCUMap(ethers.toBeHex(event.args[1], 32)),
              readFromHCUMap(ethers.toBeHex(event.args[2], 32)),
            );
        }
        handleSet.add(handleResult);
        totalHCUConsumed += hcuConsumed;
        break;

      case 'FheLt':
        handleResult = ethers.toBeHex(event.args[4], 32);
        handle = ethers.toBeHex(event.args[1], 32);
        typeIndex = parseInt(handle.slice(-4, -2), 16);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;
        if (!type) {
          throw new Error(`Invalid FheType index: ${typeIndex}`);
        }
        if (event.args[3] === '0x01') {
          hcuConsumed = (operatorsPrices['fheLt'].scalar as Record<string, number>)[type];
          hcuMap[handleResult] = hcuConsumed + readFromHCUMap(ethers.toBeHex(event.args[1], 32));
        } else {
          hcuConsumed = (operatorsPrices['fheLt'].nonScalar as Record<string, number>)[type];
          hcuMap[handleResult] =
            hcuConsumed +
            Math.max(
              readFromHCUMap(ethers.toBeHex(event.args[1], 32)),
              readFromHCUMap(ethers.toBeHex(event.args[2], 32)),
            );
        }
        handleSet.add(handleResult);
        totalHCUConsumed += hcuConsumed;
        break;

      case 'FheMax':
        handleResult = ethers.toBeHex(event.args[4], 32);
        typeIndex = parseInt(handleResult.slice(-4, -2), 16);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;

        if (!type) {
          throw new Error(`Invalid FheType index: ${typeIndex}`);
        }

        if (event.args[3] === '0x01') {
          hcuConsumed = (operatorsPrices['fheMax'].scalar as Record<string, number>)[type];
          hcuMap[handleResult] = hcuConsumed + readFromHCUMap(ethers.toBeHex(event.args[1], 32));
        } else {
          hcuConsumed = (operatorsPrices['fheMax'].nonScalar as Record<string, number>)[type];
          hcuMap[handleResult] =
            hcuConsumed +
            Math.max(
              readFromHCUMap(ethers.toBeHex(event.args[1], 32)),
              readFromHCUMap(ethers.toBeHex(event.args[2], 32)),
            );
        }
        handleSet.add(handleResult);
        totalHCUConsumed += hcuConsumed;
        break;

      case 'FheMin':
        handleResult = ethers.toBeHex(event.args[4], 32);
        typeIndex = parseInt(handleResult.slice(-4, -2), 16);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;

        if (!type) {
          throw new Error(`Invalid FheType index: ${typeIndex}`);
        }

        if (event.args[3] === '0x01') {
          hcuConsumed = (operatorsPrices['fheMin'].scalar as Record<string, number>)[type];
          hcuMap[handleResult] = hcuConsumed + readFromHCUMap(ethers.toBeHex(event.args[1], 32));
        } else {
          hcuConsumed = (operatorsPrices['fheMin'].nonScalar as Record<string, number>)[type];
          hcuMap[handleResult] =
            hcuConsumed +
            Math.max(
              readFromHCUMap(ethers.toBeHex(event.args[1], 32)),
              readFromHCUMap(ethers.toBeHex(event.args[2], 32)),
            );
        }

        handleSet.add(handleResult);
        totalHCUConsumed += hcuConsumed;
        break;

      case 'Cast':
        handle = ethers.toBeHex(event.args[1], 32);
        typeIndex = parseInt(handle.slice(-4, -2), 16);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;
        handleResult = ethers.toBeHex(event.args[3], 32);

        if (!type) {
          throw new Error(`Invalid FheType index: ${typeIndex}`);
        }

        hcuConsumed = (operatorsPrices['cast'].types as Record<string, number>)[type];
        hcuMap[handleResult] = hcuConsumed + readFromHCUMap(handle);
        totalHCUConsumed += hcuConsumed;
        handleSet.add(handleResult);
        break;

      case 'FheNot':
        handle = ethers.toBeHex(event.args[1], 32);
        typeIndex = parseInt(handle.slice(-4, -2), 16);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;
        handleResult = ethers.toBeHex(event.args[2], 32);
        if (!type) {
          throw new Error(`Invalid FheType index: ${typeIndex}`);
        }
        hcuConsumed = (operatorsPrices['fheNot'].types as Record<string, number>)[type];
        hcuMap[handleResult] = hcuConsumed + readFromHCUMap(handle);
        totalHCUConsumed += hcuConsumed;
        handleSet.add(ethers.toBeHex(event.args[2], 32));
        break;

      case 'FheNeg':
        handle = ethers.toBeHex(event.args[1], 32);
        typeIndex = parseInt(handle.slice(-4, -2), 16);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;
        handleResult = ethers.toBeHex(event.args[2], 32);
        if (!type) {
          throw new Error(`Invalid FheType index: ${typeIndex}`);
        }
        hcuConsumed = (operatorsPrices['fheNeg'].types as Record<string, number>)[type];
        hcuMap[handleResult] = hcuConsumed + readFromHCUMap(handle);
        totalHCUConsumed += hcuConsumed;
        handleSet.add(ethers.toBeHex(event.args[2], 32));
        break;

      case 'FheIfThenElse':
        handleResult = ethers.toBeHex(event.args[4], 32);
        typeIndex = parseInt(handleResult.slice(-4, -2), 16);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;

        if (!type) {
          throw new Error(`Invalid FheType index: ${typeIndex}`);
        }

        hcuConsumed = (operatorsPrices['ifThenElse'].types as Record<string, number>)[type];
        hcuMap[handleResult] =
          hcuConsumed +
          Math.max(
            readFromHCUMap(ethers.toBeHex(event.args[1], 32)),
            readFromHCUMap(ethers.toBeHex(event.args[2], 32)),
            readFromHCUMap(ethers.toBeHex(event.args[3], 32)),
          );
        handleSet.add(handleResult);
        totalHCUConsumed += hcuConsumed;
        break;

      case 'FheRand':
        handleResult = ethers.toBeHex(event.args[3], 32);
        typeIndex = parseInt(event.args[1]);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;
        if (!type) {
          throw new Error(`Invalid FheType index: ${typeIndex}`);
        }
        hcuConsumed = (operatorsPrices['fheRand'].types as Record<string, number>)[type];
        hcuMap[handleResult] = hcuConsumed;
        handleSet.add(handleResult);
        totalHCUConsumed += hcuConsumed;
        break;

      case 'FheRandBounded':
        handleResult = ethers.toBeHex(event.args[4], 32);
        typeIndex = parseInt(event.args[2]);
        type = FheTypes.find((t) => t.value === typeIndex)?.type;
        if (!type) {
          throw new Error(`Invalid FheType index: ${typeIndex}`);
        }
        hcuConsumed = (operatorsPrices['fheRandBounded'].types as Record<string, number>)[type];
        hcuMap[handleResult] = hcuConsumed;
        handleSet.add(handleResult);
        totalHCUConsumed += hcuConsumed;
        break;
    }
  }

  let maxDepthHCU = 0;

  handleSet.forEach((handle) => {
    const hcu = hcuMap[handle];
    if (hcu > maxDepthHCU) {
      maxDepthHCU = hcu;
    }
  });

  return {
    globalTxHCU: totalHCUConsumed,
    maxTxHCUDepth: maxDepthHCU,
    HCUDepthPerHandle: hcuMap,
  };
}
