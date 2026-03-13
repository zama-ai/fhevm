import fs from "node:fs/promises";
import path from "node:path";

import YAML from "yaml";

import {
  compatPolicyForState,
  requiresLegacyRelayerReadinessConfig,
  requiresMultichainAclAddress,
  type CompatPolicy,
} from "./compat";
import {
  ADDRESS_DIR,
  COMPONENTS,
  CONFIG_DIR,
  COMPOSE_OUT_DIR,
  DEFAULT_TENANT_API_KEY,
  ENV_DIR,
  GROUP_BUILD_COMPONENTS,
  GROUP_BUILD_SERVICES,
  GROUP_SERVICE_SUFFIXES,
  TEMPLATE_COMPOSE_DIR,
  TEMPLATE_ENV_DIR,
  TEMPLATE_RELAYER_CONFIG,
  composePath,
  dockerArgs,
  envPath,
  relayerConfigPath,
  versionsEnvPath,
} from "./layout";
import type { BuiltImage, InstanceOverride, OverrideGroup, State } from "./types";
import type { RunOptions, Runner } from "./utils";
import {
  ensureDir,
  exists,
  mergeArgs,
  predictedCrsId,
  predictedKeyId,
  readEnvFile,
  toServiceName,
  writeEnvFile,
} from "./utils";

type ComposeDoc = Record<string, unknown> & {
  services: Record<string, Record<string, unknown>>;
};

type ArtifactDeps = {
  runner: Runner;
  liveRunner: (argv: string[], options?: Omit<RunOptions, "input">) => Promise<number>;
};

const HAS_PLACEHOLDER = /(?<!\$)\$\{[A-Z0-9_]+\}/;

export const resolvedComposeEnv = (state: Pick<State, "versions">): Record<string, string> => ({
  ...state.versions.env,
  COMPOSE_IGNORE_ORPHANS: "true",
});

const composeEnv = async (state?: State) =>
  state
    ? resolvedComposeEnv(state)
    : (await exists(versionsEnvPath))
      ? { ...(await readEnvFile(versionsEnvPath)), COMPOSE_IGNORE_ORPHANS: "true" }
      : {};

const ensureWritableDir = async (dir: string) => {
  await ensureDir(dir);
  await fs.chmod(dir, 0o777);
};

const LOCAL_BUILD_TAG = "fhevm-local";

const loadComposeDoc = async (component: string) =>
  YAML.parse(await fs.readFile(path.join(TEMPLATE_COMPOSE_DIR, `${component}-docker-compose.yml`), "utf8")) as ComposeDoc;

const overriddenServicesForComponent = (state: State, component: string) =>
  new Set(
    state.overrides.flatMap((o) => {
      if (!GROUP_BUILD_COMPONENTS[o.group].includes(component)) {
        return [];
      }
      return o.services?.length ? o.services : GROUP_BUILD_SERVICES[o.group];
    }),
  );

const retagLocal = (image: unknown) =>
  typeof image === "string" ? image.replace(/:([^:]+)$/, `:${LOCAL_BUILD_TAG}`) : image;

const applyBuildPolicy = (service: Record<string, unknown>, isOverridden: boolean) => {
  if (isOverridden) {
    service.image = retagLocal(service.image);
  } else {
    delete service.build;
  }
};

const appendVolume = (service: Record<string, unknown>, value: string) => {
  const volumes = Array.isArray(service.volumes) ? [...service.volumes] : [];
  const target = value.split(":").slice(1).join(":");
  // Remove any existing mount to the same container path (e.g. named volumes)
  const filtered = target ? volumes.filter((v) => typeof v !== "string" || v.split(":").slice(1).join(":") !== target) : volumes;
  if (!filtered.includes(value)) {
    filtered.push(value);
  }
  service.volumes = filtered;
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

const interpolateString = (value: string, vars: Record<string, string>) =>
  value.replace(/(?<!\$)\$\{([A-Z0-9_]+)\}/g, (match, key) => (key in vars ? vars[key] : match));

export const resolveEnvMap = (env: Record<string, string>) => {
  const unresolvedKeys = () =>
    Object.entries(env)
      .filter(([, value]) => HAS_PLACEHOLDER.test(value))
      .map(([key]) => key);
  for (let attempt = 0; attempt < 4; attempt += 1) {
    let changed = false;
    for (const [key, raw] of Object.entries(env)) {
      const value = typeof raw === "string" ? raw : "";
      const next = interpolateString(value, env);
      if (next !== value) {
        env[key] = next;
        changed = true;
      }
    }
    if (!changed) {
      const unresolved = unresolvedKeys();
      if (unresolved.length) {
        throw new Error(`Unresolved env interpolation for ${unresolved.join(", ")}`);
      }
      return env;
    }
  }
  const unresolved = unresolvedKeys();
  if (unresolved.length) {
    throw new Error(`Unresolved env interpolation for ${unresolved.join(", ")}`);
  }
  return env;
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

const applyInstanceAdjustments = (
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
    const key = String(next.container_name ?? "").replace(/^coprocessor\d*-/, "") as keyof CompatPolicy["coprocessorArgs"];
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

const buildCoprocessorOverride = async (state: State) => {
  const doc = rewriteComposePaths(await loadComposeDoc("coprocessor"));
  const next = structuredClone(doc);
  const overridden = overriddenServicesForComponent(state, "coprocessor");
  const clonedServices = new Set(Object.keys(doc.services));
  const services: Record<string, Record<string, unknown>> = {};
  const baseOverride = state.topology.instances["coprocessor-0"];
  const baseEnv = await readEnvFile(envPath("coprocessor"));
  const compat = compatPolicyForState(state);
  for (const [name, service] of Object.entries(doc.services)) {
    const compatArgs = overridden.has(name) ? {} : compat.coprocessorArgs;
    const adjusted = applyInstanceAdjustments(service, envPath("coprocessor"), baseEnv, baseOverride, compatArgs);
    applyBuildPolicy(adjusted, overridden.has(name));
    services[name] = adjusted;
  }
  for (let index = 1; index < state.topology.count; index += 1) {
    const prefix = `coprocessor${index}-`;
    const override = state.topology.instances[`coprocessor-${index}`];
    const instanceEnv = await readEnvFile(envPath(`coprocessor.${index}`));
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
};

const buildComposeOverride = async (component: string, state: State) => {
  if (component === "coprocessor") {
    return buildCoprocessorOverride(state);
  }
  const doc = rewriteComposePaths(structuredClone(await loadComposeDoc(component)));
  const overridden = overriddenServicesForComponent(state, component);
  const envVars = await readEnvFile(envPath(component));
  for (const [name, service] of Object.entries(doc.services)) {
    Object.assign(service, interpolateComposeValue(service, envVars));
    applyBuildPolicy(service, overridden.has(name));
    if (component === "gateway-sc") {
      if (name === "gateway-sc-add-network") {
        service.command = ["npx hardhat task:addHostChainsToGatewayConfig --use-internal-proxy-address true"];
      }
      if (name === "gateway-sc-add-pausers") {
        service.command = ["npx hardhat task:addGatewayPausers --use-internal-pauser-set-address true"];
      }
      if (name === "gateway-sc-trigger-keygen") {
        service.command = ["npx hardhat task:triggerKeygen --params-type 0 --use-internal-proxy-address true"];
      }
      if (name === "gateway-sc-trigger-crsgen") {
        service.command = ["npx hardhat task:triggerCrsgen --params-type 0 --max-bit-length 2048 --use-internal-proxy-address true"];
      }
    }
    if (component === "host-sc" && name === "host-sc-add-pausers") {
      service.command = ["npx hardhat task:addHostPausers --use-internal-pauser-set-address true"];
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
};

const writeComposeOverrides = async (state: State) => {
  await ensureDir(COMPOSE_OUT_DIR);
  for (const component of COMPONENTS) {
    await fs.writeFile(composePath(component), YAML.stringify(await buildComposeOverride(component, state)));
  }
};

const updateContracts = (env: Record<string, string>, values: Record<string, string>) => {
  for (const [key, value] of Object.entries(values)) {
    if (value !== undefined) {
      env[key] = value;
    }
  }
};

const deriveWallet = async (runner: Runner, mnemonic: string, index: number) => {
  const address = (
    await runner(["cast", "wallet", "address", "--mnemonic", mnemonic, "--mnemonic-index", String(index)])
  ).stdout.trim();
  const privateKey = (
    await runner(["cast", "wallet", "private-key", "--mnemonic", mnemonic, "--mnemonic-index", String(index)])
  ).stdout.trim();
  if (!/^0x[a-fA-F0-9]{40}$/.test(address)) {
    throw new Error(`cast returned invalid address for wallet ${index}: ${address}`);
  }
  if (!/^0x[a-fA-F0-9]{64}$/.test(privateKey)) {
    throw new Error(`cast returned invalid private key for wallet ${index}`);
  }
  return { address, privateKey };
};

export const rewriteRelayerConfig = (config: Record<string, unknown>, state: Pick<State, "versions">) => {
  if (!requiresLegacyRelayerReadinessConfig(state)) {
    return config;
  }
  const gateway = config.gateway;
  if (!gateway || typeof gateway !== "object") {
    return config;
  }
  const readiness = (gateway as Record<string, unknown>).readiness_checker;
  if (!readiness || typeof readiness !== "object") {
    return config;
  }
  const current = readiness as Record<string, unknown>;
  (gateway as Record<string, unknown>).readiness_checker = Object.fromEntries(
    Object.entries({
      retry:
        current.retry ??
        (current.gw_ciphertext_check as Record<string, unknown> | undefined)?.retry ??
        (current.host_acl_check as Record<string, unknown> | undefined)?.retry,
      public_decrypt: current.public_decrypt,
      user_decrypt: current.user_decrypt,
      delegated_user_decrypt: current.delegated_user_decrypt,
    }).filter(([, value]) => value !== undefined),
  );
  return config;
};

const writeRuntimeEnvFiles = async (state: State, deps: Pick<ArtifactDeps, "runner">) => {
  await ensureDir(ENV_DIR);
  const compat = compatPolicyForState(state);
  const envs = Object.fromEntries(
    await Promise.all(
      COMPONENTS.map(async (component) => [component, await readEnvFile(path.join(TEMPLATE_ENV_DIR, `.env.${component}`))]),
    ),
  ) as Record<string, Record<string, string>>;

  envs["gateway-sc"].NUM_COPROCESSORS = String(state.topology.count);
  envs["gateway-sc"].COPROCESSOR_THRESHOLD = String(state.topology.threshold);
  envs["host-sc"].NUM_COPROCESSORS = String(state.topology.count);
  envs["host-sc"].COPROCESSOR_THRESHOLD = String(state.topology.threshold);
  envs["coprocessor"].DATABASE_URL =
    `postgresql://${envs.database.POSTGRES_USER}:${envs.database.POSTGRES_PASSWORD}@db:5432/coprocessor`;
  envs["coprocessor"].TENANT_API_KEY = DEFAULT_TENANT_API_KEY;
  envs["coprocessor"].COPROCESSOR_API_KEY = DEFAULT_TENANT_API_KEY;
  envs["coprocessor"].AWS_ENDPOINT_URL = state.discovery?.endpoints.minioExternal ?? "http://minio:9000";
  const kp = state.discovery?.minioKeyPrefix ?? "PUB";
  const minioInt = state.discovery?.endpoints.minioInternal ?? "http://minio:9000";
  envs["coprocessor"].FHE_KEY_ID = state.discovery?.actualFheKeyId ?? state.discovery?.fheKeyId ?? predictedKeyId();
  envs["coprocessor"].KMS_PUBLIC_KEY = `${minioInt}/kms-public/${kp}/PublicKey/${envs["coprocessor"].FHE_KEY_ID}`;
  envs["coprocessor"].KMS_SERVER_KEY = `${minioInt}/kms-public/${kp}/ServerKey/${envs["coprocessor"].FHE_KEY_ID}`;
  envs["coprocessor"].KMS_SNS_KEY = `${minioInt}/kms-public/${kp}/SnsKey/${envs["coprocessor"].FHE_KEY_ID}`;
  envs["coprocessor"].KMS_CRS_KEY = `${minioInt}/kms-public/${kp}/CRS/${state.discovery?.actualCrsKeyId ?? state.discovery?.crsKeyId ?? predictedCrsId()}`;
  envs["relayer"].APP_KEYURL__FHE_PUBLIC_KEY__URL = `${minioInt}/kms-public/${kp}/PublicKey/${state.discovery?.actualFheKeyId ?? state.discovery?.fheKeyId ?? predictedKeyId()}`;
  envs["relayer"].APP_KEYURL__CRS__URL = `${minioInt}/kms-public/${kp}/CRS/${state.discovery?.actualCrsKeyId ?? state.discovery?.crsKeyId ?? predictedCrsId()}`;
  for (const [key, source] of Object.entries(compat.connectorEnv)) {
    if (envs["kms-connector"][source]) {
      envs["kms-connector"][key] = envs["kms-connector"][source];
    }
  }

  if (state.discovery?.kmsSigner) {
    envs["gateway-sc"].KMS_SIGNER_ADDRESS_0 = state.discovery.kmsSigner;
    envs["host-sc"].KMS_SIGNER_ADDRESS_0 = state.discovery.kmsSigner;
  }
  if (state.discovery) {
    updateContracts(envs["gateway-sc"], state.discovery.gateway);
    updateContracts(envs["gateway-mocked-payment"], {
      PROTOCOL_PAYMENT_ADDRESS: state.discovery.gateway.PROTOCOL_PAYMENT_ADDRESS,
    });
    updateContracts(envs["host-sc"], {
      DECRYPTION_ADDRESS: state.discovery.gateway.DECRYPTION_ADDRESS,
      INPUT_VERIFICATION_ADDRESS: state.discovery.gateway.INPUT_VERIFICATION_ADDRESS,
      ACL_CONTRACT_ADDRESS: state.discovery.host.ACL_CONTRACT_ADDRESS,
      PAUSER_SET_CONTRACT_ADDRESS: state.discovery.host.PAUSER_SET_CONTRACT_ADDRESS,
    });
    envs["gateway-sc"].HOST_CHAIN_FHEVM_EXECUTOR_ADDRESS_0 = state.discovery.host.FHEVM_EXECUTOR_CONTRACT_ADDRESS;
    envs["gateway-sc"].HOST_CHAIN_ACL_ADDRESS_0 = state.discovery.host.ACL_CONTRACT_ADDRESS;

    updateContracts(envs["coprocessor"], {
      ACL_CONTRACT_ADDRESS: state.discovery.host.ACL_CONTRACT_ADDRESS,
      FHEVM_EXECUTOR_CONTRACT_ADDRESS: state.discovery.host.FHEVM_EXECUTOR_CONTRACT_ADDRESS,
      INPUT_VERIFIER_ADDRESS: state.discovery.host.INPUT_VERIFIER_CONTRACT_ADDRESS,
      INPUT_VERIFICATION_ADDRESS: state.discovery.gateway.INPUT_VERIFICATION_ADDRESS,
      CIPHERTEXT_COMMITS_ADDRESS: state.discovery.gateway.CIPHERTEXT_COMMITS_ADDRESS,
      ...(requiresMultichainAclAddress(state)
        ? { MULTICHAIN_ACL_ADDRESS: state.discovery.gateway.MULTICHAIN_ACL_ADDRESS }
        : {}),
      KMS_GENERATION_ADDRESS: state.discovery.gateway.KMS_GENERATION_ADDRESS,
    });
    updateContracts(envs["kms-connector"], {
      KMS_CONNECTOR_DECRYPTION_CONTRACT__ADDRESS: state.discovery.gateway.DECRYPTION_ADDRESS,
      KMS_CONNECTOR_GATEWAY_CONFIG_CONTRACT__ADDRESS: state.discovery.gateway.GATEWAY_CONFIG_ADDRESS,
      KMS_CONNECTOR_KMS_GENERATION_CONTRACT__ADDRESS: state.discovery.gateway.KMS_GENERATION_ADDRESS,
      KMS_CONNECTOR_HOST_CHAINS: JSON.stringify([
        {
          url: state.discovery.endpoints.hostHttp,
          chain_id: Number(envs["coprocessor"].CHAIN_ID ?? "12345"),
          acl_address: state.discovery.host.ACL_CONTRACT_ADDRESS,
        },
      ]),
    });
    updateContracts(envs["relayer"], {
      APP_GATEWAY__CONTRACTS__DECRYPTION_ADDRESS: state.discovery.gateway.DECRYPTION_ADDRESS,
      APP_GATEWAY__CONTRACTS__INPUT_VERIFICATION_ADDRESS: state.discovery.gateway.INPUT_VERIFICATION_ADDRESS,
    });
    updateContracts(envs["test-suite"], {
      DECRYPTION_ADDRESS: state.discovery.gateway.DECRYPTION_ADDRESS,
      INPUT_VERIFICATION_ADDRESS: state.discovery.gateway.INPUT_VERIFICATION_ADDRESS,
      KMS_VERIFIER_CONTRACT_ADDRESS: state.discovery.host.KMS_VERIFIER_CONTRACT_ADDRESS,
      ACL_CONTRACT_ADDRESS: state.discovery.host.ACL_CONTRACT_ADDRESS,
      INPUT_VERIFIER_CONTRACT_ADDRESS: state.discovery.host.INPUT_VERIFIER_CONTRACT_ADDRESS,
      FHEVM_EXECUTOR_CONTRACT_ADDRESS: state.discovery.host.FHEVM_EXECUTOR_CONTRACT_ADDRESS,
    });
  }

  const indices = [5, 8, 9, 10, 11];
  if (state.topology.count > 1) {
    const mnemonic = envs["gateway-sc"].MNEMONIC;
    if (!mnemonic) {
      throw new Error("Missing gateway mnemonic for multicopro setup");
    }
    for (let index = 0; index < state.topology.count; index += 1) {
      const wallet = await deriveWallet(deps.runner, mnemonic, indices[index]);
      envs["gateway-sc"][`COPROCESSOR_TX_SENDER_ADDRESS_${index}`] = wallet.address;
      envs["gateway-sc"][`COPROCESSOR_SIGNER_ADDRESS_${index}`] = wallet.address;
      envs["gateway-sc"][`COPROCESSOR_S3_BUCKET_URL_${index}`] = "http://minio:9000/ct128";
      envs["host-sc"][`COPROCESSOR_SIGNER_ADDRESS_${index}`] = wallet.address;
      if (index === 0) {
        envs["coprocessor"].TX_SENDER_PRIVATE_KEY = wallet.privateKey;
        continue;
      }
      const next = { ...envs["coprocessor"] };
      next.DATABASE_URL = `postgresql://${envs.database.POSTGRES_USER}:${envs.database.POSTGRES_PASSWORD}@db:5432/coprocessor_${index}`;
      next.TX_SENDER_PRIVATE_KEY = wallet.privateKey;
      const instance = state.topology.instances[`coprocessor-${index}`];
      Object.assign(next, instance?.env ?? {});
      resolveEnvMap(next);
      await writeEnvFile(envPath(`coprocessor.${index}`), next);
    }
  }
  for (const component of COMPONENTS) {
    resolveEnvMap(envs[component]);
    await writeEnvFile(envPath(component), envs[component]);
  }
  await writeEnvFile(
    versionsEnvPath,
    state.versions.env,
  );
  const relayerConfig = rewriteRelayerConfig(
    YAML.parse(await fs.readFile(TEMPLATE_RELAYER_CONFIG, "utf8")) as Record<string, unknown>,
    state,
  );
  await fs.writeFile(relayerConfigPath, YAML.stringify(relayerConfig));
};

const imageRefsForServices = async (component: string, services: string[]) => {
  const doc = YAML.parse(await fs.readFile(composePath(component), "utf8")) as ComposeDoc;
  const selected = services.length ? services : Object.keys(doc.services);
  return [...new Set(selected.map((name) => doc.services[name]?.image).filter((value): value is string => typeof value === "string" && value.length > 0))];
};

export const inspectImageId = async (runner: Runner, ref: string) => {
  const result = await runner(["docker", "image", "inspect", ref, "--format", "{{.Id}}"], { allowFailure: true });
  return result.code === 0 ? result.stdout.trim() : "";
};

const rememberBuiltImages = async (
  state: State,
  component: string,
  group: OverrideGroup,
  services: string[],
  deps: Pick<ArtifactDeps, "runner">,
  saveState: (state: State) => Promise<void>,
) => {
  const current = new Map((state.builtImages ?? []).map((item) => [item.ref, item] as const));
  for (const ref of await imageRefsForServices(component, services)) {
    const id = await inspectImageId(deps.runner, ref);
    if (!id) {
      continue;
    }
    current.set(ref, { ref, id, group } satisfies BuiltImage);
  }
  state.builtImages = [...current.values()].sort((a, b) => a.ref.localeCompare(b.ref));
  await saveState(state);
};

const maybeBuild = async (
  component: string,
  state: State,
  deps: ArtifactDeps,
  saveState: (state: State) => Promise<void>,
  log: (value: string) => void,
) => {
  for (const override of state.overrides) {
    if (GROUP_BUILD_COMPONENTS[override.group].includes(component)) {
      const doc = YAML.parse(await fs.readFile(composePath(component), "utf8")) as ComposeDoc;
      const available = new Set(Object.keys(doc.services));
      const candidates = override.services?.length ? override.services : GROUP_BUILD_SERVICES[override.group];
      const services = candidates.filter((s) => available.has(s));
      if (!services.length) {
        continue;
      }
      // Deduplicate services by image tag — buildx fails when two services
      // produce the same output tag in a single `docker compose build`.
      const seen = new Set<string>();
      const deduped = services.filter((s) => {
        const img = doc.services[s]?.image;
        if (typeof img !== "string" || seen.has(img)) {
          return false;
        }
        seen.add(img);
        return true;
      });
      log(`[build] ${override.group} (${component})`);
      for (const ref of await imageRefsForServices(component, deduped)) {
        await deps.runner(["docker", "image", "rm", "-f", ref], { allowFailure: true });
      }
      const buildBatches = override.group === "coprocessor" ? deduped.map((service) => [service]) : [deduped];
      for (const batch of buildBatches) {
        await deps.liveRunner([...dockerArgs(component), "build", ...batch], { env: await composeEnv(state) });
      }
      await rememberBuiltImages(state, component, override.group, services, deps, saveState);
    }
  }
};

export const composeUp = async (
  component: string,
  state: State,
  deps: ArtifactDeps,
  saveState: (state: State) => Promise<void>,
  log: (value: string) => void,
  services: string[] = [],
  options: { noDeps?: boolean } = {},
) => {
  await maybeBuild(component, state, deps, saveState, log);
  await deps.liveRunner(
    [
      ...dockerArgs(component),
      "up",
      "-d",
      ...(options.noDeps ? ["--no-deps"] : []),
      ...(services.length ? services : []),
    ],
    { env: await composeEnv(state) },
  );
};

export const composeDown = async (component: string, deps: Pick<ArtifactDeps, "liveRunner">) => {
  if (!(await exists(composePath(component)))) {
    return true;
  }
  const code = await deps.liveRunner([...dockerArgs(component), "down", "-v"], {
    allowFailure: true,
    env: await composeEnv(),
  });
  if (code !== 0) {
    console.warn(`[warn] compose down failed for ${component} (${code})`);
    return false;
  }
  return true;
};

export const regen = async (state: State, deps: Pick<ArtifactDeps, "runner">) => {
  await ensureWritableDir(path.join(ADDRESS_DIR, "gateway"));
  await ensureWritableDir(path.join(ADDRESS_DIR, "host"));
  await ensureDir(CONFIG_DIR);
  await writeRuntimeEnvFiles(state, deps);
  await writeComposeOverrides(state);
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
