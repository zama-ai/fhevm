import type { BytesHex, ChecksummedAddress, Uint32BigInt } from '../../../types/primitives.js';
import type { FetchUserDecryptParametersV2, FetchUserDecryptReturnType, RelayerClientWithRuntime } from '../types.js';
import { asUint32BigInt, tryParseUintBigIntString } from '../../../base/uint.js';
import { getTrustedClient } from '../../../runtime/CoreFhevm-p.js';
import { signKmsSigncryptedShares } from './signers.js';

////////////////////////////////////////////////////////////////////////////////
// runUserDecryptV2OnChain
////////////////////////////////////////////////////////////////////////////////

const userDecryptV2Abi = [
  {
    type: 'function',
    name: 'userDecryptV2',
    inputs: [
      {
        name: 'handles',
        type: 'tuple[]',
        internalType: 'struct HandleEntry[]',
        components: [
          { name: 'handle', type: 'bytes32', internalType: 'bytes32' },
          {
            name: 'contractAddress',
            type: 'address',
            internalType: 'address',
          },
          {
            name: 'ownerAddress',
            type: 'address',
            internalType: 'address',
          },
        ],
      },
      {
        name: 'permit',
        type: 'tuple',
        internalType: 'struct UserDecryptPermitV2',
        components: [
          { name: 'userAddress', type: 'address', internalType: 'address' },
          { name: 'publicKey', type: 'bytes', internalType: 'bytes' },
          {
            name: 'allowedContracts',
            type: 'address[]',
            internalType: 'address[]',
          },
          {
            name: 'startTimestamp',
            type: 'uint256',
            internalType: 'uint256',
          },
          {
            name: 'durationSeconds',
            type: 'uint256',
            internalType: 'uint256',
          },
          { name: 'extraData', type: 'bytes', internalType: 'bytes' },
        ],
      },
      { name: 'signature', type: 'bytes', internalType: 'bytes' },
    ],
    outputs: [
      { name: 'payload', type: 'bytes', internalType: 'bytes' },
      { name: 'signers', type: 'address[]', internalType: 'address[]' },
      { name: 'threshold', type: 'uint256', internalType: 'uint256' },
      { name: 'signedExtraData', type: 'bytes', internalType: 'bytes' },
    ],
    stateMutability: 'view',
  },
] as const;

async function runUserDecryptV2OnChain(
  relayerClient: RelayerClientWithRuntime,
  payload: FetchUserDecryptParametersV2['payload'],
): Promise<{
  readonly commonKmsSigncryptedSharePayload: BytesHex;
  readonly signersAddress: readonly ChecksummedAddress[];
  readonly threshold: Uint32BigInt;
  readonly extraData: BytesHex;
}> {
  const { kmsDecryptEip712Message, kmsDecryptEip712Signature } = payload;
  const pairs = payload.handleContractPairs;
  const trustedClient = getTrustedClient(relayerClient);
  // The signed permit fields are passed verbatim, extraData included: the host runs the gateway's and the
  // kms-worker's checks against them and returns the signers of the KMS context the extraData names.
  const res = (await relayerClient.runtime.ethereum.readContract(trustedClient, {
    abi: userDecryptV2Abi,
    address: relayerClient.chain.fhevm.contracts.kmsVerifier.address as ChecksummedAddress,
    args: [
      pairs.map((p) => ({
        handle: p.handle.bytes32Hex,
        contractAddress: p.contractAddress,
        ownerAddress: p.ownerAddress,
      })),
      {
        userAddress: kmsDecryptEip712Message.userAddress,
        publicKey: kmsDecryptEip712Message.publicKey,
        allowedContracts: kmsDecryptEip712Message.allowedContracts,
        startTimestamp: _parseUintBigIntString(
          'kmsDecryptEip712Message.startTimestamp',
          kmsDecryptEip712Message.startTimestamp,
        ),
        durationSeconds: _parseUintBigIntString(
          'kmsDecryptEip712Message.durationSeconds',
          kmsDecryptEip712Message.durationSeconds,
        ),
        extraData: kmsDecryptEip712Message.extraData,
      },
      kmsDecryptEip712Signature,
    ],
    functionName: userDecryptV2Abi[0].name,
  })) as unknown[];

  return {
    commonKmsSigncryptedSharePayload: res[0] as BytesHex,
    signersAddress: res[1] as readonly ChecksummedAddress[],
    threshold: asUint32BigInt(res[2]),
    extraData: res[3] as BytesHex,
  };
}

////////////////////////////////////////////////////////////////////////////////

export async function fetchUserDecryptV2(
  relayerClient: RelayerClientWithRuntime,
  parameters: FetchUserDecryptParametersV2,
): Promise<FetchUserDecryptReturnType> {
  // On chain only: forge-fhevm v1 has no unified (V2) decryption permit path.
  const res = await runUserDecryptV2OnChain(relayerClient, parameters.payload);
  return signKmsSigncryptedShares(relayerClient, { ...res, threshold: Number(res.threshold) });
}

////////////////////////////////////////////////////////////////////////////////

function _parseUintBigIntString(label: string, value: string): bigint {
  const bn = tryParseUintBigIntString(value);
  if (bn === undefined) {
    throw new Error(`${label} is not a valid uint string, got ${JSON.stringify(value)}`);
  }
  return bn;
}
