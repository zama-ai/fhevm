/**
 * Smoke-tests legacy coprocessor images against the CLI's rendered runtime commands to catch compatibility regressions.
 */
import { mkdir, rm } from "node:fs/promises";
import path from "node:path";

import { COMPONENTS, STATE_DIR, TEMPLATE_ENV_DIR, versionsEnvPath, dockerArgs, envPath } from "./layout";
import { generateComposeOverrides, type ComposeDoc } from "./render-compose";
import { renderEnvMaps, type WalletMaterial } from "./render-env";
import { runtimePlanForState } from "./runtime-plan";
import { composeEnv, run } from "./shell";
import type { State } from "./types";
import { readEnvFile, readJson, writeEnvFile } from "./utils";

const COMPAT_DOC = "test-suite/fhevm/COMPAT.md";
const LEGACY_START_TIMEOUT_MS = 3_000;
const PARSE_ERROR = /(unexpected argument|unexpected value|required arguments were not provided|unrecognized option|missing .*argument)/i;
const STARTUP_ERROR =
  /(connection refused|timed out|dns|network|database|postgres|tcp|websocket|transport|provider|rpc|lookup address information|name or service not known)/i;

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

const fakeWallet: WalletMaterial = {
  address: "0x0000000000000000000000000000000000000007",
  privateKey: "0x0000000000000000000000000000000000000000000000000000000000000007",
};

const fakeDiscovery: NonNullable<State["discovery"]> = {
  gateway: {
    DECRYPTION_ADDRESS: "0x0000000000000000000000000000000000000001",
    INPUT_VERIFICATION_ADDRESS: "0x0000000000000000000000000000000000000003",
    CIPHERTEXT_COMMITS_ADDRESS: "0x0000000000000000000000000000000000000004",
    GATEWAY_CONFIG_ADDRESS: "0x0000000000000000000000000000000000000005",
    KMS_GENERATION_ADDRESS: "0x0000000000000000000000000000000000000006",
    MULTICHAIN_ACL_ADDRESS: "0x0000000000000000000000000000000000000008",
    PROTOCOL_PAYMENT_ADDRESS: "0x0000000000000000000000000000000000000009",
  },
  host: {
    ACL_CONTRACT_ADDRESS: "0x0000000000000000000000000000000000000010",
    PAUSER_SET_CONTRACT_ADDRESS: "0x0000000000000000000000000000000000000011",
    FHEVM_EXECUTOR_CONTRACT_ADDRESS: "0x0000000000000000000000000000000000000002",
    INPUT_VERIFIER_CONTRACT_ADDRESS: "0x0000000000000000000000000000000000000012",
    KMS_VERIFIER_CONTRACT_ADDRESS: "0x0000000000000000000000000000000000000013",
  },
  kmsSigner: "0x0000000000000000000000000000000000000014",
  fheKeyId: "0000000000000000000000000000000000000000000000000000000000000001",
  crsKeyId: "0000000000000000000000000000000000000000000000000000000000000002",
  actualFheKeyId: "0000000000000000000000000000000000000000000000000000000000000001",
  actualCrsKeyId: "0000000000000000000000000000000000000000000000000000000000000002",
  minioKeyPrefix: "PUB",
  endpoints: {
    gatewayHttp: "http://localhost:8546",
    gatewayWs: "ws://127.0.0.1:1",
    hostHttp: "http://localhost:8545",
    hostWs: "ws://127.0.0.1:1",
    minioInternal: "http://127.0.0.1:9000",
    minioExternal: "http://127.0.0.1:9000",
  },
};

const loadLegacyCompose = async () => {
  await rm(STATE_DIR, { recursive: true, force: true });
  await mkdir(path.dirname(versionsEnvPath), { recursive: true });
  const runtimeState = { ...state, discovery: fakeDiscovery };
  const plan = runtimePlanForState(runtimeState);
  const templateEnvs = Object.fromEntries(
    await Promise.all(
      COMPONENTS.map(async (component) => [
        component,
        await readEnvFile(path.join(TEMPLATE_ENV_DIR, `.env.${component}`)),
      ]),
    ),
  ) as Record<string, Record<string, string>>;
  const rendered = await renderEnvMaps(
    runtimeState,
    plan,
    templateEnvs,
    async () => fakeWallet,
  );
  await Promise.all([
    ...COMPONENTS.map((component) => writeEnvFile(envPath(component), rendered.componentEnvs[component])),
    ...Object.entries(rendered.instanceEnvs).map(([name, env]) => writeEnvFile(envPath(name), env)),
    writeEnvFile(versionsEnvPath, rendered.versionsEnv),
  ]);
  await generateComposeOverrides(runtimeState, plan);
  const { stdout } = await run([...dockerArgs("coprocessor"), "config", "--format", "json"], {
    env: await composeEnv("coprocessor"),
  });
  return JSON.parse(stdout) as ComposeDoc;
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
