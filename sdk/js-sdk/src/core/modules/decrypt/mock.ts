import type { FhevmRuntime } from '../../types/coreFhevmRuntime.js';
import type {
  DecryptAndReconstructParameters,
  DecryptAndReconstructReturnType,
  DecryptAndReconstructUserParameters,
  DeserializeTkmsPrivateKeyParameters,
  DeserializeTkmsPrivateKeyReturnType,
  GenerateTkmsPrivateKeyReturnType,
  GetTkmsPublicKeyHexParameters,
  GetTkmsPublicKeyHexReturnType,
  SerializeTkmsPrivateKeyParameters,
  SerializeTkmsPrivateKeyReturnType,
  VerifyTkmsPrivateKeyParameters,
  UserDecryptModuleParameters,
  DecryptModuleFactory,
  UserDecryptModuleFactory,
} from './types.js';
import type { CleartextEthereumModule } from '../ethereum/types-ct.js';
import type { TkmsPrivateKey } from '../../types/tkms-p.js';
import type { BytesHex } from '../../types/primitives.js';
import type { KmsSigncryptedShare, KmsSigncryptedSharesMetadata } from '../../types/kms-p.js';
import type { ClearValue } from '../../types/encryptedTypes-p.js';
import { assertIsKmsExtraData } from '../../kms/kmsExtraData.js';
import { ensure0x, remove0x } from '../../base/string.js';
import { getMetadata, getShares } from '../../kms/KmsSigncryptedShares-p.js';
import { asBytesHex, bytesToHex, hexToBytes } from '../../base/bytes.js';
import { bigintToClearValueType } from '../../handle/FheType.js';
import { createClearValue } from '../../handle/ClearValue.js';

////////////////////////////////////////////////////////////////////////////////

const PRIVATE_CLEARTEXT_TKMS_LIB_TOKEN = Symbol('CleartextTKMSLib.token');

////////////////////////////////////////////////////////////////////////////////

function _xorUnmaskWithPublicKey(publicKey: BytesHex, maskedCleartexts: readonly bigint[]): bigint[] {
  const hex = remove0x(publicKey);
  if (hex.length < 64) {
    throw new Error(`PublicKeyTooShort: publicKey has ${hex.length / 2} bytes, need >= 32`);
  }
  const mask = BigInt('0x' + hex.slice(0, 64));
  return maskedCleartexts.map((c) => c ^ mask);
}

////////////////////////////////////////////////////////////////////////////////
// decryptAndReconstruct
////////////////////////////////////////////////////////////////////////////////

export async function decryptAndReconstruct(
  runtime: FhevmRuntime,
  parameters: DecryptAndReconstructParameters,
): Promise<DecryptAndReconstructReturnType> {
  const cleartextEthereumModule = runtime.ethereum as CleartextEthereumModule;
  const { tkmsPrivateKey, shares } = parameters;

  const privateKeySecp256k1 = asBytesHex(tkmsPrivateKey);
  const publicKeySecp256k1 = cleartextEthereumModule.getPublicKey({ privateKey: privateKeySecp256k1 });

  const metadata: KmsSigncryptedSharesMetadata = getMetadata(shares);
  const sharesArray: readonly KmsSigncryptedShare[] = getShares(shares);

  const firstShare = sharesArray[0];
  if (firstShare === undefined) {
    throw new Error('Expected at least one signcrypted share.');
  }

  const firstExtraData = firstShare.extraData;
  for (let i = 1; i < sharesArray.length; i++) {
    const share = sharesArray[i];
    if (share !== undefined && share.extraData !== firstExtraData) {
      throw new Error(
        `Mismatched extraData across shares: share[0]="${firstExtraData}" vs share[${i}]="${share.extraData}".`,
      );
    }
  }

  const extraData: BytesHex = ensure0x(firstExtraData);
  assertIsKmsExtraData(extraData, {});

  for (let i = 0; i < sharesArray.length; ++i) {
    const s = sharesArray[i];
    if (!s) {
      throw new Error('Internal error');
    }
    const recoveredAddress = await cleartextEthereumModule.recoverAddress({
      hash: ensure0x(s.payload),
      signature: ensure0x(s.signature),
    });
    if (!metadata.kmsSignersContext.has(recoveredAddress)) {
      throw new Error(
        `Unknown KMS signer: recovered address ${recoveredAddress} from share[${i}] is not a registered KMS signer.`,
      );
    }
  }

  const decoded = cleartextEthereumModule.decode({
    encodedData: ensure0x(firstShare.payload),
    types: ['uint256[]', 'bytes'],
  });

  if (decoded.length !== 2) {
    throw new Error('Invalid decrypted result.');
  }

  const maskedCleartexts = decoded[0] as bigint[];
  const signedExtraData = decoded[1] as BytesHex;

  const cleartexts: bigint[] = _xorUnmaskWithPublicKey(publicKeySecp256k1, maskedCleartexts);

  if (extraData !== signedExtraData) {
    throw new Error(
      `extraData mismatch: share extraData="${extraData}" does not match the signed payload's extraData="${signedExtraData}".`,
    );
  }

  // 2. Build an unforgeable structure that contains the decrypted FhevmHandles
  const orderedClearValues: readonly ClearValue[] = cleartexts.map((plaintext: bigint, idx: number) => {
    const fhevmHandle = metadata.handles[idx];
    if (fhevmHandle === undefined) {
      throw new Error('Internal error');
    }
    return createClearValue({
      value: bigintToClearValueType(fhevmHandle.fheType, plaintext),
      handle: fhevmHandle,
      originToken: PRIVATE_CLEARTEXT_TKMS_LIB_TOKEN, // origin token for authenticity assertion
    });
  });
  Object.freeze(orderedClearValues);

  return orderedClearValues;
}

//////////////////////////////////////////////////////////////////////////////
// generateTkmsPrivateKey
//////////////////////////////////////////////////////////////////////////////

export async function generateTkmsPrivateKey(runtime: FhevmRuntime): Promise<GenerateTkmsPrivateKeyReturnType> {
  const cleartextEthereumModule = runtime.ethereum as CleartextEthereumModule;
  const privateKeySecp256k1 = cleartextEthereumModule.generatePrivateKey();
  return Promise.resolve(privateKeySecp256k1 as unknown as TkmsPrivateKey);
}

//////////////////////////////////////////////////////////////////////////////
// getTkmsPublicKeyHex
//////////////////////////////////////////////////////////////////////////////

export async function getTkmsPublicKeyHex(
  runtime: FhevmRuntime,
  parameters: GetTkmsPublicKeyHexParameters,
): Promise<GetTkmsPublicKeyHexReturnType> {
  const { tkmsPrivateKey } = parameters;
  const privateKeySecp256k1 = asBytesHex(tkmsPrivateKey);
  const cleartextEthereumModule = runtime.ethereum as CleartextEthereumModule;
  const publicKeySecp256k1 = cleartextEthereumModule.getPublicKey({ privateKey: privateKeySecp256k1 });
  return Promise.resolve(publicKeySecp256k1);
}

//////////////////////////////////////////////////////////////////////////////
// serializeTkmsPrivateKey
//////////////////////////////////////////////////////////////////////////////

export async function serializeTkmsPrivateKey(
  _runtime: FhevmRuntime,
  parameters: SerializeTkmsPrivateKeyParameters,
): Promise<SerializeTkmsPrivateKeyReturnType> {
  const { tkmsPrivateKey } = parameters;
  const privateKeySecp256k1 = asBytesHex(tkmsPrivateKey);
  const privateKeySecp256k1Bytes = hexToBytes(privateKeySecp256k1);
  return Promise.resolve(privateKeySecp256k1Bytes);
}

//////////////////////////////////////////////////////////////////////////////
// deserializeTkmsPrivateKey
//////////////////////////////////////////////////////////////////////////////

export async function deserializeTkmsPrivateKey(
  _runtime: FhevmRuntime,
  parameters: DeserializeTkmsPrivateKeyParameters,
): Promise<DeserializeTkmsPrivateKeyReturnType> {
  const { tkmsPrivateKeyBytes } = parameters;
  const privateKeySecp256k1 = bytesToHex(tkmsPrivateKeyBytes);
  return Promise.resolve(privateKeySecp256k1 as unknown as TkmsPrivateKey);
}

//////////////////////////////////////////////////////////////////////////////
// verifyTkmsPrivateKey
//////////////////////////////////////////////////////////////////////////////

export function verifyTkmsPrivateKey(_runtime: FhevmRuntime, _parameters: VerifyTkmsPrivateKeyParameters): void {
  throw new Error('Not yet implemented');
}

//////////////////////////////////////////////////////////////////////////////
// decryptActions
//////////////////////////////////////////////////////////////////////////////

export const decryptModule: DecryptModuleFactory = (runtime: FhevmRuntime) => {
  return Object.freeze({
    decrypt: Object.freeze({
      initTkmsModule: () => Promise.resolve(),
      getTkmsModuleInfo: () => {
        throw new Error('Not yet implemented');
      },
      generateTkmsPrivateKey: () => generateTkmsPrivateKey(runtime),
      decryptAndReconstruct: (args: DecryptAndReconstructParameters) => decryptAndReconstruct(runtime, args),
      serializeTkmsPrivateKey: (args: SerializeTkmsPrivateKeyParameters) => serializeTkmsPrivateKey(runtime, args),
      deserializeTkmsPrivateKey: (args: DeserializeTkmsPrivateKeyParameters) =>
        deserializeTkmsPrivateKey(runtime, args),
      verifyTkmsPrivateKey: (args: VerifyTkmsPrivateKeyParameters) => {
        verifyTkmsPrivateKey(runtime, args);
      },
      getTkmsPublicKeyHex: (args: GetTkmsPublicKeyHexParameters) => getTkmsPublicKeyHex(runtime, args),
    }),
  });
};

//////////////////////////////////////////////////////////////////////////////
// userDecryptActions
//////////////////////////////////////////////////////////////////////////////

export const userDecryptModule: UserDecryptModuleFactory = (
  runtime: FhevmRuntime,
  parameters: UserDecryptModuleParameters,
) => {
  const { privateKey } = parameters;
  return Object.freeze({
    userDecrypt: Object.freeze({
      initTkmsModule: () => Promise.resolve(),
      getTkmsModuleInfo: () => {
        throw new Error('Not yet implemented');
      },
      decryptAndReconstruct: (args: DecryptAndReconstructUserParameters) =>
        decryptAndReconstruct(runtime, {
          ...args,
          tkmsPrivateKey: privateKey,
        }),
      getTkmsPublicKeyHex: () =>
        getTkmsPublicKeyHex(runtime, {
          tkmsPrivateKey: privateKey,
        }),
      serializeTkmsPrivateKey: () =>
        serializeTkmsPrivateKey(runtime, {
          tkmsPrivateKey: privateKey,
        }),
    }),
  });
};
