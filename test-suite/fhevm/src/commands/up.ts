/**
 * commands/up.ts — The `up` command handler.
 *
 * Handles both full boot and resume scenarios. Also handles dry-run mode.
 */
import { Effect } from "effect";

import { ensureLockSnapshot, resolveBundle, previewBundle } from "../cache";
import {
  assertSchemaCompatibility,
  describeOverride,
  ensureRuntimeArtifacts,
  overrideWarnings,
  preflight,
  projectContainers,
  printBundle,
  printPlan,
  resetAfterStep,
  runStep,
  stateStepIndex,
} from "../pipeline";
import { ResumeError } from "../errors";
import {
  resolveScenarioForOptions,
  topologyForState,
} from "../runtime-plan";
import { StateManager } from "../services/StateManager";
import type {
  State,
  StepName,
  UpOptions,
} from "../types";
import { STEP_NAMES } from "../types";
import { down } from "./down";

const describeResumeState = (state: State) =>
  (() => {
    const topology = topologyForState(state);
    return [
      `profile=${state.target}`,
      `topology=${topology.count}/${topology.threshold}`,
      ...(state.scenario.origin !== "default"
        ? [
            `scenario=${state.scenario.origin}${
              state.scenario.sourcePath ? `:${state.scenario.sourcePath}` : ""
            }`,
          ]
        : []),
      ...(state.overrides.length
        ? [
            `overrides=${state.overrides.map(describeOverride).join(", ")}`,
          ]
        : []),
    ].join(" ");
  })();

const ensureResumeOptions = (
  state: State,
  options: Pick<
    UpOptions,
    | "requestedTarget"
    | "sha"
    | "lockFile"
    | "scenarioPath"
    | "overrides"
    | "allowSchemaMismatch"
    | "reset"
  >,
) => {
  const mismatches: string[] = [];
  if (options.requestedTarget && state.target !== options.requestedTarget) {
    mismatches.push(`target=${options.requestedTarget}`);
  }
  if (options.sha) {
    mismatches.push(`sha=${options.sha}`);
  }
  if (options.lockFile) {
    mismatches.push(`lock-file=${options.lockFile}`);
  }
  if (options.scenarioPath) {
    mismatches.push(`scenario=${options.scenarioPath}`);
  }
  if (options.overrides.length) {
    mismatches.push(
      `overrides=${options.overrides.map(describeOverride).join(", ")}`,
    );
  }
  if (options.allowSchemaMismatch) {
    mismatches.push("--allow-schema-mismatch");
  }
  if (options.reset) {
    mismatches.push("--reset");
  }
  if (mismatches.length) {
    return Effect.fail(
      new ResumeError({
        message:
          `--resume uses the persisted stack configuration; remove ${mismatches.join(", ")} or start a fresh stack. ` +
          `Persisted state: ${describeResumeState(state)}`,
      }),
    );
  }
  return Effect.void;
};

const startStep = (
  state: State,
  options: Pick<UpOptions, "resume" | "fromStep">,
): StepName => {
  if (options.fromStep) {
    return options.fromStep;
  }
  if (!options.resume || !state.completedSteps.length) {
    return STEP_NAMES[0];
  }
  const remaining = STEP_NAMES.find(
    (step) => !state.completedSteps.includes(step),
  );
  return remaining ?? STEP_NAMES[STEP_NAMES.length - 1];
};

const targetNeedsGitHub = (
  options: Pick<UpOptions, "target" | "lockFile">,
) => !options.lockFile && options.target !== "latest-supported";

// ---------------------------------------------------------------------------
// bootstrapState — resolve bundle + create initial state
// ---------------------------------------------------------------------------

const bootstrapState = (options: UpOptions) =>
  Effect.gen(function* () {
    const stateManager = yield* StateManager;
    yield* Effect.log(`[up] target=${options.target}`);
    const resolved = yield* resolveBundle(options, process.env);
    const scenario = yield* resolveScenarioForOptions(options);
    yield* assertSchemaCompatibility(
      resolved.bundle,
      options.overrides,
      scenario,
      options.allowSchemaMismatch,
    );
    yield* ensureLockSnapshot(resolved.lockPath, resolved.bundle);
    const state: State = {
      target: resolved.bundle.target,
      lockPath: resolved.lockPath,
      requiresGitHub: targetNeedsGitHub({
        target: resolved.bundle.target,
        lockFile: options.lockFile,
      }),
      versions: resolved.bundle,
      overrides: options.overrides,
      scenario,
      scenarioSourcePath: scenario?.sourcePath,
      completedSteps: [],
      updatedAt: new Date().toISOString(),
    };
    yield* stateManager.save(state);
    return state;
  });

// ---------------------------------------------------------------------------
// runUp — full boot
// ---------------------------------------------------------------------------

export const up = (options: UpOptions) =>
  Effect.gen(function* () {
    const started = Date.now();
    const stateManager = yield* StateManager;
    let state = options.resume
      ? yield* stateManager.load
      : undefined;

    if (options.resume && !state) {
      return yield* Effect.fail(
        new ResumeError({
          message: "No .fhevm/state.json to resume from",
        }),
      );
    }
    if (!options.resume && ((yield* stateManager.load) || (yield* projectContainers(true)).length)) {
      yield* Effect.log("[up] cleaning previous run");
      yield* down;
    }
    if (!state) {
      state = yield* bootstrapState(options);
    }
    if (options.resume) {
      state.requiresGitHub ??= state.target !== "latest-supported";
      state.scenarioSourcePath ??= state.scenario?.sourcePath;
      yield* ensureResumeOptions(state, options);
      yield* ensureRuntimeArtifacts(state, "resume");
      const running = yield* projectContainers();
      if (!running.length && !options.fromStep) {
        yield* Effect.log("[resume] stack is stopped; restarting from base");
        state.completedSteps = [];
        yield* stateManager.save(state);
      } else if (
        !options.fromStep &&
        STEP_NAMES.every((step) => state!.completedSteps.includes(step))
      ) {
        yield* Effect.log("[resume] nothing to do");
        return;
      }
    }
    for (const warning of overrideWarnings(state.overrides)) {
      yield* Effect.log(`[warn] ${warning}`);
    }
    if (options.resume && options.fromStep) {
      yield* resetAfterStep(options.fromStep);
      state.completedSteps = state.completedSteps.filter(
        (step) =>
          stateStepIndex(step) < stateStepIndex(options.fromStep!),
      );
      yield* stateManager.save(state);
    }
    const from = startStep(state, options);
    for (const step of STEP_NAMES.slice(stateStepIndex(from))) {
      if (
        options.resume &&
        state.completedSteps.includes(step) &&
        !options.fromStep
      ) {
        continue;
      }
      yield* runStep(state, step);
    }
    yield* Effect.log(
      `[done] stack ready in ${Math.round((Date.now() - started) / 1000)}s`,
    );
  });

// ---------------------------------------------------------------------------
// upDryRun — preflight + print plan, no side effects
// ---------------------------------------------------------------------------

export const upDryRun = (
  options: Omit<UpOptions, "dryRun">,
) =>
  Effect.gen(function* () {
    if (options.resume) {
      const stateManager = yield* StateManager;
      const state = yield* stateManager.load;
      if (!state) {
        return yield* Effect.fail(
          new ResumeError({
            message: "No .fhevm/state.json to resume from",
          }),
        );
      }
      state.requiresGitHub ??= state.target !== "latest-supported";
      state.scenarioSourcePath ??= state.scenario?.sourcePath;
      yield* ensureResumeOptions(state, options);
      yield* preflight(state, false, state.requiresGitHub);
      yield* printBundle(state.versions, { detailed: true });
      yield* printPlan(state, options.fromStep ?? startStep(state, options));
      yield* Effect.log(
        "[dry-run] resume preview uses persisted state only; no state or containers were changed",
      );
      return;
    }
    yield* Effect.log(`[up] target=${options.target}`);
    const bundle = yield* previewBundle(options, process.env);
    const scenario = yield* resolveScenarioForOptions(options);
    yield* assertSchemaCompatibility(
      bundle,
      options.overrides,
      scenario,
      options.allowSchemaMismatch,
    );
    const state = {
      target: options.target,
      versions: bundle,
      overrides: options.overrides,
      scenario,
    };
    yield* preflight(
      {
        target: state.target,
        lockPath: "",
        requiresGitHub: targetNeedsGitHub(options),
        versions: state.versions,
        overrides: state.overrides,
        scenario: state.scenario,
        scenarioSourcePath: state.scenario?.sourcePath,
        completedSteps: [],
        updatedAt: new Date().toISOString(),
      },
      true,
      targetNeedsGitHub(options),
    );
    yield* printBundle(state.versions, { detailed: true });
    yield* printPlan(state, options.fromStep);
    yield* Effect.log(
      "[dry-run] preflight passed; no state or containers were changed",
    );
  });
