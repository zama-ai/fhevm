import { PreflightError } from "../errors";
import { kmsConnectorDbName, kmsPartyIds } from "../kms-party";
import {
  COPROCESSOR_DB_CONTAINER,
  DEFAULT_POSTGRES_PASSWORD,
  DEFAULT_POSTGRES_USER,
} from "../layout";
import type { State } from "../types";
import { run } from "../utils/process";

const RESPONSE_TIMEOUT_MS = 60_000;
const POLL_INTERVAL_MS = 1_000;

export type KmsResponseVersion = "v0" | "v1";

export type UserDecryptionResponse = {
  decryptionId: string;
  extraData: string;
  nodeId: number;
  status: string;
};

const connectorDbRuntime = () => ({
  container: process.env.POSTGRES_CONTAINER ?? COPROCESSOR_DB_CONTAINER,
  user: process.env.POSTGRES_USER ?? DEFAULT_POSTGRES_USER,
  password: process.env.POSTGRES_PASSWORD ?? DEFAULT_POSTGRES_PASSWORD,
});

export const contextIdToConnectorHex = (contextId: string) => {
  if (!/^0x[0-9a-f]{64}$/i.test(contextId)) {
    throw new PreflightError(`KMS context ID must be a 32-byte hex value; received ${JSON.stringify(contextId)}`);
  }
  return contextId.slice(2).match(/../g)!.reverse().join("").toLowerCase();
};

export const registerKmsContext = async (state: State, contextId: string): Promise<string[]> => {
  if (state.scenario.kms.mode !== "threshold") {
    throw new PreflightError("registerKmsContext requires a running threshold KMS cluster");
  }
  const db = connectorDbRuntime();
  const dbNames = kmsPartyIds(state.scenario.kms.committeeSize).map(kmsConnectorDbName);
  const id = contextIdToConnectorHex(contextId);
  const sql =
    `INSERT INTO kms_context(id, is_valid, created_at, updated_at) ` +
    `VALUES (decode('${id}','hex'), true, NOW(), NOW()) ` +
    `ON CONFLICT (id) DO UPDATE SET is_valid = true, updated_at = NOW()`;
  for (const dbName of dbNames) {
    await run([
      "docker",
      "exec",
      "-e",
      `PGPASSWORD=${db.password}`,
      db.container,
      "psql",
      "-U",
      db.user,
      "-d",
      dbName,
      "-v",
      "ON_ERROR_STOP=1",
      "-c",
      sql,
    ]);
  }
  return dbNames;
};

const latestResponse = async (nodeId: number): Promise<UserDecryptionResponse | undefined> => {
  const db = connectorDbRuntime();
  const result = await run([
    "docker",
    "exec",
    "-e",
    `PGPASSWORD=${db.password}`,
    db.container,
    "psql",
    "-U",
    db.user,
    "-d",
    kmsConnectorDbName(nodeId),
    "-t",
    "-A",
    "-F",
    "|",
    "-c",
    "SELECT encode(decryption_id, 'hex'), status::text, encode(extra_data, 'hex') FROM user_decryption_responses ORDER BY created_at DESC LIMIT 1",
  ]);
  const line = result.stdout.trim();
  if (!line) {
    return undefined;
  }
  const [decryptionId, status, extraData, ...rest] = line.split("|");
  if (!decryptionId || !status || extraData === undefined || rest.length > 0) {
    throw new PreflightError(
      `could not parse latest user-decryption response from ${kmsConnectorDbName(nodeId)}: ${JSON.stringify(line)}`,
    );
  }
  return { decryptionId, status, extraData: `0x${extraData}`, nodeId };
};

export const snapshotUserDecryptionResponseIds = async (state: State) =>
  Promise.all(kmsPartyIds(state.scenario.kms.committeeSize).map(async (nodeId) => (await latestResponse(nodeId))?.decryptionId));

export const waitForUserDecryptionResponses = async (
  state: State,
  previousIds: Array<string | undefined>,
): Promise<UserDecryptionResponse[]> => {
  const nodeIds = kmsPartyIds(state.scenario.kms.committeeSize);
  const deadline = Date.now() + RESPONSE_TIMEOUT_MS;
  let responses: Array<UserDecryptionResponse | undefined> = [];
  while (Date.now() < deadline) {
    responses = await Promise.all(nodeIds.map(latestResponse));
    const fresh = responses.every(
      (response, index) =>
        response && response.decryptionId !== previousIds[index] && response.status === "completed",
    );
    const ids = new Set(responses.map((response) => response?.decryptionId).filter(Boolean));
    if (fresh && ids.size === 1) {
      return responses as UserDecryptionResponse[];
    }
    await Bun.sleep(POLL_INTERVAL_MS);
  }
  throw new PreflightError(
    `timed out waiting for one fresh completed user-decryption response from every KMS node; observed ${responses
      .map((response) =>
        response
          ? `node ${response.nodeId}: ${response.decryptionId.slice(0, 12)}… (${response.status})`
          : "missing",
      )
      .join(", ")}`,
  );
};

export const responseVersion = (extraData: string): KmsResponseVersion | undefined => {
  if (extraData === "0x") {
    return "v0";
  }
  return /^0x01[0-9a-f]{64}$/i.test(extraData) ? "v1" : undefined;
};
