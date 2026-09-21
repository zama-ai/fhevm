import { existsSync } from "node:fs";
import path from "node:path";
import { PreflightError } from "../errors";
import { COPROCESSOR_DB_CONTAINER, RUNTIME_DIR, coprocessorDatabaseName } from "../layout";
import { topologyForState } from "../stack-spec/stack-spec";
import type { State } from "../types";
import { run } from "../utils/process";

const WORKERS = ["tfhe", "sns", "zkproof"] as const;
const OTHER_WRITER_BINARIES = ["host_listener", "host_listener_poller", "host_listener_consumer", "gw_listener", "transaction_sender", "consensus-detector", "upgrade-controller"];
type Worker = typeof WORKERS[number];
type Container = {
  Name: string;
  Path?: string;
  Args?: string[];
  State: { Pid: number; Status?: string; Paused?: boolean };
  Config: { Cmd?: string[]; Entrypoint?: string[]; Env?: string[]; ExposedPorts?: Record<string, unknown> };
  HostConfig?: {ExtraHosts?: string[]};
  NetworkSettings: { Networks: Record<string, { NetworkID: string; IPAddress?: string; GlobalIPv6Address?: string; Aliases?: string[] }> };
};

export const strayWorkerPids = (owned: ReadonlySet<number>, hostPids: readonly number[]) =>
  hostPids.filter((pid) => !owned.has(pid));

const parseWorkerEndpoint = (commandLine: string, environment: string): {host: string; port: string; database: string} | undefined => {
  const url = /--database-url[=\s]+(\S+)/.exec(commandLine)?.[1] ?? /^DATABASE_URL=(.+)$/m.exec(environment)?.[1];
  if (!url) return undefined;
  try {
    const parsed = new URL(url);
    let host = decodeURIComponent(parsed.hostname).replace(/^\[|\]$/g, "");
    let port = parsed.port || "5432";
    let database = decodeURIComponent(parsed.pathname.replace(/^\//, ""));
    // Match SQLx PgConnectOptions: query parameters override authority/path,
    // and later duplicate parameters win. Otherwise an innocuous pathname can
    // hide a worker that actually connects to this stack's database.
    for (const [key, value] of parsed.searchParams) {
      if (key === "host") host = value;
      if (key === "port") port = value;
      if (key === "dbname") database = value;
    }
    if (!/^\d+$/.test(port) || Number(port) > 65535) return undefined;
    return {host, port: String(Number(port)), database};
  } catch { return undefined; }
};
export const parseWorkerDatabase = (commandLine: string, environment: string): string | undefined => {
  return parseWorkerEndpoint(commandLine, environment)?.database || undefined;
};
export type StrayVerdict =
  | { pid: number; kind: "conflict"; database: string }
  | { pid: number; kind: "unrelated"; database: string }
  | { pid: number; kind: "unknown" };
export const classifyStray = (pid: number, database: string | undefined, ownedDatabases: ReadonlySet<string>): StrayVerdict =>
  !database ? { pid, kind: "unknown" } : { pid, kind: ownedDatabases.has(database) ? "conflict" : "unrelated", database };

const command = (container: Container) => [container.Path ?? "", ...(container.Args ?? []),
  ...(container.Config.Entrypoint ?? []), ...(container.Config.Cmd ?? [])].join(" ");
const database = (container: Container) => parseWorkerDatabase(command(container), (container.Config.Env ?? []).join("\n"));
const networks = (container: Container) => Object.values(container.NetworkSettings.Networks).map((network) => network.NetworkID);
const role = (container: Container): Worker | undefined => WORKERS.find((kind) =>
  new RegExp(`(?:^|[/\\s])${kind}_worker(?:$|\\s)`).test(command(container)) ||
  container.Name.endsWith(`-${kind}-worker`));
const isOtherWriter = (container: Container) => OTHER_WRITER_BINARIES.some((binary) =>
  new RegExp(`(?:^|[/\\s])${binary}(?:$|\\s)`).test(command(container)));
const prefix = (index: number) => index === 0 ? "coprocessor" : `coprocessor${index}`;

// A different bridge can still reach this stack through its published DB port
// (and host-network containers use localhost directly). Only exempt an endpoint
// whose name/address actually identifies another DB container on that bridge.
const targetsIsolatedDatabase = (worker: Container, containers: Container[], stackNetworks: Set<string>) => {
  const endpoint = parseWorkerEndpoint(command(worker), (worker.Config.Env ?? []).join("\n"));
  if (!endpoint) return false;
  const workerNetworks = new Set(networks(worker));
  const {host, port} = endpoint;
  // Docker's --add-host entries take precedence over network DNS aliases. An
  // alias that normally resolves to an isolated DB can point at the host's
  // published stack DB instead; metadata for that alias no longer proves it.
  if (worker.HostConfig?.ExtraHosts?.some(entry => entry.split(/[:=]/, 1)[0].toLowerCase() === host.toLowerCase())) return false;
  return containers.some((candidate) =>
    candidate.Config.ExposedPorts?.[`${port}/tcp`] !== undefined &&
    !networks(candidate).some((network) => stackNetworks.has(network)) &&
    Object.values(candidate.NetworkSettings.Networks).some((network) =>
      workerNetworks.has(network.NetworkID) &&
      [candidate.Name.replace(/^\//, ""), network.IPAddress, network.GlobalIPv6Address, ...(network.Aliases ?? [])].includes(host),
    ),
  );
};

/**
 * Discover queue owners through Docker metadata and host process identities.
 * Blue/green standby containers are explicitly owned by that scenario: their
 * compiled stack-version and activation gates intentionally share a database.
 * GPU ownership must be opted into, so ordinary `up` still rejects host units.
 */
class OwnerSnapshotChanged extends Error {}

const inspectQueueOwnership = async (
  state: Pick<State, "scenario">,
  options: { allowGpu?: boolean; requireEveryRole?: boolean; managedWriterContainers?: readonly string[]; requireQuiescent?: boolean } = {},
) => {
  const count = topologyForState(state).count;
  let containers: Container[] | undefined;
  for (let attempt = 0; attempt < 3; attempt++) {
    const listed = await run(["docker", "ps", "-q"], { allowFailure: true, timeoutMs: 15_000 });
    if (listed.code !== 0) throw new PreflightError("Could not enumerate running Docker containers");
    const ids = listed.stdout.trim().split(/\s+/).filter(Boolean);
    const inspected = ids.length ? await run(["docker", "inspect", ...ids], { allowFailure: true, timeoutMs: 15_000 }) : { code: 0, stdout: "[]", stderr: "" };
    if (inspected.code === 0) { containers = JSON.parse(inspected.stdout) as Container[]; break; }
    // An unrelated short-lived container can disappear between enumeration
    // and inspect. Retry a complete snapshot; never trust a partial result.
    if (!/no such object|no such container/i.test(inspected.stderr)) break;
  }
  if (!containers) throw new PreflightError("Could not inspect Docker queue owners after bounded discovery retries");
  const byName = new Map(containers.map((container) => [container.Name.replace(/^\//, ""), container]));
  const byPid = new Map(containers.map((container) => [container.State.Pid, container]));
  const stackNetworks = new Set<string>();
  for (const container of containers) {
    if (container.Name.replace(/^\//, "") === COPROCESSOR_DB_CONTAINER ||
      WORKERS.some((kind) => Array.from({ length: count }, (_, index) => `${prefix(index)}-${kind}-worker`).includes(container.Name.replace(/^\//, "")))) {
      for (const network of networks(container)) stackNetworks.add(network);
    }
  }
  const ownedDatabases = new Set(Array.from({ length: count }, (_, index) => coprocessorDatabaseName(index)));
  const gpu = options.allowGpu && existsSync(path.join(RUNTIME_DIR, "gpu-consensus-workers", "node-config.env"));
  const owned = new Set<number>();
  const hostPids = new Map<string, number[]>();
  const failures: string[] = [];
  for (const kind of WORKERS) {
    const found = await run(["pgrep", "-x", `${kind}_worker`], { allowFailure: true, timeoutMs: 15_000 });
    if (found.code !== 0 && found.code !== 1) throw new PreflightError(`Cannot enumerate host ${kind} workers`);
    if (found.code === 1 && found.stderr.trim()) throw new PreflightError(`Cannot enumerate host ${kind} workers: ${found.stderr.trim()}`);
    hostPids.set(kind, found.stdout.trim().split(/\s+/).filter(Boolean).map(Number));
    for (let index = 0; index < count; index += 1) {
      const name = `${prefix(index)}-${kind}-worker`;
      const container = byName.get(name);
      const cpuRunning = container?.State.Status === "running" && container.State.Pid > 0 && !container.State.Paused;
      if (container) {
        owned.add(container.State.Pid);
        if (database(container) !== coprocessorDatabaseName(index)) failures.push(`${name}: Docker owner does not target its operator database`);
      }
      let gpuRunning = false;
      if (gpu) {
        const unit = `fhevm-gpu-consensus-${kind}-${index}`;
        const runtime = `/run/user/${process.getuid?.() ?? 0}`;
        const result = await run(["systemctl", "--user", "show", unit, "--property=ActiveState", "--property=MainPID"], {
          allowFailure: true, timeoutMs: 15_000,
          env: { XDG_RUNTIME_DIR: runtime, DBUS_SESSION_BUS_ADDRESS: `unix:path=${runtime}/bus` },
        });
        if (result.code !== 0) throw new PreflightError(`Cannot inspect GPU owner ${unit}`);
        const active = /^ActiveState=(.+)$/m.exec(result.stdout)?.[1];
        const pid = Number(/^MainPID=(\d+)$/m.exec(result.stdout)?.[1]);
        if (!active || !Number.isInteger(pid)) throw new PreflightError(`Unreadable GPU owner ${unit}`);
        if (options.requireQuiescent && !["inactive", "failed"].includes(active)) {
          failures.push(`${unit}: GPU writer is ${active}, not quiescent`);
        }
        gpuRunning = active === "active" && pid > 0;
        if (pid > 0) {
          const stat = await run(["cat", `/proc/${pid}/stat`], { allowFailure: true, timeoutMs: 15_000 });
          if (stat.code !== 0) throw new PreflightError(`Cannot verify GPU process ${unit}`);
          const processState = /\) ([A-Zt]) /.exec(stat.stdout)?.[1];
          if (!processState) throw new PreflightError(`Unreadable GPU process state for ${unit}`);
          gpuRunning &&= !["T", "t", "Z", "X"].includes(processState);
          const cmdline = await run(["cat", `/proc/${pid}/cmdline`], { allowFailure: true, timeoutMs: 15_000 });
          const environ = await run(["cat", `/proc/${pid}/environ`], { allowFailure: true, timeoutMs: 15_000 });
          if (parseWorkerDatabase(cmdline.stdout.replaceAll("\0", " "), environ.stdout.replaceAll("\0", "\n")) !== coprocessorDatabaseName(index)) {
            failures.push(`${unit}: host owner does not target its operator database`);
          }
          owned.add(pid);
        }
      }
      const green = state.scenario.kind === "blue-green" ? byName.get(`${prefix(index)}-gcs-${kind}-worker`) : undefined;
      const greenRunning = green?.State.Status === "running" && green.State.Pid > 0 && !green.State.Paused;
      if (green) {
        owned.add(green.State.Pid);
        if (database(green) !== coprocessorDatabaseName(index)) failures.push(`${green.Name}: GCS owner does not target its operator database`);
      }
      // Blue and green use separate schemas during activation; after cutover
      // the old fleet retires and the green names remain. Both phases belong
      // to the planned owner family, rather than being arbitrary duplicates.
      if (container && gpuRunning) failures.push(`${name}: both Docker and GPU unit serve its queue`);
      if (green && gpuRunning) failures.push(`${name}: both GCS Docker and GPU unit serve its queue`);
      if (gpu && cpuRunning) failures.push(`${name}: GPU session ownership is recorded but its Docker worker is running`);
      if (gpu && options.requireEveryRole && !gpuRunning) failures.push(`${name}: GPU session requires a running GPU owner`);
      if (options.requireEveryRole && !cpuRunning && !gpuRunning && !greenRunning) {
        failures.push(`${name}: expected one running owner, found none`);
      }
    }
  }
  // Destructive DB operations also fence listeners, senders and controllers.
  // These are explicitly managed names, never an exemption for arbitrary
  // processes merely because they use the same database.
  if (options.managedWriterContainers) {
    for (const name of options.managedWriterContainers) {
      const container = byName.get(name);
      if (!container) continue;
      const index = /^coprocessor(\d*)-/.exec(name)?.[1];
      if (index === undefined || database(container) !== coprocessorDatabaseName(Number(index || 0))) {
        failures.push(`${name}: managed writer does not target its operator database`);
      }
      owned.add(container.State.Pid);
    }
    // Linux comm is limited to 15 bytes, including long Rust binary names.
    // Attribute container children below; unreadable unmanaged host processes
    // remain a refusal rather than being assumed unrelated.
    for (const binary of new Set(OTHER_WRITER_BINARIES.map((name) => name.slice(0, 15)))) {
      const found = await run(["pgrep", "-x", binary], {allowFailure: true, timeoutMs: 15_000});
      if ((found.code !== 0 && found.code !== 1) || (found.code === 1 && found.stderr.trim())) {
        throw new PreflightError(`Cannot enumerate host writer ${binary}`);
      }
      hostPids.set(binary, found.stdout.trim().split(/\s+/).filter(Boolean).map(Number));
    }
  }
  if (options.requireQuiescent) {
    // No managed name or systemd unit is exempt once SQL requires zero writers.
    for (const container of containers) if (owned.has(container.State.Pid)) {
      failures.push(`${container.Name}: managed writer is still ${container.State.Status ?? "present"}`);
    }
    for (const pid of owned) if (pid > 0 && !byPid.has(pid)) failures.push(`managed host writer pid=${pid} is still live`);
    owned.clear();
  }
  // Worker entrypoints can have a PID-1 wrapper. Attribute their child PIDs
  // through Docker rather than trying to read root-owned /proc environments.
  const unaccounted = [...hostPids.values()].flat().filter((pid) => !byPid.has(pid) && !owned.has(pid));
  if (unaccounted.length) {
    for (const container of containers) {
      const top = await run(["docker", "top", container.Name.replace(/^\//, ""), "-eo", "pid,comm"], { allowFailure: true, timeoutMs: 15_000 });
      if (top.code !== 0) {
        if (/no such object|no such container/i.test(top.stderr)) throw new OwnerSnapshotChanged(container.Name);
        throw new PreflightError(`Cannot attribute container processes for ${container.Name}`);
      }
      for (const line of top.stdout.trim().split("\n").slice(1)) {
        const pid = Number(line.trim().split(/\s+/)[0]);
        if (!unaccounted.includes(pid)) continue;
        byPid.set(pid, container);
        if (owned.has(container.State.Pid)) owned.add(pid);
      }
    }
  }
  const candidates = new Set([...hostPids.values()].flat());
  for (const container of containers) if (role(container) || (options.managedWriterContainers && isOtherWriter(container))) candidates.add(container.State.Pid);
  for (const pid of candidates) {
    if (owned.has(pid)) continue;
    const container = byPid.get(pid);
    let target: string | undefined;
    if (container) {
      if (!stackNetworks.size) throw new PreflightError("Cannot identify the stack's Docker network for queue ownership");
      if (!networks(container).some((network) => stackNetworks.has(network)) && targetsIsolatedDatabase(container, containers, stackNetworks)) continue;
      target = database(container);
    } else {
      const cmdline = await run(["cat", `/proc/${pid}/cmdline`], { allowFailure: true, timeoutMs: 15_000 });
      const environ = await run(["cat", `/proc/${pid}/environ`], { allowFailure: true, timeoutMs: 15_000 });
      target = parseWorkerDatabase(cmdline.stdout.replaceAll("\0", " "), environ.stdout.replaceAll("\0", "\n"));
    }
    if (classifyStray(pid, target, ownedDatabases).kind !== "unrelated") {
      failures.push(`${container?.Name ?? "host worker"} pid=${pid} -> ${target ?? "unreadable database"}`);
    }
  }
  if (failures.length) throw new PreflightError(`Queue ownership could not be established:\n${failures.map((failure) => `  ${failure}`).join("\n")}`);
};

/** A vanished container during child-PID attribution invalidates the whole snapshot. */
export const assertOneWorkerPerQueue: typeof inspectQueueOwnership = async (state, options) => {
  for (let attempt = 0; attempt < 3; attempt++) {
    try { return await inspectQueueOwnership(state, options); }
    catch (error) {
      if (!(error instanceof OwnerSnapshotChanged)) throw error;
      if (attempt === 2) throw new PreflightError("Could not attribute Docker queue owners after bounded snapshot retries");
    }
  }
};
