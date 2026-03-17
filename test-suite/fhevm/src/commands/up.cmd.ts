import { Command, Options } from "@effect/cli";
import { Effect, Option } from "effect";
import {
  shaOption,
  overrideOption,
  fromStepOption,
  lockFileOption,
  scenarioOption,
  resumeOption,
  dryRunOption,
  resetOption,
  allowSchemaMismatchOption,
  parseLocalOverride,
} from "../options";
import { PreflightError } from "../errors";
import { TARGETS, STEP_NAMES } from "../types";
import type { StepName, VersionTarget } from "../types";
import { up, upDryRun } from "./up";

const upOptions = {
  target: Options.text("target").pipe(Options.optional),
  sha: shaOption,
  override: overrideOption,
  fromStep: fromStepOption,
  lockFile: lockFileOption,
  scenario: scenarioOption,
  resume: resumeOption,
  dryRun: dryRunOption,
  reset: resetOption,
  allowSchemaMismatch: allowSchemaMismatchOption,
};

const upHandler = (parsed: {
  target: Option.Option<string>;
  sha: Option.Option<string>;
  override: Array<string>;
  fromStep: Option.Option<string>;
  lockFile: Option.Option<string>;
  scenario: Option.Option<string>;
  resume: boolean;
  dryRun: boolean;
  reset: boolean;
  allowSchemaMismatch: boolean;
}) =>
  Effect.gen(function* () {
    const target = Option.getOrUndefined(parsed.target);
    const sha = Option.getOrUndefined(parsed.sha);
    const fromStepRaw = Option.getOrUndefined(parsed.fromStep);
    const lockFile = Option.getOrUndefined(parsed.lockFile);
    const scenarioPath = Option.getOrUndefined(parsed.scenario);

    // Validate target (preserves exact error message for tests)
    if (target && !TARGETS.includes(target as VersionTarget)) {
      return yield* Effect.fail(
        new PreflightError({ message: `Unsupported target ${target}` }),
      );
    }
    const validTarget = (target ?? "latest-main") as VersionTarget;

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
    const hasCoprocessorOverride = overrides.some((item) => item.group === "coprocessor");
    if (scenarioPath && hasCoprocessorOverride) {
      return yield* Effect.fail(
        new PreflightError({
          message: "--scenario cannot be combined with --override coprocessor",
        }),
      );
    }
    const topology = { count: 1, threshold: 1 };

    if (parsed.dryRun) {
      yield* upDryRun({
        target: validTarget,
        requestedTarget: target as VersionTarget | undefined,
        sha,
        overrides,
        topology,
        scenarioPath,
        fromStep,
        lockFile,
        allowSchemaMismatch: parsed.allowSchemaMismatch,
        resume: parsed.resume,
        reset: parsed.reset,
      });
    } else {
      yield* up({
        target: validTarget,
        requestedTarget: target as VersionTarget | undefined,
        sha,
        overrides,
        topology,
        scenarioPath,
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
