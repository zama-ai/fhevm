// The linker vectors, and what a response has to match to be this request's.
//
// `solana/test-fixtures/user-decrypt/solana_linker_v2.json` is generated on the KMS side and travels
// with the blob that computes links here — the regeneration script copies both from one kms commit, so
// a blob checked against another commit's vectors is not a state this tree can reach. The set carries
// its own digest, and the digest is checked here: the same cross-repository contract the KMS-side
// suites assert.
//
// The records do the work. Four accepted ones pin the link this SDK computes against the link the KMS
// computes. Twelve divergence records each mutate one bound field — four of them a field of the gateway
// domain the struct is hashed under — and carry the different link that mutation produces; a response
// signed over the reference link is refused for every one of them. Eight construction rejects are
// inputs the checked construction refuses outright, a request without a domain among them. And two
// foreign-link records carry links from another construction — a different type string, and the
// retired list hash — which this one must never reproduce.

import type { BytesHex } from '../../core/types/primitives.js';
import type {
  SolanaGatewayEip712Domain,
  SolanaUserDecryptLinkInputs,
  SolanaUserDecryptRequestInputs,
} from './index.js';
import { readFileSync } from 'node:fs';
import { secp256k1 } from '@noble/curves/secp256k1.js';
import { sha256 } from '@noble/hashes/sha2.js';
import { hashTypedData, keccak256, toBytes } from 'viem';
import { privateKeyToAccount } from 'viem/accounts';
import { describe, expect, it } from 'vitest';
import { compute_solana_user_decrypt_link_from_js } from '../../wasm/tkms/kms_lib.v0.15.0-0-solana.294802eb.js';
import { toChecksummedAddress } from '../../core/base/address.js';
import { sign } from '../../core/base/sign.js';
import { PERMIT_KMS_ROUTING_VERSION, encodeSolanaKmsRouting } from '../permit/index.js';
import {
  generateSolanaTransportKeyPair,
  solanaUserDecryptLink,
  solanaUserDecryptRequestHalf,
  verifySolanaUserDecryptPlaintexts,
  verifySolanaUserDecryptResponse,
} from './index.js';
import { bytesToHex, hexToBytes } from '../proof.js';

/* eslint-disable @typescript-eslint/naming-convention -- the fixture's own field names are snake_case */

interface LinkerDomain {
  readonly name: string;
  readonly version: string;
  readonly chain_id_decimal: string;
  readonly verifying_contract: string;
}

/** One record as the file spells it. */
interface LinkerRecordJson {
  readonly name: string;
  readonly comment: string;
  readonly result: 'valid' | 'invalid';
  readonly class: 'valid' | 'link-divergence' | 'construction-reject' | 'foreign-link';
  readonly rule?: string;
  readonly rejected_by?: string;
  readonly derived_from?: string;
  readonly mutation?: string;
  readonly chain_id_decimal: string;
  readonly declared_chain_id_decimal?: string;
  readonly receiver_id: string;
  readonly verifying_program_id: string;
  readonly handles: readonly string[];
  readonly transport_key: string;
  /** The gateway EIP-712 domain the record's link was hashed under; null on the domain-less reject. */
  readonly domain: LinkerDomain | null;
  readonly construction: 'eip712' | 'shake256-list-hash-v1';
  readonly type_string: string;
  readonly domain_separator?: string;
  readonly hash_struct?: string;
  readonly link?: string;
  /** Only the retired construction bound the route; the record that replays it carries the bytes. */
  readonly extra_data?: string;
}

interface LinkerFixtureJson {
  readonly schema: string;
  readonly set_digest_file: string;
  readonly type_string: string;
  readonly type_hash: string;
  readonly domain_type_string: string;
  readonly construction_rule: string;
  readonly transport_keys: Readonly<Record<string, string>>;
  readonly records: readonly LinkerRecordJson[];
}

/* eslint-enable @typescript-eslint/naming-convention */

/** A record as read here: the gateway domain under its own name, the rest as the file spells it. */
type LinkerRecord = Omit<LinkerRecordJson, 'domain'> & { readonly gatewayDomain: LinkerDomain | null };

interface LinkerFixture extends Omit<LinkerFixtureJson, 'records'> {
  readonly records: readonly LinkerRecord[];
}

const FIXTURE_DIR = new URL('../../../../../solana/test-fixtures/user-decrypt/', import.meta.url);
const VECTOR_FILE = 'solana_linker_v2.json';

/** The struct the link hashes; the type string is the construction's version boundary. */
const TYPE_STRING =
  'SolanaUserDecryptionLinker(bytes publicKey,bytes32[] handles,bytes32 userPubkey,bytes32 verifyingProgramId)';
const DOMAIN_TYPE_STRING = 'EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)';

const fixtureBytes = readFileSync(new URL(VECTOR_FILE, FIXTURE_DIR));
const fixtureJson = JSON.parse(fixtureBytes.toString('utf8')) as LinkerFixtureJson;
const fixture: LinkerFixture = {
  ...fixtureJson,
  records: fixtureJson.records.map(({ domain: gatewayDomain, ...record }) => ({ ...record, gatewayDomain })),
};

/** A record's gateway domain in the SDK's configuration shape. */
function domainOf(domain: LinkerDomain): SolanaGatewayEip712Domain {
  return {
    name: domain.name,
    version: domain.version,
    chainId: BigInt(domain.chain_id_decimal),
    verifyingContract: domain.verifying_contract,
  };
}

/** The inputs one record stands for, in the form the SDK computes a link from. */
function inputsOf(record: LinkerRecord): SolanaUserDecryptLinkInputs {
  const transportKey = fixture.transport_keys[record.transport_key];
  if (transportKey === undefined) {
    throw new Error(`${record.name}: transport key ${record.transport_key} is not in the file's table`);
  }
  if (record.gatewayDomain === null) {
    throw new Error(`${record.name}: carries no domain; the domain-less record is exercised on its own below`);
  }
  return {
    userAddress: hexToBytes(`0x${record.receiver_id}`),
    // A record that declares a chain id different from the one its handles embed is testing exactly
    // that disagreement, so the declared one is what goes in.
    hostChainId: BigInt(record.declared_chain_id_decimal ?? record.chain_id_decimal),
    verifyingProgramId: hexToBytes(`0x${record.verifying_program_id}`),
    handles: record.handles.map((handle) => hexToBytes(`0x${handle}`)),
    transportKey: hexToBytes(`0x${transportKey}`),
    gatewayEip712Domain: domainOf(record.gatewayDomain),
  };
}

/**
 * The KMS route of the reference request: a versioned routing envelope, the version byte over two
 * ids. Not a vector field — the link does not bind the route — so the tests supply one of their own.
 */
const REFERENCE_ROUTE = encodeSolanaKmsRouting({
  version: PERMIT_KMS_ROUTING_VERSION,
  kmsContextId: new Uint8Array(32).fill(0x33),
  kmsEpochId: new Uint8Array(32).fill(0x44),
});

/** The request a record stands for: its link inputs beside the route the permit signed. */
function requestOf(record: LinkerRecord, extraData: Uint8Array = REFERENCE_ROUTE): SolanaUserDecryptRequestInputs {
  return { ...inputsOf(record), extraData };
}

const byClass = (className: LinkerRecord['class']): readonly LinkerRecord[] =>
  fixture.records.filter((record) => record.class === className);

const named = (records: readonly LinkerRecord[]): ReadonlyArray<readonly [string, LinkerRecord]> =>
  records.map((record) => [record.name, record] as const);

const recordNamed = (name: string): LinkerRecord => {
  const record = fixture.records.find((candidate) => candidate.name === name);
  if (record === undefined) {
    throw new Error(`the linker vector set carries no ${name} record`);
  }
  return record;
};

const reference = recordNamed('reference-two-handles');
if (reference.gatewayDomain === null || reference.link === undefined) {
  throw new Error('the reference record carries no domain or no link');
}
const referenceDomain = reference.gatewayDomain;
const referenceLink = reference.link;

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

  // The type string is the version boundary of the construction: a struct with one more field, or
  // a field renamed, is another type hash and therefore another link for the same request.
  it('is read under the schema and the typed struct it declares', () => {
    expect(fixture.schema).toBe('zama-solana-linker-vectors/v2');
    expect(fixture.type_string).toBe(TYPE_STRING);
    expect(fixture.domain_type_string).toBe(DOMAIN_TYPE_STRING);
    expect(keccak256(toBytes(TYPE_STRING)).slice(2)).toBe(fixture.type_hash);
  });

  it('covers every class the response verification has to handle', () => {
    for (const className of ['valid', 'link-divergence', 'construction-reject', 'foreign-link'] as const) {
      expect(byClass(className).length, `no record of class ${className}`).toBeGreaterThan(0);
    }
    expect(new Set(fixture.records.map((record) => record.name)).size).toBe(fixture.records.length);
  });

  // A link without the domain it was hashed under is not reproducible; the set never states one.
  it('names the gateway domain beside every link it records', () => {
    for (const record of fixture.records) {
      if (record.link !== undefined) {
        expect(record.gatewayDomain, `${record.name} records a link but no domain`).not.toBeNull();
      }
    }
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
    expect(link).not.toBe(referenceLink);
  });

  // The domain is a bound input, not a verification-time setting: the four records below leave every
  // request field of the reference untouched and change one field of the domain each. Under the
  // wrong domain the client computes a link no response for its request will ever carry — which is
  // the fail-closed direction, and the reason the domain has to be the gateway's and not a guess.
  it('binds the gateway domain: each domain field alone moves the link', async () => {
    const domainRecords = fixture.records.filter((record) => record.rule === 'wrong-gateway-domain');
    expect(domainRecords.map((record) => record.name).sort()).toEqual([
      'wrong-gateway-chain-id',
      'wrong-gateway-domain-name',
      'wrong-gateway-domain-version',
      'wrong-gateway-verifying-contract',
    ]);
    for (const record of domainRecords) {
      expect(record.receiver_id).toBe(reference.receiver_id);
      expect(record.handles).toEqual(reference.handles);
      expect(record.transport_key).toBe(reference.transport_key);
      expect(record.gatewayDomain).not.toEqual(referenceDomain);
      expect(bytesToHex(await solanaUserDecryptLink(inputsOf(record))).slice(2)).not.toBe(referenceLink);
    }
  });

  // Not a divergent link but no link at all: the construction refuses inputs that are not a request,
  // rather than hashing them into something that looks like one.
  it.each(named(byClass('construction-reject').filter((record) => record.gatewayDomain !== null)))(
    '%s: is refused, not computed',
    async (_name, record) => {
      await expect(solanaUserDecryptLink(inputsOf(record))).rejects.toThrow();
    },
  );

  // The SDK's type makes the domain required and its marshaling reads it, so the domain-less record
  // cannot be expressed through the SDK. It is pinned at the blob instead — the same call the SDK
  // makes, with the domain left out — so that the refusal behind the types is the blob's own and no
  // caller of the raw export hashes under an empty domain.
  it('missing-domain: is refused by the blob, not hashed under an empty domain', async () => {
    const missing = recordNamed('missing-domain');
    expect(missing.gatewayDomain).toBeNull();
    expect(missing.rejected_by).toBe('domain-required');
    // The SDK call initializes the shared blob; the raw export below runs on the same instance.
    await solanaUserDecryptLink(inputsOf(reference));
    expect(() =>
      compute_solana_user_decrypt_link_from_js(
        {
          user_pubkey: missing.receiver_id,
          host_chain_id: missing.chain_id_decimal,
          verifying_program_id: missing.verifying_program_id,
        },
        missing.handles,
        hexToBytes(`0x${fixture.transport_keys[missing.transport_key]!}`),
        undefined,
      ),
    ).toThrow(/eip712_domain is required/);
  });

  // A link from another construction: a struct with a different type string, or the retired list hash
  // over the same fields. This one must be unable to produce either — otherwise a response minted for a
  // different version of the protocol would satisfy this version's comparison.
  it.each(named(byClass('foreign-link')))('%s: is not reachable from this construction', async (_name, record) => {
    expect(record.construction !== 'eip712' || record.type_string !== fixture.type_string).toBe(true);
    const link = bytesToHex(await solanaUserDecryptLink(inputsOf(record))).slice(2);
    expect(link).not.toBe(record.link);
  });
});

// The request half handed to the blob: every field the client's own. The KMS route is the field
// that makes this pinnable and worth pinning — the link does not bind it, but the blob compares it
// against the route the response carries before any signature is checked, so a request half that
// zeroes it (or copies it from the response) turns that comparison into nothing. The committed
// vectors carry no signcrypted shares, so no test reaches the comparison through a full
// verification; what is pinned instead is the bytes this side brings to it.
describe('the request half of the link contract', () => {
  it('carries the signed KMS route: the permit’s extra_data, byte for byte', () => {
    const half = solanaUserDecryptRequestHalf(requestOf(reference));
    // The reference route is the versioned routing envelope: the version byte over two ids.
    expect(half.extra_data).toMatch(/^02[0-9a-f]{128}$/);
    expect(half.extra_data).toBe(bytesToHex(REFERENCE_ROUTE).slice(2));
  });

  it('follows a changed route byte in the client inputs — the route is the client’s, not an echo', () => {
    const mutated = REFERENCE_ROUTE.map((byte, index) => (index === 1 ? byte ^ 0x01 : byte));
    const half = solanaUserDecryptRequestHalf(requestOf(reference, mutated));
    expect(half.extra_data).not.toBe(bytesToHex(REFERENCE_ROUTE).slice(2));
    expect(half.extra_data).toBe(bytesToHex(mutated).slice(2));
  });

  it('zeroes the EVM-shaped fields and carries the handles in request order', () => {
    const half = solanaUserDecryptRequestHalf(requestOf(reference));
    expect(half.client_address).toBe('0x0000000000000000000000000000000000000000');
    expect(half.eip712_verifying_contract).toBe('0x0000000000000000000000000000000000000000');
    expect(half.signature).toBeUndefined();
    expect(half.ciphertext_handles).toEqual(reference.handles);
  });

  // The route travels beside the link inputs and never into them: the same request under any route,
  // or under none, computes the reference link. It is the node signature that vouches for the route.
  it('keeps the route out of the link: any route, or none, computes the same link', async () => {
    const mutated = REFERENCE_ROUTE.map((byte, index) => (index === 33 ? byte ^ 0x01 : byte));
    expect(bytesToHex(await solanaUserDecryptLink(requestOf(reference, mutated))).slice(2)).toBe(referenceLink);
    expect(bytesToHex(await solanaUserDecryptLink(requestOf(reference, new Uint8Array(0)))).slice(2)).toBe(
      referenceLink,
    );
    expect(solanaUserDecryptRequestHalf(requestOf(reference, new Uint8Array(0))).extra_data).toBe('');
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
        request: requestOf(reference),
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
        request: requestOf(reference),
        shares: [{ signature: '0x00', payload: '0x00', extraData: '0x' }],
        keyPair,
        signers,
        fheParameter: 'test',
      }),
    ).rejects.toThrow();
  });
});

////////////////////////////////////////////////////////////////////////////////
// The signature, route and link rules, through the blob
////////////////////////////////////////////////////////////////////////////////

// The rules the committed linker vectors cannot reach: a share is authenticated against the message
// the server signed — the payload, the request's transport key, and the request's KMS route — under
// the gateway domain, and its digest is then compared to the link recomputed from the request. The
// signer set is trusted configuration, so the test registers its own node key and signs the share
// itself; what it cannot produce is a real signcryption, so the accepted share is caught one rule
// later, at unsigncryption — which is the proof the signature, route and link rules passed. The share
// bytes are hand-built to the wire layout (proto field order, bincode legacy: u64-LE length-prefixed
// byte fields, fixed-width little-endian ints). That layout is pinned by the vendored blob: if a kms
// bump changes it, this suite fails loudly at parsing rather than drifting.
//
// Unsigncryption itself is the deliberate stopping point. Carrying these cases through the
// release — a plaintext out of the accepted share — needs signcrypted shares only the kms
// repository's signcryption code can mint; those arrive as the committed transcripts beside the
// blob (see wasmTranscript.test.ts), and the release path is covered by the live e2e suite.
describe('the signature, route and link rules', () => {
  // Deterministic test-only node key; its address is the trusted registry entry.
  const nodePrivateKey = `0x${'2a'.repeat(32)}` as const;
  const nodeAccount = privateKeyToAccount(nodePrivateKey);
  // The reference domain of the vector set: the link in the hand-built payload below is the
  // reference link, so the request has to be verified under the very domain that link was hashed
  // under. Its contract address is letterful and EIP-55, so the spelling cases below are not vacuous.
  const gatewayEip712Domain = domainOf(referenceDomain);
  expect(toChecksummedAddress(gatewayEip712Domain.verifyingContract)).toBe(gatewayEip712Domain.verifyingContract);
  /** The signed KMS route of the reference request: its extra_data, the version byte over two ids. */
  const referenceRoute = bytesToHex(REFERENCE_ROUTE) as `0x${string}`;

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
    const link = hexToBytes(`0x${referenceLink}`);
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
   * A share signed by the registered node key over `route`, under `domain` — the very message the
   * server builds: the request's transport key, the payload's handles, the serialized payload, and
   * the route.
   */
  const shareOver = (
    route: `0x${string}`,
    domain: SolanaGatewayEip712Domain = gatewayEip712Domain,
  ): { signature: string; payload: string; extraData: string } => {
    const payload = payloadBytes();
    const hash = hashTypedData({
      domain: {
        name: domain.name,
        version: domain.version,
        chainId: domain.chainId,
        verifyingContract: domain.verifyingContract as `0x${string}`,
      },
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
    request: SolanaUserDecryptRequestInputs,
    share: ReturnType<typeof shareOver>,
    spelling: { signerAddress?: string; verifyingContract?: string } = {},
  ) =>
    verifySolanaUserDecryptResponse({
      request: {
        ...request,
        gatewayEip712Domain: {
          ...request.gatewayEip712Domain,
          verifyingContract: spelling.verifyingContract ?? request.gatewayEip712Domain.verifyingContract,
        },
      },
      shares: [share],
      keyPair: await generateSolanaTransportKeyPair(),
      signers: [{ partyId: 1, address: spelling.signerAddress ?? nodeAccount.address }],
      fheParameter: 'test',
    });

  // The control: with the request's own route under the request's own domain, the share
  // authenticates and carries the link — it is refused only at unsigncryption, the first rule this
  // test cannot satisfy. This is what makes the rejections below the doing of the mutated field and
  // not of garbage failing anywhere.
  it('a share signed over the request route passes the signature and link rules', async () => {
    await expect(verifyWith(requestOf(reference), shareOver(referenceRoute))).rejects.toThrow(
      /could not unsigncrypt the response from party 1/,
    );
  });

  // The route, response side: one changed route byte, everything else untouched. The share must be
  // refused by the signature rule — the wire's route is not the one this client's permit signed.
  it('one changed route byte in the response is refused by the node-signature rule', async () => {
    const mutated = `0x03${referenceRoute.slice(4)}` as `0x${string}`;
    await expect(verifyWith(requestOf(reference), shareOver(mutated))).rejects.toThrow(
      /the KMS node signature on the response from party 1 is not valid/,
    );
  });

  // The same mutation, request side: the client's signed route differs by one byte (the first byte
  // of the epoch id), the share is the untouched reference one. The verification takes the route
  // from the client's own inputs, so the same rule refuses it — a response cannot bring its own
  // route.
  it('one changed route byte in the client inputs refuses the untouched share', async () => {
    const mutatedRoute = REFERENCE_ROUTE.map((byte, index) => (index === 33 ? byte ^ 0x01 : byte));
    await expect(verifyWith(requestOf(reference, mutatedRoute), shareOver(referenceRoute))).rejects.toThrow(
      /the KMS node signature on the response from party 1 is not valid/,
    );
  });

  // The link rule proper: the share is the reference request's, authenticated and untouched, but
  // the client asks for another recipient. The signed message does not name the recipient, so the
  // signature rule passes; the recomputed link is another request's, and the digest is not it.
  it('an authenticated share for another request is refused by the link rule', async () => {
    const otherRecipient = recordNamed('wrong-recipient');
    expect(otherRecipient.handles).toEqual(reference.handles);
    await expect(verifyWith(requestOf(otherRecipient), shareOver(referenceRoute))).rejects.toThrow(
      /not the link recomputed from the request/,
    );
  });

  // The domain, verification side: the client configured with another gateway's domain refuses the
  // real share by the signature rule, since the node signed under the gateway's.
  it('a foreign gateway domain refuses the untouched share by the node-signature rule', async () => {
    const foreign = { ...gatewayEip712Domain, chainId: gatewayEip712Domain.chainId + 1n };
    await expect(
      verifyWith({ ...requestOf(reference), gatewayEip712Domain: foreign }, shareOver(referenceRoute)),
    ).rejects.toThrow(/the KMS node signature on the response from party 1 is not valid/);
  });

  // The domain, link side: a share signed under a foreign domain, verified under that same foreign
  // domain, passes the signature rule — and is then refused because the link it carries was hashed
  // under the gateway's domain, not this one. The domain is an input to the link, not only to the
  // signature.
  it('a share whose link was hashed under another domain is refused by the link rule', async () => {
    const foreign = { ...gatewayEip712Domain, chainId: gatewayEip712Domain.chainId + 1n };
    await expect(
      verifyWith({ ...requestOf(reference), gatewayEip712Domain: foreign }, shareOver(referenceRoute, foreign)),
    ).rejects.toThrow(/not the link recomputed from the request/);
  });

  // The address spelling at the same crossing. The blob's parser accepts only EIP-55 mixed case,
  // while configuration read from on-chain bytes is naturally all-lowercase — a valid address the
  // parser would refuse as "Bad address checksum". The boundary re-encodes exactly the spellings
  // that carry no checksum; a mixed-case spelling claims one, and a wrong claim must stay refused.
  // (The checksummed spelling is the control above: `nodeAccount.address` and the reference domain's
  // contract are both EIP-55.)

  it('lowercase signer and gateway addresses are the same trust anchor, not a bad checksum', async () => {
    await expect(
      verifyWith(requestOf(reference), shareOver(referenceRoute), {
        signerAddress: nodeAccount.address.toLowerCase(),
        verifyingContract: gatewayEip712Domain.verifyingContract.toLowerCase(),
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
      verifyWith(requestOf(reference), shareOver(referenceRoute), { signerAddress: flipped }),
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
