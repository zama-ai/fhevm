import type {
  Bytes32Hex,
  BytesHex,
  ChecksummedAddress,
  Uint16Number,
  Uint32Number,
  Uint64BigInt,
} from '../../types/primitives.js';
import type { FhevmRuntime } from '../../types/coreFhevmRuntime.js';
import type { PadDecryptResponse, PadEntry } from '../relayer/types.js';
import type { PadDecryptResponsePayload, PadShare } from './types.js';
import { describe, expect, it } from 'vitest';
import { bytesToHex, hexToBytes, recoverTypedDataAddress } from 'viem';
import { privateKeyToAccount } from 'viem/accounts';
import { encryptWithPadModule } from './mock.js';
import { decode, encode } from '../../../viem/internal/ethereum.js';
import { decode as ethersDecode, encode as ethersEncode } from '../../../ethers/internal/ethereum.js';
import { createKmsEip712Domain } from '../../kms/createKmsEip712Domain.js';
import {
  createKmsPadDecryptionEip712Domain,
  kmsPadDecryptResponseEip712PrimaryType,
  kmsPadDecryptResponseEip712Types,
  kmsPadShareEip712PrimaryType,
} from '../../kms/kmsPadDecryptResponseEip712Types.js';

////////////////////////////////////////////////////////////////////////////////
// npx vitest run --config src/vitest.config.ts src/core/modules/encryptWithPad/mock.test.ts
//
// RFC-030 response processing (tech-spec#547, #564) against the cleartext encoding: every node signs each of its
// shares together with the payload's extraData (PadShareVerification) and its response
// (PadDecryptResponseVerification), both under the `fhevm-pad-decryption` domain anchored on ProtocolConfig; a
// cleartext share's rawShares is a version byte followed by the whole pad, and the payload is XOR-masked with the
// first 32 bytes of the transport public key.
////////////////////////////////////////////////////////////////////////////////

/** ProtocolConfig's chain, which anchors the response domain. */
const PROTOCOL_CONFIG_CHAIN_ID = 31337n as Uint64BigInt;
const OTHER_CHAIN_ID = 54321n as Uint64BigInt;
/** 4 KMS signers: n = 3t+1 with t = 1. */
const MPC_THRESHOLD = 1;
const PROTOCOL_CONFIG_ADDRESS = '0x5ffdaAB0373E62E2ea2944776209aEf29E631A64' as ChecksummedAddress;
const DOMAIN = createKmsPadDecryptionEip712Domain({
  protocolConfigChainId: PROTOCOL_CONFIG_CHAIN_ID,
  protocolConfigAddress: PROTOCOL_CONFIG_ADDRESS,
});
/** extraData v2: 0x02 ‖ contextId ‖ epochId. */
function extraData(contextId: number, epochId: number): BytesHex {
  const word = (n: number) => n.toString(16).padStart(64, '0');
  return `0x02${word(contextId)}${word(epochId)}` as BytesHex;
}
const EXTRA_DATA = extraData(1, 1);
const TRANSPORT_PUBLIC_KEY = `0x${'a5'.repeat(64)}` as BytesHex;
const REQUEST_ID = `0x${'ee'.repeat(32)}` as Bytes32Hex;
/** Bytes per cleartext PRF block (one keccak256 output). */
const PAD_BLOCK_BYTES = 32;

const KMS_KEYS = [
  `0x${'01'.repeat(32)}`,
  `0x${'02'.repeat(32)}`,
  `0x${'03'.repeat(32)}`,
  `0x${'04'.repeat(32)}`,
] as const;
const KMS_ACCOUNTS = KMS_KEYS.map((k) => privateKeyToAccount(k));
const KMS_SIGNERS = KMS_ACCOUNTS.map((a) => a.address as ChecksummedAddress);

const ethereum = {
  decode,
  recoverTypedDataAddress: async (parameters: Parameters<typeof recoverTypedDataAddress>[0]) =>
    (await recoverTypedDataAddress(parameters)) as ChecksummedAddress,
};

function padEntry(byte: string, padLength: number): PadEntry {
  return {
    label: `0x${byte.repeat(32)}` as Bytes32Hex,
    witness: {
      submitter: KMS_SIGNERS[0]!,
      consumer: KMS_SIGNERS[1]!,
      expirationTimestamp: 0n as Uint64BigInt,
      decryptors: [],
      nonce: `0x${'00'.repeat(16)}` as BytesHex,
    },
    padLength: padLength as Uint16Number,
  };
}

/** The expected plaintext pad bytes of `w`: blocks filled with `fill`, truncated to `padLength` bits. */
function expectedPad(w: PadEntry, fill: number): Uint8Array {
  const numBytes = Math.ceil(w.padLength / 8);
  const pad = new Uint8Array(numBytes).fill(fill);
  const extraBits = numBytes * 8 - w.padLength;
  if (extraBits !== 0) {
    pad[numBytes - 1]! &= 0xff >> extraBits;
  }
  return pad;
}

/** Cleartext rawShares of `w`: the version byte, then ⌈padLength/256⌉ blocks of `fill`. */
function rawShares(w: PadEntry, fill: number, { extraBlocks = 0, version = 0 } = {}): BytesHex {
  const numBlocks = Math.ceil(w.padLength / (8 * PAD_BLOCK_BYTES)) + extraBlocks;
  const byte = (n: number) => n.toString(16).padStart(2, '0');
  return `0x${byte(version)}${byte(fill).repeat(numBlocks * PAD_BLOCK_BYTES)}` as BytesHex;
}

function session(byte: string): BytesHex {
  return `0x${byte.repeat(16)}` as BytesHex;
}

type ShareSpec = { readonly label: Bytes32Hex; readonly sessionId: BytesHex; readonly rawShares: BytesHex };

type ResponseOptions = {
  /** The EIP-712 domain every signature of the response is made under. */
  readonly domain?: Parameters<(typeof KMS_ACCOUNTS)[number]['signTypedData']>[0]['domain'];
  readonly degree?: number;
  /** The payload's `extraData`, which every share signature covers. */
  readonly extraData?: BytesHex;
  /** The `extraData` the shares are signed over (default: the payload's). */
  readonly signedExtraData?: BytesHex;
  /** The party id the shares claim (default: the responding node's own, `partyIndex + 1`). */
  readonly partyId?: number;
  /** The node that signs the shares (default: the responding node). */
  readonly shareSignerIndex?: number;
  /** Alters the encrypted payload before the transport is signed over it. */
  readonly alterPayload?: (encryptedPayload: BytesHex) => BytesHex;
};

/**
 * Node `partyIndex`'s response: each share signed on its own with the payload's `extraData`, the payload encrypted,
 * then the transport signed.
 */
async function response(
  partyIndex: number,
  specs: readonly ShareSpec[],
  {
    domain = DOMAIN,
    degree = MPC_THRESHOLD,
    extraData: payloadExtraData = EXTRA_DATA,
    signedExtraData = payloadExtraData,
    partyId = partyIndex + 1,
    shareSignerIndex = partyIndex,
    alterPayload = (p) => p,
  }: ResponseOptions = {},
): Promise<PadDecryptResponse> {
  const shares: PadShare[] = [];
  for (const spec of specs) {
    const fields = {
      label: spec.label,
      rawShares: spec.rawShares,
      partyId: partyId as Uint32Number,
      degree: degree as Uint32Number,
      sessionId: spec.sessionId,
    };
    const signature = (await KMS_ACCOUNTS[shareSignerIndex]!.signTypedData({
      domain,
      types: { PadShareVerification: [...kmsPadDecryptResponseEip712Types.PadShareVerification] },
      primaryType: kmsPadShareEip712PrimaryType,
      message: { ...fields, extraData: signedExtraData },
    })) as BytesHex;
    shares.push({ ...fields, signature });
  }
  const encryptedPayload = alterPayload(cleartextEncrypt({ shares, extraData: payloadExtraData }));
  const signature = (await KMS_ACCOUNTS[partyIndex]!.signTypedData({
    domain,
    types: { PadDecryptResponseVerification: [...kmsPadDecryptResponseEip712Types.PadDecryptResponseVerification] },
    primaryType: kmsPadDecryptResponseEip712PrimaryType,
    message: { requestId: REQUEST_ID, encryptedPayload },
  })) as BytesHex;
  return { requestId: REQUEST_ID, encryptedPayload, signature };
}

// The mock module only reads `runtime.ethereum` in `reconstructPads`.
const PAD_DECRYPT_RESPONSE_PAYLOAD_TYPE = '((bytes32,bytes,uint32,uint32,bytes16,bytes)[],bytes)';

/** `payload` as the positional values of `PAD_DECRYPT_RESPONSE_PAYLOAD_TYPE`. */
function payloadValues(payload: PadDecryptResponsePayload): unknown[] {
  return [
    [
      payload.shares.map((s) => [s.label, s.rawShares, s.partyId, s.degree, s.sessionId, s.signature]),
      payload.extraData,
    ],
  ];
}

/** The contract's cleartext encryption: `abi.encode(payload)` XOR-masked with the transport key's first 32 bytes. */
function cleartextEncrypt(payload: PadDecryptResponsePayload): BytesHex {
  const encoded = hexToBytes(encode({ types: [PAD_DECRYPT_RESPONSE_PAYLOAD_TYPE], values: payloadValues(payload) }));
  const mask = hexToBytes(TRANSPORT_PUBLIC_KEY);
  // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
  return bytesToHex(encoded.map((b, i) => b ^ mask[i % 32]!)) as BytesHex;
}

const { encryptWithPad } = encryptWithPadModule({ ethereum } as unknown as FhevmRuntime);

function run(
  responses: readonly PadDecryptResponse[],
  pads: readonly PadEntry[],
  { module = encryptWithPad, transportPublicKey = TRANSPORT_PUBLIC_KEY } = {},
) {
  return module.reconstructPads({
    responses,
    pads,
    extraData: EXTRA_DATA,
    transportPublicKey,
    kmsSigners: KMS_SIGNERS,
    mpcThreshold: MPC_THRESHOLD,
    protocolConfigChainId: PROTOCOL_CONFIG_CHAIN_ID,
    protocolConfigAddress: PROTOCOL_CONFIG_ADDRESS,
  });
}

////////////////////////////////////////////////////////////////////////////////

describe('reconstructPads', () => {
  const a = padEntry('aa', 300); // 2 blocks, 38 bytes, 4 trailing bits cleared
  const b = padEntry('bb', 8); // 1 block, 1 byte
  const shareOf = (w: PadEntry, fill: number, sid = '11', opts = {}): ShareSpec => ({
    label: w.label,
    sessionId: session(sid),
    rawShares: rawShares(w, fill, opts),
  });

  it('reconstructs every label from 2t+1 agreeing responses', async () => {
    const responses = await Promise.all([0, 1, 2].map((i) => response(i, [shareOf(a, 0x3c), shareOf(b, 0x5a, '22')])));
    const pads = await run(responses, [a, b]);
    expect(pads.get(a.label)).toEqual(expectedPad(a, 0x3c));
    expect(pads.get(b.label)).toEqual(expectedPad(b, 0x5a));
  });

  it('selects shares per label: a dissenting session only drops that label', async () => {
    // Party 1 names another session for `a` only. `a` must use parties {0,2,3}; `b` may use all.
    const responses = await Promise.all(
      [0, 1, 2, 3].map((i) =>
        response(i, [shareOf(a, i === 1 ? 0x00 : 0x3c, i === 1 ? '99' : '11'), shareOf(b, 0x5a, '22')]),
      ),
    );
    const pads = await run(responses, [a, b]);
    expect(pads.get(a.label)).toEqual(expectedPad(a, 0x3c));
    expect(pads.get(b.label)).toEqual(expectedPad(b, 0x5a));
  });

  it('reconstructs a label when one validly signed party sends divergent rawShares in the same session', async () => {
    // Party 0 signs its share correctly but with other pad bytes, same session and version: the 3 honest parties
    // {1,2,3} still reach 2t+1 and must provide the pad.
    const responses = await Promise.all([0, 1, 2, 3].map((i) => response(i, [shareOf(a, i === 0 ? 0x00 : 0x3c)])));
    const pads = await run(responses, [a]);
    expect(pads.get(a.label)).toEqual(expectedPad(a, 0x3c));
  });

  it('fails a label when fewer than 2t+1 shares agree on its session', async () => {
    const responses = await Promise.all([0, 1, 2].map((i) => response(i, [shareOf(a, 0x3c, i === 1 ? '99' : '11')])));
    await expect(run(responses, [a])).rejects.toThrow(/Not enough consistent pad decryption responses for label/);
  });

  it('leaves out a response whose rawShares version is unsupported', async () => {
    const responses = await Promise.all(
      [0, 1, 2, 3].map((i) => response(i, [shareOf(a, i === 1 ? 0x00 : 0x3c, '11', { version: i === 1 ? 1 : 0 })])),
    );
    const pads = await run(responses, [a]);
    expect(pads.get(a.label)).toEqual(expectedPad(a, 0x3c));
  });

  it('rejects rawShares whose length does not match the requested pad length', async () => {
    const responses = await Promise.all(
      [0, 1, 2].map((i) => response(i, [shareOf(a, 0x3c, '11', { extraBlocks: 1 })])),
    );
    await expect(run(responses, [a])).rejects.toThrow(/is not 65 bytes/);
  });

  it('rejects a response that misses a requested label', async () => {
    const responses = await Promise.all([0, 1, 2].map((i) => response(i, [shareOf(a, 0x3c)])));
    await expect(run(responses, [a, b])).rejects.toThrow(/requested label\(s\) missing/);
  });

  it("rejects a response signed under another chain id than ProtocolConfig's", async () => {
    const domain = { ...DOMAIN, chainId: OTHER_CHAIN_ID };
    const responses = await Promise.all([0, 1, 2].map((i) => response(i, [shareOf(a, 0x3c)], { domain })));
    await expect(run(responses, [a])).rejects.toThrow(/is not a KMS signer/);
  });

  it('rejects a response signed under the KMS Decryption domain', async () => {
    const domain = createKmsEip712Domain({
      chainId: PROTOCOL_CONFIG_CHAIN_ID,
      verifyingContractAddressDecryption: PROTOCOL_CONFIG_ADDRESS,
    });
    const responses = await Promise.all([0, 1, 2].map((i) => response(i, [shareOf(a, 0x3c)], { domain })));
    await expect(run(responses, [a])).rejects.toThrow(/is not a KMS signer/);
  });

  it.each([
    ['another KMS context', extraData(2, 1)],
    ['another epoch', extraData(1, 2)],
    ['a v1 extraData', `0x01${'00'.repeat(31)}01` as BytesHex],
  ])('rejects a payload whose extraData names %s', async (_, other) => {
    const responses = await Promise.all([0, 1, 2].map((i) => response(i, [shareOf(a, 0x3c)], { extraData: other })));
    await expect(run(responses, [a])).rejects.toThrow(/does not name the request's context 1 and epoch 1/);
  });

  it('rejects a payload whose extraData is malformed', async () => {
    const responses = await Promise.all(
      [0, 1, 2].map((i) => response(i, [shareOf(a, 0x3c)], { extraData: '0x02aa' as BytesHex })),
    );
    await expect(run(responses, [a])).rejects.toThrow(/payload extraData 0x02aa is malformed/);
  });

  it("rejects a share signed over another extraData than its payload's", async () => {
    // The node signs its shares for context 2 but its payload names the request's context: the share signature
    // covers the payload's extraData, so it no longer recovers to the node.
    const responses = await Promise.all(
      [0, 1, 2].map((i) => response(i, [shareOf(a, 0x3c)], { signedExtraData: extraData(2, 1) })),
    );
    await expect(run(responses, [a])).rejects.toThrow(/not party \d's signer/);
  });

  it('rejects a share whose signer is not the one registered under its partyId', async () => {
    // Node 0 claims party 2 for its shares: they recover to node 0, not to party 2's signer.
    const responses = await Promise.all(
      [0, 1, 2].map((i) => response(i, [shareOf(a, 0x3c)], i === 0 ? { partyId: 2 } : {})),
    );
    await expect(run(responses, [a])).rejects.toThrow(/Rejected responses: #0: .*not party 2's signer/);
  });

  it('rejects a response whose shares are signed by another node than its transport', async () => {
    // Node 0 relays node 3's shares (party 4) under its own transport signature.
    const responses = await Promise.all(
      [0, 1, 2].map((i) => response(i, [shareOf(a, 0x3c)], i === 0 ? { partyId: 4, shareSignerIndex: 3 } : {})),
    );
    await expect(run(responses, [a])).rejects.toThrow(/Rejected responses: #0: .*signed by different nodes/);
  });

  it.each<[string, ResponseOptions & { extraBlocks?: number }]>([
    ['a signature under the wrong chain id', { domain: { ...DOMAIN, chainId: OTHER_CHAIN_ID } }],
    ['rawShares of the wrong length', { extraBlocks: 1 }],
    ['a payload extraData naming another context', { extraData: extraData(2, 1) }],
  ])('skips a response with %s when 2t+1 valid ones remain', async (_, bad) => {
    const responses = await Promise.all(
      [0, 1, 2, 3].map((i) => {
        const { extraBlocks = 0, ...opts } = i === 1 ? bad : {};
        return response(i, [shareOf(a, 0x3c, '11', { extraBlocks })], opts);
      }),
    );
    const pads = await run(responses, [a]);
    expect(pads.get(a.label)).toEqual(expectedPad(a, 0x3c));
  });

  it('reports the rejected responses when fewer than 2t+1 valid ones remain', async () => {
    const responses = await Promise.all(
      [0, 1, 2].map((i) =>
        response(i, [shareOf(a, 0x3c)], i === 1 ? { domain: { ...DOMAIN, chainId: OTHER_CHAIN_ID } } : {}),
      ),
    );
    await expect(run(responses, [a])).rejects.toThrow(
      /got 2 valid of 3, .* Rejected responses: #1: .*is not a KMS signer/,
    );
  });

  it('rejects a response signed with a degree other than the context MPC threshold', async () => {
    // A single node claiming degree 0 must not reconstruct the pad alone (1 >= 2*0+1).
    const responses = [await response(0, [shareOf(a, 0x00, '66')], { degree: 0 })];
    await expect(run(responses, [a])).rejects.toThrow(/degree 0 is not the context's MPC threshold 1/);
  });

  it('requires 2t+1 shares from the context threshold, not from the signed degree', async () => {
    // A malicious party 0 signs degree 0 first; the 3 honest parties still provide the pad.
    const responses = await Promise.all(
      [0, 1, 2, 3].map((i) =>
        i === 0 ? response(i, [shareOf(a, 0x00, '66')], { degree: 0 }) : response(i, [shareOf(a, 0x3c)]),
      ),
    );
    const pads = await run(responses, [a]);
    expect(pads.get(a.label)).toEqual(expectedPad(a, 0x3c));
  });
});

////////////////////////////////////////////////////////////////////////////////
// Cleartext payload decoding
//
// `encryptedPayload` is `abi.encode(PadDecryptResponsePayload)` XOR-masked with the transport key: the reference
// encoding below is Solidity's (`cast abi-encode`), which CleartextPaddedInputter.getCleartextPadResponsesMaterial
// masks. The mock decodes it with the runtime's ethereum module, so both adapters are exercised.
////////////////////////////////////////////////////////////////////////////////

const SAMPLE: PadDecryptResponsePayload = {
  shares: [
    {
      label: `0x${'11'.repeat(32)}` as Bytes32Hex,
      rawShares: '0x00aabbcc' as BytesHex,
      partyId: 1 as Uint32Number,
      degree: 1 as Uint32Number,
      sessionId: `0x${'22'.repeat(16)}` as BytesHex,
      signature:
        '0xababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababab' as BytesHex,
    },
    {
      label: `0x${'33'.repeat(32)}` as Bytes32Hex,
      rawShares: '0x' as BytesHex,
      partyId: 2 as Uint32Number,
      degree: 1 as Uint32Number,
      sessionId: `0x${'44'.repeat(16)}` as BytesHex,
      signature: '0x' as BytesHex,
    },
  ],
  extraData: EXTRA_DATA,
};

/** `cast abi-encode "f(((bytes32,bytes,uint32,uint32,bytes16,bytes)[],bytes))" <SAMPLE>`. */
const SAMPLE_ENCODING =
  '0x0000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000000000000000003200000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000000000000000001c0111111111111111111111111111111111111111111111111111111111111111100000000000000000000000000000000000000000000000000000000000000c00000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000122222222222222222222222222222222000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000000400aabbcc000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000041ababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababab00000000000000000000000000000000000000000000000000000000000000333333333333333333333333333333333333333333333333333333333333333300000000000000000000000000000000000000000000000000000000000000c000000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000001444444444444444444444444444444440000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000e0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000041020000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000' as BytesHex;

const ADAPTERS = [
  ['viem', { decode, encode }],
  ['ethers', { decode: ethersDecode, encode: ethersEncode }],
] as const;

describe.each(ADAPTERS)('cleartext payload decoding (%s)', (_name, adapter) => {
  const { encryptWithPad: module } = encryptWithPadModule({
    ethereum: { ...ethereum, decode: adapter.decode },
  } as unknown as FhevmRuntime);
  const a = padEntry('aa', 300);
  const shareOf = (w: PadEntry): ShareSpec => ({
    label: w.label,
    sessionId: session('11'),
    rawShares: rawShares(w, 0x3c),
  });

  it('encodes the sample to the Solidity reference encoding', () => {
    expect(adapter.encode({ types: [PAD_DECRYPT_RESPONSE_PAYLOAD_TYPE], values: payloadValues(SAMPLE) })).toBe(
      SAMPLE_ENCODING,
    );
  });

  it('decrypts and decodes every response', async () => {
    const responses = await Promise.all([0, 1, 2].map((i) => response(i, [shareOf(a)])));
    const pads = await run(responses, [a], { module });
    expect(pads.get(a.label)).toEqual(expectedPad(a, 0x3c));
  });

  it('rejects a truncated payload', async () => {
    const truncate = (p: BytesHex): BytesHex => p.slice(0, 402) as BytesHex;
    const responses = await Promise.all([0, 1, 2].map((i) => response(i, [shareOf(a)], { alterPayload: truncate })));
    await expect(run(responses, [a], { module })).rejects.toThrow(/does not decrypt/);
  });

  it('rejects a transport key shorter than 32 bytes', async () => {
    const responses = await Promise.all([0, 1, 2].map((i) => response(i, [shareOf(a)])));
    await expect(run(responses, [a], { module, transportPublicKey: '0x00' as BytesHex })).rejects.toThrow(
      /shorter than 32 bytes/,
    );
  });
});
