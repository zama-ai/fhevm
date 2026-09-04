import {
  bootstrapUsesHostKmsGeneration,
  kmsConnectorUsesHostKmsGeneration,
  supportsConsensusDetector,
  supportsHostListenerConsumer,
  supportsUpgradeController,
} from "../compat/compat";
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
  coprocessorDatabaseName,
  defaultHostChainKey,
  hostChainSuffix,
} from "../layout";
import { blueGreenServiceNames } from "../generate/compose";
import { kmsConnectorPrefix, kmsPublicPrefix } from "../kms-party";
import { topologyForState } from "../stack-spec/stack-spec";
import type { State } from "../types";
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
  if (state.scenario.kind === "blue-green") {
    return blueGreenServiceNames(state, { includeMigration: false });
  }
  const suffixes = GROUP_SERVICE_SUFFIXES.coprocessor.filter(
    (suffix) =>
      !suffix.includes("migration") &&
      (suffix !== "host-listener-consumer" || supportsHostListenerConsumer(state)) &&
      (suffix !== "consensus-detector" || supportsConsensusDetector(state)) &&
      (suffix !== "upgrade-controller" || supportsUpgradeController(state)),
  );
  const topology = topologyForState(state);
  const names: string[] = [];
  for (let index = 0; index < topology.count; index += 1) {
    for (const suffix of suffixes) {
      names.push(toServiceName(suffix, index));
    }
  }
  return names;
};

/**
 * Worker roles that claim rows from a per-operator queue, with the process name
 * each one runs under.
 */
const QUEUE_WORKERS = [
  { service: "tfhe-worker", process: "tfhe_worker" },
  { service: "zkproof-worker", process: "zkproof_worker" },
  { service: "sns-worker", process: "sns_worker" },
] as const;

/** PIDs of the running containers for one worker role, across all operators. */
const containerWorkerPids = async (service: string, count: number) => {
  const pids = new Set<number>();
  for (let index = 0; index < count; index += 1) {
    const inspected = await run(
      ["docker", "inspect", "-f", "{{.State.Pid}}", toServiceName(service, index)],
      { allowFailure: true },
    );
    const pid = Number.parseInt(inspected.stdout.trim(), 10);
    if (Number.isInteger(pid) && pid > 0) pids.add(pid);
  }
  return pids;
};

/** PIDs of every process on this host running under `name`. */
const hostProcessPids = async (name: string) => {
  const found = await run(["pgrep", "-x", name], { allowFailure: true });
  return found.stdout
    .split("\n")
    .map((line) => Number.parseInt(line.trim(), 10))
    .filter((pid) => Number.isInteger(pid) && pid > 0);
};

/** Host PIDs for a role that no container of this stack accounts for. */
export const strayWorkerPids = (owned: ReadonlySet<number>, hostPids: readonly number[]) =>
  hostPids.filter((pid) => !owned.has(pid));

/**
 * Reads `gpu_enabled` out of an sns-worker's startup output.
 *
 * The worker logs JSON (`"gpu_enabled":true`), but accept a bare `=` form too
 * rather than have the guard go quiet on a formatting change -- a check that
 * silently stops checking is worse than no check.
 */
export const parseSquashBackend = (logs: string) => {
  const match = /gpu_enabled"?\s*[:=]\s*"?(true|false)/i.exec(logs);
  return match ? (match[1].toLowerCase() as "true" | "false") : undefined;
};

/**
 * Groups operators by squash backend. Operators whose backend could not be
 * read are left out: an older image without the startup line is not evidence
 * of a split.
 */
export const backendSplit = (backends: readonly (string | undefined)[]) => {
  const grouped = new Map<string, number[]>();
  backends.forEach((backend, index) => {
    if (!backend) return;
    grouped.set(backend, [...(grouped.get(backend) ?? []), index]);
  });
  return grouped;
};

/**
 * Refuses to call the stack ready while a second process is serving any
 * operator's work queue.
 *
 * Every worker role here claims rows with `FOR UPDATE SKIP LOCKED`, which is
 * right for one worker and silently wrong for two: each row is served by
 * whichever process won it, so two *different builds* on one database split the
 * queue between them. That is not a liveness problem -- both are healthy, both
 * make progress, and every other check here passes -- and the only symptom is
 * bytes that disagree for no visible reason. A CPU container racing a CUDA host
 * worker produces exactly that: one operator holding a mix of CPU-squashed and
 * GPU-squashed ct128 for handles whose ct64 is identical, which reads as a
 * consensus defect and costs an investigation to tell apart from one
 * (Consensus Defect Log, B-1/L-6). `gpu-consensus-workers.sh` displaces the
 * containers deliberately and restores them on `stop`, but its units are
 * transient with `Restart=on-failure`, so they outlive a teardown and the next
 * `up` brings the containers back underneath them.
 *
 * This compares host process IDs against the container PIDs, so it only sees
 * what shares this PID namespace. With a remote Docker daemon `pgrep` finds
 * nothing and the check passes vacuously -- it is a guard against the local
 * footgun, not a proof of exclusivity.
 */
export const assertOneWorkerPerQueue = async (state: Pick<State, "scenario">) => {
  const count = topologyForState(state).count;
  const strays: string[] = [];
  for (const worker of QUEUE_WORKERS) {
    const owned = await containerWorkerPids(worker.service, count);
    if (owned.size === 0) continue; // role not deployed in this scenario
    const hostPids = await hostProcessPids(worker.process);
    for (const pid of strayWorkerPids(owned, hostPids)) {
      const exe = await run(["readlink", "-f", `/proc/${pid}/exe`], { allowFailure: true });
      strays.push(`${worker.process} pid=${pid} ${exe.stdout.trim() || "(unknown binary)"}`);
    }
  }
  if (strays.length > 0) {
    throw new PreflightError(
      [
        "A worker outside this stack is serving the same operator queues:",
        ...strays.map((stray) => `  ${stray}`),
        "",
        "Two workers on one queue split it between two builds, which corrupts",
        "byte-consensus without failing any health check. Run",
        "`test-suite/fhevm/scripts/gpu-consensus-workers.sh stop` to hand the",
        "queues back to the containers, or stop the containers if the host",
        "workers are the ones you want.",
      ].join("\n"),
    );
  }
};

/**
 * Refuses to call a multi-operator stack ready unless every operator squashes
 * on the same backend.
 *
 * Consensus is a byte comparison and the CPU and CUDA squash paths produce
 * different (both correct) ct128 for the same ct64, so a fleet split across
 * backends cannot reach quorum on the SNS digest while agreeing on everything
 * else. Reading it from the worker's own startup line is cheap; discovering it
 * from divergent bytes is not.
 */
export const assertUniformSquashBackend = async (state: Pick<State, "scenario">) => {
  const count = topologyForState(state).count;
  if (count < 2) return;
  const observed: (string | undefined)[] = [];
  for (let index = 0; index < count; index += 1) {
    const logs = await run(["docker", "logs", toServiceName("sns-worker", index)], {
      allowFailure: true,
    });
    observed.push(parseSquashBackend(`${logs.stdout}${logs.stderr}`));
  }
  const backends = backendSplit(observed);
  if (backends.size > 1) {
    const split = [...backends.entries()]
      .map(([backend, operators]) => `gpu_enabled=${backend}: operator ${operators.join(", ")}`)
      .join("; ");
    throw new PreflightError(
      `Operators are split across squash backends (${split}). CPU and GPU squashing ` +
        "produce different ct128 bytes for the same input, so this fleet cannot reach " +
        "byte-consensus on the SNS digest. Make every operator use the same backend.",
    );
  }
};

/** Waits for all coprocessor runtime services to reach their expected states. */
export const waitForCoprocessorServices = async (state: State, skipMigration: boolean) => {
  const waitCoreFleet = async (prefix: string, withMigration: boolean) => {
    if (withMigration && !skipMigration) {
      await waitForContainer(`${prefix}db-migration`, "complete");
    }
    await waitForContainer(`${prefix}host-listener`, "running");
    await waitForContainer(`${prefix}host-listener-poller`, "running");
    if (supportsHostListenerConsumer(state)) {
      await waitForContainer(`${prefix}host-listener-consumer`, "running");
    }
    await waitForContainer(`${prefix}gw-listener`, "running");
    await waitForContainer(`${prefix}tfhe-worker`, "running");
    await waitForContainer(`${prefix}zkproof-worker`, "running");
    await waitForContainer(`${prefix}sns-worker`, "running");
    await waitForContainer(`${prefix}transaction-sender`, "running");
  };
  const count = topologyForState(state).count;
  for (let index = 0; index < count; index += 1) {
    const prefix = index === 0 ? "coprocessor-" : `coprocessor${index}-`;
    await waitCoreFleet(prefix, true);
    if (state.scenario.kind === "blue-green") {
      await waitCoreFleet(`${prefix}gcs-`, false);
      await waitForContainer(`${prefix}gcs-upgrade-controller`, "running");
      await waitForContainer(`${prefix}gcs-consensus-detector`, "running");
    }
  }
  // Both of these are about who is allowed to serve the queues, so they belong
  // after the containers are up and before anything calls the stack ready.
  await assertOneWorkerPerQueue(state);
  await assertUniformSquashBackend(state);
};

/** Waits for the full coprocessor stack, including migrations, to become ready. */
export const waitForCoprocessor = async (state: State) => waitForCoprocessorServices(state, false);

/** Waits for db-migration containers (one per operator) to exit successfully. */
export const waitForCoprocessorDbMigrations = async (state: Pick<State, "scenario" | "versions">) => {
  const count = topologyForState(state).count;
  for (let index = 0; index < count; index += 1) {
    await waitForContainer(toServiceName("db-migration", index), "complete");
  }
};

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

/**
 * Waits until one of the supplied material artifacts is available through
 * host-reachable MinIO.  Probing alternatives together preserves the normal
 * bootstrap retry budget when a bundle supports more than one wire format.
 */
export const ensureOneMaterial = async (urls: readonly string[]) => {
  if (!urls.length) {
    throw new PreflightError("At least one material URL is required");
  }
  for (let attempt = 0; attempt <= 30; attempt += 1) {
    const available = await Promise.all(
      urls.map(async (url) => {
        try {
          return (await fetch(hostReachableMaterialUrl(url), { method: "HEAD" })).ok;
        } catch {
          return false;
        }
      }),
    );
    if (available.some(Boolean)) {
      return;
    }
    if (attempt === 30) {
      throw new MinioError(`Material not ready: ${urls.join(" or ")}`);
    }
    await Bun.sleep(1_000);
  }
};

/** Waits until one material artifact becomes available through host-reachable MinIO. */
export const ensureMaterial = async (url: string) => ensureOneMaterial([url]);

/**
 * Waits for the server-side part of an FHE key activation.  Current kms-core
 * publishes that material as a `CompressedXofKeySet`; older bundles publish a
 * plain `ServerKey`.  PublicKey and CRS availability alone is not sufficient:
 * a coprocessor cannot materialize `keys` or start a TFHE worker without one
 * of these authenticated server-side blobs.
 */
const ensureServerKeyMaterial = async (baseUrl: string, keyPrefix: string, keyId: string) => {
  const root = `${baseUrl}/kms-public/${keyPrefix}`;
  const candidates = [
    `${root}/CompressedXofKeySet/${keyId}`,
    `${root}/ServerKey/${keyId}`,
  ];
  await ensureOneMaterial(candidates);
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
      ensureServerKeyMaterial(discovery.endpoints.minioExternal, keyPrefix, actualFheKeyId),
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

/**
 * Waits until every coprocessor database holds the ingested keyset.
 *
 * `waitForBootstrap` proves the KMS produced key material and the gateway
 * recorded it; it says nothing about whether each coprocessor has ingested it.
 * The insert is large (~444 MB compressed) and lands seconds to minutes later,
 * so `up` used to report the stack ready while operators still held no key rows
 * — and anything that trusted that signal raced the ingest. A suite launched on
 * the readiness signal failed in two seconds with "operator 2 holds no key
 * rows"; every driver since has carried its own wait, which is a workaround for
 * a readiness signal that overstates readiness (Consensus Defect Log, F-1).
 *
 * The fork topology is the deliberate exception: the operator following
 * `fork-anvil` cannot ingest keys until a suite seeds that chain from a
 * post-keygen canonical tip, so only the operators on the canonical chain are
 * required.
 */
export const waitForCoprocessorKeyMaterial = async (state: State, attempts = 150) => {
  const count = topologyForState(state).count;
  // Narrowed the same way `scenarioUsesForkAnvil` does: only a consensus
  // scenario has instances, and the fork is identified by the hostname an
  // instance is pointed at rather than by a flag.
  const forkOperators = new Set<number>();
  if (state.scenario.kind === "coprocessor-consensus") {
    state.scenario.instances.forEach((instance, position) => {
      const onFork = [instance.env.RPC_HTTP_URL, instance.env.RPC_WS_URL].some((value) => {
        if (!value) return false;
        try {
          return new URL(String(value)).hostname === "fork-anvil";
        } catch {
          return false;
        }
      });
      if (onFork) forkOperators.add(instance.index ?? position);
    });
  }
  const required = Array.from({ length: count }, (_, index) => index).filter(
    (index) => !forkOperators.has(index),
  );
  if (forkOperators.size > 0) {
    console.log(
      `[wait] key material: operator(s) ${[...forkOperators].join(", ")} follow fork-anvil and ` +
        "cannot ingest until a suite seeds that chain, so they are not required here",
    );
  }
  for (let attempt = 0; attempt < attempts; attempt += 1) {
    const pending: number[] = [];
    for (const index of required) {
      const result = await run(
        [
          "docker",
          "exec",
          COPROCESSOR_DB_CONTAINER,
          "psql",
          "-U",
          "postgres",
          "-d",
          coprocessorDatabaseName(index),
          "-tAc",
          "SELECT count(compressed_xof_keyset) FROM keys",
        ],
        { allowFailure: true },
      );
      if (Number.parseInt((result.stdout ?? "").trim(), 10) < 1) pending.push(index);
    }
    if (pending.length === 0) {
      console.log(`[wait] key material ingested on operator(s) ${required.join(", ")}`);
      return;
    }
    if (attempt === 0 || (attempt + 1) % 10 === 0) {
      console.log(`[wait] key material on operator(s) ${pending.join(", ")} (${(attempt + 1) * 2}s elapsed)`);
    }
    await Bun.sleep(2_000);
  }
  throw new Error(
    `coprocessor key material never landed on operator(s) ${required.join(", ")} within ${attempts * 2}s; ` +
      "the stack is not usable and a suite started now would fail its own key-material gate",
  );
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
