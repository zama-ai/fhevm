import type {
  Bytes32Hex,
  BytesHex,
  ChecksummedAddress,
  Uint16Number,
  Uint32Number,
  Uint64BigInt,
} from '../../types/primitives.js';
import type { FhevmRuntime } from '../../types/coreFhevmRuntime.js';
import type { PadDecryptResponse, PadShare, PadWitness } from '../relayer/types.js';
import { describe, expect, it } from 'vitest';
import { decodeAbiParameters, encodeAbiParameters, parseAbiParameters, recoverTypedDataAddress } from 'viem';
import { privateKeyToAccount } from 'viem/accounts';
import { encryptTransciModule } from './mock.js';
import { createKmsEip712Domain } from '../../kms/createKmsEip712Domain.js';
import {
  kmsPadDecryptResponseEip712PrimaryType,
  kmsPadDecryptResponseEip712Types,
} from '../../kms/kmsPadDecryptResponseEip712Types.js';
import {
  CLEARTEXT_PAD_BLOCK_BYTES,
  CLEARTEXT_PAD_PACKING_FACTOR,
  encodeCleartextPadDecryptResponse,
  type CleartextPadDecryptResponsePayload,
} from '../relayer/cleartext/padDecryptResponse.js';

////////////////////////////////////////////////////////////////////////////////
// npx vitest run --config src/vitest.config.ts src/core/modules/encryptTransci/mock.test.ts
//
// RFC-030 response processing against the cleartext share encoding: every
// share is the whole encrypted pad (one 32-byte block per ring element),
// XOR-masked with the first 32 bytes of the transport public key.
////////////////////////////////////////////////////////////////////////////////

const HOST_CHAIN_ID = 31337n as Uint64BigInt;
const GATEWAY_CHAIN_ID = 54321n as Uint64BigInt;
/** 4 KMS signers: n = 3t+1 with t = 1. */
const MPC_THRESHOLD = 1;
const DECRYPTION_ADDRESS = '0x5ffdaAB0373E62E2ea2944776209aEf29E631A64' as ChecksummedAddress;
const EXTRA_DATA = `0x02${'00'.repeat(31)}01${'00'.repeat(31)}01` as BytesHex;
const TRANSPORT_PUBLIC_KEY = `0x${'a5'.repeat(64)}` as BytesHex;

const KMS_KEYS = [
  `0x${'01'.repeat(32)}`,
  `0x${'02'.repeat(32)}`,
  `0x${'03'.repeat(32)}`,
  `0x${'04'.repeat(32)}`,
] as const;
const KMS_ACCOUNTS = KMS_KEYS.map((k) => privateKeyToAccount(k));
const KMS_SIGNERS = KMS_ACCOUNTS.map((a) => a.address as ChecksummedAddress);

const ethereum = {
  encode: ({ types, values }: { types: readonly string[]; values: readonly unknown[] }) =>
    encodeAbiParameters(parseAbiParameters(types.join(',')), values as never) as BytesHex,
  decode: ({ types, encodedData }: { types: readonly string[]; encodedData: BytesHex }) =>
    Array.from(decodeAbiParameters(parseAbiParameters(types.join(',')), encodedData)),
  recoverTypedDataAddress: async (parameters: Parameters<typeof recoverTypedDataAddress>[0]) =>
    (await recoverTypedDataAddress(parameters)) as ChecksummedAddress,
};

function witness(byte: string, padLength: number): PadWitness {
  return {
    submitter: KMS_SIGNERS[0]!,
    consumer: KMS_SIGNERS[1]!,
    expirationTimestamp: 0n as Uint64BigInt,
    decryptors: [],
    nonce: `0x${'00'.repeat(16)}` as BytesHex,
    label: `0x${byte.repeat(32)}` as Bytes32Hex,
    padLength: padLength as Uint16Number,
  };
}

/** The expected plaintext pad bytes of `w`: blocks filled with `fill`, truncated to `padLength` bits. */
function expectedPad(w: PadWitness, fill: number): Uint8Array {
  const numBytes = Math.ceil(w.padLength / 8);
  const pad = new Uint8Array(numBytes).fill(fill);
  const extraBits = numBytes * 8 - w.padLength;
  if (extraBits !== 0) {
    pad[numBytes - 1]! &= 0xff >> extraBits;
  }
  return pad;
}

/** Cleartext share of `w`: ⌈padLength/256⌉ blocks of `fill`, masked with the transport key. */
function maskedShare(w: PadWitness, fill: number, extraBlocks = 0): BytesHex {
  const numBlocks = Math.ceil(w.padLength / (8 * CLEARTEXT_PAD_BLOCK_BYTES)) + extraBlocks;
  const mask = Number.parseInt(TRANSPORT_PUBLIC_KEY.slice(2, 4), 16);
  return `0x${(fill ^ mask)
    .toString(16)
    .padStart(2, '0')
    .repeat(numBlocks * CLEARTEXT_PAD_BLOCK_BYTES)}` as BytesHex;
}

async function response(
  partyIndex: number,
  padShares: readonly PadShare[],
  {
    packingFactor = CLEARTEXT_PAD_PACKING_FACTOR,
    chainId = GATEWAY_CHAIN_ID,
    degree = MPC_THRESHOLD,
  }: { packingFactor?: number; chainId?: Uint64BigInt; degree?: number } = {},
): Promise<PadDecryptResponse> {
  const payload: CleartextPadDecryptResponsePayload = {
    packingFactor: packingFactor as Uint32Number,
    extraData: EXTRA_DATA,
    degree: degree as Uint32Number,
    partyId: (partyIndex + 1) as Uint32Number,
    padShares,
  };
  const signature = await KMS_ACCOUNTS[partyIndex]!.signTypedData({
    domain: createKmsEip712Domain({ chainId, verifyingContractAddressDecryption: DECRYPTION_ADDRESS }),
    types: {
      PadDecryptResponsePayload: [...kmsPadDecryptResponseEip712Types.PadDecryptResponsePayload],
      PadShare: [...kmsPadDecryptResponseEip712Types.PadShare],
    },
    primaryType: kmsPadDecryptResponseEip712PrimaryType,
    message: {
      packingFactor: payload.packingFactor,
      extraData: payload.extraData,
      degree: payload.degree,
      partyId: payload.partyId,
      pad_shares: padShares.map((s) => ({ label: s.label, sessionId: s.sessionId, share: s.share })),
    },
  });
  return {
    requestId: `0x${'ee'.repeat(32)}` as Bytes32Hex,
    partyId: payload.partyId,
    encrypted: encodeCleartextPadDecryptResponse(ethereum, payload, signature as never),
  };
}

function session(byte: string): BytesHex {
  return `0x${byte.repeat(16)}` as BytesHex;
}

// The mock module only reads `runtime.ethereum` in `reconstructPads`.
const { encryptTransci } = encryptTransciModule({ ethereum } as unknown as FhevmRuntime);

function run(responses: readonly PadDecryptResponse[], witnesses: readonly PadWitness[]) {
  return encryptTransci.reconstructPads({
    responses,
    witnesses,
    extraData: EXTRA_DATA,
    transportPublicKey: TRANSPORT_PUBLIC_KEY,
    kmsSigners: KMS_SIGNERS,
    mpcThreshold: MPC_THRESHOLD,
    gatewayChainId: GATEWAY_CHAIN_ID,
    verifyingContractAddressDecryption: DECRYPTION_ADDRESS,
  });
}

////////////////////////////////////////////////////////////////////////////////

describe('reconstructPads', () => {
  const a = witness('aa', 300); // 2 blocks, 38 bytes, 4 trailing bits cleared
  const b = witness('bb', 8); // 1 block, 1 byte

  it('reconstructs every label from 2t+1 agreeing responses', async () => {
    const responses = await Promise.all(
      [0, 1, 2].map((i) =>
        response(i, [
          { label: a.label, sessionId: session('11'), share: maskedShare(a, 0x3c) },
          { label: b.label, sessionId: session('22'), share: maskedShare(b, 0x5a) },
        ]),
      ),
    );
    const pads = await run(responses, [a, b]);
    expect(pads.get(a.label)).toEqual(expectedPad(a, 0x3c));
    expect(pads.get(b.label)).toEqual(expectedPad(b, 0x5a));
  });

  it('selects responses per label: a dissenting session only drops that label', async () => {
    // Party 1 names another session for `a` only. `a` must use parties {0,2,3}; `b` may use all.
    const responses = await Promise.all(
      [0, 1, 2, 3].map((i) =>
        response(i, [
          { label: a.label, sessionId: session(i === 1 ? '99' : '11'), share: maskedShare(a, i === 1 ? 0x00 : 0x3c) },
          { label: b.label, sessionId: session('22'), share: maskedShare(b, 0x5a) },
        ]),
      ),
    );
    const pads = await run(responses, [a, b]);
    expect(pads.get(a.label)).toEqual(expectedPad(a, 0x3c));
    expect(pads.get(b.label)).toEqual(expectedPad(b, 0x5a));
  });

  it('fails a label when fewer than 2t+1 responses agree on its session', async () => {
    const responses = await Promise.all(
      [0, 1, 2].map((i) =>
        response(i, [{ label: a.label, sessionId: session(i === 1 ? '99' : '11'), share: maskedShare(a, 0x3c) }]),
      ),
    );
    await expect(run(responses, [a])).rejects.toThrow(/Not enough consistent pad decryption responses for label/);
  });

  it('leaves out a response whose packing factor dissents', async () => {
    // Party 1 claims packingFactor 2 (its share is padded to whole 2-block elements).
    const responses = await Promise.all(
      [0, 1, 2, 3].map((i) =>
        i === 1
          ? response(i, [{ label: a.label, sessionId: session('11'), share: maskedShare(a, 0x00) }], {
              packingFactor: 2,
            })
          : response(i, [{ label: a.label, sessionId: session('11'), share: maskedShare(a, 0x3c) }]),
      ),
    );
    const pads = await run(responses, [a]);
    expect(pads.get(a.label)).toEqual(expectedPad(a, 0x3c));
  });

  it('rejects a share whose length does not match the packing factor', async () => {
    const responses = await Promise.all(
      [0, 1, 2].map((i) => response(i, [{ label: a.label, sessionId: session('11'), share: maskedShare(a, 0x3c, 1) }])),
    );
    await expect(run(responses, [a])).rejects.toThrow(/is not 64 bytes/);
  });

  it('rejects a response that misses a requested label', async () => {
    const responses = await Promise.all(
      [0, 1, 2].map((i) => response(i, [{ label: a.label, sessionId: session('11'), share: maskedShare(a, 0x3c) }])),
    );
    await expect(run(responses, [a, b])).rejects.toThrow(/requested label\(s\) missing/);
  });

  it('rejects a response signed under the host chain id instead of the gateway chain id', async () => {
    const responses = await Promise.all(
      [0, 1, 2].map((i) =>
        response(i, [{ label: a.label, sessionId: session('11'), share: maskedShare(a, 0x3c) }], {
          chainId: HOST_CHAIN_ID,
        }),
      ),
    );
    await expect(run(responses, [a])).rejects.toThrow(/is not a KMS signer/);
  });

  it.each<[string, { chainId?: Uint64BigInt; extraBlocks?: number }]>([
    ['a signature under the wrong chain id', { chainId: HOST_CHAIN_ID }],
    ['a share of the wrong length', { extraBlocks: 1 }],
  ])('skips a response with %s when 2t+1 valid ones remain', async (_, bad) => {
    const responses = await Promise.all(
      [0, 1, 2, 3].map((i) => {
        const { chainId = GATEWAY_CHAIN_ID, extraBlocks = 0 } = i === 1 ? bad : {};
        return response(i, [{ label: a.label, sessionId: session('11'), share: maskedShare(a, 0x3c, extraBlocks) }], {
          chainId,
        });
      }),
    );
    const pads = await run(responses, [a]);
    expect(pads.get(a.label)).toEqual(expectedPad(a, 0x3c));
  });

  it('reports the rejected responses when fewer than 2t+1 valid ones remain', async () => {
    const responses = await Promise.all(
      [0, 1, 2].map((i) =>
        response(i, [{ label: a.label, sessionId: session('11'), share: maskedShare(a, 0x3c) }], {
          chainId: i === 1 ? HOST_CHAIN_ID : GATEWAY_CHAIN_ID,
        }),
      ),
    );
    await expect(run(responses, [a])).rejects.toThrow(
      /got 2 valid of 3, .* Rejected responses: #1 \(partyId hint 2\): .*is not a KMS signer/,
    );
  });

  it('rejects a response signed with a degree other than the context MPC threshold', async () => {
    // A single node claiming degree 0 must not reconstruct the pad alone (1 >= 2*0+1).
    const responses = [
      await response(0, [{ label: a.label, sessionId: session('66'), share: maskedShare(a, 0x00) }], { degree: 0 }),
    ];
    await expect(run(responses, [a])).rejects.toThrow(/degree 0 is not the context's MPC threshold 1/);
  });

  it('requires 2t+1 responses from the context threshold, not from the signed degree', async () => {
    // A malicious party 0 signs degree 0 first; the 3 honest parties still provide the pad.
    const responses = await Promise.all(
      [0, 1, 2, 3].map((i) =>
        i === 0
          ? response(i, [{ label: a.label, sessionId: session('66'), share: maskedShare(a, 0x00) }], { degree: 0 })
          : response(i, [{ label: a.label, sessionId: session('11'), share: maskedShare(a, 0x3c) }]),
      ),
    );
    const pads = await run(responses, [a]);
    expect(pads.get(a.label)).toEqual(expectedPad(a, 0x3c));
  });
});
