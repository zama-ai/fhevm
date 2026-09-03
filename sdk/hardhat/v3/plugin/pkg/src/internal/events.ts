// The FHEVMExecutor operator events in a transaction's logs. Nothing is computed from them any more
// (the cleartext stack evaluates operators on-chain), but they are still emitted, and HCU pricing
// (D5) is keyed by them. Logs may come from viem (`logIndex`) or ethers (`index`): both shapes fit.

import { decodeEventLog } from 'viem';

import type { CoprocessorEvent, CoprocessorEventName, FhevmLog } from '../types.js';
import type { FhevmContractWrapper } from './contracts.js';

/** Every event `FHEVMExecutor` emits for an operator or an input, per the host contracts' `FHEEvents.sol`. */
export const COPROCESSOR_EVENT_NAMES: readonly CoprocessorEventName[] = [
  'TrivialEncrypt',
  'FheAdd',
  'FheSub',
  'FheMul',
  'FheDiv',
  'FheRem',
  'FheBitAnd',
  'FheBitOr',
  'FheBitXor',
  'FheShl',
  'FheShr',
  'FheRotl',
  'FheRotr',
  'FheEq',
  'FheNe',
  'FheGe',
  'FheGt',
  'FheLe',
  'FheLt',
  'FheMin',
  'FheMax',
  'FheRand',
  'FheRandBounded',
  'FheNot',
  'FheNeg',
  'Cast',
  'FheIfThenElse',
  'FheSum',
  'FheIsIn',
  'VerifyInput',
];

const NAMES: ReadonlySet<string> = new Set(COPROCESSOR_EVENT_NAMES);

export function isCoprocessorEventName(value: unknown): value is CoprocessorEventName {
  return typeof value === 'string' && NAMES.has(value);
}

export function parseCoprocessorEvents(
  executor: FhevmContractWrapper,
  logs: readonly FhevmLog[] | null | undefined,
): CoprocessorEvent[] {
  if (logs === null || logs === undefined) return [];
  const events: CoprocessorEvent[] = [];
  for (const log of logs) {
    if (log.address.toLowerCase() !== executor.address.toLowerCase()) continue;
    const event = decodeCoprocessorEvent(executor, log);
    if (event !== undefined) events.push(event);
  }
  return events;
}

function decodeCoprocessorEvent(executor: FhevmContractWrapper, log: FhevmLog): CoprocessorEvent | undefined {
  const [signature, ...rest] = log.topics;
  if (signature === undefined) return undefined;
  try {
    const { eventName, args } = decodeEventLog({ abi: executor.abi, data: log.data, topics: [signature, ...rest] });
    if (!isCoprocessorEventName(eventName)) return undefined;
    return {
      eventName,
      args: args ?? {},
      index: log.logIndex ?? log.index ?? -1,
      blockNumber: Number(log.blockNumber ?? -1),
      transactionHash: log.transactionHash ?? '0x',
      transactionIndex: log.transactionIndex ?? -1,
    };
  } catch {
    return undefined;
  }
}
