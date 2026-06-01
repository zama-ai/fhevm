import { supportsHostListenerConsumer } from "../compat/compat";
import { hasLocalCoprocessorInstance } from "../scenario/resolve";
import { topologyForState } from "../stack-spec/stack-spec";
import {
  GROUP_BUILD_COMPONENTS,
  GROUP_BUILD_SERVICES,
  coprocessorHostKey,
  defaultHostChainKey,
  hostChainSuffix,
} from "../layout";
import type { LocalOverride, ResolvedCoprocessorScenarioInstance, State, StepName } from "../types";
import { extraHostChains } from "./topology";

const UPGRADEABLE_GROUPS = ["coprocessor", "kms-connector", "kms-core", "kms", "listener-core", "relayer", "test-suite"] as const;
export type UpgradeGroup = (typeof UPGRADEABLE_GROUPS)[number];
const UPGRADE_VERSION_KEYS: Record<UpgradeGroup, string[]> = {
  "coprocessor": [
    "COPROCESSOR_DB_MIGRATION_VERSION",
    "COPROCESSOR_HOST_LISTENER_VERSION",
    "COPROCESSOR_GW_LISTENER_VERSION",
    "COPROCESSOR_TX_SENDER_VERSION",
    "COPROCESSOR_TFHE_WORKER_VERSION",
    "COPROCESSOR_ZKPROOF_WORKER_VERSION",
    "COPROCESSOR_SNS_WORKER_VERSION",
  ],
  "kms-connector": [
    "CONNECTOR_DB_MIGRATION_VERSION",
    "CONNECTOR_GW_LISTENER_VERSION",
    "CONNECTOR_KMS_WORKER_VERSION",
    "CONNECTOR_TX_SENDER_VERSION",
  ],
  "kms-core": ["CORE_VERSION"],
  "kms": [
    "CORE_VERSION",
    "CONNECTOR_DB_MIGRATION_VERSION",
    "CONNECTOR_GW_LISTENER_VERSION",
    "CONNECTOR_KMS_WORKER_VERSION",
    "CONNECTOR_TX_SENDER_VERSION",
  ],
  "listener-core": ["LISTENER_CORE_VERSION"],
  "relayer": ["RELAYER_VERSION", "RELAYER_MIGRATE_VERSION"],
  "test-suite": ["TEST_SUITE_VERSION"],
};
const LISTENER_CORE_SERVICES = ["listener-redis", "listener-publisher-for-anvil"];

type UpgradeComponentPlan = {
  component: string;
  services: string[];
  migrationServices: string[];
  runtimeServices: string[];
};

const supportsConsumerForState = (state: { versions?: State["versions"] }) =>
  !state.versions || supportsHostListenerConsumer({ versions: state.versions });

const coprocessorServices = (state: { versions?: State["versions"] }) =>
  GROUP_BUILD_SERVICES.coprocessor.filter(
    (service) => service !== "coprocessor-host-listener-consumer" || supportsConsumerForState(state),
  );

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
  state: Pick<State, "overrides" | "scenario"> & { versions?: State["versions"] },
  groupValue: string | undefined,
  options: { lockFile?: boolean } = {},
) => {
  if (!groupValue || !UPGRADEABLE_GROUPS.includes(groupValue as UpgradeGroup)) {
    throw new Error(`upgrade expects one of ${UPGRADEABLE_GROUPS.join(", ")}`);
  }
  const group = groupValue as UpgradeGroup;
  const lockFileMode = options.lockFile === true;
  if (group === "kms") {
    if (!lockFileMode) {
      throw new Error("upgrade kms requires --lock-file");
    }
    const core = splitServices("core", ["kms-core"]);
    const connector = splitServices("kms-connector", GROUP_BUILD_SERVICES["kms-connector"]);
    return upgradePlan(group, [core, connector], ["base", "kms-connector"]);
  }
  if (group === "kms-core") {
    if (!lockFileMode) {
      throw new Error("upgrade kms-core requires --lock-file");
    }
    return upgradePlan(group, [splitServices("core", ["kms-core"])], ["base"]);
  }
  if (group === "listener-core" && lockFileMode) {
    return upgradePlan(group, [splitServices("listener-core", LISTENER_CORE_SERVICES)], ["listener-core"]);
  }

  const groupOverrides = state.overrides.filter((item) => item.group === group);
  if (!lockFileMode && group === "coprocessor" && !hasLocalCoprocessorInstance(state) && !groupOverrides.length) {
    throw new Error("upgrade requires an active local coprocessor instance");
  }
  if (!lockFileMode && group !== "coprocessor" && !groupOverrides.length) {
    throw new Error(`upgrade requires an active local override for ${group}`);
  }
  const [component] = GROUP_BUILD_COMPONENTS[group];
  if (!component) {
    throw new Error(`No runtime component registered for ${group}`);
  }

  const selectedServices = selectedOverrideServices(groupOverrides);
  const plannedServices = group === "coprocessor"
    ? coprocessorPlannedServices(state, groupOverrides, lockFileMode)
    : selectedServices.length
      ? selectedServices
      : GROUP_BUILD_SERVICES[group];
  return upgradePlan(group, [splitServices(component, plannedServices)], [group === "coprocessor" ? "coprocessor" : group]);
};

const selectedOverrideServices = (groupOverrides: LocalOverride[]) =>
  [...new Set(groupOverrides.flatMap((item) => item.services ?? []))];

const coprocessorPlannedServices = (
  state: Pick<State, "overrides" | "scenario"> & { versions?: State["versions"] },
  groupOverrides: LocalOverride[],
  lockFileMode: boolean,
) => {
  const selectedServices = selectedOverrideServices(groupOverrides);
  const fullGroupServices = groupOverrides.length && !selectedServices.length ? coprocessorServices(state) : [];
  const overrideServices = selectedServices.length ? selectedServices : fullGroupServices;
  const releaseServices = lockFileMode ? GROUP_BUILD_SERVICES.coprocessor : overrideServices;
  const instances: ResolvedCoprocessorScenarioInstance[] = state.scenario.instances.length
    ? state.scenario.instances
    : Array.from({ length: topologyForState(state).count }, (_, index) => ({
        index,
        source: { mode: "inherit" },
        env: {},
        args: {},
      }));
  return instances.flatMap((instance) => {
    if (!lockFileMode && instance.source.mode === "registry") {
      return [];
    }
    const selected =
      instance.source.mode === "local"
        ? instance.localServices ?? coprocessorServices(state)
        : releaseServices;
    return selected.map((service) =>
      instance.index === 0 ? service : service.replace(/^coprocessor-/, `coprocessor${instance.index}-`),
    );
  });
};

const splitServices = (component: string, plannedServices: string[]): UpgradeComponentPlan => {
  const services = [...new Set(plannedServices)];
  return {
    component,
    services,
    migrationServices: services.filter((service) => service.endsWith("-db-migration")),
    runtimeServices: services.filter((service) => !service.endsWith("-db-migration")),
  };
};

const upgradePlan = (group: UpgradeGroup, components: UpgradeComponentPlan[], steps: StepName[]) => {
  const runtimeServices = components.flatMap((component) => component.runtimeServices);
  if (!runtimeServices.length) {
    throw new Error(`upgrade requires restartable runtime services for ${group}`);
  }
  return {
    component: components[0].component,
    components,
    group,
    migrationServices: components.flatMap((component) => component.migrationServices),
    runtimeServices,
    step: steps[0],
    steps,
    versionKeys: UPGRADE_VERSION_KEYS[group],
  } as const;
};
