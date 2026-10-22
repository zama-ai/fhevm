import type { ResolvedScenario } from "./types";

/** Consumer-only scenarios run no legacy host listener or poller. */
export const consumerOnlyHostListeners = (scenario: ResolvedScenario) =>
  scenario.kind === "coprocessor-consensus" && scenario.hostListenerMode === "consumer";

export const isLegacyHostListener = (suffix: string) =>
  suffix === "host-listener" || suffix === "host-listener-poller";
