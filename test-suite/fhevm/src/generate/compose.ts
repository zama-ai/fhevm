/**
 * Generates compose overrides for local builds, scenario instances, and compatibility-adjusted service commands.
 */
import fs from "node:fs/promises";
import path from "node:path";

import YAML from "yaml";

import { compatPolicyForState, type CompatPolicy } from "../compat/compat";
import { topologyForState, type StackSpec } from "../stack-spec/stack-spec";
import {
  COMPONENTS,
  COMPOSE_OUT_DIR,
  DEFAULT_CHAIN_ID,
  GROUP_BUILD_COMPONENTS,
  GROUP_BUILD_SERVICES,
  GROUP_SERVICE_SUFFIXES,
  TEMPLATE_COMPOSE_DIR,
  composePath,
  envPath,
  hostChainNames,
  hostChainRuntimes,
} from "../layout";
import type { HostChainScenario, ResolvedCoprocessorScenarioInstance, State } from "../types";
import { ensureDir, exists, mergeArgs, readEnvFile, remove, toServiceName } from "../utils/fs";

export type ComposeDoc = Record<string, unknown> & {
  services: Record<string, Record<string, unknown>>;
};

const LOCAL_BUILD_TAG = "fhevm-local";
/** Returns the local image tag used for a specific coprocessor instance. */
const localInstanceTag = (index: number) => `${LOCAL_BUILD_TAG}-i${index}`;

/** Builds the environment passed to docker compose from resolved versions. */
export const resolvedComposeEnv = (state: Pick<State, "versions" | "overrides" | "scenario">): Record<string, string> => ({
  ...state.versions.env,
  ...compatPolicyForState(state).composeEnv,
  COMPOSE_IGNORE_ORPHANS: "true",
});

/** Computes which services in a component should be locally overridden. */
const overriddenServicesForComponent = (
  state: Pick<State, "overrides"> | Pick<StackSpec, "overrides">,
  component: string,
) =>
  new Set(
    state.overrides.flatMap((override) => {
      if (override.group === "coprocessor" || !GROUP_BUILD_COMPONENTS[override.group].includes(component)) {
        return [];
      }
      return override.services?.length ? override.services : GROUP_BUILD_SERVICES[override.group];
    }),
  );

/** Rewrites the tag portion of an image reference. */
const rewriteImageTag = (image: unknown, tag: string) =>
  typeof image === "string" ? image.replace(/:([^:]+)$/, `:${tag}`) : image;

/** Retags an image reference with the default local build tag. */
const retagLocal = (image: unknown, tag = LOCAL_BUILD_TAG) => rewriteImageTag(image, tag);

/** Keeps build metadata only for services that should be built locally. */
const applyBuildPolicy = (service: Record<string, unknown>, isOverridden: boolean) => {
  if (isOverridden) {
    service.image = retagLocal(service.image);
  } else {
    delete service.build;
  }
};

/** Resolves relative compose paths against the template compose directory. */
const resolveComposePath = (value: string) =>
  value.startsWith(".") ? path.resolve(TEMPLATE_COMPOSE_DIR, value) : value;
const buildSpec = (context: string, dockerfile: string, extra: Record<string, unknown> = {}) => ({
  context: resolveComposePath(context),
  dockerfile: resolveComposePath(dockerfile),
  ...extra,
});
const COMPONENT_BUILD_SPECS: Record<string, Record<string, Record<string, unknown>>> = {
  coprocessor: {
    "coprocessor-db-migration": buildSpec("../../..", "coprocessor/fhevm-engine/Dockerfile.workspace", { target: "db-migration" }),
    "coprocessor-host-listener": buildSpec("../../..", "coprocessor/fhevm-engine/Dockerfile.workspace", { target: "host-listener" }),
    "coprocessor-host-listener-poller": buildSpec("../../..", "coprocessor/fhevm-engine/Dockerfile.workspace", { target: "host-listener" }),
    "coprocessor-gw-listener": buildSpec("../../..", "coprocessor/fhevm-engine/Dockerfile.workspace", { target: "gw-listener" }),
    "coprocessor-tfhe-worker": buildSpec("../../..", "coprocessor/fhevm-engine/Dockerfile.workspace", { target: "tfhe-worker" }),
    "coprocessor-zkproof-worker": buildSpec("../../..", "coprocessor/fhevm-engine/Dockerfile.workspace", { target: "zkproof-worker" }),
    "coprocessor-sns-worker": buildSpec("../../..", "coprocessor/fhevm-engine/Dockerfile.workspace", { target: "sns-worker" }),
    "coprocessor-transaction-sender": buildSpec("../../..", "coprocessor/fhevm-engine/Dockerfile.workspace", { target: "transaction-sender" }),
  },
  "kms-connector": {
    "kms-connector-db-migration": buildSpec("../../..", "kms-connector/connector-db/Dockerfile", { args: { RUST_IMAGE_VERSION: "1.91.0" } }),
    "kms-connector-gw-listener": buildSpec("../../..", "kms-connector/Dockerfile.workspace", { target: "gw-listener", args: { RUST_IMAGE_VERSION: "1.91.0" } }),
    "kms-connector-kms-worker": buildSpec("../../..", "kms-connector/Dockerfile.workspace", { target: "kms-worker", args: { RUST_IMAGE_VERSION: "1.91.0" } }),
    "kms-connector-tx-sender": buildSpec("../../..", "kms-connector/Dockerfile.workspace", { target: "tx-sender", args: { RUST_IMAGE_VERSION: "1.91.0" } }),
  },
  relayer: {
    "relayer-db-migration": buildSpec("../../..", "relayer/docker/relayer-migrate/Dockerfile"),
    relayer: buildSpec("../../..", "relayer/docker/relayer/Dockerfile"),
  },
  "gateway-mocked-payment": {
    "gateway-deploy-mocked-zama-oft": buildSpec("../../../gateway-contracts", "Dockerfile"),
    "gateway-set-relayer-mocked-payment": buildSpec("../../../gateway-contracts", "Dockerfile"),
  },
  "gateway-sc": {
    "gateway-sc-deploy": buildSpec("../../../gateway-contracts", "Dockerfile"),
    "gateway-sc-add-network": buildSpec("../../../gateway-contracts", "Dockerfile"),
    "gateway-sc-add-pausers": buildSpec("../../../gateway-contracts", "Dockerfile"),
  },
  "host-sc": {
    "host-sc-deploy": buildSpec("../../..", "host-contracts/Dockerfile"),
    "host-sc-add-pausers": buildSpec("../../..", "host-contracts/Dockerfile"),
    "host-sc-trigger-keygen": buildSpec("../../..", "host-contracts/Dockerfile"),
    "host-sc-trigger-crsgen": buildSpec("../../..", "host-contracts/Dockerfile"),
  },
  "test-suite": {
    "test-suite-e2e-debug": buildSpec("../../..", "test-suite/e2e/Dockerfile", {
      args: { RELAYER_SDK_VERSION: "${RELAYER_SDK_VERSION}" },
    }),
  },
};
const localBuildSpecFor = (component: string, service: string) => COMPONENT_BUILD_SPECS[component]?.[service];

/** Rewrites bind-mount volume paths to absolute template-rooted paths. */
const rewriteVolume = (value: unknown) => {
  if (typeof value !== "string") {
    return value;
  }
  const parts = value.split(":");
  if (!parts[0] || (!parts[0].startsWith(".") && !parts[0].startsWith("/"))) {
    return value;
  }
  parts[0] = resolveComposePath(parts[0]);
  return parts.join(":");
};

/** Rewrites filesystem paths inside a compose document for generated use. */
const rewriteComposePaths = (doc: ComposeDoc) => {
  for (const service of Object.values(doc.services)) {
    if (Array.isArray(service.volumes)) {
      service.volumes = service.volumes.map(rewriteVolume);
    }
    if (service.build && typeof service.build === "object") {
      const build = service.build as Record<string, unknown>;
      if (typeof build.context === "string") {
        build.context = resolveComposePath(build.context);
      }
      if (typeof build.dockerfile === "string") {
        build.dockerfile = resolveComposePath(build.dockerfile);
      }
    }
  }
  return doc;
};

/** Interpolates `${VAR}` placeholders inside a string using env vars. */
const interpolateString = (value: string, vars: Record<string, string>) =>
  value.replace(/(?<!\$)\$\{([A-Z0-9_]+)\}/g, (match, key) => (key in vars ? vars[key] : match));

/** Normalizes compose environment syntax into a flat map before local overrides are merged. */
const normalizeEnvironment = (value: unknown) => {
  if (Array.isArray(value)) {
    return Object.fromEntries(
      value.map((item) => {
        const entry = String(item);
        const index = entry.indexOf("=");
        return index < 0 ? [entry, ""] : [entry.slice(0, index), entry.slice(index + 1)];
      }),
    ) as Record<string, string>;
  }
  if (value && typeof value === "object") {
    return Object.fromEntries(
      Object.entries(value).map(([key, item]) => [key, String(item ?? "")]),
    ) as Record<string, string>;
  }
  return {} as Record<string, string>;
};

/** Recursively interpolates compose values using a flat env map. */
const interpolateComposeValue = (value: unknown, vars: Record<string, string>): unknown => {
  if (typeof value === "string") {
    return interpolateString(value, vars);
  }
  if (Array.isArray(value)) {
    return value.map((item) => interpolateComposeValue(item, vars));
  }
  if (!value || typeof value !== "object") {
    return value;
  }
  return Object.fromEntries(
    Object.entries(value).map(([key, item]) => [key, interpolateComposeValue(item, vars)]),
  );
};

/** Rewrites copied coprocessor dependencies to point at instance-specific service names. */
const rewriteCoprocessorDependsOn = (
  dependsOn: Record<string, unknown>,
  prefix: string,
  clonedServices: ReadonlySet<string>,
) =>
  Object.fromEntries(
    Object.entries(dependsOn).map(([dep, value]) => [
      clonedServices.has(dep) ? `${prefix}${dep.replace(/^coprocessor-/, "")}` : dep,
      value,
    ]),
  );

/** Applies env, compat, and instance-specific command adjustments to a service. */
const applyInstanceAdjustments = (
  baseServiceName: string,
  service: Record<string, unknown>,
  envFileValue: string,
  envVars: Record<string, string>,
  override: Pick<ResolvedCoprocessorScenarioInstance, "env" | "args"> = { env: {}, args: {} },
  compatArgs: CompatPolicy["coprocessorArgs"] = {},
  compatDropFlags: CompatPolicy["coprocessorDropFlags"] = {},
) => {
  const next = interpolateComposeValue(structuredClone(service), envVars) as Record<string, unknown>;
  const serviceKey = baseServiceName.replace(/^coprocessor-/, "");
  const command = Array.isArray(next.command) ? next.command.map((item) => String(item)) : undefined;
  if (command?.some((item) => item.startsWith("--key-cache-size"))) {
    next.command = command.map((item) => item.replace("--key-cache-size", "--tenant-key-cache-size"));
  }
  next.env_file = [envFileValue];
  if (Object.keys(override.env).length) {
    next.environment = { ...normalizeEnvironment(next.environment), ...override.env };
  }
  if (next.command) {
    const current = Array.isArray(next.command) ? next.command : [];
    const key = serviceKey as keyof CompatPolicy["coprocessorArgs"];
    const dropFlags = compatDropFlags[key] ?? [];
    const filtered: string[] = [];
    for (let index = 0; index < current.length; index += 1) {
      const value = String(current[index]);
      if (dropFlags.some((flag) => value.startsWith(`${flag}=`))) {
        continue;
      }
      if (dropFlags.includes(value)) {
        const nextValue = current[index + 1];
        if (typeof nextValue === "string" && !nextValue.startsWith("--")) {
          index += 1;
        }
        continue;
      }
      filtered.push(value);
    }
    const extras = (compatArgs[key] ?? []).flatMap(([flag, source]) =>
      "value" in source ? [flag, source.value] : envVars[source.env] ? [flag, envVars[source.env]] : [],
    );
    next.command = extras.length ? mergeArgs(filtered, extras) : filtered;
  }
  if (next.command) {
    const current = Array.isArray(next.command) ? next.command : [];
    next.command = mergeArgs(current, [...(override.args["*"] ?? []), ...(override.args[serviceKey] ?? [])]);
  }
  return next;
};

/** Lists runtime service names for the requested component and topology. */
export const serviceNameList = (state: Pick<State, "scenario">, component: string) => {
  if (component !== "coprocessor") {
    return [];
  }
  const topology = topologyForState(state);
  const suffixes = GROUP_SERVICE_SUFFIXES.coprocessor;
  const names: string[] = [];
  for (let index = 0; index < topology.count; index += 1) {
    for (const suffix of suffixes) {
      names.push(toServiceName(suffix, index));
    }
  }
  return names;
};

/** Loads a template compose document for a component. */
const loadComposeDoc = async (component: string) =>
  YAML.parse(await fs.readFile(path.join(TEMPLATE_COMPOSE_DIR, `${component}-docker-compose.yml`), "utf8")) as ComposeDoc;

/** Loads a previously generated compose override document for a component. */
const loadGeneratedComposeDoc = async (component: string) =>
  YAML.parse(await fs.readFile(composePath(component), "utf8")) as ComposeDoc;

/** Merges a base compose document with a generated override document. */
const mergeComposeDocs = (base: ComposeDoc, override: ComposeDoc): ComposeDoc => ({
  ...base,
  ...override,
  services: { ...(base.services ?? {}), ...(override.services ?? {}) },
});

/** Loads the effective compose document seen by a component after overrides. */
export const loadMergedComposeDoc = async (component: string) => {
  const base = await loadComposeDoc(component);
  if (!(await exists(composePath(component)))) {
    return base;
  }
  return mergeComposeDocs(base, await loadGeneratedComposeDoc(component));
};

/** Returns the locally built service set owned by one coprocessor instance. */
const localServicesForInstance = (instance: ResolvedCoprocessorScenarioInstance) =>
  new Set(instance.localServices ?? GROUP_BUILD_SERVICES.coprocessor);

/** Computes the inherited coprocessor services that should be built locally. */
const coprocessorBuildServices = (plan: Pick<StackSpec, "overrides">) => {
  const overrides = plan.overrides.filter((override) => override.group === "coprocessor");
  if (!overrides.length) {
    return new Set<string>();
  }
  if (overrides.some((override) => !override.services?.length)) {
    return new Set(GROUP_BUILD_SERVICES.coprocessor);
  }
  return new Set(overrides.flatMap((override) => override.services ?? []));
};

/** Applies scenario image sourcing rules to one coprocessor service clone. */
const applyCoprocessorSource = (
  service: Record<string, unknown>,
  serviceName: string,
  instance: ResolvedCoprocessorScenarioInstance,
  locallyBuilt: boolean,
) => {
  if (locallyBuilt) {
    service.image = retagLocal(service.image, localInstanceTag(instance.index));
    service.build = localBuildSpecFor("coprocessor", serviceName);
    return;
  }
  if (instance.source.mode === "registry") {
    service.image = rewriteImageTag(service.image, instance.source.tag);
  }
  delete service.build;
};

/** Builds the generated coprocessor compose override across all scenario instances. */
const buildCoprocessorOverride = async (plan: StackSpec) => {
  const doc = rewriteComposePaths(await loadComposeDoc("coprocessor"));
  const next = structuredClone(doc);
  const clonedServices = new Set(Object.keys(doc.services));
  const services: Record<string, Record<string, unknown>> = {};
  const compat = compatPolicyForState(plan);
  const inheritedBuildServices = coprocessorBuildServices(plan);
  for (const instance of plan.coprocessor.instances) {
    const localServices =
      instance.source.mode === "local"
        ? localServicesForInstance(instance)
        : instance.source.mode === "inherit"
          ? inheritedBuildServices
          : new Set<string>();
    const envName = instance.index === 0 ? "coprocessor" : `coprocessor.${instance.index}`;
    const envFileValue = envPath(envName);
    const instanceEnv = await readEnvFile(envFileValue);
    const prefix = instance.index === 0 ? "coprocessor-" : `coprocessor${instance.index}-`;
    for (const [name, service] of Object.entries(doc.services)) {
      const suffix = name.replace(/^coprocessor-/, "");
      const serviceName = `${prefix}${suffix}`;
      const locallyBuilt = localServices.has(name);
      const adjusted = applyInstanceAdjustments(
        name,
        service,
        envFileValue,
        instanceEnv,
        instance,
        locallyBuilt ? {} : compat.coprocessorArgs,
        locallyBuilt ? {} : compat.coprocessorDropFlags,
      );
      adjusted.container_name = serviceName;
      applyCoprocessorSource(adjusted, name, instance, locallyBuilt);
      if (instance.index > 0 && adjusted.depends_on && typeof adjusted.depends_on === "object") {
        adjusted.depends_on = rewriteCoprocessorDependsOn(
          adjusted.depends_on as Record<string, unknown>,
          prefix,
          clonedServices,
        );
      }
      services[serviceName] = adjusted;
    }
  }
  next.services = services;
  return next;
};

/** Builds the generated compose override for one component. */
const buildComposeOverride = async (component: string, plan: StackSpec) => {
  if (component === "coprocessor") {
    return buildCoprocessorOverride(plan);
  }
  const template = rewriteComposePaths(structuredClone(await loadComposeDoc(component)));
  const overridden = overriddenServicesForComponent(plan, component);
  const services: ComposeDoc["services"] = {};
  for (const [name, service] of Object.entries(template.services)) {
    if (!overridden.has(name)) {
      continue;
    }
    const next = structuredClone(service);
    applyBuildPolicy(next, true);
    const build = localBuildSpecFor(component, name);
    if (build) {
      next.build = build;
    }
    services[name] = next;
  }
  return { services };
};

/** Builds a host-node compose override for an extra host chain. */
const buildExtraHostNodeOverride = async (
  chain: HostChainScenario,
  defaultChain: HostChainScenario,
): Promise<ComposeDoc> => {
  const doc = rewriteComposePaths(await loadComposeDoc("host-node"));
  const hostNode = doc.services["host-node"];
  if (!hostNode) return { services: {} };
  const { node: container } = hostChainNames(chain.key, defaultChain.key);
  const clone = structuredClone(hostNode);
  clone.container_name = container;
  clone.env_file = [envPath(container)];
  clone.ports = [`${chain.rpcPort}:${chain.rpcPort}`];
  if (Array.isArray(clone.entrypoint)) {
    clone.entrypoint = clone.entrypoint.map((arg: string) => {
      if (arg === String(defaultChain.rpcPort)) return String(chain.rpcPort);
      if (arg === defaultChain.chainId) return chain.chainId;
      return arg;
    });
  }
  return { services: { [container]: clone } };
};

/** Builds a host-sc compose override for an extra host chain. */
const buildExtraHostScOverride = async (
  plan: StackSpec,
  chain: HostChainScenario,
  defaultChain: HostChainScenario,
): Promise<ComposeDoc> => {
  const doc = rewriteComposePaths(await loadComposeDoc("host-sc"));
  const { sc: scPrefix } = hostChainNames(chain.key, defaultChain.key);
  const localHostContracts = overriddenServicesForComponent(plan, "host-sc").size > 0;
  const services: Record<string, Record<string, unknown>> = {};
  for (const [name, service] of Object.entries(doc.services)) {
    const cloneName = name.replace("host-sc-", `${scPrefix}-`);
    const cloneService = structuredClone(service);
    cloneService.container_name = cloneName;
    cloneService.env_file = [envPath(scPrefix)];
    applyBuildPolicy(cloneService, localHostContracts);
    if (localHostContracts) {
      cloneService.build = localBuildSpecFor("host-sc", name);
    }
    if (cloneService.depends_on && typeof cloneService.depends_on === "object") {
      cloneService.depends_on = Object.fromEntries(
        Object.entries(cloneService.depends_on as Record<string, unknown>).map(([dep, value]) => [
          dep.replace("host-sc-", `${scPrefix}-`),
          value,
        ]),
      );
    }
    if (Array.isArray(cloneService.volumes)) {
      cloneService.volumes = (cloneService.volumes as string[]).map((vol: string) =>
        vol.replace("${HOST_ADDRESS_DIR:-host}", chain.key),
      );
    }
    services[cloneName] = cloneService;
  }
  return { services };
};

/** Builds coprocessor host-listener overrides for an extra host chain. */
const buildExtraCoprocessorListenerOverride = async (
  plan: StackSpec,
  chain: HostChainScenario,
  defaultChain: HostChainScenario,
): Promise<ComposeDoc> => {
  const doc = rewriteComposePaths(await loadComposeDoc("coprocessor"));
  const services: Record<string, Record<string, unknown>> = {};
  const compat = compatPolicyForState(plan);
  const inheritedBuildServices = coprocessorBuildServices(plan);
  const listenerServices = ["coprocessor-host-listener", "coprocessor-host-listener-poller"];
  const { suffix: chainSuffix } = hostChainNames(chain.key, defaultChain.key);
  for (const instance of plan.coprocessor.instances) {
    const localServices =
      instance.source.mode === "local"
        ? localServicesForInstance(instance)
        : instance.source.mode === "inherit"
          ? inheritedBuildServices
          : new Set<string>();
    const prefix = instance.index === 0 ? "coprocessor-" : `coprocessor${instance.index}-`;
    const envName = `coprocessor-${chain.key}.${instance.index}`;
    const envFileValue = envPath(envName);
    const instanceEnv = await readEnvFile(envFileValue);
    for (const baseName of listenerServices) {
      const suffix = baseName.replace(/^coprocessor-/, "");
      const cloneName = `${prefix}${suffix}${chainSuffix}`;
      const baseService = doc.services[baseName];
      if (!baseService) continue;
      const locallyBuilt = localServices.has(baseName);
      const adjusted = applyInstanceAdjustments(
        baseName,
        baseService,
        envFileValue,
        instanceEnv,
        instance,
        locallyBuilt ? {} : compat.coprocessorArgs,
        locallyBuilt ? {} : compat.coprocessorDropFlags,
      );
      adjusted.container_name = cloneName;
      applyCoprocessorSource(adjusted, baseName, instance, locallyBuilt);
      delete adjusted.depends_on;
      services[cloneName] = adjusted;
    }
  }
  return { services };
};

/** Lists which components need generated compose overrides for a runtime plan. */
export const generatedComposeComponents = (plan: Pick<StackSpec, "overrides">) =>
  new Set(["coprocessor", ...plan.overrides.flatMap((override) => GROUP_BUILD_COMPONENTS[override.group])]);

/** Generates or removes compose override files to match the current runtime plan. */
export const generateComposeOverrides = async (_state: State, plan: StackSpec) => {
  await ensureDir(COMPOSE_OUT_DIR);
  const generated = generatedComposeComponents(plan);
  for (const component of COMPONENTS) {
    const target = composePath(component);
    if (!generated.has(component)) {
      await remove(target);
      continue;
    }
    const doc = await buildComposeOverride(component, plan);
    await fs.writeFile(target, YAML.stringify(doc));
  }

  // Extra host chain compose overrides
  const chains = hostChainRuntimes(plan.hostChains);
  const defaultChain = chains[0];
  if (!defaultChain) {
    return;
  }
  const extraChains = chains.filter((chain) => !chain.isDefault);
  const extraChainFileNames: string[] = [];
  for (const chain of extraChains) {
    const { node, sc, copro } = chain;
    extraChainFileNames.push(node, sc, copro);
    const [hostNodeDoc, hostScDoc, coproDoc] = await Promise.all([
      buildExtraHostNodeOverride(chain, defaultChain),
      buildExtraHostScOverride(plan, chain, defaultChain),
      buildExtraCoprocessorListenerOverride(plan, chain, defaultChain),
    ]);
    await fs.writeFile(composePath(node), YAML.stringify(hostNodeDoc));
    await fs.writeFile(composePath(sc), YAML.stringify(hostScDoc));
    await fs.writeFile(composePath(copro), YAML.stringify(coproDoc));
  }
  // Clean up stale multi-chain compose files from previous runs.
  // Scan the output directory for files matching multi-chain naming patterns
  // and remove any that are not part of the current active set.
  const multiChainPrefixes = (({ node, sc, copro }) => [node, sc, copro])(hostChainNames("__placeholder__"))
    .map((value) => value.replace("__placeholder__", ""));
  const activeSet = new Set(extraChainFileNames);
  const dirEntries = await fs.readdir(COMPOSE_OUT_DIR).catch(() => [] as string[]);
  for (const entry of dirEntries) {
    if (!entry.endsWith(".yml")) continue;
    const name = entry.slice(0, -4); // strip .yml
    if (multiChainPrefixes.some((prefix) => name.startsWith(prefix)) && !activeSet.has(name)) {
      await remove(composePath(name));
    }
  }
};
