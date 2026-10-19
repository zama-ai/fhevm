// HCU pricing keyed by the event `FHEVMExecutor` emits. The numbers are not maintained here: they come
// from the vendored `operatorsPrices.ts`, verbatim from the fhevm repository. This module bridges the
// two vocabularies (upstream `fheAdd`, executor `FheAdd`) and exposes the lookups the HCU walk needs.

import { HardhatPluginError } from 'hardhat/plugins';

import { PLUGIN_ID } from '../constants.js';
import { ALL_OPERATORS_PRICES } from '../vendored/operatorsPrices.js';
import type { FheOperatorName, FheTypeName, NBucketedCost } from '../vendored/priceTypes.js';

export type { FheTypeName } from '../vendored/priceTypes.js';

/** Upstream operator name -> the event `FHEVMExecutor` emits for it. */
const EVENT_NAME_BY_OPERATOR: Readonly<Record<FheOperatorName, string>> = {
  fheAdd: 'FheAdd',
  fheSub: 'FheSub',
  fheMul: 'FheMul',
  fheDiv: 'FheDiv',
  fheRem: 'FheRem',
  fheBitAnd: 'FheBitAnd',
  fheBitOr: 'FheBitOr',
  fheBitXor: 'FheBitXor',
  fheShl: 'FheShl',
  fheShr: 'FheShr',
  fheRotl: 'FheRotl',
  fheRotr: 'FheRotr',
  fheEq: 'FheEq',
  fheNe: 'FheNe',
  fheGe: 'FheGe',
  fheGt: 'FheGt',
  fheLe: 'FheLe',
  fheLt: 'FheLt',
  fheMin: 'FheMin',
  fheMax: 'FheMax',
  fheNeg: 'FheNeg',
  fheNot: 'FheNot',
  cast: 'Cast',
  trivialEncrypt: 'TrivialEncrypt',
  // Upstream says `ifThenElse`, the executor emits `FheIfThenElse`.
  ifThenElse: 'FheIfThenElse',
  fheRand: 'FheRand',
  fheRandBounded: 'FheRandBounded',
  fheSum: 'FheSum',
  fheIsIn: 'FheIsIn',
  // No `FheMulDiv` event exists in v13 yet; the price is published, so it is carried through.
  fheMulDiv: 'FheMulDiv',
};

export type HCUOperator = (typeof ALL_OPERATORS_PRICES)[FheOperatorName];

/** The price table by event name. `VerifyInput` is absent: the executor emits it, upstream prices nothing for it. */
export const HCU_PRICE_BY_EVENT: Readonly<Record<string, HCUOperator>> = Object.freeze(
  Object.fromEntries(
    (Object.entries(EVENT_NAME_BY_OPERATOR) as Array<[FheOperatorName, string]>).map(([operator, event]) => [
      event,
      ALL_OPERATORS_PRICES[operator],
    ]),
  ),
);

export function hcuPriceOf(eventName: string): HCUOperator | undefined {
  return Object.hasOwn(HCU_PRICE_BY_EVENT, eventName) ? HCU_PRICE_BY_EVENT[eventName] : undefined;
}

// Buckets are upper bounds (`le10` covers 1..10, `le30` covers 11..30, …). A type may stop early:
// larger arrays exceed the per-transaction limit anyway, so beyond the last bucket is an error.
export function getBucketedHCU(cost: NBucketedCost, n: number): number {
  const buckets: Array<[number, number | undefined]> = [
    [10, cost.le10],
    [30, cost.le30],
    [60, cost.le60],
    [100, cost.le100],
    [128, cost.le128],
  ];
  for (const [limit, price] of buckets) {
    if (n > limit) continue;
    if (price === undefined) break;
    return price;
  }
  throw new HardhatPluginError(
    PLUGIN_ID,
    `No HCU price for an array of ${String(n)} elements (largest priced bucket exceeded).`,
  );
}

/**
 * The HCU an operator costs for a type. `scalar` picks the scalar column of a binary operator;
 * `n` is the input count of a bucketed operator (`FheSum`, `FheIsIn`).
 */
export function getHCU(eventName: string, type: FheTypeName, opts?: { scalar?: boolean; n?: number }): number {
  const op = hcuPriceOf(eventName);
  if (op === undefined) throw new HardhatPluginError(PLUGIN_ID, `Unknown HCU operator '${eventName}'`);

  if (op.nBucketed !== undefined) {
    const cost = op.nBucketed[type];
    if (cost === undefined) throw unknownType(eventName, type);
    if (opts?.n === undefined) {
      throw new HardhatPluginError(PLUGIN_ID, `Operator '${eventName}' is priced per input count; 'n' is required.`);
    }
    return getBucketedHCU(cost, opts.n);
  }

  const column = op.types ?? ((opts?.scalar ?? false) ? op.scalar : (op.nonScalar ?? op.scalar));
  const hcu = column?.[type];
  if (hcu === undefined) throw unknownType(eventName, type);
  return hcu;
}

function unknownType(eventName: string, type: FheTypeName): HardhatPluginError {
  return new HardhatPluginError(PLUGIN_ID, `Unknown HCU type '${type}' for operator '${eventName}'`);
}
