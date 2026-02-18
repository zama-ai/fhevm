#!/usr/bin/env bun

import fs from "node:fs";
import path from "node:path";
import net from "node:net";
import dgram from "node:dgram";
import { createCommandHandlers } from "./commands";
import { createDeployOptionHandlers } from "./deploy-options";
import {
  COLORS,
  COPROCESSOR_ACCOUNT_INDICES,
  CORE_VERSION_OVERRIDE_ENV,
  DEFAULT_OTEL_EXPORTER_OTLP_ENDPOINT,
  DEPLOYMENT_STEPS,
  LOCAL_CACHE_SERVICES,
  MAX_LOCAL_COPROCESSORS,
  PROJECT,
  RELAYER_VERSION_OVERRIDE_ENV,
  STACK_VERSION_OVERRIDE_ENV,
  TEST_TYPE_CONFIG,
  VERSION_ENTRIES,
  type DeploymentStep,
  type ServiceState,
} from "./manifest";
import { createNetworkProfileHandlers } from "./network-profile";
import { createDeployEnvHandlers } from "./deploy-env";
import { createLocalBuildHandlers } from "./local-build";
import { createDeployRuntimeHandlers } from "./deploy-runtime";
import { runCommand, sleep } from "./process";
import type { DeployOptions, HostPortSpec } from "./types";

const BUN_DIR = import.meta.dir;
const FHEVM_DIR = path.resolve(BUN_DIR, "..", "..");
const SCRIPTS_DIR = path.resolve(FHEVM_DIR, "scripts");
const COMPOSE_DIR = path.resolve(FHEVM_DIR, "docker-compose");
const ENV_DIR = path.resolve(FHEVM_DIR, "env", "staging");
const CONFIG_DIR = path.resolve(FHEVM_DIR, "config");
const ACTIVE_VERSIONS_FILE = path.resolve(ENV_DIR, ".env.versions.active");
const NETWORK_VERSION_CACHE_FILE = path.resolve(FHEVM_DIR, ".cache", "network-profile-versions.json");
const WORKTREE_BUILD_CONTEXT_CACHE_RELATIVE_DIR = ".cache/worktree-build-context";
const WORKTREE_BUILD_CONTEXT_ENV = "FHEVM_WORKTREE_BUILD_CONTEXT_ROOT";

const STEP_BY_NAME = new Map(DEPLOYMENT_STEPS.map((step) => [step.name, step]));

const EXIT_SUCCESS = 0;
const EXIT_FAILURE = 1;
// Protocol reserves key/CRS IDs by high-byte namespace; gateway bootstrap increments from these bases.
const KEY_COUNTER_BASE = 4n << 248n;
const CRS_COUNTER_BASE = 5n << 248n;

function defaultKeyMaterialIdFromCounterBase(counterBase: bigint): string {
  // KMS IDs are uint256 counters. Legacy defaults map to first generated IDs: base + 1.
  return (counterBase + 1n).toString(16).padStart(64, "0");
}

const DEFAULT_RELAYER_PUBLIC_KEY_ID = defaultKeyMaterialIdFromCounterBase(KEY_COUNTER_BASE);
const DEFAULT_RELAYER_CRS_ID = defaultKeyMaterialIdFromCounterBase(CRS_COUNTER_BASE);
const CLI_USAGE_PREFIX = process.env.FHEVM_CLI_USAGE_PREFIX?.trim() || "bun run";
// Keep scan local to avoid probing unrelated high ports while still handling crowded developer machines.
const HOST_PORT_SCAN_MAX_OFFSET = 500;
const GATEWAY_BOOTSTRAP_MAX_ATTEMPTS = 36;
const GATEWAY_BOOTSTRAP_RETRY_DELAY_SECONDS = 5;
const SERVICE_WAIT_RETRY_INTERVAL_SECONDS = 5;
const SERVICE_WAIT_MAX_RETRIES_DEFAULT = 30;
const SERVICE_WAIT_MAX_RETRIES_GATEWAY_SC_DEPLOY = 60;
// Docker reports OOM kills as exit code 137 (SIGKILL).
const OOM_KILLED_EXIT_CODE = 137;
// Fallback to selector matching when revert text is not decoded in container logs.
const EXPECTED_PAUSE_SELECTOR = "0x8dfc202b";
const ENFORCED_PAUSE_SELECTOR = "0xd93c0665";

class CliError extends Error {
  code: string;
  showUsage: boolean;

  constructor(code: string, message: string, options?: { showUsage?: boolean }) {
    super(message);
    this.code = code;
    this.showUsage = options?.showUsage ?? false;
  }
}

const HOST_PORT_SPECS: HostPortSpec[] = [
  { envVar: "MINIO_API_PORT", defaultPort: 9000, protocol: "tcp" },
  { envVar: "MINIO_CONSOLE_PORT", defaultPort: 9001, protocol: "tcp" },
  { envVar: "HOST_NODE_RPC_PORT", defaultPort: 8545, protocol: "tcp" },
  { envVar: "GATEWAY_NODE_RPC_PORT", defaultPort: 8546, protocol: "tcp" },
  { envVar: "KMS_CORE_GRPC_PORT", defaultPort: 50051, protocol: "tcp" },
  { envVar: "DB_EXPOSE_PORT", defaultPort: 5432, protocol: "tcp" },
  { envVar: "RELAYER_DB_EXPOSE_PORT", defaultPort: 5433, protocol: "tcp" },
  { envVar: "RELAYER_MIGRATE_PORT", defaultPort: 3001, protocol: "tcp" },
  { envVar: "RELAYER_API_PORT", defaultPort: 3000, protocol: "tcp" },
  { envVar: "PROMETHEUS_PORT", defaultPort: 9090, protocol: "tcp" },
  { envVar: "JAEGER_UDP_6831_PORT", defaultPort: 6831, protocol: "udp" },
  { envVar: "JAEGER_UDP_6832_PORT", defaultPort: 6832, protocol: "udp" },
  { envVar: "JAEGER_5778_PORT", defaultPort: 5778, protocol: "tcp" },
  { envVar: "JAEGER_QUERY_PORT", defaultPort: 16686, protocol: "tcp" },
  { envVar: "JAEGER_OTLP_GRPC_PORT", defaultPort: 4317, protocol: "tcp" },
  { envVar: "JAEGER_OTLP_HTTP_PORT", defaultPort: 4318, protocol: "tcp" },
  { envVar: "JAEGER_14250_PORT", defaultPort: 14250, protocol: "tcp" },
  { envVar: "JAEGER_14268_PORT", defaultPort: 14268, protocol: "tcp" },
  { envVar: "JAEGER_14269_PORT", defaultPort: 14269, protocol: "tcp" },
  { envVar: "JAEGER_ZIPKIN_PORT", defaultPort: 9411, protocol: "tcp" },
];

let hostPortsAssigned = false;

function color(text: string, tone: string): string {
  return `${tone}${text}${COLORS.reset}`;
}

function printLogo(): void {
  console.log(COLORS.lightBlue);
  console.log("  ______   _    _   ______  __      __  __  __");
  console.log(" |  ____| | |  | | |  ____| \\ \\    / / |  \\/  |");
  console.log(" | |__    | |__| | | |__     \\ \\  / /  | \\  / |");
  console.log(" |  __|   |  __  | |  __|     \\ \\/ /   | |\\/| |");
  console.log(" | |      | |  | | | |____     \\  /    | |  | |");
  console.log(" |_|      |_|  |_| |______|     \\/     |_|  |_|");
  console.log(COLORS.reset);
}

function cliCommand(args?: string): string {
  const trimmedArgs = args?.trim();
  if (!trimmedArgs) {
    return CLI_USAGE_PREFIX;
  }
  return `${CLI_USAGE_PREFIX} ${trimmedArgs}`;
}

function usage(): void {
  const testTypes = Object.keys(TEST_TYPE_CONFIG).join("|");

  printLogo();
  console.log(`${COLORS.bold}Usage:${COLORS.reset} ${COLORS.yellow}${CLI_USAGE_PREFIX}${COLORS.reset} ${COLORS.cyan}COMMAND [OPTIONS]${COLORS.reset}`);
  console.log("");
  console.log(`${COLORS.bold}${COLORS.lightBlue}Commands:${COLORS.reset}`);
  console.log(`  ${COLORS.yellow}up${COLORS.reset} ${COLORS.cyan}[deploy options]${COLORS.reset}    Alias of deploy (recommended default command)`);
  console.log(`  ${COLORS.yellow}deploy${COLORS.reset} ${COLORS.cyan}[--build] [--local] [--network testnet|mainnet] [--coprocessors N] [--coprocessor-threshold T] [--resume STEP] [--only STEP] [--telemetry-smoke] [--strict-otel] [--no-tracing]${COLORS.reset}    Deploy the full fhevm stack`);
  console.log(`  ${COLORS.yellow}down${COLORS.reset} ${COLORS.cyan}[clean options]${COLORS.reset}  Alias of clean`);
  console.log(`  ${COLORS.yellow}pause${COLORS.reset} ${COLORS.cyan}[CONTRACTS]${COLORS.reset}     Pause specific contracts (host|gateway)`);
  console.log(`  ${COLORS.yellow}unpause${COLORS.reset} ${COLORS.cyan}[CONTRACTS]${COLORS.reset}     Unpause specific contracts (host|gateway)`);
  console.log(`  ${COLORS.yellow}test${COLORS.reset} ${COLORS.cyan}[TYPE]${COLORS.reset}         Run tests (${testTypes})`);
  console.log(`  ${COLORS.yellow}upgrade${COLORS.reset} ${COLORS.cyan}[SERVICE]${COLORS.reset}   Upgrade specific service`);
  console.log(`  ${COLORS.yellow}clean${COLORS.reset} ${COLORS.cyan}[--purge] [--purge-images] [--purge-build-cache] [--purge-networks] [--purge-local-cache] [--all-fhevm-projects]${COLORS.reset}  Clean stack resources`);
  console.log(`  ${COLORS.yellow}trace${COLORS.reset} ${COLORS.cyan}[up|down|status]${COLORS.reset}   Manage local tracing stack (Jaeger + Prometheus)`);
  console.log(`  ${COLORS.yellow}status${COLORS.reset}                Show full stack status for current project`);
  console.log(`  ${COLORS.yellow}logs${COLORS.reset} ${COLORS.cyan}[SERVICE]${COLORS.reset}      View logs for a specific service`);
  console.log(`  ${COLORS.yellow}telemetry-smoke${COLORS.reset}     Validate Jaeger telemetry services`);
  console.log(`  ${COLORS.yellow}help${COLORS.reset}                Display this help message`);
  console.log("");
  console.log(`${COLORS.bold}${COLORS.lightBlue}Deploy Options:${COLORS.reset}`);
  console.log(`  ${COLORS.cyan}--build${COLORS.reset}                Rebuild buildable services (advanced; no local cache tuning)`);
  console.log(`  ${COLORS.cyan}--local | --dev${COLORS.reset}       Recommended local mode: rebuild with BuildKit local cache optimizations`);
  console.log(`  ${COLORS.cyan}--network NAME${COLORS.reset}        Version profile for deploy (${COLORS.green}testnet${COLORS.reset}|${COLORS.green}mainnet${COLORS.reset})`);
  console.log(`  ${COLORS.cyan}--coprocessors N${COLORS.reset}      Number of coprocessor instances for local n/t topology`);
  console.log(`  ${COLORS.cyan}--coprocessor-threshold T${COLORS.reset}  Coprocessor threshold override (must be <= N)`);
  console.log(`  ${COLORS.cyan}--resume STEP${COLORS.reset}         Redeploy from a specific step onward`);
  console.log(`  ${COLORS.cyan}--only STEP${COLORS.reset}           Redeploy only one step`);
  console.log(`  ${COLORS.cyan}--no-tracing${COLORS.reset}         Disable automatic tracing startup (Jaeger + Prometheus)`);
  console.log(`  ${COLORS.cyan}--telemetry-smoke${COLORS.reset}     Run Jaeger service smoke-check after deploy`);
  console.log(`  ${COLORS.cyan}--strict-otel${COLORS.reset}          Fail fast if OTEL endpoint requires unavailable Jaeger`);
  console.log("");
  console.log(`${COLORS.bold}${COLORS.lightBlue}Test Options:${COLORS.reset}`);
  console.log(`  ${COLORS.cyan}-v, --verbose${COLORS.reset}       Enable verbose output`);
  console.log(`  ${COLORS.cyan}-n, --network NAME${COLORS.reset}  Specify network (default: ${COLORS.green}staging${COLORS.reset})`);
  console.log(`  ${COLORS.cyan}-g, --grep PATTERN${COLORS.reset}  Override default test pattern`);
  console.log(`  ${COLORS.cyan}-r, --no-relayer${COLORS.reset}    Disable Rust relayer`);
  console.log(`  ${COLORS.cyan}--no-hardhat-compile${COLORS.reset}        Skip Hardhat compilation step`);
  console.log("");
  console.log(`${COLORS.bold}${COLORS.lightBlue}Clean Options:${COLORS.reset}`);
  console.log(`  ${COLORS.cyan}--purge${COLORS.reset}               Equivalent to --purge-images --purge-build-cache --purge-networks`);
  console.log(`  ${COLORS.cyan}--purge-images${COLORS.reset}        Remove images for fhevm compose services only`);
  console.log(`  ${COLORS.cyan}--purge-build-cache${COLORS.reset}   Remove local fhevm Buildx cache directory`);
  console.log(`  ${COLORS.cyan}--purge-networks${COLORS.reset}      Remove networks labeled for the active fhevm compose project only`);
  console.log(`  ${COLORS.cyan}--purge-local-cache${COLORS.reset}   Alias of --purge-build-cache (kept for compatibility)`);
  console.log(`  ${COLORS.cyan}--all-fhevm-projects${COLORS.reset}  Clean every Docker resource prefixed with ${COLORS.green}fhevm-${COLORS.reset}`);
  console.log("");
  console.log(`${COLORS.bold}${COLORS.lightBlue}Examples:${COLORS.reset}`);
  console.log(`  ${COLORS.purple}${cliCommand("up --network testnet")}${COLORS.reset}`);
  console.log(`  ${COLORS.purple}${cliCommand("trace up")}${COLORS.reset}`);
  console.log(`  ${COLORS.purple}${cliCommand("deploy")}${COLORS.reset}`);
  console.log(`  ${COLORS.purple}${cliCommand("deploy --build")}${COLORS.reset}`);
  console.log(`  ${COLORS.purple}${cliCommand("deploy --local")}${COLORS.reset}`);
  console.log(`  ${COLORS.purple}${cliCommand("deploy --local --telemetry-smoke")}${COLORS.reset}`);
  console.log(`  ${COLORS.purple}${cliCommand("deploy --network testnet")}${COLORS.reset}`);
  console.log(`  ${COLORS.purple}${cliCommand("deploy --network testnet --no-tracing")}${COLORS.reset}`);
  console.log(`  ${COLORS.purple}${cliCommand("deploy --coprocessors 2 --coprocessor-threshold 2")}${COLORS.reset}`);
  console.log(`  ${COLORS.purple}${cliCommand("deploy --resume kms-connector")}${COLORS.reset}`);
  console.log(`  ${COLORS.purple}${cliCommand("deploy --only coprocessor")}${COLORS.reset}`);
  console.log(`  ${COLORS.purple}${cliCommand("test input-proof")}${COLORS.reset}`);
  console.log(`  ${COLORS.purple}${cliCommand("test input-proof --no-hardhat-compile")}${COLORS.reset}`);
  console.log(`  ${COLORS.purple}${cliCommand("test user-decryption")}${COLORS.reset}`);
  console.log(`  ${COLORS.purple}${cliCommand("test public-decrypt-http-ebool")}${COLORS.reset}`);
  console.log(`  ${COLORS.purple}${cliCommand("test public-decrypt-http-mixed -n staging")}${COLORS.reset}`);
  console.log(`  ${COLORS.purple}${cliCommand("test erc20")}${COLORS.reset}`);
  console.log(`  ${COLORS.purple}${cliCommand("upgrade coprocessor")}${COLORS.reset}`);
  console.log(`  ${COLORS.purple}${cliCommand("telemetry-smoke")}${COLORS.reset}`);
  console.log(`  ${COLORS.purple}${cliCommand("clean --purge")}${COLORS.reset}`);
  console.log(`  ${COLORS.purple}${cliCommand("clean --all-fhevm-projects")}${COLORS.reset}`);
  console.log(`  ${COLORS.purple}${cliCommand("down --purge")}${COLORS.reset}`);
  console.log(`  ${COLORS.purple}${cliCommand("clean --purge-build-cache")}${COLORS.reset}`);
  console.log(`  ${COLORS.purple}${cliCommand("status")}${COLORS.reset}`);
  console.log(`  ${COLORS.purple}${cliCommand("trace down")}${COLORS.reset}`);
  console.log(`  ${COLORS.purple}FHEVM_DOCKER_PROJECT=fhevm-dev ${cliCommand("up")}${COLORS.reset}`);
  console.log(`${COLORS.blue}============================================================${COLORS.reset}`);
}

type HelpOption = {
  flag: string;
  description: string;
};

function usageCommand(
  command: string,
  synopsis: string,
  description: string,
  options: HelpOption[],
  examples: string[],
): void {
  const synopsisSegment = synopsis.trim() === "" ? "" : ` ${COLORS.cyan}${synopsis}${COLORS.reset}`;
  printLogo();
  console.log(`${COLORS.bold}Usage:${COLORS.reset} ${COLORS.yellow}${cliCommand(command)}${COLORS.reset}${synopsisSegment}`);
  console.log("");
  console.log(`${COLORS.bold}${description}${COLORS.reset}`);
  if (options.length > 0) {
    console.log("");
    console.log(`${COLORS.bold}${COLORS.lightBlue}Options:${COLORS.reset}`);
    for (const option of options) {
      console.log(`  ${COLORS.cyan}${option.flag}${COLORS.reset}  ${option.description}`);
    }
  }
  if (examples.length > 0) {
    console.log("");
    console.log(`${COLORS.bold}${COLORS.lightBlue}Examples:${COLORS.reset}`);
    for (const example of examples) {
      console.log(`  ${COLORS.purple}${example}${COLORS.reset}`);
    }
  }
  console.log(`${COLORS.blue}============================================================${COLORS.reset}`);
}

function usageDeployCommand(command = "deploy"): void {
  usageCommand(
    command,
    "[--build] [--local] [--network testnet|mainnet] [--coprocessors N] [--coprocessor-threshold T] [--resume STEP] [--only STEP] [--telemetry-smoke] [--strict-otel] [--no-tracing]",
    "Deploy the full fhevm stack",
    [
      { flag: "--build", description: "Rebuild buildable services (advanced; no local cache tuning)" },
      { flag: "--local | --dev", description: "Recommended local mode: rebuild with BuildKit local cache optimizations" },
      { flag: "--network NAME", description: "Version profile for deploy (testnet|mainnet)" },
      { flag: "--coprocessors N", description: "Number of coprocessor instances for local n/t topology" },
      { flag: "--coprocessor-threshold T", description: "Coprocessor threshold override (must be <= N)" },
      { flag: "--resume STEP", description: "Redeploy from a specific step onward" },
      { flag: "--only STEP", description: "Redeploy only one step" },
      { flag: "--no-tracing", description: "Disable automatic tracing startup (Jaeger + Prometheus)" },
      { flag: "--telemetry-smoke", description: "Run Jaeger service smoke-check after deploy" },
      { flag: "--strict-otel", description: "Fail fast if OTEL endpoint requires unavailable Jaeger" },
    ],
    [
      cliCommand("up --network testnet"),
      cliCommand("up --network testnet --telemetry-smoke"),
      cliCommand("up --only coprocessor --no-tracing"),
    ],
  );
}

function usageCleanCommand(command = "clean"): void {
  usageCommand(
    command,
    "[--purge] [--purge-images] [--purge-build-cache] [--purge-networks] [--purge-local-cache] [--all-fhevm-projects]",
    "Clean stack resources",
    [
      { flag: "--purge", description: "Equivalent to --purge-images --purge-build-cache --purge-networks" },
      { flag: "--purge-images", description: "Remove images for fhevm compose services only" },
      { flag: "--purge-build-cache", description: "Remove local fhevm Buildx cache directory" },
      { flag: "--purge-networks", description: "Remove networks labeled for the active fhevm compose project only" },
      { flag: "--purge-local-cache", description: "Alias of --purge-build-cache (kept for compatibility)" },
      { flag: "--all-fhevm-projects", description: "Clean every Docker resource prefixed with fhevm-" },
    ],
    [cliCommand("down --purge"), cliCommand("clean --all-fhevm-projects")],
  );
}

function usageTraceCommand(): void {
  usageCommand(
    "trace",
    "[up|down|status]",
    "Manage local tracing stack (Jaeger + Prometheus only)",
    [],
    [cliCommand("trace up"), cliCommand("trace status"), cliCommand("trace down")],
  );
}

function usageStatusCommand(): void {
  usageCommand(
    "status",
    "",
    "Show full stack status for current project",
    [],
    [cliCommand("status")],
  );
}

function usagePauseCommand(command: "pause" | "unpause"): void {
  usageCommand(
    command,
    "[host|gateway]",
    `${command === "pause" ? "Pause" : "Unpause"} specific contracts`,
    [],
    [cliCommand(`${command} host`), cliCommand(`${command} gateway`)],
  );
}

function usageTestCommand(): void {
  const testTypes = Object.keys(TEST_TYPE_CONFIG).join("|");
  usageCommand(
    "test",
    `[${testTypes}]`,
    "Run test-suite scenarios",
    [
      { flag: "-v, --verbose", description: "Enable verbose output" },
      { flag: "-n, --network NAME", description: "Specify network (default: staging)" },
      { flag: "-g, --grep PATTERN", description: "Override default test pattern" },
      { flag: "-r, --no-relayer", description: "Disable Rust relayer" },
      { flag: "--no-hardhat-compile", description: "Skip Hardhat compilation step" },
    ],
    [
      cliCommand("test input-proof"),
      cliCommand("test input-proof --no-hardhat-compile"),
      cliCommand("test operators -g \"pattern\""),
    ],
  );
}

function usageUpgradeCommand(): void {
  usageCommand(
    "upgrade",
    "[SERVICE]",
    "Upgrade a specific service",
    [],
    [cliCommand("upgrade coprocessor"), cliCommand("upgrade relayer")],
  );
}

function usageLogsCommand(): void {
  usageCommand(
    "logs",
    "[SERVICE]",
    "View logs for a specific service",
    [],
    [cliCommand("logs coprocessor"), cliCommand("logs relayer")],
  );
}

function usageTelemetrySmokeCommand(): void {
  usageCommand(
    "telemetry-smoke",
    "",
    "Validate required Jaeger telemetry services",
    [],
    [cliCommand("telemetry-smoke")],
  );
}

function logInfo(message: string): void {
  console.log(`${COLORS.green}[INFO]${COLORS.reset} ${message}`);
}

function logWarn(message: string): void {
  console.log(`${COLORS.yellow}[WARN]${COLORS.reset} ${message}`);
}

function logError(message: string): void {
  console.error(`${COLORS.red}[ERROR]${COLORS.reset} ${message}`);
}

function cliError(code: string, message: string, options?: { showUsage?: boolean }): never {
  throw new CliError(code, message, options);
}

function usageError(message: string): never {
  throw new CliError("E_USAGE", message, { showUsage: true });
}

function errorMessage(error: unknown): string {
  return error instanceof Error ? error.message : String(error);
}

function isTcpPortBindable(port: number): Promise<boolean> {
  return new Promise((resolve) => {
    const server = net.createServer();
    server.once("error", () => resolve(false));
    server.once("listening", () => {
      server.close(() => resolve(true));
    });
    server.listen(port, "0.0.0.0");
  });
}

function isUdpPortBindable(port: number): Promise<boolean> {
  return new Promise((resolve) => {
    const socket = dgram.createSocket("udp4");
    socket.once("error", () => {
      socket.close();
      resolve(false);
    });
    socket.once("listening", () => {
      socket.close();
      resolve(true);
    });
    socket.bind(port, "0.0.0.0");
  });
}

function runningContainerNamesPublishingPort(port: number): string[] {
  const result = runCommand(["docker", "ps", "--filter", `publish=${port}`, "--format", "{{.Names}}"], {
    capture: true,
    check: false,
    allowFailure: true,
  });
  return result.stdout
    .split("\n")
    .map((line) => line.trim())
    .filter(Boolean);
}

async function isHostPortAvailable(port: number, protocol: "tcp" | "udp"): Promise<boolean> {
  const occupants = runningContainerNamesPublishingPort(port);
  if (occupants.length > 0) {
    const allOwnedByProject = occupants.every((name) => name.startsWith(`${PROJECT}-`));
    if (!allOwnedByProject) {
      return false;
    }
  }
  return protocol === "tcp" ? isTcpPortBindable(port) : isUdpPortBindable(port);
}

async function findAvailableHostPort(
  startPort: number,
  protocol: "tcp" | "udp",
  reserved: Set<number>,
): Promise<number> {
  const maxOffset = HOST_PORT_SCAN_MAX_OFFSET;
  for (let offset = 0; offset <= maxOffset; offset += 1) {
    const candidate = startPort + offset;
    if (reserved.has(candidate)) {
      continue;
    }
    if (await isHostPortAvailable(candidate, protocol)) {
      return candidate;
    }
  }
  throw new Error(`No available ${protocol.toUpperCase()} port found near ${startPort}`);
}

async function ensureHostPortAssignments(): Promise<void> {
  if (hostPortsAssigned) {
    return;
  }

  const reservedTcp = new Set<number>();
  const reservedUdp = new Set<number>();

  for (const spec of HOST_PORT_SPECS) {
    const existing = process.env[spec.envVar]?.trim();
    if (existing && /^[0-9]+$/.test(existing)) {
      const parsed = Number.parseInt(existing, 10);
      if (spec.protocol === "tcp") {
        reservedTcp.add(parsed);
      } else {
        reservedUdp.add(parsed);
      }
      continue;
    }

    const reserved = spec.protocol === "tcp" ? reservedTcp : reservedUdp;
    const resolved = await findAvailableHostPort(spec.defaultPort, spec.protocol, reserved);
    process.env[spec.envVar] = String(resolved);
    reserved.add(resolved);
    if (resolved !== spec.defaultPort) {
      logWarn(
        `Port ${spec.defaultPort}/${spec.protocol} is busy. Using ${resolved}/${spec.protocol} for ${spec.envVar}.`,
      );
    }
  }

  hostPortsAssigned = true;
}

function isHelpToken(value: string): boolean {
  return value === "--help" || value === "-h" || value === "help";
}

function hasHelpArg(args: string[]): boolean {
  return args.some((arg) => isHelpToken(arg));
}

function loadDotEnvFile(filePath: string, overrideExisting = false): void {
  if (!fs.existsSync(filePath)) {
    return;
  }

  const raw = fs.readFileSync(filePath, "utf8");
  const lines = raw.split("\n");

  for (const line of lines) {
    const trimmed = line.trim();
    if (!trimmed || trimmed.startsWith("#")) {
      continue;
    }

    const idx = trimmed.indexOf("=");
    if (idx <= 0) {
      continue;
    }

    const key = trimmed.slice(0, idx).trim();
    let value = trimmed.slice(idx + 1).trim();

    if (
      (value.startsWith("\"") && value.endsWith("\"")) ||
      (value.startsWith("'") && value.endsWith("'"))
    ) {
      value = value.slice(1, -1);
    }

    if (overrideExisting || !(key in process.env) || process.env[key] === "") {
      process.env[key] = value;
    }
  }
}

function ensureDefaultVersions(): void {
  loadDotEnvFile(path.resolve(ENV_DIR, ".env.versions"));
  loadActiveVersionsIfPresent();

  const versionAliases: Record<string, string | undefined> = {
    [STACK_VERSION_OVERRIDE_ENV]: process.env[STACK_VERSION_OVERRIDE_ENV],
    [CORE_VERSION_OVERRIDE_ENV]: process.env[CORE_VERSION_OVERRIDE_ENV],
    [RELAYER_VERSION_OVERRIDE_ENV]: process.env[RELAYER_VERSION_OVERRIDE_ENV],
  };

  const seen = new Set<string>();

  for (const version of VERSION_ENTRIES) {
    if (seen.has(version.envVar)) {
      continue;
    }
    seen.add(version.envVar);

    if (!process.env[version.envVar] || process.env[version.envVar] === "") {
      const aliasValue = version.groupOverrideEnv ? versionAliases[version.groupOverrideEnv] : undefined;
      process.env[version.envVar] = aliasValue && aliasValue !== "" ? aliasValue : version.defaultValue;
    }
  }
}

function uniqueVersionEnvVars(): string[] {
  const seen = new Set<string>();
  const keys: string[] = [];
  for (const version of VERSION_ENTRIES) {
    if (seen.has(version.envVar)) {
      continue;
    }
    seen.add(version.envVar);
    keys.push(version.envVar);
  }
  return keys;
}

function persistActiveVersions(): void {
  const lines = [
    "# Auto-generated by fhevm deploy command.",
    "# Stores the last deployed image version set for follow-up commands like upgrade.",
  ];

  for (const envVar of uniqueVersionEnvVars()) {
    const value = process.env[envVar];
    if (value && value.trim() !== "") {
      lines.push(`${envVar}=${value.trim()}`);
    }
  }

  fs.writeFileSync(ACTIVE_VERSIONS_FILE, `${lines.join("\n")}\n`, "utf8");
}

function loadActiveVersionsIfPresent(): void {
  if (!fs.existsSync(ACTIVE_VERSIONS_FILE)) {
    return;
  }
  loadDotEnvFile(ACTIVE_VERSIONS_FILE, true);
}

function printVersionSummary(buildTag: string): void {
  logInfo("FHEVM Stack Versions:");

  let currentGroup = "";
  for (const version of VERSION_ENTRIES) {
    if (version.group !== currentGroup) {
      logInfo(`${version.group}:`);
      currentGroup = version.group;
    }

    const value = process.env[version.envVar] ?? version.defaultValue;
    const suffix = version.appendBuildTag ? buildTag : "";
    logInfo(`  ${version.displayName}:${value}${suffix}`);
  }
}

function stepNames(): string[] {
  return DEPLOYMENT_STEPS.map((step) => step.name);
}

function stepIndex(stepName: string): number {
  return DEPLOYMENT_STEPS.findIndex((step) => step.name === stepName);
}

function getStep(stepName: string): DeploymentStep {
  const step = STEP_BY_NAME.get(stepName);
  if (!step) {
    throw new Error(`Unknown step: ${stepName}`);
  }
  return step;
}

function uniqueComponents(): string[] {
  const seen = new Set<string>();
  const components: string[] = [];

  for (const step of DEPLOYMENT_STEPS) {
    if (!step.component || seen.has(step.component)) {
      continue;
    }
    seen.add(step.component);
    components.push(step.component);
  }

  return components;
}

function localEnvFile(component: string): string {
  return path.resolve(ENV_DIR, `.env.${component}.local`);
}

function baseEnvFile(component: string): string {
  return path.resolve(ENV_DIR, `.env.${component}`);
}

function composeFile(component: string): string {
  return path.resolve(COMPOSE_DIR, `${component}-docker-compose.yml`);
}

function readEnvValue(filePath: string, key: string): string | undefined {
  if (!fs.existsSync(filePath)) {
    return undefined;
  }

  const lines = fs.readFileSync(filePath, "utf8").split("\n");
  for (const line of lines) {
    if (!line || line.startsWith("#")) {
      continue;
    }
    const idx = line.indexOf("=");
    if (idx <= 0) {
      continue;
    }
    if (line.slice(0, idx).trim() !== key) {
      continue;
    }
    let value = line.slice(idx + 1).trim();
    if (
      (value.startsWith("\"") && value.endsWith("\"")) ||
      (value.startsWith("'") && value.endsWith("'"))
    ) {
      value = value.slice(1, -1);
    }
    return value;
  }
  return undefined;
}

function upsertEnvValue(filePath: string, key: string, value: string): void {
  const lines = fs.readFileSync(filePath, "utf8").split("\n");
  let replaced = false;

  const nextLines = lines.map((line) => {
    const idx = line.indexOf("=");
    if (idx <= 0 || line.slice(0, idx).trim() !== key) {
      return line;
    }
    replaced = true;
    return `${key}=${value}`;
  });

  if (!replaced) {
    if (nextLines.length > 0 && nextLines[nextLines.length - 1] !== "") {
      nextLines.push("");
    }
    nextLines.push(`${key}=${value}`);
  }

  fs.writeFileSync(filePath, `${nextLines.join("\n").replace(/\n*$/, "\n")}`, "utf8");
}

function trimTrailingSlashes(value: string): string {
  return value.replace(/\/+$/, "");
}

function extractTrailingPathSegment(url: string | undefined, fallback: string): string {
  if (!url || url.trim() === "") {
    return fallback;
  }
  const normalized = url.trim().replace(/\/+$/, "");
  const idx = normalized.lastIndexOf("/");
  if (idx === -1 || idx === normalized.length - 1) {
    return fallback;
  }
  return normalized.slice(idx + 1);
}

function buildKmsPublicObjectPrefix(rawPrefix: string | undefined): string {
  const parts = (rawPrefix ?? "")
    .split("/")
    .map((part) => part.trim())
    .filter((part) => part !== "");

  // KMS stores public objects under <prefix>/PUB/<kind>/..., so always append PUB.
  parts.push("PUB");

  return parts.join("/");
}

function additionalCoprocessorEnvFile(instanceIdx: number): string {
  return path.resolve(ENV_DIR, `.env.coprocessor.${instanceIdx}.local`);
}

function generatedCoprocessorComposeFile(instanceIdx: number): string {
  return path.resolve(COMPOSE_DIR, `coprocessor-${instanceIdx}.generated.yml`);
}

function parsePositiveInteger(value: string, flagName: string): number {
  if (!/^[0-9]+$/.test(value)) {
    usageError(`${flagName} expects a positive integer`);
  }
  const parsed = Number.parseInt(value, 10);
  if (parsed < 1) {
    usageError(`${flagName} expects a positive integer`);
  }
  return parsed;
}

function coprocessorChecks(): { service: string; state: ServiceState }[] {
  const step = getStep("coprocessor");
  return step.serviceChecks.filter((check) => check.service.startsWith("coprocessor-"));
}

function mapCoprocessorServiceForInstance(service: string, instanceIdx: number): string {
  return service.replace(/^coprocessor-/, `coprocessor${instanceIdx}-`);
}

function resolveMainCoprocessorCompose(): string {
  return composeFile("coprocessor");
}

function createGeneratedCoprocessorCompose(instanceIdx: number): string {
  const sourceCompose = composeFile("coprocessor");
  if (!fs.existsSync(sourceCompose)) {
    throw new Error(`Coprocessor compose file not found: ${sourceCompose}`);
  }

  const source = fs.readFileSync(sourceCompose, "utf8");
  const replacedEnv = source.replaceAll(
    "../env/staging/.env.coprocessor.local",
    `../env/staging/.env.coprocessor.${instanceIdx}.local`,
  );
  const renamedServices = replacedEnv.replaceAll("coprocessor-", `coprocessor${instanceIdx}-`);
  const normalizedArgs = renamedServices.replaceAll(`--coprocessor${instanceIdx}-fhe-threads`, "--coprocessor-fhe-threads");

  const targetCompose = generatedCoprocessorComposeFile(instanceIdx);
  fs.writeFileSync(targetCompose, normalizedArgs, "utf8");
  return targetCompose;
}

function findAdditionalCoprocessorIndices(): number[] {
  const indices = new Set<number>();

  for (const fileName of fs.readdirSync(ENV_DIR)) {
    const match = fileName.match(/^\.env\.coprocessor\.(\d+)\.local$/);
    if (!match) {
      continue;
    }
    const idx = Number.parseInt(match[1], 10);
    if (Number.isFinite(idx) && idx > 0) {
      indices.add(idx);
    }
  }

  const dockerNames = runCommand(["docker", "ps", "-a", "--format", "{{.Names}}"], {
    capture: true,
    check: false,
    allowFailure: true,
  });
  for (const name of dockerNames.stdout.split("\n").map((line) => line.trim()).filter(Boolean)) {
    const match = name.match(/(?:^|-)coprocessor(\d+)-/);
    if (!match) {
      continue;
    }
    const idx = Number.parseInt(match[1], 10);
    if (Number.isFinite(idx) && idx > 0) {
      indices.add(idx);
    }
  }

  return [...indices].sort((a, b) => a - b);
}

function cleanupGeneratedCoprocessorArtifacts(instanceIdx: number): void {
  const generatedCompose = generatedCoprocessorComposeFile(instanceIdx);
  if (fs.existsSync(generatedCompose)) {
    fs.rmSync(generatedCompose, { force: true });
  }
}

function composeBaseCommand(composePath: string, envFile?: string): string[] {
  const command = ["docker", "compose", "-p", PROJECT];
  if (envFile && fs.existsSync(envFile)) {
    command.push("--env-file", envFile);
  }
  command.push("-f", composePath);
  return command;
}

function cleanupComposeServices(
  composePath: string,
  envFile?: string,
  removeVolumes = true,
  removeOrphans = false,
): void {
  const base = composeBaseCommand(composePath, envFile);
  const downArgs = [...base, "down"];
  if (removeVolumes) {
    downArgs.push("-v");
  }
  if (removeOrphans) {
    downArgs.push("--remove-orphans");
  }
  runCommand(downArgs, { check: false, allowFailure: true });
}

function listProjectContainers(): string[] {
  const result = runCommand(
    ["docker", "ps", "-a", "--filter", `label=com.docker.compose.project=${PROJECT}`, "--format", "{{.Names}}"],
    {
      capture: true,
      check: false,
      allowFailure: true,
    },
  );
  if (result.status !== 0) {
    return [];
  }
  return result.stdout
    .split("\n")
    .map((line) => line.trim())
    .filter(Boolean);
}

function projectScopedName(name: string): string {
  return `${PROJECT}-${name}`;
}

function resolveProjectContainerName(logicalName: string): string | undefined {
  const containers = listProjectContainers();
  if (containers.length === 0) {
    return undefined;
  }

  const exactCandidates = [logicalName, projectScopedName(logicalName)];
  for (const candidate of exactCandidates) {
    if (containers.includes(candidate)) {
      return candidate;
    }
  }

  return containers.find((name) => name.endsWith(`-${logicalName}`) || name.endsWith(logicalName));
}

function componentContainerPrefixes(component: string): string[] {
  switch (component) {
    case "gateway-sc":
      return ["gateway-sc-"];
    case "host-sc":
      return ["host-sc-"];
    default:
      return [];
  }
}

function cleanupComponentScopedContainers(component: string): void {
  const prefixes = componentContainerPrefixes(component);
  if (prefixes.length === 0) {
    return;
  }

  const containers = listProjectContainers().filter((name) => prefixes.some((prefix) => name.includes(prefix)));
  if (containers.length === 0) {
    return;
  }

  runCommand(["docker", "rm", "-f", ...containers], {
    check: false,
    allowFailure: true,
  });
}

function cleanupAdditionalCoprocessorInstances(removeVolumes = true, removeOrphans = false): void {
  const indices = findAdditionalCoprocessorIndices();
  if (indices.length === 0) {
    return;
  }

  for (const instanceIdx of indices) {
    const envFile = additionalCoprocessorEnvFile(instanceIdx);
    const compose = createGeneratedCoprocessorCompose(instanceIdx);
    cleanupComposeServices(compose, envFile, removeVolumes, removeOrphans);
    cleanupGeneratedCoprocessorArtifacts(instanceIdx);
  }
}

function cleanupManagedProjectContainers(): void {
  const managedNamePattern =
    /^(?:[a-zA-Z0-9_.-]+-)?(fhevm-minio|fhevm-minio-setup|kms-core|coprocessor-and-kms-db|gateway-node|host-node|fhevm-relayer|fhevm-relayer-db|relayer-db-migration|fhevm-test-suite-e2e-debug|gateway-.*|host-sc-.*|coprocessor(\d+)?-.*|kms-connector-.*)$/;

  const managed = listProjectContainers().filter((name) => managedNamePattern.test(name));
  if (managed.length === 0) {
    return;
  }

  runCommand(["docker", "rm", "-f", ...managed], {
    check: false,
    allowFailure: true,
  });
}

function cleanupKnownStack(removeVolumes: boolean): void {
  const components = [...uniqueComponents()].reverse();

  for (const component of components) {
    cleanupComponentScopedContainers(component);

    if (component === "coprocessor") {
      cleanupAdditionalCoprocessorInstances(removeVolumes, false);
    }

    const compose = composeFile(component);
    if (!fs.existsSync(compose)) {
      continue;
    }

    const envFile = resolveCleanupEnvFile(component);
    cleanupComposeServices(compose, envFile, removeVolumes, false);
  }

  cleanupManagedProjectContainers();
}

function removeStaleAdditionalCoprocessorEnvFiles(): void {
  for (const instanceIdx of findAdditionalCoprocessorIndices()) {
    const envFile = additionalCoprocessorEnvFile(instanceIdx);
    if (fs.existsSync(envFile)) {
      fs.rmSync(envFile, { force: true });
    }
    cleanupGeneratedCoprocessorArtifacts(instanceIdx);
  }
}

function castWalletValue(kind: "address" | "private-key", mnemonic: string, mnemonicIndex: number): string {
  const result = runCommand(
    ["cast", "wallet", kind, "--mnemonic", mnemonic, "--mnemonic-index", String(mnemonicIndex)],
    { capture: true, check: true },
  );
  const value = result.stdout.trim();
  if (!value) {
    throw new Error(`cast wallet ${kind} returned an empty value for mnemonic index ${mnemonicIndex}`);
  }
  return value;
}

function configureMulticoprocessorEnvs(options: DeployOptions): void {
  const gatewayEnv = localEnvFile("gateway-sc");
  const hostEnv = localEnvFile("host-sc");
  const coprocessorEnv = localEnvFile("coprocessor");

  const configuredThresholdRaw = readEnvValue(gatewayEnv, "COPROCESSOR_THRESHOLD");
  let configuredThreshold = 1;
  if (configuredThresholdRaw && configuredThresholdRaw !== "") {
    if (!/^[0-9]+$/.test(configuredThresholdRaw) || Number.parseInt(configuredThresholdRaw, 10) < 1) {
      throw new Error(`Invalid COPROCESSOR_THRESHOLD value in ${path.basename(gatewayEnv)}: ${configuredThresholdRaw}`);
    }
    configuredThreshold = Number.parseInt(configuredThresholdRaw, 10);
  }
  if (options.coprocessorThresholdOverride) {
    configuredThreshold = options.coprocessorThresholdOverride;
  }

  if (configuredThreshold > options.coprocessorCount) {
    throw new Error(
      `Configured coprocessor threshold (${configuredThreshold}) cannot exceed number of coprocessors (${options.coprocessorCount})`,
    );
  }

  upsertEnvValue(gatewayEnv, "NUM_COPROCESSORS", String(options.coprocessorCount));
  upsertEnvValue(gatewayEnv, "COPROCESSOR_THRESHOLD", String(configuredThreshold));
  upsertEnvValue(hostEnv, "NUM_COPROCESSORS", String(options.coprocessorCount));
  upsertEnvValue(hostEnv, "COPROCESSOR_THRESHOLD", String(configuredThreshold));

  if (options.coprocessorCount === 1) {
    removeStaleAdditionalCoprocessorEnvFiles();
    return;
  }

  const mnemonic = readEnvValue(gatewayEnv, "MNEMONIC");
  if (!mnemonic) {
    throw new Error(`Missing MNEMONIC in ${path.basename(gatewayEnv)}; cannot derive coprocessor accounts`);
  }

  const castExists = runCommand(["cast", "--version"], { check: false, allowFailure: true });
  if (castExists.status !== 0) {
    throw new Error("cast is required to derive coprocessor accounts for multicoprocessor deploys");
  }

  if (options.coprocessorCount > COPROCESSOR_ACCOUNT_INDICES.length) {
    throw new Error(`Not enough predefined account indices for ${options.coprocessorCount} coprocessors`);
  }

  const postgresUser = process.env.POSTGRES_USER ?? "postgres";
  const postgresPassword = process.env.POSTGRES_PASSWORD ?? "postgres";

  for (let idx = 0; idx < options.coprocessorCount; idx += 1) {
    const mnemonicIndex = COPROCESSOR_ACCOUNT_INDICES[idx];
    const cpAddress = castWalletValue("address", mnemonic, mnemonicIndex);
    const cpPrivateKey = castWalletValue("private-key", mnemonic, mnemonicIndex);

    upsertEnvValue(gatewayEnv, `COPROCESSOR_TX_SENDER_ADDRESS_${idx}`, cpAddress);
    upsertEnvValue(gatewayEnv, `COPROCESSOR_SIGNER_ADDRESS_${idx}`, cpAddress);
    upsertEnvValue(gatewayEnv, `COPROCESSOR_S3_BUCKET_URL_${idx}`, "http://minio:9000/ct128");
    upsertEnvValue(hostEnv, `COPROCESSOR_SIGNER_ADDRESS_${idx}`, cpAddress);

    if (idx === 0) {
      upsertEnvValue(coprocessorEnv, "TX_SENDER_PRIVATE_KEY", cpPrivateKey);
      continue;
    }

    const instanceEnv = additionalCoprocessorEnvFile(idx);
    fs.copyFileSync(coprocessorEnv, instanceEnv);
    upsertEnvValue(instanceEnv, "DATABASE_URL", `postgresql://${postgresUser}:${postgresPassword}@db:5432/coprocessor_${idx}`);
    upsertEnvValue(instanceEnv, "TX_SENDER_PRIVATE_KEY", cpPrivateKey);
  }
}

function runAdditionalCoprocessorInstance(instanceIdx: number, useBuild: boolean): void {
  const envFile = additionalCoprocessorEnvFile(instanceIdx);
  const compose = localBuildHandlers.resolveComposeForBuild(createGeneratedCoprocessorCompose(instanceIdx), useBuild);

  try {
    const checks = coprocessorChecks();
    const dbMigration = checks.find((check) => check.service === "coprocessor-db-migration");
    const runtimeChecks = checks.filter(
      (check) => check.service !== "coprocessor-db-migration" && check.service !== "coprocessor-and-kms-db",
    );

    if (!dbMigration) {
      throw new Error("Coprocessor deployment checks are missing db migration state");
    }

    const dbMigrationService = mapCoprocessorServiceForInstance(dbMigration.service, instanceIdx);
    const runtimeServices = runtimeChecks.map((check) => mapCoprocessorServiceForInstance(check.service, instanceIdx));

    logInfo(`Starting additional coprocessor instance #${instanceIdx} (db migration phase)`);
    const dbMigrationCommand = ["docker", "compose", "-p", PROJECT, "--env-file", envFile, "-f", compose, "up"];
    if (useBuild) {
      dbMigrationCommand.push("--build");
    }
    dbMigrationCommand.push("-d", dbMigrationService);
    runCommand(dbMigrationCommand, { check: true });
    deployRuntimeHandlers.waitForService(dbMigrationService, "coprocessor", dbMigration.state);

    logInfo(`Starting additional coprocessor instance #${instanceIdx} (runtime phase)`);
    const runtimeCommand = ["docker", "compose", "-p", PROJECT, "--env-file", envFile, "-f", compose, "up"];
    if (useBuild) {
      runtimeCommand.push("--build");
    }
    runtimeCommand.push("-d", ...runtimeServices);
    runCommand(runtimeCommand, { check: true });

    for (const runtimeCheck of runtimeChecks) {
      deployRuntimeHandlers.waitForService(
        mapCoprocessorServiceForInstance(runtimeCheck.service, instanceIdx),
        "coprocessor",
        runtimeCheck.state,
      );
    }
  } finally {
    cleanupGeneratedCoprocessorArtifacts(instanceIdx);
  }
}

function shouldSkipStep(stepName: string, options: DeployOptions): boolean {
  if (options.onlyStep) {
    return stepName !== options.onlyStep;
  }

  if (options.resumeStep) {
    return stepIndex(stepName) < stepIndex(options.resumeStep);
  }

  return false;
}

function shouldPrebuildGatewayImage(options: DeployOptions): boolean {
  if (!options.forceBuild) {
    return false;
  }

  return DEPLOYMENT_STEPS.some((step) => !shouldSkipStep(step.name, options) && deployRuntimeHandlers.isGatewayBuildStep(step));
}

function tracingStackAlreadyRunning(): boolean {
  return isContainerRunningExact("jaeger") && isContainerRunningExact("prometheus");
}

function resolveCleanupEnvFile(component: string): string | undefined {
  const local = localEnvFile(component);
  const base = baseEnvFile(component);

  if (fs.existsSync(local)) {
    return local;
  }

  if (fs.existsSync(base)) {
    return base;
  }

  return undefined;
}

function cleanupComponent(component: string): void {
  cleanupComponentScopedContainers(component);

  if (component === "coprocessor") {
    cleanupAdditionalCoprocessorInstances();
  }

  const compose = composeFile(component);
  if (!fs.existsSync(compose)) {
    return;
  }

  const envFile = resolveCleanupEnvFile(component);

  if (envFile) {
    logInfo(`Stopping ${component} services...`);
    cleanupComposeServices(compose, envFile, component !== "core");
    return;
  }

  logWarn(`Env file missing for ${component}, attempting service-scoped cleanup without explicit env file`);
  cleanupComposeServices(compose, undefined, component !== "core");
}

function purgeComponentImages(component: string): void {
  if (component === "coprocessor") {
    for (const instanceIdx of findAdditionalCoprocessorIndices()) {
      const envFile = additionalCoprocessorEnvFile(instanceIdx);
      const compose = createGeneratedCoprocessorCompose(instanceIdx);
      const command = ["docker", "compose", "-p", PROJECT];
      if (fs.existsSync(envFile)) {
        command.push("--env-file", envFile);
      }
      command.push("-f", compose, "down", "-v", "--remove-orphans", "--rmi", "all");
      runCommand(command, { check: false, allowFailure: true });
      cleanupGeneratedCoprocessorArtifacts(instanceIdx);
    }
  }

  const compose = composeFile(component);
  if (!fs.existsSync(compose)) {
    return;
  }

  const envFile = resolveCleanupEnvFile(component);
  if (envFile) {
    runCommand(
      ["docker", "compose", "-p", PROJECT, "--env-file", envFile, "-f", compose, "down", "-v", "--remove-orphans", "--rmi", "all"],
      { check: false, allowFailure: true },
    );
    return;
  }

  runCommand(["docker", "compose", "-p", PROJECT, "-f", compose, "down", "-v", "--remove-orphans", "--rmi", "all"], {
    check: false,
    allowFailure: true,
  });
}

function purgeProjectImages(): void {
  const seen = new Set<string>();
  for (const step of DEPLOYMENT_STEPS) {
    const component = step.component;
    if (!component || seen.has(component)) {
      continue;
    }
    seen.add(component);
    purgeComponentImages(component);
  }
}

function purgeLocalBuildxCache(): void {
  const cacheRoot = localBuildHandlers.resolveLocalBuildxCacheRoot();
  if (fs.existsSync(cacheRoot)) {
    fs.rmSync(cacheRoot, { recursive: true, force: true });
    logInfo(`Removed local Buildx cache directory: ${cacheRoot}`);
  } else {
    logInfo(`Local Buildx cache directory not found: ${cacheRoot}`);
  }
}

function cleanupFull(): void {
  logWarn("Setup new environment, cleaning up...");
  cleanupKnownStack(true);
}

function cleanupFromStep(startStep: string): void {
  const startIndex = stepIndex(startStep);
  logWarn(`Resume mode: cleaning up services from '${startStep}' onwards...`);

  const components: string[] = [];
  const seen = new Set<string>();

  for (let i = startIndex; i < DEPLOYMENT_STEPS.length; i += 1) {
    const component = DEPLOYMENT_STEPS[i].component;
    if (!component || seen.has(component)) {
      continue;
    }
    seen.add(component);
    components.push(component);
  }

  for (let i = components.length - 1; i >= 0; i -= 1) {
    cleanupComponent(components[i]);
  }

  logInfo(`Cleanup complete. Services before '${startStep}' preserved.`);
}

function cleanupSingleStep(stepName: string): void {
  const step = getStep(stepName);
  if (!step.component) {
    logInfo(`Step '${stepName}' has no compose file to clean up`);
    return;
  }

  logWarn(`Only mode: cleaning up '${stepName}' services...`);
  cleanupComponent(step.component);
  logInfo(`Cleanup complete. Only '${stepName}' was cleaned.`);
}

function isContainerRunningExact(containerName: string): boolean {
  const resolvedContainer = resolveProjectContainerName(containerName) ?? containerName;
  const result = runCommand(["docker", "ps", "--filter", `name=^${resolvedContainer}$`, "--format", "{{.Names}}"], {
    capture: true,
    check: true,
  });

  return result.stdout
    .split("\n")
    .map((line) => line.trim())
    .filter(Boolean)
    .includes(resolvedContainer);
}

function dockerVolumeExistsExact(volumeName: string): boolean {
  const result = runCommand(["docker", "volume", "ls", "--filter", `name=^${volumeName}$`, "--format", "{{.Name}}"], {
    capture: true,
    check: false,
    allowFailure: true,
  });

  return result.stdout
    .split("\n")
    .map((line) => line.trim())
    .filter(Boolean)
    .includes(volumeName);
}

function minioPrerequisitesMissing(): string[] {
  const missing: string[] = [];
  if (!isContainerRunningExact("fhevm-minio")) {
    missing.push("fhevm-minio container");
  }
  const minioSecretsVolume = `${PROJECT}_minio_secrets`;
  if (!dockerVolumeExistsExact(minioSecretsVolume)) {
    missing.push(`${minioSecretsVolume} volume`);
  }
  return missing;
}

const deployEnvHandlers = createDeployEnvHandlers({
  CONFIG_DIR,
  DEFAULT_OTEL_EXPORTER_OTLP_ENDPOINT,
  defaultRelayerPublicKeyId: DEFAULT_RELAYER_PUBLIC_KEY_ID,
  defaultRelayerCrsId: DEFAULT_RELAYER_CRS_ID,
  HOST_PORT_SPECS,
  baseEnvFile,
  localEnvFile,
  uniqueComponents,
  uniqueVersionEnvVars,
  upsertEnvValue,
  readEnvValue,
  trimTrailingSlashes,
  buildKmsPublicObjectPrefix,
  extractTrailingPathSegment,
  isContainerRunningExact,
  cliCommand,
  cliError,
  logInfo,
  logWarn,
});

const localBuildHandlers = createLocalBuildHandlers({
  FHEVM_DIR,
  PROJECT,
  WORKTREE_BUILD_CONTEXT_CACHE_RELATIVE_DIR,
  WORKTREE_BUILD_CONTEXT_ENV,
  LOCAL_CACHE_SERVICES,
  runCommand,
  logInfo,
  logWarn,
});

const deployRuntimeHandlers = createDeployRuntimeHandlers({
  PROJECT,
  KEY_COUNTER_BASE,
  CRS_COUNTER_BASE,
  GATEWAY_BOOTSTRAP_MAX_ATTEMPTS,
  GATEWAY_BOOTSTRAP_RETRY_DELAY_SECONDS,
  SERVICE_WAIT_RETRY_INTERVAL_SECONDS,
  SERVICE_WAIT_MAX_RETRIES_DEFAULT,
  SERVICE_WAIT_MAX_RETRIES_GATEWAY_SC_DEPLOY,
  OOM_KILLED_EXIT_CODE,
  EXPECTED_PAUSE_SELECTOR,
  ENFORCED_PAUSE_SELECTOR,
  runCommand,
  sleep,
  logInfo,
  logWarn,
  logError,
  cliCommand,
  cliError,
  errorMessage,
  composeFile,
  localEnvFile,
  resolveMainCoprocessorCompose,
  resolveComposeForBuild: localBuildHandlers.resolveComposeForBuild,
  resolveProjectContainerName,
  isContainerRunningExact,
  readEnvValue,
  upsertEnvValue,
  findAdditionalCoprocessorIndices,
  additionalCoprocessorEnvFile,
});

const commandHandlers = createCommandHandlers({
  PROJECT,
  COMPOSE_DIR,
  COLORS,
  runCommand,
  sleep,
  usageError,
  cliError,
  logInfo,
  logWarn,
  color,
  resolveProjectContainerName,
  isContainerRunningExact,
  readContainerLogs: deployRuntimeHandlers.readContainerLogs,
  detectExpectedPause: deployRuntimeHandlers.detectExpectedPause,
  detectEnforcedPause: deployRuntimeHandlers.detectEnforcedPause,
  cleanupKnownStack,
  purgeProjectImages,
  purgeLocalBuildxCache,
  loadActiveVersionsIfPresent,
  localEnvFile,
  composeFile,
  ensureHostPortAssignments,
  runComposeUp: deployRuntimeHandlers.runComposeUp,
});

const deployOptionHandlers = createDeployOptionHandlers({
  usageError,
  logInfo,
  logWarn,
  parsePositiveInteger,
  maxLocalCoprocessors: MAX_LOCAL_COPROCESSORS,
  stepNames,
  stepIndex,
  deploymentSteps: DEPLOYMENT_STEPS,
  minioPrerequisitesMissing,
  isContainerRunningExact,
});

const networkProfileHandlers = createNetworkProfileHandlers({
  networkVersionCacheFile: NETWORK_VERSION_CACHE_FILE,
  runCommand,
  errorMessage,
  logWarn,
  logInfo,
});

async function deploy(args: string[]): Promise<void> {
  const options = deployOptionHandlers.parseDeployArgs(args);
  const effectiveResumeStep = deployOptionHandlers.resolveEffectiveResumeStep(options);
  const runtimeOptions: DeployOptions = { ...options, resumeStep: effectiveResumeStep };
  const worktreeBuildContextPrepared = localBuildHandlers.prepareWorktreeBuildContextIfNeeded(runtimeOptions);

  try {
    // Keep step-specific deploy operations aligned with the currently active stack versions.
    loadActiveVersionsIfPresent();

    if (options.networkProfile) {
      networkProfileHandlers.applyNetworkProfileVersions(options.networkProfile);
    }

    if (options.localBuild) {
      localBuildHandlers.configureLocalBuild();
    }
    if (runtimeOptions.forceBuild) {
      localBuildHandlers.configureBuildId();
    }

    persistActiveVersions();

    if (runtimeOptions.onlyStep) {
      deployOptionHandlers.ensureOnlyStepPrerequisites(runtimeOptions.onlyStep);
    }

    if (runtimeOptions.onlyStep) {
      cleanupSingleStep(runtimeOptions.onlyStep);
    } else if (runtimeOptions.resumeStep) {
      cleanupFromStep(runtimeOptions.resumeStep);
    } else {
      cleanupFull();
    }

    await ensureHostPortAssignments();

    if (runtimeOptions.autoTracing) {
      if (tracingStackAlreadyRunning()) {
        logInfo("Tracing stack already running (Jaeger + Prometheus). Skipping startup.");
      } else {
        logInfo("Ensuring tracing stack is running (Jaeger + Prometheus)...");
        await commandHandlers.traceUp();
      }
    } else {
      logInfo("Skipping tracing startup (--no-tracing).");
    }

    deployEnvHandlers.prepareAllEnvFiles();
    deployEnvHandlers.syncVersionEnvVarsIntoLocalEnvFiles();
    deployEnvHandlers.syncHostPortEnvVarsIntoLocalEnvFiles();
    deployEnvHandlers.prepareLocalConfigRelayer();
    deployEnvHandlers.ensureCoprocessorTelemetryEnv(runtimeOptions.strictOtel || runtimeOptions.telemetrySmoke);
    deployEnvHandlers.ensureCoprocessorLegacyEnvCompatibility();
    deployEnvHandlers.ensureKmsConnectorLegacyEnvCompatibility();
    deployEnvHandlers.ensureRelayerKeyUrlEnvCompatibility();
    deployEnvHandlers.ensureTestSuiteRelayerUrlEnvCompatibility();
    configureMulticoprocessorEnvs(runtimeOptions);
    const gatewayImagePrebuilt = shouldPrebuildGatewayImage(runtimeOptions);
    if (gatewayImagePrebuilt) {
      deployRuntimeHandlers.prebuildGatewayImage();
    }

    logInfo("Deploying FHEVM Stack...");

    const buildTag = runtimeOptions.forceBuild ? " (local build)" : "";
    printVersionSummary(buildTag);

    const runNodePairInParallel =
      !runtimeOptions.onlyStep &&
      !shouldSkipStep("host-node", runtimeOptions) &&
      !shouldSkipStep("gateway-node", runtimeOptions);
    let nodePairHandled = false;

    for (const step of DEPLOYMENT_STEPS) {
      if (runNodePairInParallel && !nodePairHandled && step.name === "host-node") {
        deployRuntimeHandlers.runParallelHostGatewayNodeStartup(
          getStep("host-node"),
          getStep("gateway-node"),
          runtimeOptions.forceBuild && step.buildable,
        );
        nodePairHandled = true;
        continue;
      }

      if (runNodePairInParallel && nodePairHandled && step.name === "gateway-node") {
        continue;
      }

      if (shouldSkipStep(step.name, runtimeOptions)) {
        if (runtimeOptions.onlyStep) {
          logInfo(`Skipping step: ${step.name} (only mode: ${runtimeOptions.onlyStep})`);
        } else {
          logInfo(`Skipping step: ${step.name} (resuming from ${runtimeOptions.resumeStep})`);
        }

        if (step.name === "minio" && isContainerRunningExact("fhevm-minio")) {
          deployRuntimeHandlers.getMinioIp("fhevm-minio");
        }
        continue;
      }

      if (step.name === "kms-signer") {
        sleep(5);
        runCommand([path.resolve(SCRIPTS_DIR, "setup-kms-signer-address.sh")], { check: true });
        continue;
      }

      const useBuild = runtimeOptions.forceBuild &&
        step.buildable &&
        !(gatewayImagePrebuilt && deployRuntimeHandlers.isGatewayBuildStep(step));
      deployRuntimeHandlers.runComposeStep(step, useBuild);

      if (step.name === "coprocessor") {
        for (let idx = 1; idx < runtimeOptions.coprocessorCount; idx += 1) {
          runAdditionalCoprocessorInstance(idx, useBuild);
        }
      }

      if (step.name === "minio") {
        deployRuntimeHandlers.getMinioIp("fhevm-minio");
        continue;
      }

      if (step.name === "gateway-sc") {
        deployRuntimeHandlers.waitForGatewayBootstrapReady();
      }
    }

    if (runtimeOptions.telemetrySmoke) {
      commandHandlers.runTelemetrySmokeCheck(true);
    }

    logInfo("All services started successfully!");
  } finally {
    if (worktreeBuildContextPrepared) {
      localBuildHandlers.cleanupWorktreeBuildArtifacts();
    }
  }
}

async function main(): Promise<number> {
  ensureDefaultVersions();

  const [, , command, ...args] = process.argv;

  if (!command) {
    usage();
    return EXIT_SUCCESS;
  }

  try {
    switch (command) {
      case "up":
      case "deploy":
        if (hasHelpArg(args)) {
          usageDeployCommand(command);
          return EXIT_SUCCESS;
        }
        printLogo();
        console.log(`${COLORS.lightBlue}${COLORS.bold}[DEPLOY] Deploying fhevm stack...${COLORS.reset}`);
        await deploy(args);
        console.log(`${COLORS.green}${COLORS.bold} [SUCCESS] fhevm stack deployment complete!${COLORS.reset}`);
        return EXIT_SUCCESS;
      case "down":
      case "clean":
        if (hasHelpArg(args)) {
          usageCleanCommand(command);
          return EXIT_SUCCESS;
        }
        commandHandlers.clean(args);
        return EXIT_SUCCESS;
      case "pause":
        if (hasHelpArg(args)) {
          usagePauseCommand("pause");
          return EXIT_SUCCESS;
        }
        printLogo();
        commandHandlers.pauseOrUnpause("pause", args[0]);
        return EXIT_SUCCESS;
      case "unpause":
        if (hasHelpArg(args)) {
          usagePauseCommand("unpause");
          return EXIT_SUCCESS;
        }
        printLogo();
        commandHandlers.pauseOrUnpause("unpause", args[0]);
        return EXIT_SUCCESS;
      case "test":
        if (hasHelpArg(args)) {
          usageTestCommand();
          return EXIT_SUCCESS;
        }
        printLogo();
        commandHandlers.runTests(args);
        return EXIT_SUCCESS;
      case "upgrade":
        if (hasHelpArg(args)) {
          usageUpgradeCommand();
          return EXIT_SUCCESS;
        }
        printLogo();
        commandHandlers.upgrade(args[0]);
        return EXIT_SUCCESS;
      case "logs":
        if (hasHelpArg(args)) {
          usageLogsCommand();
          return EXIT_SUCCESS;
        }
        commandHandlers.logs(args[0]);
        return EXIT_SUCCESS;
      case "telemetry-smoke":
        if (hasHelpArg(args)) {
          usageTelemetrySmokeCommand();
          return EXIT_SUCCESS;
        }
        commandHandlers.runTelemetrySmokeCheck(true);
        return EXIT_SUCCESS;
      case "trace": {
        const subcommand = args[0] ?? "status";
        if (isHelpToken(subcommand)) {
          usageTraceCommand();
          return EXIT_SUCCESS;
        }
        if (subcommand === "up") {
          await commandHandlers.traceUp();
          return EXIT_SUCCESS;
        }
        if (subcommand === "down") {
          commandHandlers.traceDown();
          return EXIT_SUCCESS;
        }
        if (subcommand === "status") {
          commandHandlers.traceStatus();
          return EXIT_SUCCESS;
        }
        usageError(`Unknown trace action: ${subcommand}`);
      }
      case "status":
        if (hasHelpArg(args)) {
          usageStatusCommand();
          return EXIT_SUCCESS;
        }
        commandHandlers.stackStatus();
        return EXIT_SUCCESS;
      case "help":
      case "-h":
      case "--help":
        usage();
        return EXIT_SUCCESS;
      default:
        usageError(`Unknown command: ${command}`);
    }
  } catch (error) {
    const errorCode = error instanceof CliError ? error.code : "E_UNEXPECTED";
    logError(`ERROR_CODE=${errorCode}`);
    if (error instanceof Error) {
      const lines = error.message.split("\n");
      for (const line of lines) {
        logError(line);
      }
    } else {
      logError(String(error));
    }
    if (error instanceof CliError && error.showUsage) {
      usage();
    }
    return EXIT_FAILURE;
  }
}

process.exit(await main());
