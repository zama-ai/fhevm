import { Effect } from "effect";
import fs from "node:fs/promises";
import path from "node:path";
import YAML from "yaml";

import { compatPolicyForState, type CompatPolicy } from "./compat";
import type { RuntimePlan } from "./runtime-plan";
import {
  COMPOSE_OUT_DIR,
  TEMPLATE_COMPOSE_DIR,
  composePath,
  envPath,
  GROUP_BUILD_COMPONENTS,
  GROUP_BUILD_SERVICES,
  GROUP_SERVICE_SUFFIXES,
  COMPONENTS,
} from "./layout";
import type { ResolvedCoprocessorScenarioInstance, State } from "./types";
import {
  ensureDir,
  exists,
  mergeArgs,
  readEnvFile,
  remove,
  toServiceName,
} from "./utils";

export type ComposeDoc = Record<string, unknown> & {
  services: Record<string, Record<string, unknown>>;
};

export const LOCAL_BUILD_TAG = "fhevm-local";
export const localInstanceTag = (index: number) => `${LOCAL_BUILD_TAG}-i${index}`;

export const resolvedComposeEnv = (state: Pick<State, "versions">): Record<string, string> => ({
  ...state.versions.env,
  COMPOSE_IGNORE_ORPHANS: "true",
});

export const overriddenServicesForComponent = (
  state: Pick<State, "overrides"> | Pick<RuntimePlan, "overrides">,
  component: string,
) =>
  new Set(
    state.overrides.flatMap((o) => {
      if (o.group === "coprocessor" || !GROUP_BUILD_COMPONENTS[o.group].includes(component)) {
        return [];
      }
      return o.services?.length ? o.services : GROUP_BUILD_SERVICES[o.group];
    }),
  );

export const rewriteImageTag = (image: unknown, tag: string) =>
  typeof image === "string" ? image.replace(/:([^:]+)$/, `:${tag}`) : image;

export const retagLocal = (image: unknown, tag = LOCAL_BUILD_TAG) =>
  rewriteImageTag(image, tag);

export const applyBuildPolicy = (service: Record<string, unknown>, isOverridden: boolean) => {
  if (isOverridden) {
    service.image = retagLocal(service.image);
  } else {
    delete service.build;
  }
};

export const appendVolume = (service: Record<string, unknown>, value: string) => {
  const volumes = Array.isArray(service.volumes) ? [...service.volumes] : [];
  const target = value.split(":").slice(1).join(":");
  const filtered = target
    ? volumes.filter((v) => typeof v !== "string" || v.split(":").slice(1).join(":") !== target)
    : volumes;
  if (!filtered.includes(value)) {
    filtered.push(value);
  }
  service.volumes = filtered;
};

export const resolveComposePath = (value: string) =>
  value.startsWith(".") ? path.resolve(TEMPLATE_COMPOSE_DIR, value) : value;

export const rewriteVolume = (value: unknown) => {
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

export const rewriteComposePaths = (doc: ComposeDoc) => {
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
  value.replace(/(?<!\$)\$\{([A-Z0-9_]+)\}/g, (match, key) =>
    key in vars ? vars[key] : match,
  );

export const interpolateComposeValue = (value: unknown, vars: Record<string, string>): unknown => {
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

export const rewriteCoprocessorDependsOn = (
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

export const applyInstanceAdjustments = (
  service: Record<string, unknown>,
  envFileValue: string,
  envVars: Record<string, string>,
  override: Pick<ResolvedCoprocessorScenarioInstance, "env" | "args"> = {
    env: {},
    args: {},
  },
  compatArgs: CompatPolicy["coprocessorArgs"] = {},
  compatDropFlags: CompatPolicy["coprocessorDropFlags"] = {},
) => {
  const next = interpolateComposeValue(structuredClone(service), envVars) as Record<string, unknown>;
  const containerName = String(next.container_name ?? "");
  const command = Array.isArray(next.command) ? next.command.map((item) => String(item)) : undefined;
  if (command?.some((item) => item.startsWith("--key-cache-size"))) {
    next.command = command.map((item) => item.replace("--key-cache-size", "--tenant-key-cache-size"));
  }
  if (containerName.endsWith("gw-listener")) {
    next.healthcheck = { disable: true };
  }
  next.env_file = [envFileValue];
  if (Object.keys(override.env).length) {
    next.environment = { ...(next.environment as Record<string, string> | undefined), ...override.env };
  }
  if (next.command) {
    const current = Array.isArray(next.command) ? next.command : [];
    const key = containerName.replace(
      /^coprocessor\d*-/,
      "",
    ) as keyof CompatPolicy["coprocessorArgs"];
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
    const extras = (compatArgs[key] ?? []).flatMap(([flag, source]) => {
      if ("value" in source) {
        return [flag, source.value];
      }
      return envVars[source.env] ? [flag, envVars[source.env]] : [];
    });
    next.command = extras.length ? mergeArgs(filtered, extras) : filtered;
  }
  if (next.command) {
    const current = Array.isArray(next.command) ? next.command : [];
    const key = containerName.replace(/^coprocessor\d*-/, "");
    next.command = mergeArgs(current, override.args[key] ?? override.args["*"] ?? []);
  }
  return next;
};

export const serviceNameList = (state: Pick<State, "topology">, component: string) => {
  if (component !== "coprocessor") {
    return [];
  }
  const suffixes = GROUP_SERVICE_SUFFIXES["coprocessor"];
  const names: string[] = [];
  for (let index = 0; index < state.topology.count; index += 1) {
    for (const suffix of suffixes) {
      names.push(toServiceName(suffix, index));
    }
  }
  return names;
};

export const loadComposeDoc = (component: string) =>
  Effect.tryPromise({
    try: () =>
      fs
        .readFile(path.join(TEMPLATE_COMPOSE_DIR, `${component}-docker-compose.yml`), "utf8")
        .then((text) => YAML.parse(text) as ComposeDoc),
    catch: (cause) =>
      new Error(`Failed to load compose template for ${component}: ${cause}`),
  });

export const loadGeneratedComposeDoc = (component: string) =>
  Effect.tryPromise({
    try: () =>
      fs
        .readFile(composePath(component), "utf8")
        .then((text) => YAML.parse(text) as ComposeDoc),
    catch: (cause) =>
      new Error(`Failed to load generated compose override for ${component}: ${cause}`),
  });

export const mergeComposeDocs = (base: ComposeDoc, override: ComposeDoc): ComposeDoc => ({
  ...base,
  ...override,
  services: {
    ...(base.services ?? {}),
    ...(override.services ?? {}),
  },
});

export const loadMergedComposeDoc = (component: string) =>
  Effect.gen(function* () {
    const base = yield* loadComposeDoc(component);
    if (!(yield* Effect.promise(() => exists(composePath(component))))) {
      return base;
    }
    const override = yield* loadGeneratedComposeDoc(component);
    return mergeComposeDocs(base, override);
  });

const localServicesForInstance = (instance: ResolvedCoprocessorScenarioInstance) =>
  new Set(instance.localServices ?? GROUP_BUILD_SERVICES["coprocessor"]);

const applyCoprocessorSource = (
  service: Record<string, unknown>,
  serviceName: string,
  instance: ResolvedCoprocessorScenarioInstance,
) => {
  if (instance.source.mode === "local") {
    const localServices = localServicesForInstance(instance);
    if (localServices.has(serviceName)) {
      service.image = retagLocal(service.image, localInstanceTag(instance.index));
      return;
    }
  }
  if (instance.source.mode === "registry") {
    service.image = rewriteImageTag(service.image, instance.source.tag);
  }
  delete service.build;
};

const buildCoprocessorOverride = (state: State, plan: RuntimePlan) =>
  Effect.gen(function* () {
    const doc = rewriteComposePaths(yield* loadComposeDoc("coprocessor"));
    const next = structuredClone(doc);
    const clonedServices = new Set(Object.keys(doc.services));
    const services: Record<string, Record<string, unknown>> = {};
    const compat = compatPolicyForState(plan);
    for (const instance of plan.coprocessor.instances) {
      const envName = instance.index === 0 ? "coprocessor" : `coprocessor.${instance.index}`;
      const envFileValue = envPath(envName);
      const instanceEnv = yield* Effect.promise(() => readEnvFile(envFileValue));
      const prefix = instance.index === 0 ? "coprocessor-" : `coprocessor${instance.index}-`;
      for (const [name, service] of Object.entries(doc.services)) {
        const suffix = name.replace(/^coprocessor-/, "");
        const serviceName = `${prefix}${suffix}`;
        const locallyBuilt =
          instance.source.mode === "local" && localServicesForInstance(instance).has(name);
        const compatArgs = locallyBuilt ? {} : compat.coprocessorArgs;
        const compatDropFlags = locallyBuilt ? {} : compat.coprocessorDropFlags;
        const adjusted = applyInstanceAdjustments(
          service,
          envFileValue,
          instanceEnv,
          instance,
          compatArgs,
          compatDropFlags,
        );
        adjusted.container_name = serviceName;
        applyCoprocessorSource(adjusted, name, instance);
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
  });

export const buildComposeOverride = (
  component: string,
  state: State,
  plan: RuntimePlan,
) =>
  Effect.gen(function* () {
    if (component === "coprocessor") {
      return yield* buildCoprocessorOverride(state, plan);
    }
    const template = rewriteComposePaths(structuredClone(yield* loadComposeDoc(component)));
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
  });

export const generatedComposeComponents = (plan: Pick<RuntimePlan, "overrides">) =>
  new Set([
    "coprocessor",
    ...plan.overrides.flatMap((override) => GROUP_BUILD_COMPONENTS[override.group]),
  ]);

export const generateComposeOverrides = (
  state: State,
  plan: RuntimePlan,
) =>
  Effect.gen(function* () {
    yield* Effect.promise(() => ensureDir(COMPOSE_OUT_DIR));
    const generated = generatedComposeComponents(plan);
    for (const component of COMPONENTS) {
      const target = composePath(component);
      if (!generated.has(component)) {
        yield* Effect.promise(() => remove(target));
        continue;
      }
      const doc = yield* buildComposeOverride(component, state, plan);
      yield* Effect.promise(() => fs.writeFile(target, YAML.stringify(doc)));
    }
  });
