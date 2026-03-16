/**
 * commands/unpause.ts — The `unpause` command handler.
 *
 * Unpauses host or gateway contracts via a hardhat task.
 */
import { Effect } from "effect";

import { PreflightError } from "../errors";
import { runContractTask } from "./contract-task";

export const unpause = (scope: string | undefined) =>
  Effect.gen(function* () {
    if (scope === "host") {
      yield* runContractTask(
        "host-sc",
        "host-sc-deploy",
        "npx hardhat compile && npx hardhat task:unpauseACL",
      );
      return;
    }
    if (scope === "gateway") {
      yield* runContractTask(
        "gateway-sc",
        "gateway-sc-deploy",
        "npx hardhat compile && npx hardhat task:unpauseAllGatewayContracts",
      );
      return;
    }
    return yield* Effect.fail(
      new PreflightError({ message: "unpause expects `host` or `gateway`" }),
    );
  });
