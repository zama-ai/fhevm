import type { BytesHex, ChecksummedAddress } from '../../../types/primitives.js';
import type { KmsSigncryptedShare } from '../../../types/kms-p.js';
import type { FetchUserDecryptResultItem } from '../../../types/relayer.js';
import type { CleartextEthereumModule } from '../../ethereum/types-ct.js';
import type { RelayerClientWithRuntime } from '../types.js';
import { remove0x } from '../../../base/string.js';
import { randomUniqueUints } from '../../../base/uint.js';
import { userDecryptResultToKmsSigncryptedShares } from '../utils.js';

////////////////////////////////////////////////////////////////////////////////

const FHEVM_TEST_MNEMONIC = 'test test test test test test test future home engine virtual motion';
const COPROCESSOR_PATH = "0'/2/";
const KMS_PATH = "0'/3/";

////////////////////////////////////////////////////////////////////////////////
//
// forge-fhevm v1
// --------------
//
// forge-fhevm v1 registers a single KMS signer and a single coprocessor signer from
// these deploy-default private keys (`KMS_SIGNER_PRIVATE_KEY_0` /
// `COPROCESSOR_SIGNER_PRIVATE_KEY_0` in `deploy-local.sh`). They are NOT derivable
// from `FHEVM_TEST_MNEMONIC`, so a cleartext client running against a forge-fhevm v1
// deployment must know them explicitly: the on-chain `getKmsSigners()` returns
// these addresses and the cleartext relayer signs shares with `map.get(address)`.
//
////////////////////////////////////////////////////////////////////////////////

const FORGE_FHEVM_V1_COPROCESSOR_SIGNER: { readonly address: ChecksummedAddress; readonly privateKey: BytesHex } = {
  address: '0xc9990FEfE0c27D31D0C2aa36196b085c0c4d456c' as ChecksummedAddress,
  privateKey: '0x7ec8ada6642fc4ccfb7729bc29c17cf8d21b61abd5642d1db992c0b8672ab901' as BytesHex,
};

const FORGE_FHEVM_V1_KMS_SIGNER: { readonly address: ChecksummedAddress; readonly privateKey: BytesHex } = {
  address: '0x0971C80fF03B428fD2094dd5354600ab103201C5' as ChecksummedAddress,
  privateKey: '0x388b7680e4e1afa06efbfd45cdd1fe39f3c6af381df6555a19661f283b97de91' as BytesHex,
};

export function isForgeFhevmV1KmsSigner(signerAddress: string | undefined): boolean {
  return signerAddress?.toLowerCase() === FORGE_FHEVM_V1_KMS_SIGNER.address.toLowerCase();
}

////////////////////////////////////////////////////////////////////////////////

type PrivateKeyMap = Map<ChecksummedAddress, BytesHex>;

const signersPrivateKeyMap: { readonly coprocessor: PrivateKeyMap; readonly kms: PrivateKeyMap } = {
  coprocessor: new Map<ChecksummedAddress, BytesHex>(),
  kms: new Map<ChecksummedAddress, BytesHex>(),
};

////////////////////////////////////////////////////////////////////////////////

export function getCoprocessorSignersPrivateKeyMap(relayerClient: RelayerClientWithRuntime): PrivateKeyMap {
  if (signersPrivateKeyMap.coprocessor.size === 0) {
    _fillSignersPrivateKey(relayerClient, COPROCESSOR_PATH, 20, signersPrivateKeyMap.coprocessor);
    signersPrivateKeyMap.coprocessor.set(
      FORGE_FHEVM_V1_COPROCESSOR_SIGNER.address,
      FORGE_FHEVM_V1_COPROCESSOR_SIGNER.privateKey,
    );
  }
  return signersPrivateKeyMap.coprocessor;
}

////////////////////////////////////////////////////////////////////////////////

export function getKmsSignersPrivateKeyMap(relayerClient: RelayerClientWithRuntime): PrivateKeyMap {
  if (signersPrivateKeyMap.kms.size === 0) {
    _fillSignersPrivateKey(relayerClient, KMS_PATH, 20, signersPrivateKeyMap.kms);
    signersPrivateKeyMap.kms.set(FORGE_FHEVM_V1_KMS_SIGNER.address, FORGE_FHEVM_V1_KMS_SIGNER.privateKey);
  }
  return signersPrivateKeyMap.kms;
}

////////////////////////////////////////////////////////////////////////////////

/**
 * The KMS nodes that answer a request, as a relayer forwards one response per node: a random subset of
 * `signersAddress`, of random size between `threshold` and all of them.
 */
export function pickRespondingKmsSigners(
  signersAddress: readonly ChecksummedAddress[],
  threshold: number,
): ChecksummedAddress[] {
  const n = signersAddress.length;
  if (!Number.isInteger(threshold) || threshold < 1 || threshold > n) {
    throw new Error(`Invalid KMS threshold ${threshold} for ${n} signers`);
  }
  let count = threshold + Math.floor(Math.random() * (n - threshold + 1));
  // Defensive: the guard above already keeps count within [threshold, n].
  if (count < 1) {
    count = 1;
  }
  if (count > n) {
    count = n;
  }
  // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
  return randomUniqueUints(n, count).map((i) => signersAddress[i]!);
}

/** One signcrypted share per responding KMS node, each signing the common payload. */
export async function signKmsSigncryptedShares(
  relayerClient: RelayerClientWithRuntime,
  parameters: {
    readonly commonKmsSigncryptedSharePayload: BytesHex;
    readonly extraData: BytesHex;
    readonly signersAddress: readonly ChecksummedAddress[];
    readonly threshold: number;
  },
): Promise<readonly KmsSigncryptedShare[]> {
  const cleartextEthereumModule = relayerClient.runtime.ethereum as CleartextEthereumModule;
  const { commonKmsSigncryptedSharePayload: payload, extraData, signersAddress, threshold } = parameters;
  const privateKeys = getKmsSignersPrivateKeyMap(relayerClient);
  const responders = pickRespondingKmsSigners(signersAddress, threshold);

  const result: FetchUserDecryptResultItem[] = [];
  for (const signerAddress of responders) {
    const privateKey = privateKeys.get(signerAddress);
    if (privateKey === undefined) {
      throw new Error(
        `Unable to find KMS signer for address ${signerAddress} (${responders.length} of ${signersAddress.length} responding)`,
      );
    }
    const signature = await cleartextEthereumModule.sign({ hash: payload, privateKey });
    result.push({ signature: remove0x(signature), payload: remove0x(payload), extraData });
  }

  return userDecryptResultToKmsSigncryptedShares(result);
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
