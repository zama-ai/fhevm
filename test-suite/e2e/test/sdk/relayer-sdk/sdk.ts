import { createHash } from 'node:crypto';

import type { FhevmInstance } from '@zama-fhe/relayer-sdk/node';
import { createInstance } from '@zama-fhe/relayer-sdk/node';
import type { Signer } from 'ethers';

import type { Auth, ClearValueType, ClearValues, EncryptedInputResult, SdkInstance, TypedValue } from '../types';

type KeyData = {
  readonly dataId: string;
  readonly urls: readonly string[];
  readonly extra: Record<string, unknown>;
};

const isRecord = (value: unknown): value is Record<string, unknown> =>
  typeof value === 'object' && value !== null;

const asStringArray = (value: unknown): readonly string[] =>
  Array.isArray(value) ? value.filter((item): item is string => typeof item === 'string') : [];

const firstFhePublicKey = (value: unknown): KeyData | undefined => {
  if (!isRecord(value) || !isRecord(value.response)) {
    return undefined;
  }
  const [firstKeyInfo] = Array.isArray(value.response.fheKeyInfo) ? value.response.fheKeyInfo : [];
  if (!isRecord(firstKeyInfo) || !isRecord(firstKeyInfo.fhePublicKey)) {
    return undefined;
  }
  const { dataId, urls } = firstKeyInfo.fhePublicKey;
  if (typeof dataId !== 'string') {
    return undefined;
  }
  const parsedUrls = asStringArray(urls);
  const { urls: _urls, dataId: _dataId, ...extra } = firstKeyInfo.fhePublicKey;
  return parsedUrls.length > 0 ? { dataId, urls: parsedUrls, extra } : undefined;
};

const interestingHeaders = (headers: Headers): Record<string, string> => {
  const values: Record<string, string> = {};
  headers.forEach((value, key) => {
    const normalized = key.toLowerCase();
    if (
      normalized === 'content-type' ||
      normalized === 'content-length' ||
      normalized === 'etag' ||
      normalized === 'last-modified' ||
      normalized.startsWith('x-amz-') ||
      normalized.startsWith('x-minio-')
    ) {
      values[normalized] = value;
    }
  });
  return values;
};

const tagUrl = (url: string): string => `${url}${url.includes('?') ? '&' : '?'}tagging`;

const compactPreview = (value: string): string => value.replace(/\s+/g, ' ').trim().slice(0, 2048);

const hexPreview = (bytes: Uint8Array, offset: number, length: number): string =>
  Array.from(bytes.slice(offset, offset + length), (byte) => byte.toString(16).padStart(2, '0')).join('');

const readU32Le = (bytes: Uint8Array): number | undefined =>
  bytes.length >= 4 ? (bytes[0] | (bytes[1] << 8) | (bytes[2] << 16) | (bytes[3] << 24)) >>> 0 : undefined;

const readU32Be = (bytes: Uint8Array): number | undefined =>
  bytes.length >= 4 ? ((bytes[0] << 24) | (bytes[1] << 16) | (bytes[2] << 8) | bytes[3]) >>> 0 : undefined;

const printableHints = (bytes: Uint8Array): readonly string[] => {
  const preview = Buffer.from(bytes.slice(0, Math.min(bytes.length, 8192))).toString('latin1');
  const runs = preview.match(/[ -~]{4,}/g) ?? [];
  return runs
    .filter((run) => /tfhe|key|public|compact|compressed|version|zama|type/i.test(run))
    .slice(0, 10);
};

const formatDiagnosticError = (error: unknown): string =>
  error instanceof Error ? `${error.name}: ${error.message}` : String(error);

const diagnosedKeyUrls = new Set<string>();

const logFhePublicKeyDiagnostics = async (keyUrl: string) => {
  if (diagnosedKeyUrls.has(keyUrl)) {
    return;
  }
  diagnosedKeyUrls.add(keyUrl);

  try {
    const keyUrlResponse = await fetch(keyUrl);
    const keyUrlText = await keyUrlResponse.text();
    const keyUrlHeaders = JSON.stringify(interestingHeaders(keyUrlResponse.headers));
    console.log(`[relayer-sdk] keyUrl status=${keyUrlResponse.status} headers=${keyUrlHeaders}`);

    if (!keyUrlResponse.ok) {
      console.warn(`[relayer-sdk] keyUrl non-2xx body preview=${JSON.stringify(keyUrlText.slice(0, 512))}`);
      return;
    }

    const keyUrlJson = JSON.parse(keyUrlText) as unknown;
    const publicKey = firstFhePublicKey(keyUrlJson);
    if (!publicKey) {
      console.warn(`[relayer-sdk] keyUrl response did not contain response.fheKeyInfo[0].fhePublicKey`);
      return;
    }

    const [publicKeyUrl] = publicKey.urls;
    const publicKeyUrls = JSON.stringify(publicKey.urls);
    console.log(
      [
        `[relayer-sdk] fhePublicKey dataId=${publicKey.dataId}`,
        `extra=${JSON.stringify(publicKey.extra)}`,
        `url=${publicKeyUrl}`,
        `urls=${publicKeyUrls}`,
      ].join(' '),
    );

    const publicKeyResponse = await fetch(publicKeyUrl);
    const bytes = new Uint8Array(await publicKeyResponse.arrayBuffer());
    const metadataHeaders = interestingHeaders(publicKeyResponse.headers);
    const sha256 = createHash('sha256').update(Buffer.from(bytes)).digest('hex');
    const firstByte = bytes.length > 0 ? bytes[0] : undefined;
    const firstU32Le = readU32Le(bytes);
    const firstU32Be = readU32Be(bytes);
    const hints = printableHints(bytes);

    console.log(
      [
        `[relayer-sdk] fhePublicKey fetch status=${publicKeyResponse.status}`,
        `metadataHeaders=${JSON.stringify(metadataHeaders)}`,
        `bytes=${bytes.byteLength}`,
        `sha256=${sha256}`,
        `firstByte=${firstByte ?? 'n/a'}`,
        `firstU32Le=${firstU32Le ?? 'n/a'}`,
        `firstU32Be=${firstU32Be ?? 'n/a'}`,
      ].join(' '),
    );
    console.log(
      `[relayer-sdk] fhePublicKey bytePreview first32=${hexPreview(bytes, 0, 32)} last32=${hexPreview(
        bytes,
        Math.max(0, bytes.length - 32),
        32,
      )}`,
    );
    console.log(
      `[relayer-sdk] fhePublicKey serializedRustTypeHints=${
        hints.length > 0 ? JSON.stringify(hints) : 'none-found'
      }`,
    );

    try {
      const tagsResponse = await fetch(tagUrl(publicKeyUrl));
      const tagsText = await tagsResponse.text();
      console.log(
        [
          `[relayer-sdk] fhePublicKey objectTags status=${tagsResponse.status}`,
          `headers=${JSON.stringify(interestingHeaders(tagsResponse.headers))}`,
          `body=${JSON.stringify(compactPreview(tagsText))}`,
        ].join(' '),
      );
    } catch (error) {
      console.warn(`[relayer-sdk] fhePublicKey objectTags diagnostics failed: ${formatDiagnosticError(error)}`);
    }
  } catch (error) {
    console.warn(`[relayer-sdk] fhePublicKey diagnostics failed: ${formatDiagnosticError(error)}`);
  }
};

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
    const normalizedRelayerUrl = relayerUrl.replace(/\/+$/, "");
    console.log(`[relayer-sdk] relayerUrl=${relayerUrl}`);
    console.log(`[relayer-sdk] keyUrl=${normalizedRelayerUrl}/keyurl`);
    await logFhePublicKeyDiagnostics(`${normalizedRelayerUrl}/keyurl`);
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

    const extraData = await this.#instance.getExtraData?.();
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

    const extraData = await this.#instance.getExtraData?.();
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
