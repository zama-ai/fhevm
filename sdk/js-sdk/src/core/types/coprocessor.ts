import type {
  Bytes32,
  Bytes32Hex,
  BytesHex,
  ChecksummedAddress,
  Uint64BigInt,
  Uint8Number,
} from "./primitives.js";
import type { Prettify } from "./utils.js";

////////////////////////////////////////////////////////////////////////////////

export declare const coprocessorBrand: unique symbol;

export type Coprocessor = { readonly [coprocessorBrand]: never };

////////////////////////////////////////////////////////////////////////////////
//
// InputVerifier
//
////////////////////////////////////////////////////////////////////////////////

export type CoprocessorEIP712Domain = Readonly<{
  name: "InputVerification";
  version: "1";
  chainId: Uint64BigInt;
  verifyingContract: ChecksummedAddress;
}>;

export type CoprocessorEIP712Message = Readonly<{
  ctHandles: readonly Bytes32Hex[] | readonly Bytes32[];
  userAddress: ChecksummedAddress;
  contractAddress: ChecksummedAddress;
  contractChainId: Uint64BigInt;
  extraData: BytesHex;
}>;

export type CoprocessorEIP712MessageHex = Readonly<{
  ctHandles: readonly Bytes32Hex[];
  userAddress: ChecksummedAddress;
  contractAddress: ChecksummedAddress;
  contractChainId: Uint64BigInt;
  extraData: BytesHex;
}>;

export type CoprocessorEIP712Types = {
  readonly CiphertextVerification: readonly [
    { readonly name: "ctHandles"; readonly type: "bytes32[]" },
    { readonly name: "userAddress"; readonly type: "address" },
    { readonly name: "contractAddress"; readonly type: "address" },
    { readonly name: "contractChainId"; readonly type: "uint256" },
    { readonly name: "extraData"; readonly type: "bytes" },
  ];
};

export type CoprocessorEIP712 = Prettify<{
  readonly domain: CoprocessorEIP712Domain;
  readonly types: CoprocessorEIP712Types;
  readonly message: CoprocessorEIP712Message;
}>;

////////////////////////////////////////////////////////////////////////////////
//
// InputVerifier
//
////////////////////////////////////////////////////////////////////////////////

export type InputVerifierContractData = {
  readonly address: ChecksummedAddress;
  readonly eip712Domain: CoprocessorEIP712Domain;
  readonly gatewayChainId: Uint64BigInt;
  readonly coprocessorSigners: ChecksummedAddress[];
  readonly coprocessorSignerThreshold: Uint8Number;
  readonly verifyingContractAddressInputVerification: ChecksummedAddress;

  has(signer: string): boolean;
};

////////////////////////////////////////////////////////////////////////////////
//
// FHEVMExecutor
//
////////////////////////////////////////////////////////////////////////////////

export type FhevmExecutorContractData = {
  readonly address: ChecksummedAddress;
  readonly aclContractAddress: ChecksummedAddress;
  readonly inputVerifierContractAddress: ChecksummedAddress;
  readonly hcuLimitContractAddress: ChecksummedAddress;
  readonly handleVersion: Uint8Number;
};

export type FhevmExecutorContractDataJson = {
  address: string;
  aclContractAddress: string;
  inputVerifierContractAddress: string;
  hcuLimitContractAddress: string;
  handleVersion: number;
};
