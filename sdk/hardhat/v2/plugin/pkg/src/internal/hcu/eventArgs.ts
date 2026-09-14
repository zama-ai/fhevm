import { ethers as EthersT } from 'ethers';

import { HardhatFhevmError } from '../../error';

/**
 * Shape assertions for decoded event arguments, ported from `@fhevm/mock-utils`.
 *
 * They exist because the HCU walk indexes into `event.args` positionally: if an ABI ever changes
 * shape, this fails at the argument rather than producing a silently wrong cost.
 */

// eslint-disable-next-line @typescript-eslint/naming-convention
function __fail(eventName: string, argIndex: number, expected: string, value: unknown): never {
  throw new HardhatFhevmError(
    `Unexpected ${eventName} event arg #${argIndex}: expected ${expected}, got '${String(value)}'.`,
  );
}

// eslint-disable-next-line @typescript-eslint/naming-convention
function __assertBytes(value: unknown, eventName: string, argIndex: number, numBytes: number): void {
  if (typeof value !== 'string' || !EthersT.isHexString(value, numBytes)) {
    __fail(eventName, argIndex, `a ${numBytes}-byte hex string`, value);
  }
}

export function assertEventArgIsBigUint8(value: unknown, eventName: string, argIndex: number): asserts value is bigint {
  if (typeof value !== 'bigint' || value < 0n || value > 0xffn) {
    __fail(eventName, argIndex, 'a uint8', value);
  }
}

export function assertEventArgIsBigUint256(
  value: unknown,
  eventName: string,
  argIndex: number,
): asserts value is bigint {
  if (typeof value !== 'bigint' || value < 0n || value > (1n << 256n) - 1n) {
    __fail(eventName, argIndex, 'a uint256', value);
  }
}

export function assertEventArgIsBytes1String(
  value: unknown,
  eventName: string,
  argIndex: number,
): asserts value is `0x${string}` {
  __assertBytes(value, eventName, argIndex, 1);
}

export function assertEventArgIsBytes16String(
  value: unknown,
  eventName: string,
  argIndex: number,
): asserts value is `0x${string}` {
  __assertBytes(value, eventName, argIndex, 16);
}

export function assertEventArgIsBytes32String(
  value: unknown,
  eventName: string,
  argIndex: number,
): asserts value is `0x${string}` {
  __assertBytes(value, eventName, argIndex, 32);
}

export function assertEventArgIsArrayOfBytes32String(
  value: unknown,
  eventName: string,
  argIndex: number,
): asserts value is Array<`0x${string}`> {
  if (!Array.isArray(value)) {
    __fail(eventName, argIndex, 'an array of 32-byte hex strings', value);
  }
  value.forEach((entry: unknown) => {
    __assertBytes(entry, eventName, argIndex, 32);
  });
}
