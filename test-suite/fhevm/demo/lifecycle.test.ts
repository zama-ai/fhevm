import { afterEach, describe, expect, test } from "bun:test";
import fs from "node:fs/promises";
import os from "node:os";
import path from "node:path";

import { centralizedKmsCorePlatform } from "../src/generate/compose";
import { createDemoAuthorizationFile } from "./authorization";
import {
  acceptableDockerContainerState,
  authorizedServiceEnv,
  assertExactOwnedDockerResources,
  bootAuthorizationTokenPath,
  collisionErrors,
  DEMO_REQUIRED_COMMANDS,
  doctorEnvironmentErrors,
  demoComposeProject,
  demoLaunchUrl,
  expectedBootShutdownAction,
  existingBootAction,
  isExactOwnedProcess,
  ownedProcessBaseEnv,
  readLifecycleLockState,
  readOwnedDockerResources,
  recoverPartialDockerResources,
  reseedTargetAction,
  resolveOwnedLogContainers,
  supervisorControlSocketPath,
  supervisorObservationAction,
  supervisedBootAction,
  terminateUntrackedChild,
  withLifecycleLock,
  type DemoManifest,
} from "./lifecycle";

const temporaryDirectories: string[] = [];
afterEach(async () => {
  for (const directory of temporaryDirectories.splice(0)) {
    await fs.rm(directory, { recursive: true, force: true });
  }
});

const manifest = (): DemoManifest => ({
  version: 3,
  bootId: "12345678-1234-4123-8123-123456789abc",
  repoRoot: "/repo",
  composeProject:
    "fhevm-demo-12345678-1234-4123-8123-123456789abc",
  configPath: "/repo/.fhevm/runtime/solana-demo.json",
  createdAt: "2026-01-01T00:00:00.000Z",
  state: "running",
  containers: [{ id: "container-1", name: "kms-core" }],
  volumes: [{ id: "volume-1", name: "volume-1" }],
  networks: [{ id: "network-1", name: "fhevm_default" }],
  processes: {},
});

const readyCommands = new Map([
  ["bun", true],
  ["docker", true],
]);

describe("demo lifecycle collision policy", () => {
  test("demo-up has no direct-use or global-kill bypass", async () => {
    const script = await fs.readFile(
      path.join(import.meta.dir, "../../../solana/scripts/demo/demo-up.sh"),
      "utf8",
    );
    expect(script).not.toContain("pkill");
    expect(script).not.toContain("validator already healthy");
    expect(script).not.toContain("DEMO_LIFECYCLE_MANIFEST");
    expect(script).toContain("DEMO_BOOT_ID");
    expect(script).toContain("FHEVM_COMPOSE_PROJECT");
    expect(script).toContain('state") != "starting');
    expect(script).toContain('cd "$ROOT/solana/demo-dapp"');
    expect(script).toContain("bun install --frozen-lockfile");
    expect(script).toContain('NODE_PATH="$ROOT/solana/demo-dapp/node_modules" bun run demo:seed');
  });

  test("Solana setup keeps every lifecycle compose call on the per-boot project", async () => {
    const script = await fs.readFile(
      path.join(
        import.meta.dir,
        "../../../solana/scripts/e2e/setup-solana-side.sh",
      ),
      "utf8",
    );
    expect(script).not.toMatch(/-p\s+fhevm(?:\s|\\)/);
    expect(script).toContain(
      ': "${FHEVM_COMPOSE_PROJECT:?lifecycle mode requires FHEVM_COMPOSE_PROJECT}"',
    );
    expect(script).toContain('COMPOSE_PROJECT="$FHEVM_COMPOSE_PROJECT"');
    expect(script).toContain('-p "$COMPOSE_PROJECT" run');
  });

  test("doctor checks commands used by collision and Solana bootstrap paths", () => {
    expect(DEMO_REQUIRED_COMMANDS).toEqual(
      expect.arrayContaining([
        "docker",
        "dirname",
        "lsof",
        "python3",
        "cut",
        "seq",
        "solana-keygen",
        "solana-test-validator",
        "tr",
      ]),
    );
  });

  test("preserves an existing degraded boot instead of starting over", () => {
    expect(existingBootAction(manifest(), false)).toBe("preserve-degraded");
    expect(existingBootAction(manifest(), true)).toBe("noop");
    expect(existingBootAction({ ...manifest(), state: "stopped" }, false)).toBe(
      "start",
    );
  });

  test("fails closed on foreign compose containers and reserved ports", () => {
    expect(
      collisionErrors(
        {
          composeContainers: [{ id: "foreign", name: "kms-core" }],
          composeVolumes: [],
          composeNetworks: [],
          occupiedPorts: new Map([[8899, "solana-test-validator"]]),
          persistedFhevmState: false,
          requiredCommands: readyCommands,
        },
        null,
        new Map(),
      ),
    ).toEqual([
      "unowned fhevm containers: kms-core",
      "reserved ports already in use: 8899",
    ]);
  });

  test("accepts collisions only when the complete recorded process set is exactly owned", () => {
    expect(
      collisionErrors(
        {
          composeContainers: [{ id: "container-1", name: "kms-core" }],
          composeVolumes: [{ id: "volume-1", name: "volume-1" }],
          composeNetworks: [{ id: "network-1", name: "fhevm_default" }],
          occupiedPorts: new Map([[8899, "solana-test-validator"]]),
          persistedFhevmState: true,
          requiredCommands: readyCommands,
        },
        manifest(),
        new Map([
          ["validator", true],
          ["listener", true],
          ["faucet", true],
          ["dapp", true],
        ]),
      ),
    ).toEqual([]);
  });

  test("reports missing prerequisites and an incomplete owned boot", () => {
    expect(
      collisionErrors(
        {
          composeContainers: [],
          composeVolumes: [],
          composeNetworks: [],
          occupiedPorts: new Map(),
          persistedFhevmState: false,
          requiredCommands: new Map([["docker", false]]),
        },
        manifest(),
        new Map([["validator", true]]),
      ),
    ).toEqual([
      "required command not found: docker",
      "demo manifest is running but its exact owned process set is not healthy",
    ]);
  });
});

describe("demo lifecycle ownership primitives", () => {
  test("matches a PID only with its exact start identity", async () => {
    const identity = "Mon Jul 27 21:00:00 2026 bun test";
    const identityReader = async () => identity;
    expect(
      await isExactOwnedProcess(
        {
          pid: process.pid,
          identity,
          command: ["bun", "test"],
          logPath: "/tmp/test.log",
        },
        identityReader,
      ),
    ).toBe(true);
    expect(
      await isExactOwnedProcess(
        {
          pid: process.pid,
          identity: `${identity} changed`,
          command: ["bun", "test"],
          logPath: "/tmp/test.log",
        },
        identityReader,
      ),
    ).toBe(false);
  });

  test("serializes lifecycle mutations and removes the lock afterward", async () => {
    const directory = await fs.mkdtemp(
      path.join(os.tmpdir(), "demo-lifecycle-"),
    );
    temporaryDirectories.push(directory);
    const lockPath = path.join(directory, "lock");
    const identityReader = async (pid: number) => `identity:${pid}`;
    await withLifecycleLock(
      async () => {
        await expect(
          withLifecycleLock(async () => undefined, lockPath, identityReader),
        ).rejects.toThrow("another demo lifecycle command");
      },
      lockPath,
      identityReader,
    );
    expect(
      await fs
        .access(lockPath)
        .then(() => true)
        .catch(() => false),
    ).toBe(false);
  });

  test("distinguishes an active lifecycle lock from absent and stale locks", async () => {
    const directory = await fs.mkdtemp(
      path.join(os.tmpdir(), "demo-lifecycle-lock-"),
    );
    temporaryDirectories.push(directory);
    const lockPath = path.join(directory, "lock");
    expect(await readLifecycleLockState(lockPath)).toBe("absent");

    await fs.mkdir(lockPath);
    await fs.writeFile(
      path.join(lockPath, "owner.json"),
      JSON.stringify({ pid: 42, identity: "identity:42" }),
    );
    expect(
      await readLifecycleLockState(
        lockPath,
        async (pid) => `identity:${pid}`,
      ),
    ).toBe("active");
    expect(
      await readLifecycleLockState(lockPath, async () => "reused pid"),
    ).toBe("stale");
    await fs.rm(path.join(lockPath, "owner.json"));
    expect(
      await readLifecycleLockState(
        lockPath,
        async (pid) => `identity:${pid}`,
      ),
    ).toBe("active");
  });

  test("keeps a delayed lock owner publication transitional", async () => {
    const directory = await fs.mkdtemp(
      path.join(os.tmpdir(), "demo-lifecycle-lock-"),
    );
    temporaryDirectories.push(directory);
    const lockPath = path.join(directory, "lock");
    await fs.mkdir(lockPath);
    await Bun.sleep(50);
    expect(await readLifecycleLockState(lockPath)).toBe("active");
    await fs.writeFile(
      path.join(lockPath, "owner.json"),
      JSON.stringify({ pid: 42, identity: "identity:42" }),
    );
    expect(
      await readLifecycleLockState(
        lockPath,
        async (pid) => `identity:${pid}`,
      ),
    ).toBe("active");
  });

  test("supervisor waits only for an exact active mutation and recognizes clean stop markers", () => {
    const running = {
      bootId: manifest().bootId,
      state: "running" as const,
    };
    expect(
      supervisedBootAction({
        expectedBootId: running.bootId,
        manifest: running,
        stoppedProcesses: [],
        lockState: "absent",
        stoppedMarker: false,
      }),
    ).toBe("continue");
    expect(
      supervisedBootAction({
        expectedBootId: running.bootId,
        manifest: running,
        stoppedProcesses: ["dapp"],
        lockState: "active",
        stoppedMarker: false,
      }),
    ).toBe("wait");
    expect(
      supervisedBootAction({
        expectedBootId: running.bootId,
        manifest: running,
        stoppedProcesses: ["dapp"],
        lockState: "stale",
        stoppedMarker: false,
      }),
    ).toBe("fail");
    expect(
      supervisedBootAction({
        expectedBootId: running.bootId,
        manifest: running,
        stoppedProcesses: [],
        lockState: "stale",
        stoppedMarker: false,
      }),
    ).toBe("fail");
    expect(
      supervisedBootAction({
        expectedBootId: running.bootId,
        manifest: { bootId: crypto.randomUUID(), state: "running" },
        stoppedProcesses: [],
        lockState: "absent",
        stoppedMarker: true,
      }),
    ).toBe("clean-stop");
    expect(
      supervisedBootAction({
        expectedBootId: running.bootId,
        manifest: { ...running, state: "stopped" },
        stoppedProcesses: ["validator", "listener", "faucet", "dapp"],
        lockState: "absent",
        stoppedMarker: false,
      }),
    ).toBe("clean-stop");
  });

  test("supervisor acts only on a stable lifecycle observation", () => {
    const before = manifest();
    const after = { ...before, state: "starting" as const };
    expect(
      supervisorObservationAction({
        lockBefore: "absent",
        lockAfter: "absent",
        manifestBefore: before,
        manifestAfter: before,
      }),
    ).toBe("observe");
    expect(
      supervisorObservationAction({
        lockBefore: "absent",
        lockAfter: "absent",
        manifestBefore: before,
        manifestAfter: after,
      }),
    ).toBe("wait");
    expect(
      supervisorObservationAction({
        lockBefore: "active",
        lockAfter: "absent",
        manifestBefore: before,
        manifestAfter: before,
      }),
    ).toBe("wait");
    expect(
      supervisorObservationAction({
        lockBefore: "absent",
        lockAfter: "stale",
        manifestBefore: before,
        manifestAfter: before,
      }),
    ).toBe("fail");
  });

  test("signal cleanup cannot stop a replacement boot", () => {
    const expectedBootId = manifest().bootId;
    expect(
      expectedBootShutdownAction(expectedBootId, {
        bootId: expectedBootId,
        state: "running",
      }),
    ).toBe("down");
    expect(
      expectedBootShutdownAction(expectedBootId, {
        bootId: crypto.randomUUID(),
        state: "running",
      }),
    ).toBe("clean-stop");
    expect(expectedBootShutdownAction(expectedBootId, null)).toBe(
      "clean-stop",
    );
  });

  test("supervised reseed cannot target a replacement boot", () => {
    const expectedBootId = manifest().bootId;
    expect(
      reseedTargetAction(expectedBootId, { bootId: expectedBootId }),
    ).toBe("proceed");
    expect(
      reseedTargetAction(expectedBootId, { bootId: crypto.randomUUID() }),
    ).toBe("replaced");
    expect(reseedTargetAction(expectedBootId, null)).toBe("replaced");
    expect(
      reseedTargetAction(undefined, { bootId: crypto.randomUUID() }),
    ).toBe("proceed");
  });

  test("derives a private macOS-compatible supervisor socket path", () => {
    const socketPath = supervisorControlSocketPath(manifest().bootId);
    expect(socketPath.startsWith("/tmp/fhevm-demo-")).toBe(true);
    expect(Buffer.byteLength(socketPath)).toBeLessThanOrEqual(103);
    expect(socketPath).not.toContain(process.cwd());
    expect(socketPath).not.toContain(manifest().bootId);
  });

  test("rejects a same-project resource that appears after the first teardown check", () => {
    expect(() =>
      assertExactOwnedDockerResources(manifest(), {
        containers: [
          { id: "container-1", name: "kms-core" },
          { id: "late-foreign", name: "foreign" },
        ],
        volumes: [{ id: "volume-1", name: "volume-1" }],
        networks: [{ id: "network-1", name: "fhevm_default" }],
      }),
    ).toThrow("containers no longer exactly match");
  });

  test("recovers exact resources from a partial bring-up that started from an empty project", () => {
    const empty = { containers: [], volumes: [], networks: [] };
    const partial = {
      containers: [{ id: "container-1", name: "kms-core" }],
      volumes: [{ id: "volume-1", name: "fhevm_data" }],
      networks: [{ id: "network-1", name: "fhevm_default" }],
    };
    const recovered = recoverPartialDockerResources(empty, partial);
    expect(recovered).toEqual(partial);
    expect(() =>
      assertExactOwnedDockerResources(recovered, {
        ...partial,
        containers: [
          ...partial.containers,
          { id: "late-foreign", name: "foreign-same-project" },
        ],
      }),
    ).toThrow("containers no longer exactly match");
  });

  test("inventories only exact per-boot Compose labels with Docker's real format fields", async () => {
    const project = demoComposeProject(
      "12345678-1234-4123-8123-123456789abc",
    );
    const seen: (readonly string[])[] = [];
    const resources = await readOwnedDockerResources(project, async (argv) => {
      seen.push(argv);
      if (argv[1] === "ps") {
        return {
          stdout: `container-id\tkms-core\t${project}\n`,
          stderr: "",
          code: 0,
        };
      }
      if (argv[1] === "volume") {
        return {
          stdout: `volume-name\tvolume-name\t${project}\n`,
          stderr: "",
          code: 0,
        };
      }
      return {
        stdout: `network-id\tfhevm_default\t${project}\n`,
        stderr: "",
        code: 0,
      };
    });
    expect(resources).toEqual({
      containers: [{ id: "container-id", name: "kms-core" }],
      volumes: [{ id: "volume-name", name: "volume-name" }],
      networks: [{ id: "network-id", name: "fhevm_default" }],
    });
    expect(
      seen.every((argv) =>
        argv.includes(
          `label=com.docker.compose.project=${project}`,
        ),
      ),
    ).toBe(true);
    expect(seen.find((argv) => argv[1] === "volume")?.at(-1)).toContain(
      "{{.Name}}",
    );
  });

  test("does not capture an unlabeled concurrent Docker resource", async () => {
    const project = demoComposeProject(
      "12345678-1234-4123-8123-123456789abc",
    );
    await expect(
      readOwnedDockerResources(project, async (argv) => ({
        stdout:
          argv[1] === "ps"
            ? "foreign-id\tforeign-same-project\t\n"
            : "",
        stderr: "",
        code: 0,
      })),
    ).rejects.toThrow("cannot prove Docker containers ownership");
  });

  test("fails closed when any ownership-critical Docker inventory command fails", async () => {
    const project = demoComposeProject(
      "12345678-1234-4123-8123-123456789abc",
    );
    await expect(
      readOwnedDockerResources(project, async (argv) => ({
        stdout: "",
        stderr: argv[1] === "volume" ? "daemon unavailable" : "",
        code: argv[1] === "volume" ? 1 : 0,
      })),
    ).rejects.toThrow(
      "cannot inventory owned Docker volumes: daemon unavailable",
    );
  });

  test("refuses partial-resource recovery when the preflight project was not empty", () => {
    expect(() =>
      recoverPartialDockerResources(
        {
          containers: [{ id: "preexisting", name: "foreign-same-project" }],
          volumes: [],
          networks: [],
        },
        {
          containers: [
            { id: "preexisting", name: "foreign-same-project" },
            { id: "new", name: "kms-core" },
          ],
          volumes: [],
          networks: [],
        },
      ),
    ).toThrow("compose project was not empty before bring-up");
  });

  test("accepts running healthy services and successful one-shot containers", () => {
    expect(
      acceptableDockerContainerState("kms-core", "running", 0, "healthy"),
    ).toBe(true);
    expect(acceptableDockerContainerState("kms-core", "running", 0, "")).toBe(
      true,
    );
    expect(
      acceptableDockerContainerState(
        "coprocessor-db-migration",
        "exited",
        0,
        "",
      ),
    ).toBe(true);
    expect(acceptableDockerContainerState("kms-core", "exited", 0, "")).toBe(
      false,
    );
    expect(
      acceptableDockerContainerState("kms-core", "running", 0, "starting"),
    ).toBe(false);
    expect(
      acceptableDockerContainerState(
        "coprocessor-db-migration",
        "exited",
        1,
        "",
      ),
    ).toBe(false);
  });

  test("derives log aliases from every exact owned container", () => {
    const owned = {
      containers: [
        { id: "coprocessor", name: "fhevm-coprocessor" },
        { id: "db", name: "coprocessor-and-kms-db" },
        { id: "worker", name: "kms-connector-kms-worker" },
      ],
    };
    expect(resolveOwnedLogContainers(owned, "coprocessor")).toEqual([
      owned.containers[0],
    ]);
    expect(
      resolveOwnedLogContainers(owned, "kms-connector-kms-worker"),
    ).toEqual([owned.containers[2]]);
    expect(resolveOwnedLogContainers(owned, "all")).toEqual(owned.containers);
  });

  test("rotates a protected boot token without exposing it in env or manifest", async () => {
    const directory = await fs.mkdtemp(
      path.join(os.tmpdir(), "demo-authorization-"),
    );
    temporaryDirectories.push(directory);
    const bootId = "12345678-1234-4123-8123-123456789abc";
    const first = await createDemoAuthorizationFile(directory, bootId);
    const second = await createDemoAuthorizationFile(directory, bootId);
    expect(second.authorization.token).not.toBe(first.authorization.token);
    expect((await fs.stat(second.tokenFile)).mode & 0o777).toBe(0o600);
    const env = authorizedServiceEnv(bootId, second.tokenFile);
    expect(env.DEMO_BOOT_ID).toBe(bootId);
    expect(env.DEMO_AUTH_TOKEN_FILE).toBe(second.tokenFile);
    expect(env.DEMO_ALLOWED_ORIGIN).toBe("http://127.0.0.1:5173");
    expect(Object.keys(env).sort()).toEqual([
      "DEMO_ALLOWED_ORIGIN",
      "DEMO_AUTH_TOKEN_FILE",
      "DEMO_BOOT_ID",
    ]);
    expect(Object.values(env)).not.toContain(second.authorization.token);
    expect(Object.keys(env)).not.toContain("DEMO_LIFECYCLE_MANIFEST");
    expect(JSON.stringify(manifest())).not.toContain(
      second.authorization.token,
    );
    expect(bootAuthorizationTokenPath(bootId)).toEndWith(
      `${bootId}/authorization-token`,
    );
  });

  test("passes runtime paths but strips runner credentials from owned child processes", () => {
    expect(
      ownedProcessBaseEnv({
        PATH: "/tools/bin",
        HOME: "/home/runner",
        LANG: "C.UTF-8",
        CI: "true",
        AWS_ACCESS_KEY_S3_USER: "access-secret",
        AWS_SECRET_KEY_S3_USER: "secret-secret",
        GH_TOKEN: "github-secret",
        DEMO_AUTH_TOKEN: "demo-secret",
      }),
    ).toEqual({
      PATH: "/tools/bin",
      HOME: "/home/runner",
      LANG: "C.UTF-8",
      CI: "true",
    });
  });

  test("terminates and reaps a spawned child that cannot be recorded", async () => {
    const signals: (number | undefined)[] = [];
    let resolveExit!: (code: number) => void;
    const exited = new Promise<number>((resolve) => {
      resolveExit = resolve;
    });
    await terminateUntrackedChild({
      exited,
      kill: (signal?: number) => {
        signals.push(signal);
        resolveExit(143);
      },
    });
    expect(signals).toEqual([undefined]);
  });

  test("uses one stable plain local URL", () => {
    expect(demoLaunchUrl()).toBe("http://127.0.0.1:5173/");
  });
});

describe("Apple Silicon compose policy", () => {
  test("emulates only the centralized kms-core on Darwin arm64", () => {
    expect(centralizedKmsCorePlatform("darwin", "arm64")).toBe("linux/amd64");
    expect(centralizedKmsCorePlatform("linux", "arm64")).toBeUndefined();
    expect(centralizedKmsCorePlatform("darwin", "x64")).toBeUndefined();
  });

  test("doctor requires Docker resources, keypairs, writability, and the emulated image manifest", () => {
    expect(
      doctorEnvironmentErrors(
        {
          docker: {
            cpus: 2,
            memoryBytes: 4 * 1024 ** 3,
            osType: "linux",
            architecture: "aarch64",
          },
          coreManifestArchitectures: ["arm64"],
          missingKeypairs: ["/repo/missing.json"],
          runtimeWritable: false,
        },
        "darwin",
        "arm64",
      ),
    ).toEqual([
      "Docker has 2 CPUs; the demo requires at least 4",
      "Docker has 4.0 GiB; the demo requires at least 8 GiB",
      expect.stringContaining("has no linux/amd64 manifest"),
      expect.stringContaining("runtime parent is not writable"),
      "required demo keypair not found: /repo/missing.json",
    ]);
  });
});
