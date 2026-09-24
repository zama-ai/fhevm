import { withRolloutSupervisor } from "../../src/consensus/rollout-supervision";

/** Child stdin owns the outage lifetime, including abrupt parent termination. */
export async function withHeldRecipient<T>(stateDir: string, containers: string[], task: () => Promise<T>): Promise<T> {
  return withRolloutSupervisor(stateDir, "hold-rollout-services.sh", containers, {}, task);
}
