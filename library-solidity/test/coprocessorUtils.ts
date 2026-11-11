import dotenv from 'dotenv';
import type { ethers as EthersT } from 'ethers';
import * as fs from 'fs';
import { ethers } from 'hardhat';

import type { FheTypeInfo } from '../lib-js/common';
import { ALL_FHE_TYPE_INFOS } from '../lib-js/fheTypeInfos';
import { ALL_OPERATORS_PRICES } from '../lib-js/operatorsPrices';
import { HOST_ADDRESSES_ENV_PATH } from './utils';

const parsedEnvCoprocessor = dotenv.parse(fs.readFileSync(HOST_ADDRESSES_ENV_PATH));
const coprocAddress = parsedEnvCoprocessor.FHEVM_EXECUTOR_CONTRACT_ADDRESS;

let firstBlockListening = 0;
let lastBlockSnapshot = 0;
let chainId: number | undefined;

// Clear value (as a decimal string) of every handle, keyed by the 0x-prefixed lowercase 32-byte hex handle.
const clearTexts = new Map<string, string>();

// Returns the key under which `handle` is stored in `clearTexts`.
function toClearTextKey(handle: string | bigint): string {
  return ethers.toBeHex(handle, 32);
}

// Stores the clear value of `handle`. Keeps an existing value unless `replace` is true.
export function coprocDbWriteCleartext(handle: string, clearText: bigint | string, replace: boolean = false): void {
  const key = toClearTextKey(handle);
  // `replace` is useful if using snapshots while sampling different random numbers on each revert
  if (replace || !clearTexts.has(key)) {
    clearTexts.set(key, clearText.toString());
  }
}

// Decrypt any handle, bypassing ACL
// WARNING : only for testing or internal use
export function coprocDbReadCleartext(handle: string | bigint): Promise<string> {
  const clearText = clearTexts.get(toClearTextKey(handle));
  if (clearText === undefined) {
    return Promise.reject(new Error(`No clear value found for handle ${toClearTextKey(handle)}`));
  }
  return Promise.resolve(clearText);
}

// floor(log2(x)) for a bigint, i.e. its bit length minus one.
// Returns 0n for x <= 0 (same as extra-bigint's log2).
function log2(x: bigint): bigint {
  const n = x.toString(2).length - 1;
  return x <= 0n ? 0n : BigInt(n);
}

interface FHEVMEvent {
  eventName: string;
  args: EthersT.Result;
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
};

// Returns `fheType` as a key of NumBits, throwing if NumBits has no entry for it.
function toNumBitsFheType(fheType: number): keyof typeof NumBits {
  if (!(fheType in NumBits)) {
    throw new Error(`Unsupported FheType: ${fheType}`);
  }
  return fheType as keyof typeof NumBits;
}

// Returns the FheType index encoded in byte 30 of `handle`.
function getHandleTypeIndex(handle: string): number {
  return parseInt(handle.slice(-4, -2), 16);
}

// Returns the FheType encoded in byte 30 of `handle`, as a key of NumBits.
function getHandleFheType(handle: string): keyof typeof NumBits {
  return toNumBitsFheType(getHandleTypeIndex(handle));
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

function bitwiseNotUintBits(value: bigint, numBits: number): bigint {
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
  const currentChainId = Number((await ethers.provider.getNetwork()).chainId);
  if (chainId !== undefined && chainId !== currentChainId) {
    throw new Error(`Chain ID changed from ${chainId} to ${currentChainId} since the previous awaitCoprocessor() call`);
  }
  chainId = currentChainId;
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
  'event VerifyInput(address indexed caller, bytes32 inputHandle, address userAddress, bytes inputProof, uint8 inputType, bytes32 result)',
  'event Cast(address indexed caller, bytes32 ct, uint8 toType, bytes32 result)',
  'event TrivialEncrypt(address indexed caller, uint256 pt, uint8 toType, bytes32 result)',
  'event TrivialEncryptBytes(address indexed caller, bytes pt, uint8 toType, bytes32 result)',
  'event FheIfThenElse(address indexed caller, bytes32 control, bytes32 ifTrue, bytes32 ifFalse, bytes32 result)',
  'event FheRand(address indexed caller, uint8 randType, bytes16 seed, bytes32 result)',
  'event FheRandBounded(address indexed caller, uint256 upperBound, uint8 randType, bytes16 seed, bytes32 result)',
  'event FheSum(address indexed caller, bytes32[] values, bytes32 result)',
  'event FheIsIn(address indexed caller, bytes32 value, bytes32[] values, bytes32 result)',
  'event FheMulDiv(address indexed caller, bytes32 factor1, bytes32 factor2, bytes32 divisor, bytes1 scalarByte, bytes32 result)',
];

// Parses `log` as an FHEVMExecutor event, or returns undefined if `iface` does not know it.
function parseFHEVMEvent(iface: EthersT.Interface, log: EthersT.Log): FHEVMEvent | undefined {
  let parsedLog: EthersT.LogDescription | null;
  try {
    parsedLog = iface.parseLog(log);
  } catch {
    return undefined;
  }
  if (parsedLog === null) {
    return undefined;
  }
  return { eventName: parsedLog.name, args: parsedLog.args };
}

// Throws if `logs` are not in emission order (by block number, then by log index within a block).
function assertLogsInEmissionOrder(logs: readonly EthersT.Log[]): void {
  for (let i = 1; i < logs.length; i++) {
    const previous = logs[i - 1];
    const current = logs[i];
    const isAfterPrevious =
      current.blockNumber > previous.blockNumber ||
      (current.blockNumber === previous.blockNumber && current.index > previous.index);
    if (!isAfterPrevious) {
      throw new Error(
        `Logs are not in emission order: log ${i} (block ${current.blockNumber}, index ${current.index}) ` +
          `follows block ${previous.blockNumber}, index ${previous.index}`,
      );
    }
  }
}

// Validates the [lastBlockSnapshot, lastCounterRand] pair returned by CustomProvider's `get_lastBlockSnapshot`.
// lastCounterRand is a number, or a 32-byte hex string once CustomProvider has read it from storage after `evm_revert`.
function parseLastBlockSnapshot(value: unknown): [number, number] {
  if (!Array.isArray(value) || value.length !== 2) {
    throw new Error('get_lastBlockSnapshot: expected a [lastBlockSnapshot, lastCounterRand] pair');
  }
  const blockSnapshot: unknown = value[0];
  const counterRandValue: unknown = value[1];
  if (typeof blockSnapshot !== 'number') {
    throw new Error(`get_lastBlockSnapshot: expected lastBlockSnapshot to be a number, got ${typeof blockSnapshot}`);
  }
  if (typeof counterRandValue !== 'number' && typeof counterRandValue !== 'string') {
    throw new Error(`get_lastBlockSnapshot: expected lastCounterRand to be a number or a hex string`);
  }
  const counterRandNumber = Number(counterRandValue);
  if (!Number.isInteger(counterRandNumber)) {
    throw new Error(`get_lastBlockSnapshot: invalid lastCounterRand ${String(counterRandValue)}`);
  }
  return [blockSnapshot, counterRandNumber];
}

async function processAllPastFHEVMExecutorEvents(): Promise<void> {
  const provider = ethers.provider;
  const latestBlockNumber = await provider.getBlockNumber();

  if (process.env.SOLIDITY_COVERAGE !== 'true') {
    // evm_snapshot is not supported in coverage mode
    [lastBlockSnapshot] = parseLastBlockSnapshot(await provider.send('get_lastBlockSnapshot', []));
    if (lastBlockSnapshot < firstBlockListening) {
      firstBlockListening = lastBlockSnapshot + 1;
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

  assertLogsInEmissionOrder(logs);

  const events: FHEVMEvent[] = [];
  for (const log of logs) {
    const event = parseFHEVMEvent(contract.interface, log);
    if (event !== undefined) {
      events.push(event);
    }
  }

  firstBlockListening = latestBlockNumber + 1;
  if (process.env.SOLIDITY_COVERAGE !== 'true') {
    // evm_snapshot is not supported in coverage mode
    await provider.send('set_lastBlockSnapshot', [firstBlockListening]);
  }
  // Process events in emission order: each operation's inputs are produced by earlier events.
  for (const event of events) {
    await insertHandleFromEvent(event);
  }
}

// Returns `event.args[index]`, throwing if it is not a bigint.
function getBigIntEventArg(event: FHEVMEvent, index: number): bigint {
  const value: unknown = event.args[index];
  if (typeof value !== 'bigint') {
    throw new Error(`${event.eventName}: expected argument ${index} to be a bigint, got ${typeof value}`);
  }
  return value;
}

// Returns `event.args[index]`, throwing if it is not a string.
function getStringEventArg(event: FHEVMEvent, index: number): string {
  const value: unknown = event.args[index];
  if (typeof value !== 'string') {
    throw new Error(`${event.eventName}: expected argument ${index} to be a string, got ${typeof value}`);
  }
  return value;
}

// Returns `event.args[index]`, throwing if it is not an array of strings.
function getStringArrayEventArg(event: FHEVMEvent, index: number): string[] {
  const value: unknown = event.args[index];
  if (!Array.isArray(value) || !value.every((v) => typeof v === 'string')) {
    throw new Error(`${event.eventName}: expected argument ${index} to be an array of strings`);
  }
  return value;
}

async function insertHandleFromEvent(event: FHEVMEvent): Promise<void> {
  let handle;
  let clearText;
  let clearLHS;
  let clearRHS;
  let resultType: keyof typeof NumBits;
  let shift;

  switch (event.eventName) {
    case 'TrivialEncrypt': {
      const pt: bigint = getBigIntEventArg(event, 1);
      const result: string = getStringEventArg(event, 3);

      clearText = pt;
      handle = ethers.toBeHex(result, 32);
      coprocDbWriteCleartext(handle, clearText);
      break;
    }

    case 'FheAdd': {
      const lhs: string = getStringEventArg(event, 1);
      const rhs: string = getStringEventArg(event, 2);
      const scalarByte: string = getStringEventArg(event, 3);
      const result: string = getStringEventArg(event, 4);

      handle = ethers.toBeHex(result, 32);
      resultType = getHandleFheType(handle);
      clearLHS = await coprocDbReadCleartext(lhs);
      if (scalarByte === '0x01') {
        clearText = BigInt(clearLHS) + BigInt(rhs);
        clearText = clearText % 2n ** NumBits[resultType];
      } else {
        clearRHS = await coprocDbReadCleartext(rhs);
        clearText = BigInt(clearLHS) + BigInt(clearRHS);
        clearText = clearText % 2n ** NumBits[resultType];
      }

      coprocDbWriteCleartext(ethers.toBeHex(handle, 32), clearText);
      break;
    }

    case 'FheSub': {
      const lhs: string = getStringEventArg(event, 1);
      const rhs: string = getStringEventArg(event, 2);
      const scalarByte: string = getStringEventArg(event, 3);
      const result: string = getStringEventArg(event, 4);

      handle = ethers.toBeHex(result, 32);
      resultType = getHandleFheType(handle);
      clearLHS = await coprocDbReadCleartext(lhs);
      if (scalarByte === '0x01') {
        clearText = BigInt(clearLHS) - BigInt(rhs);
        if (clearText < 0n) clearText = clearText + 2n ** NumBits[resultType];
        clearText = clearText % 2n ** NumBits[resultType];
      } else {
        clearRHS = await coprocDbReadCleartext(rhs);
        clearText = BigInt(clearLHS) - BigInt(clearRHS);
        if (clearText < 0n) clearText = clearText + 2n ** NumBits[resultType];
        clearText = clearText % 2n ** NumBits[resultType];
      }
      coprocDbWriteCleartext(handle, clearText);
      break;
    }

    case 'FheMul': {
      const lhs: string = getStringEventArg(event, 1);
      const rhs: string = getStringEventArg(event, 2);
      const scalarByte: string = getStringEventArg(event, 3);
      const result: string = getStringEventArg(event, 4);

      handle = ethers.toBeHex(result, 32);
      resultType = getHandleFheType(handle);
      clearLHS = await coprocDbReadCleartext(lhs);
      if (scalarByte === '0x01') {
        clearText = BigInt(clearLHS) * BigInt(rhs);
        clearText = clearText % 2n ** NumBits[resultType];
      } else {
        clearRHS = await coprocDbReadCleartext(rhs);
        clearText = BigInt(clearLHS) * BigInt(clearRHS);
        clearText = clearText % 2n ** NumBits[resultType];
      }
      coprocDbWriteCleartext(handle, clearText);
      break;
    }

    case 'FheDiv': {
      const lhs: string = getStringEventArg(event, 1);
      const rhs: string = getStringEventArg(event, 2);
      const scalarByte: string = getStringEventArg(event, 3);
      const result: string = getStringEventArg(event, 4);

      handle = ethers.toBeHex(result, 32);
      clearLHS = await coprocDbReadCleartext(lhs);
      if (scalarByte === '0x01') {
        clearText = BigInt(clearLHS) / BigInt(rhs);
      } else {
        throw new Error('Non-scalar div not implemented yet');
      }
      coprocDbWriteCleartext(handle, clearText);
      break;
    }

    case 'FheRem': {
      const lhs: string = getStringEventArg(event, 1);
      const rhs: string = getStringEventArg(event, 2);
      const scalarByte: string = getStringEventArg(event, 3);
      const result: string = getStringEventArg(event, 4);

      handle = ethers.toBeHex(result, 32);
      clearLHS = await coprocDbReadCleartext(lhs);
      if (scalarByte === '0x01') {
        clearText = BigInt(clearLHS) % BigInt(rhs);
      } else {
        throw new Error('Non-scalar rem not implemented yet');
      }
      coprocDbWriteCleartext(handle, clearText);
      break;
    }

    case 'FheBitAnd': {
      const lhs: string = getStringEventArg(event, 1);
      const rhs: string = getStringEventArg(event, 2);
      const scalarByte: string = getStringEventArg(event, 3);
      const result: string = getStringEventArg(event, 4);

      handle = ethers.toBeHex(result, 32);
      resultType = getHandleFheType(handle);
      clearLHS = await coprocDbReadCleartext(lhs);
      if (scalarByte === '0x01') {
        clearText = BigInt(clearLHS) & BigInt(rhs);
        clearText = clearText % 2n ** NumBits[resultType];
      } else {
        clearRHS = await coprocDbReadCleartext(rhs);
        clearText = BigInt(clearLHS) & BigInt(clearRHS);
        clearText = clearText % 2n ** NumBits[resultType];
      }
      coprocDbWriteCleartext(handle, clearText);
      break;
    }

    case 'FheBitOr': {
      const lhs: string = getStringEventArg(event, 1);
      const rhs: string = getStringEventArg(event, 2);
      const scalarByte: string = getStringEventArg(event, 3);
      const result: string = getStringEventArg(event, 4);

      handle = ethers.toBeHex(result, 32);
      resultType = getHandleFheType(handle);
      clearLHS = await coprocDbReadCleartext(lhs);
      if (scalarByte === '0x01') {
        clearText = BigInt(clearLHS) | BigInt(rhs);
        clearText = clearText % 2n ** NumBits[resultType];
      } else {
        clearRHS = await coprocDbReadCleartext(rhs);
        clearText = BigInt(clearLHS) | BigInt(clearRHS);
        clearText = clearText % 2n ** NumBits[resultType];
      }
      coprocDbWriteCleartext(handle, clearText);
      break;
    }

    case 'FheBitXor': {
      const lhs: string = getStringEventArg(event, 1);
      const rhs: string = getStringEventArg(event, 2);
      const scalarByte: string = getStringEventArg(event, 3);
      const result: string = getStringEventArg(event, 4);

      handle = ethers.toBeHex(result, 32);
      resultType = getHandleFheType(handle);
      clearLHS = await coprocDbReadCleartext(lhs);
      if (scalarByte === '0x01') {
        clearText = BigInt(clearLHS) ^ BigInt(rhs);
        clearText = clearText % 2n ** NumBits[resultType];
      } else {
        clearRHS = await coprocDbReadCleartext(rhs);
        clearText = BigInt(clearLHS) ^ BigInt(clearRHS);
        clearText = clearText % 2n ** NumBits[resultType];
      }
      coprocDbWriteCleartext(handle, clearText);
      break;
    }

    case 'FheShl': {
      const lhs: string = getStringEventArg(event, 1);
      const rhs: string = getStringEventArg(event, 2);
      const scalarByte: string = getStringEventArg(event, 3);
      const result: string = getStringEventArg(event, 4);

      handle = ethers.toBeHex(result, 32);
      resultType = getHandleFheType(handle);
      clearLHS = await coprocDbReadCleartext(lhs);
      if (scalarByte === '0x01') {
        clearText = BigInt(clearLHS) << (BigInt(rhs) % NumBits[resultType]);
        clearText = clearText % 2n ** NumBits[resultType];
      } else {
        clearRHS = await coprocDbReadCleartext(rhs);
        clearText = BigInt(clearLHS) << (BigInt(clearRHS) % NumBits[resultType]);
        clearText = clearText % 2n ** NumBits[resultType];
      }
      coprocDbWriteCleartext(handle, clearText);
      break;
    }

    case 'FheShr': {
      const lhs: string = getStringEventArg(event, 1);
      const rhs: string = getStringEventArg(event, 2);
      const scalarByte: string = getStringEventArg(event, 3);
      const result: string = getStringEventArg(event, 4);

      handle = ethers.toBeHex(result, 32);
      resultType = getHandleFheType(handle);
      clearLHS = await coprocDbReadCleartext(lhs);
      if (scalarByte === '0x01') {
        clearText = BigInt(clearLHS) >> (BigInt(rhs) % NumBits[resultType]);
        clearText = clearText % 2n ** NumBits[resultType];
      } else {
        clearRHS = await coprocDbReadCleartext(rhs);
        clearText = BigInt(clearLHS) >> (BigInt(clearRHS) % NumBits[resultType]);
        clearText = clearText % 2n ** NumBits[resultType];
      }
      coprocDbWriteCleartext(handle, clearText);
      break;
    }

    case 'FheRotl': {
      const lhs: string = getStringEventArg(event, 1);
      const rhs: string = getStringEventArg(event, 2);
      const scalarByte: string = getStringEventArg(event, 3);
      const result: string = getStringEventArg(event, 4);

      handle = ethers.toBeHex(result, 32);
      resultType = getHandleFheType(handle);
      clearLHS = await coprocDbReadCleartext(lhs);
      if (scalarByte === '0x01') {
        shift = BigInt(rhs) % NumBits[resultType];
        clearText = (BigInt(clearLHS) << shift) | (BigInt(clearLHS) >> (NumBits[resultType] - shift));
        clearText = clearText % 2n ** NumBits[resultType];
      } else {
        clearRHS = await coprocDbReadCleartext(rhs);
        shift = BigInt(clearRHS) % NumBits[resultType];
        clearText = (BigInt(clearLHS) << shift) | (BigInt(clearLHS) >> (NumBits[resultType] - shift));
        clearText = clearText % 2n ** NumBits[resultType];
      }
      coprocDbWriteCleartext(handle, clearText);
      break;
    }

    case 'FheRotr': {
      const lhs: string = getStringEventArg(event, 1);
      const rhs: string = getStringEventArg(event, 2);
      const scalarByte: string = getStringEventArg(event, 3);
      const result: string = getStringEventArg(event, 4);

      handle = ethers.toBeHex(result, 32);
      resultType = getHandleFheType(handle);
      clearLHS = await coprocDbReadCleartext(lhs);
      if (scalarByte === '0x01') {
        shift = BigInt(rhs) % NumBits[resultType];
        clearText = (BigInt(clearLHS) >> shift) | (BigInt(clearLHS) << (NumBits[resultType] - shift));
        clearText = clearText % 2n ** NumBits[resultType];
      } else {
        clearRHS = await coprocDbReadCleartext(rhs);
        shift = BigInt(clearRHS) % NumBits[resultType];
        clearText = (BigInt(clearLHS) >> shift) | (BigInt(clearLHS) << (NumBits[resultType] - shift));
        clearText = clearText % 2n ** NumBits[resultType];
      }
      coprocDbWriteCleartext(handle, clearText);
      break;
    }

    case 'FheEq': {
      const lhs: string = getStringEventArg(event, 1);
      const rhs: string = getStringEventArg(event, 2);
      const scalarByte: string = getStringEventArg(event, 3);
      const result: string = getStringEventArg(event, 4);

      handle = ethers.toBeHex(result, 32);
      clearLHS = await coprocDbReadCleartext(lhs);

      if (scalarByte === '0x01') {
        clearText = BigInt(clearLHS) === BigInt(rhs) ? 1n : 0n;
      } else {
        clearRHS = await coprocDbReadCleartext(rhs);
        clearText = clearLHS === clearRHS ? 1n : 0n;
      }

      coprocDbWriteCleartext(handle, clearText);
      break;
    }

    case 'FheEqBytes': {
      const lhs: string = getStringEventArg(event, 1);
      const rhs: string = getStringEventArg(event, 2);
      const scalarByte: string = getStringEventArg(event, 3);
      const result: string = getStringEventArg(event, 4);

      handle = ethers.toBeHex(result, 32);
      clearLHS = await coprocDbReadCleartext(lhs);
      if (scalarByte === '0x01') {
        clearText = BigInt(clearLHS) === BigInt(rhs) ? 1n : 0n;
      } else {
        clearRHS = await coprocDbReadCleartext(rhs);
        clearText = BigInt(clearLHS) === BigInt(clearRHS) ? 1n : 0n;
      }
      coprocDbWriteCleartext(handle, clearText);
      break;
    }

    case 'FheNe': {
      const lhs: string = getStringEventArg(event, 1);
      const rhs: string = getStringEventArg(event, 2);
      const scalarByte: string = getStringEventArg(event, 3);
      const result: string = getStringEventArg(event, 4);

      handle = ethers.toBeHex(result, 32);
      clearLHS = await coprocDbReadCleartext(lhs);
      if (scalarByte === '0x01') {
        clearText = BigInt(clearLHS) !== BigInt(rhs) ? 1n : 0n;
      } else {
        clearRHS = await coprocDbReadCleartext(rhs);
        clearText = BigInt(clearLHS) !== BigInt(clearRHS) ? 1n : 0n;
      }
      coprocDbWriteCleartext(handle, clearText);
      break;
    }

    case 'FheNeBytes': {
      const lhs: string = getStringEventArg(event, 1);
      const rhs: string = getStringEventArg(event, 2);
      const scalarByte: string = getStringEventArg(event, 3);
      const result: string = getStringEventArg(event, 4);

      handle = ethers.toBeHex(result, 32);
      clearLHS = await coprocDbReadCleartext(lhs);
      if (scalarByte === '0x01') {
        clearText = BigInt(clearLHS) !== BigInt(rhs) ? 1n : 0n;
      } else {
        clearRHS = await coprocDbReadCleartext(rhs);
        clearText = BigInt(clearLHS) !== BigInt(clearRHS) ? 1n : 0n;
      }
      coprocDbWriteCleartext(handle, clearText);
      break;
    }

    case 'FheGe': {
      const lhs: string = getStringEventArg(event, 1);
      const rhs: string = getStringEventArg(event, 2);
      const scalarByte: string = getStringEventArg(event, 3);
      const result: string = getStringEventArg(event, 4);

      handle = ethers.toBeHex(result, 32);
      clearLHS = await coprocDbReadCleartext(lhs);
      if (scalarByte === '0x01') {
        clearText = BigInt(clearLHS) >= BigInt(rhs) ? 1n : 0n;
      } else {
        clearRHS = await coprocDbReadCleartext(rhs);
        clearText = BigInt(clearLHS) >= BigInt(clearRHS) ? 1n : 0n;
      }
      coprocDbWriteCleartext(handle, clearText);
      break;
    }

    case 'FheGt': {
      const lhs: string = getStringEventArg(event, 1);
      const rhs: string = getStringEventArg(event, 2);
      const scalarByte: string = getStringEventArg(event, 3);
      const result: string = getStringEventArg(event, 4);

      handle = ethers.toBeHex(result, 32);
      clearLHS = await coprocDbReadCleartext(lhs);
      if (scalarByte === '0x01') {
        clearText = BigInt(clearLHS) > BigInt(rhs) ? 1n : 0n;
      } else {
        clearRHS = await coprocDbReadCleartext(rhs);
        clearText = BigInt(clearLHS) > BigInt(clearRHS) ? 1n : 0n;
      }
      coprocDbWriteCleartext(handle, clearText);
      break;
    }

    case 'FheLe': {
      const lhs: string = getStringEventArg(event, 1);
      const rhs: string = getStringEventArg(event, 2);
      const scalarByte: string = getStringEventArg(event, 3);
      const result: string = getStringEventArg(event, 4);

      handle = ethers.toBeHex(result, 32);
      clearLHS = await coprocDbReadCleartext(lhs);
      if (scalarByte === '0x01') {
        clearText = BigInt(clearLHS) <= BigInt(rhs) ? 1n : 0n;
      } else {
        clearRHS = await coprocDbReadCleartext(rhs);
        clearText = BigInt(clearLHS) <= BigInt(clearRHS) ? 1n : 0n;
      }
      coprocDbWriteCleartext(handle, clearText);
      break;
    }

    case 'FheLt': {
      const lhs: string = getStringEventArg(event, 1);
      const rhs: string = getStringEventArg(event, 2);
      const scalarByte: string = getStringEventArg(event, 3);
      const result: string = getStringEventArg(event, 4);

      handle = ethers.toBeHex(result, 32);
      clearLHS = await coprocDbReadCleartext(lhs);
      if (scalarByte === '0x01') {
        clearText = BigInt(clearLHS) < BigInt(rhs) ? 1n : 0n;
      } else {
        clearRHS = await coprocDbReadCleartext(rhs);
        clearText = BigInt(clearLHS) < BigInt(clearRHS) ? 1n : 0n;
      }
      coprocDbWriteCleartext(handle, clearText);
      break;
    }

    case 'FheMax': {
      const lhs: string = getStringEventArg(event, 1);
      const rhs: string = getStringEventArg(event, 2);
      const scalarByte: string = getStringEventArg(event, 3);
      const result: string = getStringEventArg(event, 4);

      handle = ethers.toBeHex(result, 32);
      clearLHS = await coprocDbReadCleartext(lhs);
      if (scalarByte === '0x01') {
        clearText = BigInt(clearLHS) > BigInt(rhs) ? clearLHS : BigInt(rhs);
      } else {
        clearRHS = await coprocDbReadCleartext(rhs);
        clearText = BigInt(clearLHS) > BigInt(clearRHS) ? clearLHS : clearRHS;
      }
      coprocDbWriteCleartext(handle, clearText);
      break;
    }

    case 'FheMin': {
      const lhs: string = getStringEventArg(event, 1);
      const rhs: string = getStringEventArg(event, 2);
      const scalarByte: string = getStringEventArg(event, 3);
      const result: string = getStringEventArg(event, 4);

      handle = ethers.toBeHex(result, 32);
      clearLHS = await coprocDbReadCleartext(lhs);
      if (scalarByte === '0x01') {
        clearText = BigInt(clearLHS) < BigInt(rhs) ? clearLHS : BigInt(rhs);
      } else {
        clearRHS = await coprocDbReadCleartext(rhs);
        clearText = BigInt(clearLHS) < BigInt(clearRHS) ? clearLHS : clearRHS;
      }
      coprocDbWriteCleartext(handle, clearText);
      break;
    }

    case 'Cast': {
      const ct: string = getStringEventArg(event, 1);
      const toType: bigint = getBigIntEventArg(event, 2);
      const result: string = getStringEventArg(event, 3);

      resultType = toNumBitsFheType(Number(toType));
      handle = ethers.toBeHex(result, 32);
      clearText = BigInt(await coprocDbReadCleartext(ct)) % 2n ** NumBits[resultType];
      coprocDbWriteCleartext(handle, clearText);
      break;
    }

    case 'FheNot': {
      const ct: string = getStringEventArg(event, 1);
      const result: string = getStringEventArg(event, 2);

      handle = ethers.toBeHex(result, 32);
      resultType = getHandleFheType(handle);
      clearText = BigInt(await coprocDbReadCleartext(ct));
      clearText = bitwiseNotUintBits(clearText, Number(NumBits[resultType]));
      coprocDbWriteCleartext(handle, clearText);
      break;
    }

    case 'FheNeg': {
      const ct: string = getStringEventArg(event, 1);
      const result: string = getStringEventArg(event, 2);

      handle = ethers.toBeHex(result, 32);
      resultType = getHandleFheType(handle);
      clearText = BigInt(await coprocDbReadCleartext(ct));
      clearText = bitwiseNotUintBits(clearText, Number(NumBits[resultType]));
      clearText = (clearText + 1n) % 2n ** NumBits[resultType];
      coprocDbWriteCleartext(handle, clearText);
      break;
    }

    case 'VerifyInput': {
      const inputHandle: string = getStringEventArg(event, 1);

      // Clear values are stored keyed by the 0x-prefixed 32-byte hex handle.
      handle = ethers.toBeHex(inputHandle, 32);
      try {
        await coprocDbReadCleartext(handle);
      } catch {
        throw Error('User input was not found in DB');
      }
      break;
    }

    case 'FheIfThenElse': {
      const control: string = getStringEventArg(event, 1);
      const ifTrue: string = getStringEventArg(event, 2);
      const ifFalse: string = getStringEventArg(event, 3);
      const result: string = getStringEventArg(event, 4);

      handle = ethers.toBeHex(result, 32);

      const clearControl = BigInt(await coprocDbReadCleartext(control));
      const clearIfTrue = BigInt(await coprocDbReadCleartext(ifTrue));
      const clearIfFalse = BigInt(await coprocDbReadCleartext(ifFalse));

      if (clearControl === 1n) {
        clearText = clearIfTrue;
      } else {
        clearText = clearIfFalse;
      }

      coprocDbWriteCleartext(handle, clearText);
      break;
    }

    case 'FheRand': {
      const randType: bigint = getBigIntEventArg(event, 1);
      const result: string = getStringEventArg(event, 3);

      resultType = toNumBitsFheType(Number(randType));
      handle = ethers.toBeHex(result, 32);
      clearText = getRandomBigInt(Number(NumBits[resultType]));
      coprocDbWriteCleartext(handle, clearText, true);
      break;
    }

    case 'FheRandBounded': {
      const upperBound: bigint = getBigIntEventArg(event, 1);
      const result: string = getStringEventArg(event, 4);

      handle = ethers.toBeHex(result, 32);
      clearText = getRandomBigInt(Number(log2(upperBound)));
      coprocDbWriteCleartext(handle, clearText, true);
      break;
    }

    case 'FheSum': {
      const values: string[] = getStringArrayEventArg(event, 1);
      const result: string = getStringEventArg(event, 2);

      handle = ethers.toBeHex(result, 32);
      resultType = getHandleFheType(handle);
      clearText = 0n;
      for (const valueHandle of values) {
        clearText = clearText + BigInt(await coprocDbReadCleartext(ethers.toBeHex(valueHandle, 32)));
      }
      clearText = clearText % 2n ** NumBits[resultType];
      coprocDbWriteCleartext(handle, clearText);
      break;
    }

    case 'FheIsIn': {
      const value: string = getStringEventArg(event, 1);
      const values: string[] = getStringArrayEventArg(event, 2);
      const result: string = getStringEventArg(event, 3);

      handle = ethers.toBeHex(result, 32);
      const valueText = BigInt(await coprocDbReadCleartext(ethers.toBeHex(value, 32)));
      const setClearTexts = await Promise.all(
        values.map((h) => coprocDbReadCleartext(ethers.toBeHex(h, 32)).then(BigInt)),
      );
      clearText = setClearTexts.some((s) => s === valueText) ? 1n : 0n;
      coprocDbWriteCleartext(handle, clearText);
      break;
    }

    case 'FheMulDiv': {
      const factor1: string = getStringEventArg(event, 1);
      const factor2: string = getStringEventArg(event, 2);
      const divisor: string = getStringEventArg(event, 3);
      const scalarByte: string = getStringEventArg(event, 4);
      const result: string = getStringEventArg(event, 5);

      handle = ethers.toBeHex(result, 32);
      resultType = getHandleFheType(handle);
      const clearFactor1 = await coprocDbReadCleartext(factor1);
      const clearDivisor = BigInt(divisor);
      const factor2IsScalar = (BigInt(scalarByte) & 0x02n) !== 0n;
      if (factor2IsScalar) {
        clearText = (BigInt(clearFactor1) * BigInt(factor2)) / clearDivisor;
      } else {
        const clearFactor2 = await coprocDbReadCleartext(factor2);
        clearText = (BigInt(clearFactor1) * BigInt(clearFactor2)) / clearDivisor;
      }
      clearText = clearText % 2n ** NumBits[resultType];
      coprocDbWriteCleartext(handle, clearText);
      break;
    }
  }
}

// Returns the name of the FHE type whose index is `typeIndex`, throwing if there is none.
function getFheTypeName(fheTypeInfos: FheTypeInfo[], typeIndex: number): string {
  const type = fheTypeInfos.find((t) => t.value === typeIndex)?.type;
  if (type === undefined) {
    throw new Error(`Invalid FheType index: ${typeIndex}`);
  }
  return type;
}

export function getTxHCUFromTxReceipt(
  receipt: EthersT.TransactionReceipt,
  fheTypeInfos: FheTypeInfo[] = ALL_FHE_TYPE_INFOS,
): {
  globalTxHCU: number;
  maxTxHCUDepth: number;
  HCUDepthPerHandle: Record<string, number>;
} {
  if (receipt.status === 0) {
    throw new Error('Transaction reverted');
  }

  // Returns the HCU depth recorded for `handle`, or 0 if none was recorded.
  function readFromHCUMap(handle: string): number {
    return handle in hcuMap ? hcuMap[handle] : 0;
  }

  const hcuMap: Record<string, number> = {};
  const handleSet: Set<string> = new Set<string>();

  // FHEVMExecutor events emitted in the transaction (VerifyInput consumes no HCU, so it is skipped).
  const contract = new ethers.Contract(coprocAddress, abi, ethers.provider);
  const fhevmEvents: FHEVMEvent[] = [];
  for (const log of receipt.logs) {
    if (log.address.toLowerCase() !== coprocAddress.toLowerCase()) {
      continue;
    }
    const event = parseFHEVMEvent(contract.interface, log);
    if (event === undefined || event.eventName === 'VerifyInput') {
      continue;
    }
    fhevmEvents.push(event);
  }

  let totalHCUConsumed = 0;

  for (const event of fhevmEvents) {
    let type: string;
    let typeIndex: number;
    let handle: string;
    let handleResult: string;
    let hcuConsumed: number;

    switch (event.eventName) {
      case 'TrivialEncrypt': {
        const toType: bigint = getBigIntEventArg(event, 2);
        const result: string = getStringEventArg(event, 3);

        typeIndex = Number(toType);
        type = getFheTypeName(fheTypeInfos, typeIndex);

        hcuConsumed = (ALL_OPERATORS_PRICES.trivialEncrypt.types as Record<string, number>)[type];
        totalHCUConsumed += hcuConsumed;
        handleResult = ethers.toBeHex(result, 32);
        hcuMap[handleResult] = hcuConsumed;
        handleSet.add(handleResult);
        break;
      }

      case 'FheAdd': {
        const lhs: string = getStringEventArg(event, 1);
        const rhs: string = getStringEventArg(event, 2);
        const scalarByte: string = getStringEventArg(event, 3);
        const result: string = getStringEventArg(event, 4);

        handleResult = ethers.toBeHex(result, 32);
        typeIndex = getHandleTypeIndex(handleResult);
        type = getFheTypeName(fheTypeInfos, typeIndex);

        if (scalarByte === '0x01') {
          hcuConsumed = (ALL_OPERATORS_PRICES.fheAdd.scalar as Record<string, number>)[type];
          hcuMap[handleResult] = hcuConsumed + readFromHCUMap(ethers.toBeHex(lhs, 32));
        } else {
          hcuConsumed = (ALL_OPERATORS_PRICES.fheAdd.nonScalar as Record<string, number>)[type];
          hcuMap[handleResult] =
            hcuConsumed + Math.max(readFromHCUMap(ethers.toBeHex(lhs, 32)), readFromHCUMap(ethers.toBeHex(rhs, 32)));
        }

        handleSet.add(handleResult);
        totalHCUConsumed += hcuConsumed;
        break;
      }

      case 'FheSub': {
        const lhs: string = getStringEventArg(event, 1);
        const rhs: string = getStringEventArg(event, 2);
        const scalarByte: string = getStringEventArg(event, 3);
        const result: string = getStringEventArg(event, 4);

        handleResult = ethers.toBeHex(result, 32);
        typeIndex = getHandleTypeIndex(handleResult);
        type = getFheTypeName(fheTypeInfos, typeIndex);

        if (scalarByte === '0x01') {
          hcuConsumed = (ALL_OPERATORS_PRICES.fheSub.scalar as Record<string, number>)[type];
          hcuMap[handleResult] = hcuConsumed + readFromHCUMap(ethers.toBeHex(lhs, 32));
        } else {
          hcuConsumed = (ALL_OPERATORS_PRICES.fheSub.nonScalar as Record<string, number>)[type];
          hcuMap[handleResult] =
            hcuConsumed + Math.max(readFromHCUMap(ethers.toBeHex(lhs, 32)), readFromHCUMap(ethers.toBeHex(rhs, 32)));
        }

        handleSet.add(handleResult);
        totalHCUConsumed += hcuConsumed;
        break;
      }

      case 'FheMul': {
        const lhs: string = getStringEventArg(event, 1);
        const rhs: string = getStringEventArg(event, 2);
        const scalarByte: string = getStringEventArg(event, 3);
        const result: string = getStringEventArg(event, 4);

        handleResult = ethers.toBeHex(result, 32);
        typeIndex = getHandleTypeIndex(handleResult);
        type = getFheTypeName(fheTypeInfos, typeIndex);

        if (scalarByte === '0x01') {
          hcuConsumed = (ALL_OPERATORS_PRICES.fheMul.scalar as Record<string, number>)[type];
          hcuMap[handleResult] = hcuConsumed + readFromHCUMap(ethers.toBeHex(lhs, 32));
        } else {
          hcuConsumed = (ALL_OPERATORS_PRICES.fheMul.nonScalar as Record<string, number>)[type];
          hcuMap[handleResult] =
            hcuConsumed + Math.max(readFromHCUMap(ethers.toBeHex(lhs, 32)), readFromHCUMap(ethers.toBeHex(rhs, 32)));
        }

        handleSet.add(handleResult);
        totalHCUConsumed += hcuConsumed;
        break;
      }

      case 'FheDiv': {
        const lhs: string = getStringEventArg(event, 1);
        const scalarByte: string = getStringEventArg(event, 3);
        const result: string = getStringEventArg(event, 4);

        handleResult = ethers.toBeHex(result, 32);
        typeIndex = getHandleTypeIndex(handleResult);
        type = getFheTypeName(fheTypeInfos, typeIndex);
        if (scalarByte === '0x01') {
          hcuConsumed = (ALL_OPERATORS_PRICES.fheDiv.scalar as Record<string, number>)[type];
          hcuMap[handleResult] = hcuConsumed + readFromHCUMap(ethers.toBeHex(lhs, 32));
        } else {
          throw new Error('Non-scalar div not implemented yet');
        }

        handleSet.add(handleResult);
        totalHCUConsumed += hcuConsumed;
        break;
      }

      case 'FheRem': {
        const lhs: string = getStringEventArg(event, 1);
        const scalarByte: string = getStringEventArg(event, 3);
        const result: string = getStringEventArg(event, 4);

        handleResult = ethers.toBeHex(result, 32);
        typeIndex = getHandleTypeIndex(handleResult);
        type = getFheTypeName(fheTypeInfos, typeIndex);
        if (scalarByte === '0x01') {
          hcuConsumed = (ALL_OPERATORS_PRICES.fheRem.scalar as Record<string, number>)[type];
          hcuMap[handleResult] = hcuConsumed + readFromHCUMap(ethers.toBeHex(lhs, 32));
        } else {
          throw new Error('Non-scalar rem not implemented yet');
        }
        handleSet.add(handleResult);
        totalHCUConsumed += hcuConsumed;
        break;
      }

      case 'FheBitAnd': {
        const lhs: string = getStringEventArg(event, 1);
        const rhs: string = getStringEventArg(event, 2);
        const scalarByte: string = getStringEventArg(event, 3);
        const result: string = getStringEventArg(event, 4);

        handleResult = ethers.toBeHex(result, 32);
        typeIndex = getHandleTypeIndex(handleResult);
        type = getFheTypeName(fheTypeInfos, typeIndex);
        if (scalarByte === '0x01') {
          hcuConsumed = (ALL_OPERATORS_PRICES.fheBitAnd.scalar as Record<string, number>)[type];
          hcuMap[handleResult] = hcuConsumed + readFromHCUMap(ethers.toBeHex(lhs, 32));
        } else {
          hcuConsumed = (ALL_OPERATORS_PRICES.fheBitAnd.nonScalar as Record<string, number>)[type];
          hcuMap[handleResult] =
            hcuConsumed + Math.max(readFromHCUMap(ethers.toBeHex(lhs, 32)), readFromHCUMap(ethers.toBeHex(rhs, 32)));
        }
        handleSet.add(handleResult);
        totalHCUConsumed += hcuConsumed;
        break;
      }

      case 'FheBitOr': {
        const lhs: string = getStringEventArg(event, 1);
        const rhs: string = getStringEventArg(event, 2);
        const scalarByte: string = getStringEventArg(event, 3);
        const result: string = getStringEventArg(event, 4);

        handleResult = ethers.toBeHex(result, 32);
        typeIndex = getHandleTypeIndex(handleResult);
        type = getFheTypeName(fheTypeInfos, typeIndex);
        if (scalarByte === '0x01') {
          hcuConsumed = (ALL_OPERATORS_PRICES.fheBitOr.scalar as Record<string, number>)[type];
          hcuMap[handleResult] = hcuConsumed + readFromHCUMap(ethers.toBeHex(lhs, 32));
        } else {
          hcuConsumed = (ALL_OPERATORS_PRICES.fheBitOr.nonScalar as Record<string, number>)[type];
          hcuMap[handleResult] =
            hcuConsumed + Math.max(readFromHCUMap(ethers.toBeHex(lhs, 32)), readFromHCUMap(ethers.toBeHex(rhs, 32)));
        }
        handleSet.add(handleResult);
        totalHCUConsumed += hcuConsumed;
        break;
      }

      case 'FheBitXor': {
        const lhs: string = getStringEventArg(event, 1);
        const rhs: string = getStringEventArg(event, 2);
        const scalarByte: string = getStringEventArg(event, 3);
        const result: string = getStringEventArg(event, 4);

        handleResult = ethers.toBeHex(result, 32);
        typeIndex = getHandleTypeIndex(handleResult);
        type = getFheTypeName(fheTypeInfos, typeIndex);
        if (scalarByte === '0x01') {
          hcuConsumed = (ALL_OPERATORS_PRICES.fheBitXor.scalar as Record<string, number>)[type];
          hcuMap[handleResult] = hcuConsumed + readFromHCUMap(ethers.toBeHex(lhs, 32));
        } else {
          hcuConsumed = (ALL_OPERATORS_PRICES.fheBitXor.nonScalar as Record<string, number>)[type];
          hcuMap[handleResult] =
            hcuConsumed + Math.max(readFromHCUMap(ethers.toBeHex(lhs, 32)), readFromHCUMap(ethers.toBeHex(rhs, 32)));
        }
        handleSet.add(handleResult);
        totalHCUConsumed += hcuConsumed;
        break;
      }

      case 'FheShl': {
        const lhs: string = getStringEventArg(event, 1);
        const rhs: string = getStringEventArg(event, 2);
        const scalarByte: string = getStringEventArg(event, 3);
        const result: string = getStringEventArg(event, 4);

        handleResult = ethers.toBeHex(result, 32);
        typeIndex = getHandleTypeIndex(handleResult);
        type = getFheTypeName(fheTypeInfos, typeIndex);
        if (scalarByte === '0x01') {
          hcuConsumed = (ALL_OPERATORS_PRICES.fheShl.scalar as Record<string, number>)[type];
          hcuMap[handleResult] = hcuConsumed + readFromHCUMap(ethers.toBeHex(lhs, 32));
        } else {
          hcuConsumed = (ALL_OPERATORS_PRICES.fheShl.nonScalar as Record<string, number>)[type];
          hcuMap[handleResult] =
            hcuConsumed + Math.max(readFromHCUMap(ethers.toBeHex(lhs, 32)), readFromHCUMap(ethers.toBeHex(rhs, 32)));
        }
        handleSet.add(handleResult);
        totalHCUConsumed += hcuConsumed;
        break;
      }

      case 'FheShr': {
        const lhs: string = getStringEventArg(event, 1);
        const rhs: string = getStringEventArg(event, 2);
        const scalarByte: string = getStringEventArg(event, 3);
        const result: string = getStringEventArg(event, 4);

        handleResult = ethers.toBeHex(result, 32);
        typeIndex = getHandleTypeIndex(handleResult);
        type = getFheTypeName(fheTypeInfos, typeIndex);
        if (scalarByte === '0x01') {
          hcuConsumed = (ALL_OPERATORS_PRICES.fheShr.scalar as Record<string, number>)[type];
          hcuMap[handleResult] = hcuConsumed + readFromHCUMap(ethers.toBeHex(lhs, 32));
        } else {
          hcuConsumed = (ALL_OPERATORS_PRICES.fheShr.nonScalar as Record<string, number>)[type];
          hcuMap[handleResult] =
            hcuConsumed + Math.max(readFromHCUMap(ethers.toBeHex(lhs, 32)), readFromHCUMap(ethers.toBeHex(rhs, 32)));
        }
        handleSet.add(handleResult);
        totalHCUConsumed += hcuConsumed;
        break;
      }

      case 'FheRotl': {
        const lhs: string = getStringEventArg(event, 1);
        const rhs: string = getStringEventArg(event, 2);
        const scalarByte: string = getStringEventArg(event, 3);
        const result: string = getStringEventArg(event, 4);

        handleResult = ethers.toBeHex(result, 32);
        typeIndex = getHandleTypeIndex(handleResult);
        type = getFheTypeName(fheTypeInfos, typeIndex);
        if (scalarByte === '0x01') {
          hcuConsumed = (ALL_OPERATORS_PRICES.fheRotl.scalar as Record<string, number>)[type];
          hcuMap[handleResult] = hcuConsumed + readFromHCUMap(ethers.toBeHex(lhs, 32));
        } else {
          hcuConsumed = (ALL_OPERATORS_PRICES.fheRotl.nonScalar as Record<string, number>)[type];
          hcuMap[handleResult] =
            hcuConsumed + Math.max(readFromHCUMap(ethers.toBeHex(lhs, 32)), readFromHCUMap(ethers.toBeHex(rhs, 32)));
        }
        handleSet.add(handleResult);
        totalHCUConsumed += hcuConsumed;
        break;
      }

      case 'FheRotr': {
        const lhs: string = getStringEventArg(event, 1);
        const rhs: string = getStringEventArg(event, 2);
        const scalarByte: string = getStringEventArg(event, 3);
        const result: string = getStringEventArg(event, 4);

        handleResult = ethers.toBeHex(result, 32);
        typeIndex = getHandleTypeIndex(handleResult);
        type = getFheTypeName(fheTypeInfos, typeIndex);
        if (scalarByte === '0x01') {
          hcuConsumed = (ALL_OPERATORS_PRICES.fheRotr.scalar as Record<string, number>)[type];
          hcuMap[handleResult] = hcuConsumed + readFromHCUMap(ethers.toBeHex(lhs, 32));
        } else {
          hcuConsumed = (ALL_OPERATORS_PRICES.fheRotr.nonScalar as Record<string, number>)[type];
          hcuMap[handleResult] =
            hcuConsumed + Math.max(readFromHCUMap(ethers.toBeHex(lhs, 32)), readFromHCUMap(ethers.toBeHex(rhs, 32)));
        }
        handleSet.add(handleResult);
        totalHCUConsumed += hcuConsumed;
        break;
      }

      case 'FheEq': {
        const lhs: string = getStringEventArg(event, 1);
        const rhs: string = getStringEventArg(event, 2);
        const scalarByte: string = getStringEventArg(event, 3);
        const result: string = getStringEventArg(event, 4);

        handleResult = ethers.toBeHex(result, 32);
        handle = ethers.toBeHex(lhs, 32);
        typeIndex = getHandleTypeIndex(handle);
        type = getFheTypeName(fheTypeInfos, typeIndex);

        if (scalarByte === '0x01') {
          hcuConsumed = (ALL_OPERATORS_PRICES.fheEq.scalar as Record<string, number>)[type];
          hcuMap[handleResult] = hcuConsumed + readFromHCUMap(ethers.toBeHex(lhs, 32));
        } else {
          hcuConsumed = (ALL_OPERATORS_PRICES.fheEq.nonScalar as Record<string, number>)[type];
          hcuMap[handleResult] =
            hcuConsumed + Math.max(readFromHCUMap(ethers.toBeHex(lhs, 32)), readFromHCUMap(ethers.toBeHex(rhs, 32)));
        }
        handleSet.add(handleResult);
        totalHCUConsumed += hcuConsumed;
        break;
      }

      case 'FheEqBytes': {
        const lhs: string = getStringEventArg(event, 1);
        const rhs: string = getStringEventArg(event, 2);
        const scalarByte: string = getStringEventArg(event, 3);
        const result: string = getStringEventArg(event, 4);

        handleResult = ethers.toBeHex(result, 32);
        handle = ethers.toBeHex(lhs, 32);
        typeIndex = getHandleTypeIndex(handle);
        type = getFheTypeName(fheTypeInfos, typeIndex);

        if (scalarByte === '0x01') {
          hcuConsumed = (ALL_OPERATORS_PRICES.fheEq.scalar as Record<string, number>)[type];
          hcuMap[handleResult] = hcuConsumed + readFromHCUMap(ethers.toBeHex(lhs, 32));
        } else {
          hcuConsumed = (ALL_OPERATORS_PRICES.fheEq.nonScalar as Record<string, number>)[type];
          hcuMap[handleResult] =
            hcuConsumed + Math.max(readFromHCUMap(ethers.toBeHex(lhs, 32)), readFromHCUMap(ethers.toBeHex(rhs, 32)));
        }
        handleSet.add(handleResult);
        totalHCUConsumed += hcuConsumed;
        break;
      }

      case 'FheNe': {
        const lhs: string = getStringEventArg(event, 1);
        const rhs: string = getStringEventArg(event, 2);
        const scalarByte: string = getStringEventArg(event, 3);
        const result: string = getStringEventArg(event, 4);

        handleResult = ethers.toBeHex(result, 32);
        handle = ethers.toBeHex(lhs, 32);
        typeIndex = getHandleTypeIndex(handle);
        type = getFheTypeName(fheTypeInfos, typeIndex);

        if (scalarByte === '0x01') {
          hcuConsumed = (ALL_OPERATORS_PRICES.fheNe.scalar as Record<string, number>)[type];
          hcuMap[handleResult] = hcuConsumed + readFromHCUMap(ethers.toBeHex(lhs, 32));
        } else {
          hcuConsumed = (ALL_OPERATORS_PRICES.fheNe.nonScalar as Record<string, number>)[type];
          hcuMap[handleResult] =
            hcuConsumed + Math.max(readFromHCUMap(ethers.toBeHex(lhs, 32)), readFromHCUMap(ethers.toBeHex(rhs, 32)));
        }
        handleSet.add(handleResult);
        totalHCUConsumed += hcuConsumed;
        break;
      }

      case 'FheNeBytes': {
        const lhs: string = getStringEventArg(event, 1);
        const rhs: string = getStringEventArg(event, 2);
        const scalarByte: string = getStringEventArg(event, 3);
        const result: string = getStringEventArg(event, 4);

        handleResult = ethers.toBeHex(result, 32);
        handle = ethers.toBeHex(lhs, 32);
        typeIndex = getHandleTypeIndex(handle);
        type = getFheTypeName(fheTypeInfos, typeIndex);

        if (scalarByte === '0x01') {
          hcuConsumed = (ALL_OPERATORS_PRICES.fheNe.scalar as Record<string, number>)[type];
          hcuMap[handleResult] = hcuConsumed + readFromHCUMap(ethers.toBeHex(lhs, 32));
        } else {
          hcuConsumed = (ALL_OPERATORS_PRICES.fheNe.nonScalar as Record<string, number>)[type];
          hcuMap[handleResult] =
            hcuConsumed + Math.max(readFromHCUMap(ethers.toBeHex(lhs, 32)), readFromHCUMap(ethers.toBeHex(rhs, 32)));
        }
        handleSet.add(handleResult);
        totalHCUConsumed += hcuConsumed;
        break;
      }

      case 'FheGe': {
        const lhs: string = getStringEventArg(event, 1);
        const rhs: string = getStringEventArg(event, 2);
        const scalarByte: string = getStringEventArg(event, 3);
        const result: string = getStringEventArg(event, 4);

        handleResult = ethers.toBeHex(result, 32);
        handle = ethers.toBeHex(lhs, 32);
        typeIndex = getHandleTypeIndex(handle);
        type = getFheTypeName(fheTypeInfos, typeIndex);

        if (scalarByte === '0x01') {
          hcuConsumed = (ALL_OPERATORS_PRICES.fheGe.scalar as Record<string, number>)[type];
          hcuMap[handleResult] = hcuConsumed + readFromHCUMap(ethers.toBeHex(lhs, 32));
        } else {
          hcuConsumed = (ALL_OPERATORS_PRICES.fheGe.nonScalar as Record<string, number>)[type];
          hcuMap[handleResult] =
            hcuConsumed + Math.max(readFromHCUMap(ethers.toBeHex(lhs, 32)), readFromHCUMap(ethers.toBeHex(rhs, 32)));
        }
        handleSet.add(handleResult);
        totalHCUConsumed += hcuConsumed;
        break;
      }

      case 'FheGt': {
        const lhs: string = getStringEventArg(event, 1);
        const rhs: string = getStringEventArg(event, 2);
        const scalarByte: string = getStringEventArg(event, 3);
        const result: string = getStringEventArg(event, 4);

        handleResult = ethers.toBeHex(result, 32);
        handle = ethers.toBeHex(lhs, 32);
        typeIndex = getHandleTypeIndex(handle);
        type = getFheTypeName(fheTypeInfos, typeIndex);
        if (scalarByte === '0x01') {
          hcuConsumed = (ALL_OPERATORS_PRICES.fheGt.scalar as Record<string, number>)[type];
          hcuMap[handleResult] = hcuConsumed + readFromHCUMap(ethers.toBeHex(lhs, 32));
        } else {
          hcuConsumed = (ALL_OPERATORS_PRICES.fheGt.nonScalar as Record<string, number>)[type];
          hcuMap[handleResult] =
            hcuConsumed + Math.max(readFromHCUMap(ethers.toBeHex(lhs, 32)), readFromHCUMap(ethers.toBeHex(rhs, 32)));
        }
        handleSet.add(handleResult);
        totalHCUConsumed += hcuConsumed;
        break;
      }

      case 'FheLe': {
        const lhs: string = getStringEventArg(event, 1);
        const rhs: string = getStringEventArg(event, 2);
        const scalarByte: string = getStringEventArg(event, 3);
        const result: string = getStringEventArg(event, 4);

        handleResult = ethers.toBeHex(result, 32);
        handle = ethers.toBeHex(lhs, 32);
        typeIndex = getHandleTypeIndex(handle);
        type = getFheTypeName(fheTypeInfos, typeIndex);
        if (scalarByte === '0x01') {
          hcuConsumed = (ALL_OPERATORS_PRICES.fheLe.scalar as Record<string, number>)[type];
          hcuMap[handleResult] = hcuConsumed + readFromHCUMap(ethers.toBeHex(lhs, 32));
        } else {
          hcuConsumed = (ALL_OPERATORS_PRICES.fheLe.nonScalar as Record<string, number>)[type];
          hcuMap[handleResult] =
            hcuConsumed + Math.max(readFromHCUMap(ethers.toBeHex(lhs, 32)), readFromHCUMap(ethers.toBeHex(rhs, 32)));
        }
        handleSet.add(handleResult);
        totalHCUConsumed += hcuConsumed;
        break;
      }

      case 'FheLt': {
        const lhs: string = getStringEventArg(event, 1);
        const rhs: string = getStringEventArg(event, 2);
        const scalarByte: string = getStringEventArg(event, 3);
        const result: string = getStringEventArg(event, 4);

        handleResult = ethers.toBeHex(result, 32);
        handle = ethers.toBeHex(lhs, 32);
        typeIndex = getHandleTypeIndex(handle);
        type = getFheTypeName(fheTypeInfos, typeIndex);
        if (scalarByte === '0x01') {
          hcuConsumed = (ALL_OPERATORS_PRICES.fheLt.scalar as Record<string, number>)[type];
          hcuMap[handleResult] = hcuConsumed + readFromHCUMap(ethers.toBeHex(lhs, 32));
        } else {
          hcuConsumed = (ALL_OPERATORS_PRICES.fheLt.nonScalar as Record<string, number>)[type];
          hcuMap[handleResult] =
            hcuConsumed + Math.max(readFromHCUMap(ethers.toBeHex(lhs, 32)), readFromHCUMap(ethers.toBeHex(rhs, 32)));
        }
        handleSet.add(handleResult);
        totalHCUConsumed += hcuConsumed;
        break;
      }

      case 'FheMax': {
        const lhs: string = getStringEventArg(event, 1);
        const rhs: string = getStringEventArg(event, 2);
        const scalarByte: string = getStringEventArg(event, 3);
        const result: string = getStringEventArg(event, 4);

        handleResult = ethers.toBeHex(result, 32);
        typeIndex = getHandleTypeIndex(handleResult);
        type = getFheTypeName(fheTypeInfos, typeIndex);

        if (scalarByte === '0x01') {
          hcuConsumed = (ALL_OPERATORS_PRICES.fheMax.scalar as Record<string, number>)[type];
          hcuMap[handleResult] = hcuConsumed + readFromHCUMap(ethers.toBeHex(lhs, 32));
        } else {
          hcuConsumed = (ALL_OPERATORS_PRICES.fheMax.nonScalar as Record<string, number>)[type];
          hcuMap[handleResult] =
            hcuConsumed + Math.max(readFromHCUMap(ethers.toBeHex(lhs, 32)), readFromHCUMap(ethers.toBeHex(rhs, 32)));
        }
        handleSet.add(handleResult);
        totalHCUConsumed += hcuConsumed;
        break;
      }

      case 'FheMin': {
        const lhs: string = getStringEventArg(event, 1);
        const rhs: string = getStringEventArg(event, 2);
        const scalarByte: string = getStringEventArg(event, 3);
        const result: string = getStringEventArg(event, 4);

        handleResult = ethers.toBeHex(result, 32);
        typeIndex = getHandleTypeIndex(handleResult);
        type = getFheTypeName(fheTypeInfos, typeIndex);

        if (scalarByte === '0x01') {
          hcuConsumed = (ALL_OPERATORS_PRICES.fheMin.scalar as Record<string, number>)[type];
          hcuMap[handleResult] = hcuConsumed + readFromHCUMap(ethers.toBeHex(lhs, 32));
        } else {
          hcuConsumed = (ALL_OPERATORS_PRICES.fheMin.nonScalar as Record<string, number>)[type];
          hcuMap[handleResult] =
            hcuConsumed + Math.max(readFromHCUMap(ethers.toBeHex(lhs, 32)), readFromHCUMap(ethers.toBeHex(rhs, 32)));
        }

        handleSet.add(handleResult);
        totalHCUConsumed += hcuConsumed;
        break;
      }

      case 'Cast': {
        const ct: string = getStringEventArg(event, 1);
        const result: string = getStringEventArg(event, 3);

        handle = ethers.toBeHex(ct, 32);
        typeIndex = getHandleTypeIndex(handle);
        type = getFheTypeName(fheTypeInfos, typeIndex);
        handleResult = ethers.toBeHex(result, 32);

        hcuConsumed = (ALL_OPERATORS_PRICES.cast.types as Record<string, number>)[type];
        hcuMap[handleResult] = hcuConsumed + readFromHCUMap(handle);
        totalHCUConsumed += hcuConsumed;
        handleSet.add(handleResult);
        break;
      }

      case 'FheNot': {
        const ct: string = getStringEventArg(event, 1);
        const result: string = getStringEventArg(event, 2);

        handle = ethers.toBeHex(ct, 32);
        typeIndex = getHandleTypeIndex(handle);
        type = getFheTypeName(fheTypeInfos, typeIndex);
        handleResult = ethers.toBeHex(result, 32);
        hcuConsumed = (ALL_OPERATORS_PRICES.fheNot.types as Record<string, number>)[type];
        hcuMap[handleResult] = hcuConsumed + readFromHCUMap(handle);
        totalHCUConsumed += hcuConsumed;
        handleSet.add(ethers.toBeHex(result, 32));
        break;
      }

      case 'FheNeg': {
        const ct: string = getStringEventArg(event, 1);
        const result: string = getStringEventArg(event, 2);

        handle = ethers.toBeHex(ct, 32);
        typeIndex = getHandleTypeIndex(handle);
        type = getFheTypeName(fheTypeInfos, typeIndex);
        handleResult = ethers.toBeHex(result, 32);
        hcuConsumed = (ALL_OPERATORS_PRICES.fheNeg.types as Record<string, number>)[type];
        hcuMap[handleResult] = hcuConsumed + readFromHCUMap(handle);
        totalHCUConsumed += hcuConsumed;
        handleSet.add(ethers.toBeHex(result, 32));
        break;
      }

      case 'FheIfThenElse': {
        const control: string = getStringEventArg(event, 1);
        const ifTrue: string = getStringEventArg(event, 2);
        const ifFalse: string = getStringEventArg(event, 3);
        const result: string = getStringEventArg(event, 4);

        handleResult = ethers.toBeHex(result, 32);
        typeIndex = getHandleTypeIndex(handleResult);
        type = getFheTypeName(fheTypeInfos, typeIndex);

        hcuConsumed = (ALL_OPERATORS_PRICES.ifThenElse.types as Record<string, number>)[type];
        hcuMap[handleResult] =
          hcuConsumed +
          Math.max(
            readFromHCUMap(ethers.toBeHex(control, 32)),
            readFromHCUMap(ethers.toBeHex(ifTrue, 32)),
            readFromHCUMap(ethers.toBeHex(ifFalse, 32)),
          );
        handleSet.add(handleResult);
        totalHCUConsumed += hcuConsumed;
        break;
      }

      case 'FheRand': {
        const randType: bigint = getBigIntEventArg(event, 1);
        const result: string = getStringEventArg(event, 3);

        handleResult = ethers.toBeHex(result, 32);
        typeIndex = Number(randType);
        type = getFheTypeName(fheTypeInfos, typeIndex);
        hcuConsumed = (ALL_OPERATORS_PRICES.fheRand.types as Record<string, number>)[type];
        hcuMap[handleResult] = hcuConsumed;
        handleSet.add(handleResult);
        totalHCUConsumed += hcuConsumed;
        break;
      }

      case 'FheRandBounded': {
        const randType: bigint = getBigIntEventArg(event, 2);
        const result: string = getStringEventArg(event, 4);

        handleResult = ethers.toBeHex(result, 32);
        typeIndex = Number(randType);
        type = getFheTypeName(fheTypeInfos, typeIndex);
        hcuConsumed = (ALL_OPERATORS_PRICES.fheRandBounded.types as Record<string, number>)[type];
        hcuMap[handleResult] = hcuConsumed;
        handleSet.add(handleResult);
        totalHCUConsumed += hcuConsumed;
        break;
      }

      case 'FheSum': {
        const values: string[] = getStringArrayEventArg(event, 1);
        const result: string = getStringEventArg(event, 2);

        handleResult = ethers.toBeHex(result, 32);
        typeIndex = getHandleTypeIndex(handleResult);
        type = getFheTypeName(fheTypeInfos, typeIndex);
        const nBucketedPrices = (
          ALL_OPERATORS_PRICES.fheSum.nBucketed as Record<
            string,
            { le10: number; le30?: number; le60?: number; le100?: number }
          >
        )[type];
        const n = values.length;
        if (n <= 10) hcuConsumed = nBucketedPrices.le10;
        else if (n <= 30) hcuConsumed = nBucketedPrices.le30 ?? nBucketedPrices.le10;
        else if (n <= 60) hcuConsumed = nBucketedPrices.le60 ?? nBucketedPrices.le30 ?? nBucketedPrices.le10;
        else
          hcuConsumed = nBucketedPrices.le100 ?? nBucketedPrices.le60 ?? nBucketedPrices.le30 ?? nBucketedPrices.le10;
        const inputValues = values;
        const maxInputHCU =
          inputValues.length > 0 ? Math.max(...inputValues.map((v) => readFromHCUMap(ethers.toBeHex(v, 32)))) : 0;
        hcuMap[handleResult] = hcuConsumed + maxInputHCU;
        handleSet.add(handleResult);
        totalHCUConsumed += hcuConsumed;
        break;
      }

      case 'FheIsIn': {
        const value: string = getStringEventArg(event, 1);
        const values: string[] = getStringArrayEventArg(event, 2);
        const result: string = getStringEventArg(event, 3);

        handleResult = ethers.toBeHex(result, 32);
        const isInInputHandle = ethers.toBeHex(value, 32);
        typeIndex = getHandleTypeIndex(isInInputHandle);
        type = getFheTypeName(fheTypeInfos, typeIndex);
        const nBucketedIsInPrices = (
          ALL_OPERATORS_PRICES.fheIsIn.nBucketed as Record<
            string,
            { le10: number; le30?: number; le60?: number; le100?: number }
          >
        )[type];
        const setHandles = values;
        const setSize = setHandles.length;
        if (setSize <= 10) hcuConsumed = nBucketedIsInPrices.le10;
        else if (setSize <= 30) hcuConsumed = nBucketedIsInPrices.le30 ?? nBucketedIsInPrices.le10;
        else if (setSize <= 60)
          hcuConsumed = nBucketedIsInPrices.le60 ?? nBucketedIsInPrices.le30 ?? nBucketedIsInPrices.le10;
        else
          hcuConsumed =
            nBucketedIsInPrices.le100 ??
            nBucketedIsInPrices.le60 ??
            nBucketedIsInPrices.le30 ??
            nBucketedIsInPrices.le10;
        const valueHandle = ethers.toBeHex(value, 32);
        const allInputHandles = [valueHandle, ...setHandles.map((h) => ethers.toBeHex(h, 32))];
        const maxInputHCU = Math.max(...allInputHandles.map((h) => readFromHCUMap(h)));
        hcuMap[handleResult] = hcuConsumed + maxInputHCU;
        handleSet.add(handleResult);
        totalHCUConsumed += hcuConsumed;
        break;
      }

      case 'FheMulDiv': {
        const factor1: string = getStringEventArg(event, 1);
        const factor2: string = getStringEventArg(event, 2);
        const scalarByte: string = getStringEventArg(event, 4);
        const result: string = getStringEventArg(event, 5);

        handleResult = ethers.toBeHex(result, 32);
        typeIndex = getHandleTypeIndex(handleResult);
        type = getFheTypeName(fheTypeInfos, typeIndex);
        const rhsIsScalar = (BigInt(scalarByte) & 0x02n) !== 0n;
        if (rhsIsScalar) {
          hcuConsumed = (ALL_OPERATORS_PRICES.fheMulDiv.scalar as Record<string, number>)[type];
          hcuMap[handleResult] = hcuConsumed + readFromHCUMap(ethers.toBeHex(factor1, 32));
        } else {
          hcuConsumed = (ALL_OPERATORS_PRICES.fheMulDiv.nonScalar as Record<string, number>)[type];
          hcuMap[handleResult] =
            hcuConsumed +
            Math.max(readFromHCUMap(ethers.toBeHex(factor1, 32)), readFromHCUMap(ethers.toBeHex(factor2, 32)));
        }
        handleSet.add(handleResult);
        totalHCUConsumed += hcuConsumed;
        break;
      }
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
