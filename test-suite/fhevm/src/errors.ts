import { Data } from "effect";

export class ContainerStartError extends Data.TaggedError("ContainerStartError")<{
  component: string;
  stderr: string;
}> {}

export class ContainerCrashed extends Data.TaggedError("ContainerCrashed")<{
  container: string;
  exitCode: number;
  logs: string;
}> {}

export class ProbeTimeout extends Data.TaggedError("ProbeTimeout")<{
  container: string;
  elapsed: number;
}> {}

export class BootstrapTimeout extends Data.TaggedError("BootstrapTimeout")<{
  elapsed: number;
}> {}

export class IncompatibleVersions extends Data.TaggedError("IncompatibleVersions")<{
  issues: string[];
}> {}

export class RpcError extends Data.TaggedError("RpcError")<{
  url: string;
  message: string;
}> {}

export class GitHubApiError extends Data.TaggedError("GitHubApiError")<{
  message: string;
}> {}

export class BuildError extends Data.TaggedError("BuildError")<{
  component: string;
  stderr: string;
}> {}

export class MinioError extends Data.TaggedError("MinioError")<{
  message: string;
}> {}

export class CommandError extends Data.TaggedError("CommandError")<{
  argv: string[];
  code: number;
  stderr: string;
}> {}

export class ResumeError extends Data.TaggedError("ResumeError")<{
  message: string;
}> {}

export class PreflightError extends Data.TaggedError("PreflightError")<{
  message: string;
}> {}

export class SchemaGuardError extends Data.TaggedError("SchemaGuardError")<{
  group: string;
  message: string;
}> {}

/** Union of all CLI errors for top-level catch. */
export type CliError =
  | ContainerStartError
  | ContainerCrashed
  | ProbeTimeout
  | BootstrapTimeout
  | IncompatibleVersions
  | RpcError
  | GitHubApiError
  | BuildError
  | MinioError
  | CommandError
  | ResumeError
  | PreflightError
  | SchemaGuardError;

export const formatCliError = (error: unknown): string | undefined => {
  if (!error || typeof error !== "object") {
    return error === undefined ? undefined : String(error);
  }
  if ("message" in error && typeof error.message === "string" && error.message) {
    return error.message;
  }
  if ("_tag" in error && typeof error._tag === "string") {
    switch (error._tag) {
      case "BootstrapTimeout":
        return `Bootstrap timed out after ${String((error as BootstrapTimeout).elapsed)}s`;
      case "IncompatibleVersions":
        return (error as IncompatibleVersions).issues.join("\n");
      case "ContainerCrashed": {
        const crashed = error as ContainerCrashed;
        return `${crashed.container} exited with code ${crashed.exitCode}\n${crashed.logs}`.trim();
      }
      case "CommandError": {
        const command = error as CommandError;
        return `${command.argv.join(" ")} failed (${command.code})\n${command.stderr}`.trim();
      }
      case "ContainerStartError": {
        const start = error as ContainerStartError;
        return `${start.component} failed to start\n${start.stderr}`.trim();
      }
      case "BuildError": {
        const build = error as BuildError;
        return `${build.component} build failed\n${build.stderr}`.trim();
      }
      case "ProbeTimeout": {
        const timeout = error as ProbeTimeout;
        return `${timeout.container} was not ready after ${timeout.elapsed}ms`;
      }
    }
  }
  return typeof error === "string" ? error : undefined;
};
