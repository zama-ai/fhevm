import type { BytesHex, ChecksummedAddress, Uint32BigInt } from '../../../types/primitives.js';
import type {
  FetchDelegatedUserDecryptParameters,
  FetchUserDecryptReturnType,
  RelayerClientWithRuntime,
} from '../types.js';
import type { KmsSignersContext } from '../../../types/kmsSignersContext.js';
import type { FhevmClientFrozenContext } from '../../../types/fhevmClientFrozenContext-p.js';
import { asUint32BigInt, tryParseUintBigIntString } from '../../../base/uint.js';
import { getTrustedClient } from '../../../runtime/CoreFhevm-p.js';
import { signKmsSigncryptedShares } from './signers.js';
import { isForgeFhevmV1, readPlaintexts, xorMaskWithPublicKey } from './forgeFhevmV1.js';
import { readCurrentKmsSignersContext } from '../../../host-contracts/readKmsSignersContext-p.js';

////////////////////////////////////////////////////////////////////////////////
// runDelegatedUserDecryptOffChain
////////////////////////////////////////////////////////////////////////////////

async function runDelegatedUserDecryptOffChain(
  relayerClient: RelayerClientWithRuntime,
  payload: FetchDelegatedUserDecryptParameters['payload'],
  fhevmContext: FhevmClientFrozenContext,
): Promise<{
  readonly commonKmsSigncryptedSharePayload: BytesHex;
  readonly signersAddress: readonly ChecksummedAddress[];
  readonly threshold: Uint32BigInt;
  readonly extraData: BytesHex;
}> {
  const { kmsDecryptEip712Message } = payload;
  const pairs = payload.handleContractPairs;
  const trustedClient = getTrustedClient(relayerClient);

  const rawCleartexts = await readPlaintexts(
    relayerClient,
    trustedClient,
    pairs.map((p) => p.handle),
  );

  const currentKmsSignersContext: KmsSignersContext = await readCurrentKmsSignersContext(relayerClient, {
    kmsVerifierAddress: relayerClient.chain.fhevm.contracts.kmsVerifier.address as ChecksummedAddress,
    protocolConfigAddress: relayerClient.chain.fhevm.contracts.protocolConfig?.address as
      | ChecksummedAddress
      | undefined,
    fhevmContext,
  });
  const signersAddress = currentKmsSignersContext.signers;
  const threshold = currentKmsSignersContext.threshold;

  const maskedCleartexts = xorMaskWithPublicKey(kmsDecryptEip712Message.publicKey, rawCleartexts);

  // extraData: the permit's message.extraData is already asserted equal to the
  // current KmsSignersContext extraData by the caller (fetchKmsSigncryptedSharesV1
  // step 9), so reusing it keeps the shares consistent with that context.
  const extraData = kmsDecryptEip712Message.extraData;

  const commonKmsSigncryptedSharePayload = relayerClient.runtime.ethereum.encode({
    types: ['uint256[]', 'bytes'],
    values: [maskedCleartexts, extraData],
  });

  return {
    signersAddress,
    threshold: BigInt(threshold) as Uint32BigInt,
    commonKmsSigncryptedSharePayload,
    extraData,
  };
}

////////////////////////////////////////////////////////////////////////////////
// runDelegatedUserDecryptOnChain
////////////////////////////////////////////////////////////////////////////////

const delegatedUserDecryptV1Abi = [
  {
    type: 'function',
    name: 'delegatedUserDecryptV1',
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
      {
        name: 'permit',
        type: 'tuple',
        internalType: 'struct DelegatedUserDecryptPermitV1',
        components: [
          { name: 'delegateAddress', type: 'address', internalType: 'address' },
          { name: 'publicKey', type: 'bytes', internalType: 'bytes' },
          {
            name: 'contractAddresses',
            type: 'address[]',
            internalType: 'address[]',
          },
          { name: 'delegatorAddress', type: 'address', internalType: 'address' },
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

async function runDelegatedUserDecryptOnChain(
  relayerClient: RelayerClientWithRuntime,
  payload: FetchDelegatedUserDecryptParameters['payload'],
): Promise<{
  readonly commonKmsSigncryptedSharePayload: BytesHex;
  readonly signersAddress: readonly ChecksummedAddress[];
  readonly threshold: Uint32BigInt;
  readonly extraData: BytesHex;
}> {
  const { kmsDecryptEip712Message, kmsDecryptEip712Signer, kmsDecryptEip712Signature } = payload;
  const pairs = payload.handleContractPairs;
  const trustedClient = getTrustedClient(relayerClient);
  // The permit's extraData is passed verbatim, as in fetchUserDecryptV1.
  const res = (await relayerClient.runtime.ethereum.readContract(trustedClient, {
    abi: delegatedUserDecryptV1Abi,
    address: relayerClient.chain.fhevm.contracts.kmsVerifier.address as ChecksummedAddress,
    args: [
      pairs.map((p) => ({
        handle: p.handle.bytes32Hex,
        contractAddress: p.contractAddress,
      })),
      {
        delegateAddress: kmsDecryptEip712Signer,
        publicKey: kmsDecryptEip712Message.publicKey,
        contractAddresses: kmsDecryptEip712Message.contractAddresses,
        delegatorAddress: kmsDecryptEip712Message.delegatorAddress,
        startTimestamp: _parseUintBigIntString(
          'kmsDecryptEip712Message.startTimestamp',
          kmsDecryptEip712Message.startTimestamp,
        ),
        durationDays: _parseUintBigIntString(
          'kmsDecryptEip712Message.durationDays',
          kmsDecryptEip712Message.durationDays,
        ),
        extraData: kmsDecryptEip712Message.extraData,
      },
      kmsDecryptEip712Signature,
    ],
    functionName: delegatedUserDecryptV1Abi[0].name,
  })) as unknown[];

  const commonKmsSigncryptedSharePayload = res[0] as BytesHex;
  const signersAddress = res[1] as readonly ChecksummedAddress[];
  const threshold = asUint32BigInt(res[2]);
  const extraData = res[3] as BytesHex;

  return {
    commonKmsSigncryptedSharePayload,
    signersAddress,
    threshold,
    extraData,
  };
}

////////////////////////////////////////////////////////////////////////////////

export async function fetchDelegatedUserDecrypt(
  relayerClient: RelayerClientWithRuntime,
  parameters: FetchDelegatedUserDecryptParameters,
): Promise<FetchUserDecryptReturnType> {
  const { payload, fhevmContext } = parameters;
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

  const offChain = await isForgeFhevmV1(relayerClient, fhevmContext);

  let res;
  if (offChain) {
    res = await runDelegatedUserDecryptOffChain(relayerClient, payload, fhevmContext);
  } else {
    res = await runDelegatedUserDecryptOnChain(relayerClient, payload);
  }

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
