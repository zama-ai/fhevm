import { supportsHostListenerConsumer } from "../compat/compat";
import { topologyForState } from "../stack-spec/stack-spec";
import {
  GROUP_SERVICE_SUFFIXES,
  KMS_CORE_CONTAINER,
  TEST_SUITE_CONTAINER,
} from "../layout";
import type { State, StepName } from "../types";
import { hostChainsForState } from "./topology";

const supportsConsumerForState = (state: { versions?: State["versions"] }) =>
  !state.versions || supportsHostListenerConsumer({ versions: state.versions });
const coprocessorRuntimeSuffixes = (state: { versions?: State["versions"] }) =>
  GROUP_SERVICE_SUFFIXES.coprocessor.filter(
    (service) =>
      service !== "db-migration" &&
      (service !== "host-listener-consumer" || supportsConsumerForState(state)),
  );
const coprocessorListenerSuffixes = (state: { versions?: State["versions"] }) =>
  coprocessorRuntimeSuffixes(state).filter((service) => /^host-listener(?:-poller)?$/.test(service));
/** Lists steady-state services expected for each resumable lifecycle step. */
export const resumeSteadyStateServices = (state: State) => {
  const chains = hostChainsForState(state);
  const [defaultChain, ...extraChains] = chains;
  const topology = topologyForState(state);
  const runtimeSuffixes = coprocessorRuntimeSuffixes(state);
  const listenerSuffixes = coprocessorListenerSuffixes(state);
  return {
    "base": ["fhevm-minio", "coprocessor-and-kms-db", KMS_CORE_CONTAINER, "gateway-node", ...chains.map((chain) => chain.node)],
    ...(supportsHostListenerConsumer(state) ? { "listener-core": ["listener-redis", "listener-publisher-for-anvil"] } : {}),
    "coprocessor": [
      ...Array.from({ length: topology.count }, (_, index) => {
        const prefix = index === 0 ? "coprocessor-" : `coprocessor${index}-`;
        return runtimeSuffixes.map((service) => `${prefix}${service}${defaultChain?.suffix ?? ""}`);
      }).flat(),
      ...extraChains.flatMap((chain) =>
        Array.from({ length: topology.count }, (_, index) => {
          const prefix = index === 0 ? "coprocessor-" : `coprocessor${index}-`;
          return listenerSuffixes.map((service) => `${prefix}${service}${chain.suffix}`);
        }).flat(),
      ),
    ],
    "kms-connector": [
      "kms-connector-gw-listener",
      "kms-connector-kms-worker",
      "kms-connector-tx-sender",
    ],
    "relayer": ["fhevm-relayer-db", "fhevm-relayer"],
    "test-suite": [TEST_SUITE_CONTAINER],
  } satisfies Partial<Record<StepName, string[]>>;
};

type RuntimeServiceStatus = { status: string; health?: string };
const normalizeRuntimeStatuses = (running: Iterable<string> | ReadonlyMap<string, RuntimeServiceStatus>) =>
  running instanceof Map
    ? running
    : new Map([...running].map((name) => [name, { status: "running" }] as const));

const isRuntimeServiceHealthy = (status: RuntimeServiceStatus | undefined) =>
  status?.status === "running" && (status.health === undefined || status.health === "healthy");

/** Chooses the earliest step that must rerun to repair a degraded resumed stack. */
export const resumeRepairStep = (
  state: State,
  running: Iterable<string> | ReadonlyMap<string, RuntimeServiceStatus>,
): StepName | undefined => {
  const live = normalizeRuntimeStatuses(running);
  const expected = resumeSteadyStateServices(state);
  const completed = new Set(state.completedSteps);
  return (Object.entries(expected) as Array<[StepName, string[]]>).find(
    ([step, services]) => completed.has(step) && services.some((service) => !isRuntimeServiceHealthy(live.get(service))),
  )?.[0];
};
