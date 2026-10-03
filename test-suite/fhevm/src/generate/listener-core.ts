/**
 * Per-operator listener-core topology.
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
 * honors it by giving every operator its own listener: its own publisher
 * container, its own listener database, and its own Redis logical database on
 * the shared Redis container. One Redis server with N keyspaces costs one
 * container instead of N and separates the streams just as completely.
 */

import { COPROCESSOR_DB_CONTAINER, DEFAULT_POSTGRES_PASSWORD, DEFAULT_POSTGRES_USER } from "../layout";
import type { ResolvedScenario, State } from "../types";

/** The broker URL the compose templates ship, which operator 0 keeps. */
export const LISTENER_BROKER_URL = "redis://listener-redis:6379";

/** Scenario args are keyed by coprocessor service suffix. */
const CONSUMER_SERVICE_KEY = "host-listener-consumer";

type ScenarioInstance = { index: number; args?: Record<string, string[]> };

/** Publisher container for one operator. Operator 0 keeps the template name. */
export const listenerPublisherService = (operator: number) =>
  operator === 0 ? "listener-publisher-for-anvil" : `listener${operator}-publisher-for-anvil`;

/** Listener database for one operator. Operator 0 keeps the template name. */
export const listenerDatabaseName = (operator: number) =>
  operator === 0 ? "listener" : `listener${operator}`;

/** Redis logical database for one operator, as a broker URL. */
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

/** Every listener-core container a stack of this size runs, Redis first. */
export const listenerCoreServices = (count: number, instances: ScenarioInstance[] = []) => [
  "listener-redis",
  ...listenerOperators(count, instances).map(listenerPublisherService),
];

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

/** Every listener-core container this state's stack runs, Redis first. */
export const listenerCoreServicesForState = (state: Pick<State, "scenario">) =>
  listenerCoreServices(state.scenario.topology.count, scenarioInstances(state.scenario));
