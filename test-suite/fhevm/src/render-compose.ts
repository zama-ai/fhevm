import fs from "node:fs/promises";
import path from "node:path";

import YAML from "yaml";

import { compatPolicyForState, type CompatPolicy } from "./compat";
import { topologyForState, type RuntimePlan } from "./runtime-plan";
import {
  COMPONENTS,
  COMPOSE_OUT_DIR,
  GROUP_BUILD_COMPONENTS,
  GROUP_BUILD_SERVICES,
  GROUP_SERVICE_SUFFIXES,
  TEMPLATE_COMPOSE_DIR,
  composePath,
  envPath,
} from "./layout";
import type { ResolvedCoprocessorScenarioInstance, State } from "./types";
import { ensureDir, exists, mergeArgs, readEnvFile, remove, toServiceName } from "./utils";

export type ComposeDoc = Record<string, unknown> & {
  services: Record<string, Record<string, unknown>>;
};

const LOCAL_BUILD_TAG = "fhevm-local";
const localInstanceTag = (index: number) => `${LOCAL_BUILD_TAG}-i${index}`;

export const resolvedComposeEnv = (state: Pick<State, "versions">): Record<string, string> => ({
  ...state.versions.env,
  COMPOSE_IGNORE_ORPHANS: "true",
});

const overriddenServicesForComponent = (
  state: Pick<State, "overrides"> | Pick<RuntimePlan, "overrides">,
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

const rewriteImageTag = (image: unknown, tag: string) =>
  typeof image === "string" ? image.replace(/:([^:]+)$/, `:${tag}`) : image;

const retagLocal = (image: unknown, tag = LOCAL_BUILD_TAG) => rewriteImageTag(image, tag);

const applyBuildPolicy = (service: Record<string, unknown>, isOverridden: boolean) => {
  if (isOverridden) {
    service.image = retagLocal(service.image);
  } else {
    delete service.build;
  }
};

const resolveComposePath = (value: string) =>
  value.startsWith(".") ? path.resolve(TEMPLATE_COMPOSE_DIR, value) : value;

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

export const interpolateString = (value: string, vars: Record<string, string>) =>
  value.replace(/(?<!\$)\$\{([A-Z0-9_]+)\}/g, (match, key) => (key in vars ? vars[key] : match));

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
    next.environment = { ...(next.environment as Record<string, string> | undefined), ...override.env };
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
    next.command = mergeArgs(current, override.args[serviceKey] ?? override.args["*"] ?? []);
  }
  return next;
};

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

const loadComposeDoc = async (component: string) =>
  YAML.parse(await fs.readFile(path.join(TEMPLATE_COMPOSE_DIR, `${component}-docker-compose.yml`), "utf8")) as ComposeDoc;

const loadGeneratedComposeDoc = async (component: string) =>
  YAML.parse(await fs.readFile(composePath(component), "utf8")) as ComposeDoc;

const mergeComposeDocs = (base: ComposeDoc, override: ComposeDoc): ComposeDoc => ({
  ...base,
  ...override,
  services: { ...(base.services ?? {}), ...(override.services ?? {}) },
});

export const loadMergedComposeDoc = async (component: string) => {
  const base = await loadComposeDoc(component);
  if (!(await exists(composePath(component)))) {
    return base;
  }
  return mergeComposeDocs(base, await loadGeneratedComposeDoc(component));
};

const localServicesForInstance = (instance: ResolvedCoprocessorScenarioInstance) =>
  new Set(instance.localServices ?? GROUP_BUILD_SERVICES.coprocessor);

const applyCoprocessorSource = (
  service: Record<string, unknown>,
  serviceName: string,
  instance: ResolvedCoprocessorScenarioInstance,
  localServices: ReadonlySet<string>,
) => {
  if (instance.source.mode === "local" && localServices.has(serviceName)) {
    service.image = retagLocal(service.image, localInstanceTag(instance.index));
    return;
  }
  if (instance.source.mode === "registry") {
    service.image = rewriteImageTag(service.image, instance.source.tag);
  }
  delete service.build;
};

const buildCoprocessorOverride = async (plan: RuntimePlan) => {
  const doc = rewriteComposePaths(await loadComposeDoc("coprocessor"));
  const next = structuredClone(doc);
  const clonedServices = new Set(Object.keys(doc.services));
  const services: Record<string, Record<string, unknown>> = {};
  const compat = compatPolicyForState(plan);
  for (const instance of plan.coprocessor.instances) {
    const localServices = localServicesForInstance(instance);
    const envName = instance.index === 0 ? "coprocessor" : `coprocessor.${instance.index}`;
    const envFileValue = envPath(envName);
    const instanceEnv = await readEnvFile(envFileValue);
    const prefix = instance.index === 0 ? "coprocessor-" : `coprocessor${instance.index}-`;
    for (const [name, service] of Object.entries(doc.services)) {
      const suffix = name.replace(/^coprocessor-/, "");
      const serviceName = `${prefix}${suffix}`;
      const locallyBuilt = instance.source.mode === "local" && localServices.has(name);
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
      applyCoprocessorSource(adjusted, name, instance, localServices);
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

const buildComposeOverride = async (component: string, plan: RuntimePlan) => {
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
    services[name] = next;
  }
  return { services };
};

export const generatedComposeComponents = (plan: Pick<RuntimePlan, "overrides">) =>
  new Set(["coprocessor", ...plan.overrides.flatMap((override) => GROUP_BUILD_COMPONENTS[override.group])]);

export const generateComposeOverrides = async (_state: State, plan: RuntimePlan) => {
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
};
