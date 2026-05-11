import { supportsHostListenerConsumer } from "../compat/compat";
import { BootstrapTimeout, ContainerCrashed, MinioError, PreflightError, ProbeTimeout, RpcError } from "../errors";
import { requiresModernHostAddressArtifacts } from "../compat/compat";
import {
  COPROCESSOR_DB_CONTAINER,
  CRSGEN_ID_SELECTOR,
  DEFAULT_GATEWAY_RPC_PORT,
  GROUP_SERVICE_SUFFIXES,
  KEYGEN_ID_SELECTOR,
  KMS_CORE_CONTAINER,
  MINIO_EXTERNAL_URL,
  TEST_SUITE_CONTAINER,
  defaultHostChainKey,
  hostChainSuffix,
} from "../layout";
import { topologyForState } from "../stack-spec/stack-spec";
import type { State } from "../types";
import { hostReachableMaterialUrl, hostReachableRpcUrl, predictedCrsId, predictedKeyId, toServiceName, withHexPrefix } from "../utils/fs";
import { run } from "../utils/process";

const POST_BOOT_HEALTH_GATE_DELAY_MS = 5_000;

export const KMS_CONNECTOR_HEALTH_CONTAINERS = [
  "kms-connector-gw-listener",
  "kms-connector-kms-worker",
  "kms-connector-tx-sender",
];

/** Reads docker inspect data for a container and validates the JSON payload. */
export const dockerInspect = async (name: string) => {
  const result = await run(["docker", "inspect", name], { allowFailure: true });
  if (result.code !== 0) {
    const message = (result.stderr || result.stdout).trim();
    if (/no such object|no such container/i.test(message)) {
      return [] as Array<{
        Name: string;
        State: { Status: string; ExitCode: number; StartedAt?: string; Health?: { Status: string } };
        NetworkSettings: { Networks: Record<string, { IPAddress: string }> };
      }>;
    }
    throw new PreflightError(message || `docker inspect ${name} failed`);
  }
  try {
    return JSON.parse(result.stdout) as Array<{
      Name: string;
      State: { Status: string; ExitCode: number; StartedAt?: string; Health?: { Status: string } };
      NetworkSettings: { Networks: Record<string, { IPAddress: string }> };
    }>;
  } catch (error) {
    throw new PreflightError(
      `docker inspect ${name} returned invalid JSON: ${error instanceof Error ? error.message : String(error)}`,
    );
  }
};

/** Polls one container until it reaches the requested lifecycle state. */
export const waitForContainer = async (container: string, want: "running" | "healthy" | "complete") => {
  const attempts = 90;
  for (let attempt = 0; attempt <= attempts; attempt += 1) {
    const [inspect] = await dockerInspect(container);
    if (inspect) {
      if (want === "healthy" && inspect.State.Health?.Status === "healthy") {
        return;
      }
      if (want === "running" && inspect.State.Status === "running") {
        return;
      }
      if (want === "complete" && inspect.State.Status === "exited" && inspect.State.ExitCode === 0) {
        return;
      }
      if (inspect.State.Status === "exited" && inspect.State.ExitCode !== 0) {
        const logs = await run(["docker", "logs", container], { allowFailure: true });
        throw new ContainerCrashed(container, inspect.State.ExitCode, (logs.stdout + logs.stderr).trim());
      }
    }
    if (attempt === attempts) {
      throw new ProbeTimeout(container, 180);
    }
    await Bun.sleep(2_000);
  }
};

/** Waits until container logs contain the requested pattern. */
export const waitForLog = async (container: string, pattern: RegExp) => {
  for (let attempt = 0; attempt <= 90; attempt += 1) {
    const [inspect] = await dockerInspect(container);
    const logs = await run(
      ["docker", "logs", ...(inspect?.State.StartedAt ? ["--since", inspect.State.StartedAt] : []), container],
      { allowFailure: true },
    );
    const combined = logs.stdout + logs.stderr;
    const match = combined.match(pattern);
    if (match) {
      return match[0];
    }
    if (inspect?.State.Status === "exited") {
      if (inspect.State.ExitCode !== 0) {
        throw new ContainerCrashed(container, inspect.State.ExitCode, combined.trim());
      }
      throw new PreflightError(`${container} exited before emitting expected log pattern ${pattern}`);
    }
    if (attempt === 90) {
      throw new ProbeTimeout(container, 180);
    }
    await Bun.sleep(2_000);
  }
};

/** Waits until an RPC endpoint answers a basic `eth_chainId` request. */
export const waitForRpc = async (url: string) => {
  for (let attempt = 0; attempt <= 60; attempt += 1) {
    try {
      const response = await fetch(url, {
        method: "POST",
        headers: { "content-type": "application/json" },
        body: JSON.stringify({ jsonrpc: "2.0", id: 1, method: "eth_chainId", params: [] }),
      });
      if (response.ok) {
        const body = await response.json().catch(() => null) as
          | { jsonrpc?: string; result?: unknown; error?: unknown }
          | null;
        if (body?.jsonrpc === "2.0" && typeof body.result === "string" && !body.error) {
          return;
        }
      }
    } catch {
      // retry
    }
    if (attempt === 60) {
      throw new ProbeTimeout(url, 60);
    }
    await Bun.sleep(1_000);
  }
};

/** Fails fast if post-boot containers crash shortly after becoming ready. */
export const postBootHealthGate = async (containers: string[], delayMs = POST_BOOT_HEALTH_GATE_DELAY_MS) => {
  if (delayMs > 0) {
    await Bun.sleep(delayMs);
  }
  const crashed: { name: string; exitCode: number; logs: string }[] = [];
  for (const name of containers) {
    const [inspect] = await dockerInspect(name);
    if (!inspect) {
      crashed.push({ name, exitCode: -1, logs: "(container not found)" });
      continue;
    }
    if (inspect.State.Status === "exited" && inspect.State.ExitCode !== 0) {
      const result = await run(["docker", "logs", "--tail", "30", name], { allowFailure: true });
      crashed.push({ name, exitCode: inspect.State.ExitCode, logs: (result.stdout + result.stderr).trim() });
    }
  }
  if (crashed.length) {
    const first = crashed[0];
    const details = crashed
      .map((item) => `  ${item.name} (exit ${item.exitCode}):\n    ${item.logs.split("\n").join("\n    ")}`)
      .join("\n");
    throw new ContainerCrashed(first.name, first.exitCode, `Post-boot health gate: ${crashed.length} container(s) crashed:\n${details}`);
  }
};

/** Lists the coprocessor containers whose health determines coprocessor readiness. */
export const coprocessorHealthContainers = (state: Pick<State, "scenario" | "versions">) => {
  const topology = topologyForState(state);
  const suffixes = GROUP_SERVICE_SUFFIXES.coprocessor.filter(
    (suffix) =>
      !suffix.includes("migration") &&
      (suffix !== "host-listener-consumer" || supportsHostListenerConsumer(state)),
  );
  const names: string[] = [];
  for (let index = 0; index < topology.count; index += 1) {
    for (const suffix of suffixes) {
      names.push(toServiceName(suffix, index));
    }
  }
  return names;
};

/** Waits for all coprocessor runtime services to reach their expected states. */
export const waitForCoprocessorServices = async (state: State, skipMigration: boolean) => {
  const topology = topologyForState(state);
  for (let index = 0; index < topology.count; index += 1) {
    if (!skipMigration) {
      await waitForContainer(toServiceName("db-migration", index), "complete");
    }
    await waitForContainer(toServiceName("host-listener", index), "running");
    await waitForContainer(toServiceName("host-listener-poller", index), "running");
    if (supportsHostListenerConsumer(state)) {
      await waitForContainer(toServiceName("host-listener-consumer", index), "running");
    }
    await waitForContainer(toServiceName("gw-listener", index), "running");
    await waitForContainer(toServiceName("tfhe-worker", index), "running");
    await waitForContainer(toServiceName("zkproof-worker", index), "running");
    await waitForContainer(toServiceName("sns-worker", index), "running");
    await waitForContainer(toServiceName("transaction-sender", index), "running");
  }
};

/** Waits for the full coprocessor stack, including migrations, to become ready. */
export const waitForCoprocessor = async (state: State) => waitForCoprocessorServices(state, false);

/** Waits for extra-chain host listeners to reach running state. */
const waitForExtraChainCoprocessorListeners = async (state: Pick<State, "scenario">, chainKey: string) => {
  const suffix = hostChainSuffix(chainKey, defaultHostChainKey(state.scenario.hostChains));
  const topology = topologyForState(state);
  for (let index = 0; index < topology.count; index += 1) {
    const prefix = index === 0 ? "coprocessor-" : `coprocessor${index}-`;
    await waitForContainer(`${prefix}host-listener${suffix}`, "running");
    await waitForContainer(`${prefix}host-listener-poller${suffix}`, "running");
  }
};

/** Lists listener container names for one chain across all coprocessor instances. */
export const listenerContainersForChain = (state: Pick<State, "scenario">, chainKey: string) => {
  const suffix = hostChainSuffix(chainKey, defaultHostChainKey(state.scenario.hostChains));
  const topology = topologyForState(state);
  return Array.from({ length: topology.count }, (_, index) => {
    const prefix = index === 0 ? "coprocessor-" : `coprocessor${index}-`;
    return [`${prefix}host-listener${suffix}`, `${prefix}host-listener-poller${suffix}`];
  }).flat();
};

/** Waits for one chain listener set to become stable after startup. */
export const waitForStableChainListeners = async (state: Pick<State, "scenario">, chainKey: string) => {
  await waitForExtraChainCoprocessorListeners(state, chainKey);
  await postBootHealthGate(listenerContainersForChain(state, chainKey));
};

/** Discovers the KMS signer address and MinIO key prefix after bootstrap. */
export const discoverSigner = async () => {
  for (let attempt = 0; attempt <= 60; attempt += 1) {
    const logs = await run(["docker", "logs", KMS_CORE_CONTAINER], { allowFailure: true });
    const match = logs.stdout.match(/handle ([a-zA-Z0-9]+)/) ?? logs.stderr.match(/handle ([a-zA-Z0-9]+)/);
    if (match) {
      const handle = match[1];
      for (const prefix of ["PUB/PUB", "PUB"]) {
        try {
          const response = await fetch(`${MINIO_EXTERNAL_URL}/kms-public/${prefix}/VerfAddress/${handle}`);
          if (response.ok) {
            return {
              address: (await response.text()).trim(),
              minioKeyPrefix: prefix,
            };
          }
        } catch {
          // retry
        }
      }
    }
    if (attempt === 60) {
      throw new MinioError("Could not discover KMS signer after 60 attempts");
    }
    await Bun.sleep(1_000);
  }
  throw new MinioError("Could not discover KMS signer after 60 attempts");
};

/** Waits until one material artifact becomes available through host-reachable MinIO. */
export const ensureMaterial = async (url: string) => {
  for (let attempt = 0; attempt <= 30; attempt += 1) {
    try {
      const response = await fetch(hostReachableMaterialUrl(url), { method: "HEAD" });
      if (response.ok) {
        return;
      }
    } catch {
      // retry
    }
    if (attempt === 30) {
      throw new MinioError(`Material not ready: ${url}`);
    }
    await Bun.sleep(1_000);
  }
};

/** Calls a contract view through cast and interprets the result as a boolean. */
export const castBool = async (rpcUrl: string, to: string, signature: string, ...args: string[]) => {
  try {
    const result = await run(["cast", "call", to, signature, ...args, "--rpc-url", hostReachableRpcUrl(rpcUrl)]);
    const stdout = result.stdout.trim();
    return stdout === "true" || stdout === "0x1" || stdout === "0x0000000000000000000000000000000000000000000000000000000000000001";
  } catch (error) {
    throw new RpcError(rpcUrl, error instanceof Error ? error.message : String(error));
  }
};

/** Probes whether bootstrap produced stable key ids and published materials. */
export const probeBootstrap = async (state: State) => {
  const discovery = state.discovery!;
  const keyPrefix = discovery.minioKeyPrefix ?? "PUB";
  try {
    const defaultHostKey = defaultHostChainKey(state.scenario.hostChains);
    const useHostKms = requiresModernHostAddressArtifacts(state);
    const rawRpcUrl = useHostKms
      ? discovery.endpoints.hosts[defaultHostKey]?.http
      : discovery.endpoints.gateway.http;
    const contractAddress = useHostKms
      ? discovery.hosts[defaultHostKey]?.KMS_GENERATION_CONTRACT_ADDRESS
      : discovery.gateway.KMS_GENERATION_ADDRESS;
    if (!rawRpcUrl) {
      throw new PreflightError(
        useHostKms
          ? `Missing host RPC endpoint for chain "${defaultHostKey}" during bootstrap probe`
          : "Missing gateway RPC endpoint for bootstrap probe",
      );
    }
    const rpcUrl = hostReachableRpcUrl(rawRpcUrl);
    if (!contractAddress) {
      throw new PreflightError(
        useHostKms
          ? `Missing host KMS_GENERATION_CONTRACT_ADDRESS for chain "${defaultHostKey}" during bootstrap probe`
          : "Missing gateway KMS_GENERATION_ADDRESS for bootstrap probe",
      );
    }
    const ethCallRaw = async (data: string) => {
      const response = await fetch(rpcUrl, {
        method: "POST",
        headers: { "content-type": "application/json" },
        body: JSON.stringify({
          jsonrpc: "2.0",
          id: 1,
          method: "eth_call",
          params: [{ to: withHexPrefix(contractAddress), data }, "latest"],
        }),
      });
      if (!response.ok) return 0n;
      const payload = (await response.json()) as { result?: string };
      if (!payload.result) {
        return 0n;
      }
      try {
        return BigInt(payload.result);
      } catch {
        throw new RpcError(rpcUrl, `eth_call returned malformed bigint result: ${payload.result}`);
      }
    };
    const actualKey = await ethCallRaw(KEYGEN_ID_SELECTOR);
    const actualCrs = await ethCallRaw(CRSGEN_ID_SELECTOR);
    if (actualKey === 0n || actualCrs === 0n) {
      return null;
    }
    const actualFheKeyId = actualKey.toString(16).padStart(64, "0");
    const actualCrsKeyId = actualCrs.toString(16).padStart(64, "0");
    await Promise.all([
      ensureMaterial(`${discovery.endpoints.minioExternal}/kms-public/${keyPrefix}/PublicKey/${actualFheKeyId}`),
      ensureMaterial(`${discovery.endpoints.minioExternal}/kms-public/${keyPrefix}/CRS/${actualCrsKeyId}`),
    ]);
    if (discovery.fheKeyId !== actualFheKeyId || discovery.crsKeyId !== actualCrsKeyId) {
      throw new PreflightError(
        `Predicted bootstrap ids drifted: expected ${discovery.fheKeyId}/${discovery.crsKeyId}, got ${actualFheKeyId}/${actualCrsKeyId}`,
      );
    }
    return { actualFheKeyId, actualCrsKeyId };
  } catch (error) {
    if (error instanceof MinioError || error instanceof PreflightError) {
      throw error;
    }
    console.log(`[warn] bootstrap probe error (will retry): ${error instanceof Error ? error.message : String(error)}`);
    return null;
  }
};

/** Waits until bootstrap materials are fully published and discoverable. */
export const waitForBootstrap = async (state: State, attempts = 120) => {
  for (let attempt = 0; attempt < attempts; attempt += 1) {
    const result = await probeBootstrap(state);
    if (result) {
      state.discovery!.actualFheKeyId = result.actualFheKeyId;
      state.discovery!.actualCrsKeyId = result.actualCrsKeyId;
      return result;
    }
    if (attempt < attempts - 1) {
      if (attempt === 0 || (attempt + 1) % 5 === 0) {
        console.log(`[wait] bootstrap materials (${(attempt + 1) * 2}s elapsed)`);
      }
      await Bun.sleep(2_000);
    }
  }
  throw new BootstrapTimeout(attempts * 2);
};

/** Waits for the kms-connector runtime services to become ready. */
export const waitForKmsConnector = async () => {
  await waitForContainer("kms-connector-db-migration", "complete");
  await waitForContainer("kms-connector-gw-listener", "running");
  await waitForContainer("kms-connector-kms-worker", "running");
  await waitForContainer("kms-connector-tx-sender", "running");
  await waitForLog("kms-connector-gw-listener", /Started Decryption polling from block/);
  await waitForLog("kms-connector-gw-listener", /Started KMSGeneration polling from block/);
};

/** Waits for the e2e test-suite container to reach running state. */
export const waitForTestSuite = async () => waitForContainer(TEST_SUITE_CONTAINER, "running");
