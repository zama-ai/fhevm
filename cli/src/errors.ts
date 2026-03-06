export const ExitCode = {
  SUCCESS: 0,
  GENERAL: 1,
  CONFIG: 2,
  DOCKER: 3,
  TEST_FAILURE: 10,
} as const;

export type ExitCodeValue = (typeof ExitCode)[keyof typeof ExitCode] | number;

export interface CliError {
  exitCode: ExitCodeValue;
  message: string;
  step?: string;
  service?: string;
  logLines?: string[];
  logHint?: string;
  cause?: unknown;
}

export class FhevmCliError extends Error implements CliError {
  readonly exitCode: ExitCodeValue;
  readonly step?: string;
  readonly service?: string;
  readonly logLines?: string[];
  readonly logHint?: string;
  override readonly cause?: unknown;

  constructor(error: CliError) {
    super(error.message);
    this.name = "FhevmCliError";
    this.exitCode = error.exitCode;
    this.step = error.step;
    this.service = error.service;
    this.logLines = error.logLines;
    this.logHint = error.logHint;
    this.cause = error.cause;
  }
}

export function formatError(error: CliError): string {
  const lines = [`Error [exit ${error.exitCode}]: ${error.message}`];

  if (error.step) {
    lines.push(`Step: ${error.step}`);
  }
  if (error.service) {
    lines.push(`Service: ${error.service}`);
  }
  if (error.logLines?.length) {
    lines.push("Last 20 log lines:");
    lines.push(...error.logLines.slice(-20));
  }

  const hint = error.logHint ?? (error.service ? `fhevm-cli logs ${error.service}` : undefined);
  if (hint) {
    lines.push(`Hint: ${hint}`);
  }

  return lines.join("\n");
}

export function formatErrorJson(error: CliError): string {
  return JSON.stringify({
    error: true,
    exitCode: error.exitCode,
    message: error.message,
    step: error.step ?? null,
    service: error.service ?? null,
    logLines: error.logLines?.slice(-20) ?? null,
    hint: error.logHint ?? (error.service ? `fhevm-cli logs ${error.service}` : null),
  });
}

export function exitWithError(error: CliError, options: { json?: boolean } = {}): never {
  console.error(options.json ? formatErrorJson(error) : formatError(error));
  process.exit(error.exitCode);
}
