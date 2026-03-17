/**
 * commands/logs.ts — The `logs` command handler.
 *
 * Follows container logs for a specified (or first) container.
 */
import { Effect } from "effect";

import { PreflightError } from "../errors";
import { LOG_TARGETS, PROJECT } from "../layout";
import { CommandRunner } from "../services/CommandRunner";

const dockerPsNames = (includeExited: boolean) => [
  "docker",
  "ps",
  ...(includeExited ? ["-a"] : []),
  "--filter",
  `label=com.docker.compose.project=${PROJECT}`,
  "--format",
  "{{.Names}}",
];

export const logs = (service: string | undefined, options: { follow: boolean } = { follow: true }) =>
  Effect.gen(function* () {
    const cmd = yield* CommandRunner;
    const requested = service
      ? LOG_TARGETS[service] ?? service
      : undefined;
    const list = (includeExited: boolean) =>
      cmd
        .run(dockerPsNames(includeExited), { allowFailure: true })
        .pipe(Effect.mapError((error) => new PreflightError({ message: error.stderr })));
    const running = yield* list(false);
    if (running.code !== 0) {
      return yield* Effect.fail(
        new PreflightError({
          message: running.stderr.trim() || "docker ps failed",
        }),
      );
    }
    const pickContainers = (stdout: string) =>
      stdout
        .split("\n")
        .map((item: string) => item.trim())
        .filter(Boolean)
        .filter((item: string) => !service || item.includes(service));
    let containers = pickContainers(running.stdout);
    const hasRequestedMatch = () =>
      requested
        ? containers.some((item) => item === requested || item.endsWith(`-${requested}`))
        : containers.length > 0;
    if (requested && !hasRequestedMatch()) {
      const all = yield* list(true);
      if (all.code !== 0) {
        return yield* Effect.fail(
          new PreflightError({
            message: all.stderr.trim() || "docker ps -a failed",
          }),
        );
      }
      containers = pickContainers(all.stdout);
    }
    if (!containers.length) {
      return yield* Effect.fail(
        new PreflightError({ message: `No containers match ${service ?? "fhevm"}` }),
      );
    }
    const exactMatch = requested
      ? containers.find((item) => item === requested) ??
        containers.find((item) => item.endsWith(`-${requested}`))
      : undefined;
    if (requested && !exactMatch && containers.length > 1) {
      return yield* Effect.fail(
        new PreflightError({
          message: `Multiple containers match ${service}: ${containers.join(", ")}`,
        }),
      );
    }
    const container = !requested
      ? containers[0]
      : exactMatch ?? containers[0];
    yield* cmd.runLive([
      "docker",
      "logs",
      ...(options.follow ? ["--follow"] : []),
      "--tail",
      "200",
      container,
    ]);
  });
