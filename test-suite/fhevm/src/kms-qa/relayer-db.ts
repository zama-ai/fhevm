/**
 * Relayer database inspection for the `kms-context-qa-tests` profile.
 *
 * Separate from `src/kms-connector-db.ts` on purpose. That module reads the **kms-connector**
 * databases, one per party, inside the coprocessor postgres container; this one reads the
 * **relayer's** database, in its own container, with its own schema and its own id encoding. They
 * share nothing but the shell-out shape, and this workstream does not modify pre-existing modules.
 *
 * ## What lives here that lives nowhere else
 *
 * The relayer records, per user-decryption request, both what was asked and what each KMS share
 * answered:
 *
 *   - `user_decrypt_req.req` — a JSONB of the submitted request, carrying `extra_data`;
 *   - `user_decrypt_share.extra_data` — the extraData of one share's response;
 *   - joined on `gw_reference_id`.
 *
 * That pairing is the only place in the system where the response `extraData` is observable. The SDK
 * receives it and never compares or exposes it (`qa-extradata-check.md`), and the raw unified client
 * sees only the aggregated outcome, not the per-share rows.
 *
 * Two request shapes reach the table — the legacy `/v2` route stores
 * `contract_addresses`/`ct_handle_contract_pairs`, the unified `/v3` route stores
 * `handles`/`allowed_contracts` — but `extra_data` sits at the top level of both, so the read below
 * is route-agnostic.
 */
import { PreflightError } from "../errors";
import { DEFAULT_POSTGRES_PASSWORD, DEFAULT_POSTGRES_USER } from "../layout";
import { run } from "../utils/process";

/** The relayer's postgres container, as the compose stack names it. */
export const RELAYER_DB_CONTAINER = "fhevm-relayer-db";

/** The relayer's database name. */
export const RELAYER_DB_NAME = "relayer_db";

/** Field separator for the `-A -F` psql output parsed below; absent from every column read. */
const FIELD_SEPARATOR = "|";

const relayerDbRuntime = () => ({
  container: process.env.RELAYER_DB_CONTAINER ?? RELAYER_DB_CONTAINER,
  database: process.env.RELAYER_DB_NAME ?? RELAYER_DB_NAME,
  user: process.env.RELAYER_POSTGRES_USER ?? DEFAULT_POSTGRES_USER,
  password: process.env.RELAYER_POSTGRES_PASSWORD ?? DEFAULT_POSTGRES_PASSWORD,
});

/** Runs a query against the relayer database and returns its raw, unaligned rows. */
const relayerQuery = async (sql: string): Promise<string[]> => {
  const db = relayerDbRuntime();
  const result = await run([
    "docker", "exec", "-e", `PGPASSWORD=${db.password}`, db.container,
    "psql", "-U", db.user, "-d", db.database, "-t", "-A", "-F", FIELD_SEPARATOR, "-c", sql,
  ]);
  return result.stdout
    .split("\n")
    .map((line) => line.trim())
    .filter(Boolean);
};

/** The highest `user_decrypt_req.id`, or 0 when the table is empty. */
export const readLatestUserDecryptRequestId = async (): Promise<number> => {
  const [raw] = await relayerQuery("select coalesce(max(id), 0) from user_decrypt_req;");
  const id = Number(raw ?? "");
  if (!Number.isFinite(id)) {
    throw new PreflightError(
      `kms-context-qa: could not read the relayer's latest user_decrypt_req id (got ${JSON.stringify(raw)}). ` +
        `Is the ${relayerDbRuntime().container} container running?`,
    );
  }
  return id;
};

/** One request's extraData paired with one share's, as the relayer stored them. */
export type ExtraDataEchoRow = {
  readonly requestId: number;
  readonly requestExtraData: string;
  readonly shareIndex: number;
  readonly shareExtraData: string;
};

/**
 * Reads every request/share extraData pair recorded after `afterRequestId`.
 *
 * Bounding by id rather than by timestamp keeps the read exact: the caller takes a baseline before
 * driving its probes, so the rows returned are precisely the probes' own and nothing else's — no
 * clock skew, no overlap with a concurrent suite.
 *
 * The comparison itself is deliberately NOT done in SQL. The raw values are returned so the caller
 * asserts on them and can print both sides when they differ; a boolean from postgres would say that
 * the echo broke without saying how.
 */
export const readExtraDataEchoRows = async (afterRequestId: number): Promise<ExtraDataEchoRow[]> => {
  const rows = await relayerQuery(
    `select r.id, r.req->>'extra_data', s.share_index, s.extra_data ` +
      `from user_decrypt_req r ` +
      `join user_decrypt_share s on r.gw_reference_id = s.gw_reference_id ` +
      `where r.id > ${afterRequestId} ` +
      `order by r.id, s.share_index;`,
  );
  return rows.map((line) => {
    const [requestId, requestExtraData, shareIndex, shareExtraData] = line.split(FIELD_SEPARATOR);
    if (requestExtraData === undefined || shareExtraData === undefined) {
      throw new PreflightError(
        `kms-context-qa: unparsable relayer row ${JSON.stringify(line)} — the user_decrypt_req / ` +
          `user_decrypt_share schema may have changed.`,
      );
    }
    return {
      requestId: Number(requestId),
      requestExtraData,
      shareIndex: Number(shareIndex),
      shareExtraData,
    };
  });
};

/** The legacy "no context" marker a v0 request carries. */
export const LEGACY_ZERO_MARKER = "0x00";

/** The empty payload the connector normalizes {@link LEGACY_ZERO_MARKER} to. */
export const EMPTY_EXTRA_DATA = "0x";

/**
 * Returns whether `shareExtraData` is an acceptable echo of `requestExtraData`.
 *
 * Byte-exact for every versioned payload. The one exception is the legacy `0x00` marker, which the
 * kms-connector deliberately normalizes to empty before handing the request to the KMS core — see
 * `kms-connector/.../event_processor/decryption.rs`, whose unit test
 * `kms_decryption_extra_data_normalizes_legacy_zero_marker` asserts exactly that. Both values mean
 * "no context", so the normalization preserves meaning even though it does not preserve bytes.
 *
 * **This exception is deliberately narrow, and that is the point.** It admits only the empty payload.
 * A v0 request answered with 33 or 65 bytes is still a failure — that would be the field being
 * rebuilt from the responder's own view of the active context, which is the behaviour this whole case
 * exists to detect. Widening the rule to "any length is fine for v0" would surrender exactly the
 * discrimination v0 was chosen to provide.
 *
 * Pure; exported for unit testing.
 */
export const isAcceptableEcho = (requestExtraData: string, shareExtraData: string): boolean => {
  if (shareExtraData === requestExtraData) return true;
  return requestExtraData === LEGACY_ZERO_MARKER && shareExtraData === EMPTY_EXTRA_DATA;
};
