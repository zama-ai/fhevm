import type { FhevmErrorBaseParams } from './FhevmErrorBase.js';
import type { Prettify } from '../types/utils.js';
import { FhevmErrorBase } from './FhevmErrorBase.js';

////////////////////////////////////////////////////////////////////////////////
// Erc1271VerificationError
//
// Definitive, client-side rejections of a user-decryption signature, mirroring
// the shared `user-decryption-signature` crate's error taxonomy. These are
// thrown fail-fast (matching the relayer's sync 400). Inconclusive outcomes
// (no read provider / transport error) do NOT throw — the caller degrades
// gracefully and forwards to the KMS, which is authoritative.
////////////////////////////////////////////////////////////////////////////////

export type Erc1271VerificationErrorParams = Prettify<Omit<FhevmErrorBaseParams, 'name' | 'message'>>;

/** Base class for all definitive ERC-1271 / EOA signature rejections. */
export abstract class Erc1271VerificationError extends FhevmErrorBase {}

////////////////////////////////////////////////////////////////////////////////

export type Erc1271EoaMismatchNoCodeErrorType = Erc1271EoaMismatchNoCodeError & {
  name: 'Erc1271EoaMismatchNoCodeError';
};

/** ecrecover did not match `userAddress` and `userAddress` has no contract code. */
export class Erc1271EoaMismatchNoCodeError extends Erc1271VerificationError {
  constructor(params: Erc1271VerificationErrorParams & { userAddress: string }) {
    super({
      ...params,
      name: 'Erc1271EoaMismatchNoCodeError',
      message: `ecrecover signer mismatch and userAddress ${params.userAddress} has no contract code on the host chain`,
    });
  }
}

////////////////////////////////////////////////////////////////////////////////

export type Erc1271EmptySigOnEoaErrorType = Erc1271EmptySigOnEoaError & {
  name: 'Erc1271EmptySigOnEoaError';
};

/** An empty signature was supplied but `userAddress` has no contract code. */
export class Erc1271EmptySigOnEoaError extends Erc1271VerificationError {
  constructor(params: Erc1271VerificationErrorParams & { userAddress: string }) {
    super({
      ...params,
      name: 'Erc1271EmptySigOnEoaError',
      message: `empty signature is only valid for contracts; userAddress ${params.userAddress} has no contract code`,
    });
  }
}

////////////////////////////////////////////////////////////////////////////////

export type Erc1271WrongMagicErrorType = Erc1271WrongMagicError & {
  name: 'Erc1271WrongMagicError';
};

/** `isValidSignature` returned a non-magic value. */
export class Erc1271WrongMagicError extends Erc1271VerificationError {
  constructor(params: Erc1271VerificationErrorParams & { userAddress: string; magicValue: string }) {
    super({
      ...params,
      name: 'Erc1271WrongMagicError',
      message: `ERC-1271 isValidSignature returned non-magic value ${params.magicValue} for userAddress ${params.userAddress}`,
    });
  }
}

////////////////////////////////////////////////////////////////////////////////

export type Erc1271RejectedErrorType = Erc1271RejectedError & {
  name: 'Erc1271RejectedError';
};

/** `isValidSignature` reverted or returned malformed data. */
export class Erc1271RejectedError extends Erc1271VerificationError {
  constructor(params: Erc1271VerificationErrorParams & { userAddress: string; detail: string }) {
    super({
      ...params,
      name: 'Erc1271RejectedError',
      message: `ERC-1271 isValidSignature reverted or returned malformed data for userAddress ${params.userAddress}: ${params.detail}`,
    });
  }
}
