import { createHash } from "node:crypto";
import { closeSync, openSync } from "node:fs";
import fs from "node:fs/promises";
import path from "node:path";

import {
  FHEVM_COMPOSE_PROJECT_ENV,
  PORTS,
  REPO_ROOT,
} from "../src/layout";
import { solanaImages } from "../src/solana/images";
import { run, runStreaming } from "../src/utils/process";
import {
  authorizeDemoHeaders,
  createDemoAuthorizationFile,
  DEMO_ALLOWED_ORIGIN_ENV,
  DEMO_AUTH_TOKEN_FILE_ENV,
  DEMO_AUTH_TOKEN_FILENAME,
  DEMO_BOOT_ID_ENV,
  readDemoAuthorizationFromEnv,
} from "./authorization";
import { DEMO_CONFIG_DEFAULT_PATH } from "./config";
import {
  requestSupervisorReseed,
  startSupervisorControl,
  type SupervisorReseedResult,
} from "./supervisorControl";

export const DEMO_RUNTIME_DIR = path.join(
  REPO_ROOT,
  ".fhevm",
  "runtime",
  "solana-demo",
);
export const DEMO_CONFIG_PATH = DEMO_CONFIG_DEFAULT_PATH;
export const DEMO_MANIFEST_PATH = path.join(DEMO_RUNTIME_DIR, "manifest.json");
export const DEMO_LOCK_PATH = path.join(DEMO_RUNTIME_DIR, "lifecycle.lock");
export const DEMO_COMPOSE_PROJECT = "fhevm";
const DEMO_COMPOSE_PROJECT_PREFIX = "fhevm-demo-";
const BOOT_ID_PATTERN =
  /^[0-9a-f]{8}-[0-9a-f]{4}-[1-8][0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/i;

export const demoComposeProject = (bootId: string): string => {
  if (!BOOT_ID_PATTERN.test(bootId)) {
    throw new Error("demo boot id must be a UUID");
  }
  return `${DEMO_COMPOSE_PROJECT_PREFIX}${bootId.toLowerCase()}`;
};

export const DEMO_REQUIRED_COMMANDS = [
  "anchor",
  "bash",
  "bun",
  "cargo",
  "cast",
  "cp",
  "curl",
  "cut",
  "dirname",
  "docker",
  "find",
  "git",
  "grep",
  "head",
  "kill",
  "lsof",
  "mkdir",
  "nohup",
  "node",
  "npm",
  "pgrep",
  "ps",
  "python3",
  "rm",
  "sed",
  "seq",
  "sleep",
  "solana",
  "solana-keygen",
  "solana-test-validator",
  "tail",
  "tar",
  "tr",
  "uname",
  "which",
];
const RESERVED_PORTS = [...new Set([...PORTS, 50051, 5173, 8090, 8899, 10000])];
const PROCESS_NAMES = ["validator", "listener", "faucet", "dapp"] as const;
const CORE_IMAGE = `ghcr.io/zama-ai/kms/core-service:${solanaImages.CORE_VERSION}`;
const REQUIRED_KEYPAIRS = [
  ...[
    "confidential_batcher",
    "confidential_token",
    "confidential_token_receiver",
    "demo_vault",
    "zama_host",
  ].map((name) =>
    path.join(
      REPO_ROOT,
      "solana",
      "scripts",
      "e2e",
      "test-keypairs",
      `${name}-keypair.json`,
    ),
  ),
  ...["alice", "bob", "keeper", "mint-authority"].map((name) =>
    path.join(
      REPO_ROOT,
      "solana",
      "scripts",
      "demo",
      "demo-keypairs",
      `${name}.json`,
    ),
  ),
];

type ProcessName = (typeof PROCESS_NAMES)[number];

export type OwnedProcess = {
  readonly pid: number;
  readonly identity: string;
  readonly command: readonly string[];
  readonly logPath: string;
};

export type DemoManifest = {
  readonly version: 3;
  readonly bootId: string;
  readonly repoRoot: string;
  readonly composeProject: string;
  readonly configPath: string;
  readonly createdAt: string;
  readonly state: "starting" | "running" | "failed" | "stopped";
  readonly containers: readonly {
    readonly id: string;
    readonly name: string;
  }[];
  readonly volumes: readonly {
    readonly id: string;
    readonly name: string;
  }[];
  readonly networks: readonly {
    readonly id: string;
    readonly name: string;
  }[];
  readonly processes: Partial<Record<ProcessName, OwnedProcess>>;
  readonly failure?: string;
};

type CollisionSnapshot = {
  readonly composeContainers: readonly {
    readonly id: string;
    readonly name: string;
  }[];
  readonly composeVolumes: readonly {
    readonly id: string;
    readonly name: string;
  }[];
  readonly composeNetworks: readonly {
    readonly id: string;
    readonly name: string;
  }[];
  readonly occupiedPorts: ReadonlyMap<number, string>;
  readonly persistedFhevmState: boolean;
  readonly requiredCommands: ReadonlyMap<string, boolean>;
};

export type DoctorEnvironmentSnapshot = {
  readonly docker:
    | {
        readonly cpus: number;
        readonly memoryBytes: number;
        readonly osType: string;
        readonly architecture: string;
      }
    | undefined;
  readonly dockerError?: string;
  readonly coreManifestArchitectures: readonly string[];
  readonly coreManifestError?: string;
  readonly missingKeypairs: readonly string[];
  readonly runtimeWritable: boolean;
};

export const readProcessIdentity = async (
  pid: number,
): Promise<string | null> => {
  const result = await run(
    ["ps", "-ww", "-p", String(pid), "-o", "lstart=", "-o", "command="],
    {
      allowFailure: true,
    },
  );
  const identity = result.stdout.trim().replace(/\s+/g, " ");
  return result.code === 0 && identity.length > 0 ? identity : null;
};

export const isExactOwnedProcess = async (
  process: OwnedProcess,
  identityReader: (pid: number) => Promise<string | null> = readProcessIdentity,
): Promise<boolean> => (await identityReader(process.pid)) === process.identity;

const atomicWriteJson = async (
  target: string,
  value: unknown,
): Promise<void> => {
  await fs.mkdir(path.dirname(target), { recursive: true });
  const temporary = `${target}.${process.pid}.${crypto.randomUUID()}.tmp`;
  try {
    const handle = await fs.open(temporary, "wx", 0o600);
    try {
      await handle.writeFile(`${JSON.stringify(value, null, 2)}\n`, "utf8");
      await handle.sync();
    } finally {
      await handle.close();
    }
    await fs.rename(temporary, target);
  } catch (error) {
    await fs.rm(temporary, { force: true });
    throw error;
  }
};

export const readDemoManifest = async (): Promise<DemoManifest | null> => {
  try {
    const manifest = JSON.parse(
      await fs.readFile(DEMO_MANIFEST_PATH, "utf8"),
    ) as DemoManifest;
    if (
      manifest.version !== 3 ||
      manifest.repoRoot !== REPO_ROOT ||
      manifest.composeProject !== demoComposeProject(manifest.bootId) ||
      manifest.configPath !== DEMO_CONFIG_PATH
    ) {
      throw new Error(
        `demo manifest at ${DEMO_MANIFEST_PATH} does not belong to this worktree`,
      );
    }
    return manifest;
  } catch (error) {
    if ((error as NodeJS.ErrnoException).code === "ENOENT") return null;
    throw error;
  }
};

const writeDemoManifest = (manifest: DemoManifest): Promise<void> =>
  atomicWriteJson(DEMO_MANIFEST_PATH, manifest);

const lockOwner = async (
  lockPath: string,
): Promise<{
  readonly pid: number;
  readonly identity: string;
} | null> => {
  try {
    return JSON.parse(
      await fs.readFile(path.join(lockPath, "owner.json"), "utf8"),
    ) as {
      readonly pid: number;
      readonly identity: string;
    };
  } catch {
    return null;
  }
};

export type LifecycleLockState = "absent" | "active" | "stale";

const readLifecycleLockStateOnce = async (
  lockPath = DEMO_LOCK_PATH,
  identityReader: (pid: number) => Promise<string | null> = readProcessIdentity,
): Promise<LifecycleLockState> => {
  try {
    await fs.access(lockPath);
  } catch (error) {
    if ((error as NodeJS.ErrnoException).code === "ENOENT") return "absent";
    throw error;
  }
  const owner = await lockOwner(lockPath);
  // Ownership publication follows mkdir. Missing ownership is indeterminate and therefore remains
  // locked until it is inspected; elapsed wall time cannot prove that the creator is dead.
  if (owner === null) return "active";
  return (await identityReader(owner.pid)) === owner.identity
    ? "active"
    : "stale";
};

export const readLifecycleLockState = async (
  lockPath = DEMO_LOCK_PATH,
  identityReader: (pid: number) => Promise<string | null> = readProcessIdentity,
): Promise<LifecycleLockState> => {
  return readLifecycleLockStateOnce(lockPath, identityReader);
};

export const withLifecycleLock = async <T>(
  operation: () => Promise<T>,
  lockPath = DEMO_LOCK_PATH,
  identityReader: (pid: number) => Promise<string | null> = readProcessIdentity,
): Promise<T> => {
  await fs.mkdir(path.dirname(lockPath), { recursive: true });
  try {
    await fs.mkdir(lockPath);
  } catch (error) {
    if ((error as NodeJS.ErrnoException).code !== "EEXIST") throw error;
    const owner = await lockOwner(lockPath);
    if ((await readLifecycleLockState(lockPath, identityReader)) === "active") {
      throw new Error(
        `another demo lifecycle command is running${owner === null ? "" : ` as pid ${owner.pid}`}`,
      );
    }
    throw new Error(
      `stale or unreadable lifecycle lock at ${lockPath}; inspect it before removing it`,
    );
  }
  try {
    const identity = await identityReader(process.pid);
    if (identity === null)
      throw new Error("cannot establish lifecycle process identity");
    await atomicWriteJson(path.join(lockPath, "owner.json"), {
      pid: process.pid,
      identity,
    });
    return await operation();
  } finally {
    await fs.rm(lockPath, { recursive: true, force: true });
  }
};

const collisionSnapshot = async (
  composeProject = DEMO_COMPOSE_PROJECT,
): Promise<CollisionSnapshot> => {
  const [
    commands,
    compose,
    composeVolumes,
    composeNetworks,
    ports,
    persistedFhevmState,
  ] = await Promise.all([
    Promise.all(
      DEMO_REQUIRED_COMMANDS.map(
        async (command) =>
          [
            command,
            (await run(["which", command], { allowFailure: true })).code === 0,
          ] as const,
      ),
    ),
    run(
      [
        "docker",
        "ps",
        "-a",
        "--no-trunc",
        "--filter",
        `label=com.docker.compose.project=${composeProject}`,
        "--format",
        "{{.ID}}\t{{.Names}}",
      ],
      { allowFailure: true },
    ),
    run(
      [
        "docker",
        "volume",
        "ls",
        "--filter",
        `label=com.docker.compose.project=${composeProject}`,
        "--format",
        "{{.Name}}\t{{.Name}}",
      ],
      { allowFailure: true },
    ),
    run(
      [
        "docker",
        "network",
        "ls",
        "--filter",
        `label=com.docker.compose.project=${composeProject}`,
        "--format",
        "{{.Name}}\t{{.Name}}",
      ],
      { allowFailure: true },
    ),
    Promise.all(
      RESERVED_PORTS.map(
        async (port) =>
          [
            port,
            await run(["lsof", "-nP", `-iTCP:${port}`, "-sTCP:LISTEN"], {
              allowFailure: true,
            }),
          ] as const,
      ),
    ),
    fs
      .access(path.join(REPO_ROOT, ".fhevm", "state", "state.json"))
      .then(() => true)
      .catch(() => false),
  ]);
  const parseResources = (output: string) =>
    output
      .split(/\r?\n/)
      .map((line) => line.trim())
      .filter(Boolean)
      .map((line) => {
        const [id = "", name = ""] = line.split("\t", 2);
        return { id, name };
      });
  return {
    requiredCommands: new Map(commands),
    composeContainers: parseResources(compose.stdout),
    composeVolumes: parseResources(composeVolumes.stdout),
    composeNetworks: parseResources(composeNetworks.stdout),
    occupiedPorts: new Map(
      ports
        .filter(([, result]) => result.code === 0 && result.stdout.trim())
        .map(([port, result]) => [port, result.stdout.trim()]),
    ),
    persistedFhevmState,
  };
};

export const collisionErrors = (
  snapshot: CollisionSnapshot,
  manifest: DemoManifest | null,
  exactProcesses: ReadonlyMap<ProcessName, boolean>,
): string[] => {
  const errors: string[] = [];
  for (const [command, found] of snapshot.requiredCommands) {
    if (!found) errors.push(`required command not found: ${command}`);
  }
  const expectedContainers = new Set(
    manifest?.containers.map(({ id }) => id) ?? [],
  );
  const actualContainers = new Set(
    snapshot.composeContainers.map(({ id }) => id),
  );
  const exactContainers =
    expectedContainers.size > 0 &&
    expectedContainers.size === actualContainers.size &&
    [...expectedContainers].every((id) => actualContainers.has(id));
  const exactResourceSet = (
    expected: readonly { readonly id: string }[] | undefined,
    actual: readonly { readonly id: string }[],
  ) => {
    const expectedIds = new Set(expected?.map(({ id }) => id) ?? []);
    const actualIds = new Set(actual.map(({ id }) => id));
    return (
      expectedIds.size === actualIds.size &&
      [...expectedIds].every((id) => actualIds.has(id))
    );
  };
  const exactVolumes = exactResourceSet(
    manifest?.volumes,
    snapshot.composeVolumes,
  );
  const exactNetworks = exactResourceSet(
    manifest?.networks,
    snapshot.composeNetworks,
  );
  const ownedRunning =
    manifest?.state === "running" &&
    exactContainers &&
    exactVolumes &&
    exactNetworks &&
    PROCESS_NAMES.every((name) => exactProcesses.get(name) === true);
  if (snapshot.persistedFhevmState && manifest === null) {
    errors.push(
      "unowned persisted fhevm state exists at .fhevm/state/state.json",
    );
  }
  if (snapshot.composeContainers.length > 0 && !ownedRunning) {
    errors.push(
      `unowned ${DEMO_COMPOSE_PROJECT} containers: ${snapshot.composeContainers.map(({ name }) => name).join(", ")}`,
    );
  }
  if (snapshot.composeVolumes.length > 0 && !ownedRunning) {
    errors.push(
      `unowned ${DEMO_COMPOSE_PROJECT} volumes: ${snapshot.composeVolumes.map(({ name }) => name).join(", ")}`,
    );
  }
  if (snapshot.composeNetworks.length > 0 && !ownedRunning) {
    errors.push(
      `unowned ${DEMO_COMPOSE_PROJECT} networks: ${snapshot.composeNetworks.map(({ name }) => name).join(", ")}`,
    );
  }
  if (snapshot.occupiedPorts.size > 0 && !ownedRunning) {
    errors.push(
      `reserved ports already in use: ${[...snapshot.occupiedPorts.keys()].join(", ")}`,
    );
  }
  if (manifest !== null && manifest.state !== "stopped" && !ownedRunning) {
    errors.push(
      `demo manifest is ${manifest.state} but its exact owned process set is not healthy`,
    );
  }
  return errors;
};

const exactProcessMap = async (
  manifest: DemoManifest | null,
): Promise<Map<ProcessName, boolean>> =>
  new Map(
    await Promise.all(
      PROCESS_NAMES.map(
        async (name) =>
          [
            name,
            manifest?.processes[name] !== undefined
              ? await isExactOwnedProcess(manifest.processes[name]!)
              : false,
          ] as const,
      ),
    ),
  );

type OwnedDockerResources = Pick<
  DemoManifest,
  "containers" | "volumes" | "networks"
>;

const emptyDockerResources = (resources: OwnedDockerResources): boolean =>
  resources.containers.length === 0 &&
  resources.volumes.length === 0 &&
  resources.networks.length === 0;

/**
 * Recovers the compose resources created during a failed lifecycle bring-up.
 *
 * The recovery is valid only because `up` established an empty project immediately before it
 * launched the guarded bring-up. The returned exact IDs become the failed manifest's teardown
 * boundary; any same-project resource added after this snapshot makes `down` refuse.
 */
export const recoverPartialDockerResources = (
  before: OwnedDockerResources,
  after: OwnedDockerResources,
): OwnedDockerResources => {
  if (!emptyDockerResources(before)) {
    throw new Error(
      "cannot recover partial demo resources: compose project was not empty before bring-up",
    );
  }
  return after;
};

const resourceIds = (resources: readonly { readonly id: string }[]) =>
  new Set(resources.map(({ id }) => id));

export const assertExactOwnedDockerResources = (
  expected: OwnedDockerResources,
  actual: OwnedDockerResources,
): void => {
  for (const kind of ["containers", "volumes", "networks"] as const) {
    const expectedIds = resourceIds(expected[kind]);
    const actualIds = resourceIds(actual[kind]);
    if (
      expectedIds.size !== actualIds.size ||
      [...expectedIds].some((id) => !actualIds.has(id))
    ) {
      throw new Error(
        `refusing teardown: ${kind} no longer exactly match the owned demo manifest`,
      );
    }
  }
};

type DockerInventoryRunner = (
  argv: readonly string[],
) => Promise<{ readonly stdout: string; readonly stderr: string; readonly code: number }>;

const runDockerInventory: DockerInventoryRunner = (argv) =>
  run([...argv], { allowFailure: true });

/**
 * Lists only resources carrying this boot's immutable Compose project label.
 * Any Docker failure or malformed/mismatched row is an ownership failure, never an empty result.
 */
export const readOwnedDockerResources = async (
  composeProject: string,
  runner: DockerInventoryRunner = runDockerInventory,
): Promise<OwnedDockerResources> => {
  if (
    !composeProject.startsWith(DEMO_COMPOSE_PROJECT_PREFIX) ||
    !/^[a-z0-9][a-z0-9_-]{0,62}$/.test(composeProject)
  ) {
    throw new Error(`invalid demo compose project: ${composeProject}`);
  }
  const label = "com.docker.compose.project";
  const specs = [
    {
      kind: "containers",
      argv: [
        "docker",
        "ps",
        "-a",
        "--no-trunc",
        "--filter",
        `label=${label}=${composeProject}`,
        "--format",
        `{{.ID}}\t{{.Names}}\t{{.Label "${label}"}}`,
      ],
    },
    {
      kind: "volumes",
      argv: [
        "docker",
        "volume",
        "ls",
        "--filter",
        `label=${label}=${composeProject}`,
        "--format",
        `{{.Name}}\t{{.Name}}\t{{.Label "${label}"}}`,
      ],
    },
    {
      kind: "networks",
      argv: [
        "docker",
        "network",
        "ls",
        "--filter",
        `label=${label}=${composeProject}`,
        "--format",
        `{{.ID}}\t{{.Name}}\t{{.Label "${label}"}}`,
      ],
    },
  ] as const;
  const results = await Promise.all(
    specs.map(async ({ kind, argv }) => ({ kind, result: await runner(argv) })),
  );
  const resources: Record<
    (typeof specs)[number]["kind"],
    { id: string; name: string }[]
  > = { containers: [], volumes: [], networks: [] };
  for (const { kind, result } of results) {
    if (result.code !== 0) {
      throw new Error(
        `cannot inventory owned Docker ${kind}: ${(result.stderr || result.stdout).trim() || `docker exited ${result.code}`}`,
      );
    }
    for (const line of result.stdout.split(/\r?\n/).filter(Boolean)) {
      const [id = "", name = "", project = ""] = line.split("\t", 3);
      if (id === "" || name === "" || project !== composeProject) {
        throw new Error(
          `cannot prove Docker ${kind} ownership for inventory row: ${line}`,
        );
      }
      resources[kind].push({ id, name });
    }
  }
  return resources;
};

const exactDockerResources = async (
  manifest: DemoManifest,
): Promise<boolean> => {
  try {
    assertExactOwnedDockerResources(
      manifest,
      await readOwnedDockerResources(manifest.composeProject),
    );
    return true;
  } catch {
    return false;
  }
};

const nearestExistingDirectory = async (target: string): Promise<string> => {
  let candidate = target;
  for (;;) {
    try {
      if ((await fs.stat(candidate)).isDirectory()) return candidate;
    } catch (error) {
      if ((error as NodeJS.ErrnoException).code !== "ENOENT") throw error;
    }
    const parent = path.dirname(candidate);
    if (parent === candidate) return candidate;
    candidate = parent;
  }
};

const collectManifestArchitectures = (value: unknown): string[] => {
  if (Array.isArray(value)) {
    return value.flatMap(collectManifestArchitectures);
  }
  if (value === null || typeof value !== "object") return [];
  const record = value as Record<string, unknown>;
  return [
    ...(typeof record.architecture === "string" ? [record.architecture] : []),
    ...Object.values(record).flatMap(collectManifestArchitectures),
  ];
};

const doctorEnvironmentSnapshot =
  async (): Promise<DoctorEnvironmentSnapshot> => {
    const [dockerInfo, coreManifest, keypairs, runtimeDirectory] =
      await Promise.all([
        run(
          [
            "docker",
            "info",
            "--format",
            "{{.NCPU}}\t{{.MemTotal}}\t{{.OSType}}\t{{.Architecture}}",
          ],
          { allowFailure: true },
        ),
        run(["docker", "manifest", "inspect", "--verbose", CORE_IMAGE], {
          allowFailure: true,
        }),
        Promise.all(
          REQUIRED_KEYPAIRS.map(
            async (keypair) =>
              [
                keypair,
                await fs
                  .access(keypair)
                  .then(() => true)
                  .catch(() => false),
              ] as const,
          ),
        ),
        nearestExistingDirectory(DEMO_RUNTIME_DIR),
      ]);
    const [cpus = "", memoryBytes = "", osType = "", architecture = ""] =
      dockerInfo.stdout.trim().split("\t");
    let manifestArchitectures: string[] = [];
    let manifestError: string | undefined;
    if (coreManifest.code === 0) {
      try {
        manifestArchitectures = collectManifestArchitectures(
          JSON.parse(coreManifest.stdout),
        );
      } catch (error) {
        manifestError = `invalid manifest JSON: ${error instanceof Error ? error.message : String(error)}`;
      }
    } else {
      manifestError =
        (coreManifest.stderr || coreManifest.stdout).trim() ||
        "manifest inspection failed";
    }
    return {
      docker:
        dockerInfo.code === 0
          ? {
              cpus: Number(cpus),
              memoryBytes: Number(memoryBytes),
              osType,
              architecture,
            }
          : undefined,
      dockerError:
        dockerInfo.code === 0
          ? undefined
          : (dockerInfo.stderr || dockerInfo.stdout).trim() ||
            "Docker daemon is not reachable",
      coreManifestArchitectures: [...new Set(manifestArchitectures)],
      coreManifestError: manifestError,
      missingKeypairs: keypairs
        .filter(([, present]) => !present)
        .map(([keypair]) => keypair),
      runtimeWritable: await fs
        .access(runtimeDirectory, fs.constants.W_OK)
        .then(() => true)
        .catch(() => false),
    };
  };

export const doctorEnvironmentErrors = (
  snapshot: DoctorEnvironmentSnapshot,
  platform: NodeJS.Platform = process.platform,
  architecture = process.arch,
): string[] => {
  const errors: string[] = [];
  if (snapshot.docker === undefined) {
    errors.push(`Docker daemon unavailable: ${snapshot.dockerError}`);
  } else {
    if (!Number.isFinite(snapshot.docker.cpus) || snapshot.docker.cpus < 4) {
      errors.push(
        `Docker has ${snapshot.docker.cpus} CPUs; the demo requires at least 4`,
      );
    }
    if (
      !Number.isFinite(snapshot.docker.memoryBytes) ||
      snapshot.docker.memoryBytes < 8 * 1024 ** 3
    ) {
      errors.push(
        `Docker has ${(snapshot.docker.memoryBytes / 1024 ** 3).toFixed(1)} GiB; the demo requires at least 8 GiB`,
      );
    }
    if (snapshot.docker.osType !== "linux") {
      errors.push(
        `Docker server OS is ${snapshot.docker.osType || "unknown"}; Linux containers are required`,
      );
    }
  }
  const requiredCoreArchitecture =
    platform === "darwin" && architecture === "arm64"
      ? "amd64"
      : architecture === "x64"
        ? "amd64"
        : architecture;
  if (snapshot.coreManifestError !== undefined) {
    errors.push(`cannot inspect ${CORE_IMAGE}: ${snapshot.coreManifestError}`);
  } else if (
    !snapshot.coreManifestArchitectures.includes(requiredCoreArchitecture)
  ) {
    errors.push(
      `${CORE_IMAGE} has no linux/${requiredCoreArchitecture} manifest`,
    );
  }
  if (!snapshot.runtimeWritable) {
    errors.push(`demo runtime parent is not writable: ${DEMO_RUNTIME_DIR}`);
  }
  for (const keypair of snapshot.missingKeypairs) {
    errors.push(`required demo keypair not found: ${keypair}`);
  }
  return errors;
};

export const doctorDemo = async (): Promise<{
  readonly manifest: DemoManifest | null;
  readonly errors: string[];
}> => {
  const manifest = await readDemoManifest();
  const [snapshot, exact, environment] = await Promise.all([
    collisionSnapshot(manifest?.composeProject ?? DEMO_COMPOSE_PROJECT),
    exactProcessMap(manifest),
    doctorEnvironmentSnapshot(),
  ]);
  const errors = [
    ...collisionErrors(snapshot, manifest, exact),
    ...doctorEnvironmentErrors(environment),
  ];
  console.log(`[doctor] repo=${REPO_ROOT}`);
  console.log(`[doctor] config=${DEMO_CONFIG_PATH}`);
  console.log(`[doctor] host=${process.platform}/${process.arch}`);
  if (environment.docker !== undefined) {
    console.log(
      `[doctor] docker=${environment.docker.osType}/${environment.docker.architecture} cpus=${environment.docker.cpus} memory=${(environment.docker.memoryBytes / 1024 ** 3).toFixed(1)}GiB`,
    );
  }
  console.log(
    `[doctor] ${CORE_IMAGE} architectures=${environment.coreManifestArchitectures.join(",") || "unavailable"}`,
  );
  console.log(
    process.platform === "darwin" && process.arch === "arm64"
      ? "[doctor] centralized kms-core=linux/amd64 (Docker Desktop emulation is exercised by up); all other services remain native"
      : "[doctor] centralized kms-core=native host platform",
  );
  if (errors.length === 0) console.log("[doctor] ready");
  for (const error of errors) console.error(`[doctor] ${error}`);
  return { manifest, errors };
};

const waitForHttp = async (
  url: string,
  label: string,
  validate: (response: Response) => boolean | Promise<boolean> = (response) =>
    response.ok,
): Promise<void> => {
  let last = "not reachable";
  for (let attempt = 0; attempt < 60; attempt += 1) {
    try {
      const response = await fetch(url, {
        signal: AbortSignal.timeout(2_000),
      });
      if (await validate(response)) return;
      last = `HTTP ${response.status}`;
    } catch (error) {
      last = error instanceof Error ? error.message : String(error);
    }
    await Bun.sleep(500);
  }
  throw new Error(`${label} did not become healthy: ${last}`);
};

const OWNED_PROCESS_ENV_KEYS = [
  "PATH",
  "HOME",
  "TMPDIR",
  "TMP",
  "TEMP",
  "LANG",
  "LC_ALL",
  "LD_LIBRARY_PATH",
  "DYLD_LIBRARY_PATH",
  "SSL_CERT_FILE",
  "SSL_CERT_DIR",
  "NODE_EXTRA_CA_CERTS",
  "CI",
] as const;

export const ownedProcessBaseEnv = (
  source: NodeJS.ProcessEnv = process.env,
): Record<string, string> =>
  Object.fromEntries(
    OWNED_PROCESS_ENV_KEYS.flatMap((key) => {
      const value = source[key];
      return value === undefined ? [] : [[key, value] as const];
    }),
  );

type SpawnedOwnedChild = Pick<
  ReturnType<typeof Bun.spawn>,
  "exited" | "kill"
>;

export const terminateUntrackedChild = async (
  child: SpawnedOwnedChild,
): Promise<void> => {
  child.kill();
  const exited = await Promise.race([
    child.exited.then(() => true),
    Bun.sleep(1_000).then(() => false),
  ]);
  if (!exited) {
    child.kill(9);
    await child.exited;
  }
};

const startOwnedProcess = async (
  name: "faucet" | "dapp",
  command: readonly string[],
  cwd: string,
  env: Record<string, string>,
  logPath: string,
): Promise<OwnedProcess> => {
  await fs.mkdir(path.dirname(logPath), { recursive: true });
  const descriptor = openSync(logPath, "a", 0o600);
  const inherited = ownedProcessBaseEnv();
  let child: ReturnType<typeof Bun.spawn>;
  try {
    child = Bun.spawn([...command], {
      cwd,
      env: { ...inherited, ...env },
      stdin: "ignore",
      stdout: descriptor,
      stderr: descriptor,
    });
  } finally {
    closeSync(descriptor);
  }
  let identity: string | null;
  try {
    identity = await readProcessIdentity(child.pid);
  } catch (error) {
    await terminateUntrackedChild(child);
    throw error;
  }
  if (identity === null) {
    await terminateUntrackedChild(child);
    throw new Error(
      `${name} exited before its identity could be recorded; see ${logPath}`,
    );
  }
  child.unref();
  return { pid: child.pid, identity, command, logPath };
};

const processFromPidFile = async (
  name: "validator" | "listener",
  command: readonly string[],
  pidFile: string,
  logPath: string,
): Promise<OwnedProcess> => {
  const pid = Number((await fs.readFile(pidFile, "utf8")).trim());
  if (!Number.isSafeInteger(pid) || pid <= 0)
    throw new Error(`invalid ${name} pid in ${pidFile}`);
  const identity = await readProcessIdentity(pid);
  if (identity === null) throw new Error(`${name} pid ${pid} is not running`);
  return { pid, identity, command, logPath };
};

const lifecycleEnv = (
  runtimeDir: string,
  composeProject: string,
): Record<string, string> => ({
  DEMO_CONFIG_PATH,
  [DEMO_BOOT_ID_ENV]: path.basename(runtimeDir),
  DEMO_LIFECYCLE_DIR: runtimeDir,
  [FHEVM_COMPOSE_PROJECT_ENV]: composeProject,
  FHEVM_REFUSE_EXISTING: "1",
  SOLANA_LOG_DIR: path.join(runtimeDir, "logs"),
});

export const authorizedServiceEnv = (
  bootId: string,
  tokenFile: string,
): Record<string, string> => ({
  [DEMO_ALLOWED_ORIGIN_ENV]: "http://127.0.0.1:5173",
  [DEMO_AUTH_TOKEN_FILE_ENV]: tokenFile,
  [DEMO_BOOT_ID_ENV]: bootId,
});

export const demoLaunchUrl = (): string => "http://127.0.0.1:5173/";

const readyMessage = (
  command: "up" | "reseed",
  bootId: string,
): string => `[${command}] demo boot ${bootId} is ready at ${demoLaunchUrl()}`;

export const bootAuthorizationTokenPath = (bootId: string): string =>
  path.join(DEMO_RUNTIME_DIR, bootId, DEMO_AUTH_TOKEN_FILENAME);

/** Loads the capability for the current running lifecycle-owned boot into this process only. */
export const readCurrentDemoAuthorization = async () => {
  const manifest = await readDemoManifest();
  if (manifest === null || manifest.state !== "running") {
    throw new Error("no running lifecycle-owned demo boot");
  }
  return readDemoAuthorizationFromEnv({
    [DEMO_BOOT_ID_ENV]: manifest.bootId,
    [DEMO_AUTH_TOKEN_FILE_ENV]: bootAuthorizationTokenPath(manifest.bootId),
  });
};

const validatorHealthy = async (): Promise<boolean> => {
  const response = await fetch("http://127.0.0.1:8899", {
    method: "POST",
    headers: { "content-type": "application/json" },
    body: '{"jsonrpc":"2.0","id":1,"method":"getHealth"}',
    signal: AbortSignal.timeout(2_000),
  });
  if (!response.ok) return false;
  const body = (await response.json()) as { readonly result?: unknown };
  return body.result === "ok";
};

const evmRpcHealthy = async (port: number): Promise<boolean> => {
  try {
    const response = await fetch(`http://127.0.0.1:${port}`, {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: '{"jsonrpc":"2.0","id":1,"method":"eth_chainId","params":[]}',
      signal: AbortSignal.timeout(2_000),
    });
    const body = (await response.json()) as { readonly result?: unknown };
    return response.ok && typeof body.result === "string";
  } catch {
    return false;
  }
};

const httpHealthy = async (url: string): Promise<boolean> => {
  try {
    return (await fetch(url, { signal: AbortSignal.timeout(2_000) })).ok;
  } catch {
    return false;
  }
};

export const isDemoDappApiResponseHealthy = async (
  response: Response,
): Promise<boolean> => {
  if (
    !response.ok ||
    !response.headers.get("content-type")?.startsWith("application/json")
  ) {
    return false;
  }
  try {
    const body = (await response.json()) as { readonly fingerprint?: unknown };
    return typeof body.fingerprint === "string" && body.fingerprint.length > 0;
  } catch {
    return false;
  }
};

const demoDappHealthy = async (): Promise<boolean> => {
  try {
    return await isDemoDappApiResponseHealthy(
      await fetch("http://127.0.0.1:5173/api/demo-encryption-key-meta", {
        signal: AbortSignal.timeout(2_000),
      }),
    );
  } catch {
    return false;
  }
};

const ownedContainer = (
  manifest: DemoManifest,
  name: string,
): DemoManifest["containers"][number] | undefined =>
  manifest.containers.find((container) => container.name === name);

const EXPECTED_ONE_SHOT_CONTAINERS = new Set([
  "fhevm-minio-setup",
  "coprocessor-db-migration",
  "kms-connector-db-migration",
  "relayer-db-migration",
  "kms-core-init",
  "gateway-deploy-mocked-zama-oft",
  "gateway-set-relayer-mocked-payment",
  "gateway-sc-deploy",
  "gateway-sc-add-network",
  "gateway-sc-add-pausers",
  "gateway-sc-trigger-keygen",
  "gateway-sc-trigger-crsgen",
  "gateway-sc-pause",
  "gateway-sc-unpause",
  "host-sc-deploy",
  "host-sc-add-pausers",
  "host-sc-trigger-keygen",
  "host-sc-trigger-crsgen",
  "host-sc-deploy-bridge",
  "host-sc-wire-bridge",
  "host-sc-context-switch",
  "host-sc-epoch-rotation",
  "host-sc-pause",
  "host-sc-unpause",
]);

export const acceptableDockerContainerState = (
  name: string,
  status: string,
  exitCode: number,
  health: string,
): boolean =>
  (status === "running" && (health === "" || health === "healthy")) ||
  (EXPECTED_ONE_SHOT_CONTAINERS.has(name) &&
    status === "exited" &&
    exitCode === 0);

const dockerContainerHealth = async (
  manifest: DemoManifest,
  name: string,
): Promise<{ readonly ready: boolean; readonly detail: string }> => {
  const container = ownedContainer(manifest, name);
  if (container === undefined)
    return { ready: false, detail: "missing from owned manifest" };
  const result = await run(
    [
      "docker",
      "inspect",
      "--format",
      "{{.State.Status}}\t{{.State.ExitCode}}\t{{if .State.Health}}{{.State.Health.Status}}{{end}}",
      container.id,
    ],
    { allowFailure: true },
  );
  if (result.code !== 0)
    return { ready: false, detail: "not inspectable by exact ID" };
  const [status = "", rawExitCode = "", health = ""] = result.stdout
    .trim()
    .split("\t");
  const exitCode = Number(rawExitCode);
  return {
    ready: acceptableDockerContainerState(name, status, exitCode, health),
    detail: `${status}${health ? `/${health}` : ""}${status === "exited" ? ` exit=${exitCode}` : ""}`,
  };
};

const dockerLogContains = async (
  manifest: DemoManifest,
  name: string,
  pattern: RegExp,
): Promise<boolean> => {
  const container = ownedContainer(manifest, name);
  if (container === undefined) return false;
  const result = await run(["docker", "logs", "--tail", "500", container.id], {
    allowFailure: true,
  });
  return result.code === 0 && pattern.test(result.stdout + result.stderr);
};

type DemoHealth = {
  readonly validator: boolean;
  readonly listener: boolean;
  readonly faucet: boolean;
  readonly dapp: boolean;
  readonly kmsCore: boolean;
  readonly relayer: boolean;
  readonly proof: boolean;
  readonly hostRpc: boolean;
  readonly gatewayRpc: boolean;
  readonly minio: boolean;
  readonly containers: ReadonlyMap<
    string,
    { readonly ready: boolean; readonly detail: string }
  >;
};

const demoHealth = async (manifest: DemoManifest): Promise<DemoHealth> => {
  const exact = await exactProcessMap(manifest);
  const containerHealth = new Map(
    await Promise.all(
      manifest.containers.map(
        async ({ name }) =>
          [name, await dockerContainerHealth(manifest, name)] as const,
      ),
    ),
  );
  const [
    validator,
    faucet,
    dapp,
    kmsReady,
    relayerEndpoint,
    proofEndpoint,
    hostRpc,
    gatewayRpc,
    minio,
  ] = await Promise.all([
    validatorHealthy().catch(() => false),
    httpHealthy("http://127.0.0.1:8090/health"),
    demoDappHealthy(),
    dockerLogContains(
      manifest,
      "kms-core",
      /KMS Server service socket address/,
    ),
    httpHealthy("http://127.0.0.1:3000/healthz"),
    httpHealthy("http://127.0.0.1:8088/health/readiness"),
    evmRpcHealthy(8545),
    evmRpcHealthy(8546),
    httpHealthy("http://127.0.0.1:9000/minio/health/ready"),
  ]);
  const containerReady = (name: string) =>
    containerHealth.get(name)?.ready === true;
  return {
    validator,
    listener: exact.get("listener") === true,
    faucet,
    dapp,
    kmsCore: containerReady("kms-core") && kmsReady,
    relayer: containerReady("fhevm-relayer") && relayerEndpoint,
    proof: containerReady("fhevm-solana-proof-service") && proofEndpoint,
    hostRpc: containerReady("host-node") && hostRpc,
    gatewayRpc: containerReady("gateway-node") && gatewayRpc,
    minio: containerReady("fhevm-minio") && minio,
    containers: containerHealth,
  };
};

const allDemoHealthReady = (health: DemoHealth): boolean =>
  health.validator &&
  health.listener &&
  health.faucet &&
  health.dapp &&
  health.kmsCore &&
  health.relayer &&
  health.proof &&
  health.hostRpc &&
  health.gatewayRpc &&
  health.minio &&
  [...health.containers.values()].every(({ ready }) => ready);

const isOwnedBootHealthy = async (manifest: DemoManifest): Promise<boolean> => {
  if (manifest.state !== "running") return false;
  const [exact, containers] = await Promise.all([
    exactProcessMap(manifest),
    exactDockerResources(manifest),
  ]);
  if (
    manifest.containers.length === 0 ||
    !containers ||
    !PROCESS_NAMES.every((name) => exact.get(name) === true)
  ) {
    return false;
  }
  return allDemoHealthReady(await demoHealth(manifest));
};

export const existingBootAction = (
  manifest: DemoManifest | null,
  healthy: boolean,
): "start" | "noop" | "preserve-degraded" => {
  if (manifest === null || manifest.state === "stopped") return "start";
  return healthy ? "noop" : "preserve-degraded";
};

export const upDemo = async (): Promise<string> =>
  withLifecycleLock(async () => {
    const existing = await readDemoManifest();
    const healthy =
      existing !== null && existing.state !== "stopped"
        ? await isOwnedBootHealthy(existing)
        : false;
    const action = existingBootAction(existing, healthy);
    if (action === "noop") {
      console.log(`[up] owned boot ${existing!.bootId} is already healthy`);
      return existing!.bootId;
    }
    if (action === "preserve-degraded") {
      await statusDemo();
      throw new Error(
        `owned boot ${existing!.bootId} is degraded; its manifest and resources were preserved. Inspect 'bun run demo logs --no-follow', then use 'bun run demo reseed' if the core stack is healthy or 'bun run demo down' for an exact-owned teardown`,
      );
    }
    const { errors } = await doctorDemo();
    if (errors.length > 0)
      throw new Error(`demo collision/preflight failed:\n${errors.join("\n")}`);
    const bootId = crypto.randomUUID();
    const composeProject = demoComposeProject(bootId);
    const resourcesBeforeBringUp =
      await readOwnedDockerResources(composeProject);
    if (!emptyDockerResources(resourcesBeforeBringUp)) {
      throw new Error(
        "demo compose project changed after preflight; refusing ambiguous bring-up",
      );
    }

    const runtimeDir = path.join(DEMO_RUNTIME_DIR, bootId);
    const logsDir = path.join(runtimeDir, "logs");
    const env = lifecycleEnv(runtimeDir, composeProject);
    let manifest: DemoManifest = {
      version: 3,
      bootId,
      repoRoot: REPO_ROOT,
      composeProject,
      configPath: DEMO_CONFIG_PATH,
      createdAt: new Date().toISOString(),
      state: "starting",
      containers: [],
      volumes: [],
      networks: [],
      processes: {},
    };
    await writeDemoManifest(manifest);
    try {
      const authorization = await createDemoAuthorizationFile(
        runtimeDir,
        bootId,
      );
      const serviceEnv = authorizedServiceEnv(bootId, authorization.tokenFile);
      await runStreaming(
        ["bash", path.join(REPO_ROOT, "solana/scripts/demo/demo-up.sh")],
        { cwd: REPO_ROOT, env },
      );
      const validator = await processFromPidFile(
        "validator",
        ["solana-test-validator"],
        path.join(runtimeDir, "validator.pid"),
        path.join(logsDir, "validator.log"),
      );
      const listener = await processFromPidFile(
        "listener",
        ["solana_host_listener"],
        path.join(runtimeDir, "listener.pid"),
        path.join(logsDir, "host-listener.log"),
      );
      const resources = await readOwnedDockerResources(composeProject);
      if (resources.containers.length === 0)
        throw new Error("fhevm-cli brought up no compose containers");
      manifest = {
        ...manifest,
        ...resources,
        processes: { validator, listener },
      };
      await writeDemoManifest(manifest);
      const faucet = await startOwnedProcess(
        "faucet",
        ["bun", "run", "demo:faucet"],
        path.join(REPO_ROOT, "test-suite/fhevm"),
        serviceEnv,
        path.join(logsDir, "faucet.log"),
      );
      manifest = { ...manifest, processes: { ...manifest.processes, faucet } };
      await writeDemoManifest(manifest);
      await waitForHttp("http://127.0.0.1:8090/health", "demo faucet");
      const dapp = await startOwnedProcess(
        "dapp",
        ["bun", "run", "dev"],
        path.join(REPO_ROOT, "solana/demo-dapp"),
        serviceEnv,
        path.join(logsDir, "dapp.log"),
      );
      manifest = { ...manifest, processes: { ...manifest.processes, dapp } };
      await writeDemoManifest(manifest);
      await waitForHttp(
        "http://127.0.0.1:5173/api/demo-encryption-key-meta",
        "demo dApp API",
        isDemoDappApiResponseHealthy,
      );
      if (!allDemoHealthReady(await demoHealth(manifest))) {
        throw new Error(
          "full demo health gate failed after dApp startup; run 'bun run demo status'",
        );
      }
      manifest = { ...manifest, state: "running" };
      await writeDemoManifest(manifest);
      console.log(readyMessage("up", bootId));
      return bootId;
    } catch (error) {
      let recoveryFailure: string | undefined;
      try {
        manifest = {
          ...manifest,
          ...recoverPartialDockerResources(
            resourcesBeforeBringUp,
            await readOwnedDockerResources(composeProject),
          ),
        };
      } catch (recoveryError) {
        recoveryFailure =
          recoveryError instanceof Error
            ? recoveryError.message
            : String(recoveryError);
      }
      const recoveredProcesses = { ...manifest.processes };
      for (const [name, command, pidFile, logPath] of [
        [
          "validator",
          ["solana-test-validator"],
          path.join(runtimeDir, "validator.pid"),
          path.join(logsDir, "validator.log"),
        ],
        [
          "listener",
          ["solana_host_listener"],
          path.join(runtimeDir, "listener.pid"),
          path.join(logsDir, "host-listener.log"),
        ],
      ] as const) {
        if (recoveredProcesses[name] !== undefined) continue;
        try {
          recoveredProcesses[name] = await processFromPidFile(
            name,
            command,
            pidFile,
            logPath,
          );
        } catch {
          // The process may not have started; retain only identities that can be proven now.
        }
      }
      await writeDemoManifest({
        ...manifest,
        state: "failed",
        processes: recoveredProcesses,
        failure: [
          error instanceof Error ? error.message : String(error),
          ...(recoveryFailure === undefined
            ? []
            : [`resource ownership recovery failed: ${recoveryFailure}`]),
        ].join("\n"),
      });
      throw error;
    }
  });

export const bootStoppedMarkerPath = (bootId: string): string => {
  demoComposeProject(bootId);
  return path.join(DEMO_RUNTIME_DIR, bootId, "stopped.json");
};

export const supervisorControlSocketPath = (bootId: string): string => {
  demoComposeProject(bootId);
  const userId = process.getuid?.();
  if (userId === undefined) {
    throw new Error("demo supervisor control requires a Unix user id");
  }
  const socketName = createHash("sha256")
    .update(`${REPO_ROOT}\0${bootId}`)
    .digest("hex")
    .slice(0, 24);
  return path.join(
    "/tmp",
    `fhevm-demo-${userId}`,
    `${socketName}.sock`,
  );
};

const stoppedBootMarkerExists = async (bootId: string): Promise<boolean> => {
  try {
    const marker = JSON.parse(
      await fs.readFile(bootStoppedMarkerPath(bootId), "utf8"),
    ) as { readonly version?: number; readonly bootId?: string };
    return marker.version === 1 && marker.bootId === bootId;
  } catch (error) {
    if ((error as NodeJS.ErrnoException).code === "ENOENT") return false;
    return false;
  }
};

type SupervisorAction = "continue" | "wait" | "clean-stop" | "fail";
type SupervisorObservationAction = "observe" | "wait" | "fail";

export const supervisorObservationAction = ({
  lockBefore,
  lockAfter,
  manifestBefore,
  manifestAfter,
}: {
  readonly lockBefore: LifecycleLockState;
  readonly lockAfter: LifecycleLockState;
  readonly manifestBefore: DemoManifest | null;
  readonly manifestAfter: DemoManifest | null;
}): SupervisorObservationAction => {
  if (lockBefore === "stale" || lockAfter === "stale") return "fail";
  if (
    lockBefore === "active" ||
    lockAfter === "active" ||
    JSON.stringify(manifestBefore) !== JSON.stringify(manifestAfter)
  ) {
    return "wait";
  }
  return "observe";
};

export const supervisedBootAction = ({
  expectedBootId,
  manifest,
  stoppedProcesses,
  lockState,
  stoppedMarker,
}: {
  readonly expectedBootId: string;
  readonly manifest: Pick<DemoManifest, "bootId" | "state"> | null;
  readonly stoppedProcesses: readonly ProcessName[];
  readonly lockState: LifecycleLockState;
  readonly stoppedMarker: boolean;
}): SupervisorAction => {
  if (manifest === null || manifest.bootId !== expectedBootId) {
    return stoppedMarker ? "clean-stop" : "fail";
  }
  if (manifest.state === "stopped") return "clean-stop";
  if (lockState === "stale") return "fail";
  if (manifest.state === "running" && stoppedProcesses.length === 0) {
    return "continue";
  }
  if (lockState === "active") return "wait";
  return "fail";
};

const downAfterLifecycleIdle = async (expectedBootId: string): Promise<void> => {
  for (;;) {
    try {
      await downExpectedDemoBoot(expectedBootId);
      return;
    } catch (error) {
      if ((await readLifecycleLockState()) !== "active") throw error;
      await Bun.sleep(250);
    }
  }
};

export const serveDemo = async (): Promise<void> => {
  const expectedBootId = await upDemo();
  const initial = await readDemoManifest();
  if (
    initial?.state === "stopped" &&
    initial.bootId === expectedBootId
  ) {
    console.log(`[serve] boot ${expectedBootId} stopped`);
    return;
  }
  if (
    initial === null ||
    initial.bootId !== expectedBootId ||
    initial.state !== "running"
  ) {
    if (await stoppedBootMarkerExists(expectedBootId)) {
      console.log(`[serve] boot ${expectedBootId} stopped`);
      return;
    }
    throw new Error("serve requires one running demo boot");
  }
  let requestedSignal: "SIGINT" | "SIGTERM" | undefined;
  const requestStop = (signal: "SIGINT" | "SIGTERM") => {
    requestedSignal ??= signal;
  };
  const onSigint = () => requestStop("SIGINT");
  const onSigterm = () => requestStop("SIGTERM");
  const supervisorIdentity = await readProcessIdentity(process.pid);
  if (supervisorIdentity === null) {
    throw new Error("cannot establish demo supervisor process identity");
  }
  const stopSupervisorControl = await withLifecycleLock(() =>
    startSupervisorControl({
      socketPath: supervisorControlSocketPath(expectedBootId),
      bootId: expectedBootId,
      owner: { pid: process.pid, identity: supervisorIdentity },
      isExactOwner: async (owner) =>
        (await readProcessIdentity(owner.pid)) === owner.identity,
      onReseed: async (request) => {
        const authorization = await readCurrentDemoAuthorization();
        const decision = authorizeDemoHeaders(
          (name) =>
            name === "authorization"
              ? `Bearer ${request.token}`
              : name === "x-fhevm-demo-boot-id"
                ? request.bootId
                : undefined,
          authorization,
        );
        if (!decision.ok) throw new Error("supervisor authorization rejected");
        return reseedDemo({
          announce: false,
          expectedBootId,
        });
      },
    }),
  );
  process.on("SIGINT", onSigint);
  process.on("SIGTERM", onSigterm);
  console.log(
    `[serve] supervising native processes for boot ${expectedBootId}; use 'bun run demo down' to stop`,
  );
  try {
    for (;;) {
      if (requestedSignal !== undefined) {
        console.log(
          `[serve] ${requestedSignal} received; stopping exact owned boot`,
        );
        await downAfterLifecycleIdle(expectedBootId);
        console.log(`[serve] boot ${expectedBootId} stopped`);
        return;
      }

      const lockBefore = await readLifecycleLockState();
      const manifest = await readDemoManifest();
      const stopped: ProcessName[] = [];
      if (manifest?.bootId === expectedBootId) {
        for (const name of PROCESS_NAMES) {
          const owned = manifest.processes[name];
          if (owned === undefined || !(await isExactOwnedProcess(owned))) {
            stopped.push(name);
          }
        }
      }
      const manifestAfter = await readDemoManifest();
      const lockAfter = await readLifecycleLockState();
      const observation = supervisorObservationAction({
        lockBefore,
        lockAfter,
        manifestBefore: manifest,
        manifestAfter,
      });
      if (observation === "wait") {
        await Bun.sleep(250);
        continue;
      }
      if (observation === "fail") {
        throw new Error(
          `stale or unreadable lifecycle lock at ${DEMO_LOCK_PATH}; inspect it before removing it`,
        );
      }
      const lockState = lockAfter;
      const action = supervisedBootAction({
        expectedBootId,
        manifest: manifestAfter,
        stoppedProcesses: stopped,
        lockState,
        stoppedMarker: await stoppedBootMarkerExists(expectedBootId),
      });
      if (action === "clean-stop") {
        console.log(`[serve] boot ${expectedBootId} stopped`);
        return;
      }
      if (action === "wait") {
        await Bun.sleep(250);
        continue;
      }
      if (action === "fail") {
        if (lockState === "stale") {
          throw new Error(
            `stale or unreadable lifecycle lock at ${DEMO_LOCK_PATH}; inspect it before removing it`,
          );
        }
        if (manifestAfter === null || manifestAfter.bootId !== initial.bootId) {
          throw new Error("supervised demo manifest was removed or replaced");
        }
        throw new Error(
          manifestAfter.state !== "running"
            ? `supervised demo entered unexpected state ${manifestAfter.state}`
            : `supervised native process stopped: ${stopped.join(", ")}`,
        );
      }
      await Bun.sleep(1_000);
    }
  } finally {
    process.off("SIGINT", onSigint);
    process.off("SIGTERM", onSigterm);
    await withLifecycleLock(stopSupervisorControl);
  }
};

const stopOwnedProcess = async (
  name: ProcessName,
  owned: OwnedProcess | undefined,
): Promise<void> => {
  if (owned === undefined) return;
  const identity = await readProcessIdentity(owned.pid);
  if (identity === null) return;
  if (identity !== owned.identity) {
    throw new Error(
      `refusing to stop ${name}: pid ${owned.pid} no longer has its recorded start identity`,
    );
  }
  process.kill(owned.pid, "SIGTERM");
  for (let attempt = 0; attempt < 50; attempt += 1) {
    if ((await readProcessIdentity(owned.pid)) === null) return;
    await Bun.sleep(100);
  }
  if (await isExactOwnedProcess(owned)) process.kill(owned.pid, "SIGKILL");
};

const removeExactDockerResources = async (
  resources: OwnedDockerResources,
): Promise<void> => {
  if (resources.containers.length > 0) {
    await runStreaming([
      "docker",
      "rm",
      "-fv",
      ...resources.containers.map(({ id }) => id),
    ]);
  }
  if (resources.networks.length > 0) {
    await runStreaming([
      "docker",
      "network",
      "rm",
      ...resources.networks.map(({ id }) => id),
    ]);
  }
  if (resources.volumes.length > 0) {
    await runStreaming([
      "docker",
      "volume",
      "rm",
      "-f",
      ...resources.volumes.map(({ id }) => id),
    ]);
  }
};

export const expectedBootShutdownAction = (
  expectedBootId: string,
  manifest: Pick<DemoManifest, "bootId" | "state"> | null,
): "down" | "clean-stop" =>
  manifest !== null &&
  manifest.bootId === expectedBootId &&
  manifest.state !== "stopped"
    ? "down"
    : "clean-stop";

const stopDemoManifest = async (manifest: DemoManifest): Promise<void> => {
  assertExactOwnedDockerResources(
    manifest,
    await readOwnedDockerResources(manifest.composeProject),
  );
  for (const name of [...PROCESS_NAMES].reverse())
    await stopOwnedProcess(name, manifest.processes[name]);
  // Revalidate after native-process shutdown. Removal below remains scoped to these exact IDs,
  // so a foreign same-project resource appearing after this check cannot be selected.
  assertExactOwnedDockerResources(
    manifest,
    await readOwnedDockerResources(manifest.composeProject),
  );
  await removeExactDockerResources(manifest);
  await fs.rm(bootAuthorizationTokenPath(manifest.bootId), { force: true });
  await fs.rm(DEMO_CONFIG_PATH, { force: true });
  await fs.rm(path.join(REPO_ROOT, ".fhevm", "state", "state.json"), {
    force: true,
  });
  for (const directory of ["addresses", "compose", "config", "env"]) {
    await fs.rm(path.join(REPO_ROOT, ".fhevm", "runtime", directory), {
      recursive: true,
      force: true,
    });
  }
  await atomicWriteJson(bootStoppedMarkerPath(manifest.bootId), {
    version: 1,
    bootId: manifest.bootId,
    stoppedAt: new Date().toISOString(),
  });
  await writeDemoManifest({
    ...manifest,
    state: "stopped",
    containers: [],
    volumes: [],
    networks: [],
    processes: {},
  });
  console.log(`[down] stopped owned boot ${manifest.bootId}`);
};

const downExpectedDemoBoot = async (expectedBootId: string): Promise<void> =>
  withLifecycleLock(async () => {
    const manifest = await readDemoManifest();
    if (expectedBootShutdownAction(expectedBootId, manifest) === "clean-stop") {
      console.log(`[serve] boot ${expectedBootId} already stopped or replaced`);
      return;
    }
    if (manifest === null) throw new Error("expected demo boot disappeared");
    await stopDemoManifest(manifest);
  });

export const downDemo = async (): Promise<void> =>
  withLifecycleLock(async () => {
    const manifest = await readDemoManifest();
    if (manifest === null || manifest.state === "stopped") {
      console.log("[down] no owned demo boot");
      return;
    }
    await stopDemoManifest(manifest);
  });

const reseedReadyMessage = ({
  bootId,
  launchUrl,
}: SupervisorReseedResult): string => `[reseed] refreshed owned boot ${bootId}; reopen ${launchUrl}`;

export const reseedTargetAction = (
  expectedBootId: string | undefined,
  manifest: Pick<DemoManifest, "bootId"> | null,
): "proceed" | "replaced" =>
  expectedBootId === undefined || manifest?.bootId === expectedBootId
    ? "proceed"
    : "replaced";

export const reseedDemo = async ({
  announce = true,
  expectedBootId,
}: {
  readonly announce?: boolean;
  readonly expectedBootId?: string;
} = {}): Promise<SupervisorReseedResult> =>
  withLifecycleLock(async () => {
    const manifest = await readDemoManifest();
    if (reseedTargetAction(expectedBootId, manifest) === "replaced") {
      throw new Error(`supervised boot ${expectedBootId} was replaced`);
    }
    if (manifest === null || !(await isOwnedBootHealthy(manifest))) {
      throw new Error("reseed requires one healthy, exactly-owned demo boot");
    }
    let nextManifest = manifest;
    try {
      await stopOwnedProcess("dapp", manifest.processes.dapp);
      await stopOwnedProcess("faucet", manifest.processes.faucet);
      nextManifest = {
        ...manifest,
        state: "starting",
        processes: {
          validator: manifest.processes.validator,
          listener: manifest.processes.listener,
        },
      };
      await writeDemoManifest(nextManifest);
      const runtimeDir = path.join(DEMO_RUNTIME_DIR, manifest.bootId);
      const logsDir = path.join(runtimeDir, "logs");
      const env = lifecycleEnv(runtimeDir, manifest.composeProject);
      const authorization = await createDemoAuthorizationFile(
        runtimeDir,
        manifest.bootId,
      );
      const serviceEnv = authorizedServiceEnv(
        manifest.bootId,
        authorization.tokenFile,
      );
      await runStreaming(
        [
          "bash",
          path.join(REPO_ROOT, "solana/scripts/demo/deploy-demo-programs.sh"),
        ],
        {
          cwd: REPO_ROOT,
          env,
        },
      );
      await runStreaming(["bun", "run", "demo:seed"], {
        cwd: path.join(REPO_ROOT, "test-suite/fhevm"),
        env: {
          ...env,
          NODE_PATH: path.join(REPO_ROOT, "solana/demo-dapp/node_modules"),
        },
      });
      const faucet = await startOwnedProcess(
        "faucet",
        ["bun", "run", "demo:faucet"],
        path.join(REPO_ROOT, "test-suite/fhevm"),
        serviceEnv,
        path.join(logsDir, "faucet.log"),
      );
      nextManifest = {
        ...nextManifest,
        processes: { ...nextManifest.processes, faucet },
      };
      await writeDemoManifest(nextManifest);
      await waitForHttp("http://127.0.0.1:8090/health", "demo faucet");
      const dapp = await startOwnedProcess(
        "dapp",
        ["bun", "run", "dev"],
        path.join(REPO_ROOT, "solana/demo-dapp"),
        serviceEnv,
        path.join(logsDir, "dapp.log"),
      );
      nextManifest = {
        ...nextManifest,
        processes: { ...nextManifest.processes, dapp },
      };
      await writeDemoManifest(nextManifest);
      await waitForHttp(
        "http://127.0.0.1:5173/api/demo-encryption-key-meta",
        "demo dApp API",
        isDemoDappApiResponseHealthy,
      );
      if (!allDemoHealthReady(await demoHealth(nextManifest))) {
        throw new Error(
          "full demo health gate failed after reseed; run 'bun run demo status'",
        );
      }
      await writeDemoManifest({ ...nextManifest, state: "running" });
      const result = {
        bootId: manifest.bootId,
        launchUrl: demoLaunchUrl(),
      };
      if (announce) console.log(reseedReadyMessage(result));
      return result;
    } catch (error) {
      await writeDemoManifest({
        ...nextManifest,
        state: "failed",
        failure: error instanceof Error ? error.message : String(error),
      });
      throw error;
    }
  });

export const reseedThroughSupervisor = async (): Promise<void> => {
  const manifest = await readDemoManifest();
  if (manifest === null || manifest.state !== "running") {
    throw new Error("supervised reseed requires one running demo boot");
  }
  const socketPath = supervisorControlSocketPath(manifest.bootId);
  try {
    await fs.access(socketPath);
  } catch (error) {
    if ((error as NodeJS.ErrnoException).code === "ENOENT") {
      throw new Error(
        "no active demo supervisor; run 'bun run demo serve' or use 'bun run demo reseed --direct' only from a shell that preserves child processes",
      );
    }
    throw error;
  }
  const authorization = await readCurrentDemoAuthorization();
  const result = await requestSupervisorReseed(socketPath, {
    version: 1,
    action: "reseed",
    bootId: authorization.bootId,
    token: authorization.token,
  });
  console.log(reseedReadyMessage(result));
};

export const statusDemo = async (): Promise<boolean> => {
  const manifest = await readDemoManifest();
  if (manifest === null) {
    console.log("[status] no demo manifest");
    return false;
  }
  console.log(`[status] boot=${manifest.bootId} state=${manifest.state}`);
  let healthy = manifest.state === "running";
  const containers = await exactDockerResources(manifest);
  console.log(
    `[status] compose=${containers ? `${manifest.containers.length} containers, ${manifest.volumes.length} volumes, and ${manifest.networks.length} networks exactly owned` : "ownership mismatch"}`,
  );
  healthy &&= containers;
  for (const name of PROCESS_NAMES) {
    const owned = manifest.processes[name];
    const exact = owned !== undefined && (await isExactOwnedProcess(owned));
    console.log(
      `[status] ${name}=${exact ? `running pid ${owned.pid}` : "not-owned-or-stopped"}`,
    );
    healthy &&= exact;
  }
  const serviceHealth = await demoHealth(manifest);
  for (const [service, ready] of [
    ["validator", serviceHealth.validator],
    ["listener", serviceHealth.listener],
    ["faucet", serviceHealth.faucet],
    ["dapp", serviceHealth.dapp],
    ["kmsCore", serviceHealth.kmsCore],
    ["relayer", serviceHealth.relayer],
    ["proof", serviceHealth.proof],
    ["hostRpc", serviceHealth.hostRpc],
    ["gatewayRpc", serviceHealth.gatewayRpc],
    ["minio", serviceHealth.minio],
  ] as const) {
    console.log(`[status] health.${service}=${ready ? "ready" : "not-ready"}`);
    healthy &&= ready;
  }
  for (const [name, state] of serviceHealth.containers) {
    console.log(
      `[status] container.${name}=${state.ready ? "ready" : "not-ready"} (${state.detail})`,
    );
    healthy &&= state.ready;
  }
  return healthy;
};

const streamLogCommands = async (
  commands: readonly (readonly string[])[],
): Promise<void> => {
  const children = commands.map((command) =>
    Bun.spawn([...command], {
      stdin: "inherit",
      stdout: "inherit",
      stderr: "inherit",
    }),
  );
  const stop = () => {
    process.exitCode = 130;
    for (const child of children) child.kill();
  };
  process.once("SIGINT", stop);
  process.once("SIGTERM", stop);
  try {
    const codes = await Promise.all(children.map((child) => child.exited));
    const failed = codes.find((code) => code !== 0 && code !== 143);
    if (failed !== undefined) throw new Error(`log command exited ${failed}`);
  } finally {
    process.off("SIGINT", stop);
    process.off("SIGTERM", stop);
    for (const child of children) child.kill();
  }
};

const containerLogAliases = (name: string): readonly string[] => [
  name,
  name.replace(/^fhevm-/, ""),
  name === "fhevm-solana-proof-service" ? "proof" : "",
];

export const resolveOwnedLogContainers = (
  manifest: Pick<DemoManifest, "containers">,
  service: string,
): DemoManifest["containers"] =>
  service === "all"
    ? manifest.containers
    : manifest.containers.filter(({ name }) =>
        containerLogAliases(name).includes(service),
      );

export const logsDemo = async (
  service = "all",
  follow = true,
): Promise<void> => {
  const manifest = await readDemoManifest();
  if (manifest === null) throw new Error("no demo manifest");
  const processNames =
    service === "all"
      ? PROCESS_NAMES
      : PROCESS_NAMES.filter((name) => name === service);
  const dockerContainers = resolveOwnedLogContainers(manifest, service);
  if (processNames.length === 0 && dockerContainers.length === 0)
    throw new Error(`unknown demo log service: ${service}`);
  const paths = processNames.flatMap(
    (name) => manifest.processes[name]?.logPath ?? [],
  );
  const commands: (readonly string[])[] = [];
  if (paths.length > 0) {
    commands.push(["tail", ...(follow ? ["-f"] : []), "-n", "200", ...paths]);
  }
  for (const container of dockerContainers) {
    commands.push([
      "docker",
      "logs",
      ...(follow ? ["--follow"] : []),
      "--tail",
      "200",
      container.id,
    ]);
  }
  if (commands.length === 0) throw new Error(`no owned logs for ${service}`);
  await streamLogCommands(commands);
};
