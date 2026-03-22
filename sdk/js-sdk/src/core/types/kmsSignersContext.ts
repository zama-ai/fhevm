import type { Kms } from "./kms.js";
import type { ChecksummedAddress, Uint8Number } from "./primitives.js";

export type KmsSignersContext = {
  readonly address: ChecksummedAddress;
  readonly signers: readonly ChecksummedAddress[];
  readonly threshold: Uint8Number;
  has(signer: string): boolean;
} & Kms;
