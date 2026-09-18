import { HardhatFhevmError } from '../../error';
import { ALL_OPERATORS_PRICES } from '../vendored/operatorsPrices';
import type { FheOperatorName, FheTypeName, NBucketedCost } from '../vendored/priceTypes';

/**
 * HCU pricing, keyed by the event name `FHEVMExecutor` emits.
 *
 * The numbers are not maintained here: they come from the vendored `operatorsPrices.ts`, which is
 * copied verbatim from the fhevm repository and is the source of truth. This module only bridges the
 * two vocabularies — upstream keys operators as `fheAdd`, the executor emits `FheAdd` — and exposes
 * the lookups `hcu.ts` needs.
 *
 * Replaces a hand-maintained 525-line copy of the same table, which had drifted: it was missing
 * `fheSum`, `fheIsIn` and `fheMulDiv` entirely.
 */

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
  // Note the asymmetry: upstream calls it `ifThenElse`, the executor emits `FheIfThenElse`.
  ifThenElse: 'FheIfThenElse',
  fheRand: 'FheRand',
  fheRandBounded: 'FheRandBounded',
  fheSum: 'FheSum',
  fheIsIn: 'FheIsIn',
  // No `FheMulDiv` event exists in v13 — @fhevm/solidity does not expose `FHE.mulDiv` yet — but the
  // price is already published, so it is carried through and will light up when the event arrives.
  fheMulDiv: 'FheMulDiv',
};

export type HCUOperator = (typeof ALL_OPERATORS_PRICES)[FheOperatorName];

// eslint-disable-next-line @typescript-eslint/naming-convention
function __byEventName(): Readonly<Record<string, HCUOperator>> {
  const out: Record<string, HCUOperator> = {};
  for (const [operatorName, event] of Object.entries(EVENT_NAME_BY_OPERATOR)) {
    out[event] = ALL_OPERATORS_PRICES[operatorName as FheOperatorName];
  }
  return Object.freeze(out);
}

/**
 * The price table, keyed by event name.
 *
 * `VerifyInput` is deliberately absent: the executor emits it, but upstream publishes no price for
 * it, so it contributes nothing to a transaction's HCU.
 */
export const HCUByOperator = __byEventName();

/**
 * The operators HCU can price — derived from the table, not from the event union.
 *
 * `FHEVMExecutor` emits `VerifyInput` too, which has no published price. Deriving this from the
 * table keeps the two in step rather than letting an event be assumed priced because it exists.
 */
export type HCUOperatorName = keyof typeof EVENT_NAME_BY_OPERATOR extends never
  ? never
  : (typeof EVENT_NAME_BY_OPERATOR)[FheOperatorName];

/**
 * Price of an `nBucketed` operator for an input array of `n` elements.
 *
 * The buckets are upper bounds (`le10` covers 1..10, `le30` covers 11..30, and so on). A type may
 * stop early — `Uint64` sums are only priced to `le60` — because larger arrays exceed the per-
 * transaction HCU limit anyway; asking beyond the last bucket is an error rather than a guess.
 */
export function getBucketedHCU(cost: NBucketedCost, n: number): number {
  const buckets: Array<[number, number | undefined]> = [
    [10, cost.le10],
    [30, cost.le30],
    [60, cost.le60],
    [100, cost.le100],
    [128, cost.le128],
  ];
  for (const [limit, price] of buckets) {
    if (n <= limit) {
      if (price === undefined) {
        break;
      }
      return price;
    }
  }
  throw new HardhatFhevmError(`No HCU price for an array of ${n} elements (largest priced bucket exceeded).`);
}

/**
 * The HCU an operator costs for a given type.
 *
 * @param opName - the event name, e.g. `FheAdd`
 * @param type - the FHE type the price is keyed by, e.g. `Uint32`
 * @param opts.scalar - for binary operators, whether the right operand was a plaintext scalar
 * @param opts.n - for `nBucketed` operators (`FheSum`, `FheIsIn`), the input array length
 */
export function getHCU(opName: string, type: FheTypeName, opts?: { scalar?: boolean; n?: number }): number {
  const hcuOperator = HCUByOperator[opName];
  if (!hcuOperator) {
    throw new HardhatFhevmError(`Unknown HCU operator '${opName}'`);
  }

  if (hcuOperator.nBucketed) {
    const cost = hcuOperator.nBucketed[type];
    if (cost === undefined) {
      throw new HardhatFhevmError(`Unknown HCU type '${type}' for operator '${opName}'`);
    }
    if (opts?.n === undefined) {
      throw new HardhatFhevmError(`Operator '${opName}' is priced per input count; 'n' is required.`);
    }
    return getBucketedHCU(cost, opts.n);
  }

  let m: Partial<Record<FheTypeName, number>> | undefined;
  if (hcuOperator.types) {
    m = hcuOperator.types;
  } else {
    m = (opts?.scalar ?? false) ? hcuOperator.scalar : (hcuOperator.nonScalar ?? hcuOperator.scalar);
  }

  const hcu = m?.[type];
  if (hcu === undefined) {
    throw new HardhatFhevmError(`Unknown HCU type '${type}' for operator '${opName}'`);
  }

  return hcu;
}
