import { defineFhevmChain } from '@fhevm/sdk/chains';
import { createFhevmClient, hasFhevmRuntimeConfig, setFhevmRuntimeConfig } from '@fhevm/sdk/ethers';
import type { Auth } from '@fhevm/sdk/types';
import type { Signer } from 'ethers';
import { JsonRpcProvider, getBytes, hexlify } from 'ethers';

import type {
  ClearValueType,
  ClearValues,
  EncryptedInputResult,
  Auth as RelayerSdkAuth,
  SdkInstance,
  TypedValue,
} from '../types';

type FhevmClient = ReturnType<typeof createFhevmClient>;
type CreateFhevmClientParameters = Parameters<typeof createFhevmClient>[0];
type FhevmClientProvider = CreateFhevmClientParameters['provider'];

export class FhevmSdk implements SdkInstance {
  #fullClient: FhevmClient;
  #auth: Auth | undefined;

  constructor(fullClient: FhevmClient, auth: Auth | undefined) {
    this.#fullClient = fullClient;
    this.#auth = auth;
  }

  getUserDecryptErrorMessage(parameters: {
    readonly type: 'user-unauthorized' | 'user-equal-contract' | 'contract-unauthorized' | 'permit-expired';
    readonly signer: Signer & { readonly address: string };
    readonly handle?: string | undefined;
    readonly contractAddress?: string | undefined;
  }): string {
    if (parameters.type === 'user-unauthorized') {
      return `User ${parameters.signer.address} is not authorized to decrypt handle ${parameters.handle}!`;
    } else if (parameters.type === 'user-equal-contract') {
      return `userAddress ${parameters.signer.address} should not be equal to contractAddress when requesting user decryption!`;
    } else if (parameters.type === 'contract-unauthorized') {
      return `Dapp contract ${parameters.contractAddress} is not authorized to user decrypt handle ${parameters.handle}!`;
    } else if (parameters.type === 'permit-expired') {
      return 'request has expired';
    } else {
      return 'unknown error type';
    }
  }

  getDelegatedUserDecryptErrorMessage(parameters: {
    readonly type: 'revocation' | 'contract-unauthorized' | 'permit-expired' | 'delegation-does-not-exist';
    readonly signer: Signer & { readonly address: string };
    readonly handle?: string | undefined;
    readonly contractAddress?: string | undefined;
    readonly delegatorAddress?: string | undefined;
  }): string {
    if (parameters.type === 'revocation' || parameters.type === 'delegation-does-not-exist') {
      return `Delegate ${parameters.signer.address} is not delegated by ${parameters.delegatorAddress} to user decrypt handle ${parameters.handle} on contract ${parameters.contractAddress}`;
    } else if (parameters.type === 'contract-unauthorized') {
      return `Delegate ${parameters.signer.address} is not delegated by ${parameters.delegatorAddress} to user decrypt handle ${parameters.handle} on contract ${parameters.contractAddress}`;
    } else if (parameters.type === 'permit-expired') {
      return 'request has expired';
    } else {
      return 'unknown error type';
    }
  }

  static async create(parameters: {
    readonly verifyingContractAddressDecryption: string;
    readonly verifyingContractAddressInputVerification: string;
    readonly kmsContractAddress: string;
    readonly inputVerifierContractAddress: string;
    readonly aclContractAddress: string;
    readonly relayerUrl: string;
    readonly rpcUrl: string;
    readonly gatewayChainId: number;
    readonly chainId: number;
    readonly auth?: RelayerSdkAuth;
  }): Promise<SdkInstance> {
    const {
      verifyingContractAddressDecryption,
      verifyingContractAddressInputVerification,
      kmsContractAddress,
      inputVerifierContractAddress,
      aclContractAddress,
      relayerUrl,
      rpcUrl,
      gatewayChainId,
      chainId,
      auth,
    } = parameters;
    let sanitizedRelayerUrl = relayerUrl;
    if (relayerUrl.endsWith('/v1') || relayerUrl.endsWith('/v2')) {
      sanitizedRelayerUrl = relayerUrl.slice(0, -3);
    }
    if (!hasFhevmRuntimeConfig()) {
      setFhevmRuntimeConfig({
        singleThread: false,
        logger: {
          debug: (message: string) => console.log(message),
          error: (message: string, _cause: unknown) => console.error(message),
        },
      });
    }
    const fullClient = createFhevmClient({
      provider: new JsonRpcProvider(rpcUrl) as unknown as FhevmClientProvider,
      chain: defineFhevmChain({
        id: chainId,
        fhevm: {
          contracts: {
            acl: { address: aclContractAddress as `0x${string}` },
            inputVerifier: { address: inputVerifierContractAddress as `0x${string}` },
            kmsVerifier: { address: kmsContractAddress as `0x${string}` },
          },
          relayerUrl: sanitizedRelayerUrl,
          gateway: {
            id: gatewayChainId,
            contracts: {
              decryption: {
                address: verifyingContractAddressDecryption as `0x${string}`,
              },
              inputVerification: {
                address: verifyingContractAddressInputVerification as `0x${string}`,
              },
            },
          },
        },
      }),
    });
    await fullClient.ready;
    return new FhevmSdk(fullClient, toSdkAuth(auth));
  }

  get supportsWildcard(): boolean {
    return true;
  }

  async userDecryptSingleHandle(parameters: {
    readonly handle: string;
    readonly contractAddress: string;
    readonly signer: Signer & { readonly address: string };
    readonly startTimestamp?: number | undefined;
    readonly transportKeypair?: { readonly privateKey: string; readonly publicKey: string } | undefined;
  }): Promise<ClearValueType> {
    const { handle, contractAddress, signer } = parameters;

    let transportKeyPair;
    if (parameters.transportKeypair) {
      transportKeyPair = await this.#fullClient.parseTransportKeyPair(parameters.transportKeypair);
    } else {
      transportKeyPair = await this.#fullClient.generateTransportKeyPair();
    }

    const signedPermit = await this.#fullClient.signDecryptionPermit({
      contractAddresses: [contractAddress],
      durationDays: 10,
      startTimestamp: parameters.startTimestamp ?? Math.floor(Date.now() / 1000),
      transportKeyPair,
      signer,
      signerAddress: signer.address,
    });

    const res = await this.#fullClient.decryptValue({
      contractAddress,
      transportKeyPair,
      signedPermit,
      encryptedValue: handle,
      options: this.#auth ? { auth: this.#auth } : undefined,
    });

    if (typeof res.value === 'number') {
      return BigInt(res.value);
    }
    return res.value;
  }

  async delegatedUserDecryptSingleHandle(parameters: {
    readonly handle: string;
    readonly contractAddress: string;
    readonly delegatorAddress: string;
    readonly signer: Signer & { readonly address: string };
    readonly startTimestamp?: number | undefined;
    readonly delegateTransportKeypair?: { readonly privateKey: string; readonly publicKey: string } | undefined;
  }): Promise<ClearValueType> {
    const { handle, contractAddress, signer, delegatorAddress } = parameters;

    let transportKeyPair;
    if (parameters.delegateTransportKeypair) {
      transportKeyPair = await this.#fullClient.parseTransportKeyPair(parameters.delegateTransportKeypair);
    } else {
      transportKeyPair = await this.#fullClient.generateTransportKeyPair();
    }

    const signedPermit = await this.#fullClient.signDecryptionPermit({
      contractAddresses: [contractAddress],
      durationDays: 10,
      startTimestamp: parameters.startTimestamp ?? Math.floor(Date.now() / 1000),
      transportKeyPair,
      signer,
      signerAddress: signer.address,
      delegatorAddress,
    });

    const res = await this.#fullClient.decryptValue({
      contractAddress,
      transportKeyPair,
      signedPermit,
      encryptedValue: handle,
      options: this.#auth ? { auth: this.#auth } : undefined,
    });

    if (typeof res.value === 'number') {
      return BigInt(res.value);
    }
    return res.value;
  }

  async publicDecrypt(
    handles: readonly string[],
  ): Promise<{ clearValues: ClearValues; abiEncodedClearValues: `0x${string}`; decryptionProof: `0x${string}` }> {
    const res = await this.#fullClient.readPublicValuesWithSignatures({
      encryptedValues: handles,
      options: this.#auth ? { auth: this.#auth } : undefined,
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

  async encryptTypedValues(parameters: {
    readonly values: readonly TypedValue[];
    readonly contractAddress: string;
    readonly userAddress: string;
  }): Promise<EncryptedInputResult> {
    const res = await this.#fullClient.encryptValues({
      values: parameters.values,
      contractAddress: parameters.contractAddress,
      userAddress: parameters.userAddress,
      options: this.#auth ? { auth: this.#auth } : undefined,
    });
    return {
      handles: res.encryptedValues.map((ev) => getBytes(ev)),
      inputProof: getBytes(res.inputProof),
    };
  }

  async encryptUint64(parameters: {
    readonly value: number | bigint;
    readonly contractAddress: string;
    readonly userAddress: string;
  }): Promise<EncryptedInputResult> {
    const res = await this.#fullClient.encryptValue({
      contractAddress: parameters.contractAddress,
      userAddress: parameters.userAddress,
      value: { type: 'uint64', value: parameters.value },
      options: this.#auth ? { auth: this.#auth } : undefined,
    });
    return {
      handles: [getBytes(res.encryptedValue)],
      inputProof: getBytes(res.inputProof),
    };
  }

  async generateKeypair(): Promise<{ publicKey: string; privateKey: string }> {
    const p = await this.#fullClient.generateTransportKeyPair();
    return this.#fullClient.serializeTransportKeyPair({ transportKeyPair: p });
  }

  async userDecrypt(parameters: {
    readonly signer: Signer & { readonly address: string };
    readonly contractAddress: string;
    readonly startTimestamp: number;
    readonly durationDays: number;
    readonly handleContractPairs: Array<{
      handle: string | Uint8Array<ArrayBufferLike>;
      contractAddress: string;
    }>;
    readonly transportKeypair?: { readonly publicKey: string; readonly privateKey: string } | undefined;
  }): Promise<Readonly<Record<`0x${string}`, ClearValueType>>> {
    throw new Error('Not Implemented');
  }
}

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
