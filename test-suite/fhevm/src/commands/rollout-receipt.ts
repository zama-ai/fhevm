import fs from "node:fs/promises";
import path from "node:path";

import { PreflightError } from "../errors";
import { projectContainers } from "../flow/runtime-compose";
import { kmsConnectorDbName } from "../kms-party";
import { STATE_DIR } from "../layout";
import type { VersionBundle } from "../types";
import { ensureDir, readJson } from "../utils/fs";
import { run } from "../utils/process";

type VersionChange = { key: string; from?: string; to?: string };
type ReceiptContainer = {
  name: string;
  service?: string;
  image: string;
  imageId: string;
  state: string;
  health?: string;
};

type ReceiptEntry = {
  seq: number;
  at: string;
  kind: string;
  title: string;
  details?: Record<string, unknown>;
  lockFile?: string;
  versionChanges?: VersionChange[];
  containers?: ReceiptContainer[];
  dockerInspectError?: string;
  diagnostics?: DiagnosticSection[];
};

type DockerInspect = {
  Name?: string;
  Config?: {
    Image?: string;
    Labels?: Record<string, string>;
  };
  Image?: string;
  State?: {
    Status?: string;
    Health?: {
      Status?: string;
    };
  };
};

type DiagnosticSection = {
  title: string;
  command: string;
  output: string;
  error?: string;
};

type StateHashSnapshot = {
  database: string;
  command: string;
  rows: string[];
  error?: string;
};

const receiptDir = () => path.join(STATE_DIR, "rollout");
export const receiptJsonlPath = () => path.join(receiptDir(), "receipt.jsonl");
export const receiptMarkdownPath = () => path.join(receiptDir(), "receipt.md");

const versionChanges = (previous: Record<string, string> | undefined, next: Record<string, string>) =>
  Object.keys(next)
    .sort()
    .filter((key) => previous?.[key] !== next[key])
    .map((key) => ({ key, from: previous?.[key], to: next[key] }));

type InspectResult = { containers: ReceiptContainer[]; error?: string };

export const requireDockerSnapshot = (snapshot: InspectResult) => {
  if (snapshot.error) {
    throw new PreflightError(`Required Docker snapshot failed: ${snapshot.error}`);
  }
  if (!snapshot.containers.length) {
    throw new PreflightError("Required Docker snapshot contained no project containers");
  }
};

const inspectFailed = (error: string): InspectResult => {
  console.warn(`[receipt] docker inspect failed: ${error}`);
  return { containers: [], error };
};

const inspectContainers = async (): Promise<InspectResult> => {
  try {
    const names = await projectContainers(true);
    if (!names.length) {
      return { containers: [] };
    }
    const inspected = await run(["docker", "inspect", ...names], { allowFailure: true });
    if (inspected.code !== 0) {
      return inspectFailed((inspected.stderr || inspected.stdout).trim() || "docker inspect failed");
    }
    const values = JSON.parse(inspected.stdout) as DockerInspect[];
    return {
      containers: values
        .map((value) => ({
          name: value.Name?.replace(/^\//, "") ?? "",
          service: value.Config?.Labels?.["com.docker.compose.service"],
          image: value.Config?.Image ?? "",
          imageId: value.Image ?? "",
          state: value.State?.Status ?? "",
          health: value.State?.Health?.Status,
        }))
        .sort((a, b) => (a.service ?? a.name).localeCompare(b.service ?? b.name)),
    };
  } catch (error) {
    return inspectFailed(error instanceof Error ? error.message : String(error));
  }
};

const psql = async (container: string, database: string, sql: string): Promise<DiagnosticSection> => {
  const command = `docker exec ${container} psql -U postgres -d ${database} -v ON_ERROR_STOP=0 -P pager=off -c ${JSON.stringify(sql)}`;
  const result = await run(
    ["docker", "exec", container, "psql", "-U", "postgres", "-d", database, "-v", "ON_ERROR_STOP=0", "-P", "pager=off", "-c", sql],
    { allowFailure: true },
  );
  return {
    title: `${container}/${database}`,
    command,
    output: (result.stdout || "").trim(),
    error: result.code === 0 ? undefined : (result.stderr || result.stdout).trim() || `psql exited ${result.code}`,
  };
};

export const diagnosticLogArgs = (container: string) => [
  "docker",
  "logs",
  "--since",
  "1h",
  "--tail",
  "10000",
  container,
];

/** Bounds a container's logs for the receipt, dropping the OTEL exporter retry spam the
 * local stack emits with no collector attached — hence fetching more lines than we keep,
 * so what survives the filter is `maxLines` of signal rather than of noise. */
export const diagnosticLogOutput = (stdout: string, stderr: string, maxLines = 2000) => {
  const lines = [stdout, stderr]
    .filter(Boolean)
    .join("\n")
    .split(/\r?\n/)
    .filter((line) => !line.includes("BatchSpanProcessor.ExportError"));
  if (lines.length <= maxLines) {
    return lines.join("\n").trim();
  }
  const head = Math.floor(maxLines / 2);
  const tail = maxLines - head;
  return [
    ...lines.slice(0, head),
    `[receipt] ${lines.length - maxLines} diagnostic log lines omitted`,
    ...lines.slice(-tail),
  ].join("\n").trim();
};

const containerLogs = async (container: string): Promise<DiagnosticSection> => {
  const args = diagnosticLogArgs(container);
  const command = args.join(" ");
  const result = await run(args, { allowFailure: true });
  return {
    title: `${container} logs`,
    command,
    output: diagnosticLogOutput(result.stdout, result.stderr),
    error: result.code === 0 ? undefined : (result.stderr || result.stdout).trim() || `docker logs exited ${result.code}`,
  };
};

const stateHashSnapshot = async (database: string): Promise<StateHashSnapshot> => {
  const sql = `select chain_id, block_number, state_hash, s3_uploaded_at is not null
from "gcs-0.15.0".state_hash
order by chain_id, block_number;`;
  const args = [
    "docker",
    "exec",
    "coprocessor-and-kms-db",
    "psql",
    "-U",
    "postgres",
    "-d",
    database,
    "-At",
    "-F",
    "|",
    "-c",
    sql,
  ];
  const result = await run(args, { allowFailure: true });
  return {
    database,
    command: args.join(" "),
    rows: result.code === 0 ? result.stdout.trim().split(/\r?\n/).filter(Boolean) : [],
    error: result.code === 0 ? undefined : (result.stderr || result.stdout).trim() || `psql exited ${result.code}`,
  };
};

export const formatStateHashComparison = (snapshots: Array<Pick<StateHashSnapshot, "database" | "rows" | "error">>) => {
  const readable = snapshots.filter((snapshot) => !snapshot.error);
  if (readable.length < 2) {
    return `comparison unavailable: ${readable.length} operator database(s) readable`;
  }
  const hashes = new Map<string, Map<string, string>>();
  for (const snapshot of readable) {
    for (const row of snapshot.rows) {
      const [chainId, blockNumber, stateHash, uploaded] = row.split("|");
      const key = `${chainId}:${blockNumber}`;
      const byDatabase = hashes.get(key) ?? new Map<string, string>();
      byDatabase.set(snapshot.database, `${stateHash}|uploaded=${uploaded}`);
      hashes.set(key, byDatabase);
    }
  }
  const mismatches = [...hashes.entries()].filter(([, byDatabase]) => {
    const values = readable.map((snapshot) => byDatabase.get(snapshot.database));
    return values.some((value) => value === undefined) || new Set(values).size !== 1;
  });
  const summary = `${hashes.size} anchor(s) compared across ${readable.map(({ database }) => database).join(", ")}; ${mismatches.length} mismatch(es)`;
  if (!mismatches.length) {
    return summary;
  }
  return [
    summary,
    ...mismatches.map(([key, byDatabase]) =>
      `${key} ${readable.map(({ database }) => `${database}=${byDatabase.get(database) ?? "missing"}`).join(" ")}`,
    ),
  ].join("\n");
};

const diagnosticSql = {
  relayer: `
select ext_job_id, req_status, err_reason, accepted, created_at, updated_at
from input_proof_req
order by updated_at desc
limit 20;

select relname as table_name, n_live_tup as estimated_rows
from pg_stat_user_tables
where relname ilike '%decrypt%'
   or relname ilike '%request%'
   or relname ilike '%response%'
order by relname;
`,
  coprocessor: `
select table_name
from information_schema.tables
where table_schema = 'public'
  and (table_name ilike '%proof%' or table_name ilike '%ciphertext%' or table_name ilike '%transaction%')
order by table_name;

select schemaname as table_schema, relname as table_name, n_live_tup as estimated_rows
from pg_stat_user_tables
where relname ilike '%proof%'
   or relname ilike '%ciphertext%'
   or relname ilike '%transaction%'
order by schemaname, relname;

select stack_role, host_chain_id, state, status, encode(proposal_id, 'hex') as proposal_id,
       proposal_block, version, start_block, end_block, gw_start_block,
       gw_dry_run_started, host_consensus_reached, gw_consensus_reached,
       last_error, updated_at
from upgrade_state
order by stack_role, host_chain_id;

select 'public' as table_schema, chain_id, block_number, state_hash, s3_uploaded_at
from public.state_hash
order by chain_id, block_number;

select 'gcs-0.15.0' as table_schema, chain_id, block_number, state_hash, s3_uploaded_at
from "gcs-0.15.0".state_hash
order by chain_id, block_number;
`,
  kmsConnector: `
select 'prep_keygen_requests' as table_name, status, count(*) from prep_keygen_requests group by status
union all
select 'prep_keygen_responses', status, count(*) from prep_keygen_responses group by status
union all
select 'keygen_requests', status, count(*) from keygen_requests group by status
union all
select 'keygen_responses', status, count(*) from keygen_responses group by status
union all
select 'crsgen_requests', status, count(*) from crsgen_requests group by status
union all
select 'crsgen_responses', status, count(*) from crsgen_responses group by status
union all
select 'public_decryption_requests', status, count(*) from public_decryption_requests group by status
union all
select 'public_decryption_responses', status, count(*) from public_decryption_responses group by status
union all
select 'user_decryption_requests', status, count(*) from user_decryption_requests group by status
union all
select 'user_decryption_responses', status, count(*) from user_decryption_responses group by status
order by table_name, status;

select 'public_decryption_requests' as table_name, encode(decryption_id, 'hex') as decryption_id,
       status, created_at, updated_at
from public_decryption_requests
order by updated_at desc
limit 10;

select 'public_decryption_responses' as table_name, encode(decryption_id, 'hex') as decryption_id,
       status, created_at, updated_at
from public_decryption_responses
order by updated_at desc
limit 10;

select 'user_decryption_requests' as table_name, encode(decryption_id, 'hex') as decryption_id,
       status, created_at, updated_at
from user_decryption_requests
order by updated_at desc
limit 10;

select 'user_decryption_responses' as table_name, encode(decryption_id, 'hex') as decryption_id,
       status, created_at, updated_at
from user_decryption_responses
order by updated_at desc
limit 10;

select * from last_block_polled order by 1;

select * from last_block_polled_by_chain order by 1;
`,
};

export const kmsConnectorPartyIds = (containerNames: string[]) =>
  [
    ...new Set(
      containerNames.flatMap((name) => {
        const match = /^kms-connector(?:-(\d+))?-db-migration$/.exec(name);
        return match ? [match[1] ? Number(match[1]) : 1] : [];
      }),
    ),
  ].sort((a, b) => a - b);

export const failureDiagnosticContainerNames = (containers: ReceiptContainer[]) =>
  containers
    .map((container) => container.name)
    .filter(
      (name) =>
        /^kms-core(?:-|$)/.test(name) ||
        /^kms-connector(?:-|$)/.test(name) ||
        name === "fhevm-relayer" ||
        /^coprocessor\d*(?:-gcs)?-(?:(?:tfhe|zkproof|sns)-worker|consensus-detector|upgrade-controller)$/.test(name),
    );

const collectFailureDiagnostics = async (containers: ReceiptContainer[]) => {
  const sections: DiagnosticSection[] = [];
  const diagnosticContainers = failureDiagnosticContainerNames(containers);
  sections.push(...(await Promise.all(diagnosticContainers.map(containerLogs))));
  const connectorParties = kmsConnectorPartyIds(containers.map((container) => container.name));
  for (const party of connectorParties) {
    sections.push(await psql("coprocessor-and-kms-db", kmsConnectorDbName(party), diagnosticSql.kmsConnector));
  }
  sections.push(await psql("fhevm-relayer-db", "relayer_db", diagnosticSql.relayer));
  const coprocessorDatabases = ["coprocessor", "coprocessor_1", "coprocessor_2"];
  for (const database of coprocessorDatabases) {
    sections.push(await psql("coprocessor-and-kms-db", database, diagnosticSql.coprocessor));
  }
  const stateHashes = await Promise.all(coprocessorDatabases.map(stateHashSnapshot));
  sections.push({
    title: "Blue/Green GCS state-hash comparison",
    command: stateHashes.map(({ command }) => command).join("\n"),
    output: formatStateHashComparison(stateHashes),
    error:
      stateHashes
        .filter(({ error }) => error)
        .map(({ database, error }) => `${database}: ${error}`)
        .join("\n") || undefined,
  });
  return sections;
};

const mdEscape = (value: unknown) =>
  String(value ?? "")
    .replaceAll("|", "\\|")
    .replace(/\r?\n/g, " ");
const compact = (value: string) => (value.length > 120 ? `${value.slice(0, 117)}...` : value);

const markdownEntry = (entry: ReceiptEntry) => {
  const lines = [`\n## ${entry.seq}. ${entry.kind}: ${entry.title}`, `- at: ${entry.at}`];
  if (entry.lockFile) {
    lines.push(`- lock: \`${entry.lockFile}\``);
  }
  if (entry.versionChanges?.length) {
    lines.push("", "Version changes:", "", "| key | from | to |", "| --- | --- | --- |");
    for (const change of entry.versionChanges) {
      lines.push(`| \`${change.key}\` | \`${mdEscape(change.from ?? "(unset)")}\` | \`${mdEscape(change.to)}\` |`);
    }
  }
  if (entry.details && Object.keys(entry.details).length) {
    lines.push("", "Details:", "");
    for (const [key, value] of Object.entries(entry.details)) {
      lines.push(`- ${key}: \`${mdEscape(Array.isArray(value) ? value.join(", ") : value)}\``);
    }
  }
  if (entry.dockerInspectError) {
    lines.push("", `Docker inspect failed: \`${mdEscape(entry.dockerInspectError)}\``);
  }
  if (entry.containers?.length) {
    lines.push(
      "",
      "Docker state after action:",
      "",
      "| service | container | image | image id | state |",
      "| --- | --- | --- | --- | --- |",
    );
    for (const container of entry.containers) {
      lines.push(
        `| ${mdEscape(container.service ?? "")} | ${mdEscape(container.name)} | \`${mdEscape(compact(container.image))}\` | \`${mdEscape(container.imageId)}\` | ${mdEscape(container.health ? `${container.state}/${container.health}` : container.state)} |`,
      );
    }
  }
  if (entry.diagnostics?.length) {
    lines.push("", "Diagnostics:");
    for (const item of entry.diagnostics) {
      lines.push("", `### ${item.title}`, "", "```text", item.command);
      if (item.output) {
        lines.push("", item.output);
      }
      if (item.error) {
        lines.push("", `[error] ${item.error}`);
      }
      lines.push("```");
    }
  }
  return `${lines.join("\n")}\n`;
};

export const createRolloutReceipt = (
  operations: { inspectContainers?: typeof inspectContainers } = {},
) => {
  let seq = 0;
  let started = false;
  let currentEnv: Record<string, string> | undefined;

  const start = async (script: string) => {
    await ensureDir(receiptDir());
    started = true;
    seq = 0;
    currentEnv = undefined;
    await fs.writeFile(receiptJsonlPath(), "");
    await fs.writeFile(
      receiptMarkdownPath(),
      [`# Stateful Rollout Receipt`, ``, `- runbook: \`${script}\``, `- started: ${new Date().toISOString()}`, ""].join(
        "\n",
      ),
    );
    console.log(`[receipt] writing ${receiptMarkdownPath()}`);
  };

  const record = async (
    kind: string,
    title: string,
    options: {
      details?: Record<string, unknown>;
      docker?: boolean;
      diagnostics?: boolean;
      lockFile?: string;
    } = {},
  ) => {
    if (!started) {
      await start("(unknown)");
    }

    const lock = options.lockFile ? await readJson<VersionBundle>(options.lockFile) : undefined;
    const changes = lock ? versionChanges(currentEnv, lock.env) : undefined;
    if (lock) {
      currentEnv = lock.env;
    }
    const docker = options.docker || options.diagnostics
      ? await (operations.inspectContainers ?? inspectContainers)()
      : undefined;
    const diagnostics = options.diagnostics ? await collectFailureDiagnostics(docker?.containers ?? []) : undefined;
    const entry: ReceiptEntry = {
      seq: ++seq,
      at: new Date().toISOString(),
      kind,
      title,
      details: options.details,
      lockFile: options.lockFile,
      versionChanges: changes,
      containers: docker?.containers,
      dockerInspectError: docker?.error,
      diagnostics,
    };
    await fs.appendFile(receiptJsonlPath(), `${JSON.stringify(entry)}\n`);
    await fs.appendFile(receiptMarkdownPath(), markdownEntry(entry));
    console.log(`[receipt] ${entry.seq}. ${kind}: ${title}`);
    if (options.docker && docker) {
      requireDockerSnapshot(docker);
    }
  };

  return { record, start };
};

export type RolloutReceipt = ReturnType<typeof createRolloutReceipt>;

export const printRolloutReceipt = async () => {
  const file = receiptMarkdownPath();
  try {
    process.stdout.write(await fs.readFile(file, "utf8"));
  } catch (error) {
    if ((error as NodeJS.ErrnoException).code === "ENOENT") {
      throw new PreflightError(`No rollout receipt found at ${file}`);
    }
    throw error;
  }
};
