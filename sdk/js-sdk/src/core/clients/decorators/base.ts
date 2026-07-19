import type { Fhevm, FhevmBase, FhevmExtension } from '../../types/coreFhevmClient.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { SignedDecryptionPermit } from '../../types/signedDecryptionPermit.js';
import type {
  SerializeTransportKeyPairParameters,
  SerializeTransportKeyPairReturnType,
} from '../../actions/chain/serializeTransportKeyPair.js';
import { assertIsFhevmBaseClient } from '../../runtime/CoreFhevm-p.js';
import { signDecryptionPermit, type SignDecryptionPermitParameters } from '../../actions/base/signDecryptionPermit.js';
import {
  parseTransportKeyPair,
  type ParseTransportKeyPairParameters,
  type ParseTransportKeyPairReturnType,
} from '../../actions/chain/parseTransportKeyPair.js';
import {
  fetchFheEncryptionKeyBytes,
  type FetchFheEncryptionKeyBytesParameters,
  type FetchFheEncryptionKeyBytesReturnType,
} from '../../actions/chain/fetchFheEncryptionKeyBytes.js';
import { serializeTransportKeyPair } from '../../actions/chain/serializeTransportKeyPair.js';
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
  decryptPublicValue,
  type DecryptPublicValueParameters,
  type DecryptPublicValueReturnType,
} from '../../actions/base/decryptPublicValue.js';
import {
  decryptPublicValues,
  type DecryptPublicValuesParameters,
  type DecryptPublicValuesReturnType,
} from '../../actions/base/decryptPublicValues.js';
import {
  decryptPublicValuesWithSignatures,
  type DecryptPublicValuesWithSignaturesParameters,
  type DecryptPublicValuesWithSignaturesReturnType,
} from '../../actions/base/decryptPublicValuesWithSignatures.js';
import { ensureResolvedProtocolVersion } from '../../runtime/resolveFhevmVersions-p.js';
import {
  signLegacyDecryptionPermit,
  type SignLegacyDecryptionPermitParameters,
  type SignLegacyDecryptionPermitReturnType,
} from '../../actions/base/signLegacyDecryptionPermit.js';

////////////////////////////////////////////////////////////////////////////////

export type BaseActions = {
  /**
   * Reads the decrypted (clear) value of an encrypted value that was made public.
   *
   * By default, encrypted values on-chain can only be decrypted by their owner.
   * When a contract calls `TFHE.allowForDecryption(encryptedValue)` in Solidity,
   * the value becomes publicly readable. Anyone can then call `decryptPublicValue`
   * to get the clear value — no permit or private key needed.
   *
   * @example
   * ```ts
   * const result = await client.decryptPublicValue({
   *   encryptedValue: "0xabcd..."
   * });
   * console.log(result.value); // e.g. 42
   * console.log(result.type); // e.g. 'uint8'
   * ```
   */
  readonly decryptPublicValue: (parameters: DecryptPublicValueParameters) => Promise<DecryptPublicValueReturnType>;
  /**
   * Reads the decrypted (clear) values of encrypted values that were made public.
   *
   * By default, encrypted values on-chain can only be decrypted by their owner.
   * When a contract calls `TFHE.allowForDecryption(encryptedValue)` in Solidity,
   * a value becomes publicly readable. Anyone can then call `decryptPublicValues`
   * to get the clear values — no permit or private key needed.
   *
   * @example
   * ```ts
   * const results = await client.decryptPublicValues({
   *   encryptedValues: ["0xabcd...", "0xef..."]
   * });
   * console.log(results[0].value); // e.g. 42
   * console.log(results[0].type); // e.g. 'uint8'
   * console.log(results[1].value); // e.g. 123456789
   * console.log(results[1].type); // e.g. 'uint32'
   * ```
   */
  readonly decryptPublicValues: (parameters: DecryptPublicValuesParameters) => Promise<DecryptPublicValuesReturnType>;
  /**
   * Reads the decrypted (clear) value of an encrypted value that was made public.
   *
   * By default, encrypted values on-chain can only be decrypted by their owner.
   * When a contract calls `TFHE.allowForDecryption(encryptedValue)` in Solidity,
   * the value becomes publicly readable. Anyone can then call `decryptPublicValue`
   * to get the clear value — no permit or private key needed.
   *
   * Returns both the clear values and a decryption proof. The proof can be
   * forwarded on-chain so a smart contract can verify that an encrypted value
   * decrypts to a specific clear value (e.g. "handle X is actually 123").
   *
   * @example
   * ```ts
   * const result = await client.decryptPublicValuesWithSignatures({
   *   encryptedValues: ["0xabcd...", "0xef..."]
   * });
   * console.log(result.clearValues[0].value); // e.g. 42
   * console.log(result.clearValues[0].type); // e.g. 'uint8'
   * console.log(result.clearValues[1].value); // e.g. 123456789
   * console.log(result.clearValues[1].type); // e.g. 'uint32'
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
  readonly decryptPublicValuesWithSignatures: (
    parameters: DecryptPublicValuesWithSignaturesParameters,
  ) => Promise<DecryptPublicValuesWithSignaturesReturnType>;
  /**
   * Signs a decryption permit.
   *
   * @deprecated Will be deprecated in the next version. Use
   * {@link signLegacyDecryptionPermit} instead, which pins the V1 permit shape
   * and stays stable across SDK protocol-API upgrades.
   */
  readonly signDecryptionPermit: (parameters: SignDecryptionPermitParameters) => Promise<SignedDecryptionPermit>;
  /**
   * Signs a legacy decryption permit.
   *
   * - Without `delegatorAddress`: Alice signs to decrypt her own encrypted values.
   * - With `delegatorAddress`: Bob signs to decrypt values belonging to `delegatorAddress` (Alice),
   *   after Alice has granted permission via `FHE.delegateUserDecryption()` on-chain.
   *
   * Inspect `isDelegated` on the returned permit to distinguish the two cases.
   */
  readonly signLegacyDecryptionPermit: (
    parameters: SignLegacyDecryptionPermitParameters,
  ) => Promise<SignLegacyDecryptionPermitReturnType>;
  /** Deserializes a previously serialized e2e transport key pair back into a usable key pair. */
  readonly parseTransportKeyPair: (
    parameters: ParseTransportKeyPairParameters,
  ) => Promise<ParseTransportKeyPairReturnType>;
  /** Serializes an e2e transport key pair to hex strings for storage. */
  readonly serializeTransportKeyPair: (
    parameters: SerializeTransportKeyPairParameters,
  ) => SerializeTransportKeyPairReturnType;
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
    decryptPublicValue: (parameters) => decryptPublicValue(fhevm, parameters),
    decryptPublicValues: (parameters) => decryptPublicValues(fhevm, parameters),
    decryptPublicValuesWithSignatures: (parameters) => decryptPublicValuesWithSignatures(fhevm, parameters),
    signDecryptionPermit: (parameters) => signDecryptionPermit(fhevm, parameters),
    signLegacyDecryptionPermit: (parameters) => signLegacyDecryptionPermit(fhevm, parameters),
    parseTransportKeyPair: (parameters) => parseTransportKeyPair(fhevm, parameters),
    serializeTransportKeyPair: (parameters) => serializeTransportKeyPair(fhevm, parameters),
    serializeSignedDecryptionPermit: (parameters) => serializeSignedDecryptionPermit(fhevm, parameters),
    parseSignedDecryptionPermit: (parameters) => parseSignedDecryptionPermit(fhevm, parameters),
    fetchFheEncryptionKeyBytes: (parameters) => fetchFheEncryptionKeyBytes(fhevm, parameters),
  };
}

////////////////////////////////////////////////////////////////////////////////

async function _initBase(fhevm: FhevmBase<FhevmChain>): Promise<void> {
  await ensureResolvedProtocolVersion(fhevm);
}

////////////////////////////////////////////////////////////////////////////////

export function baseActions(fhevm: FhevmBase<FhevmChain>): FhevmExtension<BaseActions> {
  assertIsFhevmBaseClient(fhevm);
  return {
    actions: _baseActions(fhevm),
    runtime: fhevm.runtime,
    init: _initBase,
  };
}
