import type { FhevmInstance } from "@zama-fhe/relayer-sdk/node";
import { createInstance } from "@zama-fhe/relayer-sdk/node";
import type { Signer } from "ethers";

import type { Auth, ClearValueType, ClearValues, EncryptedInputResult, SdkInstance, TypedValue } from "../types";

export class RelayerSdk implements SdkInstance {
  #instance: FhevmInstance;

  constructor(instance: FhevmInstance) {
    this.#instance = instance;
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
    const instance = await createInstance({
      verifyingContractAddressDecryption,
      verifyingContractAddressInputVerification,
      kmsContractAddress,
      inputVerifierContractAddress,
      aclContractAddress,
      network: rpcUrl,
      relayerUrl,
      gatewayChainId,
      chainId,
      ...(auth ? { auth } : {}),
    });
    return new RelayerSdk(instance);
  }

  get supportsWildcard(): boolean {
    return false;
  }

  async generateKeypair(): Promise<{ publicKey: string; privateKey: string }> {
    const pair = this.#instance.generateKeypair();
    return Promise.resolve(pair);
  }

  async encryptTypedValues(parameters: {
    readonly values: readonly TypedValue[];
    readonly contractAddress: string;
    readonly userAddress: string;
  }): Promise<EncryptedInputResult> {
    const input = this.#instance.createEncryptedInput(parameters.contractAddress, parameters.userAddress);

    for (const typedValue of parameters.values) {
      switch (typedValue.type) {
        case "bool":
          input.addBool(typedValue.value);
          break;
        case "uint8":
          input.add8(typedValue.value);
          break;
        case "uint16":
          input.add16(typedValue.value);
          break;
        case "uint32":
          input.add32(typedValue.value);
          break;
        case "uint64":
          input.add64(typedValue.value);
          break;
        case "uint128":
          input.add128(typedValue.value);
          break;
        case "uint256":
          input.add256(typedValue.value);
          break;
        case "address":
          input.addAddress(typedValue.value);
          break;
      }
    }

    return await input.encrypt();
  }

  async encryptUint64(parameters: {
    readonly value: number | bigint;
    readonly contractAddress: string;
    readonly userAddress: string;
  }): Promise<EncryptedInputResult> {
    return await this.encryptTypedValues({
      values: [{ type: "uint64", value: parameters.value }],
      contractAddress: parameters.contractAddress,
      userAddress: parameters.userAddress,
    });
  }

  async userDecryptSingleHandle(parameters: {
    readonly handle: string;
    readonly contractAddress: string;
    readonly signer: Signer & { readonly address: string };
    readonly startTimestamp?: number | undefined;
    readonly transportKeypair?: { readonly privateKey: string; readonly publicKey: string } | undefined;
  }): Promise<ClearValueType> {
    const { handle, contractAddress, signer } = parameters;

    const transportKeypair = parameters.transportKeypair ?? (await this.generateKeypair());

    const result = await this.userDecrypt({
      transportKeypair,
      handleContractPairs: [
        {
          handle: handle,
          contractAddress: contractAddress,
        },
      ],
      durationDays: 10,
      startTimestamp: parameters.startTimestamp ?? Math.floor(Date.now() / 1000),
      signer,
      contractAddress,
    });

    const decryptedValue = result[handle as `0x${string}`];
    return decryptedValue;
  }

  async delegatedUserDecryptSingleHandle(parameters: {
    readonly handle: string;
    readonly contractAddress: string;
    readonly delegatorAddress: string;
    readonly signer: Signer & { readonly address: string };
    readonly startTimestamp?: number | undefined;
    readonly delegateTransportKeypair?: { readonly privateKey: string; readonly publicKey: string } | undefined;
  }): Promise<ClearValueType> {
    const { handle, contractAddress, delegatorAddress, signer } = parameters;
    const delegateTransportKeypair = parameters.delegateTransportKeypair ?? (await this.generateKeypair());
    const handleContractPairs = [
      {
        handle,
        contractAddress,
      },
    ];
    const startTimeStamp = parameters.startTimestamp ?? Math.floor(Date.now() / 1000);
    const durationDays = 10;
    const contractAddresses = [contractAddress];

    // Build the extraData field
    const extraData = await this.#instance.getExtraData();

    // The `delegate` creates a EIP712 with the `delegator` address
    const eip712 = this.#instance.createDelegatedUserDecryptEIP712(
      delegateTransportKeypair.publicKey,
      contractAddresses,
      delegatorAddress,
      startTimeStamp,
      durationDays,
      extraData,
    );

    // Update the signing to match the new primaryType
    const delegateSignature = await signer.signTypedData(
      eip712.domain,
      {
        DelegatedUserDecryptRequestVerification: [...eip712.types.DelegatedUserDecryptRequestVerification],
      },
      eip712.message,
    );

    const result = await this.#instance.delegatedUserDecrypt(
      handleContractPairs,
      delegateTransportKeypair.privateKey,
      delegateTransportKeypair.publicKey,
      delegateSignature.replace("0x", ""),
      contractAddresses,
      delegatorAddress,
      signer.address,
      startTimeStamp,
      durationDays,
      extraData,
    );

    return result[handle as `0x${string}`];
  }

  async publicDecrypt(handles: readonly string[]): Promise<{
    clearValues: ClearValues;
    abiEncodedClearValues: `0x${string}`;
    decryptionProof: `0x${string}`;
  }> {
    const res = await this.#instance.publicDecrypt(handles as `0x${string}`[]);
    return res;
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
    readonly transportKeypair?:
      | {
          readonly publicKey: string;
          readonly privateKey: string;
        }
      | undefined;
  }): Promise<ClearValues> {
    const { signer, contractAddress, handleContractPairs, startTimestamp, durationDays, transportKeypair } = parameters;

    const { publicKey, privateKey } = transportKeypair ?? this.#instance.generateKeypair();
    const contractAddresses = [contractAddress];

    // Build the extraData field
    const extraData = await this.#instance.getExtraData();

    const eip712 = this.#instance.createEIP712(publicKey, contractAddresses, startTimestamp, durationDays, extraData);

    const signature = await signer.signTypedData(
      eip712.domain,
      {
        UserDecryptRequestVerification: [...eip712.types.UserDecryptRequestVerification],
      },
      eip712.message,
    );

    return await this.#instance.userDecrypt(
      handleContractPairs,
      privateKey,
      publicKey,
      signature.replace("0x", ""),
      contractAddresses,
      signer.address,
      startTimestamp,
      durationDays,
      extraData,
    );
  }

  getUserDecryptErrorMessage(parameters: {
    readonly type: "user-unauthorized" | "user-equal-contract" | "contract-unauthorized" | "permit-expired";
    readonly signer: Signer & { readonly address: string };
    readonly handle?: string | undefined;
    readonly contractAddress?: string | undefined;
  }): string {
    if (parameters.type === "user-unauthorized") {
      return `User address ${parameters.signer.address} is not authorized to user decrypt handle ${parameters.handle}!`;
    } else if (parameters.type === "user-equal-contract") {
      return `User address ${parameters.signer.address} should not be equal to contract address when requesting user decryption!`;
    } else if (parameters.type === "contract-unauthorized") {
      return `is not authorized to user decrypt handle`;
    } else if (parameters.type === "permit-expired") {
      return "request has expired";
    } else {
      return "unknown error type";
    }
  }

  getDelegatedUserDecryptErrorMessage(parameters: {
    readonly type: "revocation" | "contract-unauthorized" | "permit-expired" | "delegation-does-not-exist";
    readonly signer: Signer & { readonly address: string };
    readonly handle?: string | undefined;
    readonly contractAddress?: string | undefined;
    readonly delegatorAddress?: string | undefined;
  }): string {
    //Not allowed on host ACL: ACL check failed for 1 handle(s): handle=0x0a082d5542aa39e21e0f0b3bb51a38ef5e2d2f7c36ff00000000000030390500 check=isHandleDelegatedForUserDecryption
    return `Not allowed on host ACL: ACL check failed for 1 handle(s): handle=${parameters.handle} check=isHandleDelegatedForUserDecryption`;
  }
}
