import type { ReadPublicValueParameters, ReadPublicValueReturnType } from '../../actions/base/readPublicValue.js';
import type { Fhevm, FhevmBase, FhevmExtension } from '../../types/coreFhevmClient.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type {
  SignedSelfDecryptionPermit,
  SignedDelegatedDecryptionPermit,
} from '../../types/signedDecryptionPermit.js';
import type {
  SerializeE2eTransportKeypairParameters,
  SerializeE2eTransportKeypairReturnType,
} from '../../actions/chain/serializeE2eTransportKeypair.js';
import {
  publicDecrypt,
  type PublicDecryptParameters,
  type PublicDecryptReturnType,
} from '../../actions/base/publicDecrypt.js';
import { assertIsFhevmBaseClient } from '../../runtime/CoreFhevm-p.js';
import {
  signDecryptionPermit,
  type SignSelfDecryptionPermitParameters,
  type SignDelegatedDecryptionPermitParameters,
} from '../../actions/base/signDecryptionPermit.js';
import {
  parseE2eTransportKeypair,
  type ParseE2eTransportKeypairParameters,
  type ParseE2eTransportKeypairReturnType,
} from '../../actions/chain/parseE2eTransportKeypair.js';
import {
  fetchFheEncryptionKeyBytes,
  type FetchFheEncryptionKeyBytesParameters,
  type FetchFheEncryptionKeyBytesReturnType,
} from '../../actions/chain/fetchFheEncryptionKeyBytes.js';
import { serializeE2eTransportKeypair } from '../../actions/chain/serializeE2eTransportKeypair.js';
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

////////////////////////////////////////////////////////////////////////////////

export type BaseActions = {
  /** Alias for {@link readPublicValue}. */
  readonly publicDecrypt: (parameters: PublicDecryptParameters) => Promise<PublicDecryptReturnType>;
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
   * const handles = ["0xabcd..."]; // handle(s) of public encrypted value(s)
   * const result = await client.readPublicValue({
   *   encryptedValues: handles
   * });
   * console.log(result.orderedClearValues[0].value); // e.g. 42
   *
   * // Forward the proof on-chain for smart contract verification
   * // Solidity side:
   * //   function verify(bytes32[] calldata handlesList, bytes memory cleartexts, bytes memory decryptionProof) external {
   * //       FHE.checkSignatures(handlesList, cleartexts, decryptionProof);
   * //   }
   * await contract.verify(handles, result.orderedAbiEncodedClearValues, result.decryptionProof);
   * ```
   */
  readonly readPublicValue: (parameters: ReadPublicValueParameters) => Promise<ReadPublicValueReturnType>;
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
  readonly parseE2eTransportKeypair: (
    parameters: ParseE2eTransportKeypairParameters,
  ) => Promise<ParseE2eTransportKeypairReturnType>;
  /** Serializes an e2e transport keypair to hex strings for storage. */
  readonly serializeE2eTransportKeypair: (
    parameters: SerializeE2eTransportKeypairParameters,
  ) => SerializeE2eTransportKeypairReturnType;
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
    publicDecrypt: (parameters) => publicDecrypt(fhevm, parameters),
    readPublicValue: (parameters) => publicDecrypt(fhevm, parameters),
    signDecryptionPermit: ((
      parameters: SignSelfDecryptionPermitParameters | SignDelegatedDecryptionPermitParameters,
    ) => {
      if (parameters.delegatorAddress !== undefined) {
        return signDecryptionPermit(fhevm, parameters);
      }
      return signDecryptionPermit(fhevm, parameters);
    }) as BaseActions['signDecryptionPermit'],
    parseE2eTransportKeypair: (parameters) => parseE2eTransportKeypair(fhevm, parameters),
    serializeE2eTransportKeypair: (parameters) => serializeE2eTransportKeypair(fhevm, parameters),
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
