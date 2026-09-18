/**
 * The shared comparison oracle, and the one place that decides what "the
 * operators agree" means.
 *
 * Every suite in this directory ultimately asks the same question, and each of
 * them used to answer it slightly differently: one compared `rows[0]` against
 * `rows[0]`, which is a coin toss once a handle has several legitimate
 * producing transactions; one treated a missing SNS digest as normal
 * post-submission cleanup, which it is not; one asserted
 * `report.snsDigestsChecked || true`, which is not an assertion. So the
 * comparison lives here, it names what it compared, and it fails with a
 * classified reason rather than a bare boolean.
 *
 * Two facts about the schema shape everything below.
 *
 * A handle may have SEVERAL producing computations. Under the
 * minted-in-transaction discriminant, two transactions with identical operand
 * sourcing alias to one handle, so `computations` legitimately holds one row per
 * producing transaction while `ciphertexts` holds exactly one row for the
 * value. Row multiplicity is therefore normal and value multiplicity is not,
 * and the two are asserted separately.
 *
 * The SNS digest is durable. `transaction-sender`'s `delete_ct128_from_db`
 * deletes the raw squashed ciphertext from `ciphertexts128` once
 * AddCiphertextMaterial has landed, but `set_txn_is_sent` only stamps
 * `txn_is_sent`/`txn_hash`/`txn_block_number` on `ciphertext_digest` -- the
 * `ciphertext` and `ciphertext128` digest columns stay. So an absent RAW ct128
 * is expected after submission, and an absent SNS DIGEST is missing evidence.
 * Excusing the latter as cleanup is what let a two-out-of-three digest
 * comparison pass as unanimous.
 */
import { Pool } from 'pg';

import { type CanonicalOutputRow, queryCanonicalOutputs } from './helpers';
import { type MismatchKind, ComparisonMismatch } from './mismatch';

export { type MismatchKind, ComparisonMismatch } from './mismatch';

/** One operator's complete evidence for one handle. */
export interface OperatorEvidence {
  operator: number;
  handle: string;
  /** The single distinct value the operator holds for the handle. */
  ciphertext: Buffer;
  ciphertextType: number;
  ciphertextVersion: number;
  computeDigest: Buffer | null;
  /** `ciphertext_digest.ciphertext128`: durable, so absence is missing evidence. */
  snsDigest: Buffer | null;
  /** `ciphertexts128.ciphertext`: deleted after submission, so absence is expected. */
  rawSnsCiphertext: Buffer | null;
  ciphertext128Format: number | null;
  keyId: Buffer;
  fheOperation: number;
  /** Normalized, sorted, de-duplicated `txid:chain:block` triples. */
  provenance: string[];
  /** Rows in `ciphertexts` for this handle and version. Exactly one is correct. */
  storageRows: number;
  /** Rows in `computations` that produced it. More than one is an alias, not a fault. */
  computationRows: number;
}

/** Which fields a comparison covers. */
export interface ComparisonFields {
  rawBytes: boolean;
  typeVersion: boolean;
  computeDigest: boolean;
  snsDigest: boolean;
  keyIdentity: boolean;
  operation: boolean;
  /**
   * Off for cross-branch comparisons: an operator that observed the handle on
   * the other branch legitimately attributes it to a different transaction and
   * block, so demanding equal provenance there would fail by design.
   */
  provenance: boolean;
  ciphertext128Format: boolean;
}

export const FULL_COMPARISON: ComparisonFields = {
  rawBytes: true,
  typeVersion: true,
  computeDigest: true,
  snsDigest: true,
  keyIdentity: true,
  operation: true,
  provenance: true,
  ciphertext128Format: true,
};

/** What a cross-branch comparison can honestly compare. */
export const BRANCH_COMPARISON: ComparisonFields = {
  ...FULL_COMPARISON,
  provenance: false,
};

export interface AgreementReport {
  handle: string;
  operators: number[];
  ciphertextDigest: string;
  snsDigest: string;
  /** The normalized provenance every compared operator agreed on. */
  provenance: string[];
  /** Field names this comparison actually covered, for the run log. */
  compared: string[];
}

const hex = (value: Buffer) => value.toString('hex');
const handleBuffer = (handle: string) => Buffer.from(handle.replace(/^0x/, ''), 'hex');

const normalizeProvenance = (rows: readonly CanonicalOutputRow[]): string[] =>
  [...new Set(rows.map((row) => `0x${hex(row.transactionId)}:${row.hostChainId}:${row.blockNumber}`))].sort();

/**
 * Counts the storage rows for a handle, independently of the computation join.
 *
 * `queryCanonicalOutputs` joins `ciphertexts` to `computations`, so an aliased
 * handle returns several rows for one stored value. Asserting "exactly one row"
 * on that join would fail a legitimate alias; asserting nothing about storage
 * would miss a genuine duplicate. Counting here separates the two questions.
 */
export async function queryStorageRowCount(
  databaseUrl: string,
  handle: string,
  ciphertextVersion = 0,
): Promise<number> {
  const pool = new Pool({ connectionString: databaseUrl, max: 1 });
  try {
    const result = await pool.query<{ count: string }>(
      'SELECT COUNT(*)::text AS count FROM ciphertexts WHERE handle = $1 AND ciphertext_version = $2',
      [handleBuffer(handle), ciphertextVersion],
    );
    return Number.parseInt(result.rows[0].count, 10);
  } finally {
    await pool.end();
  }
}

/**
 * Folds one operator's rows for a handle into a single evidence record.
 *
 * Exported separately from the collection so a unit test can drive the
 * comparator with synthetic rows and falsify each class in turn.
 */
export function evidenceFromRows(
  operator: number,
  handle: string,
  rows: readonly CanonicalOutputRow[],
  storageRows: number,
): OperatorEvidence {
  if (rows.length === 0) {
    throw new ComparisonMismatch('evidence-missing', handle, [operator], `operator ${operator} holds no completed row`);
  }
  const values = new Set(rows.map((row) => hex(row.ciphertext)));
  if (values.size !== 1) {
    throw new ComparisonMismatch(
      'value-multiplicity',
      handle,
      [operator],
      `operator ${operator} holds ${values.size} different values for one handle; a handle denotes one value`,
    );
  }
  if (storageRows !== 1) {
    throw new ComparisonMismatch(
      'storage-row-uniqueness',
      handle,
      [operator],
      `operator ${operator} holds ${storageRows} storage row(s) for the handle; first-write-wins must leave exactly one`,
    );
  }
  // The digest columns are keyed by (handle, host_chain_id), so an alias's
  // several computation rows carry the same digest. A disagreement between them
  // on one operator is a validation gap rather than an alias.
  const digests = new Set(rows.map((row) => (row.ciphertextDigest ? hex(row.ciphertextDigest) : 'absent')));
  if (digests.size !== 1) {
    throw new ComparisonMismatch(
      'compute-digest',
      handle,
      [operator],
      `operator ${operator}'s rows disagree with each other on the compute digest`,
    );
  }
  const reference = rows[0];
  return {
    operator,
    handle,
    ciphertext: reference.ciphertext,
    ciphertextType: reference.ciphertextType,
    ciphertextVersion: reference.ciphertextVersion,
    computeDigest: reference.ciphertextDigest,
    snsDigest: reference.snsCiphertextDigest,
    rawSnsCiphertext: reference.snsCiphertext,
    ciphertext128Format: reference.ciphertext128Format,
    keyId: reference.keyId,
    fheOperation: reference.fheOperation,
    provenance: normalizeProvenance(rows),
    storageRows,
    computationRows: rows.length,
  };
}

/** Reads one operator's evidence straight from its database. */
export async function collectOperatorEvidence(
  databaseUrl: string,
  operator: number,
  handle: string,
  ciphertextVersion = 0,
): Promise<OperatorEvidence> {
  const rows = await queryCanonicalOutputs(databaseUrl, [handle], { ciphertextVersion });
  const storageRows = await queryStorageRowCount(databaseUrl, handle, ciphertextVersion);
  return evidenceFromRows(operator, handle, rows, storageRows);
}

/**
 * The comparison itself.
 *
 * Every compared field is checked against the first operator's evidence and
 * fails with its own class. Missing evidence fails as missing evidence: the
 * comparison never quietly narrows the field set or the participant set after
 * seeing that something is absent, which is how a partial comparison used to
 * report unanimity.
 */
export function compareOperatorEvidence(
  evidence: readonly OperatorEvidence[],
  fields: ComparisonFields = FULL_COMPARISON,
): AgreementReport {
  if (evidence.length < 2) {
    throw new Error('a consensus comparison needs at least two operators');
  }
  const handle = evidence[0].handle;
  const operators = evidence.map((entry) => entry.operator);
  for (const entry of evidence) {
    if (entry.handle !== handle) {
      throw new Error(`evidence set mixes handles ${handle} and ${entry.handle}`);
    }
  }

  if (fields.computeDigest) {
    const missing = evidence.filter((entry) => entry.computeDigest === null).map((entry) => entry.operator);
    if (missing.length > 0) {
      throw new ComparisonMismatch(
        'compute-digest-missing',
        handle,
        operators,
        `operator(s) ${missing.join(',')} hold no compute digest, so agreement on it cannot be established`,
      );
    }
  }
  if (fields.snsDigest) {
    // Required from EVERY expected participant. The digest columns survive
    // submission -- only the raw ct128 blob is deleted -- so a missing digest
    // is missing evidence, and two matching digests out of three is not
    // unanimous.
    const missing = evidence.filter((entry) => entry.snsDigest === null).map((entry) => entry.operator);
    if (missing.length > 0) {
      throw new ComparisonMismatch(
        'sns-evidence-missing',
        handle,
        operators,
        `operator(s) ${missing.join(',')} hold no SNS digest in ciphertext_digest.ciphertext128. That column is ` +
          'retained after submission (transaction-sender clears ciphertexts128, not the digest), so this is ' +
          'absent evidence rather than ordinary cleanup, and a full-pipeline agreement claim cannot be made',
      );
    }
  }

  const reference = evidence[0];
  for (const entry of evidence.slice(1)) {
    const pair = [reference.operator, entry.operator];
    if (fields.rawBytes && !entry.ciphertext.equals(reference.ciphertext)) {
      throw new ComparisonMismatch(
        'raw-bytes',
        handle,
        pair,
        `operator ${entry.operator} holds ${entry.ciphertext.length} byte(s) differing from operator ` +
          `${reference.operator}'s ${reference.ciphertext.length}`,
      );
    }
    if (
      fields.typeVersion &&
      (entry.ciphertextType !== reference.ciphertextType || entry.ciphertextVersion !== reference.ciphertextVersion)
    ) {
      throw new ComparisonMismatch(
        'type-version',
        handle,
        pair,
        `operator ${entry.operator} recorded type/version ${entry.ciphertextType}/${entry.ciphertextVersion} ` +
          `against ${reference.ciphertextType}/${reference.ciphertextVersion}`,
      );
    }
    if (fields.computeDigest && !entry.computeDigest!.equals(reference.computeDigest!)) {
      throw new ComparisonMismatch(
        'compute-digest',
        handle,
        pair,
        `operator ${entry.operator} reports compute digest ${hex(entry.computeDigest!).slice(0, 16)} against ` +
          `${hex(reference.computeDigest!).slice(0, 16)}`,
      );
    }
    if (fields.snsDigest && !entry.snsDigest!.equals(reference.snsDigest!)) {
      throw new ComparisonMismatch(
        'sns-digest',
        handle,
        pair,
        `operator ${entry.operator} reports SNS digest ${hex(entry.snsDigest!).slice(0, 16)} against ` +
          `${hex(reference.snsDigest!).slice(0, 16)}`,
      );
    }
    if (fields.keyIdentity && !entry.keyId.equals(reference.keyId)) {
      throw new ComparisonMismatch(
        'key-identity',
        handle,
        pair,
        `operator ${entry.operator} attributes the output to key ${hex(entry.keyId).slice(0, 16)} against ` +
          `${hex(reference.keyId).slice(0, 16)}; the operators are not computing under one key`,
      );
    }
    if (fields.operation && entry.fheOperation !== reference.fheOperation) {
      throw new ComparisonMismatch(
        'operation',
        handle,
        pair,
        `operator ${entry.operator} recorded producing operation ${entry.fheOperation} against ` +
          `${reference.fheOperation}`,
      );
    }
    if (fields.provenance && entry.provenance.join('|') !== reference.provenance.join('|')) {
      throw new ComparisonMismatch(
        'provenance',
        handle,
        pair,
        `operator ${entry.operator} attributes the output to [${entry.provenance.join(', ')}] against ` +
          `[${reference.provenance.join(', ')}]`,
      );
    }
    if (
      fields.ciphertext128Format &&
      entry.ciphertext128Format !== null &&
      reference.ciphertext128Format !== null &&
      entry.ciphertext128Format !== reference.ciphertext128Format
    ) {
      // RFC-023 records which backend squashed the value (11 compressed on CPU,
      // 21 compressed on GPU), so a disagreement here is a fleet split across
      // squash backends rather than a bad squash -- worth naming distinctly,
      // because it presents as an SNS digest disagreement with everything else
      // matching.
      throw new ComparisonMismatch(
        'ciphertext128-format',
        handle,
        pair,
        `operator ${entry.operator} squashed in format ${entry.ciphertext128Format} against ` +
          `${reference.ciphertext128Format}; the fleet is split across squash backends`,
      );
    }
  }

  return {
    handle,
    operators,
    ciphertextDigest: reference.computeDigest ? hex(reference.computeDigest) : 'not compared',
    snsDigest: reference.snsDigest ? hex(reference.snsDigest) : 'not compared',
    provenance: reference.provenance,
    compared: Object.entries(fields)
      .filter(([, enabled]) => enabled)
      .map(([name]) => name),
  };
}
