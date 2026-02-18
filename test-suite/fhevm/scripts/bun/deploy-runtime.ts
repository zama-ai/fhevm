import fs from "node:fs";
import type { DeploymentStep, ServiceState } from "./manifest";
import type { RunCommandFn, SleepFn } from "./process";

type GatewayAddressSyncOptions = {
  warnOnMissing?: boolean;
};

type DeployRuntimeHandlersDeps = {
  PROJECT: string;
  KEY_COUNTER_BASE: bigint;
  CRS_COUNTER_BASE: bigint;
  GATEWAY_BOOTSTRAP_MAX_ATTEMPTS: number;
  GATEWAY_BOOTSTRAP_RETRY_DELAY_SECONDS: number;
  SERVICE_WAIT_RETRY_INTERVAL_SECONDS: number;
  SERVICE_WAIT_MAX_RETRIES_DEFAULT: number;
  SERVICE_WAIT_MAX_RETRIES_GATEWAY_SC_DEPLOY: number;
  OOM_KILLED_EXIT_CODE: number;
  EXPECTED_PAUSE_SELECTOR: string;
  ENFORCED_PAUSE_SELECTOR: string;
  runCommand: RunCommandFn;
  sleep: SleepFn;
  logInfo: (message: string) => void;
  logWarn: (message: string) => void;
  logError: (message: string) => void;
  cliCommand: (args?: string) => string;
  cliError: (code: string, message: string, options?: { showUsage?: boolean }) => never;
  errorMessage: (error: unknown) => string;
  composeFile: (component: string) => string;
  localEnvFile: (component: string) => string;
  resolveMainCoprocessorCompose: () => string;
  resolveComposeForBuild: (composePath: string, useBuild: boolean) => string;
  resolveProjectContainerName: (logicalName: string) => string | undefined;
  isContainerRunningExact: (containerName: string) => boolean;
  readEnvValue: (filePath: string, key: string) => string | undefined;
  upsertEnvValue: (filePath: string, key: string, value: string) => void;
  findAdditionalCoprocessorIndices: () => number[];
  additionalCoprocessorEnvFile: (instanceIdx: number) => string;
};

type GatewayBootstrapState = {
  activeKeyId: bigint;
  activeCrsId: bigint;
};

const GATEWAY_ADDRESS_KEYS = [
  "GATEWAY_CONFIG_ADDRESS",
  "MULTICHAIN_ACL_ADDRESS",
  "CIPHERTEXT_COMMITS_ADDRESS",
  "DECRYPTION_ADDRESS",
  "KMS_GENERATION_ADDRESS",
  "INPUT_VERIFICATION_ADDRESS",
  "PROTOCOL_PAYMENT_ADDRESS",
  "PAUSER_SET_ADDRESS",
] as const;

export function createDeployRuntimeHandlers(deps: DeployRuntimeHandlersDeps) {
  const {
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
    resolveComposeForBuild,
    resolveProjectContainerName,
    isContainerRunningExact,
    readEnvValue,
    upsertEnvValue,
    findAdditionalCoprocessorIndices,
    additionalCoprocessorEnvFile,
  } = deps;

  function parseCastBigInt(rawOutput: string): bigint | undefined {
    const lines = rawOutput.split("\n").map((line) => line.trim()).filter(Boolean);
    const leadingPattern = /^(0x[0-9a-fA-F]+|\d+)\b/;

    for (const line of lines) {
      const match = line.match(leadingPattern);
      if (!match?.[1]) {
        continue;
      }
      try {
        return BigInt(match[1]);
      } catch {
        // ignore and continue trying other lines
      }
    }

    const fallbackTokenPattern = /\b0x[0-9a-fA-F]+\b|\b\d+\b/g;
    const tokens = rawOutput.match(fallbackTokenPattern);
    if (!tokens) {
      return undefined;
    }
    for (const token of tokens) {
      try {
        return BigInt(token);
      } catch {
        // ignore and continue trying other tokens
      }
    }

    return undefined;
  }

  function readGatewayBootstrapState(kmsGenerationAddress: string): GatewayBootstrapState {
    const gatewayNodeContainer = resolveProjectContainerName("gateway-node");
    if (!gatewayNodeContainer || !isContainerRunningExact("gateway-node")) {
      throw new Error("gateway-node container is not running");
    }

    const callActiveKey = runCommand(
      [
        "docker",
        "exec",
        gatewayNodeContainer,
        "cast",
        "call",
        kmsGenerationAddress,
        "getActiveKeyId()(uint256)",
        "--rpc-url",
        "http://127.0.0.1:8546",
      ],
      { capture: true, check: false, allowFailure: true },
    );
    const callActiveCrs = runCommand(
      [
        "docker",
        "exec",
        gatewayNodeContainer,
        "cast",
        "call",
        kmsGenerationAddress,
        "getActiveCrsId()(uint256)",
        "--rpc-url",
        "http://127.0.0.1:8546",
      ],
      { capture: true, check: false, allowFailure: true },
    );

    if (callActiveKey.status !== 0 || callActiveCrs.status !== 0) {
      const combinedOutput = [
        callActiveKey.stdout.trim(),
        callActiveKey.stderr.trim(),
        callActiveCrs.stdout.trim(),
        callActiveCrs.stderr.trim(),
      ]
        .filter(Boolean)
        .join("\n");
      throw new Error(`Could not query gateway bootstrap state via cast.${combinedOutput ? `\n${combinedOutput}` : ""}`);
    }

    const activeKeyId = parseCastBigInt(callActiveKey.stdout);
    const activeCrsId = parseCastBigInt(callActiveCrs.stdout);
    if (activeKeyId === undefined || activeCrsId === undefined) {
      const combinedOutput = [callActiveKey.stdout.trim(), callActiveCrs.stdout.trim()].filter(Boolean).join("\n");
      throw new Error(
        `Could not parse gateway bootstrap state output from cast.${combinedOutput ? `\n${combinedOutput}` : ""}`,
      );
    }

    return { activeKeyId, activeCrsId };
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
    if (exitCode === OOM_KILLED_EXIT_CODE || oomKilled) {
      logError(`${serviceName} looks OOM-killed (exit code: ${exitCode}, OOMKilled: ${oomKilled}).`);
      logError(`Action: increase Docker memory and retry from this step: ${cliCommand(`deploy --resume ${stepHint}`)}`);
    }

    if (detectKeyBootstrapNotReady(logs)) {
      logError(`Detected key-bootstrap-not-ready state while starting ${serviceName}.`);
      logError(`Action: wait for gateway keygen/CRS generation to settle, then retry: ${cliCommand("deploy --resume gateway-sc")}`);
    }
  }

  function waitForService(serviceName: string, stepHint: string, expected: ServiceState): void {
    const expectRunning = expected === "running";
    const retryInterval = SERVICE_WAIT_RETRY_INTERVAL_SECONDS;
    const maxRetries = serviceName === "gateway-sc-deploy"
      ? SERVICE_WAIT_MAX_RETRIES_GATEWAY_SC_DEPLOY
      : SERVICE_WAIT_MAX_RETRIES_DEFAULT;

    if (expectRunning) {
      logInfo(`Waiting for ${serviceName} to be running...`);
    } else {
      logInfo(`Waiting for ${serviceName} to complete...`);
    }

    for (let i = 1; i <= maxRetries; i += 1) {
      const containerName = resolveProjectContainerName(serviceName);
      if (!containerName) {
        logWarn(`Container for ${serviceName} not found, waiting...`);
        sleep(retryInterval);
        continue;
      }

      const status = runCommand(["docker", "inspect", "--format", "{{.State.Status}}", containerName], {
        capture: true,
        check: true,
      }).stdout.trim();

      const exitCodeRaw = runCommand(["docker", "inspect", "--format", "{{.State.ExitCode}}", containerName], {
        capture: true,
        check: true,
      }).stdout.trim();

      const oomKilledRaw = runCommand(["docker", "inspect", "--format", "{{.State.OOMKilled}}", containerName], {
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
        const logs = runCommand(["docker", "logs", containerName], {
          capture: true,
          check: false,
          allowFailure: true,
        });

        const combinedLogs = [logs.stdout.trim(), logs.stderr.trim()].filter(Boolean).join("\n");
        printFailureHints(serviceName, stepHint, exitCode, oomKilled, combinedLogs);
        if (combinedLogs) {
          console.error(combinedLogs);
        }
        if (exitCode === OOM_KILLED_EXIT_CODE || oomKilled) {
          cliError("E_SERVICE_OOM", `Service ${serviceName} failed (OOM)`);
        }
        if (detectKeyBootstrapNotReady(combinedLogs)) {
          cliError("E_KEY_BOOTSTRAP_NOT_READY", `Service ${serviceName} failed (key bootstrap not ready)`);
        }
        cliError("E_SERVICE_FAILED", `Service ${serviceName} failed`);
      }

      if (i < maxRetries) {
        logWarn(`${serviceName} not ready yet (status: ${status}), waiting ${retryInterval}s... (${i}/${maxRetries})`);
        sleep(retryInterval);
        continue;
      }

      logError(`${serviceName} failed to reach desired state within the expected time`);
      const logs = runCommand(["docker", "logs", containerName], {
        capture: true,
        check: false,
        allowFailure: true,
      });
      const combinedLogs = [logs.stdout.trim(), logs.stderr.trim()].filter(Boolean).join("\n");
      printFailureHints(serviceName, stepHint, exitCode, oomKilled, combinedLogs);
      if (combinedLogs) {
        console.error(combinedLogs);
      }
      if (detectKeyBootstrapNotReady(combinedLogs)) {
        cliError("E_KEY_BOOTSTRAP_NOT_READY", `Service ${serviceName} timed out (key bootstrap not ready)`);
      }
      cliError("E_SERVICE_TIMEOUT", `Service ${serviceName} timed out`);
    }
  }

  function runComposeUp(command: string[]): { status: number; output: string } {
    const result = runCommand(command, { capture: true, check: false, allowFailure: true });
    if (result.stdout) {
      process.stdout.write(result.stdout);
    }
    if (result.stderr) {
      process.stderr.write(result.stderr);
    }
    const combinedOutput = [result.stdout.trim(), result.stderr.trim()].filter(Boolean).join("\n");
    if (result.status !== 0 && combinedOutput.includes("port is already allocated")) {
      const ports = [...combinedOutput.matchAll(/0\.0\.0\.0:(\d+)/g)]
        .map((match) => match[1])
        .filter(Boolean);
      const uniquePorts = [...new Set(ports)];
      if (uniquePorts.length > 0) {
        for (const port of uniquePorts) {
          const holder = runCommand(
            ["docker", "ps", "--filter", `publish=${port}`, "--format", "{{.Names}}"],
            { capture: true, check: false, allowFailure: true },
          );
          const holders = holder.stdout
            .split("\n")
            .map((line) => line.trim())
            .filter(Boolean);
          logWarn(`Port ${port} is already in use by: ${holders.length > 0 ? holders.join(", ") : "unknown container"}`);
        }
        logWarn("Stop conflicting containers and retry.");
      }
    }
    return {
      status: result.status,
      output: combinedOutput,
    };
  }

  function parseEnvText(content: string): Record<string, string> {
    const values: Record<string, string> = {};
    for (const rawLine of content.split("\n")) {
      const line = rawLine.trim();
      if (line === "" || line.startsWith("#")) {
        continue;
      }
      const idx = line.indexOf("=");
      if (idx <= 0) {
        continue;
      }
      const key = line.slice(0, idx).trim();
      const value = line.slice(idx + 1).trim();
      values[key] = value;
    }
    return values;
  }

  function syncGatewayAddressesFromVolume(envFile: string, options: GatewayAddressSyncOptions = {}): boolean {
    const warnOnMissing = options.warnOnMissing ?? true;
    const read = runCommand(
      ["docker", "run", "--rm", "-v", `${PROJECT}_addresses-volume:/data`, "alpine:3.20", "sh", "-lc", "cat /data/.env.gateway 2>/dev/null || true"],
      {
        capture: true,
        check: false,
        allowFailure: true,
      },
    );

    const parsed = parseEnvText(read.stdout);
    let updated = 0;

    for (const key of GATEWAY_ADDRESS_KEYS) {
      const value = parsed[key];
      if (!value) {
        continue;
      }
      upsertEnvValue(envFile, key, value);
      updated += 1;
    }

    if (updated > 0) {
      logInfo(`Synced ${updated} gateway contract address env var(s) from addresses volume.`);
      return true;
    }

    if (warnOnMissing) {
      logWarn("Could not sync gateway contract addresses from addresses volume; keeping current env values.");
    }
    return false;
  }

  function resolveKmsGenerationAddressForBootstrapCheck(envFile: string): string | undefined {
    const existing = readEnvValue(envFile, "KMS_GENERATION_ADDRESS");
    if (existing) {
      return existing;
    }

    const synced = syncGatewayAddressesFromVolume(envFile, { warnOnMissing: false });
    if (!synced) {
      return undefined;
    }

    return readEnvValue(envFile, "KMS_GENERATION_ADDRESS");
  }

  function hasGatewayAddressesVolume(): boolean {
    const list = runCommand(
      ["docker", "volume", "ls", "--filter", `name=^${PROJECT}_addresses-volume$`, "--format", "{{.Name}}"],
      { capture: true, check: false, allowFailure: true },
    );

    return list.stdout
      .split("\n")
      .map((line) => line.trim())
      .some((line) => line === `${PROJECT}_addresses-volume`);
  }

  function detectExpectedPause(logs: string): boolean {
    return /ExpectedPause/i.test(logs) || logs.toLowerCase().includes(EXPECTED_PAUSE_SELECTOR);
  }

  function detectEnforcedPause(logs: string): boolean {
    return /EnforcedPause/i.test(logs) || logs.toLowerCase().includes(ENFORCED_PAUSE_SELECTOR);
  }

  function readContainerLogs(containerName: string): string {
    const resolvedContainer = resolveProjectContainerName(containerName) ?? containerName;
    const logs = runCommand(["docker", "logs", resolvedContainer], {
      capture: true,
      check: false,
      allowFailure: true,
    });
    return [logs.stdout.trim(), logs.stderr.trim()].filter(Boolean).join("\n");
  }

  function runNodeStepUpOnly(step: DeploymentStep, useBuild: boolean): void {
    if (!step.component) {
      throw new Error(`Step ${step.name} has no compose component`);
    }

    const envFile = localEnvFile(step.component);
    const compose = resolveComposeForBuild(composeFile(step.component), useBuild);

    if (useBuild) {
      logInfo(`Building and starting ${step.description} using local environment file...`);
    } else {
      logInfo(`Starting ${step.description} using local environment file...`);
    }
    logInfo(`Using environment file: ${envFile}`);

    const command = ["docker", "compose", "-p", PROJECT, "--env-file", envFile, "-f", compose, "up"];
    if (useBuild) {
      command.push("--build");
    }
    command.push("-d");

    const upResult = runComposeUp(command);
    if (upResult.status !== 0) {
      if (useBuild) {
        throw new Error(`Failed to build and start ${step.description}`);
      }
      throw new Error(`Failed to start ${step.description}`);
    }
  }

  function runParallelHostGatewayNodeStartup(hostNodeStep: DeploymentStep, gatewayNodeStep: DeploymentStep, useBuild: boolean): void {
    runNodeStepUpOnly(hostNodeStep, useBuild);
    runNodeStepUpOnly(gatewayNodeStep, useBuild);

    for (const check of hostNodeStep.serviceChecks) {
      waitForService(check.service, hostNodeStep.name, check.state);
    }
    for (const check of gatewayNodeStep.serviceChecks) {
      waitForService(check.service, gatewayNodeStep.name, check.state);
    }
  }

  function prebuildGatewayImage(): void {
    const compose = resolveComposeForBuild(composeFile("gateway-sc"), true);
    const envFile = localEnvFile("gateway-sc");
    if (!fs.existsSync(compose) || !fs.existsSync(envFile)) {
      throw new Error("Gateway prebuild prerequisites are missing (compose or env file not found)");
    }

    logInfo("Prebuilding gateway-contracts image once for this deploy (shared across gateway steps).");
    runCommand(
      ["docker", "compose", "-p", PROJECT, "--env-file", envFile, "-f", compose, "build", "gateway-sc-deploy"],
      { check: true },
    );
  }

  function waitForGatewayBootstrapReady(): void {
    const envFile = localEnvFile("gateway-sc");
    const kmsGenerationAddress = resolveKmsGenerationAddressForBootstrapCheck(envFile);
    if (!kmsGenerationAddress) {
      if (!hasGatewayAddressesVolume()) {
        logWarn(
          "Skipping gateway key bootstrap readiness check: gateway addresses volume is unavailable in this environment.",
        );
        return;
      }

      cliError(
        "E_GATEWAY_ADDRESS_MISSING",
        "KMS_GENERATION_ADDRESS is missing in gateway-sc env file after gateway deployment. " +
          `Retry: ${cliCommand("deploy --resume gateway-sc")}`,
      );
    }

    const maxAttempts = GATEWAY_BOOTSTRAP_MAX_ATTEMPTS;
    const retryDelaySeconds = GATEWAY_BOOTSTRAP_RETRY_DELAY_SECONDS;
    let lastState = "unknown";

    for (let attempt = 1; attempt <= maxAttempts; attempt += 1) {
      try {
        const state = readGatewayBootstrapState(kmsGenerationAddress);
        const keyReady = state.activeKeyId > KEY_COUNTER_BASE;
        const crsReady = state.activeCrsId > CRS_COUNTER_BASE;

        if (keyReady && crsReady) {
          logInfo(
            `Gateway key bootstrap is ready (activeKeyId=${state.activeKeyId.toString()}, activeCrsId=${state.activeCrsId.toString()}).`,
          );
          return;
        }

        const missing: string[] = [];
        if (!keyReady) {
          missing.push("active key");
        }
        if (!crsReady) {
          missing.push("active CRS");
        }
        lastState =
          `missing ${missing.join(", ")} (activeKeyId=${state.activeKeyId.toString()}, ` +
          `activeCrsId=${state.activeCrsId.toString()})`;
      } catch (error) {
        lastState = errorMessage(error);
      }

      if (attempt < maxAttempts) {
        logWarn(
          `Gateway key bootstrap not ready (${lastState}). Retrying in ${retryDelaySeconds}s... (${attempt}/${maxAttempts})`,
        );
        sleep(retryDelaySeconds);
        continue;
      }
    }

    cliError(
      "E_GATEWAY_BOOTSTRAP_TIMEOUT",
      `Gateway key bootstrap did not become ready after ${maxAttempts} attempts (${lastState}). ` +
        `Check gateway-sc/kms-connector logs and retry: ${cliCommand("deploy --resume gateway-sc")}`,
    );
  }

  function isGatewayBuildStep(step: DeploymentStep): boolean {
    return step.component === "gateway-sc" || step.component === "gateway-mocked-payment";
  }

  function runGatewayScStep(step: DeploymentStep, compose: string, envFile: string, useBuild: boolean): void {
    if (useBuild) {
      logInfo(`Building and starting ${step.description} using local environment file...`);
    } else {
      logInfo(`Starting ${step.description} using local environment file...`);
    }
    logInfo(`Using environment file: ${envFile}`);

    const deployCheck = step.serviceChecks.find((check) => check.service === "gateway-sc-deploy");
    if (!deployCheck) {
      throw new Error("Gateway step is missing gateway-sc-deploy service check");
    }

    const runtimeChecks = step.serviceChecks.filter((check) => check.service !== "gateway-sc-deploy");
    const base = ["docker", "compose", "-p", PROJECT, "--env-file", envFile, "-f", compose, "up"];

    const deployCommand = [...base];
    if (useBuild) {
      deployCommand.push("--build");
    }
    deployCommand.push("--force-recreate", "-d", deployCheck.service);

    const deployResult = runComposeUp(deployCommand);
    if (deployResult.status !== 0) {
      if (useBuild) {
        throw new Error(`Failed to build and start ${step.description}`);
      }
      throw new Error(`Failed to start ${step.description}`);
    }

    waitForService(deployCheck.service, step.name, deployCheck.state);
    syncGatewayAddressesFromVolume(envFile);

    if (runtimeChecks.length === 0) {
      return;
    }

    const runtimeCommand = [...base];
    if (useBuild) {
      runtimeCommand.push("--build");
    }
    runtimeCommand.push("--no-deps", "--force-recreate", "-d", ...runtimeChecks.map((check) => check.service));

    const runtimeResult = runComposeUp(runtimeCommand);
    if (runtimeResult.status !== 0) {
      if (useBuild) {
        throw new Error(`Failed to build and start ${step.description}`);
      }
      throw new Error(`Failed to start ${step.description}`);
    }

    for (const check of runtimeChecks) {
      waitForService(check.service, step.name, check.state);
    }
  }

  function runHostScStep(step: DeploymentStep, compose: string, envFile: string, useBuild: boolean): void {
    if (useBuild) {
      logInfo(`Building and starting ${step.description} using local environment file...`);
    } else {
      logInfo(`Starting ${step.description} using local environment file...`);
    }
    logInfo(`Using environment file: ${envFile}`);

    const deployCheck = step.serviceChecks.find((check) => check.service === "host-sc-deploy");
    const addPausersCheck = step.serviceChecks.find((check) => check.service === "host-sc-add-pausers");
    if (!deployCheck) {
      throw new Error("Host contracts step is missing host-sc-deploy service check");
    }

    const base = ["docker", "compose", "-p", PROJECT, "--env-file", envFile, "-f", compose, "up"];

    const deployCommand = [...base];
    if (useBuild) {
      deployCommand.push("--build");
    }
    deployCommand.push("--force-recreate", "-d", deployCheck.service);
    const deployResult = runComposeUp(deployCommand);
    if (deployResult.status !== 0) {
      if (useBuild) {
        throw new Error(`Failed to build and start ${step.description}`);
      }
      throw new Error(`Failed to start ${step.description}`);
    }
    waitForService(deployCheck.service, step.name, deployCheck.state);

    if (!addPausersCheck) {
      return;
    }

    const addPausersCommand = [...base];
    if (useBuild) {
      addPausersCommand.push("--build");
    }
    addPausersCommand.push("--no-deps", "--force-recreate", "-d", addPausersCheck.service);

    const addPausersResult = runComposeUp(addPausersCommand);
    if (addPausersResult.status !== 0) {
      if (useBuild) {
        throw new Error(`Failed to build and start ${step.description}`);
      }
      throw new Error(`Failed to start ${step.description}`);
    }
    waitForService(addPausersCheck.service, step.name, addPausersCheck.state);
  }

  function runComposeStep(step: DeploymentStep, useBuild: boolean): void {
    if (!step.component) {
      throw new Error(`Step ${step.name} has no compose component`);
    }

    const envFile = localEnvFile(step.component);
    const composeBase = step.component === "coprocessor" ? resolveMainCoprocessorCompose() : composeFile(step.component);
    const compose = resolveComposeForBuild(composeBase, useBuild);
    if (useBuild) {
      logInfo(`Building and starting ${step.description} using local environment file...`);
    } else {
      logInfo(`Starting ${step.description} using local environment file...`);
    }
    logInfo(`Using environment file: ${envFile}`);

    if (step.name === "gateway-sc") {
      runGatewayScStep(step, compose, envFile, useBuild);
      return;
    }
    if (step.name === "host-sc") {
      runHostScStep(step, compose, envFile, useBuild);
      return;
    }

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

    const upResult = runComposeUp(command);

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
    const resolvedContainer = resolveProjectContainerName(containerName) ?? containerName;
    const inspect = runCommand(
      ["docker", "inspect", "-f", "{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}", resolvedContainer],
      { capture: true, check: true },
    );
    const minioIp = inspect.stdout.trim();

    if (!minioIp) {
      throw new Error(`Could not find IP address for ${containerName} container`);
    }

    const coprocessorEnvFiles = [localEnvFile("coprocessor"), ...findAdditionalCoprocessorIndices().map(additionalCoprocessorEnvFile)];
    if (!fs.existsSync(coprocessorEnvFiles[0])) {
      throw new Error(`Coprocessor local env file not found: ${coprocessorEnvFiles[0]}`);
    }
    let updatedCount = 0;
    for (const envFile of coprocessorEnvFiles) {
      if (!fs.existsSync(envFile)) {
        continue;
      }
      const original = fs.readFileSync(envFile, "utf8");
      fs.writeFileSync(`${envFile}.bak`, original, "utf8");
      upsertEnvValue(envFile, "AWS_ENDPOINT_URL", `http://${minioIp}:9000`);
      updatedCount += 1;
    }

    console.log(`Found ${containerName} container IP: ${minioIp}`);
    console.log(`Updated AWS_ENDPOINT_URL to http://${minioIp}:9000 for ${updatedCount} coprocessor env file(s)`);
  }

  return {
    waitForService,
    runComposeUp,
    detectExpectedPause,
    detectEnforcedPause,
    readContainerLogs,
    runParallelHostGatewayNodeStartup,
    prebuildGatewayImage,
    waitForGatewayBootstrapReady,
    isGatewayBuildStep,
    runComposeStep,
    getMinioIp,
  };
}
