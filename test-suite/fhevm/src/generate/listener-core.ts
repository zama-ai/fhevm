/**
 * Listener-core topology: one publisher per operator, per host chain.
 *
 * In production every operator party runs its own listener stack: its own
 * chain reader, its own Postgres, its own Redis. Its coprocessor consumes from
 * that stack and from nothing else.
 *
 * The coprocessor consumer's broker identity is a compiled-in constant
 * (`DEFAULT_CONSUMER_ID` in `host-listener`), so two coprocessors pointed at
 * one Redis keyspace are two members of one consumer group, and Redis
 * load-balances blocks between them instead of delivering every block to each.
 * That is right for Blue/Green, whose two fleets share one coprocessor
 * database and therefore want each block handled once. It is wrong for
 * operators, who have a database each and all need every block.
 *
 * The invariant is one consumer identity per coprocessor database. The harness
 * honours it by giving every operator its own listener: its own publisher
 * container, its own listener database, and its own Redis logical database on
 * the shared Redis container. One Redis server with N keyspaces costs one
 * container instead of N and separates the streams just as completely.
 *
 * Chains are the second axis. A consumer-only scenario reads each host chain
 * with a producer of its own, so the publishers are the cross product: operator
 * `i` running chain `c` is one container. The chain axis stops at the
 * container. The database and the Redis keyspace stay per operator and are
 * shared by that operator's chains, because every listener table is keyed by
 * `chain_id` (`blocks`, `filters` and `final_blocks` all lead their unique
 * indexes with it), so one database holds several chains without collision —
 * which is also how an operator runs in production.
 */

import { consumerOnlyHostListeners } from "../host-listener-mode";
import {
  COPROCESSOR_DB_CONTAINER,
  DEFAULT_POSTGRES_PASSWORD,
  DEFAULT_POSTGRES_USER,
  hostChainRuntimes,
} from "../layout";
import type { HostChainRuntime } from "../layout";
import type { HostChainScenario, ResolvedScenario, State } from "../types";

/** The broker URL the compose templates ship, which operator 0 keeps. */
export const LISTENER_BROKER_URL = "redis://listener-redis:6379";

/** Scenario args are keyed by coprocessor service suffix. */
const CONSUMER_SERVICE_KEY = "host-listener-consumer";

type ScenarioInstance = { index: number; args?: Record<string, string[]> };

/** The part of a host chain that names a publisher. */
type PublisherChain = Pick<HostChainRuntime, "key" | "isDefault">;

/**
 * Publisher container for one operator on one host chain.
 *
 * Operator 0 and the default chain each keep the template's own name, so a
 * single-operator single-chain stack renders exactly as it did before either
 * axis existed.
 */
export const listenerPublisherService = (operator: number, chain?: PublisherChain) =>
  `${operator === 0 ? "listener" : `listener${operator}`}-publisher-${
    !chain || chain.isDefault ? "for-anvil" : chain.key
  }`;

/** Listener database for one operator, shared by all of that operator's chains. */
export const listenerDatabaseName = (operator: number) =>
  operator === 0 ? "listener" : `listener${operator}`;

/** Redis logical database for one operator, shared by all of that operator's chains. */
export const listenerBrokerUrl = (operator: number) =>
  operator === 0 ? LISTENER_BROKER_URL : `${LISTENER_BROKER_URL}/${operator}`;

/** Mirrors the `database.db_url` in `config/listener/listener-publisher-for-anvil.yaml`. */
export const listenerDatabaseUrl = (operator: number) =>
  `postgres://${DEFAULT_POSTGRES_USER}:${DEFAULT_POSTGRES_PASSWORD}@${COPROCESSOR_DB_CONTAINER}:5432/${listenerDatabaseName(operator)}`;

/**
 * Whether a scenario routes this operator's consumer itself.
 *
 * `three-of-three-fork` parks operator 2 on an empty Redis database because
 * that operator follows the fork while the publisher follows the canonical
 * chain. A scenario that makes that choice keeps it, and gets no publisher of
 * its own — there would be nothing to read it.
 */
const routesItsOwnConsumer = (instance: ScenarioInstance) =>
  (instance.args?.[CONSUMER_SERVICE_KEY] ?? []).some(
    (argument) => argument === "--url" || argument.startsWith("--url="),
  );

/** Operators that get a listener publisher of their own, in index order. */
export const listenerOperators = (count: number, instances: ScenarioInstance[] = []) =>
  Array.from({ length: count }, (_, index) => index).filter(
    (index) => !instances.some((instance) => instance.index === index && routesItsOwnConsumer(instance)),
  );

/**
 * Host chains that get a publisher of their own.
 *
 * Only consumer-only scenarios read each chain with a producer of its own;
 * everything else runs the single template publisher. `undefined` stands for
 * "the template's own chain", which keeps a scenario that declares no host
 * chains rendering the one service it always had.
 */
export const listenerPublisherChains = (
  chains: HostChainScenario[] | undefined,
  consumerOnly: boolean,
): (HostChainRuntime | undefined)[] => {
  const runtimes = hostChainRuntimes(chains ?? []);
  const selected = consumerOnly ? runtimes : runtimes.filter((chain) => chain.isDefault);
  return selected.length ? selected : [undefined];
};

/** Every publisher container a stack runs: operators crossed with their chains. */
export const listenerPublisherServices = (
  operators: number[],
  chains: HostChainScenario[] | undefined,
  consumerOnly: boolean,
) =>
  operators.flatMap((operator) =>
    listenerPublisherChains(chains, consumerOnly).map((chain) => listenerPublisherService(operator, chain)),
  );

/**
 * The scenario instances that decide publisher placement. A Blue/Green
 * scenario has one `bcs` args block that applies to every operator.
 */
const scenarioInstances = (scenario: ResolvedScenario): ScenarioInstance[] =>
  scenario.kind === "blue-green"
    ? Array.from({ length: scenario.topology.count }, (_, index) => ({ index, args: scenario.bcs.args }))
    : scenario.instances;

/** Operators that get a listener publisher of their own, from live state. */
export const listenerOperatorsForState = (state: Pick<State, "scenario">) =>
  listenerOperators(state.scenario.topology.count, scenarioInstances(state.scenario));

/** Every publisher container this scenario runs, without the shared Redis. */
export const listenerPublishersForScenario = (scenario: ResolvedScenario) =>
  listenerPublisherServices(
    listenerOperators(scenario.topology.count, scenarioInstances(scenario)),
    scenario.hostChains,
    consumerOnlyHostListeners(scenario),
  );

/** Every listener-core container this scenario runs, Redis first. */
export const listenerCoreServices = (scenario: ResolvedScenario) => [
  "listener-redis",
  ...listenerPublishersForScenario(scenario),
];

/** Every publisher container this state's stack runs, without the shared Redis. */
export const listenerPublishersForState = (state: Pick<State, "scenario">) =>
  listenerPublishersForScenario(state.scenario);

/** Every listener-core container this state's stack runs, Redis first. */
export const listenerCoreServicesForState = (state: Pick<State, "scenario">) =>
  listenerCoreServices(state.scenario);
