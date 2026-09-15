/** The coverage verdict includes validity gates which run after case records. */
export const failedSelectedJobs = (
  legs: string[],
  needs: Record<string, { result?: string }>,
): string[] => {
  const jobs = new Set(["plan"]);
  for (const leg of legs) {
    jobs.add(["harness", "rust-regression", "gpu"].includes(leg) ? leg : "consensus");
  }
  return [...jobs].filter((job) => needs[job]?.result !== "success")
    .map((job) => `${job}: ${needs[job]?.result ?? "missing"}`);
};

/** Explicit dispatch contract; planning fails before provisioning an unrouted leg. */
export const CI_STACK_LEGS = ["byte-agreement", "degraded", "fork", "crash-retry", "failure-matrix", "failure-matrix-db"] as const;
export const CI_SPECIAL_LEGS = ["harness", "rust-regression", "gpu"] as const;
export function assertCiLeg(leg: string): void {
  if (![...CI_STACK_LEGS, ...CI_SPECIAL_LEGS].includes(leg as never)) throw new Error(`CI has no dispatch for inventory leg ${leg}`);
}
