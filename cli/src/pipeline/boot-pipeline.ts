import { rm } from "fs/promises";

import { buildConfig } from "../config/config-builder";
import {
  ensureDotFhevm,
  resolveProjectRoot,
  type DotFhevmPaths,
} from "../config/dotfhevm";
import { generateAllEnvFiles, generateEnvFile } from "../config/env-writer";
import type { FhevmConfig } from "../config/model";
import { generateRelayerConfigFile } from "../config/relayer-config";
import { getServiceByName, type ServiceDefinition } from "../config/service-map";
import { ExitCode, FhevmCliError } from "../errors";
import { buildCacheConfig, type CacheConfig } from "../ci/buildkit-cache";
import { captureDiagnosticLogs } from "../ci/diagnostics";
import { detectCI, type CIEnvironment } from "../ci/detect";
import { buildVersionEnvVars, resolveAllVersions } from "../config/versions";

import { getContainerLogs } from "../docker/containers";
import { toLogLines } from "../docker/logs";
import {
  startAndWaitForServiceBatches,
  startAndWaitForServices,
  stopServices,
} from "../docker/services";
import type { ServiceStartOptions } from "../docker/types";
import {
  buildTopology,
  generateAllCoprocessorEnvFiles,
  generateInstanceServices,
  isMultiCoprocessor,
  writeComposeFile,
} from "../multi-coproc";

import {
  discoverAndApplyMinioIp,
  discoverContractAddresses,
  discoverCrsKeyId,
  discoverFheKeyId,
  discoverKmsSigner,
  discoverMockedPaymentAddress,
} from "./discovery";
import {
  createPipelineOutput,
  type PipelineOutput,
} from "./output";
import {
  createInitialState,
  loadState,
  markPipelineCompleted,
  markPipelineFailed,
  markStepCompleted,
  markStepFailed,
  markStepRunning,
  saveState,
  type BootState,
} from "./state";
import {
  BOOT_STEPS,
  getParallelGroup,
  getServicesForStep,
  getStepsTeardownOrder,
  resolveStepRef,
  type BootStep,
} from "./steps";
import {
  checkKeyCache,
  snapshotKeys,
  restoreKeys,
  type KeyCacheState,
} from "../keys/cache";

export interface BootOptions {
  resume?: boolean;
  from?: string;
  local?: string[];
  build?: boolean;
  noCache?: boolean;
  json?: boolean;
  numCoprocessors?: number;
  threshold?: number;
}

export interface BootContext {
  config: FhevmConfig;
  paths: DotFhevmPaths;
  envFiles: Map<string, string>;
  versionEnvVars: Record<string, string>;
  state: BootState;
  options: BootOptions;
  ci: CIEnvironment;
  cacheConfig: CacheConfig;
  output: PipelineOutput;
  keyCache: KeyCacheState;
  keyCacheRestored: boolean;
}

function getStepState(state: BootState, stepNumber: number) {
  const found = state.steps.find((step) => step.number === stepNumber);
  if (!found) {
    throw new Error(`missing step state for step ${stepNumber}`);
  }
  return found;
}

function resetStepState(state: BootState, stepNumber: number): void {
  const step = getStepState(state, stepNumber);
  step.status = "pending";
  step.startedAt = undefined;
  step.completedAt = undefined;
  step.durationMs = undefined;
  step.error = undefined;
}

function recomputeLastCompletedStep(state: BootState): void {
  const completed = state.steps
    .filter((step) => step.status === "completed")
    .sort((a, b) => a.number - b.number)
    .at(-1);

  state.lastStep = completed?.number ?? 0;
  state.lastStepName = completed?.name ?? "";
}

function clearPipelineFailure(state: BootState): void {
  state.status = "running";
  state.completedAt = undefined;
  state.failedStep = undefined;
  state.failedStepName = undefined;
  state.error = undefined;
}

async function restoreRuntimeValues(
  config: FhevmConfig,
  state: BootState,
  paths: DotFhevmPaths,
  envFiles: Map<string, string>,
): Promise<void> {
  let shouldRegenerateCoprocessorEnv = false;

  if (state.runtime.minioIp) {
    config.runtime.minioIp = state.runtime.minioIp;
    shouldRegenerateCoprocessorEnv = true;
  }

  if (state.runtime.kmsSigner) {
    config.runtime.kmsSigner = state.runtime.kmsSigner;
    envFiles.set("gateway-sc", await generateEnvFile(config, paths.env, "gateway-sc"));
    envFiles.set("host-sc", await generateEnvFile(config, paths.env, "host-sc"));
  }

  if (state.runtime.fheKeyId) {
    config.runtime.fheKeyId = state.runtime.fheKeyId;
    shouldRegenerateCoprocessorEnv = true;
  }

  if (state.runtime.crsKeyId) {
    config.runtime.crsKeyId = state.runtime.crsKeyId;
  }

  if (state.runtime.contractAddresses) {
    Object.assign(config.contracts, state.runtime.contractAddresses);
    shouldRegenerateCoprocessorEnv = true;
    for (const envName of ["host-sc", "kms-connector", "gateway-mocked-payment", "gateway-sc", "test-suite"] as const) {
      envFiles.set(envName, await generateEnvFile(config, paths.env, envName));
    }
  }

  if (shouldRegenerateCoprocessorEnv) {
    await regenerateCoprocessorEnvFiles(config, paths, envFiles);
  }
}

function serviceOptions(context: BootContext): ServiceStartOptions {
  return {
    envFileByName: context.envFiles,
    envVars: {
      ...context.versionEnvVars,
      ...(context.options.build ? context.cacheConfig.envVars : {}),
    },
    build: context.options.build,
    noCache: context.options.noCache,
    cwd: resolveProjectRoot(),
  };
}

async function regenerateCoprocessorEnvFiles(
  config: FhevmConfig,
  paths: DotFhevmPaths,
  envFiles: Map<string, string>,
): Promise<void> {
  if (!isMultiCoprocessor(config)) {
    envFiles.set("coprocessor", await generateEnvFile(config, paths.env, "coprocessor"));
    return;
  }

  const topology = buildTopology(config, paths);
  const generated = await generateAllCoprocessorEnvFiles(config, topology);
  for (const [name, filePath] of generated) {
    envFiles.set(name, filePath);
  }
}

async function executeMultiCoprocessorStep(context: BootContext): Promise<void> {
  const topology = buildTopology(context.config, context.paths);

  await Promise.all(
    topology
      .filter((instance) => !instance.usesBaseCompose)
      .map((instance) => writeComposeFile(instance)),
  );

  const envFiles = await generateAllCoprocessorEnvFiles(context.config, topology);
  for (const [name, filePath] of envFiles) {
    context.envFiles.set(name, filePath);
  }

  await startAndWaitForServiceBatches(
    topology.map((instance) => generateInstanceServices(instance)),
    serviceOptions(context),
  );
}

function resolveServicesByName(step: BootStep, names: readonly string[]): ServiceDefinition[] {
  return names.map((name) => {
    const service = getServiceByName(name);
    if (!service) {
      throw new FhevmCliError({
        exitCode: ExitCode.CONFIG,
        step: `step-${step.number}`,
        message: `unknown service in step ${step.number}: ${name}`,
      });
    }
    return service;
  });
}

async function executeStep(context: BootContext, step: BootStep): Promise<void> {
  switch (step.name) {
    case "minio": {
      const services = getServicesForStep(step);
      await startAndWaitForServices(services, serviceOptions(context));

      const minioIp = await discoverAndApplyMinioIp(context.config);
      context.state.runtime.minioIp = minioIp;
      await regenerateCoprocessorEnvFiles(context.config, context.paths, context.envFiles);
      context.output.discoveryResult("MinIO IP", minioIp);
      return;
    }

    case "kms-core": {
      if (context.keyCache.isComplete && !context.keyCacheRestored) {
        await restoreKeys(context.paths, context.config);
        context.keyCacheRestored = true;
        context.output.discoveryResult("Key cache", "restored from snapshot");
      }

      const services = getServicesForStep(step);
      await startAndWaitForServices(services, serviceOptions(context));
      return;
    }

    case "kms-signer": {
      const signer = await discoverKmsSigner(context.config);
      context.state.runtime.kmsSigner = signer;
      context.output.discoveryResult("KMS signer", signer);

      for (const envFile of step.envFilesToRegenerate ?? []) {
        context.envFiles.set(envFile, await generateEnvFile(context.config, context.paths.env, envFile));
      }
      return;
    }

    case "kms-connector": {
      const services = getServicesForStep(step);
      await startAndWaitForServices(services, serviceOptions(context));

      // After kms-connector starts, it catches up on keygen events from the
      // gateway chain and forwards them to kms-core. KMS core then generates
      // FHE keys and stores them in MinIO. Poll until a ServerKey appears.
      const [fheKeyId, crsKeyId] = await Promise.all([
        discoverFheKeyId(context.config),
        discoverCrsKeyId(context.config),
      ]);
      context.state.runtime.fheKeyId = fheKeyId;
      context.state.runtime.crsKeyId = crsKeyId;
      context.output.discoveryResult("FHE Key ID", fheKeyId);
      context.output.discoveryResult("CRS Key ID", crsKeyId);

      // Regenerate coprocessor env with the discovered FHE_KEY_ID
      await regenerateCoprocessorEnvFiles(context.config, context.paths, context.envFiles);
      return;
    }

    case "coprocessor": {
      if (isMultiCoprocessor(context.config)) {
        await executeMultiCoprocessorStep(context);
        return;
      }
      const services = getServicesForStep(step);
      await startAndWaitForServices(services, serviceOptions(context));
      return;
    }

    case "gateway-mocked-payment": {
      // Deploy the mocked ZamaOFT contract and discover its address.
      // The set-relayer service is deferred until after host-contracts
      // when all contract addresses are available.
      const deployServices = resolveServicesByName(step, ["gateway-deploy-mocked-zama-oft"]);
      await startAndWaitForServices(deployServices, serviceOptions(context));

      const zamaOftAddress = await discoverMockedPaymentAddress(context.config);
      if (zamaOftAddress) {
        context.state.runtime.contractAddresses = {
          ...context.state.runtime.contractAddresses,
          zamaOft: zamaOftAddress,
        };
        context.output.discoveryResult("ZamaOFT address", zamaOftAddress);
      }

      // Regenerate gateway-sc env so it has ZAMA_OFT_ADDRESS for Solidity compilation
      context.envFiles.set(
        "gateway-sc",
        await generateEnvFile(context.config, context.paths.env, "gateway-sc"),
      );
      return;
    }

    case "gateway-contracts": {
      // Phase 1: only deploy gateway contracts (gateway-sc-deploy).
      // Phase 2 (add-network, add-pausers, keygen, crsgen) requires host
      // chain addresses and runs after host-contracts.
      const deployServices = resolveServicesByName(step, ["gateway-sc-deploy"]);
      await startAndWaitForServices(deployServices, serviceOptions(context));

      // Discover gateway contract addresses (PROTOCOL_PAYMENT_ADDRESS, etc.)
      const gatewayAddresses = await discoverContractAddresses(context.config);
      context.state.runtime.contractAddresses = {
        ...context.state.runtime.contractAddresses,
        ...gatewayAddresses,
      };
      context.output.discoveryResult(
        "Gateway addresses",
        `ProtocolPayment=${gatewayAddresses.protocolPayment ?? "?"}, GatewayConfig=${gatewayAddresses.gatewayConfig ?? "?"}`,
      );

      // Regenerate host-sc env with gateway addresses before host contracts deploy
      context.envFiles.set(
        "host-sc",
        await generateEnvFile(context.config, context.paths.env, "host-sc"),
      );
      return;
    }

    case "host-contracts": {
      // Phase 1: Deploy host contracts only.
      const deployServices = resolveServicesByName(step, ["host-sc-deploy"]);
      await startAndWaitForServices(deployServices, serviceOptions(context));

      // Discover all contract addresses (both gateway and host are now available)
      const addresses = await discoverContractAddresses(context.config);
      context.state.runtime.contractAddresses = {
        ...context.state.runtime.contractAddresses,
        ...addresses,
      };
      context.output.discoveryResult(
        "Host addresses",
        `ACL=${addresses.acl ?? "?"}, FhevmExecutor=${addresses.fhevmExecutor ?? "?"}`,
      );

      // Regenerate all downstream env files with complete contract addresses.
      // Include host-sc so PAUSER_SET_CONTRACT_ADDRESS is set for add-pausers.
      await regenerateCoprocessorEnvFiles(context.config, context.paths, context.envFiles);
      for (const envName of ["host-sc", "kms-connector", "gateway-sc", "gateway-mocked-payment", "test-suite"] as const) {
        context.envFiles.set(envName, await generateEnvFile(context.config, context.paths.env, envName));
      }

      // Phase 1b: Add pausers — needs PAUSER_SET_CONTRACT_ADDRESS from discovery.
      const addPausersServices = resolveServicesByName(step, ["host-sc-add-pausers"]);
      await startAndWaitForServices(addPausersServices, serviceOptions(context));

      // Phase 2 of gateway contracts + deferred set-relayer from mocked-payment.
      // These need host chain addresses (ACL, FhevmExecutor) which are now available.
      const phase2Names = context.keyCache.isComplete
        ? ["gateway-sc-add-network", "gateway-sc-add-pausers", "gateway-set-relayer-mocked-payment"]
        : [
            "gateway-sc-add-network",
            "gateway-sc-add-pausers",
            "gateway-sc-trigger-keygen",
            "gateway-sc-trigger-crsgen",
            "gateway-set-relayer-mocked-payment",
          ];

      if (context.keyCache.isComplete) {
        context.output.discoveryResult("Key cache", "skipping keygen and crsgen");
      }

      const phase2Services = resolveServicesByName(step, phase2Names);
      await startAndWaitForServices(phase2Services, serviceOptions(context));

      if (!context.keyCache.isComplete) {
        await snapshotKeys(context.paths, context.config);
        context.keyCache = await checkKeyCache(context.paths);
        context.output.discoveryResult("Key cache", "snapshot saved");
      }
      return;
    }

    case "relayer": {
      await generateRelayerConfigFile(context.config);
      const services = getServicesForStep(step);
      await startAndWaitForServices(services, serviceOptions(context));
      return;
    }

    default: {
      const services = getServicesForStep(step);
      if (services.length === 0) {
        return;
      }
      await startAndWaitForServices(services, serviceOptions(context));
    }
  }
}

function toError(error: unknown): Error {
  if (error instanceof Error) {
    return error;
  }
  return new Error(String(error));
}

async function toStepError(step: BootStep, error: unknown): Promise<FhevmCliError> {
  if (error instanceof FhevmCliError) {
    const service = error.service ?? (step.serviceNames.length === 1 ? step.serviceNames[0] : undefined);
    let logLines = error.logLines;

    if (!logLines?.length && service) {
      const logs = await getContainerLogs(service, { tail: 20 });
      logLines = toLogLines(logs);
    }

    return new FhevmCliError({
      exitCode: error.exitCode,
      step: `Step ${step.number}: ${step.displayName}`,
      service,
      message: error.message,
      logLines,
      logHint: error.logHint ?? (service ? `fhevm-cli logs ${service}` : undefined),
      cause: error.cause,
    });
  }

  return new FhevmCliError({
    exitCode: ExitCode.GENERAL,
    step: `Step ${step.number}: ${step.displayName}`,
    message: toError(error).message,
    cause: error,
  });
}

async function executeStepGroup(context: BootContext, steps: BootStep[]): Promise<void> {
  if (steps.length === 0) {
    return;
  }

  if (steps.length === 1) {
    context.output.stepStart(steps[0], BOOT_STEPS.length);
  } else {
    context.output.parallelStepStart(steps, BOOT_STEPS.length);
  }

  const startedAt = new Map<number, number>();

  for (const step of steps) {
    startedAt.set(step.number, Date.now());
    markStepRunning(context.state, step.number);
    await saveState(context.paths.stateFile, context.state);
  }

  const results = await Promise.allSettled(
    steps.map(async (step) => {
      await executeStep(context, step);
      return Date.now() - (startedAt.get(step.number) ?? Date.now());
    }),
  );

  let firstFailure:
    | {
        step: BootStep;
        error: unknown;
        durationMs: number;
      }
    | undefined;

  for (let index = 0; index < steps.length; index += 1) {
    const step = steps[index];
    const result = results[index];
    const durationMs = Date.now() - (startedAt.get(step.number) ?? Date.now());

    if (result?.status === "fulfilled") {
      markStepCompleted(context.state, step.number, result.value);
      await saveState(context.paths.stateFile, context.state);
      context.output.stepSuccess(step, result.value);
      continue;
    }

    const reason = result?.status === "rejected" ? result.reason : new Error("step execution failed");
    const message = toError(reason).message;
    markStepFailed(context.state, step.number, message);
    await saveState(context.paths.stateFile, context.state);
    context.output.stepFail(step, message, durationMs);

    if (!firstFailure) {
      firstFailure = { step, error: reason, durationMs };
    }
  }

  if (!firstFailure) {
    return;
  }

  markPipelineFailed(context.state, firstFailure.step.number, toError(firstFailure.error).message);
  await saveState(context.paths.stateFile, context.state);
  if (context.ci.isCI) {
    try {
      await captureDiagnosticLogs(context.paths);
    } catch {
      // Best-effort diagnostics in CI mode.
    }
  }
  throw await toStepError(firstFailure.step, firstFailure.error);
}

async function teardownFromStep(context: BootContext, fromStep: number): Promise<void> {
  for (const step of getStepsTeardownOrder(fromStep)) {
    if (step.name === "coprocessor" && isMultiCoprocessor(context.config)) {
      const topology = buildTopology(context.config, context.paths).reverse();
      for (const instance of topology) {
        const services = generateInstanceServices(instance);
        await stopServices(
          services.map((service) => service.name),
          {
            cwd: resolveProjectRoot(),
            definitions: services,
          },
        );
      }

      for (const instance of topology) {
        if (!instance.usesBaseCompose) {
          await rm(instance.composeFile, { force: true });
        }
      }
    } else if (step.serviceNames.length > 0) {
      await stopServices(step.serviceNames, { cwd: resolveProjectRoot() });
    }
    resetStepState(context.state, step.number);
  }

  recomputeLastCompletedStep(context.state);
  clearPipelineFailure(context.state);
}

function parseStartStepFromResume(state: BootState): number {
  if (state.status === "failed" && typeof state.failedStep === "number") {
    return state.failedStep;
  }

  return Math.min(BOOT_STEPS.length, Math.max(1, state.lastStep + 1));
}

function requireStateForResume(state: BootState | null): BootState {
  if (state) {
    return state;
  }

  throw new FhevmCliError({
    exitCode: ExitCode.CONFIG,
    step: "boot-resume",
    message: "cannot resume without .fhevm/state.json",
  });
}

export async function runBootPipeline(options: BootOptions): Promise<void> {
  const paths = await ensureDotFhevm();
  const ci = detectCI({ noCache: options.noCache });
  const cacheConfig = buildCacheConfig({ noCache: options.noCache });
  const keyCache = await checkKeyCache(paths);
  const output = createPipelineOutput(options.json ? "json" : "human");

  const config = buildConfig({
    local: options.local,
    numCoprocessors: options.numCoprocessors,
    threshold: options.threshold,
  });

  const envFiles = await generateAllEnvFiles(config, paths.env);
  const resolvedVersions = await resolveAllVersions(paths.versionCache);
  const versionEnvVars = buildVersionEnvVars(resolvedVersions);
  let state = createInitialState(BOOT_STEPS.map((step) => ({ number: step.number, name: step.name })));
  let startStep = 1;

  const loadedState = await loadState(paths.stateFile);

  if (options.resume) {
    state = requireStateForResume(loadedState);
    if (state.status === "completed") {
      output.pipelineHeader(BOOT_STEPS.length, { resume: true });
      output.pipelineSummary(state);
      return;
    }

    startStep = parseStartStepFromResume(state);
    await restoreRuntimeValues(config, state, paths, envFiles);

    for (const step of state.steps) {
      if (step.number >= startStep && step.status !== "completed") {
        resetStepState(state, step.number);
      }
    }

    clearPipelineFailure(state);
  }

  if (options.from) {
    const fromStep = resolveStepRef(options.from).number;
    if (fromStep > 1 && !loadedState) {
      throw new FhevmCliError({
        exitCode: ExitCode.CONFIG,
        step: "boot-from",
        message: "cannot use --from without existing .fhevm/state.json",
      });
    }

    if (loadedState) {
      state = loadedState;
      await restoreRuntimeValues(config, state, paths, envFiles);
    }

    startStep = fromStep;
    clearPipelineFailure(state);
  }

  const context: BootContext = {
    config,
    paths,
    envFiles,
    versionEnvVars,
    state,
    options,
    ci,
    cacheConfig,
    output,
    keyCache,
    keyCacheRestored: false,
  };

  if (options.from) {
    await teardownFromStep(context, startStep);
  }

  context.output.pipelineHeader(BOOT_STEPS.length, {
    resume: options.resume,
    from: options.from ? startStep : undefined,
  });

  await saveState(paths.stateFile, context.state);

  try {
    for (let index = 0; index < BOOT_STEPS.length; ) {
      const group = getParallelGroup(BOOT_STEPS, index);
      const toSkip = group.filter((step) => step.number < startStep);
      const toRun = group.filter((step) => step.number >= startStep);

      for (const step of toSkip) {
        const stepState = getStepState(context.state, step.number);
        if (stepState.status !== "completed") {
          stepState.status = "skipped";
        }
        context.output.stepSkipped(step, "preserved from previous run");
      }

      if (toRun.length > 0) {
        await executeStepGroup(context, toRun);
      }

      index += Math.max(1, group.length);
    }

    markPipelineCompleted(context.state);
    await saveState(paths.stateFile, context.state);
    context.output.pipelineSummary(context.state);
  } catch (error) {
    const failedStep = context.state.failedStep ?? 0;
    context.output.pipelineFail(failedStep, toError(error).message);
    throw error;
  }
}
