#!/usr/bin/env bun

import fs from "node:fs";
import path from "node:path";
import {
  COLORS,
  DEPLOYMENT_STEPS,
  LOCAL_CACHE_SERVICES,
  PROJECT,
  TEST_TYPE_CONFIG,
  UPGRADE_SERVICES,
  VERSION_ENTRIES,
  type DeploymentStep,
  type ServiceState,
} from "./manifest";
import { runCommand, sleep } from "./process";

const BUN_DIR = import.meta.dir;
const FHEVM_DIR = path.resolve(BUN_DIR, "..", "..");
const SCRIPTS_DIR = path.resolve(FHEVM_DIR, "scripts");
const COMPOSE_DIR = path.resolve(FHEVM_DIR, "docker-compose");
const ENV_DIR = path.resolve(FHEVM_DIR, "env", "staging");
const CONFIG_DIR = path.resolve(FHEVM_DIR, "config");

const STEP_BY_NAME = new Map(DEPLOYMENT_STEPS.map((step) => [step.name, step]));

const EXIT_SUCCESS = 0;
const EXIT_FAILURE = 1;

type DeployOptions = {
  forceBuild: boolean;
  localBuild: boolean;
  resumeStep?: string;
  onlyStep?: string;
};

type TestOptions = {
  verbose: boolean;
  network: string;
  grep?: string;
  noRelayer: boolean;
  noHardhatCompile: boolean;
};

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

function usage(): void {
  printLogo();
  console.log(`${COLORS.bold}Usage:${COLORS.reset} ${COLORS.yellow}fhevm-cli${COLORS.reset} ${COLORS.cyan}COMMAND [OPTIONS]${COLORS.reset}`);
  console.log("");
  console.log(`${COLORS.bold}${COLORS.lightBlue}Commands:${COLORS.reset}`);
  console.log(`  ${COLORS.yellow}deploy${COLORS.reset} ${COLORS.cyan}[--build] [--local] [--resume STEP] [--only STEP]${COLORS.reset}    Deploy the full fhevm stack`);
  console.log(`  ${COLORS.yellow}pause${COLORS.reset} ${COLORS.cyan}[CONTRACTS]${COLORS.reset}     Pause specific contracts (host|gateway)`);
  console.log(`  ${COLORS.yellow}unpause${COLORS.reset} ${COLORS.cyan}[CONTRACTS]${COLORS.reset}     Unpause specific contracts (host|gateway)`);
  console.log(`  ${COLORS.yellow}test${COLORS.reset} ${COLORS.cyan}[TYPE]${COLORS.reset}         Run tests (input-proof|user-decryption|public-decryption|delegated-user-decryption|random|random-subset|operators|erc20|debug)`);
  console.log(`  ${COLORS.yellow}upgrade${COLORS.reset} ${COLORS.cyan}[SERVICE]${COLORS.reset}   Upgrade specific service`);
  console.log(`  ${COLORS.yellow}clean${COLORS.reset}               Remove all containers and volumes`);
  console.log(`  ${COLORS.yellow}logs${COLORS.reset} ${COLORS.cyan}[SERVICE]${COLORS.reset}      View logs for a specific service`);
  console.log(`  ${COLORS.yellow}help${COLORS.reset}                Display this help message`);
  console.log("");
  console.log(`${COLORS.bold}${COLORS.lightBlue}Test Options:${COLORS.reset}`);
  console.log(`  ${COLORS.cyan}-v, --verbose${COLORS.reset}       Enable verbose output`);
  console.log(`  ${COLORS.cyan}-n, --network NAME${COLORS.reset}  Specify network (default: ${COLORS.green}staging${COLORS.reset})`);
  console.log(`  ${COLORS.cyan}-g, --grep PATTERN${COLORS.reset}  Override default test pattern`);
  console.log(`  ${COLORS.cyan}-r, --no-relayer${COLORS.reset}    Disable Rust relayer`);
  console.log(`  ${COLORS.cyan}--no-hardhat-compile${COLORS.reset}        Skip Hardhat compilation step`);
  console.log("");
  console.log(`${COLORS.bold}${COLORS.lightBlue}Examples:${COLORS.reset}`);
  console.log(`  ${COLORS.purple}./fhevm-cli deploy${COLORS.reset}`);
  console.log(`  ${COLORS.purple}./fhevm-cli deploy --build${COLORS.reset}`);
  console.log(`  ${COLORS.purple}./fhevm-cli deploy --local${COLORS.reset}`);
  console.log(`  ${COLORS.purple}./fhevm-cli deploy --resume kms-connector${COLORS.reset}`);
  console.log(`  ${COLORS.purple}./fhevm-cli deploy --only coprocessor${COLORS.reset}`);
  console.log(`  ${COLORS.purple}./fhevm-cli test input-proof${COLORS.reset}`);
  console.log(`  ${COLORS.purple}./fhevm-cli test input-proof --no-hardhat-compile${COLORS.reset}`);
  console.log(`  ${COLORS.purple}./fhevm-cli test user-decryption${COLORS.reset}`);
  console.log(`  ${COLORS.purple}./fhevm-cli test public-decrypt-http-ebool${COLORS.reset}`);
  console.log(`  ${COLORS.purple}./fhevm-cli test public-decrypt-http-mixed -n staging${COLORS.reset}`);
  console.log(`  ${COLORS.purple}./fhevm-cli test erc20${COLORS.reset}`);
  console.log(`  ${COLORS.purple}./fhevm-cli upgrade coprocessor${COLORS.reset}`);
  console.log(`${COLORS.blue}============================================================${COLORS.reset}`);
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

function ensureDefaultVersions(): void {
  const seen = new Set<string>();

  for (const version of VERSION_ENTRIES) {
    if (seen.has(version.envVar)) {
      continue;
    }
    seen.add(version.envVar);

    if (!process.env[version.envVar] || process.env[version.envVar] === "") {
      process.env[version.envVar] = version.defaultValue;
    }
  }
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

function prepareLocalEnvFile(component: string): string {
  const baseFile = baseEnvFile(component);
  const localFile = localEnvFile(component);

  if (!fs.existsSync(baseFile)) {
    throw new Error(`Base environment file for ${component} not found: ${baseFile}`);
  }

  logInfo(`Creating/updating local environment file for ${component}...`);
  fs.copyFileSync(baseFile, localFile);
  return localFile;
}

function prepareLocalConfigRelayer(): string {
  const baseFile = path.resolve(CONFIG_DIR, "relayer", "local.yaml");
  const localFile = path.resolve(CONFIG_DIR, "relayer", "local.yaml.local");

  if (!fs.existsSync(baseFile)) {
    throw new Error(`Base configuration file for relayer not found: ${baseFile}`);
  }

  logInfo("Creating/updating local configuration file for relayer...");
  fs.copyFileSync(baseFile, localFile);
  return localFile;
}

function prepareAllEnvFiles(): void {
  logInfo("Preparing all local environment files...");

  for (const component of uniqueComponents()) {
    prepareLocalEnvFile(component);
  }

  logInfo("All local environment files prepared successfully");
}

function configureLocalBuild(): void {
  logInfo("Enabling local BuildKit cache and disabling provenance attestations.");
  process.env.DOCKER_BUILDKIT = "1";
  process.env.COMPOSE_DOCKER_CLI_BUILD = "1";
  process.env.BUILDX_NO_DEFAULT_ATTESTATIONS = "1";
  process.env.DOCKER_BUILD_PROVENANCE = "false";
  process.env.FHEVM_CARGO_PROFILE = "local";

  const cacheRoot = process.env.FHEVM_BUILDX_CACHE_DIR ?? ".buildx-cache";
  process.env.FHEVM_BUILDX_CACHE_DIR = cacheRoot;
  fs.mkdirSync(cacheRoot, { recursive: true });

  const setLocalCache = (serviceName: string): void => {
    const serviceKey = serviceName.replace(/-/g, "_").toUpperCase();
    const cacheDir = path.resolve(cacheRoot, serviceName);
    fs.mkdirSync(cacheDir, { recursive: true });
    process.env[`FHEVM_CACHE_FROM_${serviceKey}`] = `type=local,src=${cacheDir}`;
    process.env[`FHEVM_CACHE_TO_${serviceKey}`] = `type=local,dest=${cacheDir},mode=max`;
  };

  const coprocessorCacheDir = path.resolve(cacheRoot, "coprocessor");
  fs.mkdirSync(coprocessorCacheDir, { recursive: true });
  process.env.FHEVM_CACHE_FROM_COPROCESSOR = `type=local,src=${coprocessorCacheDir}`;
  process.env.FHEVM_CACHE_TO_COPROCESSOR = `type=local,dest=${coprocessorCacheDir},mode=max`;

  const kmsConnectorCacheDir = path.resolve(cacheRoot, "kms-connector");
  fs.mkdirSync(kmsConnectorCacheDir, { recursive: true });
  process.env.FHEVM_CACHE_FROM_KMS_CONNECTOR = `type=local,src=${kmsConnectorCacheDir}`;
  process.env.FHEVM_CACHE_TO_KMS_CONNECTOR = `type=local,dest=${kmsConnectorCacheDir},mode=max`;

  for (const service of LOCAL_CACHE_SERVICES) {
    setLocalCache(service);
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

function detectKeyBootstrapNotReady(logs: string): boolean {
  const patterns = [
    /CrsNotGenerated/i,
    /CrsgenNotRequested/i,
    /KeygenNotRequested/i,
    /PrepKeygenNotRequested/i,
    /key bootstrap.*not ready/i,
    /bootstrap key.*not ready/i,
    /materials are not ready/i,
  ];

  return patterns.some((pattern) => pattern.test(logs));
}

function printFailureHints(serviceName: string, stepHint: string, exitCode: number, oomKilled: boolean, logs: string): void {
  if (exitCode === 137 || oomKilled) {
    logError(`${serviceName} looks OOM-killed (exit code: ${exitCode}, OOMKilled: ${oomKilled}).`);
    logError(`Action: increase Docker memory and retry from this step: ./fhevm-cli deploy --resume ${stepHint}`);
  }

  if (detectKeyBootstrapNotReady(logs)) {
    logError(`Detected key-bootstrap-not-ready state while starting ${serviceName}.`);
    logError("Action: wait for gateway keygen/CRS generation to settle, then retry: ./fhevm-cli deploy --resume gateway-sc");
  }
}

function waitForService(serviceName: string, stepHint: string, expected: ServiceState): void {
  const expectRunning = expected === "running";
  const maxRetries = 30;
  const retryInterval = 5;

  if (expectRunning) {
    logInfo(`Waiting for ${serviceName} to be running...`);
  } else {
    logInfo(`Waiting for ${serviceName} to complete...`);
  }

  for (let i = 1; i <= maxRetries; i += 1) {
    const containerLookup = runCommand(
      ["docker", "ps", "-a", "--filter", `name=${serviceName}$`, "--format", "{{.ID}}"],
      { capture: true, check: true },
    );

    const containerId = containerLookup.stdout.trim();
    if (!containerId) {
      logWarn(`Container for ${serviceName} not found, waiting...`);
      sleep(retryInterval);
      continue;
    }

    const status = runCommand(["docker", "inspect", "--format", "{{.State.Status}}", containerId], {
      capture: true,
      check: true,
    }).stdout.trim();

    const exitCodeRaw = runCommand(["docker", "inspect", "--format", "{{.State.ExitCode}}", containerId], {
      capture: true,
      check: true,
    }).stdout.trim();

    const oomKilledRaw = runCommand(["docker", "inspect", "--format", "{{.State.OOMKilled}}", containerId], {
      capture: true,
      check: false,
      allowFailure: true,
    }).stdout.trim();

    const exitCode = Number.parseInt(exitCodeRaw || "0", 10);
    const oomKilled = oomKilledRaw.toLowerCase() === "true";

    if (expectRunning && status === "running") {
      logInfo(`${serviceName} is now running`);
      return;
    }

    if (!expectRunning && status === "exited" && exitCode === 0) {
      logInfo(`${serviceName} completed successfully`);
      return;
    }

    if (status === "exited" && exitCode !== 0) {
      logError(`${serviceName} failed with exit code ${exitCode}`);
      const logs = runCommand(["docker", "logs", containerId], {
        capture: true,
        check: false,
        allowFailure: true,
      });

      const combinedLogs = [logs.stdout.trim(), logs.stderr.trim()].filter(Boolean).join("\n");
      printFailureHints(serviceName, stepHint, exitCode, oomKilled, combinedLogs);
      if (combinedLogs) {
        console.error(combinedLogs);
      }
      throw new Error(`Service ${serviceName} failed`);
    }

    if (i < maxRetries) {
      logWarn(`${serviceName} not ready yet (status: ${status}), waiting ${retryInterval}s... (${i}/${maxRetries})`);
      sleep(retryInterval);
      continue;
    }

    logError(`${serviceName} failed to reach desired state within the expected time`);
    const logs = runCommand(["docker", "logs", containerId], {
      capture: true,
      check: false,
      allowFailure: true,
    });
    const combinedLogs = [logs.stdout.trim(), logs.stderr.trim()].filter(Boolean).join("\n");
    printFailureHints(serviceName, stepHint, exitCode, oomKilled, combinedLogs);
    if (combinedLogs) {
      console.error(combinedLogs);
    }
    throw new Error(`Service ${serviceName} timed out`);
  }
}

function runComposeStep(step: DeploymentStep, useBuild: boolean): void {
  if (!step.component) {
    throw new Error(`Step ${step.name} has no compose component`);
  }

  const envFile = localEnvFile(step.component);
  const compose = composeFile(step.component);

  if (useBuild) {
    logInfo(`Building and starting ${step.description} using local environment file...`);
  } else {
    logInfo(`Starting ${step.description} using local environment file...`);
  }
  logInfo(`Using environment file: ${envFile}`);

  const command = [
    "docker",
    "compose",
    "-p",
    PROJECT,
    "--env-file",
    envFile,
    "-f",
    compose,
    "up",
  ];

  if (useBuild) {
    command.push("--build");
  }
  command.push("-d");

  const upResult = runCommand(command, { check: false, allowFailure: true });
  if (upResult.status !== 0) {
    if (useBuild) {
      throw new Error(`Failed to build and start ${step.description}`);
    }
    throw new Error(`Failed to start ${step.description}`);
  }

  for (const check of step.serviceChecks) {
    waitForService(check.service, step.name, check.state);
  }
}

function getMinioIp(containerName: string): void {
  const inspect = runCommand(
    ["docker", "inspect", "-f", "{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}", containerName],
    { capture: true, check: true },
  );
  const minioIp = inspect.stdout.trim();

  if (!minioIp) {
    throw new Error(`Could not find IP address for ${containerName} container`);
  }

  const coprocessorLocalEnv = localEnvFile("coprocessor");
  if (!fs.existsSync(coprocessorLocalEnv)) {
    throw new Error(`Coprocessor local env file not found: ${coprocessorLocalEnv}`);
  }

  const original = fs.readFileSync(coprocessorLocalEnv, "utf8");
  fs.writeFileSync(`${coprocessorLocalEnv}.bak`, original, "utf8");

  const patched = original.replace(/AWS_ENDPOINT_URL=http:\/\/[^:]*:9000/g, `AWS_ENDPOINT_URL=http://${minioIp}:9000`);
  fs.writeFileSync(coprocessorLocalEnv, patched, "utf8");

  console.log(`Found ${containerName} container IP: ${minioIp}`);
  console.log(`Updated AWS_ENDPOINT_URL to http://${minioIp}:9000`);
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
  const compose = composeFile(component);
  if (!fs.existsSync(compose)) {
    return;
  }

  const envFile = resolveCleanupEnvFile(component);

  if (envFile) {
    logInfo(`Stopping ${component} services...`);
    runCommand(
      ["docker", "compose", "-p", PROJECT, "--env-file", envFile, "-f", compose, "down", "-v", "--remove-orphans"],
      { check: false, allowFailure: true },
    );
    return;
  }

  logWarn(`Env file missing for ${component}, attempting cleanup without explicit env file`);
  runCommand(["docker", "compose", "-p", PROJECT, "-f", compose, "down", "-v", "--remove-orphans"], {
    check: false,
    allowFailure: true,
  });
}

function cleanupFull(): void {
  logWarn("Setup new environment, cleaning up...");
  runCommand(["docker", "compose", "-p", PROJECT, "down", "-v", "--remove-orphans"], { check: true });
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

function isContainerRunning(containerName: string): boolean {
  const result = runCommand(["docker", "ps", "--filter", `name=${containerName}`, "--format", "{{.Names}}"], {
    capture: true,
    check: true,
  });

  return result.stdout
    .split("\n")
    .map((line) => line.trim())
    .filter(Boolean)
    .includes(containerName);
}

function parseDeployArgs(args: string[]): DeployOptions {
  const options: DeployOptions = {
    forceBuild: false,
    localBuild: false,
  };

  let expectResumeStep = false;
  let expectOnlyStep = false;

  for (const arg of args) {
    if (expectResumeStep) {
      options.resumeStep = arg;
      expectResumeStep = false;
      continue;
    }

    if (expectOnlyStep) {
      options.onlyStep = arg;
      expectOnlyStep = false;
      continue;
    }

    if (arg === "--build") {
      options.forceBuild = true;
      logInfo("Force build option detected. Services will be rebuilt.");
      continue;
    }

    if (arg === "--local" || arg === "--dev") {
      options.localBuild = true;
      logInfo("Local optimization option detected.");
      continue;
    }

    if (arg === "--resume") {
      expectResumeStep = true;
      continue;
    }

    if (arg === "--only") {
      expectOnlyStep = true;
      continue;
    }

    throw new Error(`Unknown argument for deploy: ${arg}`);
  }

  const validSteps = stepNames().join(" ");

  if (expectResumeStep) {
    throw new Error(`--resume requires a step name\nValid steps are: ${validSteps}`);
  }

  if (expectOnlyStep) {
    throw new Error(`--only requires a step name\nValid steps are: ${validSteps}`);
  }

  if (options.resumeStep && stepIndex(options.resumeStep) === -1) {
    throw new Error(`Invalid resume step: ${options.resumeStep}\nValid steps are: ${validSteps}`);
  }

  if (options.onlyStep && stepIndex(options.onlyStep) === -1) {
    throw new Error(`Invalid step: ${options.onlyStep}\nValid steps are: ${validSteps}`);
  }

  if (options.resumeStep && options.onlyStep) {
    throw new Error("Cannot use --resume and --only together");
  }

  if (options.resumeStep) {
    logInfo(`Resume mode: starting from step '${options.resumeStep}'`);
  }

  if (options.onlyStep) {
    logInfo(`Only mode: deploying only step '${options.onlyStep}'`);
  }

  return options;
}

function deploy(args: string[]): void {
  const options = parseDeployArgs(args);

  if (options.localBuild) {
    configureLocalBuild();
  }

  if (options.onlyStep) {
    cleanupSingleStep(options.onlyStep);
  } else if (options.resumeStep) {
    cleanupFromStep(options.resumeStep);
  } else {
    cleanupFull();
  }

  prepareAllEnvFiles();
  prepareLocalConfigRelayer();

  logInfo("Deploying FHEVM Stack...");

  const buildTag = options.forceBuild ? " (local build)" : "";
  printVersionSummary(buildTag);

  for (const step of DEPLOYMENT_STEPS) {
    if (shouldSkipStep(step.name, options)) {
      if (options.onlyStep) {
        logInfo(`Skipping step: ${step.name} (only mode: ${options.onlyStep})`);
      } else {
        logInfo(`Skipping step: ${step.name} (resuming from ${options.resumeStep})`);
      }

      if (step.name === "minio" && isContainerRunning("fhevm-minio")) {
        getMinioIp("fhevm-minio");
      }
      continue;
    }

    if (step.name === "kms-signer") {
      sleep(5);
      runCommand([path.resolve(SCRIPTS_DIR, "setup-kms-signer-address.sh")], { check: true });
      continue;
    }

    const useBuild = options.forceBuild && step.buildable;
    runComposeStep(step, useBuild);

    if (step.name === "minio") {
      getMinioIp("fhevm-minio");
    }
  }

  logInfo("All services started successfully!");
}

function parseTestArgs(args: string[]): { testType: string; options: TestOptions } {
  const testType = args[0] ?? "";
  const options: TestOptions = {
    verbose: false,
    network: "staging",
    noRelayer: false,
    noHardhatCompile: false,
  };

  let i = 1;
  while (i < args.length) {
    const arg = args[i];

    if (arg === "-v" || arg === "--verbose") {
      options.verbose = true;
      i += 1;
      continue;
    }

    if (arg === "-n" || arg === "--network") {
      const value = args[i + 1];
      if (!value || value.startsWith("-")) {
        throw new Error("Network argument missing");
      }
      options.network = value;
      i += 2;
      continue;
    }

    if (arg === "-g" || arg === "--grep") {
      const value = args[i + 1];
      if (!value || value.startsWith("-")) {
        throw new Error("Grep pattern missing");
      }
      options.grep = value;
      i += 2;
      continue;
    }

    if (arg === "-r" || arg === "--no-relayer") {
      options.noRelayer = true;
      i += 1;
      continue;
    }

    if (arg === "--no-hardhat-compile") {
      options.noHardhatCompile = true;
      i += 1;
      continue;
    }

    throw new Error(`Unknown option: ${arg}`);
  }

  return { testType, options };
}

function runTests(args: string[]): void {
  const { testType, options } = parseTestArgs(args);

  const dockerArgs: string[] = ["./run-tests.sh"];

  if (options.verbose) {
    dockerArgs.push("-v");
  }

  dockerArgs.push("-n", options.network);

  if (options.noRelayer) {
    dockerArgs.push("-r");
  }

  if (options.noHardhatCompile) {
    dockerArgs.push("--no-hardhat-compile");
  }

  let logMessage = "";

  if (options.grep) {
    dockerArgs.push("-g", options.grep);
  } else {
    const config = TEST_TYPE_CONFIG[testType];
    if (!config) {
      throw new Error(`Unknown test type: ${testType}`);
    }

    if (config.debugShell) {
      console.log(color("[DEBUG] Starting debug session...", `${COLORS.lightBlue}${COLORS.bold}`));
      runCommand(["docker", "exec", "-it", "fhevm-test-suite-e2e-debug", "bash"], { check: true });
      return;
    }

    if (config.parallel) {
      dockerArgs.push("--parallel");
    }

    if (config.grep) {
      dockerArgs.push("-g", config.grep);
    }

    if (config.logMessage) {
      logMessage = color(config.logMessage, `${COLORS.lightBlue}${COLORS.bold}`);
    }
  }

  if (testType !== "debug") {
    if (logMessage) {
      console.log(logMessage);
    }
    runCommand(["docker", "exec", "fhevm-test-suite-e2e-debug", ...dockerArgs], { check: true });
  }
}

function pauseOrUnpause(command: "pause" | "unpause", contractsArg?: string): void {
  if (contractsArg !== "gateway" && contractsArg !== "host") {
    throw new Error(`Unknown service: ${contractsArg ?? ""}`);
  }

  const action = command === "pause" ? "PAUSE" : "UNPAUSE";
  const composePath = path.resolve(COMPOSE_DIR, `${contractsArg}-${command}-docker-compose.yml`);
  const waitService = `${contractsArg}-sc-${command}`;

  console.log(`${COLORS.lightBlue}[${action}]${COLORS.reset} ${COLORS.bold}${command === "pause" ? "Pausing" : "Unpausing"} ${contractsArg}...${COLORS.reset}`);
  runCommand(["docker", "compose", "-p", PROJECT, "-f", composePath, "up", "-d"], { check: true });
  console.log(`${COLORS.yellow}[WAIT]${COLORS.reset} ${COLORS.bold}Waiting for ${command} operation to complete...${COLORS.reset}`);
  runCommand(["docker", "compose", "-p", PROJECT, "-f", composePath, "wait", waitService], { check: true });
  console.log(`${COLORS.green}[SUCCESS]${COLORS.reset} ${COLORS.bold}${contractsArg} ${command}d successfully${COLORS.reset}`);
}

function upgrade(service?: string): void {
  if (!service || !UPGRADE_SERVICES.includes(service)) {
    throw new Error(`Unknown service: ${service ?? ""}`);
  }

  const envFile = localEnvFile(service);
  const compose = composeFile(service);

  console.log(`${COLORS.lightBlue}[UPGRADE]${COLORS.reset} ${COLORS.bold}Upgrading ${service}...${COLORS.reset}`);
  runCommand(["docker", "compose", "-p", PROJECT, "--env-file", envFile, "-f", compose, "up", "-d"], { check: true });
  console.log(`${COLORS.green}[SUCCESS]${COLORS.reset} ${COLORS.bold}${service} upgraded successfully${COLORS.reset}`);
}

function clean(): void {
  console.log(`${COLORS.lightBlue}[CLEAN]${COLORS.reset} ${COLORS.bold}Cleaning up FHEVM stack...${COLORS.reset}`);
  runCommand(["docker", "compose", "-p", PROJECT, "down", "-v", "--remove-orphans"], { check: true });
  console.log(`${COLORS.green}[SUCCESS]${COLORS.reset} ${COLORS.bold}FHEVM stack cleaned successfully${COLORS.reset}`);
}

function logs(service?: string): void {
  if (!service) {
    throw new Error("Service name is required");
  }

  console.log(`${COLORS.lightBlue}[LOGS]${COLORS.reset} ${COLORS.bold}Showing logs for ${service}...${COLORS.reset}`);
  runCommand(["docker", "logs", service], { check: true });
}

function main(): number {
  ensureDefaultVersions();

  const [, , command, ...args] = process.argv;

  if (!command) {
    usage();
    return EXIT_SUCCESS;
  }

  try {
    switch (command) {
      case "deploy":
        printLogo();
        console.log(`${COLORS.lightBlue}${COLORS.bold}[DEPLOY] Deploying fhevm stack...${COLORS.reset}`);
        deploy(args);
        console.log(`${COLORS.green}${COLORS.bold} [SUCCESS] fhevm stack deployment complete!${COLORS.reset}`);
        return EXIT_SUCCESS;
      case "pause":
        printLogo();
        pauseOrUnpause("pause", args[0]);
        return EXIT_SUCCESS;
      case "unpause":
        printLogo();
        pauseOrUnpause("unpause", args[0]);
        return EXIT_SUCCESS;
      case "test":
        printLogo();
        runTests(args);
        return EXIT_SUCCESS;
      case "upgrade":
        printLogo();
        upgrade(args[0]);
        return EXIT_SUCCESS;
      case "clean":
        clean();
        return EXIT_SUCCESS;
      case "logs":
        logs(args[0]);
        return EXIT_SUCCESS;
      case "help":
      case "-h":
      case "--help":
        usage();
        return EXIT_SUCCESS;
      default:
        throw new Error(`Unknown command: ${command}`);
    }
  } catch (error) {
    if (error instanceof Error) {
      const lines = error.message.split("\n");
      for (const line of lines) {
        logError(line);
      }
    } else {
      logError(String(error));
    }
    usage();
    return EXIT_FAILURE;
  }
}

process.exit(main());
