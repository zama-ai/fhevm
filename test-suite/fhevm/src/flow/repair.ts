import { hasLocalCoprocessorInstance } from "../scenario/resolve";
import { topologyForState } from "../stack-spec/stack-spec";
import {
  GROUP_BUILD_COMPONENTS,
  GROUP_BUILD_SERVICES,
  GROUP_SERVICE_SUFFIXES,
  KMS_CORE_CONTAINER,
  TEST_SUITE_CONTAINER,
  coprocessorHostKey,
  defaultHostChainKey,
  hostChainSuffix,
} from "../layout";
import type { LocalOverride, OverrideGroup, State, StepName } from "../types";
import { extraHostChains, hostChainsForState } from "./topology";

const UPGRADEABLE_GROUPS = ["coprocessor", "kms-connector", "test-suite"] as const;
type UpgradeGroup = (typeof UPGRADEABLE_GROUPS)[number];
const COPROCESSOR_RUNTIME_SUFFIXES = GROUP_SERVICE_SUFFIXES.coprocessor.filter((service) => service !== "db-migration");
const COPROCESSOR_LISTENER_SUFFIXES = COPROCESSOR_RUNTIME_SUFFIXES.filter((service) => /^host-listener(?:-poller)?$/.test(service));

/** Lists steady-state services expected for each resumable lifecycle step. */
export const resumeSteadyStateServices = (state: State) => {
  const chains = hostChainsForState(state);
  const [defaultChain, ...extraChains] = chains;
  const topology = topologyForState(state);
  return {
    "base": ["fhevm-minio", "coprocessor-and-kms-db", KMS_CORE_CONTAINER, "gateway-node", ...chains.map((chain) => chain.node)],
    "coprocessor": [
      ...Array.from({ length: topology.count }, (_, index) => {
        const prefix = index === 0 ? "coprocessor-" : `coprocessor${index}-`;
        return COPROCESSOR_RUNTIME_SUFFIXES.map((service) => `${prefix}${service}${defaultChain?.suffix ?? ""}`);
      }).flat(),
      ...extraChains.flatMap((chain) =>
        Array.from({ length: topology.count }, (_, index) => {
          const prefix = index === 0 ? "coprocessor-" : `coprocessor${index}-`;
          return COPROCESSOR_LISTENER_SUFFIXES.map((service) => `${prefix}${service}${chain.suffix}`);
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

/** Resolves extra-chain coprocessor listener restart targets for in-place upgrades. */
export const multiChainCoprocessorUpgradeTargets = (
  state: Pick<State, "scenario">,
  runtimeServices: string[],
) => {
  const restartableSuffixes = new Set(
    runtimeServices.flatMap((service) => {
      const match = service.match(/^coprocessor\d*-(host-listener(?:-poller)?)$/);
      return match ? [match[1]] : [];
    }),
  );
  return extraHostChains(state).map((chain) => {
    const suffix = hostChainSuffix(chain.key, defaultHostChainKey(state.scenario.hostChains));
    const topology = topologyForState(state);
    const services = [...restartableSuffixes].flatMap((serviceSuffix) =>
      Array.from({ length: topology.count }, (_, index) => {
        const prefix = index === 0 ? "coprocessor-" : `coprocessor${index}-`;
        return `${prefix}${serviceSuffix}${suffix}`;
      }),
    );
    return { compose: coprocessorHostKey(chain.key), chainKey: chain.key, services };
  });
};

/** Resolves which services and step an in-place upgrade should restart. */
export const resolveUpgradePlan = (
  state: Pick<State, "overrides" | "scenario">,
  groupValue: string | undefined,
) => {
  if (!groupValue || !UPGRADEABLE_GROUPS.includes(groupValue as UpgradeGroup)) {
    throw new Error(`upgrade expects one of ${UPGRADEABLE_GROUPS.join(", ")}`);
  }
  const group = groupValue as UpgradeGroup;
  const groupOverrides = state.overrides.filter((item) => item.group === group);
  if (group === "coprocessor" && !hasLocalCoprocessorInstance(state) && !groupOverrides.length) {
    throw new Error("upgrade requires an active local coprocessor instance");
  }
  if (group !== "coprocessor" && !groupOverrides.length) {
    throw new Error(`upgrade requires an active local override for ${group}`);
  }
  const [component] = GROUP_BUILD_COMPONENTS[group];
  if (!component) {
    throw new Error(`No runtime component registered for ${group}`);
  }
  const selectedServices = groupOverrides.flatMap((item) => item.services ?? []);
  const fullGroupServices = groupOverrides.length && !selectedServices.length ? GROUP_BUILD_SERVICES[group] : [];
  const overrideServices = selectedServices.length ? [...new Set(selectedServices)] : fullGroupServices;
  const scenario = state.scenario;
  const plannedServices =
    group === "coprocessor"
      ? scenario.instances.flatMap((instance) => {
          if (instance.source.mode === "registry") {
            return [];
          }
          const selected =
            instance.source.mode === "local"
              ? instance.localServices ?? GROUP_BUILD_SERVICES.coprocessor
              : overrideServices;
          return selected.map((service) =>
            instance.index === 0 ? service : service.replace(/^coprocessor-/, `coprocessor${instance.index}-`),
          );
        })
      : selectedServices.length
        ? [...new Set(selectedServices)]
        : GROUP_BUILD_SERVICES[group];
  const services = [...new Set(plannedServices)];
  const runtimeServices = services.filter((service) => !service.endsWith("-db-migration"));
  if (!runtimeServices.length) {
    throw new Error(`upgrade requires restartable runtime services for ${group}`);
  }
  return {
    component,
    group,
    runtimeServices,
    step: group === "coprocessor" ? "coprocessor" : group,
  } as const;
};
