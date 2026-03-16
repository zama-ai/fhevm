import { Effect } from "effect";
import fs from "node:fs/promises";
import path from "node:path";
import YAML from "yaml";

import { compatPolicyForState, type CompatPolicy } from "./compat";
import { EnvWriter } from "./services/EnvWriter";
import {
  ADDRESS_DIR,
  COMPONENTS,
  COMPOSE_OUT_DIR,
  TEMPLATE_COMPOSE_DIR,
  composePath,
  envPath,
  relayerConfigPath,
  GROUP_BUILD_COMPONENTS,
  GROUP_BUILD_SERVICES,
  GROUP_SERVICE_SUFFIXES,
} from "./layout";
import type { InstanceOverride, State } from "./types";
import { ensureDir, mergeArgs, readEnvFile, toServiceName } from "./utils";

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

export type ComposeDoc = Record<string, unknown> & {
  services: Record<string, Record<string, unknown>>;
};

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

export const LOCAL_BUILD_TAG = "fhevm-local";

// ---------------------------------------------------------------------------
// Pure functions
// ---------------------------------------------------------------------------

export const resolvedComposeEnv = (state: Pick<State, "versions">): Record<string, string> => ({
  ...state.versions.env,
  COMPOSE_IGNORE_ORPHANS: "true",
});

export const overriddenServicesForComponent = (state: State, component: string) =>
  new Set(
    state.overrides.flatMap((o) => {
      if (!GROUP_BUILD_COMPONENTS[o.group].includes(component)) {
        return [];
      }
      return o.services?.length ? o.services : GROUP_BUILD_SERVICES[o.group];
    }),
  );

export const retagLocal = (image: unknown) =>
  typeof image === "string" ? image.replace(/:([^:]+)$/, `:${LOCAL_BUILD_TAG}`) : image;

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
  // Remove any existing mount to the same container path (e.g. named volumes)
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
  override?: InstanceOverride,
  compatArgs: CompatPolicy["coprocessorArgs"] = {},
) => {
  const next = interpolateComposeValue(structuredClone(service), envVars) as Record<string, unknown>;
  const command = Array.isArray(next.command) ? next.command.map((item) => String(item)) : undefined;
  if (command?.some((item) => item.startsWith("--key-cache-size"))) {
    next.command = command.map((item) => item.replace("--key-cache-size", "--tenant-key-cache-size"));
  }
  if (typeof next.container_name === "string" && next.container_name.endsWith("gw-listener")) {
    next.healthcheck = { disable: true };
  }
  next.env_file = [envFileValue];
  if (override?.env && Object.keys(override.env).length) {
    next.environment = { ...(next.environment as Record<string, string> | undefined), ...override.env };
  }
  if (next.command) {
    const current = Array.isArray(next.command) ? next.command : [];
    const key = String(next.container_name ?? "").replace(
      /^coprocessor\d*-/,
      "",
    ) as keyof CompatPolicy["coprocessorArgs"];
    const extras = (compatArgs[key] ?? []).flatMap(([flag, source]) => {
      if ("value" in source) {
        return [flag, source.value];
      }
      return envVars[source.env] ? [flag, envVars[source.env]] : [];
    });
    if (extras.length) {
      next.command = mergeArgs(current, extras);
    }
  }
  if (override?.args && next.command) {
    const current = Array.isArray(next.command) ? next.command : [];
    const key = (next.container_name as string).replace(/^coprocessor\d*-/, "");
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

// ---------------------------------------------------------------------------
// File I/O — Effect-wrapped
// ---------------------------------------------------------------------------

export const loadComposeDoc = (component: string) =>
  Effect.tryPromise({
    try: () =>
      fs
        .readFile(path.join(TEMPLATE_COMPOSE_DIR, `${component}-docker-compose.yml`), "utf8")
        .then((text) => YAML.parse(text) as ComposeDoc),
    catch: (cause) =>
      new Error(`Failed to load compose template for ${component}: ${cause}`),
  });

const buildCoprocessorOverride = (state: State) =>
  Effect.gen(function* () {
    const doc = rewriteComposePaths(yield* loadComposeDoc("coprocessor"));
    const next = structuredClone(doc);
    const overridden = overriddenServicesForComponent(state, "coprocessor");
    const clonedServices = new Set(Object.keys(doc.services));
    const services: Record<string, Record<string, unknown>> = {};
    const baseOverride = state.topology.instances["coprocessor-0"];
    const baseEnv = yield* Effect.promise(() => readEnvFile(envPath("coprocessor")));
    const compat = compatPolicyForState(state);
    for (const [name, service] of Object.entries(doc.services)) {
      const compatArgs = overridden.has(name) ? {} : compat.coprocessorArgs;
      const adjusted = applyInstanceAdjustments(
        service,
        envPath("coprocessor"),
        baseEnv,
        baseOverride,
        compatArgs,
      );
      applyBuildPolicy(adjusted, overridden.has(name));
      services[name] = adjusted;
    }
    for (let index = 1; index < state.topology.count; index += 1) {
      const prefix = `coprocessor${index}-`;
      const override = state.topology.instances[`coprocessor-${index}`];
      const instanceEnv = yield* Effect.promise(() =>
        readEnvFile(envPath(`coprocessor.${index}`)),
      );
      for (const [name, service] of Object.entries(doc.services)) {
        const suffix = name.replace(/^coprocessor-/, "");
        const compatArgs = overridden.has(name) ? {} : compat.coprocessorArgs;
        const cloned = applyInstanceAdjustments(
          service,
          envPath(`coprocessor.${index}`),
          instanceEnv,
          override,
          compatArgs,
        );
        cloned.container_name = prefix + suffix;
        applyBuildPolicy(cloned, overridden.has(name));
        if (cloned.depends_on && typeof cloned.depends_on === "object") {
          cloned.depends_on = rewriteCoprocessorDependsOn(
            cloned.depends_on as Record<string, unknown>,
            prefix,
            clonedServices,
          );
        }
        services[prefix + suffix] = cloned;
      }
    }
    next.services = services;
    return next;
  });

export const buildComposeOverride = (component: string, state: State) =>
  Effect.gen(function* () {
    if (component === "coprocessor") {
      return yield* buildCoprocessorOverride(state);
    }
    const doc = rewriteComposePaths(structuredClone(yield* loadComposeDoc(component)));
    const overridden = overriddenServicesForComponent(state, component);
    const envVars = yield* Effect.promise(() => readEnvFile(envPath(component)));
    for (const [name, service] of Object.entries(doc.services)) {
      Object.assign(service, interpolateComposeValue(service, envVars));
      applyBuildPolicy(service, overridden.has(name));
      if (component === "gateway-sc") {
        if (name === "gateway-sc-add-network") {
          service.command = [
            "npx hardhat task:addHostChainsToGatewayConfig --use-internal-proxy-address true",
          ];
        }
        if (name === "gateway-sc-add-pausers") {
          service.command = [
            "npx hardhat task:addGatewayPausers --use-internal-pauser-set-address true",
          ];
        }
        if (name === "gateway-sc-trigger-keygen") {
          service.command = [
            "npx hardhat task:triggerKeygen --params-type 0 --use-internal-proxy-address true",
          ];
        }
        if (name === "gateway-sc-trigger-crsgen") {
          service.command = [
            "npx hardhat task:triggerCrsgen --params-type 0 --max-bit-length 2048 --use-internal-proxy-address true",
          ];
        }
      }
      if (component === "host-sc" && name === "host-sc-add-pausers") {
        service.command = [
          "npx hardhat task:addHostPausers --use-internal-pauser-set-address true",
        ];
      }
      service.env_file = [envPath(component)];
      if (component === "gateway-sc") {
        appendVolume(service, `${path.join(ADDRESS_DIR, "gateway")}:/app/addresses`);
      }
      if (component === "host-sc") {
        appendVolume(service, `${path.join(ADDRESS_DIR, "host")}:/app/addresses`);
      }
      if (component === "relayer" && name === "relayer") {
        appendVolume(service, `${relayerConfigPath}:/app/config/local.yaml`);
      }
      if (component === "core" && name === "kms-core") {
        service.healthcheck = { disable: true };
      }
    }
    return doc;
  });

export const generateComposeOverrides = (state: State) =>
  Effect.gen(function* () {
    yield* Effect.promise(() => ensureDir(COMPOSE_OUT_DIR));
    for (const component of COMPONENTS) {
      const doc = yield* buildComposeOverride(component, state);
      yield* Effect.promise(() =>
        fs.writeFile(composePath(component), YAML.stringify(doc)),
      );
    }
  });

// ---------------------------------------------------------------------------
// regen — combines EnvWriter + compose override generation
// ---------------------------------------------------------------------------

export const regen = (state: State) =>
  Effect.gen(function* () {
    const envWriter = yield* EnvWriter;
    yield* envWriter.generateEnvFiles(state);
    yield* generateComposeOverrides(state);
  });
