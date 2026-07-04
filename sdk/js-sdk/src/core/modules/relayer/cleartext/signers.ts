import type { BytesHex, ChecksummedAddress } from '../../../types/primitives.js';
import type { CleartextEthereumModule } from '../../ethereum/types-ct.js';
import type { RelayerClientWithRuntime } from '../types.js';

////////////////////////////////////////////////////////////////////////////////

const FHEVM_TEST_MNEMONIC = 'test test test test test test test future home engine virtual motion';
const COPROCESSOR_PATH = "0'/2/";
const KMS_PATH = "0'/3/";

// forge-fhevm registers a single KMS signer and a single coprocessor signer from
// these deploy-default private keys (`KMS_SIGNER_PRIVATE_KEY_0` /
// `COPROCESSOR_SIGNER_PRIVATE_KEY_0` in `deploy-local.sh`). They are NOT derivable
// from `FHEVM_TEST_MNEMONIC`, so a cleartext client running against a forge-fhevm
// deployment must know them explicitly: the on-chain `getKmsSigners()` returns
// these addresses and the cleartext relayer signs shares with `map.get(address)`.
const FORGE_KMS_SIGNER: { readonly address: ChecksummedAddress; readonly privateKey: BytesHex } = {
  address: '0x0971C80fF03B428fD2094dd5354600ab103201C5' as ChecksummedAddress,
  privateKey: '0x388b7680e4e1afa06efbfd45cdd1fe39f3c6af381df6555a19661f283b97de91' as BytesHex,
};
const FORGE_COPROCESSOR_SIGNER: { readonly address: ChecksummedAddress; readonly privateKey: BytesHex } = {
  address: '0xc9990FEfE0c27D31D0C2aa36196b085c0c4d456c' as ChecksummedAddress,
  privateKey: '0x7ec8ada6642fc4ccfb7729bc29c17cf8d21b61abd5642d1db992c0b8672ab901' as BytesHex,
};

type PrivateKeyMap = Map<ChecksummedAddress, BytesHex>;

const signersPrivateKeyMap: { readonly coprocessor: PrivateKeyMap; readonly kms: PrivateKeyMap } = {
  coprocessor: new Map<ChecksummedAddress, BytesHex>(),
  kms: new Map<ChecksummedAddress, BytesHex>(),
};

////////////////////////////////////////////////////////////////////////////////

export function getCoprocessorSignersPrivateKeyMap(relayerClient: RelayerClientWithRuntime): PrivateKeyMap {
  if (signersPrivateKeyMap.coprocessor.size === 0) {
    _fillSignersPrivateKey(relayerClient, COPROCESSOR_PATH, 20, signersPrivateKeyMap.coprocessor);
    signersPrivateKeyMap.coprocessor.set(FORGE_COPROCESSOR_SIGNER.address, FORGE_COPROCESSOR_SIGNER.privateKey);
  }
  return signersPrivateKeyMap.coprocessor;
}

////////////////////////////////////////////////////////////////////////////////

export function getKmsSignersPrivateKeyMap(relayerClient: RelayerClientWithRuntime): PrivateKeyMap {
  if (signersPrivateKeyMap.kms.size === 0) {
    _fillSignersPrivateKey(relayerClient, KMS_PATH, 20, signersPrivateKeyMap.kms);
    signersPrivateKeyMap.kms.set(FORGE_KMS_SIGNER.address, FORGE_KMS_SIGNER.privateKey);
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
