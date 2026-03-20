import { mkdir, readFile, rm, writeFile } from "node:fs/promises";
import path from "node:path";

import YAML from "yaml";

import { composePath, envPath, STATE_DIR } from "./layout";
import { generateComposeOverrides, interpolateString, type ComposeDoc } from "./render-compose";
import { runtimePlanForState } from "./runtime-plan";
import type { State } from "./types";
import { readJson } from "./utils";

const COMPAT_DOC = "test-suite/fhevm/COMPAT.md";
const LEGACY_START_TIMEOUT_MS = 3_000;
const PARSE_ERROR = /(unexpected argument|unexpected value|required arguments were not provided|unrecognized option|missing .*argument)/i;
const STARTUP_ERROR = /(connection refused|timed out|dns|network|database|postgres|tcp|websocket|transport|provider|rpc)/i;

const defaultScenario: State["scenario"] = {
  version: 1,
  kind: "coprocessor-consensus",
  origin: "default",
  topology: { count: 1, threshold: 1 },
  instances: [{ index: 0, source: { mode: "inherit" }, env: {}, args: {} }],
};

const latestSupported = await readJson<Pick<State["versions"], "target" | "lockName" | "env" | "sources">>(
  path.join(import.meta.dir, "..", "profiles", "latest-supported.json"),
);

const state: State = {
  target: "latest-supported",
  lockPath: "/tmp/latest-supported.json",
  requiresGitHub: false,
  versions: latestSupported,
  overrides: [],
  scenario: defaultScenario,
  completedSteps: [],
  updatedAt: "2026-03-20T00:00:00.000Z",
};

const compatFailure = (message: string) =>
  `${message}\nRead ${COMPAT_DOC}.\nEither add/update a shim in src/compat.ts or intentionally raise the support floor in src/resolve.ts.`;

const requiredEnv = {
  DATABASE_URL: "postgresql://127.0.0.1:1/coprocessor",
  ACL_CONTRACT_ADDRESS: "0x0000000000000000000000000000000000000001",
  FHEVM_EXECUTOR_CONTRACT_ADDRESS: "0x0000000000000000000000000000000000000002",
  RPC_WS_URL: "ws://127.0.0.1:1",
  GATEWAY_WS_URL: "ws://127.0.0.1:1",
  INPUT_VERIFICATION_ADDRESS: "0x0000000000000000000000000000000000000003",
  CIPHERTEXT_COMMITS_ADDRESS: "0x0000000000000000000000000000000000000004",
  GATEWAY_CONFIG_ADDRESS: "0x0000000000000000000000000000000000000005",
  KMS_GENERATION_ADDRESS: "0x0000000000000000000000000000000000000006",
  TX_SENDER_PRIVATE_KEY:
    "0x0000000000000000000000000000000000000000000000000000000000000007",
  MULTICHAIN_ACL_ADDRESS: "0x0000000000000000000000000000000000000008",
  COPROCESSOR_API_KEY: "00000000-0000-0000-0000-000000000000",
};

const interpolateValue = (value: unknown, vars: Record<string, string>): unknown => {
  if (typeof value === "string") {
    return interpolateString(value, vars);
  }
  if (Array.isArray(value)) {
    return value.map((item) => interpolateValue(item, vars));
  }
  if (!value || typeof value !== "object") {
    return value;
  }
  return Object.fromEntries(
    Object.entries(value).map(([key, item]) => [key, interpolateValue(item, vars)]),
  );
};

const loadLegacyCompose = async () => {
  await rm(STATE_DIR, { recursive: true, force: true });
  await mkdir(path.dirname(envPath("coprocessor")), { recursive: true });
  await writeFile(
    envPath("coprocessor"),
    `${Object.entries(requiredEnv)
      .map(([key, value]) => `${key}=${value}`)
      .join("\n")}\n`,
  );
  await generateComposeOverrides(state, runtimePlanForState(state));
  const doc = YAML.parse(await readFile(composePath("coprocessor"), "utf8")) as ComposeDoc;
  return interpolateValue(doc, { ...state.versions.env, ...requiredEnv }) as ComposeDoc;
};

const runLegacyService = async (name: string, image: string, command: string[]) => {
  const argv = ["docker", "run", "--rm", image, ...command];
  const proc = Bun.spawn(argv, { stdout: "pipe", stderr: "pipe" });
  const deadline = new Promise<"timeout">((resolve) =>
    setTimeout(() => {
      proc.kill("SIGKILL");
      resolve("timeout");
    }, LEGACY_START_TIMEOUT_MS),
  );
  const result = await Promise.race([proc.exited, deadline]);
  const [stdout, stderr] = await Promise.all([
    proc.stdout ? new Response(proc.stdout).text() : "",
    proc.stderr ? new Response(proc.stderr).text() : "",
  ]);
  const output = `${stdout}\n${stderr}`.trim();
  if (result === "timeout") {
    return;
  }
  if (PARSE_ERROR.test(output)) {
    throw new Error(
      compatFailure(`${name} rejected the rendered legacy command.\n${argv.join(" ")}\n${output}`),
    );
  }
  if (Number(result) !== 0 && STARTUP_ERROR.test(output)) {
    return;
  }
  if (Number(result) === 0) {
    return;
  }
  throw new Error(
    compatFailure(`${name} failed before reaching a normal startup path.\n${argv.join(" ")}\n${output}`),
  );
};

const main = async () => {
  const doc = await loadLegacyCompose();
  try {
    for (const serviceName of [
      "coprocessor-host-listener",
      "coprocessor-gw-listener",
      "coprocessor-transaction-sender",
    ]) {
      const service = doc.services[serviceName] as { image?: string; command?: unknown } | undefined;
      if (!service?.image || !Array.isArray(service.command)) {
        throw new Error(compatFailure(`Missing rendered legacy service definition for ${serviceName}.`));
      }
      await runLegacyService(
        serviceName,
        String(service.image),
        service.command.map((value) => String(value)),
      );
    }
    console.log("compat smoke passed");
  } finally {
    await rm(STATE_DIR, { recursive: true, force: true });
  }
};

await main().catch((error) => {
  console.error(error instanceof Error ? error.message : String(error));
  process.exit(1);
});
