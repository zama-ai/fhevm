/**
 * commands/pause.ts — The `pause` command handler.
 *
 * Pauses host or gateway contracts via a hardhat task.
 */
import { Effect } from "effect";

import { PreflightError } from "../errors";
import { runContractTask } from "./contract-task";

export const pause = (scope: string | undefined) =>
  Effect.gen(function* () {
    if (scope === "host") {
      yield* runContractTask(
        "host-sc",
        "host-sc-deploy",
        "npx hardhat compile && npx hardhat task:pauseACL",
      );
      return;
    }
    if (scope === "gateway") {
      yield* runContractTask(
        "gateway-sc",
        "gateway-sc-deploy",
        "npx hardhat compile && npx hardhat task:pauseAllGatewayContracts",
      );
      return;
    }
    return yield* Effect.fail(
      new PreflightError({ message: "pause expects `host` or `gateway`" }),
    );
  });
