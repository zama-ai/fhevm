import type {
  Bytes32Hex,
  BytesHex,
  ChecksummedAddress,
  Uint16Number,
  Uint256BigInt,
  Uint64BigInt,
  Uint8Number,
} from '../../../types/primitives.js';
import type { CleartextEthereumModule } from '../../ethereum/types-ct.js';
import type { TrustedClient } from '../../ethereum/types.js';
import type {
  FetchPadSharesParameters,
  FetchPadSharesReturnType,
  PadDecryptResponse,
  PadWitness,
  RelayerClientWithRuntime,
} from '../types.js';
import { assertIsBytes32Hex, assertIsBytes32HexArray, assertIsBytesHexArray } from '../../../base/bytes.js';
import { addressToChecksummedAddress, assertIsChecksummedAddressArray } from '../../../base/address.js';
import { MAX_UINT256, asUint256BigInt, tryParseUintBigIntString } from '../../../base/uint.js';
import { createCachedFetch } from '../../../base/cachedFetch.js';
import { getTrustedClient } from '../../../runtime/CoreFhevm-p.js';
import { getKmsSignersPrivateKeyMap } from './signers.js';
import { readCleartextExecutorAddress } from './forgeFhevmV1.js';

////////////////////////////////////////////////////////////////////////////////
// CleartextFHEVMExecutor.getPaddedInputterAddress()
////////////////////////////////////////////////////////////////////////////////

const getPaddedInputterAddressAbi = [
  {
    type: 'function',
    name: 'getPaddedInputterAddress',
    inputs: [],
    outputs: [{ name: '', type: 'address', internalType: 'address' }],
    stateMutability: 'view',
  },
] as const;

////////////////////////////////////////////////////////////////////////////////
// CleartextPaddedInputter ABI
////////////////////////////////////////////////////////////////////////////////

const padDecryptRequestAbiParameter = {
  name: 'request',
  type: 'tuple',
  internalType: 'struct PadDecryptRequest',
  components: [
    {
      name: 'auth',
      type: 'tuple',
      internalType: 'struct UserDecryptPermitV2',
      components: [
        { name: 'userAddress', type: 'address', internalType: 'address' },
        { name: 'publicKey', type: 'bytes', internalType: 'bytes' },
        { name: 'allowedContracts', type: 'address[]', internalType: 'address[]' },
        { name: 'startTimestamp', type: 'uint256', internalType: 'uint256' },
        { name: 'durationSeconds', type: 'uint256', internalType: 'uint256' },
        { name: 'extraData', type: 'bytes', internalType: 'bytes' },
      ],
    },
    { name: 'signature', type: 'bytes', internalType: 'bytes' },
    {
      name: 'payload',
      type: 'tuple',
      internalType: 'struct PadDecryptRequestPayload',
      components: [
        { name: 'witnessVersion', type: 'uint8', internalType: 'uint8' },
        { name: 'prfId', type: 'uint16', internalType: 'uint16' },
        { name: 'chainId', type: 'uint64', internalType: 'uint64' },
        {
          name: 'witnesses',
          type: 'tuple[]',
          internalType: 'struct PadWitness[]',
          components: [
            { name: 'submitter', type: 'address', internalType: 'address' },
            { name: 'consumer', type: 'address', internalType: 'address' },
            { name: 'expirationTimestamp', type: 'uint64', internalType: 'uint64' },
            { name: 'decryptors', type: 'address[]', internalType: 'address[]' },
            { name: 'nonce', type: 'bytes16', internalType: 'bytes16' },
            { name: 'label', type: 'bytes32', internalType: 'bytes32' },
            { name: 'padLength', type: 'uint16', internalType: 'uint16' },
          ],
        },
      ],
    },
  ],
} as const;

// [Flow.1] getCleartextPadSharesMaterial(PadDecryptRequest)
const getCleartextPadSharesMaterialAbi = [
  {
    type: 'function',
    name: 'getCleartextPadSharesMaterial',
    inputs: [padDecryptRequestAbiParameter],
    outputs: [
      {
        name: 'material',
        type: 'tuple',
        internalType: 'struct CleartextPadSharesMaterial',
        components: [
          { name: 'requestId', type: 'bytes32', internalType: 'bytes32' },
          {
            name: 'shares',
            type: 'tuple[]',
            internalType: 'struct PadShare[]',
            components: [
              { name: 'label', type: 'bytes32', internalType: 'bytes32' },
              { name: 'rawShares', type: 'bytes', internalType: 'bytes' },
              { name: 'partyId', type: 'uint32', internalType: 'uint32' },
              { name: 'degree', type: 'uint32', internalType: 'uint32' },
              { name: 'contextId', type: 'bytes32', internalType: 'bytes32' },
              { name: 'sessionId', type: 'bytes16', internalType: 'bytes16' },
              { name: 'extraData', type: 'bytes', internalType: 'bytes' },
              { name: 'signature', type: 'bytes', internalType: 'bytes' },
            ],
          },
          { name: 'signers', type: 'address[]', internalType: 'address[]' },
          { name: 'partyIds', type: 'uint32[]', internalType: 'uint32[]' },
          { name: 'shareDigests', type: 'bytes32[][]', internalType: 'bytes32[][]' },
          { name: 'threshold', type: 'uint256', internalType: 'uint256' },
        ],
      },
    ],
    stateMutability: 'view',
  },
] as const;

// [Flow.3] getCleartextPadResponsesMaterial(PadDecryptRequest, bytes[][])
const getCleartextPadResponsesMaterialAbi = [
  {
    type: 'function',
    name: 'getCleartextPadResponsesMaterial',
    inputs: [padDecryptRequestAbiParameter, { name: 'shareSignatures', type: 'bytes[][]', internalType: 'bytes[][]' }],
    outputs: [
      {
        name: 'responses',
        type: 'tuple',
        internalType: 'struct CleartextPadResponsesMaterial',
        components: [
          { name: 'requestId', type: 'bytes32', internalType: 'bytes32' },
          { name: 'signers', type: 'address[]', internalType: 'address[]' },
          { name: 'encryptedPayloads', type: 'bytes[]', internalType: 'bytes[]' },
          { name: 'transportDigests', type: 'bytes32[]', internalType: 'bytes32[]' },
        ],
      },
    ],
    stateMutability: 'view',
  },
] as const;

////////////////////////////////////////////////////////////////////////////////
// fetchPadShares
////////////////////////////////////////////////////////////////////////////////

/** The Solidity `PadDecryptRequest` struct, as both views take it. */
type PadDecryptRequest = {
  readonly auth: {
    readonly userAddress: ChecksummedAddress;
    readonly publicKey: BytesHex;
    readonly allowedContracts: readonly ChecksummedAddress[];
    readonly startTimestamp: Uint256BigInt;
    readonly durationSeconds: Uint256BigInt;
    readonly extraData: BytesHex;
  };
  readonly signature: BytesHex;
  readonly payload: {
    readonly witnessVersion: Uint8Number;
    readonly prfId: Uint16Number;
    readonly chainId: Uint64BigInt;
    readonly witnesses: readonly PadWitness[];
  };
};

/** [Flow.1] What the request's KMS nodes sign: `shareDigests[i]` for `signers[i]`. */
type CleartextPadSharesMaterial = {
  readonly requestId: Bytes32Hex;
  readonly signers: readonly ChecksummedAddress[];
  readonly shareDigests: ReadonlyArray<readonly Bytes32Hex[]>;
};

/** [Flow.3] Every node's encrypted payload and the transport digest it signs, index-aligned with the signers. */
type CleartextPadResponsesMaterial = {
  readonly encryptedPayloads: readonly BytesHex[];
  readonly transportDigests: readonly Bytes32Hex[];
};

////////////////////////////////////////////////////////////////////////////////

/**
 * Cleartext mock of RFC-030 pad decryption (tech-spec#547). `CleartextPaddedInputter` does everything on-chain but
 * the signatures, which this function makes with each KMS node's key:
 *   [Flow.1] read the share digests, [Flow.2] sign them, [Flow.3] read every node's encrypted payload and transport
 *   digest, [Flow.4] sign those, giving the {@link PadDecryptResponse}s the real relayer returns.
 */
export async function fetchPadShares(
  relayerClient: RelayerClientWithRuntime,
  parameters: FetchPadSharesParameters,
): Promise<FetchPadSharesReturnType> {
  const ethereum = relayerClient.runtime.ethereum as CleartextEthereumModule;
  const trustedClient = getTrustedClient(relayerClient);

  const address = await _readPaddedInputterAddress(relayerClient, trustedClient);
  const request = _createPadDecryptRequest(parameters);

  // [Flow.1] One view call: the request is checked on-chain, and every node's share digests come back.
  const rawMaterial = (await ethereum.readContract(trustedClient, {
    abi: getCleartextPadSharesMaterialAbi,
    address,
    args: [request],
    functionName: getCleartextPadSharesMaterialAbi[0].name,
  })) as Record<string, unknown>;

  _assertIsCleartextPadSharesMaterial(rawMaterial);
  const material: CleartextPadSharesMaterial = rawMaterial;
  const kmsSignersPrivateKeys: readonly BytesHex[] = _kmsSignersPrivateKeysOf(relayerClient, material.signers);

  // [Flow.2] Each KMS node signs each of its share digests.
  const shareKmsSignatures = await Promise.all(
    _zip(kmsSignersPrivateKeys, material.shareDigests).map(([privateKey, digests]) =>
      Promise.all(digests.map((hash) => ethereum.sign({ hash, privateKey }))),
    ),
  );

  // [Flow.3] One view call: every node's encrypted payload and the transport digest it signs.
  const rawResponses = (await ethereum.readContract(trustedClient, {
    abi: getCleartextPadResponsesMaterialAbi,
    address,
    args: [request, shareKmsSignatures],
    functionName: getCleartextPadResponsesMaterialAbi[0].name,
  })) as Record<string, unknown>;

  _assertIsCleartextPadResponsesMaterial(rawResponses, material);
  const responses: CleartextPadResponsesMaterial = rawResponses;

  // [Flow.4] Each KMS node signs its transport digest, giving its `PadDecryptResponse`.
  const nodeResponses = _zip(responses.encryptedPayloads, responses.transportDigests);
  const padDecryptResponses: readonly PadDecryptResponse[] = await Promise.all(
    _zip(kmsSignersPrivateKeys, nodeResponses).map(
      async ([privateKey, [encryptedPayload, hash]]: Readonly<
        [BytesHex, Readonly<[BytesHex, Bytes32Hex]>]
      >): Promise<PadDecryptResponse> => {
        const signature = await ethereum.sign({ hash, privateKey });
        return Object.freeze({ requestId: material.requestId, encryptedPayload, signature });
      },
    ),
  );

  return Object.freeze(padDecryptResponses);
}

////////////////////////////////////////////////////////////////////////////////

/**
 * The `PadDecryptRequest` struct, as both views take it.
 */
function _createPadDecryptRequest(parameters: FetchPadSharesParameters): PadDecryptRequest {
  const { payload } = parameters;
  const message = payload.kmsDecryptEip712Message;
  return {
    auth: {
      userAddress: message.userAddress,
      publicKey: message.publicKey,
      allowedContracts: message.allowedContracts,
      startTimestamp: _parseUint256BigIntString('kmsDecryptEip712Message.startTimestamp', message.startTimestamp),
      durationSeconds: _parseUint256BigIntString('kmsDecryptEip712Message.durationSeconds', message.durationSeconds),
      extraData: message.extraData,
    },
    signature: payload.kmsDecryptEip712Signature,
    payload: {
      witnessVersion: payload.witnessVersion,
      prfId: payload.prfId,
      chainId: payload.chainId,
      witnesses: payload.witnesses.map((w) => ({
        submitter: w.submitter,
        consumer: w.consumer,
        expirationTimestamp: w.expirationTimestamp,
        decryptors: w.decryptors,
        nonce: w.nonce,
        label: w.label,
        padLength: w.padLength,
      })),
    },
  };
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Each signer's private key, index-aligned with `signers`.
 */
function _kmsSignersPrivateKeysOf(
  relayerClient: RelayerClientWithRuntime,
  signers: readonly ChecksummedAddress[],
): readonly BytesHex[] {
  const privateKeys = getKmsSignersPrivateKeyMap(relayerClient);
  return signers.map((signer) => {
    const privateKey = privateKeys.get(signer);
    if (privateKey === undefined) {
      throw new Error(`Unable to find KMS signer for address ${signer}`);
    }
    return privateKey;
  });
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Reads the struct by component name: viem returns a plain object, ethers
 * a `Result`, and both expose the names.
 */
function _assertIsCleartextPadSharesMaterial(
  value: Record<string, unknown>,
): asserts value is CleartextPadSharesMaterial {
  const { requestId, signers, shareDigests } = value;

  assertIsBytes32Hex(requestId, { subject: 'requestId' });
  assertIsChecksummedAddressArray(signers, { subject: 'signers' });

  // assert one array of Bytes32Hex per signer
  if (!Array.isArray(shareDigests) || shareDigests.length !== signers.length) {
    throw new Error(
      `Malformed pad shares material: ${signers.length} signers, shareDigests is not one row per signer.`,
    );
  }
  (shareDigests as unknown[]).forEach((row, i) => {
    assertIsBytes32HexArray(row, { subject: `shareDigests[${i}]` });
  });
}

////////////////////////////////////////////////////////////////////////////////

function _assertIsCleartextPadResponsesMaterial(
  value: Record<string, unknown>,
  material: CleartextPadSharesMaterial,
): asserts value is CleartextPadResponsesMaterial {
  const n = material.signers.length;
  const { requestId, encryptedPayloads, transportDigests } = value;

  // 1. assert `encryptedPayloads` is an array of BytesHex and `transportDigests` an array of Bytes32Hex
  assertIsBytesHexArray(encryptedPayloads, { subject: 'encryptedPayloads' });
  assertIsBytes32HexArray(transportDigests, { subject: 'transportDigests' });

  // 2. assert lengths are valid
  if (encryptedPayloads.length !== n || transportDigests.length !== n) {
    throw new Error(
      `Malformed pad responses material: ${n} signers, ${encryptedPayloads.length} payloads, ${transportDigests.length} digests.`,
    );
  }

  // 3. assert `requestId` is `material.requestId`
  if (requestId !== material.requestId) {
    throw new Error(
      `Malformed pad responses material: requestId ${String(requestId)}, expected ${material.requestId}.`,
    );
  }
}

////////////////////////////////////////////////////////////////////////////////
// PaddedInputter address
////////////////////////////////////////////////////////////////////////////////

type ReadPaddedInputterAddressContext = {
  readonly relayerClient: RelayerClientWithRuntime;
  readonly trustedClient: TrustedClient;
};

// The executor bakes the PaddedInputter address in, so the lookup is cached permanently. Keyed like the executor
// lookup it follows (ACL address + chainId).
const cachedReadPaddedInputterAddress = createCachedFetch<
  ReadPaddedInputterAddressContext,
  Record<string, never>,
  ChecksummedAddress
>({
  executeFn: async ({ relayerClient, trustedClient }) => {
    const res = await relayerClient.runtime.ethereum.readContract(trustedClient, {
      abi: getPaddedInputterAddressAbi,
      address: await readCleartextExecutorAddress(relayerClient, trustedClient),
      args: [],
      functionName: getPaddedInputterAddressAbi[0].name,
    });
    return addressToChecksummedAddress(res as ChecksummedAddress);
  },
  cacheKeyFn: ({ relayerClient }) =>
    `${(relayerClient.chain.fhevm.contracts.acl.address as string).toLowerCase()}:${relayerClient.chain.id}`,
});

////////////////////////////////////////////////////////////////////////////////

/**
 * The PaddedInputter of the stack, read from `CleartextFHEVMExecutor.getPaddedInputterAddress()`.
 */
function _readPaddedInputterAddress(
  relayerClient: RelayerClientWithRuntime,
  trustedClient: TrustedClient,
): Promise<ChecksummedAddress> {
  return cachedReadPaddedInputterAddress.execute({ relayerClient, trustedClient }, {});
}

////////////////////////////////////////////////////////////////////////////////

/** Pairs `a[i]` with `b[i]`, one pair per KMS node; throws if the lengths differ. */
function _zip<A, B>(a: readonly A[], b: readonly B[]): ReadonlyArray<readonly [A, B]> {
  if (a.length !== b.length) {
    throw new Error(`Expected one entry per KMS node, got ${a.length} and ${b.length}.`);
  }
  return a.map((x, i) => [x, b[i] as B] as const);
}

function _parseUint256BigIntString(label: string, value: string): Uint256BigInt {
  const bn = tryParseUintBigIntString(value, MAX_UINT256);
  if (bn === undefined) {
    throw new Error(`${label} is not a valid uint256 string, got ${JSON.stringify(value)}`);
  }
  return asUint256BigInt(bn, { subject: label });
}
