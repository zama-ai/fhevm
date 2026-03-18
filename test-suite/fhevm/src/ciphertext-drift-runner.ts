import { Effect, Fiber, Schedule } from "effect";

import { PreflightError } from "./errors";
import { CommandRunner } from "./services/CommandRunner";
import {
  DRIFT_CLEANUP_SQL,
  DRIFT_INSTALL_SQL,
  driftDatabaseName,
} from "./ciphertext-drift";

const DRIFT_WARNING =
  '"message":"Drift detected: observed multiple digest variants for handle"';
const DRIFT_HANDLE = /"handle":"0x([0-9a-f]+)"/i;

type DriftInjectorOptions = {
  instanceIndex: number;
  timeoutSeconds: number;
  pollIntervalSeconds: number;
  postgresContainer: string;
  postgresUser: string;
  postgresPassword: string;
};

type DriftLogOptions = {
  since: string;
  timeoutSeconds: number;
  pollIntervalSeconds: number;
};

type DriftWarningMatch = {
  container: string;
  handleHex?: string;
  exact: boolean;
};

export const findDriftWarning = (
  output: string,
  expectedHandleHex: string,
): Omit<DriftWarningMatch, "container"> | undefined => {
  let fallback: Omit<DriftWarningMatch, "container"> | undefined;
  for (const line of output.split(/\r?\n/)) {
    if (!line.includes(DRIFT_WARNING)) {
      continue;
    }
    const matchedHandle = line.match(DRIFT_HANDLE)?.[1];
    if (matchedHandle?.toLowerCase() === expectedHandleHex.toLowerCase()) {
      return { handleHex: matchedHandle, exact: true };
    }
    fallback ??= { handleHex: matchedHandle, exact: false };
  }
  return fallback;
};

const psql = (
  dbName: string,
  sqlArgs: string[],
  options: Pick<
    DriftInjectorOptions,
    "postgresContainer" | "postgresUser" | "postgresPassword"
  >,
  input?: string,
) =>
  Effect.gen(function* () {
    const cmd = yield* CommandRunner;
    const result = yield* cmd.run(
      [
        "docker",
        "exec",
        ...(input ? ["-i"] : []),
        "-e",
        `PGPASSWORD=${options.postgresPassword}`,
        options.postgresContainer,
        "psql",
        "-U",
        options.postgresUser,
        "-d",
        dbName,
        ...sqlArgs,
      ],
      { input },
    );
    return result.stdout.trim();
  }).pipe(
    Effect.catchTag(
      "CommandError",
      (error) =>
        Effect.fail(
          new PreflightError({
            message: error.stderr.trim() || "psql failed",
          }),
        ),
    ),
  );

const injectCiphertextDrift = (options: DriftInjectorOptions) => {
  const dbName = driftDatabaseName(options.instanceIndex);
  const cleanup = psql(dbName, [], options, DRIFT_CLEANUP_SQL).pipe(
    Effect.catchAll(() => Effect.void),
  );
  const poll = Effect.gen(function* () {
    const consumed = yield* psql(
      dbName,
      ["-t", "-A", "-c", "SELECT consumed::int FROM drift_injection_state WHERE id = TRUE;"],
      options,
    );
    if (consumed === "1") {
      const handleHex = yield* psql(
        dbName,
        [
          "-t",
          "-A",
          "-c",
          "SELECT encode(injected_handle, 'hex') FROM drift_injection_state WHERE id = TRUE;",
        ],
        options,
      );
      if (handleHex) {
        return handleHex;
      }
    }
    return yield* Effect.fail("not-ready" as const);
  }).pipe(
    Effect.retry({
      while: (error: "not-ready" | PreflightError): error is "not-ready" =>
        error === "not-ready",
      schedule: Schedule.spaced(`${options.pollIntervalSeconds} seconds`).pipe(
        Schedule.compose(Schedule.recurs(Math.max(0, Math.ceil(options.timeoutSeconds / options.pollIntervalSeconds)))),
      ),
    }),
    Effect.mapError((error) =>
      error === "not-ready"
        ? new PreflightError({
            message: `timed out waiting for drift injection trigger to fire in ${dbName}`,
          })
        : error,
    ),
  );
  return Effect.acquireUseRelease(
    psql(dbName, [], options, DRIFT_INSTALL_SQL),
    () => poll,
    () => cleanup,
  );
};

const coprocessorGwListeners = Effect.gen(function* () {
  const cmd = yield* CommandRunner;
  const result = yield* cmd.run(
    ["docker", "ps", "--format", "{{.Names}}"],
    { allowFailure: true },
  );
  if (result.code !== 0) {
    return yield* Effect.fail(
      new PreflightError({
        message: result.stderr.trim() || "docker ps failed",
      }),
    );
  }
  return result.stdout
    .split(/\r?\n/)
    .map((line) => line.trim())
    .filter((line) => /^coprocessor(\d+)?-gw-listener$/.test(line));
});

export const waitForDriftWarning = (
  handleHex: string,
  options: DriftLogOptions,
) =>
  Effect.gen(function* () {
    const cmd = yield* CommandRunner;
    const containers = yield* coprocessorGwListeners;
    let detected: DriftWarningMatch | undefined;
    for (const container of containers) {
      const logs = yield* cmd.run(
        ["docker", "logs", "--since", options.since, container],
        { allowFailure: true },
      );
      const output = logs.stdout + logs.stderr;
      const match = findDriftWarning(output, handleHex);
      if (match?.exact) {
        return { container, ...match };
      }
      detected ??= match ? { container, ...match } : undefined;
    }
    if (detected) {
      return detected;
    }
    return yield* Effect.fail("not-ready" as const);
  }).pipe(
    Effect.catchTag(
      "CommandError",
      (error) =>
        Effect.fail(
          new PreflightError({
            message: error.stderr.trim() || "docker logs failed",
          }),
        ),
    ),
    Effect.retry({
      while: (error: "not-ready" | PreflightError): error is "not-ready" =>
        error === "not-ready",
      schedule: Schedule.spaced(`${options.pollIntervalSeconds} seconds`).pipe(
        Schedule.compose(Schedule.recurs(Math.max(0, Math.ceil(options.timeoutSeconds / options.pollIntervalSeconds)))),
      ),
    }),
    Effect.mapError((error) =>
      error === "not-ready"
        ? new PreflightError({
            message: `drift warning was not observed after injecting handle ${handleHex}`,
          })
        : error,
    ),
  );

export const withDriftInjector = <A>(
  options: DriftInjectorOptions,
  use: (
    fiber: Fiber.RuntimeFiber<string, PreflightError>,
  ) => Effect.Effect<A, PreflightError | Error, CommandRunner>,
) =>
  Effect.acquireUseRelease(
    Effect.fork(injectCiphertextDrift(options)),
    use,
    (fiber) => Fiber.interrupt(fiber).pipe(Effect.orElseSucceed(() => undefined)),
  );
