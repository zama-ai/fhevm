import type { Fhevm, FhevmBase, FhevmExtension } from '../../types/coreFhevmClient.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type {
  SignedSelfDecryptionPermit,
  SignedDelegatedDecryptionPermit,
} from '../../types/signedDecryptionPermit.js';
import type {
  SerializeTransportKeypairParameters,
  SerializeTransportKeypairReturnType,
} from '../../actions/chain/serializeTransportKeypair.js';
import { assertIsFhevmBaseClient } from '../../runtime/CoreFhevm-p.js';
import {
  signDecryptionPermit,
  type SignSelfDecryptionPermitParameters,
  type SignDelegatedDecryptionPermitParameters,
} from '../../actions/base/signDecryptionPermit.js';
import {
  parseTransportKeypair,
  type ParseTransportKeypairParameters,
  type ParseTransportKeypairReturnType,
} from '../../actions/chain/parseTransportKeypair.js';
import {
  fetchFheEncryptionKeyBytes,
  type FetchFheEncryptionKeyBytesParameters,
  type FetchFheEncryptionKeyBytesReturnType,
} from '../../actions/chain/fetchFheEncryptionKeyBytes.js';
import { serializeTransportKeypair } from '../../actions/chain/serializeTransportKeypair.js';
import {
  serializeSignedDecryptionPermit,
  type SerializeSignedDecryptionPermitParameters,
  type SerializeSignedDecryptionPermitReturnType,
} from '../../actions/chain/serializeSignedDecryptionPermit.js';
import {
  parseSignedDecryptionPermit,
  type ParseSignedDecryptionPermitParameters,
  type ParseSignedDecryptionPermitReturnType,
} from '../../actions/chain/parseSignedDecryptionPermit.js';
import {
  readPublicValue,
  type ReadPublicValueParameters,
  type ReadPublicValueReturnType,
} from '../../actions/base/readPublicValue.js';
import {
  readPublicValues,
  type ReadPublicValuesParameters,
  type ReadPublicValuesReturnType,
} from '../../actions/base/readPublicValues.js';
import {
  readPublicValuesWithSignatures,
  type ReadPublicValuesWithSignaturesParameters,
  type ReadPublicValuesWithSignaturesReturnType,
} from '../../actions/base/readPublicValuesWithSignatures.js';

////////////////////////////////////////////////////////////////////////////////

export type BaseActions = {
  /**
   * Reads the decrypted (clear) value of an encrypted value that was made public.
   *
   * By default, encrypted values on-chain can only be decrypted by their owner.
   * When a contract calls `TFHE.allowForDecryption(encryptedValue)` in Solidity,
   * the value becomes publicly readable. Anyone can then call `readPublicValue`
   * to get the clear value — no permit or private key needed.
   *
   * @example
   * ```ts
   * const result = await client.readPublicValue({
   *   encryptedValue: "0xabcd..."
   * });
   * console.log(result.value); // e.g. 42
   * console.log(result.type); // e.g. 'uint8'
   * ```
   */
  readonly readPublicValue: (parameters: ReadPublicValueParameters) => Promise<ReadPublicValueReturnType>;
  /**
   * Reads the decrypted (clear) values of encrypted values that were made public.
   *
   * By default, encrypted values on-chain can only be decrypted by their owner.
   * When a contract calls `TFHE.allowForDecryption(encryptedValue)` in Solidity,
   * a value becomes publicly readable. Anyone can then call `readPublicValues`
   * to get the clear values — no permit or private key needed.
   *
   * @example
   * ```ts
   * const results = await client.readPublicValues({
   *   encryptedValues: ["0xabcd...", "0xef..."]
   * });
   * console.log(results[0].value); // e.g. 42
   * console.log(results[0].type); // e.g. 'uint8'
   * console.log(results[1].value); // e.g. 123456789
   * console.log(results[1].type); // e.g. 'uint32'
   * ```
   */
  readonly readPublicValues: (parameters: ReadPublicValuesParameters) => Promise<ReadPublicValuesReturnType>;
  /**
   * Reads the decrypted (clear) value of an encrypted value that was made public.
   *
   * By default, encrypted values on-chain can only be decrypted by their owner.
   * When a contract calls `TFHE.allowForDecryption(encryptedValue)` in Solidity,
   * the value becomes publicly readable. Anyone can then call `readPublicValue`
   * to get the clear value — no permit or private key needed.
   *
   * Returns both the clear values and a decryption proof. The proof can be
   * forwarded on-chain so a smart contract can verify that an encrypted value
   * decrypts to a specific clear value (e.g. "handle X is actually 123").
   *
   * @example
   * ```ts
   * const result = await client.readPublicValuesWithSignatures({
   *   encryptedValues: ["0xabcd...", "0xef..."]
   * });
   * console.log(results.clearValues[0].value); // e.g. 42
   * console.log(results.clearValues[0].type); // e.g. 'uint8'
   * console.log(results.clearValues[1].value); // e.g. 123456789
   * console.log(results.clearValues[1].type); // e.g. 'uint32'
   *
   * // Forward the proof on-chain for smart contract verification
   * // Solidity side:
   * //   function verify(bytes32[] calldata handlesList, bytes memory cleartexts, bytes memory decryptionProof) external {
   * //       FHE.checkSignatures(handlesList, cleartexts, decryptionProof);
   * //   }
   * const args = result.checkSignaturesArgs;
   * await contract.verify(args.handlesList, args.abiEncodedCleartexts, args.decryptionProof);
   * ```
   */
  readonly readPublicValuesWithSignatures: (
    parameters: ReadPublicValuesWithSignaturesParameters,
  ) => Promise<ReadPublicValuesWithSignaturesReturnType>;
  readonly signDecryptionPermit: {
    /**
     * Signs a self decryption permit — Alice decrypts her own encrypted values.
     *
     * 1. Alice signs a permit saying "I, Alice, want to decrypt my own handles."
     * 2. Alice calls `decrypt` with this permit.
     * 3. The KMS verifies Alice's signature and releases decrypted shares to her.
     *
     * No `delegatorAddress` needed — the signer is the owner.
     */
    (parameters: SignSelfDecryptionPermitParameters): Promise<SignedSelfDecryptionPermit>;
    /**
     * Signs a delegated decryption permit — Bob decrypts Alice's (`delegatorAddress`) encrypted values.
     *
     * 1. Alice calls `FHE.delegateUserDecryption()` on-chain, giving Bob permission to decrypt her handles.
     * 2. Bob signs a permit with `delegatorAddress: Alice`, saying "I, Bob, want to decrypt Alice's handles."
     * 3. Bob calls `decrypt` with this permit.
     * 4. The KMS authenticates Bob via his signature, then checks the on-chain ACL to verify Alice delegated to Bob.
     */
    (parameters: SignDelegatedDecryptionPermitParameters): Promise<SignedDelegatedDecryptionPermit>;
  };
  /** Deserializes a previously serialized e2e transport keypair back into a usable keypair. */
  readonly parseTransportKeypair: (
    parameters: ParseTransportKeypairParameters,
  ) => Promise<ParseTransportKeypairReturnType>;
  /** Serializes an e2e transport keypair to hex strings for storage. */
  readonly serializeTransportKeypair: (
    parameters: SerializeTransportKeypairParameters,
  ) => SerializeTransportKeypairReturnType;
  /** Serializes a signed decryption permit to a plain object for storage or transmission. */
  readonly serializeSignedDecryptionPermit: (
    parameters: SerializeSignedDecryptionPermitParameters,
  ) => SerializeSignedDecryptionPermitReturnType;
  /** Parses and verifies a previously serialized signed decryption permit. */
  readonly parseSignedDecryptionPermit: (
    parameters: ParseSignedDecryptionPermitParameters,
  ) => Promise<ParseSignedDecryptionPermitReturnType>;
  /** Fetches the ~50MB FHE public encryption key from the relayer and caches it. */
  readonly fetchFheEncryptionKeyBytes: (
    parameters?: FetchFheEncryptionKeyBytesParameters,
  ) => Promise<FetchFheEncryptionKeyBytesReturnType>;
};

////////////////////////////////////////////////////////////////////////////////

function _baseActions(fhevm: Fhevm<FhevmChain>): BaseActions {
  return {
    readPublicValue: (parameters) => readPublicValue(fhevm, parameters),
    readPublicValues: (parameters) => readPublicValues(fhevm, parameters),
    readPublicValuesWithSignatures: (parameters) => readPublicValuesWithSignatures(fhevm, parameters),
    // Preserve the original action overloads on the decorated client API.
    // Runtime behavior is unchanged: this is a direct pass-through wrapper.
    signDecryptionPermit: ((parameters: SignSelfDecryptionPermitParameters | SignDelegatedDecryptionPermitParameters) =>
      signDecryptionPermit(
        fhevm,
        parameters as SignSelfDecryptionPermitParameters,
      )) as BaseActions['signDecryptionPermit'],
    parseTransportKeypair: (parameters) => parseTransportKeypair(fhevm, parameters),
    serializeTransportKeypair: (parameters) => serializeTransportKeypair(fhevm, parameters),
    serializeSignedDecryptionPermit: (parameters) => serializeSignedDecryptionPermit(fhevm, parameters),
    parseSignedDecryptionPermit: (parameters) => parseSignedDecryptionPermit(fhevm, parameters),
    fetchFheEncryptionKeyBytes: (parameters) => fetchFheEncryptionKeyBytes(fhevm, parameters),
  };
}

////////////////////////////////////////////////////////////////////////////////

export function baseActions(fhevm: FhevmBase<FhevmChain>): FhevmExtension<BaseActions> {
  assertIsFhevmBaseClient(fhevm);
  return {
    actions: _baseActions(fhevm),
    runtime: fhevm.runtime,
    // no init required, no prefetch of the FheEncryptionKey. This is the whole purpose of the fetchFheEncryptionKeyBytes action
  };
}
