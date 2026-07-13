import type { FlowKind } from "../relayer/types";
import type { RateShape, Scenario } from "./schema";

/**
 * Authoritative protocol ceilings the coprocessor can sustain. The tool is
 * used on light workloads first, so built-in defaults stay well under these;
 * `ceilingWarnings` flags any resolved scenario whose peak per-flow arrival
 * rate would exceed them. Warnings are advisory only — they never change exit
 * codes or block a run.
 */
export const PROTOCOL_LIMITS = { inputProofRps: 20, decryptRps: 10 } as const;

/** Decrypt flows share a single combined ceiling. */
const DECRYPT_FLOWS: readonly FlowKind[] = [
  "public-decrypt",
  "user-decrypt",
  "delegated-user-decrypt",
];

/**
 * Peak arrival rate implied by a shape, or undefined when the rate is an
 * output (closed model) or unbounded knob is unset (burst without maxRps).
 */
const peakShapeRps = (shape: RateShape): number | undefined => {
  switch (shape.kind) {
    case "constant":
      return shape.rps;
    case "segments":
      return Math.max(
        ...shape.segments.flatMap((segment) => [segment.fromRps, segment.toRps]),
      );
    case "burst":
      return shape.maxRps;
    case "closed":
      return undefined;
  }
};

/**
 * Advisory warnings when a resolved scenario's peak per-flow arrival rate
 * exceeds the protocol ceilings. Per-flow peak = (weight / Σweights) ×
 * peak shape rate. Input-proof is compared alone; the three decrypt flows are
 * summed and compared against the combined decrypt ceiling.
 */
export const ceilingWarnings = (scenario: Scenario): readonly string[] => {
  const peak = peakShapeRps(scenario.shape);
  if (peak === undefined) return [];

  const totalWeight = scenario.flows.reduce((sum, mix) => sum + mix.weight, 0);
  if (totalWeight === 0) return [];

  const rateFor = (flow: FlowKind): number => {
    const mix = scenario.flows.find((entry) => entry.flow === flow);
    return mix === undefined ? 0 : (mix.weight / totalWeight) * peak;
  };

  const warnings: string[] = [];
  const inputProofRate = rateFor("input-proof");
  if (inputProofRate > PROTOCOL_LIMITS.inputProofRps) {
    warnings.push(
      `Scenario "${scenario.name}": input-proof peak ~${inputProofRate.toFixed(1)} rps exceeds ` +
        `the protocol ceiling of ${PROTOCOL_LIMITS.inputProofRps.toString()} rps.`,
    );
  }

  const decryptRate = DECRYPT_FLOWS.reduce((sum, flow) => sum + rateFor(flow), 0);
  if (decryptRate > PROTOCOL_LIMITS.decryptRps) {
    warnings.push(
      `Scenario "${scenario.name}": combined decrypt peak ~${decryptRate.toFixed(1)} rps exceeds ` +
        `the protocol ceiling of ${PROTOCOL_LIMITS.decryptRps.toString()} rps.`,
    );
  }

  return warnings;
};
