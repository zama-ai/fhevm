import type { RolloutRunContext } from "../../src/commands/rollout-run";
import { HOST_CONTRACT_UPGRADES, contractUpgradeCommand } from "../../src/flow/bootstrap";

/** KMSGeneration is canonical-only; these contracts exist on every host chain. */
export async function upgradeMigrationHostContracts(
  ctx: Pick<RolloutRunContext, "runHostContractTaskOnChain">,
  chains: readonly { key: string }[],
) {
  if (!chains.length) throw new Error("migration requires at least one host chain");
  for (const [task, contract] of HOST_CONTRACT_UPGRADES) {
    for (const chain of chains) {
      await ctx.runHostContractTaskOnChain(chain.key, contractUpgradeCommand(task, contract));
    }
  }
}
