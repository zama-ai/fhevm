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
