import type {
  CleartextEthereumModuleFactory,
  GeneratePrivateKeyReturnType,
  GetPublicKeyParameters,
  GetPublicKeyReturnType,
  HashTypedDataParameters,
  HashTypedDataReturnType,
  MnemonicToAccountParameters,
  MnemonicToAccountReturnType,
  RecoverAddressParameters,
  RecoverAddressReturnType,
  SignParameters,
  SignReturnType,
} from '../../core/modules/ethereum/types-ct.js';
import type { BytesHex, ChecksummedAddress } from '../../core/types/primitives.js';
import type { TypedDataField } from 'ethers';
import {
  decode,
  encode,
  encodePacked,
  getChainId,
  readContract,
  recoverTypedDataAddress,
  signTypedData,
} from './ethereum.js';
import { SigningKey, HDNodeWallet, Wallet, TypedDataEncoder } from 'ethers';
import { sign, recoverAddress } from '../../core/base/sign.js';
import { asBytes32Hex } from '../../core/base/bytes.js';

////////////////////////////////////////////////////////////////////////////////

export const cleartextEthereumModule: CleartextEthereumModuleFactory = () => {
  return Object.freeze({
    ethereum: Object.freeze({
      decode,
      encode,
      encodePacked,
      recoverTypedDataAddress,
      signTypedData,
      getChainId,
      readContract,
      mnemonicToAccount: (parameters: MnemonicToAccountParameters): MnemonicToAccountReturnType => {
        const signer = HDNodeWallet.fromPhrase(parameters.mnemonic, undefined, parameters.path);
        return {
          privateKey: signer.privateKey as BytesHex,
          address: signer.address as ChecksummedAddress,
        };
      },
      sign: (parameters: SignParameters): Promise<SignReturnType> => {
        const signature = sign(parameters);
        return Promise.resolve(signature);
      },
      generatePrivateKey: (): GeneratePrivateKeyReturnType => {
        return Wallet.createRandom().privateKey as BytesHex;
      },
      getPublicKey: (parameters: GetPublicKeyParameters): GetPublicKeyReturnType => {
        return SigningKey.computePublicKey(parameters.privateKey, false) as BytesHex;
      },
      recoverAddress: (parameters: RecoverAddressParameters): Promise<RecoverAddressReturnType> => {
        const recoveredAddress = recoverAddress(parameters);
        return Promise.resolve(recoveredAddress);
      },
      hashTypedData: (parameters: HashTypedDataParameters): HashTypedDataReturnType => {
        const primaryTypeFields = parameters.types[parameters.primaryType];
        if (primaryTypeFields === undefined) {
          throw new Error(`Primary type "${parameters.primaryType}" not found in types`);
        }
        const typesToHash: Record<string, TypedDataField[]> = {
          [parameters.primaryType]: [...primaryTypeFields],
        };
        return asBytes32Hex(TypedDataEncoder.hash(parameters.domain, typesToHash, parameters.message));
      },
    }),
  });
};
