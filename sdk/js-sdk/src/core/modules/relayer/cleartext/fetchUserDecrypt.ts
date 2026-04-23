import type { BytesHex, ChecksummedAddress } from '../../../types/primitives.js';
import type { FetchUserDecryptResultItem } from '../../../types/relayer.js';
import type { CleartextEthereumModule } from '../../ethereum/types-ct.js';
import type { FetchUserDecryptParameters, FetchUserDecryptReturnType, RelayerClientWithRuntime } from '../types.js';
import { remove0x } from '../../../base/string.js';
import { asUint32BigInt, parseUintBigIntString, randomUniqueUints } from '../../../base/uint.js';
import { getTrustedClient } from '../../../runtime/CoreFhevm-p.js';
import { userDecryptResultToKmsSigncryptedShares } from '../utils.js';
import { getKmsSignersPrivateKeyMap } from './signers.js';

////////////////////////////////////////////////////////////////////////////////

const userDecryptAbi = [
  {
    type: 'function',
    name: 'userDecrypt',
    inputs: [
      {
        name: 'pairs',
        type: 'tuple[]',
        internalType: 'struct HandleContractPair[]',
        components: [
          { name: 'handle', type: 'bytes32', internalType: 'bytes32' },
          {
            name: 'contractAddress',
            type: 'address',
            internalType: 'address',
          },
        ],
      },
      { name: 'userAddress', type: 'address', internalType: 'address' },
      { name: 'publicKey', type: 'bytes', internalType: 'bytes' },
      {
        name: 'contractAddresses',
        type: 'address[]',
        internalType: 'address[]',
      },
      {
        name: 'startTimestamp',
        type: 'uint256',
        internalType: 'uint256',
      },
      {
        name: 'durationDays',
        type: 'uint256',
        internalType: 'uint256',
      },
      { name: 'userSignature', type: 'bytes', internalType: 'bytes' },
    ],
    outputs: [
      { name: 'payload', type: 'bytes', internalType: 'bytes' },
      { name: 'signers', type: 'address[]', internalType: 'address[]' },
      { name: 'threshold', type: 'uint256', internalType: 'uint256' },
      { name: 'extraData', type: 'bytes', internalType: 'bytes' },
    ],
    stateMutability: 'view',
  },
] as const;

////////////////////////////////////////////////////////////////////////////////

export async function fetchUserDecrypt(
  relayerClient: RelayerClientWithRuntime,
  parameters: FetchUserDecryptParameters,
): Promise<FetchUserDecryptReturnType> {
  const cleartextEthereumModule = relayerClient.runtime.ethereum as CleartextEthereumModule;
  const { payload } = parameters;
  const pairs = payload.handleContractPairs;
  const contractAddresses = payload.kmsDecryptEip712Message.contractAddresses;

  const authCount = contractAddresses.length;
  for (const { contractAddress: c } of pairs) {
    let authorized: boolean = false;
    for (let i = 0; i < authCount; ++i) {
      if (contractAddresses[i] == c) {
        authorized = true;
        break;
      }
    }
    if (!authorized) {
      throw new Error(`ContractAddressNotAuthorized: ${c} is not in the EIP-712 authorized contractAddresses list.`);
    }
  }

  const { kmsDecryptEip712Message, kmsDecryptEip712Signer, kmsDecryptEip712Signature } = payload;
  const trustedClient = getTrustedClient(relayerClient);
  const res = (await relayerClient.runtime.ethereum.readContract(trustedClient, {
    abi: userDecryptAbi,
    address: relayerClient.chain.fhevm.contracts.kmsVerifier.address as ChecksummedAddress,
    args: [
      pairs.map((p) => ({
        handle: p.handle.bytes32Hex,
        contractAddress: p.contractAddress,
      })),
      kmsDecryptEip712Signer,
      kmsDecryptEip712Message.publicKey,
      kmsDecryptEip712Message.contractAddresses,
      _parseUintBigIntString('kmsDecryptEip712Message.startTimestamp', kmsDecryptEip712Message.startTimestamp),
      _parseUintBigIntString('kmsDecryptEip712Message.durationDays', kmsDecryptEip712Message.durationDays),
      kmsDecryptEip712Signature,
    ],
    functionName: userDecryptAbi[0].name,
  })) as unknown[];

  const commonKmsSigncryptedSharePayload = res[0] as BytesHex;
  const commonKmsSigncryptedSharePayloadNo0x = remove0x(commonKmsSigncryptedSharePayload);
  const signersAddress = res[1] as readonly ChecksummedAddress[];
  const threshold = asUint32BigInt(res[2]);
  const extraData = res[3] as BytesHex;

  const signers = getKmsSignersPrivateKeyMap(relayerClient);

  const randomSignersAddress = randomUniqueUints(signersAddress.length, Number(threshold)).map(
    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
    (i) => signersAddress[i]!,
  );

  const result: FetchUserDecryptResultItem[] = [];

  for (const signerAddress of randomSignersAddress) {
    const privateKey = signers.get(signerAddress);
    if (privateKey === undefined) {
      throw new Error(`Unable to find KMS signer for address ${signerAddress}`);
    }

    const signature = await cleartextEthereumModule.sign({ hash: commonKmsSigncryptedSharePayload, privateKey });
    result.push({ signature: remove0x(signature), payload: commonKmsSigncryptedSharePayloadNo0x, extraData });
  }

  return userDecryptResultToKmsSigncryptedShares(result);
}

////////////////////////////////////////////////////////////////////////////////

function _parseUintBigIntString(label: string, value: string): bigint {
  const bn = parseUintBigIntString(value);
  if (bn === undefined) {
    throw new Error(`${label} is not a valid uint string, got ${JSON.stringify(value)}`);
  }
  return bn;
}
