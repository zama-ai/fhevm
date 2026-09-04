import type { Bytes32Hex, Bytes65Hex, BytesHex, ChecksummedAddress, Uint256 } from '../../types/primitives.js';
import type { Prettify } from '../../types/utils.js';
import type { EthereumModule } from './types.js';

////////////////////////////////////////////////////////////////////////////////
// mnemonicToAccount
////////////////////////////////////////////////////////////////////////////////

export type MnemonicToAccountParameters = Readonly<{
  mnemonic: string;
  path: `m/44'/60'/${string}`;
}>;

export type MnemonicToAccountReturnType = { readonly privateKey: BytesHex; readonly address: ChecksummedAddress };

export type MnemonicToAccountModuleFunction = {
  mnemonicToAccount(parameters: MnemonicToAccountParameters): MnemonicToAccountReturnType;
};

////////////////////////////////////////////////////////////////////////////////
// sign
////////////////////////////////////////////////////////////////////////////////

export type SignParameters = Readonly<{
  hash: BytesHex;
  privateKey: BytesHex;
}>;

export type SignReturnType = Bytes65Hex;

export type SignModuleFunction = {
  sign(parameters: SignParameters): Promise<SignReturnType>;
};

////////////////////////////////////////////////////////////////////////////////
// hashTypedData
////////////////////////////////////////////////////////////////////////////////

export type HashTypedDataParameters = Readonly<{
  domain: {
    readonly chainId: Uint256;
    readonly name: string;
    readonly verifyingContract: ChecksummedAddress;
    readonly version: string;
  };
  types: Readonly<Record<string, ReadonlyArray<{ name: string; type: string }>>>;
  primaryType: string;
  message: Readonly<Record<string, unknown>>;
}>;

export type HashTypedDataReturnType = Bytes32Hex;

export type HashTypedDataModuleFunction = {
  hashTypedData(parameters: HashTypedDataParameters): HashTypedDataReturnType;
};

////////////////////////////////////////////////////////////////////////////////
// generatePrivateKey
////////////////////////////////////////////////////////////////////////////////

export type GeneratePrivateKeyReturnType = BytesHex;

export type GeneratePrivateKeyModuleFunction = {
  generatePrivateKey(): GeneratePrivateKeyReturnType;
};

////////////////////////////////////////////////////////////////////////////////
// getPublicKey
////////////////////////////////////////////////////////////////////////////////

export type GetPublicKeyParameters = Readonly<{
  privateKey: BytesHex;
}>;

export type GetPublicKeyReturnType = BytesHex;

export type GetPublicKeyModuleFunction = {
  getPublicKey(parameters: GetPublicKeyParameters): GetPublicKeyReturnType;
};

////////////////////////////////////////////////////////////////////////////////
// recoverAddress
////////////////////////////////////////////////////////////////////////////////

export type RecoverAddressParameters = Readonly<{
  hash: BytesHex;
  signature: Bytes65Hex;
}>;

export type RecoverAddressReturnType = ChecksummedAddress;

export type RecoverAddressModuleFunction = {
  recoverAddress(parameters: RecoverAddressParameters): Promise<RecoverAddressReturnType>;
};

////////////////////////////////////////////////////////////////////////////////
// cleartextEthereumModule
////////////////////////////////////////////////////////////////////////////////

export type CleartextEthereumModule = EthereumModule &
  Prettify<
    MnemonicToAccountModuleFunction &
      SignModuleFunction &
      GeneratePrivateKeyModuleFunction &
      GetPublicKeyModuleFunction &
      RecoverAddressModuleFunction &
      HashTypedDataModuleFunction
  >;

export type CleartextEthereumModuleFactory = () => {
  readonly ethereum: CleartextEthereumModule;
};
