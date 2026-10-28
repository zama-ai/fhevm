import type { Bytes32Hex, BytesHex, ChecksummedAddress, Uint32Number } from '../../../types/primitives.js';
import type { CleartextEthereumModule } from '../../ethereum/types-ct.js';
import type {
  FetchPadSharesParameters,
  FetchPadSharesReturnType,
  PadDecryptResponse,
  PadShare,
  RelayerClientWithRuntime,
} from '../types.js';
import { keccak_256 } from '@noble/hashes/sha3.js';
import { assertIsBytes32Hex, assertIsBytesHex, bytes32ToHex, concatBytesHex, hexToBytes } from '../../../base/bytes.js';
import { addressToChecksummedAddress } from '../../../base/address.js';
import { tryParseUintBigIntString } from '../../../base/uint.js';
import { getTrustedClient } from '../../../runtime/CoreFhevm-p.js';
import { createKmsEip712Domain } from '../../../kms/createKmsEip712Domain.js';
import {
  kmsPadDecryptResponseEip712PrimaryType,
  kmsPadDecryptResponseEip712Types,
} from '../../../kms/kmsPadDecryptResponseEip712Types.js';
import { getKmsSignersPrivateKeyMap } from './signers.js';
import { CLEARTEXT_PAD_PACKING_FACTOR, encodeCleartextPadDecryptResponse } from './padDecryptResponse.js';

////////////////////////////////////////////////////////////////////////////////
// CleartextInputVerifier.getPadShares(PadDecryptRequest)
////////////////////////////////////////////////////////////////////////////////

const getPadSharesAbi = [
  {
    type: 'function',
    name: 'getPadShares',
    inputs: [
      {
        name: 'request',
        type: 'tuple',
        internalType: 'struct PadDecryptRequest',
        components: [
          {
            name: 'auth',
            type: 'tuple',
            internalType: 'struct UserDecryptRequestVerification',
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
      { name: 'threshold', type: 'uint256', internalType: 'uint256' },
      { name: 'extraData', type: 'bytes', internalType: 'bytes' },
    ],
    stateMutability: 'view',
  },
] as const;

////////////////////////////////////////////////////////////////////////////////
// fetchPadShares
////////////////////////////////////////////////////////////////////////////////

/**
 * Cleartext mock of RFC-030 pad decryption.
 *
 * `CleartextInputVerifier.getPadShares` runs the KMS authentication and
 * authorization checks on-chain and returns each pad XOR-masked with the first
 * 32 bytes of the request's public key, with a per-label session id. This
 * function then plays every KMS
 * node of the request's context: each node EIP-712-signs a
 * `PadDecryptResponsePayload` with its local key and returns it as a
 * {@link PadDecryptResponse}, exactly the shape the real relayer will return.
 * `encrypted` is the cleartext mock encoding (see `padDecryptResponse.ts`).
 */
export async function fetchPadShares(
  relayerClient: RelayerClientWithRuntime,
  parameters: FetchPadSharesParameters,
): Promise<FetchPadSharesReturnType> {
  const cleartextEthereumModule = relayerClient.runtime.ethereum as CleartextEthereumModule;
  const { padShares, signers, extraData } = await _readPadShares(relayerClient, parameters);

  const requestId = _computeRequestId(parameters, extraData);

  // KMS responses are signed under the gateway chainId, like user decryption responses.
  const domain = createKmsEip712Domain({
    chainId: relayerClient.chain.fhevm.gateway.id,
    verifyingContractAddressDecryption: relayerClient.chain.fhevm.gateway.contracts.decryption.address,
  });
  const privateKeys = getKmsSignersPrivateKeyMap(relayerClient);

  const responses: PadDecryptResponse[] = [];
  for (let i = 0; i < signers.length; ++i) {
    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
    const signerAddress = signers[i]!;
    const privateKey = privateKeys.get(signerAddress);
    if (privateKey === undefined) {
      throw new Error(`Unable to find KMS signer for address ${signerAddress}`);
    }

    // Party ids are 1-based, matching the deployed KMS node set.
    const partyId = (i + 1) as Uint32Number;
    const payload = {
      packingFactor: CLEARTEXT_PAD_PACKING_FACTOR,
      extraData,
      // Each share is the whole pad, but the client checks the context's degree (n = 3t+1).
      degree: Math.floor((signers.length - 1) / 3) as Uint32Number,
      partyId,
      padShares,
    };

    const digest = cleartextEthereumModule.hashTypedData({
      domain,
      types: kmsPadDecryptResponseEip712Types,
      primaryType: kmsPadDecryptResponseEip712PrimaryType,
      message: {
        packingFactor: payload.packingFactor,
        extraData: payload.extraData,
        degree: payload.degree,
        partyId: payload.partyId,
        pad_shares: payload.padShares.map((s) => ({ label: s.label, sessionId: s.sessionId, share: s.share })),
      },
    });
    const signature = await cleartextEthereumModule.sign({ hash: digest, privateKey });

    responses.push(
      Object.freeze({
        requestId,
        partyId,
        encrypted: encodeCleartextPadDecryptResponse(cleartextEthereumModule, payload, signature),
      }),
    );
  }

  return Object.freeze(responses);
}

async function _readPadShares(
  relayerClient: RelayerClientWithRuntime,
  parameters: FetchPadSharesParameters,
): Promise<{
  readonly padShares: readonly PadShare[];
  readonly signers: readonly ChecksummedAddress[];
  readonly extraData: BytesHex;
}> {
  const { payload } = parameters;
  const message = payload.kmsDecryptEip712Message;
  const trustedClient = getTrustedClient(relayerClient);

  const res = (await relayerClient.runtime.ethereum.readContract(trustedClient, {
    abi: getPadSharesAbi,
    address: relayerClient.chain.fhevm.contracts.inputVerifier.address as ChecksummedAddress,
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
    functionName: getPadSharesAbi[0].name,
  })) as unknown[];

  // Tuple entries are read by name: viem returns plain objects, ethers returns
  // `Result`s, and both expose the component names.
  const rawPadShares = res[0] as ReadonlyArray<{
    readonly label: unknown;
    readonly sessionId: unknown;
    readonly share: unknown;
  }>;
  const padShares: PadShare[] = Array.from(rawPadShares, (item, i) => {
    const label: unknown = item.label;
    const sessionId: unknown = item.sessionId;
    const share: unknown = item.share;
    assertIsBytes32Hex(label, { subject: `padShares[${i}].label` });
    assertIsBytesHex(sessionId, { subject: `padShares[${i}].sessionId` });
    assertIsBytesHex(share, { subject: `padShares[${i}].share` });
    return Object.freeze({ label, sessionId, share });
  });

  const signers = Array.from(res[1] as readonly string[], (a) => addressToChecksummedAddress(a as ChecksummedAddress));
  // res[2] is the context's user-decryption threshold: unused, every node answers.
  const extraData: unknown = res[3];
  assertIsBytesHex(extraData, { subject: 'extraData' });

  return { padShares, signers, extraData };
}

/** Deterministic request id: keccak256 over the request's labels, extraData and signature. */
function _computeRequestId(parameters: FetchPadSharesParameters, extraData: BytesHex): Bytes32Hex {
  const parts: BytesHex[] = [
    ...parameters.payload.witnesses.map((w) => w.label),
    extraData,
    parameters.payload.kmsDecryptEip712Signature,
  ];
  return bytes32ToHex(keccak_256(hexToBytes(concatBytesHex(parts))));
}

////////////////////////////////////////////////////////////////////////////////

function _parseUintBigIntString(label: string, value: string): bigint {
  const bn = tryParseUintBigIntString(value);
  if (bn === undefined) {
    throw new Error(`${label} is not a valid uint string, got ${JSON.stringify(value)}`);
  }
  return bn;
}
