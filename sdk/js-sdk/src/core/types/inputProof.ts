////////////////////////////////////////////////////////////////////////////////
// InputProof Types
////////////////////////////////////////////////////////////////////////////////

import type { ExternalFhevmHandle } from "./fhevmHandle.js";
import type { Bytes65Hex, BytesHex, ChecksummedAddress } from "./primitives.js";
import type { NonEmptyReadonlyArray, Prettify } from "./utils.js";

export type InputProofBytes = Readonly<{
  handles: NonEmptyReadonlyArray<Uint8Array>;
  inputProof: Readonly<Uint8Array>;
}>;

export type InputProof = {
  readonly bytesHex: BytesHex;
  readonly coprocessorSignatures: NonEmptyReadonlyArray<Bytes65Hex>;
  readonly externalHandles: NonEmptyReadonlyArray<ExternalFhevmHandle>;
  readonly extraData: BytesHex;
  readonly verified: boolean;
  readonly coprocessorSignedParams:
    | {
        readonly contractAddress: ChecksummedAddress;
        readonly userAddress: ChecksummedAddress;
      }
    | undefined;
};

export type UnverifiedInputProof = Prettify<
  Omit<InputProof, "coprocessorSignedParams" | "verified"> & {
    readonly verified: false;
    readonly coprocessorSignedParams: undefined;
  }
>;

export type VerifiedInputProof = Prettify<
  Omit<InputProof, "coprocessorSignedParams" | "verified"> & {
    readonly verified: true;
    readonly coprocessorSignedParams: {
      readonly contractAddress: ChecksummedAddress;
      readonly userAddress: ChecksummedAddress;
    };
  }
>;
