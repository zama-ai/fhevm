import type { BytesLike, Signer } from 'ethers';

import { EncryptedERC20, Rand } from '../types';
import type { Signers } from './signers';

declare module 'mocha' {
  export interface Context {
    signers: Signers;
    contractAddress: string;
    instances: FhevmInstances;
    erc20: EncryptedERC20;
    rand: Rand;
  }
}

export type Keypair = { publicKey: string; privateKey: string };

export interface EncryptedInput {
  addBool(value: boolean | number | bigint): EncryptedInput;
  add8(value: number | bigint): EncryptedInput;
  add16(value: number | bigint): EncryptedInput;
  add32(value: number | bigint): EncryptedInput;
  add64(value: number | bigint): EncryptedInput;
  add128(value: number | bigint): EncryptedInput;
  add256(value: number | bigint): EncryptedInput;
  addAddress(value: string): EncryptedInput;
  getValues(): bigint[];
  getBits(): number[];
  resetValues(): EncryptedInput;
  encrypt(): Promise<{ handles: Uint8Array[]; inputProof: BytesLike }>;
}

export interface FhevmInstance {
  createEncryptedInput(contractAddress: string, userAddress: string): EncryptedInput;
  generateKeypair(): Promise<Keypair>;
  userDecryptSingleHandle(parameters: {
    handle: BytesLike;
    contractAddress: string;
    signer: Signer;
    keypair: Keypair;
  }): Promise<bigint>;
}

export interface FhevmInstances {
  alice: FhevmInstance;
  bob: FhevmInstance;
  carol: FhevmInstance;
  dave: FhevmInstance;
  eve: FhevmInstance;
}
