import type { Handle } from './encryptedTypes-p.js';
import type { ChecksummedAddress } from './primitives.js';

export type HandleAccountPair = {
  readonly handle: Handle;
  readonly account: ChecksummedAddress;
};

export type HandleContractPair = {
  readonly handle: Handle;
  readonly contractAddress: ChecksummedAddress;
};
