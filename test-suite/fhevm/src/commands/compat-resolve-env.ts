import fs from "node:fs";

import { Effect } from "effect";

import { COMPAT_MATRIX, resolveWorkflowCompatEnv } from "../compat";
import { CommandRunner } from "../services/CommandRunner";

const writeCompatEnv = (values: Record<string, string>) =>
  Effect.sync(() => {
    const lines = Object.entries(values).map(([key, value]) => `${key}=${value}`);
    if (!lines.length) {
      return;
    }
    if (process.env.GITHUB_ENV) {
      fs.appendFileSync(process.env.GITHUB_ENV, `${lines.join("\n")}\n`);
      return;
    }
    console.log(lines.join("\n"));
  });

export const compatResolveEnv = (
  versions: string[],
  options: { forceModernRelayer: boolean },
) =>
  Effect.gen(function* () {
    const runner = yield* CommandRunner;
    const values = yield* resolveWorkflowCompatEnv({
      versions,
      forceModernRelayer: options.forceModernRelayer,
      stackEra: process.env.STACK_ERA,
      relayerVersion: process.env.RELAYER_VERSION,
      relayerMigrateVersion: process.env.RELAYER_MIGRATE_VERSION,
      isModernRef: (ref) =>
        Effect.gen(function* () {
          const resolved = yield* runner.run(
            ["git", "rev-parse", "-q", "--verify", `${ref}^{commit}`],
            { allowFailure: true },
          );
          if (resolved.code !== 0) {
            return false;
          }
          const ancestry = yield* runner.run(
            ["git", "merge-base", "--is-ancestor", COMPAT_MATRIX.anchors.SIMPLE_ACL_MIN_SHA, ref],
            { allowFailure: true },
          );
          return ancestry.code === 0;
        }),
    });
    yield* writeCompatEnv(values);
  });
