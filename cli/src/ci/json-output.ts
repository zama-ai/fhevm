export interface JsonEvent {
  timestamp: string;
  type: string;
  [key: string]: unknown;
}

type EventWriter = (line: string) => void;

const DEFAULT_WRITER: EventWriter = (line) => {
  process.stdout.write(line);
};

let writeEvent: EventWriter = DEFAULT_WRITER;

function writeJson(type: string, fields: Record<string, unknown>): void {
  const payload: JsonEvent = {
    timestamp: new Date().toISOString(),
    type,
    ...fields,
  };
  writeEvent(`${JSON.stringify(payload)}\n`);
}

export function emitJsonEvent(event: { type: string } & Record<string, unknown>): void {
  writeJson(event.type, event);
}

export function emitJsonPipelineStart(totalSteps: number): void {
  writeJson("pipeline-start", { totalSteps });
}

export function emitJsonPipelineComplete(durationMs: number): void {
  writeJson("pipeline-complete", { durationMs });
}

export function emitJsonPipelineFail(failedStep: number, error: string): void {
  writeJson("pipeline-fail", { failedStep, error });
}

export function emitJsonStepStart(step: number, name: string, totalSteps: number): void {
  writeJson("step-start", { step, name, totalSteps });
}

export function emitJsonStepSuccess(step: number, name: string, durationMs: number): void {
  writeJson("step-success", { step, name, durationMs });
}

export function emitJsonStepFail(step: number, name: string, error: string, durationMs: number): void {
  writeJson("step-fail", { step, name, error, durationMs });
}

export function emitJsonStepSkipped(step: number, name: string, reason: string): void {
  writeJson("step-skip", { step, name, reason });
}

export function emitJsonDiscovery(label: string, value: string): void {
  writeJson("discovery", { label, value });
}

export const __internal = {
  resetWriterForTests(): void {
    writeEvent = DEFAULT_WRITER;
  },
  setWriterForTests(next: EventWriter): void {
    writeEvent = next;
  },
};
