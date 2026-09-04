/**
 * The types `operatorsPrices.ts` needs, mirroring the fhevm repository's
 * `<fhevm>/library-solidity/codegen/src/common.ts`.
 *
 * Kept separate so the vendored price table stays byte-identical to upstream: only its import line
 * differs. `FheTypeName` is narrowed to the widths the price table actually mentions — upstream
 * declares the full protocol enum (up to `Uint2048`, plus signed and odd widths), none of which the
 * plugin can price or encounter.
 */

export type FheTypeName = 'Bool' | 'Uint8' | 'Uint16' | 'Uint32' | 'Uint64' | 'Uint128' | 'Uint160' | 'Uint256';

export type FheOperatorName =
  | 'fheAdd'
  | 'fheSub'
  | 'fheMul'
  | 'fheDiv'
  | 'fheRem'
  | 'fheBitAnd'
  | 'fheBitOr'
  | 'fheBitXor'
  | 'fheShl'
  | 'fheShr'
  | 'fheRotl'
  | 'fheRotr'
  | 'fheEq'
  | 'fheNe'
  | 'fheGe'
  | 'fheGt'
  | 'fheLe'
  | 'fheLt'
  | 'fheMin'
  | 'fheMax'
  | 'fheNeg'
  | 'fheNot'
  | 'cast'
  | 'trivialEncrypt'
  | 'ifThenElse'
  | 'fheRand'
  | 'fheRandBounded'
  | 'fheSum'
  | 'fheIsIn'
  | 'fheMulDiv';

/**
 * Cost of an operator whose price depends on how many elements it was given — `fheSum` and
 * `fheIsIn` take arrays. The buckets are upper bounds: an array of 25 is priced at `le30`.
 */
export type NBucketedCost = {
  le10: number;
  le30?: number;
  le60?: number;
  le100?: number;
  le128?: number;
};

export type PriceData = Record<
  FheOperatorName,
  {
    supportScalar: boolean;
    numberInputs: number;
    scalar?: Partial<Record<FheTypeName, number>>;
    nonScalar?: Partial<Record<FheTypeName, number>>;
    types?: Partial<Record<FheTypeName, number>>;
    nBucketed?: Partial<Record<FheTypeName, NBucketedCost>>;
  }
>;
