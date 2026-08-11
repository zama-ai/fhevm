/**
 * Shared kms-connector database inspection for the KMS acceptance profiles. Each party runs its own
 * connector (gw-listener + kms-worker + tx-sender) backed by its own database in the shared
 * postgres container; these helpers run a scalar query against one party's database and poll a
 * query across every party until it settles on an expected value.
 */
import { PreflightError } from "./errors";
import { kmsConnectorDbName, kmsPartyIds } from "./kms-party";
import { uint256LeHex } from "./kms-onchain";
import { COPROCESSOR_DB_CONTAINER, DEFAULT_POSTGRES_PASSWORD, DEFAULT_POSTGRES_USER } from "./layout";
import { run } from "./utils/process";

/** Default bound for the connector to ingest an event, forward it to KMS Core, and settle. */
const CONNECTOR_SETTLE_TIMEOUT_MS = 240_000;
const CONNECTOR_POLL_MS = 2_000;

const connectorDbRuntime = () => ({
  container: process.env.POSTGRES_CONTAINER ?? COPROCESSOR_DB_CONTAINER,
  user: process.env.POSTGRES_USER ?? DEFAULT_POSTGRES_USER,
  password: process.env.POSTGRES_PASSWORD ?? DEFAULT_POSTGRES_PASSWORD,
});

/** Runs a scalar query against one party's kms-connector database. */
const connectorQuery = async (dbName: string, sql: string) => {
  const db = connectorDbRuntime();
  const result = await run([
    "docker", "exec", "-e", `PGPASSWORD=${db.password}`, db.container,
    "psql", "-U", db.user, "-d", dbName, "-t", "-A", "-c", sql,
  ]);
  return result.stdout.trim();
};

/** A bytea literal for a U256 id in the little-endian form the connector stores. */
const byteaLiteral = (id: bigint) => `decode('${uint256LeHex(id)}','hex')`;

/** A column's value as text, or 'missing' when the row does not exist. Booleans read as 't'/'f'. */
export const columnQuery = (table: string, idColumn: string, column: string, id: bigint) =>
  `SELECT COALESCE((SELECT ${column}::text FROM ${table} WHERE ${idColumn} = ${byteaLiteral(id)}), 'missing')`;

/** Polls one query in every party's connector DB until it returns one of `want` everywhere. */
export const pollConnectors = async (
  parties: number,
  label: string,
  sql: string,
  want: string[],
  opts?: { timeoutMs?: number; pollMs?: number },
) => {
  const timeoutMs = opts?.timeoutMs ?? CONNECTOR_SETTLE_TIMEOUT_MS;
  const pollMs = opts?.pollMs ?? CONNECTOR_POLL_MS;
  const finals: string[] = [];
  for (const party of kmsPartyIds(parties)) {
    const dbName = kmsConnectorDbName(party);
    const deadline = Date.now() + timeoutMs;
    let last = await connectorQuery(dbName, sql);
    while (!want.includes(last)) {
      if (Date.now() >= deadline) {
        throw new PreflightError(
          `${label}: db "${dbName}" returned ${JSON.stringify(last)} (wanted one of ${want.join("/")}) after ${timeoutMs / 1000}s — query: ${sql}`,
        );
      }
      await Bun.sleep(pollMs);
      last = await connectorQuery(dbName, sql);
    }
    finals.push(last);
  }
  console.log(`[connector] check OK on ${parties} db(s): ${label} -> ${finals.join(", ")}`);
};
