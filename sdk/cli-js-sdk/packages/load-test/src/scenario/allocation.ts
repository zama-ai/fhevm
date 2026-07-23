import type { FlowKind } from "../relayer/types";
import { plannedRequestCount, type Scenario } from "./schema";

export type PlannedFlowAllocation = Readonly<{
  flow: FlowKind;
  handlesPerRequest: number;
  /** Undefined only for duration-bound closed scenarios. */
  requests?: number;
}>;

/**
 * Exact finite allocation produced by the scheduler's smooth weighted
 * round-robin. Keeping this pure helper next to scenario data prevents pool
 * planning and executor preflight from inventing different estimates.
 */
export const plannedFlowAllocations = (
  scenario: Scenario,
): readonly PlannedFlowAllocation[] => {
  const planned = plannedRequestCount(scenario.shape);
  if (planned === undefined) {
    return scenario.flows.map((mix) => ({
      flow: mix.flow,
      handlesPerRequest: mix.handlesPerRequest,
    }));
  }

  const totalWeight = scenario.flows.reduce((total, mix) => total + mix.weight, 0);
  const credits = scenario.flows.map(() => 0);
  const counts = scenario.flows.map(() => 0);
  for (let request = 0; request < planned; request += 1) {
    let best = 0;
    for (let index = 0; index < scenario.flows.length; index += 1) {
      const mix = scenario.flows[index];
      if (!mix) continue;
      credits[index] = (credits[index] ?? 0) + mix.weight;
      if ((credits[index] ?? 0) > (credits[best] ?? 0)) best = index;
    }
    credits[best] = (credits[best] ?? 0) - totalWeight;
    counts[best] = (counts[best] ?? 0) + 1;
  }

  return scenario.flows.map((mix, index) => ({
    flow: mix.flow,
    handlesPerRequest: mix.handlesPerRequest,
    requests: counts[index] ?? 0,
  }));
};
