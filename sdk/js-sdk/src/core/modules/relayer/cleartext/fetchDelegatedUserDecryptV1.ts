import type { BytesHex, ChecksummedAddress, Uint32BigInt } from '../../../types/primitives.js';
import type { FetchUserDecryptResultItem } from '../../../types/relayer.js';
import type { CleartextEthereumModule } from '../../ethereum/types-ct.js';
import type {
  FetchDelegatedUserDecryptParameters,
  FetchUserDecryptReturnType,
  RelayerClientWithRuntime,
} from '../types.js';
import type { KmsSignersContext } from '../../../types/kmsSignersContext.js';
import type { FhevmClientFrozenContext } from '../../../types/fhevmClientFrozenContext-p.js';
import { remove0x } from '../../../base/string.js';
import { asUint32BigInt, tryParseUintBigIntString, randomUniqueUints } from '../../../base/uint.js';
import { getTrustedClient } from '../../../runtime/CoreFhevm-p.js';
import { userDecryptResultToKmsSigncryptedShares } from '../utils.js';
import { getKmsSignersPrivateKeyMap } from './signers.js';
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

const delegatedUserDecryptAbi = [
  {
    type: 'function',
    name: 'delegatedUserDecrypt',
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
      { name: 'delegator', type: 'address', internalType: 'address' },
      { name: 'delegate', type: 'address', internalType: 'address' },
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
      { name: 'delegateSignature', type: 'bytes', internalType: 'bytes' },
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
  const res = (await relayerClient.runtime.ethereum.readContract(trustedClient, {
    abi: delegatedUserDecryptAbi,
    address: relayerClient.chain.fhevm.contracts.kmsVerifier.address as ChecksummedAddress,
    args: [
      pairs.map((p) => ({
        handle: p.handle.bytes32Hex,
        contractAddress: p.contractAddress,
      })),
      kmsDecryptEip712Message.delegatorAddress,
      kmsDecryptEip712Signer,
      kmsDecryptEip712Message.publicKey,
      kmsDecryptEip712Message.contractAddresses,
      _parseUintBigIntString('kmsDecryptEip712Message.startTimestamp', kmsDecryptEip712Message.startTimestamp),
      _parseUintBigIntString('kmsDecryptEip712Message.durationDays', kmsDecryptEip712Message.durationDays),
      kmsDecryptEip712Signature,
    ],
    functionName: delegatedUserDecryptAbi[0].name,
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
  const cleartextEthereumModule = relayerClient.runtime.ethereum as CleartextEthereumModule;
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

  const commonKmsSigncryptedSharePayload = res.commonKmsSigncryptedSharePayload;
  const extraData = res.extraData;
  const threshold = res.threshold;
  const signersAddress = res.signersAddress;

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
    result.push({ signature: remove0x(signature), payload: remove0x(commonKmsSigncryptedSharePayload), extraData });
  }

  return userDecryptResultToKmsSigncryptedShares(result);
}

////////////////////////////////////////////////////////////////////////////////

function _parseUintBigIntString(label: string, value: string): bigint {
  const bn = tryParseUintBigIntString(value);
  if (bn === undefined) {
    throw new Error(`${label} is not a valid uint string, got ${JSON.stringify(value)}`);
  }
  return bn;
}
