import { ethers as EthersT } from 'ethers';

import { HardhatFhevmError } from '../error';
import { FhevmType, fhevmTypeToSdkType } from './fheType';
import type { FhevmClient } from './sdkTypes';

/**
 * The batched encrypted-input builder behind `fhevm.createEncryptedInput(...)`.
 *
 * This is not a compatibility shim kept for old tests: batching is a real capability that the
 * singular `encryptUint`/`encryptBool`/`encryptAddress` helpers cannot express. Several values
 * encrypted together share **one** input proof, which is both cheaper and what a contract taking
 * multiple `externalEuintXX` arguments requires. `@fhevm/sdk` supports it as `encryptValues`; this
 * wraps that in the accumulate-then-`encrypt()` shape the plugin has always exposed.
 */
export type FhevmEncryptedInput = {
  readonly contractAddress: string;
  readonly userAddress: string;

  addBool(value: boolean): FhevmEncryptedInput;
  add8(value: number | bigint): FhevmEncryptedInput;
  add16(value: number | bigint): FhevmEncryptedInput;
  add32(value: number | bigint): FhevmEncryptedInput;
  add64(value: number | bigint): FhevmEncryptedInput;
  add128(value: number | bigint): FhevmEncryptedInput;
  add256(value: number | bigint): FhevmEncryptedInput;
  addAddress(value: string): FhevmEncryptedInput;

  /** Encrypts everything added so far into a single proof. */
  encrypt(): Promise<{ handles: string[]; inputProof: string }>;
};

export function createEncryptedInput(
  getClient: () => FhevmClient,
  contractAddress: string,
  userAddress: string,
): FhevmEncryptedInput {
  if (!EthersT.isAddress(contractAddress)) {
    throw new HardhatFhevmError(
      `createEncryptedInput: the 'contractAddress' argument is not a valid address. Got '${contractAddress}' instead.`,
    );
  }
  if (!EthersT.isAddress(userAddress)) {
    throw new HardhatFhevmError(
      `createEncryptedInput: the 'userAddress' argument is not a valid address. Got '${userAddress}' instead.`,
    );
  }

  const values: Array<{ type: string; value: boolean | bigint | number | string }> = [];

  const builder: FhevmEncryptedInput = {
    contractAddress,
    userAddress,

    addBool(value: boolean) {
      values.push({ type: fhevmTypeToSdkType(FhevmType.ebool), value });
      return builder;
    },
    add8(value: number | bigint) {
      values.push({ type: fhevmTypeToSdkType(FhevmType.euint8), value });
      return builder;
    },
    add16(value: number | bigint) {
      values.push({ type: fhevmTypeToSdkType(FhevmType.euint16), value });
      return builder;
    },
    add32(value: number | bigint) {
      values.push({ type: fhevmTypeToSdkType(FhevmType.euint32), value });
      return builder;
    },
    add64(value: number | bigint) {
      values.push({ type: fhevmTypeToSdkType(FhevmType.euint64), value });
      return builder;
    },
    add128(value: number | bigint) {
      values.push({ type: fhevmTypeToSdkType(FhevmType.euint128), value });
      return builder;
    },
    add256(value: number | bigint) {
      values.push({ type: fhevmTypeToSdkType(FhevmType.euint256), value });
      return builder;
    },
    addAddress(value: string) {
      values.push({ type: fhevmTypeToSdkType(FhevmType.eaddress), value });
      return builder;
    },

    async encrypt() {
      if (values.length === 0) {
        throw new HardhatFhevmError(`createEncryptedInput: nothing to encrypt — add at least one value.`);
      }
      const res = await getClient().encryptValues({ values, contractAddress, userAddress });
      return {
        // `handles` is the historical name for these; they are bytes32 handles either way.
        handles: [...res.encryptedValues],
        inputProof: res.inputProof,
      };
    },
  };

  return builder;
}
