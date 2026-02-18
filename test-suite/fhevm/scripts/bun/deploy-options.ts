import { type DeploymentStep } from "./manifest";
import type { DeployOptions } from "./types";

type DeployOptionDeps = {
  usageError: (message: string) => never;
  logInfo: (message: string) => void;
  logWarn: (message: string) => void;
  parsePositiveInteger: (value: string, flagName: string) => number;
  maxLocalCoprocessors: number;
  stepNames: () => string[];
  stepIndex: (stepName: string) => number;
  deploymentSteps: DeploymentStep[];
  minioPrerequisitesMissing: () => string[];
  isContainerRunningExact: (containerName: string) => boolean;
};

export function createDeployOptionHandlers(deps: DeployOptionDeps) {
  const {
    usageError,
    logInfo,
    logWarn,
    parsePositiveInteger,
    maxLocalCoprocessors,
    stepNames,
    stepIndex,
    deploymentSteps,
    minioPrerequisitesMissing,
    isContainerRunningExact,
  } = deps;

  function parseDeployArgs(args: string[]): DeployOptions {
    const options: DeployOptions = {
      forceBuild: false,
      localBuild: false,
      autoTracing: true,
      telemetrySmoke: false,
      strictOtel: false,
      coprocessorCount: 1,
    };

    let expectResumeStep = false;
    let expectOnlyStep = false;
    let expectNetworkProfile = false;
    let expectCoprocessorCount = false;
    let expectCoprocessorThreshold = false;
    let sawBuildFlag = false;
    let sawLocalFlag = false;

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

      if (expectNetworkProfile) {
        if (arg !== "testnet" && arg !== "mainnet") {
          usageError(`Invalid deploy network profile: ${arg}\nAllowed values: testnet mainnet`);
        }
        options.networkProfile = arg;
        expectNetworkProfile = false;
        continue;
      }

      if (expectCoprocessorCount) {
        options.coprocessorCount = parsePositiveInteger(arg, "--coprocessors");
        expectCoprocessorCount = false;
        continue;
      }

      if (expectCoprocessorThreshold) {
        options.coprocessorThresholdOverride = parsePositiveInteger(arg, "--coprocessor-threshold");
        expectCoprocessorThreshold = false;
        continue;
      }

      if (arg === "--build") {
        sawBuildFlag = true;
        options.forceBuild = true;
        continue;
      }

      if (arg === "--local" || arg === "--dev") {
        sawLocalFlag = true;
        options.localBuild = true;
        continue;
      }

      if (arg === "--telemetry-smoke") {
        options.telemetrySmoke = true;
        logInfo("Telemetry smoke check enabled.");
        continue;
      }

      if (arg === "--strict-otel") {
        options.strictOtel = true;
        logInfo("Strict OTEL endpoint reachability enabled.");
        continue;
      }

      if (arg === "--no-tracing") {
        options.autoTracing = false;
        logInfo("Automatic tracing startup disabled.");
        continue;
      }

      if (arg === "--clean") {
        usageError("`--clean` is not a deploy flag. Run `bun run clean ...` (or `bun run down ...`) before deploy.");
      }

      if (arg === "--network") {
        expectNetworkProfile = true;
        continue;
      }

      if (arg === "--coprocessors") {
        expectCoprocessorCount = true;
        continue;
      }

      if (arg === "--coprocessor-threshold") {
        expectCoprocessorThreshold = true;
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

      usageError(`Unknown argument for deploy: ${arg}`);
    }

    const validSteps = stepNames().join(" ");

    if (expectResumeStep) {
      usageError(`--resume requires a step name\nValid steps are: ${validSteps}`);
    }

    if (expectOnlyStep) {
      usageError(`--only requires a step name\nValid steps are: ${validSteps}`);
    }

    if (expectNetworkProfile) {
      usageError("--network requires a profile name (testnet|mainnet)");
    }

    if (expectCoprocessorCount) {
      usageError("--coprocessors requires a value");
    }

    if (expectCoprocessorThreshold) {
      usageError("--coprocessor-threshold requires a value");
    }

    if (options.resumeStep && stepIndex(options.resumeStep) === -1) {
      usageError(`Invalid resume step: ${options.resumeStep}\nValid steps are: ${validSteps}`);
    }

    if (options.onlyStep && stepIndex(options.onlyStep) === -1) {
      usageError(`Invalid step: ${options.onlyStep}\nValid steps are: ${validSteps}`);
    }

    if (options.resumeStep && options.onlyStep) {
      usageError("Cannot use --resume and --only together");
    }

    if (options.coprocessorThresholdOverride && options.coprocessorThresholdOverride > options.coprocessorCount) {
      usageError(
        `Invalid coprocessor threshold: ${options.coprocessorThresholdOverride} (must be <= --coprocessors ${options.coprocessorCount})`,
      );
    }

    if (options.coprocessorCount > maxLocalCoprocessors) {
      usageError(`This local multicoprocessor mode currently supports up to ${maxLocalCoprocessors} coprocessors`);
    }

    if (sawBuildFlag && sawLocalFlag) {
      logWarn("`--build` is redundant with `--local`; continuing with local optimized build behavior.");
    }

    if (options.localBuild && !options.forceBuild) {
      options.forceBuild = true;
    }

    if (options.localBuild) {
      logInfo("Local optimization enabled.");
      logInfo("Local optimization implies build. Buildable services will be rebuilt.");
    } else if (options.forceBuild) {
      logInfo("Build mode enabled. Buildable services will be rebuilt.");
    }

    if (options.resumeStep) {
      logInfo(`Resume mode: starting from step '${options.resumeStep}'`);
    }

    if (options.onlyStep) {
      logInfo(`Only mode: deploying only step '${options.onlyStep}'`);
    }

    if (options.networkProfile) {
      logInfo(`Network profile mode: '${options.networkProfile}' versions will be fetched from the public dashboard.`);
    }

    logInfo(`Coprocessor topology: n=${options.coprocessorCount} threshold=${options.coprocessorThresholdOverride ?? "auto"}`);

    return options;
  }

  function resolveEffectiveResumeStep(options: DeployOptions): string | undefined {
    if (!options.resumeStep) {
      return undefined;
    }

    const minioStepIdx = stepIndex("minio");
    const requestedStepIdx = stepIndex(options.resumeStep);
    if (requestedStepIdx > minioStepIdx) {
      const missingMinioPrereqs = minioPrerequisitesMissing();
      if (missingMinioPrereqs.length > 0) {
        const adjustedStep = "minio";
        logWarn(
          `Requested resume step '${options.resumeStep}' requires MinIO prerequisites (${missingMinioPrereqs.join(", ")}), but they are missing. Forcing resume from '${adjustedStep}'.`,
        );
        return adjustedStep;
      }
    }

    const coprocessorStepIdx = stepIndex("coprocessor");
    const requestsMulticoprocessorTopology = options.coprocessorCount > 1 || options.coprocessorThresholdOverride !== undefined;

    if (requestedStepIdx >= coprocessorStepIdx && requestsMulticoprocessorTopology) {
      const adjustedStep = "minio";
      if (options.resumeStep !== adjustedStep) {
        logWarn(
          `Requested resume step '${options.resumeStep}' is too late for multicoprocessor topology changes. Forcing resume from '${adjustedStep}' for a full key-material reset (this intentionally redeploys the full stack to keep key digests coherent).`,
        );
      }
      return adjustedStep;
    }

    return options.resumeStep;
  }

  function ensureOnlyStepPrerequisites(stepName: string): void {
    const onlyIdx = stepIndex(stepName);
    if (onlyIdx <= 0) {
      return;
    }

    const missing: string[] = [];
    for (const priorStep of deploymentSteps.slice(0, onlyIdx)) {
      for (const check of priorStep.serviceChecks) {
        if (check.state !== "running") {
          continue;
        }
        if (!isContainerRunningExact(check.service)) {
          missing.push(check.service);
        }
      }
    }

    if (missing.length === 0) {
      return;
    }

    const uniqueMissing = Array.from(new Set(missing));
    const sample = uniqueMissing.slice(0, 8).join(", ");
    const suffix = uniqueMissing.length > 8 ? `, ... (+${uniqueMissing.length - 8} more)` : "";
    throw new Error(
      `--only ${stepName} requires prior-step runtime prerequisites that are not running: ${sample}${suffix}. ` +
        `Run a full deploy, or use --resume from an earlier step.`,
    );
  }

  return {
    parseDeployArgs,
    resolveEffectiveResumeStep,
    ensureOnlyStepPrerequisites,
  };
}
