import type { InputHandle } from './encryptedTypes-p.js';
import type { Bytes65Hex, BytesHex, ChecksummedAddress } from './primitives.js';
import type { NonEmptyReadonlyArray, Prettify } from './utils.js';

////////////////////////////////////////////////////////////////////////////////
// InputProof Types
////////////////////////////////////////////////////////////////////////////////

export type InputProofBytes = Readonly<{
  handles: NonEmptyReadonlyArray<Uint8Array>;
  inputProof: Readonly<Uint8Array>;
}>;

/**
 * An input proof containing encrypted handles and coprocessor signatures.
 *
 * Used in Solidity function calls where:
 * - `inputProof` (bytes calldata) corresponds to {@link InputProof.bytesHex}
 * - Each `externalE*` parameter corresponds to an entry in {@link InputProof.inputHandles}
 *
 * @example Solidity contract receiving an encrypted input
 * ```solidity
 * //                   externalHandles[0]              bytesHex
 * //                   vvvvvvvvvvvvvvvvvvvvvvvvvvvvvv  vvvvvvvvvvvvvvvvvvvvvvvvv
 * function addEuint128(externalEuint128 inputEuint128, bytes calldata inputProof) external {
 *     euint128 encValue = FHE.fromExternal(inputEuint128, inputProof);
 *     ...
 * }
 * ```
 */
export type InputProof = {
  /**
   * The full proof encoded as a 0x-prefixed hexadecimal string.
   * Passed as the `inputProof` argument to a Solidity function call
   * alongside the external handles.
   */
  readonly bytesHex: BytesHex;
  /** Coprocessor signatures (encoded in the proof). */
  readonly coprocessorSignatures: NonEmptyReadonlyArray<Bytes65Hex>;
  /** Handles signed by coprocessors, e.g. externalEbool, externalEuint8 (encoded in the proof). */
  readonly inputHandles: NonEmptyReadonlyArray<InputHandle>;
  /** Extra data (encoded in the proof). */
  readonly extraData: BytesHex;
  /** Whether the proof has been verified by the SDK. */
  readonly verified: boolean;
  /**
   * Parameters each coprocessor signed over when producing its signature.
   * Coprocessors sign `userAddress + contractAddress`, meaning the handles
   * are only allowed to be used by the specified user in the specified contract.
   *
   * Not encoded in the proof itself. `undefined` if not yet verified.
   */
  readonly signedHandleAccess:
    | {
        /** Contract address the handles are bound to. */
        readonly contractAddress: ChecksummedAddress;
        /** User address the handles are bound to. */
        readonly userAddress: ChecksummedAddress;
      }
    | undefined;
};

/**
 * An input proof that has not been verified by the SDK.
 * Without {@link InputProof.signedHandleAccess} data, the coprocessor
 * signatures cannot be programmatically verified.
 */
export type UnverifiedInputProof = Prettify<
  Omit<InputProof, 'signedHandleAccess' | 'verified'> & {
    readonly verified: false;
    readonly signedHandleAccess: undefined;
  }
>;

/**
 * An input proof whose coprocessor signatures have been verified by the SDK
 * against the provided {@link InputProof.signedHandleAccess} addresses.
 */
export type VerifiedInputProof = Prettify<
  Omit<InputProof, 'signedHandleAccess' | 'verified'> & {
    readonly verified: true;
    readonly signedHandleAccess: {
      readonly contractAddress: ChecksummedAddress;
      readonly userAddress: ChecksummedAddress;
    };
  }
>;
