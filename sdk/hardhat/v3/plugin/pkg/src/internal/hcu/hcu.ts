// HCU (Homomorphic Complexity Units) a transaction consumed: the price of every executor operator
// event in its receipt, plus the deepest dependency chain by handle. Ported from v2's walk (itself from
// @fhevm/mock-utils); viem hands the events NAMED arguments, so each family reads its fields by name.

import { HardhatPluginError } from 'hardhat/plugins';
import { type Hex, isHex, size } from 'viem';

import type { CoprocessorEvent, FhevmTransactionHCUInfo, FhevmTransactionReceipt } from '../../types.js';
import { PLUGIN_ID } from '../constants.js';
import type { FhevmContractWrapper } from '../contracts.js';
import { parseCoprocessorEvents } from '../events.js';
import { parseFhevmHandle } from '../fhevmHandle.js';
import { getFheTypeName, getFheTypeNameFromByte } from './fheTypeName.js';
import { type FheTypeName, type HCUOperator, getHCU, hcuPriceOf } from './prices.js';

type Args = Record<string, unknown>;

export function computeTransactionHCU(
  executor: FhevmContractWrapper,
  receipt: FhevmTransactionReceipt,
): FhevmTransactionHCUInfo {
  if (receipt.status === 'reverted' || receipt.status === 0) {
    throw new HardhatPluginError(PLUGIN_ID, 'Transaction reverted');
  }
  const depthByHandle: Record<Hex, number> = {};
  const depth = (handle: Hex): number => depthByHandle[handle] ?? 0;
  let total = 0;

  for (const event of parseCoprocessorEvents(executor, receipt.logs)) {
    const op = hcuPriceOf(event.eventName);
    if (op === undefined) continue;
    const { hcu, result, inputs } = priceEvent(event, op);
    total += hcu;
    depthByHandle[result] = hcu + Math.max(0, ...inputs.map(depth));
  }

  return {
    transactionHash: receipt.transactionHash ?? receipt.hash ?? '0x',
    globalHCU: total,
    maxHCUDepth: Math.max(0, ...Object.values(depthByHandle)),
    HCUDepthByHandle: depthByHandle,
  };
}

type PricedEvent = { readonly hcu: number; readonly result: Hex; readonly inputs: readonly Hex[] };

function priceEvent(event: CoprocessorEvent, op: HCUOperator): PricedEvent {
  const name = event.eventName;
  const args = event.args as Args;
  const result = bytes32(args, 'result', name);

  switch (name) {
    case 'TrivialEncrypt':
      return { hcu: typePrice(op, getFheTypeNameFromByte(uint(args, 'toType', name))), result, inputs: [] };
    case 'Cast': {
      const ct = bytes32(args, 'ct', name);
      return { hcu: typePrice(op, typeOf(ct)), result, inputs: [ct] };
    }
    case 'FheNot':
    case 'FheNeg': {
      const ct = bytes32(args, 'ct', name);
      return { hcu: typePrice(op, typeOf(ct)), result, inputs: [ct] };
    }
    case 'FheAdd':
    case 'FheSub':
    case 'FheMul':
    case 'FheBitAnd':
    case 'FheBitOr':
    case 'FheBitXor':
    case 'FheShl':
    case 'FheShr':
    case 'FheRotl':
    case 'FheRotr':
    case 'FheMax':
    case 'FheMin':
      return binary(args, name, op, result, 'result');
    // Comparisons yield an ebool: priced on the left operand's type.
    case 'FheEq':
    case 'FheNe':
    case 'FheGe':
    case 'FheGt':
    case 'FheLe':
    case 'FheLt':
      return binary(args, name, op, result, 'lhs');
    case 'FheDiv':
    case 'FheRem': {
      const lhs = bytes32(args, 'lhs', name);
      if (!isScalar(args, name)) throw new HardhatPluginError(PLUGIN_ID, `Non-scalar ${name} not implemented yet`);
      return { hcu: op.scalar?.[typeOf(result)] ?? 0, result, inputs: [lhs] };
    }
    case 'FheIfThenElse': {
      const inputs = [bytes32(args, 'control', name), bytes32(args, 'ifTrue', name), bytes32(args, 'ifFalse', name)];
      return { hcu: typePrice(op, typeOf(result)), result, inputs };
    }
    case 'FheRand':
      return { hcu: typePrice(op, getFheTypeNameFromByte(uint(args, 'randType', name))), result, inputs: [] };
    case 'FheRandBounded':
      return { hcu: typePrice(op, typeOf(result)), result, inputs: [] };
    case 'FheSum': {
      const values = bytes32Array(args, 'values', name);
      return { hcu: getHCU(name, typeOf(result), { n: values.length }), result, inputs: values };
    }
    case 'FheIsIn': {
      const value = bytes32(args, 'value', name);
      const values = bytes32Array(args, 'values', name);
      return { hcu: getHCU(name, typeOf(value), { n: values.length }), result, inputs: [value, ...values] };
    }
    case 'VerifyInput':
      return { hcu: 0, result, inputs: [] };
  }
}

// event <Name>(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result)
function binary(args: Args, name: string, op: HCUOperator, result: Hex, pricedOn: 'result' | 'lhs'): PricedEvent {
  const lhs = bytes32(args, 'lhs', name);
  const rhs = bytes32(args, 'rhs', name);
  const scalar = isScalar(args, name);
  const column = scalar ? op.scalar : op.nonScalar;
  const hcu = column?.[typeOf(pricedOn === 'lhs' ? lhs : result)] ?? 0;
  // A scalar right operand has no depth of its own.
  return { hcu, result, inputs: scalar ? [lhs] : [lhs, rhs] };
}

function typePrice(op: HCUOperator, type: FheTypeName): number {
  return op.types?.[type] ?? 0;
}

function typeOf(handle: Hex): FheTypeName {
  return getFheTypeName(parseFhevmHandle(handle).fhevmType);
}

function isScalar(args: Args, name: string): boolean {
  const byte: unknown = args.scalarByte;
  if (typeof byte !== 'string' || !isHex(byte) || size(byte) !== 1) fail(name, 'scalarByte', 'a bytes1', byte);
  return byte === '0x01';
}

function bytes32(args: Args, key: string, name: string): Hex {
  const value: unknown = args[key];
  if (typeof value !== 'string' || !isHex(value) || size(value) !== 32) fail(name, key, 'a bytes32', value);
  return value;
}

function bytes32Array(args: Args, key: string, name: string): Hex[] {
  const value: unknown = args[key];
  if (!Array.isArray(value)) fail(name, key, 'a bytes32[]', value);
  return value.map((entry: unknown) => {
    if (typeof entry !== 'string' || !isHex(entry) || size(entry) !== 32) fail(name, key, 'a bytes32[]', entry);
    return entry;
  });
}

function uint(args: Args, key: string, name: string): number {
  const value: unknown = args[key];
  if (typeof value === 'number') return value;
  if (typeof value === 'bigint' && value >= 0n && value <= 0xffn) return Number(value);
  return fail(name, key, 'a small unsigned integer', value);
}

function fail(eventName: string, key: string, expected: string, value: unknown): never {
  throw new HardhatPluginError(
    PLUGIN_ID,
    `Unexpected ${eventName} event arg '${key}': expected ${expected}, got '${String(value)}'.`,
  );
}
