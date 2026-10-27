import type { Bytes32Hex, BytesHex, ChecksummedAddress, Uint32Number } from '../../../types/primitives.js';
import type { CleartextEthereumModule } from '../../ethereum/types-ct.js';
import type { TrustedClient } from '../../ethereum/types.js';
import type {
  FetchPadSharesParameters,
  FetchPadSharesReturnType,
  PadDecryptResponse,
  PadShare,
  RelayerClientWithRuntime,
} from '../types.js';
import { assertIsBytes32Hex, assertIsBytesHex } from '../../../base/bytes.js';
import { addressToChecksummedAddress } from '../../../base/address.js';
import { asUint32Number, tryParseUintBigIntString } from '../../../base/uint.js';
import { createCachedFetch } from '../../../base/cachedFetch.js';
import { getTrustedClient } from '../../../runtime/CoreFhevm-p.js';
import { getKmsSignersPrivateKeyMap } from './signers.js';
import { readCleartextExecutorAddress } from './forgeFhevmV1.js';
import { encodeCleartextPadDecryptResponse } from './padDecryptResponse.js';

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
// CleartextPaddedInputter.getCleartextPadSharesMaterial(PadDecryptRequest)
////////////////////////////////////////////////////////////////////////////////

const getCleartextPadSharesMaterialAbi = [
  {
    type: 'function',
    name: 'getCleartextPadSharesMaterial',
    inputs: [
      {
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
              { name: 'prf', type: 'uint16', internalType: 'uint16' },
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
      },
    ],
    outputs: [
      {
        name: 'material',
        type: 'tuple',
        internalType: 'struct CleartextPadSharesMaterial',
        components: [
          { name: 'requestId', type: 'bytes32', internalType: 'bytes32' },
          { name: 'packingFactor', type: 'uint32', internalType: 'uint32' },
          { name: 'degree', type: 'uint32', internalType: 'uint32' },
          { name: 'extraData', type: 'bytes', internalType: 'bytes' },
          {
            name: 'padShares',
            type: 'tuple[]',
            internalType: 'struct PadShare[]',
            components: [
              { name: 'label', type: 'bytes32', internalType: 'bytes32' },
              { name: 'sessionId', type: 'bytes16', internalType: 'bytes16' },
              { name: 'share', type: 'bytes', internalType: 'bytes' },
            ],
          },
          { name: 'signers', type: 'address[]', internalType: 'address[]' },
          { name: 'partyIds', type: 'uint32[]', internalType: 'uint32[]' },
          { name: 'digests', type: 'bytes32[]', internalType: 'bytes32[]' },
          { name: 'threshold', type: 'uint256', internalType: 'uint256' },
        ],
      },
    ],
    stateMutability: 'view',
  },
] as const;

////////////////////////////////////////////////////////////////////////////////
// fetchPadShares
////////////////////////////////////////////////////////////////////////////////

/** What every KMS node of the request's context signs, index-aligned with `signers`. */
type CleartextPadSharesMaterial = {
  readonly requestId: Bytes32Hex;
  readonly packingFactor: Uint32Number;
  readonly degree: Uint32Number;
  readonly extraData: BytesHex;
  readonly padShares: readonly PadShare[];
  readonly signers: readonly ChecksummedAddress[];
  readonly partyIds: readonly Uint32Number[];
  readonly digests: readonly Bytes32Hex[];
};

/**
 * Cleartext mock of RFC-030 pad decryption.
 *
 * `CleartextPaddedInputter.getCleartextPadSharesMaterial` runs the KMS authentication and authorization checks
 * on-chain and returns, for every KMS node of the request's context, its party id and the EIP-712 digest of its
 * `PadDecryptResponsePayload`. This function plays those nodes: each signs its digest with its local key and the
 * result is a {@link PadDecryptResponse}, the shape the real relayer returns. `encrypted` is the cleartext mock
 * encoding (see `padDecryptResponse.ts`).
 */
export async function fetchPadShares(
  relayerClient: RelayerClientWithRuntime,
  parameters: FetchPadSharesParameters,
): Promise<FetchPadSharesReturnType> {
  const cleartextEthereumModule = relayerClient.runtime.ethereum as CleartextEthereumModule;
  const material = await _readPadSharesMaterial(relayerClient, parameters);
  const privateKeys = getKmsSignersPrivateKeyMap(relayerClient);

  const responses: PadDecryptResponse[] = [];
  for (let i = 0; i < material.signers.length; ++i) {
    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
    const signerAddress = material.signers[i]!;
    const privateKey = privateKeys.get(signerAddress);
    if (privateKey === undefined) {
      throw new Error(`Unable to find KMS signer for address ${signerAddress}`);
    }

    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
    const partyId = material.partyIds[i]!;
    const payload = {
      packingFactor: material.packingFactor,
      extraData: material.extraData,
      degree: material.degree,
      partyId,
      padShares: material.padShares,
    };
    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
    const signature = await cleartextEthereumModule.sign({ hash: material.digests[i]!, privateKey });

    responses.push(
      Object.freeze({
        requestId: material.requestId,
        partyId,
        encrypted: encodeCleartextPadDecryptResponse(cleartextEthereumModule, payload, signature),
      }),
    );
  }

  return Object.freeze(responses);
}

async function _readPadSharesMaterial(
  relayerClient: RelayerClientWithRuntime,
  parameters: FetchPadSharesParameters,
): Promise<CleartextPadSharesMaterial> {
  const { payload } = parameters;
  const message = payload.kmsDecryptEip712Message;
  const trustedClient = getTrustedClient(relayerClient);

  const res = (await relayerClient.runtime.ethereum.readContract(trustedClient, {
    abi: getCleartextPadSharesMaterialAbi,
    address: await _readPaddedInputterAddress(relayerClient, trustedClient),
    args: [
      {
        auth: {
          userAddress: message.userAddress,
          publicKey: message.publicKey,
          allowedContracts: message.allowedContracts,
          startTimestamp: _parseUintBigIntString('kmsDecryptEip712Message.startTimestamp', message.startTimestamp),
          durationSeconds: _parseUintBigIntString('kmsDecryptEip712Message.durationSeconds', message.durationSeconds),
          extraData: message.extraData,
        },
        signature: payload.kmsDecryptEip712Signature,
        payload: {
          witnessVersion: payload.witnessVersion,
          prf: payload.prf,
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
      },
    ],
    functionName: getCleartextPadSharesMaterialAbi[0].name,
  })) as Record<string, unknown>;

  return _parsePadSharesMaterial(res);
}

/**
 * Reads the struct by component name: viem returns a plain object, ethers a `Result`, and both expose the names.
 * ethers decodes small uints as `bigint`, viem as `number`.
 */
function _parsePadSharesMaterial(res: Record<string, unknown>): CleartextPadSharesMaterial {
  const { requestId, extraData } = res;
  assertIsBytes32Hex(requestId, { subject: 'requestId' });
  assertIsBytesHex(extraData, { subject: 'extraData' });

  const rawPadShares = Array.from(
    res.padShares as ReadonlyArray<{ readonly label: unknown; readonly sessionId: unknown; readonly share: unknown }>,
  );
  const padShares: PadShare[] = rawPadShares.map((item, i) => {
    const { label, sessionId, share } = item;
    assertIsBytes32Hex(label, { subject: `padShares[${i}].label` });
    assertIsBytesHex(sessionId, { subject: `padShares[${i}].sessionId` });
    assertIsBytesHex(share, { subject: `padShares[${i}].share` });
    return Object.freeze({ label, sessionId, share });
  });

  const signers = Array.from(res.signers as readonly string[], (a) =>
    addressToChecksummedAddress(a as ChecksummedAddress),
  );
  const partyIds = Array.from(res.partyIds as readonly unknown[], (p, i) =>
    asUint32Number(_toNumber(p), { subject: `partyIds[${i}]` }),
  );
  const digests = Array.from(res.digests as readonly unknown[], (d, i) => {
    assertIsBytes32Hex(d, { subject: `digests[${i}]` });
    return d;
  });
  if (partyIds.length !== signers.length || digests.length !== signers.length) {
    throw new Error(
      `Malformed pad shares material: ${signers.length} signers, ${partyIds.length} party ids, ${digests.length} digests.`,
    );
  }

  return Object.freeze({
    requestId,
    packingFactor: asUint32Number(_toNumber(res.packingFactor), { subject: 'packingFactor' }),
    degree: asUint32Number(_toNumber(res.degree), { subject: 'degree' }),
    extraData,
    padShares: Object.freeze(padShares),
    signers: Object.freeze(signers),
    partyIds: Object.freeze(partyIds),
    digests: Object.freeze(digests),
  });
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

/** The PaddedInputter of the stack, read from `CleartextFHEVMExecutor.getPaddedInputterAddress()`. */
function _readPaddedInputterAddress(
  relayerClient: RelayerClientWithRuntime,
  trustedClient: TrustedClient,
): Promise<ChecksummedAddress> {
  return cachedReadPaddedInputterAddress.execute({ relayerClient, trustedClient }, {});
}

////////////////////////////////////////////////////////////////////////////////

function _toNumber(value: unknown): unknown {
  return typeof value === 'bigint' ? Number(value) : value;
}

function _parseUintBigIntString(label: string, value: string): bigint {
  const bn = tryParseUintBigIntString(value);
  if (bn === undefined) {
    throw new Error(`${label} is not a valid uint string, got ${JSON.stringify(value)}`);
  }
  return bn;
}
