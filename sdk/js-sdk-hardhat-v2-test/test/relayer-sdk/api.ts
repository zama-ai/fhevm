import {
  RelayerInputProofProgressArgs,
  RelayerPublicDecryptProgressArgs,
  RelayerUserDecryptProgressArgs,
} from '@fhevm/sdk/core/types/relayer';
import { createKmsDelegatedUserDecryptEip712, createKmsUserDecryptEip712 } from '@fhevm/sdk/actions/chain';
import { createFhevmClient, hasFhevmRuntimeConfig, setFhevmRuntimeConfig } from '@fhevm/sdk/ethers';
import { createFhevmCleartextClient } from '@fhevm/sdk/ethers/cleartext';
import {
  FhevmConfigType,
  FhevmInstance,
  FhevmInstanceConfig,
  KeypairType,
  KmsDelegatedUserDecryptEip712,
  KmsUserDecryptEip712,
  RelayerEncryptedInput,
  ZKProofLike,
  Auth as RelayerSdkAuth,
} from './types';
import { TypedValueLike } from '@fhevm/sdk/core/types/primitives';
import { FhevmClient, FhevmEncryptClient } from '@fhevm/sdk/core/types/fhevmClient';
import { EncryptionBits } from '@fhevm/sdk/core/types/fheType';
import { JsonRpcProvider, getBytes, hexlify } from 'ethers';
import { defineFhevmChain } from '@fhevm/sdk/chains';
import { Auth } from '@fhevm/sdk/core/types/auth';
import {
  SignedDelegatedDecryptionPermit,
  SignedSelfDecryptionPermit,
} from '@fhevm/sdk/core/types/signedDecryptionPermit';

////////////////////////////////////////////////////////////////////////////////

export async function createInstance(config: FhevmInstanceConfig): Promise<FhevmInstance> {
  const instance = await FhevmInstanceImpl.create(config);
  return instance;
}

////////////////////////////////////////////////////////////////////////////////

class RelayerEncryptedInputImpl implements RelayerEncryptedInput {
  #encryptClient: FhevmEncryptClient;
  #typedValues: TypedValueLike[];
  #encryptionBits: EncryptionBits[];
  #totalBits: number;
  #userAddress: string;
  #contractAddress: string;

  constructor(encryptClient: FhevmEncryptClient, contractAddress: string, userAddress: string) {
    this.#encryptClient = encryptClient;
    this.#contractAddress = contractAddress;
    this.#userAddress = userAddress;
    this.#totalBits = 0;
    this.#typedValues = [];
    this.#encryptionBits = [];
  }
  check() {
    if (this.#encryptionBits.length > 255) {
      throw new Error('Packing more than 255 variables in a single input ciphertext is unsupported');
    }
    if (this.#totalBits > 2048) {
      throw new Error('Packing more than 2048 bits in a single input ciphertext is unsupported');
    }
  }
  addBool(value: boolean | number | bigint): RelayerEncryptedInput {
    this.#typedValues.push({ type: 'bool', value });
    this.#encryptionBits.push(2);
    this.#totalBits += 2;
    this.check();
    return this;
  }
  add8(value: number | bigint): RelayerEncryptedInput {
    this.#typedValues.push({ type: 'uint8', value });
    this.#encryptionBits.push(8);
    this.#totalBits += 8;
    this.check();
    return this;
  }
  add16(value: number | bigint): RelayerEncryptedInput {
    this.#typedValues.push({ type: 'uint16', value });
    this.#encryptionBits.push(16);
    this.#totalBits += 16;
    this.check();
    return this;
  }
  add32(value: number | bigint): RelayerEncryptedInput {
    this.#typedValues.push({ type: 'uint32', value });
    this.#encryptionBits.push(32);
    this.#totalBits += 32;
    this.check();
    return this;
  }
  add64(value: number | bigint): RelayerEncryptedInput {
    this.#typedValues.push({ type: 'uint64', value });
    this.#encryptionBits.push(64);
    this.#totalBits += 64;
    this.check();
    return this;
  }
  add128(value: number | bigint): RelayerEncryptedInput {
    this.#typedValues.push({ type: 'uint128', value });
    this.#encryptionBits.push(128);
    this.#totalBits += 128;
    this.check();
    return this;
  }
  add256(value: number | bigint): RelayerEncryptedInput {
    this.#typedValues.push({ type: 'uint256', value });
    this.#encryptionBits.push(256);
    this.#totalBits += 256;
    this.check();
    return this;
  }
  addAddress(value: string): RelayerEncryptedInput {
    this.#typedValues.push({ type: 'address', value });
    this.#encryptionBits.push(160);
    this.#totalBits += 160;
    this.check();
    return this;
  }
  getBits(): EncryptionBits[] {
    return this.#encryptionBits;
  }
  generateZKProof(): {
    readonly chainId: bigint;
    readonly aclContractAddress: `0x${string}`;
    readonly contractAddress: `0x${string}`;
    readonly userAddress: `0x${string}`;
    readonly ciphertextWithZKProof: Uint8Array | string;
    readonly encryptionBits: readonly (2 | 8 | 16 | 32 | 64 | 128 | 160 | 256)[];
  } {
    throw new Error('Method not implemented.');
  }
  async encrypt(options?: {
    auth?: RelayerSdkAuth | undefined;
    debug?: boolean | undefined;
    signal?: AbortSignal | undefined;
    timeout?: number | undefined;
    onProgress?: ((args: RelayerInputProofProgressArgs) => void) | undefined;
  }): Promise<{ handles: Uint8Array[]; inputProof: Uint8Array }> {
    const res = await this.#encryptClient.encryptValues({
      values: this.#typedValues,
      contractAddress: this.#contractAddress,
      userAddress: this.#userAddress,
      options: options ? { ...options, auth: toSdkAuth(options.auth) } : undefined,
    });
    return {
      handles: res.encryptedValues.map((ev) => getBytes(ev)),
      inputProof: getBytes(res.inputProof),
    };
  }
}

////////////////////////////////////////////////////////////////////////////////
// Auth field-rename: relayer-sdk uses `__type`, fhevm/sdk uses `type`.

function toSdkAuth(auth: RelayerSdkAuth | undefined): Auth | undefined {
  if (auth === undefined) {
    return undefined;
  }
  switch (auth.__type) {
    case 'BearerToken':
      return { type: 'BearerToken', token: auth.token };
    case 'ApiKeyHeader':
      return { type: 'ApiKeyHeader', header: auth.header, value: auth.value };
    case 'ApiKeyCookie':
      return { type: 'ApiKeyCookie', cookie: auth.cookie, value: auth.value };
  }
}

////////////////////////////////////////////////////////////////////////////////

class FhevmInstanceImpl implements FhevmInstance {
  #fullClient: FhevmClient;

  constructor(fullClient: FhevmClient) {
    this.#fullClient = fullClient;
  }

  static async create(config: FhevmInstanceConfig): Promise<FhevmInstance> {
    if (!hasFhevmRuntimeConfig()) {
      setFhevmRuntimeConfig({
        singleThread: false,
        logger: {
          debug: (message: string) => console.log(message),
          error: (message: string, _cause: unknown) => console.error(message),
        },
      });
    }
    const fullClient = createFhevmCleartextClient({
      provider: new JsonRpcProvider(config.network as string),
      chain: defineFhevmChain({
        id: config.chainId,
        fhevm: {
          contracts: {
            acl: { address: config.aclContractAddress as `0x${string}` },
            inputVerifier: { address: config.inputVerifierContractAddress as `0x${string}` },
            kmsVerifier: { address: config.kmsContractAddress as `0x${string}` },
          },
          relayerUrl: config.relayerUrl,
          gateway: {
            id: config.gatewayChainId,
            contracts: {
              decryption: {
                address: config.verifyingContractAddressDecryption as `0x${string}`,
              },
              inputVerification: {
                address: config.verifyingContractAddressInputVerification as `0x${string}`,
              },
            },
          },
        },
      }),
    });
    await fullClient.ready;
    return new FhevmInstanceImpl(fullClient);
  }

  get config(): FhevmConfigType {
    throw new Error('Method not implemented.');
  }

  createEncryptedInput(contractAddress: string, userAddress: string): RelayerEncryptedInput {
    return new RelayerEncryptedInputImpl(this.#fullClient, contractAddress, userAddress);
  }

  requestZKProofVerification(
    zkProof: ZKProofLike,
    options?: {
      auth?: RelayerSdkAuth | undefined;
      debug?: boolean | undefined;
      signal?: AbortSignal | undefined;
      timeout?: number | undefined;
      onProgress?: ((args: RelayerInputProofProgressArgs) => void) | undefined;
    }
  ): Promise<Readonly<{ handles: Uint8Array[]; inputProof: Uint8Array }>> {
    throw new Error('Method not implemented.');
  }

  generateKeypair(): KeypairType<string> {
    throw new Error('Method not implemented.');
  }

  async generateKeypairAsync(): Promise<KeypairType<string>> {
    const p = await this.#fullClient.generateTransportKeypair();
    return this.#fullClient.serializeTransportKeypair({ transportKeypair: p });
  }

  getExtraData(): Promise<`0x${string}`> {
    throw new Error('Method not implemented.');
  }

  createEIP712(
    publicKey: string,
    contractAddresses: string[],
    startTimestamp: number,
    durationDays: number,
    extraData: `0x${string}`
  ): KmsUserDecryptEip712 {
    return createKmsUserDecryptEip712(this.#fullClient, {
      contractAddresses,
      durationDays,
      startTimestamp,
      extraData,
      publicKey,
    });
  }

  createDelegatedUserDecryptEIP712(
    publicKey: string,
    contractAddresses: string[],
    delegatorAddress: string,
    startTimestamp: number,
    durationDays: number,
    extraData: `0x${string}`
  ): KmsDelegatedUserDecryptEip712 {
    return createKmsDelegatedUserDecryptEip712(this.#fullClient, {
      contractAddresses,
      durationDays,
      startTimestamp,
      extraData,
      publicKey,
      delegatorAddress,
    });
  }

  async publicDecrypt(
    handles: (string | Uint8Array)[],
    options?: {
      auth?: RelayerSdkAuth | undefined;
      debug?: boolean | undefined;
      signal?: AbortSignal | undefined;
      timeout?: number | undefined;
      onProgress?: ((args: RelayerPublicDecryptProgressArgs) => void) | undefined;
    }
  ): Promise<
    Readonly<{
      clearValues: Readonly<Record<`0x${string}`, bigint | boolean | `0x${string}`>>;
      abiEncodedClearValues: `0x${string}`;
      decryptionProof: `0x${string}`;
    }>
  > {
    const res = await this.#fullClient.readPublicValuesWithSignatures({
      encryptedValues: handles,
      options: options ? { ...options, auth: toSdkAuth(options.auth) } : undefined,
    });

    const clearValues: Record<`0x${string}`, bigint | boolean | `0x${string}`> = {};
    for (let i = 0; i < handles.length; i++) {
      const h = handles[i];
      const handleHex = (typeof h === 'string' ? h : hexlify(h)) as `0x${string}`;
      const v = res.clearValues[i].value;
      clearValues[handleHex] = typeof v === 'number' ? BigInt(v) : v;
    }

    return {
      clearValues,
      abiEncodedClearValues: res.checkSignaturesArgs.abiEncodedCleartexts,
      decryptionProof: res.checkSignaturesArgs.decryptionProof,
    };
  }

  async userDecrypt(
    handles: { handle: Uint8Array | string; contractAddress: string }[],
    privateKey: string,
    publicKey: string,
    signature: string,
    contractAddresses: string[],
    userAddress: string,
    startTimestamp: number,
    durationDays: number,
    extraData: `0x${string}`,
    options?: {
      auth?: RelayerSdkAuth | undefined;
      debug?: boolean | undefined;
      signal?: AbortSignal | undefined;
      timeout?: number | undefined;
      onProgress?: ((args: RelayerUserDecryptProgressArgs) => void) | undefined;
    }
  ): Promise<Readonly<Record<`0x${string}`, bigint | boolean | `0x${string}`>>> {
    const transportKeypair = await this.#fullClient.parseTransportKeypair({ privateKey, publicKey });

    const eip712 = createKmsUserDecryptEip712(this.#fullClient, {
      contractAddresses,
      durationDays,
      startTimestamp,
      extraData,
      publicKey,
    });

    const signedPermit = (await this.#fullClient.parseSignedDecryptionPermit({
      serialized: {
        eip712,
        signature,
        signerAddress: userAddress,
      },
      transportKeypair,
    })) as SignedSelfDecryptionPermit;

    const res = await this.#fullClient.decryptValuesFromPairs({
      pairs: handles.map((h) => ({ encryptedValue: h.handle, contractAddress: h.contractAddress })),
      transportKeypair,
      signedPermit,
      options: options ? { ...options, auth: toSdkAuth(options.auth) } : undefined,
    });

    const out: Record<`0x${string}`, bigint | boolean | `0x${string}`> = {};
    for (let i = 0; i < handles.length; i++) {
      const h = handles[i].handle;
      const handleHex = (typeof h === 'string' ? h : hexlify(h)) as `0x${string}`;
      const v = res[i].value;
      out[handleHex] = typeof v === 'number' ? BigInt(v) : v;
    }
    return out;
  }

  async delegatedUserDecrypt(
    handleContractPairs: { handle: Uint8Array | string; contractAddress: string }[],
    privateKey: string,
    publicKey: string,
    signature: string,
    contractAddresses: string[],
    delegatorAddress: string,
    delegateAddress: string,
    startTimestamp: number,
    durationDays: number,
    extraData: `0x${string}`,
    options?: {
      auth?: RelayerSdkAuth | undefined;
      debug?: boolean | undefined;
      signal?: AbortSignal | undefined;
      timeout?: number | undefined;
      onProgress?: ((args: RelayerUserDecryptProgressArgs) => void) | undefined;
    }
  ): Promise<Readonly<Record<`0x${string}`, bigint | boolean | `0x${string}`>>> {
    const transportKeypair = await this.#fullClient.parseTransportKeypair({ privateKey, publicKey });

    const eip712 = createKmsDelegatedUserDecryptEip712(this.#fullClient, {
      contractAddresses,
      durationDays,
      startTimestamp,
      extraData,
      publicKey,
      delegatorAddress,
    });

    const signedPermit = (await this.#fullClient.parseSignedDecryptionPermit({
      serialized: {
        eip712,
        signature,
        signerAddress: delegateAddress,
      },
      transportKeypair,
    })) as SignedDelegatedDecryptionPermit;

    const res = await this.#fullClient.decryptValuesFromPairs({
      pairs: handleContractPairs.map((h) => ({ encryptedValue: h.handle, contractAddress: h.contractAddress })),
      transportKeypair,
      signedPermit: signedPermit as never,
      options: options ? { ...options, auth: toSdkAuth(options.auth) } : undefined,
    });

    const out: Record<`0x${string}`, bigint | boolean | `0x${string}`> = {};
    for (let i = 0; i < handleContractPairs.length; i++) {
      const h = handleContractPairs[i].handle;
      const handleHex = (typeof h === 'string' ? h : hexlify(h)) as `0x${string}`;
      const v = res[i].value;
      out[handleHex] = typeof v === 'number' ? BigInt(v) : v;
    }
    return out;
  }

  getPublicKey(): { publicKeyId: string; publicKey: Uint8Array } | null {
    throw new Error('Method not implemented.');
  }

  getPublicParams(bits: 2048): { publicParams: Uint8Array; publicParamsId: string } | null {
    throw new Error('Method not implemented.');
  }
}
