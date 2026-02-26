import { mkdir, rename } from "fs/promises";
import { dirname } from "path";

export type StepStatus = "pending" | "running" | "completed" | "failed" | "skipped";

export interface StepState {
  number: number;
  name: string;
  status: StepStatus;
  startedAt?: string;
  completedAt?: string;
  durationMs?: number;
  error?: string;
}

export interface RuntimeState {
  minioIp?: string;
  kmsSigner?: string;
  fheKeyId?: string;
  crsKeyId?: string;
  contractAddresses?: Record<string, string | undefined>;
}

export interface BootState {
  version: 1;
  startedAt: string;
  completedAt?: string;
  lastStep: number;
  lastStepName: string;
  status: "running" | "completed" | "failed";
  failedStep?: number;
  failedStepName?: string;
  error?: string;
  runtime: RuntimeState;
  steps: StepState[];
}

interface StepIdentity {
  number: number;
  name: string;
}

const STATE_VERSION = 1;
const STEP_STATUSES = new Set<StepStatus>(["pending", "running", "completed", "failed", "skipped"]);

function isObject(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null;
}

function validateStateShape(state: Partial<BootState>): void {
  if (typeof state.startedAt !== "string") {
    throw new Error("invalid boot state: startedAt must be a string");
  }

  if (!Array.isArray(state.steps)) {
    throw new Error("invalid boot state: steps must be an array");
  }

  for (const step of state.steps) {
    if (!isObject(step)) {
      throw new Error("invalid boot state: each step must be an object");
    }

    if (typeof step.number !== "number" || typeof step.name !== "string") {
      throw new Error("invalid boot state: each step must have number and name");
    }

    if (typeof step.status !== "string" || !STEP_STATUSES.has(step.status as StepStatus)) {
      throw new Error("invalid boot state: each step must have a valid status");
    }
  }
}

function findStep(state: BootState, stepNumber: number): StepState {
  const step = state.steps.find((item) => item.number === stepNumber);
  if (!step) {
    throw new Error(`unknown step number: ${stepNumber}`);
  }
  return step;
}

export function createInitialState(stepDefinitions: readonly StepIdentity[]): BootState {
  const startedAt = new Date().toISOString();
  return {
    version: STATE_VERSION,
    startedAt,
    lastStep: 0,
    lastStepName: "",
    status: "running",
    runtime: {},
    steps: stepDefinitions.map((step) => ({
      number: step.number,
      name: step.name,
      status: "pending",
    })),
  };
}

export async function loadState(stateFilePath: string): Promise<BootState | null> {
  if (!(await Bun.file(stateFilePath).exists())) {
    return null;
  }

  const raw = await Bun.file(stateFilePath).text();
  const parsed = JSON.parse(raw) as Partial<BootState>;

  if (parsed.version !== STATE_VERSION) {
    throw new Error(
      `unsupported boot state version: ${String(parsed.version ?? "missing")} (expected ${STATE_VERSION})`,
    );
  }

  validateStateShape(parsed);

  return parsed as BootState;
}

export async function saveState(stateFilePath: string, state: BootState): Promise<void> {
  await mkdir(dirname(stateFilePath), { recursive: true });
  const tmpPath = `${stateFilePath}.tmp.${Date.now()}`;
  await Bun.write(tmpPath, `${JSON.stringify(state, null, 2)}\n`);
  await rename(tmpPath, stateFilePath);
}

export function markStepRunning(state: BootState, stepNumber: number): void {
  const step = findStep(state, stepNumber);
  step.status = "running";
  step.startedAt = new Date().toISOString();
  step.completedAt = undefined;
  step.durationMs = undefined;
  step.error = undefined;

  state.status = "running";
  state.failedStep = undefined;
  state.failedStepName = undefined;
  state.error = undefined;
}

export function markStepCompleted(state: BootState, stepNumber: number, durationMs: number): void {
  const step = findStep(state, stepNumber);
  step.status = "completed";
  step.completedAt = new Date().toISOString();
  step.durationMs = durationMs;
  step.error = undefined;
  if (!step.startedAt) {
    step.startedAt = step.completedAt;
  }

  state.lastStep = step.number;
  state.lastStepName = step.name;
}

export function markStepFailed(state: BootState, stepNumber: number, error: string): void {
  const step = findStep(state, stepNumber);
  step.status = "failed";
  step.completedAt = new Date().toISOString();
  step.error = error;
  if (!step.startedAt) {
    step.startedAt = step.completedAt;
  }
}

export function markPipelineCompleted(state: BootState): void {
  state.status = "completed";
  state.completedAt = new Date().toISOString();
  state.failedStep = undefined;
  state.failedStepName = undefined;
  state.error = undefined;
}

export function markPipelineFailed(state: BootState, stepNumber: number, error: string): void {
  const step = findStep(state, stepNumber);
  state.status = "failed";
  state.failedStep = step.number;
  state.failedStepName = step.name;
  state.error = error;
}
