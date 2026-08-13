import type { TrustedValue } from '../../base/trustedValue.js';
import type { Bytes32Hex, Bytes65Hex, BytesHex, ChecksummedAddress, Uint256 } from '../../types/primitives.js';
import type { Prettify } from '../../types/utils.js';

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
  recoverTypedDataAddress(parameters: RecoverTypedDataAddressParameters): Promise<RecoverTypedDataAddressReturnType>;
};

////////////////////////////////////////////////////////////////////////////////
// hashTypedData
////////////////////////////////////////////////////////////////////////////////

export type HashTypedDataParameters = Readonly<{
  domain: RecoverTypedDataAddressParameters['domain'];
  types: Record<string, Array<{ name: string; type: string }>>;
  primaryType: string;
  message: Record<string, unknown>;
}>;

/** The EIP-712 digest (`bytes32`) that a signer signs. */
export type HashTypedDataReturnType = Bytes32Hex;

export type HashTypedDataModuleFunction = {
  hashTypedData(parameters: HashTypedDataParameters): HashTypedDataReturnType;
};

////////////////////////////////////////////////////////////////////////////////
// call (raw eth_call / STATICCALL)
////////////////////////////////////////////////////////////////////////////////

export type EthCallParameters = Readonly<{
  readonly to: ChecksummedAddress;
  readonly data: BytesHex;
  /** Optional gas cap for the call. */
  readonly gas?: bigint | undefined;
}>;

/**
 * Outcome of a raw `eth_call`.
 *
 * - `{ success: true }` — the node returned (possibly empty) returndata. A no-code
 *   address yields empty `0x` data, mirroring EVM `STATICCALL` semantics.
 * - `{ success: false }` — the call reverted cleanly at the EVM level. This is a
 *   *definitive* on-chain rejection (distinct from a transport-level failure,
 *   which the adapter rethrows so the caller can degrade gracefully).
 */
export type EthCallResult =
  | { readonly success: true; readonly data: BytesHex }
  | { readonly success: false; readonly reason: string };

export type CallModuleFunction = {
  /**
   * Performs a raw `eth_call` (STATICCALL semantics) and returns the raw
   * returndata. Reverts are surfaced as `{ success: false }`; transport-level
   * errors (network, timeout, rate limit, …) are thrown so callers can decide
   * whether to fail or degrade.
   */
  call(hostPublicClient: TrustedClient, parameters: EthCallParameters): Promise<EthCallResult>;
};

////////////////////////////////////////////////////////////////////////////////
// signTypedData
////////////////////////////////////////////////////////////////////////////////

export type SignTypedDataParameters = {
  readonly account: ChecksummedAddress;
  readonly domain: {
    readonly chainId: Uint256;
    readonly name: string;
    readonly verifyingContract: ChecksummedAddress;
    readonly version: string;
  };
  readonly types: Readonly<Record<string, ReadonlyArray<{ name: string; type: string }>>>;
  readonly primaryType: string;
  readonly message: Readonly<Record<string, unknown>>;
};

export type SignTypedDataReturnType = Bytes65Hex;
export type NativeSigner = unknown;

export type SignTypedDataModuleFunction = {
  signTypedData(signer: NativeSigner, parameters: SignTypedDataParameters): Promise<SignTypedDataReturnType>;
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
  readContract(hostPublicClient: TrustedClient, parameters: ReadContractParameters): Promise<ReadContractReturnType>;
};

////////////////////////////////////////////////////////////////////////////////
// EthereumModule
////////////////////////////////////////////////////////////////////////////////

export type EthereumModule = Prettify<
  RecoverTypedDataAddressModuleFunction &
    HashTypedDataModuleFunction &
    SignTypedDataModuleFunction &
    ReadContractModuleFunction &
    CallModuleFunction &
    EncodeModuleFunction &
    EncodePackedModuleFunction &
    DecodeModuleFunction &
    GetChainIdModuleFunction
>;

// No runtime as argument. The EthereumModule is passed as argument of the runtime constructor
export type EthereumModuleFactory = () => {
  readonly ethereum: EthereumModule;
};
