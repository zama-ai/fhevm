import type {
  Address,
  Bytes32Hex,
  ChecksummedAddress,
} from '../types/primitives.js';
import type { FhevmErrorBaseParams } from './FhevmErrorBase.js';
import type { Prettify } from '../types/utils.js';
import { ContractErrorBase } from './ContractErrorBase.js';

////////////////////////////////////////////////////////////////////////////////
// ACLPublicDecryptionError
////////////////////////////////////////////////////////////////////////////////

export type ACLPublicDecryptionErrorType = ACLPublicDecryptionError & {
  name: 'ACLPublicDecryptionError';
};

export type ACLPublicDecryptionErrorParams = Prettify<
  Omit<FhevmErrorBaseParams, 'name'> & { handle?: string }
>;

export class ACLPublicDecryptionError extends ContractErrorBase {
  readonly #handles: Bytes32Hex[];

  constructor({
    contractAddress,
    handles,
  }: {
    contractAddress: ChecksummedAddress;
    handles: Bytes32Hex[];
  }) {
    const handleList = handles.join(', ');
    super({
      message:
        handles.length === 1
          ? `Handle ${handles[0]} is not allowed for public decryption`
          : `${handles.length} handles are not allowed for public decryption: ${handleList}`,
      name: 'ACLPublicDecryptionError',
      contractAddress,
      contractName: 'ACL',
    });
    this.#handles = handles;
  }

  public get handles(): Bytes32Hex[] {
    return this.#handles;
  }
}

////////////////////////////////////////////////////////////////////////////////
// ACLUserDecryptionError
////////////////////////////////////////////////////////////////////////////////

export type ACLUserDecryptionErrorType = ACLUserDecryptionError & {
  name: 'ACLUserDecryptionError';
};

export class ACLUserDecryptionError extends ContractErrorBase {
  constructor({
    contractAddress,
    message,
  }: {
    contractAddress: Address;
    message: string;
  }) {
    super({
      message,
      name: 'ACLUserDecryptionError',
      contractAddress,
      contractName: 'ACL',
    });
  }
}
