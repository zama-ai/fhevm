import type {
  PublicEncKeyMlKem512,
  PrivateEncKeyMlKem512,
  ServerIdAddr,
  Client,
  TypedPlaintext,
} from '../../../../wasm/tkms/kms_lib.v0.13.10.js';
import type { FhevmRuntime } from '../../../types/coreFhevmRuntime.js';
import type { ClearValue } from '../../../types/encryptedTypes-p.js';
import type { TkmsPrivateKey, TkmsPrivateKeyBrand } from '../../../types/tkms-p.js';
import type { Bytes, BytesHex, BytesHexNo0x } from '../../../types/primitives.js';
import type {
  DecryptAndReconstructParameters,
  DecryptAndReconstructReturnType,
  DeserializeTkmsPrivateKeyParameters,
  DeserializeTkmsPrivateKeyReturnType,
  GenerateTkmsPrivateKeyReturnType,
  GetTkmsPublicKeyHexParameters,
  GetTkmsPublicKeyHexReturnType,
  SerializeTkmsPrivateKeyParameters,
  SerializeTkmsPrivateKeyReturnType,
  VerifyTkmsPrivateKeyParameters,
} from '../types.js';
import type { KmsSigncryptedShare, KmsSigncryptedSharesMetadata } from '../../../types/kms-p.js';
import type { KmsEip712Domain } from '../../../types/kms.js';
import type { ChecksummedAddress } from '../../../types/primitives.js';
import { uint32ToBytes32 } from '../../../base/uint.js';
import { createClearValue } from '../../../handle/ClearValue.js';
import { bytesToClearValueType } from '../../../handle/FheType.js';
import { ensure0x, remove0x } from '../../../base/string.js';
import { assertIsKmsExtraData } from '../../../kms/kmsExtraData.js';
import {
  ml_kem_pke_keygen,
  ml_kem_pke_get_pk,
  ml_kem_pke_pk_to_u8vec,
  new_server_id_addr,
  new_client,
  process_user_decryption_resp_from_js,
  ml_kem_pke_sk_to_u8vec,
  u8vec_to_ml_kem_pke_sk,
} from '../../../../wasm/tkms/kms_lib.v0.13.10.js';
import { bytesToHexLarge } from '../../../base/bytes.js';
import { initTkmsModule } from './init-p.js';
import { getMetadata, getShares } from '../../../kms/KmsSigncryptedShares-p.js';

////////////////////////////////////////////////////////////////////////////////

const GET_NATIVE_FUNC = Symbol('TKMSLib.getNative');
const PRIVATE_TKMS_LIB_TOKEN = Symbol('TKMSLib.token');

const GET_PUBLIC_KEY_FUNC = Symbol('TkmsPrivateEncKeyMlKem512.getPublicKey');
const GET_BYTES_HEX_FUNC = Symbol('TkmsPublicEncKeyMlKem512Impl.getBytesHexNo0x');

////////////////////////////////////////////////////////////////////////////////

// eslint-disable-next-line @typescript-eslint/naming-convention
declare const __PublicEncKeyMlKem512Wasm: unique symbol;
// eslint-disable-next-line @typescript-eslint/naming-convention
declare const __PrivateEncKeyMlKem512Wasm: unique symbol;
// eslint-disable-next-line @typescript-eslint/naming-convention
declare const __ServerIdAddrWasm: unique symbol;
// eslint-disable-next-line @typescript-eslint/naming-convention
declare const __ClientWasm: unique symbol;
// eslint-disable-next-line @typescript-eslint/naming-convention
declare const __TypedPlaintextWasm: unique symbol;
////////////////////////////////////////////////////////////////////////////////

type PublicEncKeyMlKem512WasmType = PublicEncKeyMlKem512 & {
  readonly [__PublicEncKeyMlKem512Wasm]: never;
};

type PrivateEncKeyMlKem512WasmType = PrivateEncKeyMlKem512 & {
  readonly [__PrivateEncKeyMlKem512Wasm]: never;
};

type ServerIdAddrWasmType = ServerIdAddr & {
  readonly [__ServerIdAddrWasm]: never;
};

type ClientWasmType = Client & { readonly [__ClientWasm]: never };

type TypedPlaintextWasmType = TypedPlaintext & {
  readonly [__TypedPlaintextWasm]: never;
};

type KmsEIP712DomainWasmType = Readonly<
  Omit<KmsEip712Domain, 'chainId' | 'verifyingContract'> & {
    readonly chain_id: Uint8Array;
    readonly verifying_contract: ChecksummedAddress;
    readonly salt: null;
  }
>;

function verifyToken(token: symbol): void {
  if (token !== PRIVATE_TKMS_LIB_TOKEN) {
    throw new Error('Unauthorized');
  }
}

////////////////////////////////////////////////////////////////////////////////
// TkmsPublicEncKeyMlKem512Impl
////////////////////////////////////////////////////////////////////////////////

class TkmsPublicEncKeyMlKem512Impl {
  readonly #publicEncKeyMlKem512Wasm: PublicEncKeyMlKem512WasmType;
  #bytesHex: BytesHex | undefined;

  constructor(token: symbol, publicEncKeyMlKem512Wasm: PublicEncKeyMlKem512WasmType) {
    verifyToken(token);
    this.#publicEncKeyMlKem512Wasm = publicEncKeyMlKem512Wasm;
  }

  public static [GET_NATIVE_FUNC](key: unknown, token: symbol): PublicEncKeyMlKem512WasmType {
    verifyToken(token);
    if (!(key instanceof TkmsPublicEncKeyMlKem512Impl)) {
      throw new Error('Unauthorized');
    }
    return key.#publicEncKeyMlKem512Wasm;
  }

  public static [GET_BYTES_HEX_FUNC](key: unknown, token: symbol): BytesHex {
    verifyToken(token);
    if (!(key instanceof TkmsPublicEncKeyMlKem512Impl)) {
      throw new Error('Unauthorized');
    }
    if (key.#bytesHex === undefined) {
      const bytes: Bytes = ml_kem_pke_pk_to_u8vec(key.#publicEncKeyMlKem512Wasm);
      key.#bytesHex = bytesToHexLarge(bytes, false /* no 0x */);
    }
    return key.#bytesHex;
  }
}

////////////////////////////////////////////////////////////////////////////////
// TkmsPrivateEncKeyMlKem512Impl
////////////////////////////////////////////////////////////////////////////////

class TkmsPrivateEncKeyMlKem512Impl implements TkmsPrivateKey {
  declare readonly [TkmsPrivateKeyBrand]: never;

  readonly #privateEncKeyMlKem512Wasm: PrivateEncKeyMlKem512WasmType;
  #publicKey: TkmsPublicEncKeyMlKem512Impl | undefined;

  constructor(token: symbol, privateEncKeyMlKem512Wasm: PrivateEncKeyMlKem512WasmType) {
    verifyToken(token);
    this.#privateEncKeyMlKem512Wasm = privateEncKeyMlKem512Wasm;
  }

  public static [GET_NATIVE_FUNC](key: unknown, token: symbol): PrivateEncKeyMlKem512WasmType {
    verifyToken(token);
    if (!(key instanceof TkmsPrivateEncKeyMlKem512Impl)) {
      throw new Error('Unauthorized');
    }
    return key.#privateEncKeyMlKem512Wasm;
  }

  public static [GET_PUBLIC_KEY_FUNC](key: unknown, token: symbol): TkmsPublicEncKeyMlKem512Impl {
    verifyToken(token);
    if (!(key instanceof TkmsPrivateEncKeyMlKem512Impl)) {
      throw new Error('Unauthorized');
    }
    if (key.#publicKey === undefined) {
      const publicEncKeyMlKem512Wasm = ml_kem_pke_get_pk(
        key.#privateEncKeyMlKem512Wasm,
      ) as PublicEncKeyMlKem512WasmType;
      key.#publicKey = new TkmsPublicEncKeyMlKem512Impl(token, publicEncKeyMlKem512Wasm);
    }
    return key.#publicKey;
  }
}

//////////////////////////////////////////////////////////////////////////////
// generateTkmsPrivateKey
//////////////////////////////////////////////////////////////////////////////

export async function generateTkmsPrivateKey(runtime: FhevmRuntime): Promise<GenerateTkmsPrivateKeyReturnType> {
  await initTkmsModule(runtime);
  const privateEncKeyMlKem512Wasm: PrivateEncKeyMlKem512WasmType = ml_kem_pke_keygen() as PrivateEncKeyMlKem512WasmType;

  return new TkmsPrivateEncKeyMlKem512Impl(PRIVATE_TKMS_LIB_TOKEN, privateEncKeyMlKem512Wasm);
}

////////////////////////////////////////////////////////////////////////////////
// decryptAndReconstruct
////////////////////////////////////////////////////////////////////////////////

export async function decryptAndReconstruct(
  runtime: FhevmRuntime,
  parameters: DecryptAndReconstructParameters,
): Promise<DecryptAndReconstructReturnType> {
  await initTkmsModule(runtime);

  const { tkmsPrivateKey, shares } = parameters;
  if (!(tkmsPrivateKey instanceof TkmsPrivateEncKeyMlKem512Impl)) {
    throw new Error('Invalid tkmsPrivateKey');
  }

  const tkmsPublicKey: TkmsPublicEncKeyMlKem512Impl = TkmsPrivateEncKeyMlKem512Impl[GET_PUBLIC_KEY_FUNC](
    tkmsPrivateKey,
    PRIVATE_TKMS_LIB_TOKEN,
  );

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

  const aggRespWasmArg: ReadonlyArray<Omit<KmsSigncryptedShare, 'extraData'> & { extra_data: BytesHexNo0x }> =
    sharesArray.map((s) => {
      return {
        signature: s.signature,
        payload: s.payload,
        extra_data: s.extraData,
      };
    });

  const privateEncKeyMlKem512Wasm: PrivateEncKeyMlKem512WasmType = TkmsPrivateEncKeyMlKem512Impl[GET_NATIVE_FUNC](
    tkmsPrivateKey,
    PRIVATE_TKMS_LIB_TOKEN,
  );

  const publicEncKeyMlKem512Wasm: PublicEncKeyMlKem512WasmType = TkmsPublicEncKeyMlKem512Impl[GET_NATIVE_FUNC](
    tkmsPublicKey,
    PRIVATE_TKMS_LIB_TOKEN,
  );

  const publicEncKeyMlKem512WasmBytesHex: BytesHex = TkmsPublicEncKeyMlKem512Impl[GET_BYTES_HEX_FUNC](
    tkmsPublicKey,
    PRIVATE_TKMS_LIB_TOKEN,
  );

  // KmsEIP712Domain
  const kmsEIP712Domain: KmsEip712Domain = metadata.eip712Domain;
  const clientAddress: ChecksummedAddress = metadata.eip712SignerAddress;

  // To be modified! use uint64ToBytes32 instead
  const eip712DomainWasmArg: KmsEIP712DomainWasmType = {
    name: kmsEIP712Domain.name,
    version: kmsEIP712Domain.version,
    chain_id: uint32ToBytes32(kmsEIP712Domain.chainId), // gateway chainId
    verifying_contract: kmsEIP712Domain.verifyingContract,
    salt: null,
  };

  //////////////////////////////////////////////////////////////////////////////
  // Important:
  // assume the KMS Signers have the correct order
  //////////////////////////////////////////////////////////////////////////////
  const indexedServerAddressesWasm: ServerIdAddrWasmType[] = metadata.kmsSignersContext.signers.map(
    (kmsSigner, index) => {
      const kmsSignerPartyId = index + 1;
      return new_server_id_addr(kmsSignerPartyId, kmsSigner) as ServerIdAddrWasmType;
    },
  );

  const clientWasm: ClientWasmType = new_client(indexedServerAddressesWasm, clientAddress, 'default') as ClientWasmType;

  const requestWasmArg = {
    signature: remove0x(metadata.eip712Signature),
    client_address: clientAddress,
    enc_key: remove0x(publicEncKeyMlKem512WasmBytesHex),
    ciphertext_handles: metadata.handles.map((h) => h.bytes32HexNo0x),
    eip712_verifying_contract: metadata.eip712Domain.verifyingContract,
    extra_data: remove0x(extraData),
  };

  // 1. Call kms module to decrypt & reconstruct clear values
  const typedPlaintextArray: TypedPlaintextWasmType[] = process_user_decryption_resp_from_js(
    clientWasm, // client argument
    requestWasmArg, // request argument
    eip712DomainWasmArg, // eip712_domain argument
    aggRespWasmArg, // agg_resp argument
    publicEncKeyMlKem512Wasm, // enc_pk argument
    privateEncKeyMlKem512Wasm, // enc_sk argument
    true, // verify argument
  ) as TypedPlaintextWasmType[];

  // 2. Build an unforgeable structure that contains the decrypted FhevmHandles
  const orderedClearValues: readonly ClearValue[] = typedPlaintextArray.map(
    (typedPlaintext: TypedPlaintextWasmType, idx: number) => {
      const fhevmHandle = metadata.handles[idx];
      if (fhevmHandle === undefined) {
        throw new Error('Internal error');
      }
      if (typedPlaintext.fhe_type !== fhevmHandle.fheTypeId) {
        throw new Error('Internal error');
      }
      return createClearValue({
        value: bytesToClearValueType(fhevmHandle.fheType, typedPlaintext.bytes),
        handle: fhevmHandle,
        originToken: PRIVATE_TKMS_LIB_TOKEN, // origin token for authenticity assertion
      });
    },
  );
  Object.freeze(orderedClearValues);

  return orderedClearValues;
}

//////////////////////////////////////////////////////////////////////////////
// getTkmsPublicKeyHex
//////////////////////////////////////////////////////////////////////////////

export async function getTkmsPublicKeyHex(
  runtime: FhevmRuntime,
  parameters: GetTkmsPublicKeyHexParameters,
): Promise<GetTkmsPublicKeyHexReturnType> {
  await initTkmsModule(runtime);

  const { tkmsPrivateKey } = parameters;

  if (!(tkmsPrivateKey instanceof TkmsPrivateEncKeyMlKem512Impl)) {
    throw new Error('Invalid tkmsPrivateKey');
  }

  const publicKey = TkmsPrivateEncKeyMlKem512Impl[GET_PUBLIC_KEY_FUNC](tkmsPrivateKey, PRIVATE_TKMS_LIB_TOKEN);

  return TkmsPublicEncKeyMlKem512Impl[GET_BYTES_HEX_FUNC](publicKey, PRIVATE_TKMS_LIB_TOKEN);
}

//////////////////////////////////////////////////////////////////////////////
// serializeTkmsPrivateKey
//////////////////////////////////////////////////////////////////////////////

export async function serializeTkmsPrivateKey(
  runtime: FhevmRuntime,
  parameters: SerializeTkmsPrivateKeyParameters,
): Promise<SerializeTkmsPrivateKeyReturnType> {
  await initTkmsModule(runtime);

  const { tkmsPrivateKey } = parameters;

  if (!(tkmsPrivateKey instanceof TkmsPrivateEncKeyMlKem512Impl)) {
    throw new Error('Invalid tkmsPrivateKey');
  }

  const privateEncKeyMlKem512Wasm: PrivateEncKeyMlKem512WasmType = TkmsPrivateEncKeyMlKem512Impl[GET_NATIVE_FUNC](
    tkmsPrivateKey,
    PRIVATE_TKMS_LIB_TOKEN,
  );

  return ml_kem_pke_sk_to_u8vec(privateEncKeyMlKem512Wasm);
}

//////////////////////////////////////////////////////////////////////////////
// deserializeTkmsPrivateKey
//////////////////////////////////////////////////////////////////////////////

export async function deserializeTkmsPrivateKey(
  runtime: FhevmRuntime,
  parameters: DeserializeTkmsPrivateKeyParameters,
): Promise<DeserializeTkmsPrivateKeyReturnType> {
  await initTkmsModule(runtime);

  const { tkmsPrivateKeyBytes } = parameters;

  const privateEncKeyMlKem512Wasm: PrivateEncKeyMlKem512WasmType = u8vec_to_ml_kem_pke_sk(
    tkmsPrivateKeyBytes,
  ) as PrivateEncKeyMlKem512WasmType;

  return new TkmsPrivateEncKeyMlKem512Impl(PRIVATE_TKMS_LIB_TOKEN, privateEncKeyMlKem512Wasm);
}

//////////////////////////////////////////////////////////////////////////////
// verifyTkmsPrivateKey
//////////////////////////////////////////////////////////////////////////////

export function verifyTkmsPrivateKey(_runtime: FhevmRuntime, parameters: VerifyTkmsPrivateKeyParameters): void {
  if (!(parameters.tkmsPrivateKey instanceof TkmsPrivateEncKeyMlKem512Impl)) {
    throw new Error('Invalid TkmsPrivateKey');
  }
}
