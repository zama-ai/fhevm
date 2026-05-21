import type {
  CleartextEthereumModuleFactory,
  GeneratePrivateKeyReturnType,
  GetPublicKeyParameters,
  GetPublicKeyReturnType,
  MnemonicToAccountParameters,
  MnemonicToAccountReturnType,
  RecoverAddressParameters,
  RecoverAddressReturnType,
  SignParameters,
  SignReturnType,
} from '../../core/modules/ethereum/types-ct.js';
import type { Bytes65Hex, BytesHex, ChecksummedAddress } from '../../core/types/primitives.js';
import { generatePrivateKey, mnemonicToAccount, privateKeyToAccount, sign } from 'viem/accounts';
import {
  decode,
  encode,
  encodePacked,
  getChainId,
  readContract,
  recoverTypedDataAddress,
  signTypedData,
} from './ethereum.js';
import { bytesToHex } from '../../core/base/bytes.js';
import { recoverAddress } from 'viem';

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
        const signer = mnemonicToAccount(parameters.mnemonic, { path: parameters.path });
        const pk = signer.getHdKey().privateKey;
        if (pk === null) {
          throw new Error(`Failed to derive private key from mnemonic at address ${signer.address}.`);
        }
        return { privateKey: bytesToHex(pk), address: signer.address as ChecksummedAddress };
      },
      sign: (parameters: SignParameters): Promise<SignReturnType> => {
        return sign({ hash: parameters.hash, privateKey: parameters.privateKey, to: 'hex' }) as Promise<Bytes65Hex>;
      },
      generatePrivateKey: (): GeneratePrivateKeyReturnType => {
        return generatePrivateKey() as BytesHex;
      },
      getPublicKey: (parameters: GetPublicKeyParameters): GetPublicKeyReturnType => {
        return privateKeyToAccount(parameters.privateKey).publicKey as BytesHex;
      },
      recoverAddress: (parameters: RecoverAddressParameters): Promise<RecoverAddressReturnType> => {
        return recoverAddress(parameters) as Promise<ChecksummedAddress>;
      },
    }),
  });
};
