import { Command } from "@effect/cli";
import { Effect, Option } from "effect";
import {
  targetOption,
  shaOption,
  overrideOption,
  coprocessorsOption,
  thresholdOption,
  fromStepOption,
  lockFileOption,
  resumeOption,
  dryRunOption,
  resetOption,
  allowSchemaMismatchOption,
  instanceEnvOption,
  instanceArgOption,
  parseLocalOverride,
  parseInstanceEnv,
  parseInstanceArgs,
  mergeInstanceOverrides,
} from "../options";
import { PreflightError } from "../errors";
import { TARGETS, STEP_NAMES } from "../types";
import type { StepName, VersionTarget } from "../types";
import { up, upDryRun } from "./up";

const upOptions = {
  target: targetOption,
  sha: shaOption,
  override: overrideOption,
  coprocessors: coprocessorsOption,
  threshold: thresholdOption,
  fromStep: fromStepOption,
  lockFile: lockFileOption,
  resume: resumeOption,
  dryRun: dryRunOption,
  reset: resetOption,
  allowSchemaMismatch: allowSchemaMismatchOption,
  instanceEnv: instanceEnvOption,
  instanceArg: instanceArgOption,
};

const upHandler = (parsed: {
  target: string;
  sha: Option.Option<string>;
  override: Array<string>;
  coprocessors: string;
  threshold: Option.Option<string>;
  fromStep: Option.Option<string>;
  lockFile: Option.Option<string>;
  resume: boolean;
  dryRun: boolean;
  reset: boolean;
  allowSchemaMismatch: boolean;
  instanceEnv: Array<string>;
  instanceArg: Array<string>;
}) =>
  Effect.gen(function* () {
    const target = parsed.target;
    const sha = Option.getOrUndefined(parsed.sha);
    const fromStepRaw = Option.getOrUndefined(parsed.fromStep);
    const lockFile = Option.getOrUndefined(parsed.lockFile);
    const thresholdRaw = Option.getOrUndefined(parsed.threshold);

    // Validate target (preserves exact error message for tests)
    if (!TARGETS.includes(target as VersionTarget)) {
      return yield* Effect.fail(
        new PreflightError({ message: `Unsupported target ${target}` }),
      );
    }
    const validTarget = target as VersionTarget;

    // Validate from-step (preserves exact error message for tests)
    let fromStep: StepName | undefined;
    if (fromStepRaw) {
      if (!STEP_NAMES.includes(fromStepRaw as StepName)) {
        return yield* Effect.fail(
          new PreflightError({ message: `Unknown step ${fromStepRaw}` }),
        );
      }
      fromStep = fromStepRaw as StepName;
    }

    // Cross-field validations
    if (validTarget === "sha" && !sha) {
      return yield* Effect.fail(
        new PreflightError({ message: "--target sha requires --sha" }),
      );
    }
    if (validTarget !== "sha" && sha) {
      return yield* Effect.fail(
        new PreflightError({ message: "--sha requires --target sha" }),
      );
    }
    if (sha && lockFile) {
      return yield* Effect.fail(
        new PreflightError({ message: "--sha cannot be used with --lock-file" }),
      );
    }

    const count = Number(parsed.coprocessors);
    if (!Number.isInteger(count) || count < 1 || count > 5) {
      return yield* Effect.fail(
        new PreflightError({ message: "--coprocessors must be between 1 and 5" }),
      );
    }

    const thresholdVal = thresholdRaw ? Number(thresholdRaw) : undefined;
    if (
      thresholdVal !== undefined &&
      (!Number.isInteger(thresholdVal) || thresholdVal < 1 || thresholdVal > count)
    ) {
      return yield* Effect.fail(
        new PreflightError({
          message: "--threshold must be between 1 and --coprocessors",
        }),
      );
    }

    if (fromStep && !parsed.resume && !parsed.dryRun) {
      return yield* Effect.fail(
        new PreflightError({
          message: "--from-step requires --resume or --dry-run",
        }),
      );
    }

    // Transform complex options
    const overrides = yield* Effect.try({
      try: () => parsed.override.flatMap(parseLocalOverride),
      catch: (error) =>
        new PreflightError({ message: (error as Error).message }),
    });

    const topology = yield* Effect.try({
      try: () => ({
        count,
        threshold: thresholdVal ?? count,
        instances: mergeInstanceOverrides(
          parseInstanceEnv(parsed.instanceEnv as string[]),
          parseInstanceArgs(parsed.instanceArg as string[]),
        ),
      }),
      catch: (error) =>
        new PreflightError({ message: (error as Error).message }),
    });

    if (parsed.dryRun) {
      yield* upDryRun({
        target: validTarget,
        sha,
        overrides,
        topology,
        fromStep,
        lockFile,
        allowSchemaMismatch: parsed.allowSchemaMismatch,
        reset: parsed.reset,
      });
    } else {
      yield* up({
        target: validTarget,
        sha,
        overrides,
        topology,
        fromStep,
        lockFile,
        allowSchemaMismatch: parsed.allowSchemaMismatch,
        resume: parsed.resume,
        dryRun: false,
        reset: parsed.reset,
      });
    }
  });

export const upCommand = Command.make("up", upOptions, upHandler);

export const deployCommand = Command.make("deploy", upOptions, upHandler);
