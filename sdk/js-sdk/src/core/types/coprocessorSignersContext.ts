import type { Coprocessor } from './coprocessor.js';
import type { ChecksummedAddress, Uint8Number } from './primitives.js';

export type CoprocessorSignersContext = {
  readonly address: ChecksummedAddress;
  readonly signers: readonly ChecksummedAddress[];
  readonly threshold: Uint8Number;
  has(signer: string): boolean;
} & Coprocessor;

export type CoprocessorSignersContextJson = {
  address: string;
  signers: string[];
  threshold: number;
};
