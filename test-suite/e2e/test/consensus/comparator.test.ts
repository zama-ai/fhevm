/**
 * Contract tests for the shared comparison oracle (inventory case
 * MAT-05-CANARY-CLASSES).
 *
 * Live canaries prove the comparator rejects one poisoned digest on one stack.
 * They cannot cheaply prove it rejects each of the other falsification classes,
 * and they cannot prove it ACCEPTS the shapes that are legitimately different
 * -- an aliased handle whose producing transactions arrive in a different order
 * on each operator being the one that matters, because the previous comparison
 * compared `rows[0]` and would have failed or passed depending on the sort.
 *
 * So each class is falsified here against synthetic rows, in the same
 * comparator the stack suites call.
 */
import { expect } from 'chai';

import {
  type ComparisonFields,
  type OperatorEvidence,
  BRANCH_COMPARISON,
  ComparisonMismatch,
  FULL_COMPARISON,
  compareOperatorEvidence,
  evidenceFromRows,
} from './comparator';
import type { CanonicalOutputRow } from './helpers';

const HANDLE = '0x1111111111111111111111111111111111111111111111111111111111111111';

const buffer = (byte: number, length = 32) => Buffer.alloc(length, byte);

const evidence = (operator: number, overrides: Partial<OperatorEvidence> = {}): OperatorEvidence => ({
  operator,
  handle: HANDLE,
  ciphertext: buffer(0xaa, 64),
  ciphertextType: 5,
  ciphertextVersion: 0,
  computeDigest: buffer(0xbb),
  snsDigest: buffer(0xcc),
  rawSnsCiphertext: null,
  ciphertext128Format: 11,
  keyId: buffer(0xdd),
  fheOperation: 8,
  provenance: ['0xaaaa:12345:100'],
  storageRows: 1,
  computationRows: 1,
  ...overrides,
});

const row = (overrides: Partial<CanonicalOutputRow> = {}): CanonicalOutputRow =>
  ({
    handle: Buffer.from(HANDLE.slice(2), 'hex'),
    ciphertext: buffer(0xaa, 64),
    snsCiphertext: null,
    ciphertextType: 5,
    ciphertextVersion: 0,
    fheOperation: 8,
    transactionId: buffer(0x01, 32),
    hostChainId: 12345,
    blockNumber: 100,
    keyId: buffer(0xdd),
    ciphertextDigest: buffer(0xbb),
    snsCiphertextDigest: buffer(0xcc),
    ciphertext128Format: 11,
    ...overrides,
  }) as CanonicalOutputRow;

/** Runs a comparison and returns the mismatch class, or `null` if it agreed. */
const classify = (set: OperatorEvidence[], fields: ComparisonFields = FULL_COMPARISON): string | null => {
  try {
    compareOperatorEvidence(set, fields);
    return null;
  } catch (error) {
    if (error instanceof ComparisonMismatch) return error.kind;
    throw error;
  }
};

describe('consensus comparator', () => {
  it('accepts a fleet that agrees on everything', () => {
    const report = compareOperatorEvidence([evidence(0), evidence(1), evidence(2)]);
    expect(report.operators).to.deep.eq([0, 1, 2]);
    expect(report.compared).to.include('snsDigest');
    expect(report.compared).to.include('provenance');
  });

  it('detects a raw byte mismatch', () => {
    expect(classify([evidence(0), evidence(1, { ciphertext: buffer(0xab, 64) })])).to.eq('raw-bytes');
  });

  it('detects a compute digest mismatch', () => {
    expect(classify([evidence(0), evidence(1, { computeDigest: buffer(0xbc) })])).to.eq('compute-digest');
  });

  it('detects an SNS digest mismatch', () => {
    expect(classify([evidence(0), evidence(1, { snsDigest: buffer(0xcd) })])).to.eq('sns-digest');
  });

  it('treats a missing SNS digest as missing evidence rather than as agreement', () => {
    // The regression this pins: `ciphertext_digest.ciphertext128` survives
    // submission -- only `ciphertexts128` is cleared -- so an absent digest is
    // absent evidence. Excusing it let two matching digests out of three
    // report as unanimous.
    expect(classify([evidence(0), evidence(1), evidence(2, { snsDigest: null })])).to.eq('sns-evidence-missing');
  });

  it('treats a missing compute digest as missing evidence', () => {
    expect(classify([evidence(0), evidence(1, { computeDigest: null })])).to.eq('compute-digest-missing');
  });

  it('detects a wrong key identity', () => {
    expect(classify([evidence(0), evidence(1, { keyId: buffer(0xde) })])).to.eq('key-identity');
  });

  it('detects a different producing operation', () => {
    expect(classify([evidence(0), evidence(1, { fheOperation: 9 })])).to.eq('operation');
  });

  it('detects a provenance mismatch', () => {
    expect(classify([evidence(0), evidence(1, { provenance: ['0xbbbb:12345:100'] })])).to.eq('provenance');
  });

  it('detects a type or version mismatch', () => {
    expect(classify([evidence(0), evidence(1, { ciphertextVersion: 1 })])).to.eq('type-version');
  });

  it('names a squash-backend split distinctly from a bad squash', () => {
    expect(classify([evidence(0), evidence(1, { ciphertext128Format: 21 })])).to.eq('ciphertext128-format');
  });

  it('accepts an aliased handle whose producing transactions arrive in a different order', () => {
    // The bug this replaces: the comparison read `rows[0]` from each operator,
    // and the query orders by transaction id within a block. Two operators that
    // ingested the same two aliasing transactions could hand back different
    // first rows, so the provenance comparison was a coin toss.
    const forward = evidence(0, { provenance: ['0xaaaa:12345:100', '0xbbbb:12345:100'], computationRows: 2 });
    const reversed = evidence(1, { provenance: ['0xbbbb:12345:100', '0xaaaa:12345:100'].sort(), computationRows: 2 });
    expect(classify([forward, reversed])).to.eq(null);
  });

  it('compares fewer fields only when the caller says so, and still compares bytes', () => {
    const set = [evidence(0), evidence(1, { provenance: ['0xbbbb:12345:100'] })];
    expect(classify(set, BRANCH_COMPARISON)).to.eq(null);
    const divergent = [evidence(0), evidence(1, { provenance: ['0xbbbb:12345:100'], ciphertext: buffer(0xab, 64) })];
    expect(classify(divergent, BRANCH_COMPARISON)).to.eq('raw-bytes');
  });

  it('refuses to call a single operator agreement', () => {
    expect(() => compareOperatorEvidence([evidence(0)])).to.throw(/at least two operators/);
  });

  describe('evidence folding', () => {
    it('folds several producing transactions into one normalized provenance set', () => {
      const folded = evidenceFromRows(
        1,
        HANDLE,
        [row(), row({ transactionId: buffer(0x02, 32) })],
        1,
      );
      expect(folded.computationRows).to.eq(2);
      expect(folded.provenance).to.have.length(2);
      expect(folded.provenance).to.deep.eq([...folded.provenance].sort());
    });

    it('rejects one operator holding two different values for one handle', () => {
      expect(() => evidenceFromRows(1, HANDLE, [row(), row({ ciphertext: buffer(0xab, 64) })], 1))
        .to.throw(ComparisonMismatch)
        .with.property('kind', 'value-multiplicity');
    });

    it('asserts storage-row uniqueness separately from the computation join', () => {
      // Two computation rows are an alias; two storage rows are a duplicate.
      expect(() => evidenceFromRows(1, HANDLE, [row(), row({ transactionId: buffer(0x02, 32) })], 2))
        .to.throw(ComparisonMismatch)
        .with.property('kind', 'storage-row-uniqueness');
    });

    it('rejects an operator whose own rows disagree on the digest', () => {
      expect(() =>
        evidenceFromRows(1, HANDLE, [row(), row({ transactionId: buffer(0x02, 32), ciphertextDigest: buffer(0xbc) })], 1),
      )
        .to.throw(ComparisonMismatch)
        .with.property('kind', 'compute-digest');
    });

    it('reports no rows as missing evidence, so a wait can keep waiting', () => {
      expect(() => evidenceFromRows(1, HANDLE, [], 0))
        .to.throw(ComparisonMismatch)
        .with.property('kind', 'evidence-missing');
    });
  });
});
