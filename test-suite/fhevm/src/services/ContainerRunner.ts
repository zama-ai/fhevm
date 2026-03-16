import { Context, Effect, Layer } from "effect";
import { CommandRunner } from "./CommandRunner";
import { ContainerStartError } from "../errors";
import { dockerArgs, composePath, versionsEnvPath } from "../layout";
import { readEnvFile, exists } from "../utils";

export class ContainerRunner extends Context.Tag("ContainerRunner")<
  ContainerRunner,
  {
    readonly composeUp: (
      component: string,
      services?: string[],
      options?: { noDeps?: boolean; env?: Record<string, string> },
    ) => Effect.Effect<void, ContainerStartError>;
    readonly composeDown: (component: string) => Effect.Effect<boolean, never>;
    readonly composeBuild: (
      component: string,
      services: string[],
      env?: Record<string, string>,
    ) => Effect.Effect<void, ContainerStartError>;
  }
>() {
  static Live = Layer.effect(
    ContainerRunner,
    Effect.gen(function* () {
      const cmd = yield* CommandRunner;

      const composeEnv = (extra?: Record<string, string>) =>
        Effect.tryPromise({
          try: async () => {
            const env = (await exists(versionsEnvPath))
              ? { ...(await readEnvFile(versionsEnvPath)), COMPOSE_IGNORE_ORPHANS: "true" }
              : { COMPOSE_IGNORE_ORPHANS: "true" };
            return { ...env, ...extra };
          },
          catch: () => new ContainerStartError({ component: "env", stderr: "Failed to read env" }),
        });

      return {
        composeUp: (component, services = [], options = {}) =>
          Effect.gen(function* () {
            const env = yield* composeEnv(options.env);
            yield* cmd.runLive(
              [
                ...dockerArgs(component),
                "up",
                "-d",
                ...(options.noDeps ? ["--no-deps"] : []),
                ...services,
              ],
              { env },
            ).pipe(
              Effect.mapError((e) => new ContainerStartError({ component, stderr: e.stderr })),
            );
          }),

        composeDown: (component) =>
          Effect.gen(function* () {
            const hasFile = yield* Effect.promise(() => exists(composePath(component)));
            if (!hasFile) return true;
            const env = yield* composeEnv().pipe(
              Effect.catchAll(() => Effect.succeed({ COMPOSE_IGNORE_ORPHANS: "true" } as Record<string, string>)),
            );
            const code = yield* cmd
              .runLive([...dockerArgs(component), "down", "-v"], { env, allowFailure: true })
              .pipe(Effect.catchAll(() => Effect.succeed(1)));
            if (code !== 0) {
              yield* Effect.log(`[warn] compose down failed for ${component} (${code})`);
              return false;
            }
            return true;
          }),

        composeBuild: (component, services, env) =>
          Effect.gen(function* () {
            const compEnv = yield* composeEnv(env);
            yield* cmd.runLive([...dockerArgs(component), "build", ...services], { env: compEnv }).pipe(
              Effect.mapError((e) => new ContainerStartError({ component, stderr: e.stderr })),
            );
          }),
      };
    }),
  );
}
