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

import type { SolanaUserDecryptLinkInputs } from './index.js';
import { readFileSync } from 'node:fs';
import { sha256 } from '@noble/hashes/sha2.js';
import { describe, expect, it } from 'vitest';
import { solanaUserDecryptLink, verifySolanaUserDecryptPlaintexts, verifySolanaUserDecryptResponse } from './index.js';
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
