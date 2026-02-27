import type { BootState } from "./state";
import type { BootStep } from "./steps";
import { bold, dim, green, red } from "../utils/output";
import {
  emitJsonDiscovery,
  emitJsonPipelineComplete,
  emitJsonPipelineFail,
  emitJsonPipelineStart,
  emitJsonStepFail,
  emitJsonStepSkipped,
  emitJsonStepStart,
  emitJsonStepSuccess,
} from "../ci/json-output";

interface HeaderOptions {
  resume?: boolean;
  from?: number;
}

export type OutputMode = "human" | "json";

export interface PipelineOutput {
  pipelineHeader(totalSteps: number, options?: HeaderOptions): void;
  stepStart(step: Pick<BootStep, "number" | "displayName">, totalSteps: number): void;
  parallelStepStart(steps: readonly Pick<BootStep, "number" | "displayName">[], totalSteps: number): void;
  stepSuccess(step: Pick<BootStep, "number" | "displayName">, durationMs: number): void;
  stepFail(step: Pick<BootStep, "number" | "displayName">, error: string, durationMs: number): void;
  stepSkipped(step: Pick<BootStep, "number" | "displayName">, reason: string): void;
  discoveryResult(label: string, value: string): void;
  pipelineFail(failedStep: number, error: string): void;
  pipelineSummary(state: BootState): void;
}

function stepLabel(step: Pick<BootStep, "number" | "displayName">, totalSteps: number): string {
  return `[${step.number}/${totalSteps}] ${step.displayName}`;
}

export function formatDuration(ms: number): string {
  if (ms < 60_000) {
    return `${(ms / 1_000).toFixed(1)}s`;
  }

  const totalSeconds = Math.floor(ms / 1_000);
  const minutes = Math.floor(totalSeconds / 60);
  const seconds = totalSeconds % 60;
  return `${minutes}m ${seconds}s`;
}

export function printPipelineHeader(totalSteps: number, options: HeaderOptions = {}): void {
  console.log(`\n${bold(`═══ fhEVM Boot Pipeline (${totalSteps} steps) ═══`)}`);

  if (options.resume) {
    console.log(dim("Resuming from last failed step..."));
  } else if (typeof options.from === "number") {
    console.log(dim(`Starting from step ${options.from}...`));
  }

  console.log("");
}

export function printStepStart(
  step: Pick<BootStep, "number" | "displayName">,
  totalSteps: number,
): void {
  console.log(`${stepLabel(step, totalSteps)}...`);
}

export function printParallelStepStart(
  steps: readonly Pick<BootStep, "number" | "displayName">[],
  totalSteps: number,
): void {
  const label = steps.map((step) => stepLabel(step, totalSteps)).join(" + ");
  console.log(`${label}...`);
}

export function printStepSuccess(
  step: Pick<BootStep, "displayName">,
  durationMs: number,
): void {
  console.log(`  ${green("✓")} ${step.displayName} (${formatDuration(durationMs)})`);
}

export function printStepFail(
  step: Pick<BootStep, "displayName">,
  error: string,
  durationMs: number,
): void {
  console.log(`  ${red("✗")} ${step.displayName} (${formatDuration(durationMs)})`);
  console.log(`  ${dim(error)}`);
}

export function printStepSkipped(
  step: Pick<BootStep, "displayName">,
  reason: string,
): void {
  console.log(`  ${dim(`- ${step.displayName} (skipped: ${reason})`)}`);
}

export function printDiscoveryResult(label: string, value: string): void {
  console.log(`  ${dim(`→ Discovered ${label}: ${value}`)}`);
}

export function printPipelineSummary(state: BootState): void {
  const completedAt = state.completedAt ? Date.parse(state.completedAt) : Date.now();
  const startedAt = Date.parse(state.startedAt);
  const durationMs = Number.isFinite(startedAt) ? Math.max(0, completedAt - startedAt) : 0;
  const statusLabel =
    state.status === "completed"
      ? "Pipeline Complete"
      : state.status === "failed"
        ? "Pipeline Failed"
        : "Pipeline Incomplete";

  console.log(`\n${bold(`═══ ${statusLabel} (${formatDuration(durationMs)}) ═══`)}`);
}

function pipelineDurationMs(state: BootState): number {
  const completedAt = state.completedAt ? Date.parse(state.completedAt) : Date.now();
  const startedAt = Date.parse(state.startedAt);
  return Number.isFinite(startedAt) ? Math.max(0, completedAt - startedAt) : 0;
}

function createHumanOutput(): PipelineOutput {
  return {
    pipelineHeader: printPipelineHeader,
    stepStart: printStepStart,
    parallelStepStart: printParallelStepStart,
    stepSuccess: printStepSuccess,
    stepFail: printStepFail,
    stepSkipped: printStepSkipped,
    discoveryResult: printDiscoveryResult,
    pipelineFail: () => {},
    pipelineSummary: printPipelineSummary,
  };
}

function createJsonOutput(): PipelineOutput {
  return {
    pipelineHeader(totalSteps) {
      emitJsonPipelineStart(totalSteps);
    },
    stepStart(step, totalSteps) {
      emitJsonStepStart(step.number, step.displayName, totalSteps);
    },
    parallelStepStart(steps, totalSteps) {
      for (const step of steps) {
        emitJsonStepStart(step.number, step.displayName, totalSteps);
      }
    },
    stepSuccess(step, durationMs) {
      emitJsonStepSuccess(step.number, step.displayName, durationMs);
    },
    stepFail(step, error, durationMs) {
      emitJsonStepFail(step.number, step.displayName, error, durationMs);
    },
    stepSkipped(step, reason) {
      emitJsonStepSkipped(step.number, step.displayName, reason);
    },
    discoveryResult(label, value) {
      emitJsonDiscovery(label, value);
    },
    pipelineFail(failedStep, error) {
      emitJsonPipelineFail(failedStep, error);
    },
    pipelineSummary(state) {
      if (state.status === "completed") {
        emitJsonPipelineComplete(pipelineDurationMs(state));
      } else if (state.status === "failed") {
        emitJsonPipelineFail(state.failedStep ?? 0, state.error ?? "pipeline failed");
      }
    },
  };
}

export function createPipelineOutput(mode: OutputMode): PipelineOutput {
  return mode === "json" ? createJsonOutput() : createHumanOutput();
}
