import fs from "node:fs/promises";
import path from "node:path";

import { PreflightError } from "../errors";
import { projectContainers } from "../flow/runtime-compose";
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

const diagnosticSql = {
  relayer: `
select ext_job_id, req_status, err_reason, accepted, created_at, updated_at
from input_proof_req
order by updated_at desc
limit 20;
`,
  coprocessor: `
select table_name
from information_schema.tables
where table_schema = 'public'
  and (table_name ilike '%proof%' or table_name ilike '%ciphertext%' or table_name ilike '%transaction%')
order by table_name;

select relname as table_name, n_live_tup as estimated_rows
from pg_stat_user_tables
where relname ilike '%proof%'
   or relname ilike '%ciphertext%'
   or relname ilike '%transaction%'
order by relname;
`,
};

const collectFailureDiagnostics = async () => {
  const sections: DiagnosticSection[] = [];
  sections.push(await psql("fhevm-relayer-db", "relayer_db", diagnosticSql.relayer));
  for (const database of ["coprocessor", "coprocessor_1", "coprocessor_2"]) {
    sections.push(await psql("coprocessor-and-kms-db", database, diagnosticSql.coprocessor));
  }
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
    const diagnostics = options.diagnostics ? await collectFailureDiagnostics() : undefined;
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
