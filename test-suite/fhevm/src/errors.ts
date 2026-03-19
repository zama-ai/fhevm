export class CliError extends Error {
  readonly tag: string;

  constructor(tag: string, message: string) {
    super(message);
    this.name = tag;
    this.tag = tag;
  }
}

export class ContainerStartError extends CliError {
  constructor(
    readonly component: string,
    readonly stderr: string,
  ) {
    super("ContainerStartError", `${component} failed to start\n${stderr}`.trim());
  }
}

export class ContainerCrashed extends CliError {
  constructor(
    readonly container: string,
    readonly exitCode: number,
    readonly logs: string,
  ) {
    super("ContainerCrashed", `${container} exited with code ${exitCode}\n${logs}`.trim());
  }
}

export class ProbeTimeout extends CliError {
  constructor(
    readonly container: string,
    readonly elapsed: number,
  ) {
    super("ProbeTimeout", `${container} was not ready after ${elapsed}ms`);
  }
}

export class BootstrapTimeout extends CliError {
  constructor(readonly elapsed: number) {
    super("BootstrapTimeout", `Bootstrap timed out after ${elapsed}s`);
  }
}

export class IncompatibleVersions extends CliError {
  constructor(readonly issues: string[]) {
    super("IncompatibleVersions", issues.join("\n"));
  }
}

export class RpcError extends CliError {
  constructor(
    readonly url: string,
    message: string,
  ) {
    super("RpcError", message);
  }
}

export class GitHubApiError extends CliError {
  constructor(message: string) {
    super("GitHubApiError", message);
  }
}

export class BuildError extends CliError {
  constructor(
    readonly component: string,
    readonly stderr: string,
  ) {
    super("BuildError", `${component} build failed\n${stderr}${buildKitDnsHint(stderr)}`.trim());
  }
}

export class MinioError extends CliError {
  constructor(message: string) {
    super("MinioError", message);
  }
}

export class CommandError extends CliError {
  constructor(
    readonly argv: string[],
    readonly code: number,
    readonly stderr: string,
  ) {
    super("CommandError", `${argv.join(" ")} failed (${code})\n${stderr}`.trim());
  }
}

export class ResumeError extends CliError {
  constructor(message: string) {
    super("ResumeError", message);
  }
}

export class PreflightError extends CliError {
  constructor(message: string) {
    super("PreflightError", message);
  }
}

export class SchemaGuardError extends CliError {
  constructor(
    readonly group: string,
    message: string,
  ) {
    super("SchemaGuardError", message);
  }
}

const buildKitDnsHint = (stderr: string) =>
  /(no such host|temporary failure in name resolution|failed to resolve|dns|lookup .* on .*:53)/i.test(
    stderr,
  )
    ? "\nHint: Docker BuildKit could not resolve an external host. Check Docker DNS / proxy settings and retry."
    : "";

export const formatCliError = (error: unknown): string | undefined => {
  if (error instanceof Error) {
    return error.message;
  }
  if (typeof error === "string") {
    return error;
  }
  return error === undefined ? undefined : String(error);
};
