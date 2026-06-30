import { bootstrapUsesHostKmsGeneration, kmsConnectorUsesHostKmsGeneration, supportsHostListenerConsumer } from "../compat/compat";
import { BootstrapTimeout, ContainerCrashed, MinioError, PreflightError, ProbeTimeout, RpcError } from "../errors";
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
import { kmsConnectorPrefix, kmsPublicPrefix } from "../kms-party";
import { topologyForState } from "../stack-spec/stack-spec";
import type { HostChainType, State } from "../types";
import { hostReachableMaterialUrl, hostReachableRpcUrl, predictedCrsId, predictedKeyId, toServiceName, withHexPrefix } from "../utils/fs";
import { run } from "../utils/process";

const POST_BOOT_HEALTH_GATE_DELAY_MS = 5_000;
const KMS_CONNECTOR_DECRYPTION_READY =
  /Started Decryption polling from block|Last block polled updated for \d+\/\d+ event types in \[PublicDecryptionRequest, UserDecryptionRequest\]/;
const KMS_CONNECTOR_KMS_GENERATION_READY =
  /Started KMSGeneration polling from block|Started Ethereum polling from block|Last block polled updated for chain ethereum|Last block polled updated for \d+\/\d+ event types in \[[^\]]*PrepKeygenRequest[^\]]*\]/;

/** Number of KMS connector instances: one per party in threshold mode, else one. */
// `kms.parties` is the canonical connector/party count: 1 for centralized, N for threshold.
const kmsConnectorPartyCount = (state: State) => state.scenario.kms.parties;

/** gw-listener / kms-worker / tx-sender health containers across every KMS party. */
export const kmsConnectorHealthContainers = (state: State): string[] => {
  const containers: string[] = [];
  for (let party = 1; party <= kmsConnectorPartyCount(state); party += 1) {
    const prefix = kmsConnectorPrefix(party);
    containers.push(`${prefix}-gw-listener`, `${prefix}-kms-worker`, `${prefix}-tx-sender`);
  }
  return containers;
};

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

/**
 * Waits until a host RPC endpoint answers a basic liveness request. EVM hosts are probed with
 * `eth_chainId` (expects a hex string); Solana hosts with `getHealth` (expects `result === "ok"`).
 */
export const waitForRpc = async (url: string, kind: HostChainType = "evm") => {
  const method = kind === "solana" ? "getHealth" : "eth_chainId";
  for (let attempt = 0; attempt <= 60; attempt += 1) {
    try {
      const response = await fetch(url, {
        method: "POST",
        headers: { "content-type": "application/json" },
        body: JSON.stringify({ jsonrpc: "2.0", id: 1, method, params: [] }),
      });
      if (response.ok) {
        const body = await response.json().catch(() => null) as
          | { jsonrpc?: string; result?: unknown; error?: unknown }
          | null;
        const ready =
          kind === "solana"
            ? body?.result === "ok"
            : body?.jsonrpc === "2.0" && typeof body.result === "string" && !body.error;
        if (ready) {
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

/** MinIO prefixes that hold a party's VerfAddress. Centralized stores it under
 * `PUB/PUB` (or legacy `PUB`); a threshold-mode cluster stores party i under its own prefix. */
const verfAddressPrefixes = (parties: number, party: number): string[] =>
  parties === 1 ? ["PUB/PUB", "PUB"] : [kmsPublicPrefix(party)];

/** Reads a single party's VerfAddress for `handle`, trying each candidate prefix. */
const fetchVerfAddress = async (
  prefixes: string[],
  handle: string,
): Promise<{ address: string; prefix: string } | null> => {
  for (const prefix of prefixes) {
    try {
      const response = await fetch(`${MINIO_EXTERNAL_URL}/kms-public/${prefix}/VerfAddress/${handle}`);
      if (response.ok) {
        return { address: (await response.text()).trim(), prefix };
      }
    } catch {
      // try the next prefix / retry the whole discovery
    }
  }
  return null;
};

/** Reads a single party's serialized CA certificate (PEM) for `handle` under `prefix`, returning
 * it hex-encoded as `0x…`. Best-effort: returns null when the prefix has no CACert (e.g. a build
 * that ships no TLS material), so discovery can fall back to an empty `0x` cert. */
const fetchCaCert = async (prefix: string, handle: string): Promise<string> => {
  try {
    const response = await fetch(`${MINIO_EXTERNAL_URL}/kms-public/${prefix}/CACert/${handle}`);
    if (response.ok) {
      return `0x${Buffer.from(await response.arrayBuffer()).toString("hex")}`;
    }
  } catch {
    // treat as "no cert available"
    console.warn(`No CACert available for handle "${handle}" under prefix "${prefix}". Falling back to "0x"`)
  }
  return "0x";
};

/**
 * Discovers the KMS signer addresses after bootstrap: one for a centralized node,
 * one per party for a threshold-mode cluster (`parties` is 1 in the centralized case).
 * The signing-key handle is scraped from the core logs and is shared across parties;
 * each party's address lives at its own MinIO prefix.
 */
export const discoverKmsSigners = async (
  parties: number,
): Promise<{ signers: string[]; caCerts: string[]; minioKeyPrefix: string }> => {
  let lastFailure = "no signing-key handle in the kms-core logs yet";
  for (let attempt = 0; attempt <= 60; attempt += 1) {
    const logs = await run(["docker", "logs", KMS_CORE_CONTAINER], { allowFailure: true });
    const text = `${logs.stdout}\n${logs.stderr}`;
    const handle = (text.match(/SigningKey\/([a-f0-9]{64})/) ?? text.match(/handle ([a-zA-Z0-9]+)/))?.[1];
    if (handle) {
      const signers: string[] = [];
      const caCerts: string[] = [];
      let minioKeyPrefix = "";
      for (let party = 1; party <= parties; party += 1) {
        const prefixes = verfAddressPrefixes(parties, party);
        const found = await fetchVerfAddress(prefixes, handle);
        if (!found) {
          lastFailure = `party ${party}: no VerfAddress/${handle} under ${prefixes.join(" or ")}`;
          break;
        }
        signers.push(found.address);
        // The CA cert lives alongside the VerfAddress under the same prefix. Best-effort: an empty
        // `0x` when a build ships no TLS material, so non-TLS stacks still resolve a signer set.
        caCerts.push(await fetchCaCert(found.prefix, handle));
        if (party === 1) {
          minioKeyPrefix = found.prefix;
        }
      }
      if (signers.length === parties) {
        return { signers, caCerts, minioKeyPrefix };
      }
    }
    await Bun.sleep(1_000);
  }
  throw new MinioError(`Could not discover ${parties} KMS signer(s) after 60 attempts (${lastFailure})`);
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

/** Calls a contract view through cast and returns its decoded stdout (per the signature's return type). */
export const castCall = async (rpcUrl: string, to: string, signature: string, ...args: string[]) => {
  const result = await run(["cast", "call", to, signature, ...args, "--rpc-url", hostReachableRpcUrl(rpcUrl)]);
  return result.stdout.trim();
};

/**
 * Resolves the chain the KMSGeneration contract is deployed on (host on v0.13+, else gateway) and the
 * contract addresses on it. Throws PreflightError when a required endpoint/address is missing;
 * `configAddress` (ProtocolConfig / GatewayConfig) is optional on pre-v0.13 bundles.
 */
export const resolveKmsGenerationTarget = (state: State) => {
  const discovery = state.discovery!;
  const useHostKms = bootstrapUsesHostKmsGeneration(state);
  const defaultHostKey = defaultHostChainKey(state.scenario.hostChains);
  const where = useHostKms ? `host chain "${defaultHostKey}"` : "gateway";
  const rawRpcUrl = useHostKms ? discovery.endpoints.hosts[defaultHostKey]?.http : discovery.endpoints.gateway.http;
  if (!rawRpcUrl) {
    throw new PreflightError(`Missing ${where} RPC endpoint for the KMSGeneration probe`);
  }
  const kmsGenerationAddress = useHostKms
    ? discovery.hosts[defaultHostKey]?.KMS_GENERATION_CONTRACT_ADDRESS
    : discovery.gateway.KMS_GENERATION_ADDRESS;
  if (!kmsGenerationAddress) {
    throw new PreflightError(`Missing ${where} KMSGeneration contract address for the KMSGeneration probe`);
  }
  const configAddress = useHostKms
    ? discovery.hosts[defaultHostKey]?.PROTOCOL_CONFIG_CONTRACT_ADDRESS
    : discovery.gateway.GATEWAY_CONFIG_ADDRESS;
  return {
    rpcUrl: hostReachableRpcUrl(rawRpcUrl),
    kmsGenerationAddress: withHexPrefix(kmsGenerationAddress),
    configAddress: configAddress ? withHexPrefix(configAddress) : undefined,
    where,
  };
};

/** Probes whether bootstrap produced stable key ids and published materials. */
export const probeBootstrap = async (state: State) => {
  const discovery = state.discovery!;
  const keyPrefix = discovery.minioKeyPrefix ?? "PUB";
  try {
    const { rpcUrl, kmsGenerationAddress } = resolveKmsGenerationTarget(state);
    const ethCallRaw = async (data: string) => {
      const response = await fetch(rpcUrl, {
        method: "POST",
        headers: { "content-type": "application/json" },
        body: JSON.stringify({
          jsonrpc: "2.0",
          id: 1,
          method: "eth_call",
          params: [{ to: kmsGenerationAddress, data }, "latest"],
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
export const waitForKmsConnector = async (state: State) => {
  const usesHostKmsGeneration = kmsConnectorUsesHostKmsGeneration(state);
  // Threshold runs one connector per party; every party must be ready or the
  // on-chain 2t+1 quorum can never be reached. Centralized = a single party.
  for (let party = 1; party <= kmsConnectorPartyCount(state); party += 1) {
    const prefix = kmsConnectorPrefix(party);
    await waitForContainer(`${prefix}-db-migration`, "complete");
    await waitForContainer(`${prefix}-gw-listener`, "running");
    await waitForContainer(`${prefix}-kms-worker`, "running");
    await waitForContainer(`${prefix}-tx-sender`, "running");
    if (usesHostKmsGeneration) {
      await waitForLog(`${prefix}-gw-listener`, KMS_CONNECTOR_DECRYPTION_READY);
      await waitForLog(`${prefix}-gw-listener`, KMS_CONNECTOR_KMS_GENERATION_READY);
    }
  }
};

/** Waits for the e2e test-suite container to reach running state. */
export const waitForTestSuite = async () => waitForContainer(TEST_SUITE_CONTAINER, "running");
