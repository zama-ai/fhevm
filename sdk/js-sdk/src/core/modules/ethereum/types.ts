import type { TrustedValue } from "../../base/trustedValue.js";
import type {
  Bytes65Hex,
  BytesHex,
  ChecksummedAddress,
  Uint256,
} from "../../types/primitives.js";
import type { Prettify } from "../../types/utils.js";

////////////////////////////////////////////////////////////////////////////////
//
// Ethereum Module
//
////////////////////////////////////////////////////////////////////////////////

export declare const trustedClientBrand: unique symbol;

export type TrustedClient<T = unknown> = TrustedValue<T> & {
  [trustedClientBrand]: never;
};

////////////////////////////////////////////////////////////////////////////////
// recoverTypedDataAddress
////////////////////////////////////////////////////////////////////////////////

export type RecoverTypedDataAddressParameters = Readonly<{
  domain: {
    chainId: Uint256;
    name: string;
    verifyingContract: ChecksummedAddress;
    version: string;
  };
  types: Record<string, Array<{ name: string; type: string }>>;
  primaryType: string;
  message: Record<string, unknown>;
  signature: Bytes65Hex;
}>;

export type RecoverTypedDataAddressReturnType = ChecksummedAddress;

export type RecoverTypedDataAddressModuleFunction = {
  recoverTypedDataAddress(
    parameters: RecoverTypedDataAddressParameters,
  ): Promise<RecoverTypedDataAddressReturnType>;
};

////////////////////////////////////////////////////////////////////////////////
// encodePacked
////////////////////////////////////////////////////////////////////////////////

export type EncodePackedParameters = Readonly<{
  types: readonly string[];
  values: readonly unknown[];
}>;

export type EncodePackedReturnType = BytesHex;

export type EncodePackedModuleFunction = {
  encodePacked(parameters: EncodePackedParameters): EncodePackedReturnType;
};

////////////////////////////////////////////////////////////////////////////////
// encode
////////////////////////////////////////////////////////////////////////////////

export type EncodeParameters = Readonly<{
  types: readonly string[];
  values: readonly unknown[];
}>;

export type EncodeReturnType = BytesHex;

export type EncodeModuleFunction = {
  encode(parameters: EncodeParameters): EncodeReturnType;
};

////////////////////////////////////////////////////////////////////////////////
// decode
////////////////////////////////////////////////////////////////////////////////

export type DecodeParameters = Readonly<{
  types: readonly string[];
  encodedData: BytesHex;
}>;

export type DecodeReturnType = unknown[];

export type DecodeModuleFunction = {
  decode(parameters: DecodeParameters): DecodeReturnType;
};

////////////////////////////////////////////////////////////////////////////////
// getChainId
////////////////////////////////////////////////////////////////////////////////

export type GetChainIdReturnType = bigint;

export type GetChainIdModuleFunction = {
  getChainId(hostPublicClient: TrustedClient): Promise<GetChainIdReturnType>;
};

////////////////////////////////////////////////////////////////////////////////
// readContract
////////////////////////////////////////////////////////////////////////////////

export type ReadContractParameters = {
  readonly address: ChecksummedAddress;
  readonly functionName: string;
  readonly abi: ReadonlyArray<Record<string, unknown>>;
  readonly args: readonly unknown[];
};

export type ReadContractReturnType = unknown;

export type ReadContractModuleFunction = {
  readContract(
    hostPublicClient: TrustedClient,
    parameters: ReadContractParameters,
  ): Promise<ReadContractReturnType>;
};

////////////////////////////////////////////////////////////////////////////////
// EthereumModule
////////////////////////////////////////////////////////////////////////////////

export type EthereumModule = Prettify<
  RecoverTypedDataAddressModuleFunction &
    ReadContractModuleFunction &
    EncodeModuleFunction &
    EncodePackedModuleFunction &
    DecodeModuleFunction &
    GetChainIdModuleFunction
>;

// No runtime as argument. The EthereumModule is passed as argument of the runtime constructor
export type EthereumModuleFactory = () => {
  readonly ethereum: EthereumModule;
};
