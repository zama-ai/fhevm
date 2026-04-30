import { defineFhevmChain } from "@fhevm/sdk/chains";
import { createFhevmClient, hasFhevmRuntimeConfig, setFhevmRuntimeConfig } from "@fhevm/sdk/ethers";
import type { Signer } from "ethers";
import { JsonRpcProvider, getBytes, hexlify } from "ethers";

import type {
  ClearValueType,
  ClearValues,
  EncryptedInputResult,
  Auth as RelayerSdkAuth,
  SdkInstance,
  TypedValue,
} from "../types";

type FhevmClient = ReturnType<typeof createFhevmClient>;
type CreateFhevmClientParameters = Parameters<typeof createFhevmClient>[0];
type FhevmClientProvider = CreateFhevmClientParameters["provider"];
type Auth = any;

export class FhevmSdk implements SdkInstance {
  #fullClient: FhevmClient;
  #auth: Auth | undefined;

  constructor(fullClient: FhevmClient, auth: Auth | undefined) {
    this.#fullClient = fullClient;
    this.#auth = auth;
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
    readonly auth?: Auth;
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
    if (relayerUrl.endsWith("/v1") || relayerUrl.endsWith("/v2")) {
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
    return new FhevmSdk(fullClient, auth);
  }

  get supportsWildcard(): boolean {
    return true;
  }

  async userDecryptSingleHandle(parameters: {
    readonly handle: string;
    readonly contractAddress: string;
    readonly signer: Signer & { readonly address: string };
    readonly transportKeypair?: { readonly privateKey: string; readonly publicKey: string } | undefined;
  }): Promise<ClearValueType> {
    const { handle, contractAddress, signer } = parameters;

    let transportKeypair;
    if (parameters.transportKeypair) {
      transportKeypair = await this.#fullClient.parseTransportKeypair(parameters.transportKeypair);
    } else {
      transportKeypair = await this.#fullClient.generateTransportKeypair();
    }

    const signedPermit = await this.#fullClient.signDecryptionPermit({
      contractAddresses: [contractAddress],
      durationDays: 10,
      startTimestamp: Math.floor(Date.now() / 1000),
      transportKeypair,
      signer,
      signerAddress: signer.address,
    });

    const res = await this.#fullClient.decryptValue({
      contractAddress,
      transportKeypair,
      signedPermit,
      encryptedValue: handle,
    });

    if (typeof res.value === "number") {
      return BigInt(res.value);
    }
    return res.value;
  }

  async delegatedUserDecryptSingleHandle(parameters: {
    readonly handle: string;
    readonly contractAddress: string;
    readonly delegatorAddress: string;
    readonly signer: Signer & { readonly address: string };
    readonly delegateTransportKeypair?: { readonly privateKey: string; readonly publicKey: string } | undefined;
  }): Promise<ClearValueType> {
    const { handle, contractAddress, signer } = parameters;

    let transportKeypair;
    if (parameters.delegateTransportKeypair) {
      transportKeypair = await this.#fullClient.parseTransportKeypair(parameters.delegateTransportKeypair);
    } else {
      transportKeypair = await this.#fullClient.generateTransportKeypair();
    }

    const signedPermit = await this.#fullClient.signDecryptionPermit({
      contractAddresses: [contractAddress],
      durationDays: 10,
      startTimestamp: Math.floor(Date.now() / 1000),
      transportKeypair,
      signer,
      signerAddress: signer.address,
    });

    const res = await this.#fullClient.decryptValue({
      contractAddress,
      transportKeypair,
      signedPermit,
      encryptedValue: handle,
    });

    if (typeof res.value === "number") {
      return BigInt(res.value);
    }
    return res.value;
  }

  async publicDecrypt(
    handles: readonly string[],
  ): Promise<{ clearValues: ClearValues; abiEncodedClearValues: `0x${string}`; decryptionProof: `0x${string}` }> {
    const res = await this.#fullClient.readPublicValuesWithSignatures({
      encryptedValues: handles,
      options: this.#auth ? { auth: toSdkAuth(this.#auth) } : undefined,
    });

    const clearValues: Record<`0x${string}`, bigint | boolean | `0x${string}`> = {};
    for (let i = 0; i < handles.length; i++) {
      const h = handles[i];
      const handleHex = (typeof h === "string" ? h : hexlify(h)) as `0x${string}`;
      const v = res.clearValues[i].value;
      clearValues[handleHex] = typeof v === "number" ? BigInt(v) : v;
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
      options: this.#auth ? { auth: toSdkAuth(this.#auth) } : undefined,
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
      value: { type: "uint64", value: parameters.value },
    });
    return {
      handles: [getBytes(res.encryptedValue)],
      inputProof: getBytes(res.inputProof),
    };
  }

  async generateKeypair(): Promise<{ publicKey: string; privateKey: string }> {
    const p = await this.#fullClient.generateTransportKeypair();
    return this.#fullClient.serializeTransportKeypair({ transportKeypair: p });
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
    throw new Error("Not Implemented");
  }
}

function toSdkAuth(auth: RelayerSdkAuth | undefined): Auth | undefined {
  if (auth === undefined) {
    return undefined;
  }
  switch (auth.__type) {
    case "BearerToken":
      return { type: "BearerToken", token: auth.token };
    case "ApiKeyHeader":
      return { type: "ApiKeyHeader", header: auth.header, value: auth.value };
    case "ApiKeyCookie":
      return { type: "ApiKeyCookie", cookie: auth.cookie, value: auth.value };
  }
}
