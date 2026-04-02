import { DEFAULT_CHAIN_ID, DEFAULT_HOST_RPC_PORT, defaultHostChainKey, hostChainRuntimes } from "../layout";
import type { State } from "../types";

const fallbackHostChains = () => [
  { key: defaultHostChainKey([]), chainId: DEFAULT_CHAIN_ID, rpcPort: DEFAULT_HOST_RPC_PORT },
];

/** Returns host-chain runtime descriptors for the current scenario, including the implicit single-chain default. */
export const hostChainsForState = (state: Pick<State, "scenario">) =>
  hostChainRuntimes(state.scenario.hostChains.length ? state.scenario.hostChains : fallbackHostChains());

/** Returns the first/default host chain for the current scenario. */
export const defaultHostChain = (state: Pick<State, "scenario">) => hostChainsForState(state)[0];

/** Returns all non-default host chains for the current scenario. */
export const extraHostChains = (state: Pick<State, "scenario">) =>
  hostChainsForState(state).filter((chain) => !chain.isDefault);
