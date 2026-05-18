import type { BytesHex, ChecksummedAddress } from '../../../types/primitives.js';
import type { CleartextEthereumModule } from '../../ethereum/types-ct.js';
import type { RelayerClientWithRuntime } from '../types.js';

////////////////////////////////////////////////////////////////////////////////

const FHEVM_TEST_MNEMONIC = 'test test test test test test test future home engine virtual motion';
const COPROCESSOR_PATH = "0'/2/";
const KMS_PATH = "0'/3/";

type PrivateKeyMap = Map<ChecksummedAddress, BytesHex>;

const signersPrivateKeyMap: { readonly coprocessor: PrivateKeyMap; readonly kms: PrivateKeyMap } = {
  coprocessor: new Map<ChecksummedAddress, BytesHex>(),
  kms: new Map<ChecksummedAddress, BytesHex>(),
};

////////////////////////////////////////////////////////////////////////////////

export function getCoprocessorSignersPrivateKeyMap(relayerClient: RelayerClientWithRuntime): PrivateKeyMap {
  if (signersPrivateKeyMap.coprocessor.size === 0) {
    _fillSignersPrivateKey(relayerClient, COPROCESSOR_PATH, 20, signersPrivateKeyMap.coprocessor);
  }
  return signersPrivateKeyMap.coprocessor;
}

////////////////////////////////////////////////////////////////////////////////

export function getKmsSignersPrivateKeyMap(relayerClient: RelayerClientWithRuntime): PrivateKeyMap {
  if (signersPrivateKeyMap.kms.size === 0) {
    _fillSignersPrivateKey(relayerClient, KMS_PATH, 20, signersPrivateKeyMap.kms);
  }
  return signersPrivateKeyMap.kms;
}

////////////////////////////////////////////////////////////////////////////////

function _fillSignersPrivateKey(
  relayerClient: RelayerClientWithRuntime,
  hdPathSuffix: string,
  count: number,
  map: PrivateKeyMap,
): void {
  const cleartextEthereumModule = relayerClient.runtime.ethereum as CleartextEthereumModule;
  for (let i = 0; i < count; ++i) {
    const account = cleartextEthereumModule.mnemonicToAccount({
      mnemonic: FHEVM_TEST_MNEMONIC,
      path: `m/44'/60'/${hdPathSuffix}${i}`,
    });
    map.set(account.address, account.privateKey);
  }
}
