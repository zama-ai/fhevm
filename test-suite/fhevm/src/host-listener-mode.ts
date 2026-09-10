import { supportsHostListenerConsumer } from "./compat/compat";
import type { State, ResolvedCoprocessorScenario, ResolvedScenario } from "./types";
import { hostChainRuntimes } from "./layout";

export const consumerOnlyHostListeners = (scenario: ResolvedScenario) =>
  scenario.kind === "coprocessor-consensus" && scenario.hostListenerMode === "consumer";

export const isLegacyHostListener = (suffix: string) =>
  suffix === "host-listener" || suffix === "host-listener-poller";

export const listenerCoreServices = (scenario: ResolvedScenario) =>
  consumerOnlyHostListeners(scenario)
    ? hostChainRuntimes(scenario.hostChains).map((chain) =>
      chain.isDefault ? "listener-publisher-for-anvil" : `listener-publisher-${chain.key}`)
    : ["listener-publisher-for-anvil"];

export const legacyOnlyHostListeners = (scenario: ResolvedScenario) =>
  scenario.kind === "coprocessor-consensus" && scenario.hostListenerMode === "legacy";

/** Capability alone must not start consumers in an explicit legacy fallback. */
export const hostConsumerEnabled = (state: {
  versions: State["versions"];
  scenario?: ResolvedScenario;
  coprocessor?: ResolvedCoprocessorScenario;
}) => {
  const scenario = state.scenario ?? state.coprocessor;
  return (!scenario || !legacyOnlyHostListeners(scenario)) && supportsHostListenerConsumer(state);
};
