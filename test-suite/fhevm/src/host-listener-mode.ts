import type { ResolvedScenario } from "./types";
import { hostChainRuntimes } from "./layout";

/** Consumer-only scenarios run no legacy host listener or poller. */
export const consumerOnlyHostListeners = (scenario: ResolvedScenario) =>
  scenario.kind === "coprocessor-consensus" && scenario.hostListenerMode === "consumer";

export const isLegacyHostListener = (suffix: string) =>
  suffix === "host-listener" || suffix === "host-listener-poller";

/** Consumer-only scenarios run one listener-core producer per host chain. */
export const listenerCoreServices = (scenario: ResolvedScenario) =>
  consumerOnlyHostListeners(scenario)
    ? hostChainRuntimes(scenario.hostChains).map((chain) =>
      chain.isDefault ? "listener-publisher-for-anvil" : `listener-publisher-${chain.key}`)
    : ["listener-publisher-for-anvil"];
