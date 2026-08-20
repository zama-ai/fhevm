// The linker vectors, and what a response has to match to be this request's.
//
// `solana/test-fixtures/user-decrypt/solana_linker_v1.json` is generated on the KMS side and travels
// with the blob that computes links here — the regeneration script copies both from one kms commit, so
// a blob checked against another commit's vectors is not a state this tree can reach. The set carries
// its own digest, and the digest is checked here: the same cross-repository contract the KMS-side
// suites assert.
//
// The records do the work. Four accepted ones pin the link this SDK computes against the link the KMS
// computes. Ten divergence records each mutate one bound field and carry the different link that
// mutation produces — a response signed over the reference link is refused for every one of them. Nine
// construction rejects are inputs the checked construction refuses outright. And two foreign-scheme
// records carry links from another scheme version, which this one must never reproduce.

import type { BytesHex } from '../../core/types/primitives.js';
import type { SolanaUserDecryptLinkInputs } from './index.js';
import { readFileSync } from 'node:fs';
import { secp256k1 } from '@noble/curves/secp256k1.js';
import { sha256 } from '@noble/hashes/sha2.js';
import { hashTypedData } from 'viem';
import { privateKeyToAccount } from 'viem/accounts';
import { describe, expect, it } from 'vitest';
import { toChecksummedAddress } from '../../core/base/address.js';
import { sign } from '../../core/base/sign.js';
import {
  generateSolanaTransportKeyPair,
  solanaUserDecryptLink,
  solanaUserDecryptRequestHalf,
  verifySolanaUserDecryptPlaintexts,
  verifySolanaUserDecryptResponse,
} from './index.js';
import { bytesToHex, hexToBytes } from '../proof.js';

/* eslint-disable @typescript-eslint/naming-convention -- the fixture's own field names are snake_case */

interface LinkerRecord {
  readonly name: string;
  readonly comment: string;
  readonly result: 'valid' | 'invalid';
  readonly class: 'valid' | 'link-divergence' | 'construction-reject' | 'foreign-scheme-link';
  readonly rule?: string;
  readonly rejected_by?: string;
  readonly chain_id_decimal: string;
  readonly declared_chain_id_decimal?: string;
  readonly receiver_id: string;
  readonly verifying_program_id: string;
  readonly kms_context_id: string;
  readonly kms_epoch_id: string;
  readonly handles: readonly string[];
  readonly transport_key: string;
  readonly scheme_tag: string;
  readonly link?: string;
}

interface LinkerFixture {
  readonly schema: string;
  readonly set_digest_file: string;
  readonly scheme_tag: string;
  readonly dsep: string;
  readonly transport_keys: Readonly<Record<string, string>>;
  readonly records: readonly LinkerRecord[];
}

/* eslint-enable @typescript-eslint/naming-convention */

const FIXTURE_DIR = new URL('../../../../../solana/test-fixtures/user-decrypt/', import.meta.url);
const VECTOR_FILE = 'solana_linker_v1.json';

const fixtureBytes = readFileSync(new URL(VECTOR_FILE, FIXTURE_DIR));
const fixture = JSON.parse(fixtureBytes.toString('utf8')) as LinkerFixture;

/** The inputs one record stands for, in the form the SDK computes a link from. */
function inputsOf(record: LinkerRecord): SolanaUserDecryptLinkInputs {
  const transportKey = fixture.transport_keys[record.transport_key];
  if (transportKey === undefined) {
    throw new Error(`${record.name}: transport key ${record.transport_key} is not in the file's table`);
  }
  return {
    userPubkey: hexToBytes(`0x${record.receiver_id}`),
    // A record that declares a chain id different from the one its handles embed is testing exactly
    // that disagreement, so the declared one is what goes in.
    hostChainId: BigInt(record.declared_chain_id_decimal ?? record.chain_id_decimal),
    verifyingProgramId: hexToBytes(`0x${record.verifying_program_id}`),
    kmsContextId: hexToBytes(`0x${record.kms_context_id}`),
    kmsEpochId: hexToBytes(`0x${record.kms_epoch_id}`),
    handles: record.handles.map((handle) => hexToBytes(`0x${handle}`)),
    transportKey: hexToBytes(`0x${transportKey}`),
  };
}

const byClass = (className: LinkerRecord['class']): readonly LinkerRecord[] =>
  fixture.records.filter((record) => record.class === className);

const named = (records: readonly LinkerRecord[]): ReadonlyArray<readonly [string, LinkerRecord]> =>
  records.map((record) => [record.name, record] as const);

const reference = fixture.records.find((record) => record.name === 'reference-two-handles');
if (reference === undefined) {
  throw new Error('the linker vector set carries no reference-two-handles record');
}

////////////////////////////////////////////////////////////////////////////////

describe('the committed linker vector set', () => {
  // The digest is the cross-repository contract: this file was produced by the kms commit the blob
  // beside it was built from, and a set edited in transit stops matching.
  it('matches the digest it ships with', () => {
    const digestFile = readFileSync(new URL(fixture.set_digest_file, FIXTURE_DIR), 'utf8');
    const [declared, name] = digestFile.trim().split(/\s+/);
    expect(name).toBe(VECTOR_FILE);
    expect(bytesToHex(sha256(fixtureBytes)).slice(2)).toBe(declared);
  });

  it('is read under the schema and scheme it declares', () => {
    expect(fixture.schema).toBe('zama-solana-linker-vectors/v1');
    expect(fixture.scheme_tag).toBe('SolanaUserDecryptionLinker:v1');
    expect(fixture.dsep).toBe('SOLLNK01');
  });

  it('covers every class the response verification has to handle', () => {
    for (const className of ['valid', 'link-divergence', 'construction-reject', 'foreign-scheme-link'] as const) {
      expect(byClass(className).length, `no record of class ${className}`).toBeGreaterThan(0);
    }
    expect(new Set(fixture.records.map((record) => record.name)).size).toBe(fixture.records.length);
  });
});

describe('the link this SDK computes', () => {
  it.each(named(byClass('valid')))('%s: is the link the KMS recorded', async (_name, record) => {
    expect(bytesToHex(await solanaUserDecryptLink(inputsOf(record))).slice(2)).toBe(record.link);
  });

  // Each of these mutates one bound field. The recomputed link is the mutated one, so it is not the
  // link a response for the reference request carries — which is how a substituted response is caught,
  // for each field separately rather than for the set of them together.
  it.each(named(byClass('link-divergence')))('%s: diverges from the reference link', async (_name, record) => {
    const link = bytesToHex(await solanaUserDecryptLink(inputsOf(record))).slice(2);
    expect(link).toBe(record.link);
    expect(link).not.toBe(reference.link);
  });

  // Not a divergent link but no link at all: the construction refuses inputs that are not a request,
  // rather than hashing them into something that looks like one.
  it.each(named(byClass('construction-reject')))('%s: is refused, not computed', async (_name, record) => {
    await expect(solanaUserDecryptLink(inputsOf(record))).rejects.toThrow();
  });

  // A link from another scheme version. This one must be unable to produce it — otherwise a response
  // minted for a different version of the protocol would satisfy this version's comparison.
  it.each(named(byClass('foreign-scheme-link')))('%s: is not reachable from this scheme', async (_name, record) => {
    expect(record.scheme_tag).not.toBe(fixture.scheme_tag);
    const link = bytesToHex(await solanaUserDecryptLink(inputsOf(record))).slice(2);
    expect(link).not.toBe(record.link);
  });
});

// The request half handed to the blob: every field the client's own. The KMS route is the field
// that makes this pinnable and worth pinning — the blob compares it against the route the response
// carries before any signature is checked, so a request half that zeroes it (or copies it from the
// response) turns that comparison into nothing. The committed vectors carry no signcrypted shares,
// so no test reaches the comparison through a full verification; what is pinned instead is the
// bytes this side brings to it.
describe('the request half of the link contract', () => {
  it('carries the signed KMS route: the version byte over the link inputs, byte for byte', () => {
    const half = solanaUserDecryptRequestHalf(inputsOf(reference));
    expect(half.extra_data).toBe(`02${reference.kms_context_id}${reference.kms_epoch_id}`);
  });

  it('follows a changed route byte in the client inputs — the route is the client’s, not an echo', () => {
    const inputs = inputsOf(reference);
    const mutated = inputs.kmsContextId.map((byte, index) => (index === 0 ? byte ^ 0x01 : byte));
    const half = solanaUserDecryptRequestHalf({ ...inputs, kmsContextId: mutated });
    expect(half.extra_data).not.toBe(`02${reference.kms_context_id}${reference.kms_epoch_id}`);
    expect(half.extra_data.slice(2)).toBe(`${bytesToHex(mutated).slice(2)}${reference.kms_epoch_id}`);
  });

  it('zeroes the EVM-shaped fields and carries the handles in request order', () => {
    const half = solanaUserDecryptRequestHalf(inputsOf(reference));
    expect(half.client_address).toBe('0x0000000000000000000000000000000000000000');
    expect(half.eip712_verifying_contract).toBe('0x0000000000000000000000000000000000000000');
    expect(half.signature).toBeUndefined();
    expect(half.ciphertext_handles).toEqual(reference.handles);
  });

  it('refuses ids of a width the routing version does not admit', () => {
    const inputs = inputsOf(reference);
    expect(() => solanaUserDecryptRequestHalf({ ...inputs, kmsEpochId: new Uint8Array(31) })).toThrow();
  });
});

describe('the response verification', () => {
  const keyPair = { secretKey: {}, publicKey: {}, publicKeyBytes: new Uint8Array(869) } as never;
  const signers = [{ partyId: 1, address: '0x0000000000000000000000000000000000000001' }];

  // No shares is not an empty answer: it is a response that reaches no threshold. A verifier must
  // never turn the absence of shares into a successful decryption of nothing — "nothing was
  // decrypted" has to fail, not decode.
  it('refuses a response with no shares at all', async () => {
    await expect(
      verifySolanaUserDecryptResponse({
        link: inputsOf(reference),
        shares: [],
        keyPair,
        signers,
        fheParameter: 'test',
      }),
    ).rejects.toThrow();
  });

  // A share that does not authenticate is discarded before its link is looked at, so a response made
  // entirely of such shares reconstructs nothing — rather than reconstructing from whatever they say.
  it('refuses a response whose shares do not authenticate', async () => {
    await expect(
      verifySolanaUserDecryptResponse({
        link: inputsOf(reference),
        shares: [{ signature: '0x00', payload: '0x00', extraData: '0x' }],
        keyPair,
        signers,
        fheParameter: 'test',
      }),
    ).rejects.toThrow();
  });

  // The configured gateway domain changes what signatures verify against, never what is accepted
  // without one: an unauthenticated share is refused under a real domain exactly as it is under the
  // empty one. (A positive case needs real signcrypted shares, which arrive with the KMS vectors.)
  it('refuses an unauthenticated share under a configured gateway domain too', async () => {
    await expect(
      verifySolanaUserDecryptResponse({
        link: inputsOf(reference),
        shares: [{ signature: '0x00', payload: '0x00', extraData: '0x' }],
        keyPair,
        signers,
        fheParameter: 'test',
        gatewayEip712Domain: {
          name: 'Decryption',
          version: '1',
          chainId: 31337n,
          verifyingContract: '0x0000000000000000000000000000000000000042',
        },
      }),
    ).rejects.toThrow();
  });
});

////////////////////////////////////////////////////////////////////////////////
// The KMS route rule, through the blob
////////////////////////////////////////////////////////////////////////////////

// The one rule the committed linker vectors cannot reach: a share is authenticated against the
// message the server signed — the payload, the request's transport key, and the request's KMS
// route — and a response whose route is not the request's is refused before its signature is even
// tried. The signer set is trusted configuration, so the test registers its own node key and
// signs the share itself; what it cannot produce is a real signcryption, so the accepted share is
// caught one rule later, at unsigncryption — which is the proof the route and signature rules
// passed. The share bytes are hand-built to the wire layout (proto field order, bincode legacy:
// u64-LE length-prefixed byte fields, fixed-width little-endian ints). That layout is pinned by
// the vendored blob: if a kms bump changes it, this suite fails loudly at parsing rather than
// drifting.
//
// Unsigncryption itself is the deliberate stopping point. Carrying these cases through the
// release — a plaintext out of the accepted share, and the same route mutations against a real
// share — needs signcrypted shares only the kms repository's signcryption code can mint, i.e. a
// committed response vector set generated there and vendored beside the blob the way the linker
// set is (see regen-tkms-wasm.sh). That is kms-side work in its own PR, planned as a follow-up;
// until it lands, the release path is covered by the live e2e suite.
describe('the KMS route rule', () => {
  // Deterministic test-only node key; its address is the trusted registry entry.
  const nodePrivateKey = `0x${'2a'.repeat(32)}` as const;
  const nodeAccount = privateKeyToAccount(nodePrivateKey);
  // Letterful on purpose, so the spelling cases below are not vacuous; the checksummed form is
  // computed, not spelled, so it cannot silently be the lowercase one.
  const gatewayContract = toChecksummedAddress('0x00000000000000000000000000000000000000aa');
  if (gatewayContract === undefined) throw new Error('unreachable: a valid 20-byte hex address');
  const gatewayEip712Domain = {
    name: 'Decryption',
    version: '1',
    chainId: 31337n,
    verifyingContract: gatewayContract as `0x${string}`,
  } as const;
  /** The signed KMS route of the reference request: the version byte over its context and epoch. */
  const referenceRoute = `0x02${reference.kms_context_id}${reference.kms_epoch_id}` as `0x${string}`;

  const concat = (...parts: readonly Uint8Array[]): Uint8Array => {
    const out = new Uint8Array(parts.reduce((length, part) => length + part.length, 0));
    let offset = 0;
    for (const part of parts) {
      out.set(part, offset);
      offset += part.length;
    }
    return out;
  };
  const u64le = (value: number): Uint8Array => {
    const bytes = new Uint8Array(8);
    new DataView(bytes.buffer).setBigUint64(0, BigInt(value), true);
    return bytes;
  };
  const u32le = (value: number): Uint8Array => {
    const bytes = new Uint8Array(4);
    new DataView(bytes.buffer).setUint32(0, value, true);
    return bytes;
  };
  /** A bincode-legacy byte field: u64-LE length, then the bytes. */
  const lengthPrefixed = (bytes: Uint8Array): Uint8Array => concat(u64le(bytes.length), bytes);

  /**
   * The share's payload, to the wire layout of `UserDecryptionResponsePayload`: the node's
   * verification key (itself a length-prefixed SEC1 key, as the key type serializes), the digest,
   * one signcrypted ciphertext (type, opaque bytes, its handle, packing factor), party id, degree.
   * The digest is the reference record's committed link and the ciphertext bytes are garbage: every
   * rule up to unsigncryption holds, and unsigncryption cannot.
   */
  const payloadBytes = (): Uint8Array => {
    const sec1 = secp256k1.getPublicKey(hexToBytes(nodePrivateKey), true);
    const link = hexToBytes(`0x${reference.link}`);
    const handle = hexToBytes(`0x${reference.handles[0]}`);
    const ciphertext = concat(
      u32le(5), // fhe_type euint64, i32 little-endian
      lengthPrefixed(Uint8Array.from([1, 2, 3, 4])), // signcrypted bytes nothing can open
      lengthPrefixed(handle),
      u32le(1), // packing_factor
    );
    return concat(
      lengthPrefixed(lengthPrefixed(sec1)), // verification_key: the serialized key, as a byte field
      lengthPrefixed(link), // digest
      u64le(1), // one signcrypted ciphertext
      ciphertext,
      u32le(1), // party_id
      u32le(0), // degree: the centralized shape
    );
  };

  /**
   * A share signed by the registered node key over `route` — the very message the server builds:
   * the request's transport key, the payload's handles, the serialized payload, and the route.
   */
  const shareOver = (route: `0x${string}`): { signature: string; payload: string; extraData: string } => {
    const payload = payloadBytes();
    const hash = hashTypedData({
      domain: gatewayEip712Domain,
      types: {
        UserDecryptResponseVerification: [
          { name: 'publicKey', type: 'bytes' },
          { name: 'ctHandles', type: 'bytes32[]' },
          { name: 'userDecryptedShare', type: 'bytes' },
          { name: 'extraData', type: 'bytes' },
        ],
      },
      primaryType: 'UserDecryptResponseVerification',
      message: {
        publicKey: `0x${fixture.transport_keys[reference.transport_key]!}` as `0x${string}`,
        ctHandles: [`0x${reference.handles[0]}` as `0x${string}`],
        userDecryptedShare: bytesToHex(payload) as `0x${string}`,
        extraData: route,
      },
    });
    return {
      signature: sign({ hash: hash as BytesHex, privateKey: nodePrivateKey as BytesHex }),
      payload: bytesToHex(payload),
      extraData: route,
    };
  };

  // The spelling overrides change how the same 20 bytes are written, never which bytes: the
  // EIP-712 hash encodes the address as bytes, so a share signed under the canonical domain
  // verifies under any spelling of it — if the boundary admits the spelling at all.
  const verifyWith = async (
    link: SolanaUserDecryptLinkInputs,
    share: ReturnType<typeof shareOver>,
    spelling: { signerAddress?: string; verifyingContract?: string } = {},
  ) =>
    verifySolanaUserDecryptResponse({
      link,
      shares: [share],
      keyPair: await generateSolanaTransportKeyPair(),
      signers: [{ partyId: 1, address: spelling.signerAddress ?? nodeAccount.address }],
      fheParameter: 'test',
      gatewayEip712Domain: {
        ...gatewayEip712Domain,
        verifyingContract: spelling.verifyingContract ?? gatewayEip712Domain.verifyingContract,
      },
    });

  // The control: with the request's own route, the share authenticates and carries the link — it
  // is refused only at unsigncryption, the first rule this test cannot satisfy. This is what makes
  // the rejections below the route's doing and not garbage failing anywhere.
  it('a share signed over the request route passes the signature and link rules', async () => {
    await expect(verifyWith(inputsOf(reference), shareOver(referenceRoute))).rejects.toThrow(
      /could not unsigncrypt the response from party 1/,
    );
  });

  // The review's mutation, response side: one changed route byte, everything else untouched. The
  // share must be refused by the signature rule — the wire's route is not the one this client's
  // permit signed.
  it('one changed route byte in the response is refused by the node-signature rule', async () => {
    const mutated = `0x03${referenceRoute.slice(4)}` as `0x${string}`;
    await expect(verifyWith(inputsOf(reference), shareOver(mutated))).rejects.toThrow(
      /the KMS node signature on the response from party 1 is not valid/,
    );
  });

  // The same mutation, request side: the client's signed epoch differs by one byte, the share is
  // the untouched reference one. The verification rebuilds the route from the client's own inputs,
  // so the same rule refuses it — a response cannot bring its own route.
  it('one changed route byte in the client inputs refuses the untouched share', async () => {
    const inputs = inputsOf(reference);
    const mutatedEpoch = inputs.kmsEpochId.map((byte, index) => (index === 0 ? byte ^ 0x01 : byte));
    await expect(verifyWith({ ...inputs, kmsEpochId: mutatedEpoch }, shareOver(referenceRoute))).rejects.toThrow(
      /the KMS node signature on the response from party 1 is not valid/,
    );
  });

  // The address spelling at the same crossing. The blob's parser accepts only EIP-55 mixed case,
  // while configuration read from on-chain bytes is naturally all-lowercase — a valid address the
  // parser would refuse as "Bad address checksum". The boundary re-encodes exactly the spellings
  // that carry no checksum; a mixed-case spelling claims one, and a wrong claim must stay refused.
  // (The checksummed spelling is the control above: `nodeAccount.address` and the computed
  // `gatewayContract` are both EIP-55.)

  it('lowercase signer and gateway addresses are the same trust anchor, not a bad checksum', async () => {
    await expect(
      verifyWith(inputsOf(reference), shareOver(referenceRoute), {
        signerAddress: nodeAccount.address.toLowerCase(),
        verifyingContract: gatewayContract.toLowerCase(),
      }),
    ).rejects.toThrow(/could not unsigncrypt the response from party 1/);
  });

  it('a mixed-case signer address with a wrong checksum stays refused', async () => {
    // Toggle the case of the first letter: still the same 20 bytes, but now a false checksum claim.
    const letter = /[a-fA-F]/;
    const index = [...nodeAccount.address].findIndex((char, at) => at >= 2 && letter.test(char));
    expect(index).toBeGreaterThan(1);
    const char = nodeAccount.address[index]!;
    const flipped =
      nodeAccount.address.slice(0, index) +
      (char === char.toLowerCase() ? char.toUpperCase() : char.toLowerCase()) +
      nodeAccount.address.slice(index + 1);
    await expect(
      verifyWith(inputsOf(reference), shareOver(referenceRoute), { signerAddress: flipped }),
    ).rejects.toThrow(/checksum/i);
  });
});

////////////////////////////////////////////////////////////////////////////////
// The typed answer: plaintexts against the handles they claim to answer
////////////////////////////////////////////////////////////////////////////////

// The link binds the handles' bytes, not the payload's type field: every link rule passes when the
// KMS answers under the right link with the wrong type. This check is pinned directly because the
// committed vectors carry no signcrypted shares — no test reaches it through a full verification.
describe('the typed answer', () => {
  /** A handle of the given FHE type: the type byte at 30, version 0 at 31. */
  const handleOfType = (fheTypeId: number): Uint8Array => {
    const handle = new Uint8Array(32).fill(0xa1);
    handle[30] = fheTypeId;
    handle[31] = 0;
    return handle;
  };
  const EBOOL = 0;
  const EUINT64 = 5;
  const plaintext = (fheTypeId: number) => ({ bytes: new Uint8Array([0x01]), fheTypeId });

  it('accepts one plaintext per handle, each of the type its handle embeds', () => {
    expect(() =>
      verifySolanaUserDecryptPlaintexts(
        [plaintext(EBOOL), plaintext(EUINT64)],
        [handleOfType(EBOOL), handleOfType(EUINT64)],
      ),
    ).not.toThrow();
  });

  // The failure this rule exists for: a euint64 released as an ebool would be decoded as one bit of
  // a value that never was a boolean, and no other rule reads the type at all.
  it('refuses a plaintext whose type is not the one its handle embeds, naming the position and both types', () => {
    expect(() =>
      verifySolanaUserDecryptPlaintexts(
        [plaintext(EBOOL), plaintext(EBOOL)],
        [handleOfType(EBOOL), handleOfType(EUINT64)],
      ),
    ).toThrow('plaintext 1 is of FHE type 0, and the handle at that position asks for type 5');
  });

  it('refuses an answer of the wrong length, in either direction', () => {
    expect(() => verifySolanaUserDecryptPlaintexts([plaintext(EBOOL)], [])).toThrow(
      'carries 1 plaintext(s) for 0 requested handle(s)',
    );
    expect(() => verifySolanaUserDecryptPlaintexts([], [handleOfType(EBOOL)])).toThrow(
      'carries 0 plaintext(s) for 1 requested handle(s)',
    );
  });
});
