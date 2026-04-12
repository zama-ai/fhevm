import type { ChecksummedAddress } from './primitives.js';

export declare const fhevmAccountBrand: unique symbol;

export type FhevmAccount = {
  readonly [fhevmAccountBrand]: never;
  readonly userAddress: ChecksummedAddress;
};
