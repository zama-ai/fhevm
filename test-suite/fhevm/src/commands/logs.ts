/**
 * commands/logs.ts — The `logs` command handler.
 *
 * Follows container logs for a specified (or first) container.
 */
import { Effect } from "effect";

import { PreflightError } from "../errors";
import { LOG_TARGETS, PROJECT } from "../layout";
import { CommandRunner } from "../services/CommandRunner";

export const logs = (service: string | undefined, options: { follow: boolean } = { follow: true }) =>
  Effect.gen(function* () {
    const cmd = yield* CommandRunner;
    const ps = yield* cmd
      .run(
        [
          "docker",
          "ps",
          "--filter",
          `label=com.docker.compose.project=${PROJECT}`,
          "--format",
          "{{.Names}}",
        ],
        { allowFailure: true },
      )
      .pipe(
        Effect.catchAll(() =>
          Effect.succeed({ stdout: "", stderr: "", code: 1 }),
        ),
      );
    const containers = ps.stdout
      .split("\n")
      .map((item) => item.trim())
      .filter(Boolean)
      .filter((item) => !service || item.includes(service));
    if (!containers.length) {
      return yield* Effect.fail(
        new PreflightError({ message: `No containers match ${service ?? "fhevm"}` }),
      );
    }
    const requested = service
      ? LOG_TARGETS[service] ?? service
      : undefined;
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
