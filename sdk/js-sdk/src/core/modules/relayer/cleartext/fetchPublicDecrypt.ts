import type { Bytes32Hex, Bytes65Hex, BytesHex, ChecksummedAddress } from '../../../types/primitives.js';
import type { CleartextEthereumModule } from '../../ethereum/types-ct.js';
import type { FetchPublicDecryptParameters, FetchPublicDecryptReturnType, RelayerClientWithRuntime } from '../types.js';
import { getTrustedClient } from '../../../runtime/CoreFhevm-p.js';
import { getKmsSignersPrivateKeyMap } from './signers.js';

////////////////////////////////////////////////////////////////////////////////

const publicDecryptAbi = [
  {
    type: 'function',
    name: 'publicDecrypt',
    inputs: [{ name: 'handles', type: 'bytes32[]', internalType: 'bytes32[]' }],
    outputs: [
      {
        name: 'abiEncodedCleartexts',
        type: 'bytes',
        internalType: 'bytes',
      },
      { name: 'digest', type: 'bytes32', internalType: 'bytes32' },
      { name: 'signers', type: 'address[]', internalType: 'address[]' },
      { name: 'threshold', type: 'uint256', internalType: 'uint256' },
      { name: 'extraData', type: 'bytes', internalType: 'bytes' },
    ],
    stateMutability: 'view',
  },
] as const;

////////////////////////////////////////////////////////////////////////////////

export async function fetchPublicDecrypt(
  relayerClient: RelayerClientWithRuntime,
  parameters: FetchPublicDecryptParameters,
): Promise<FetchPublicDecryptReturnType> {
  const cleartextEthereumModule = relayerClient.runtime.ethereum as CleartextEthereumModule;

  const trustedClient = getTrustedClient(relayerClient);
  const res = (await relayerClient.runtime.ethereum.readContract(trustedClient, {
    abi: publicDecryptAbi,
    address: relayerClient.chain.fhevm.contracts.kmsVerifier.address as ChecksummedAddress,
    args: [parameters.payload.orderedHandles.map((h) => h.bytes32Hex)],
    functionName: publicDecryptAbi[0].name,
  })) as unknown[];

  const digest = res[1] as Bytes32Hex;
  const signersAddress = res[2] as readonly ChecksummedAddress[];
  const signers = getKmsSignersPrivateKeyMap(relayerClient);

  const kmsPublicDecryptEIP712Signatures: Bytes65Hex[] = [];

  for (let i = 0; i < signersAddress.length; ++i) {
    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
    const privateKey = signers.get(signersAddress[i]!);
    if (privateKey === undefined) {
      throw new Error('Unable to find kms signer');
    }

    const signature = await cleartextEthereumModule.sign({ hash: digest, privateKey });
    kmsPublicDecryptEIP712Signatures.push(signature);
  }

  return {
    orderedAbiEncodedClearValues: res[0] as BytesHex,
    kmsPublicDecryptEIP712Signatures,
    extraData: res[4] as BytesHex,
  };
}
