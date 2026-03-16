/**
 * commands/contract-task.ts — Shared helper for running hardhat tasks in contract containers.
 */
import { Effect } from "effect";

import { ensureRuntimeArtifacts } from "../pipeline";
import { resolvedComposeEnv } from "../codegen";
import { PreflightError } from "../errors";
import { dockerArgs } from "../layout";
import { CommandRunner } from "../services/CommandRunner";
import { StateManager } from "../services/StateManager";

export const runContractTask = (
  component: "host-sc" | "gateway-sc",
  service: "host-sc-deploy" | "gateway-sc-deploy",
  command: string,
) =>
  Effect.gen(function* () {
    const stateManager = yield* StateManager;
    const cmd = yield* CommandRunner;
    const state = yield* stateManager.load;
    if (!state) {
      return yield* Effect.fail(
        new PreflightError({ message: "Stack is not running; run `fhevm-cli up` first" }),
      );
    }
    yield* ensureRuntimeArtifacts(state, "contract task");
    yield* cmd.runLive(
      [
        ...dockerArgs(component),
        "run",
        "--rm",
        "--no-deps",
        "--entrypoint",
        "sh",
        service,
        "-lc",
        command,
      ],
      {
        env: resolvedComposeEnv(state),
      },
    );
  });
