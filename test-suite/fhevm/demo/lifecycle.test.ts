import { afterEach, describe, expect, test } from "bun:test";
import fs from "node:fs/promises";
import os from "node:os";
import path from "node:path";

import { centralizedKmsCorePlatform } from "../src/generate/compose";
import { solanaProgramIdFromKeypairFile } from "../src/generate/solana";
import { createDemoAuthorizationFile } from "./authorization";
import {
  acceptableDockerContainerState,
  authorizedServiceEnv,
  assertExactOwnedDockerResources,
  bootAuthorizationTokenPath,
  collisionErrors,
  DEMO_REQUIRED_COMMANDS,
  demoReservedPorts,
  doctorEnvironmentErrors,
  demoComposeProject,
  demoLaunchUrl,
  demoSolanaLedgerPath,
  exactEndpointReady,
  expectedBootShutdownAction,
  existingBootAction,
  isDemoDappApiResponseHealthy,
  isExactOwnedProcess,
  observabilityComposeCommand,
  observabilityModeMatches,
  ownedContainer,
  ownedProcessBaseEnv,
  parseDemoOptions,
  prometheusTargetsReady,
  readLifecycleLockState,
  readOwnedDockerResources,
  recoverPartialDockerResources,
  reseedHealthReady,
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
  version: 4,
  observability: false,
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
  test("uses a short per-worktree Solana ledger path", () => {
    const ledgerPath = demoSolanaLedgerPath(manifest().bootId);
    expect(ledgerPath).toStartWith("/tmp/fhevm-demo-");
    expect(ledgerPath.length).toBeLessThan(100);
    expect(demoSolanaLedgerPath(manifest().bootId)).toBe(ledgerPath);
  });

  test("deployment overwrites stale target program identities", async () => {
    const demoDeploy = await fs.readFile(
      path.join(import.meta.dir, "../../../solana/scripts/demo/deploy-demo-programs.sh"),
      "utf8",
    );
    expect(demoDeploy).toContain('cp -f "$SOLANA/scripts/e2e/test-keypairs/$p-keypair.json"');
    expect(demoDeploy).not.toContain("cp -n");
    // The e2e side (src/solana/validator.ts seedProgramKeypairs) copies with node's copyFile,
    // which overwrites by default; a COPYFILE_EXCL flag would reintroduce the stale-identity bug.
    const validator = await fs.readFile(
      path.join(import.meta.dir, "../src/solana/validator.ts"),
      "utf8",
    );
    expect(validator).toContain("copyFile(");
    expect(validator).not.toContain("COPYFILE_EXCL");
  });

  test("committed program keypairs match declared program identities", async () => {
    for (const program of [
      "zama-host",
      "confidential-token",
      "demo-vault",
      "confidential-batcher",
    ]) {
      const root = path.join(import.meta.dir, "../../../solana");
      const source = await fs.readFile(
        path.join(root, "programs", program, "src/lib.rs"),
        "utf8",
      );
      const declaredId = source.match(/declare_id!\("([^"]+)"\)/)?.[1];
      if (declaredId === undefined) {
        throw new Error(`${program} has no declare_id!`);
      }
      expect(
        solanaProgramIdFromKeypairFile(
          path.join(
            root,
            "scripts/e2e/test-keypairs",
            `${program.replaceAll("-", "_")}-keypair.json`,
          ),
        ),
      ).toBe(declaredId);
    }
  });

  test("reseed may recover unhealthy replaceable clients only", () => {
    const health = {
      validator: true,
      listener: true,
      faucet: false,
      dapp: false,
      kmsCore: true,
      relayer: true,
      proof: true,
      hostRpc: true,
      gatewayRpc: true,
      minio: true,
      prometheus: true,
      jaeger: true,
      containers: new Map([
        ["kms-core", { ready: true, detail: "running" }],
      ]),
    };
    expect(reseedHealthReady(health)).toBe(true);
    expect(reseedHealthReady({ ...health, relayer: false })).toBe(false);
    expect(
      reseedHealthReady({
        ...health,
        containers: new Map([
          ["kms-core", { ready: false, detail: "stopped" }],
        ]),
      }),
    ).toBe(false);
  });

  test("endpoint health cannot adopt a foreign replacement process", () => {
    expect(exactEndpointReady(true, true)).toBe(true);
    expect(exactEndpointReady(false, true)).toBe(false);
    expect(exactEndpointReady(true, false)).toBe(false);
  });

  test("parses only the explicit observability option", () => {
    expect(parseDemoOptions([])).toEqual({ observability: false });
    expect(parseDemoOptions(["--observability"])).toEqual({
      observability: true,
    });
    expect(() => parseDemoOptions(["--unknown"])).toThrow(
      "unknown demo option",
    );
  });

  test("reserves telemetry ports only for an observability boot", () => {
    expect(demoReservedPorts(false)).not.toContain(9090);
    expect(demoReservedPorts(false)).not.toContain(16686);
    expect(demoReservedPorts(true)).toEqual(
      expect.arrayContaining([9090, 16686]),
    );
  });

  test("requires a restart when a healthy boot topology differs", () => {
    expect(observabilityModeMatches(manifest(), false)).toBe(true);
    expect(observabilityModeMatches(manifest(), true)).toBe(false);
    expect(
      observabilityModeMatches({ ...manifest(), observability: true }, true),
    ).toBe(true);
  });

  test("keeps observability on the per-boot Compose project", () => {
    const project = manifest().composeProject;
    const command = observabilityComposeCommand(project);
    expect(command).toEqual(
      expect.arrayContaining(["docker", "compose", "-p", project, "-f"]),
    );
    expect(command.slice(-2)).toEqual(["up", "-d"]);
  });

  test("observability compose has no global names or external network", async () => {
    const compose = await fs.readFile(
      path.join(
        import.meta.dir,
        "observability-docker-compose.yml",
      ),
      "utf8",
    );
    expect(compose).not.toContain("container_name:");
    expect(compose).not.toContain("external: true");
    expect(compose).not.toContain("4317:4317");
    expect(compose).toContain("127.0.0.1:9090:9090");
    expect(compose).toContain("127.0.0.1:16686:16686");
  });

  test("Prometheus includes the relayer and centralized KMS targets", async () => {
    const prometheus = await fs.readFile(
      path.join(
        import.meta.dir,
        "../static/config/prometheus/prometheus.yml",
      ),
      "utf8",
    );
    expect(prometheus).toContain('"relayer:9898"');
    expect(prometheus).toContain('"kms-core:9646"');
  });

  test("resolves Compose-prefixed observability containers", () => {
    const current = manifest();
    const name = `${current.composeProject}-prometheus-1`;
    expect(
      ownedContainer(
        {
          ...current,
          containers: [{ id: "prometheus-id", name }],
        },
        "prometheus",
      ),
    ).toEqual({ id: "prometheus-id", name });
  });

  test("requires every configured Prometheus target to be up", async () => {
    const scrapeUrls = [
      "http://kms-connector-gw-listener:9100/metrics",
      "http://kms-connector-kms-worker:9100/metrics",
      "http://kms-connector-tx-sender:9100/metrics",
      "http://coprocessor-transaction-sender:9100/metrics",
      "http://coprocessor-gw-listener:9100/metrics",
      "http://coprocessor-tfhe-worker:9100/metrics",
      "http://coprocessor-sns-worker:9100/metrics",
      "http://coprocessor-zkproof-worker:9100/metrics",
      "http://relayer:9898/metrics",
      "http://kms-core:9646/metrics",
    ];
    const healthyResponse = () =>
      Response.json({
        data: {
          activeTargets: scrapeUrls.map((scrapeUrl) => ({
            health: "up",
            scrapeUrl,
          })),
        },
      });
    await expect(prometheusTargetsReady(healthyResponse())).resolves.toBe(true);
    const missing = scrapeUrls.slice(1).map((scrapeUrl) => ({
      health: "up",
      scrapeUrl,
    }));
    await expect(
      prometheusTargetsReady(
        Response.json({ data: { activeTargets: missing } }),
      ),
    ).resolves.toBe(false);
    await expect(
      prometheusTargetsReady(
        Response.json({
          data: {
            activeTargets: scrapeUrls.map((scrapeUrl, index) => ({
              health: index === 0 ? "down" : "up",
              scrapeUrl,
            })),
          },
        }),
      ),
    ).resolves.toBe(false);
  });

  test("requires the demo API rather than accepting only the Vite HTML shell", async () => {
    await expect(
      isDemoDappApiResponseHealthy(
        new Response("<!doctype html>", {
          status: 200,
          headers: { "content-type": "text/html" },
        }),
      ),
    ).resolves.toBe(false);
    await expect(
      isDemoDappApiResponseHealthy(
        Response.json({ fingerprint: "kms-key-fingerprint" }),
      ),
    ).resolves.toBe(true);
    await expect(
      isDemoDappApiResponseHealthy(Response.json({ fingerprint: "" })),
    ).resolves.toBe(false);
  });

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
    expect(script.match(/bun install --frozen-lockfile/g)).toHaveLength(1);
    expect(script.match(/bun install --force --no-cache --frozen-lockfile/g)).toHaveLength(1);
    expect(script).toMatch(
      /if ! \( cd "\$ROOT\/solana\/demo-dapp" && bun install --frozen-lockfile \); then[\s\S]*\( cd "\$ROOT\/solana\/demo-dapp" && bun install --force --no-cache --frozen-lockfile \)\nfi/,
    );
    expect(script).not.toContain("--no-verify");
    expect(script).toContain('NODE_PATH="$ROOT/solana/demo-dapp/node_modules" bun run demo:seed');
  });

  test("root package exposes one-command observable start and owned stop", async () => {
    const rootPackage = JSON.parse(
      await fs.readFile(path.join(import.meta.dir, "../../../package.json"), "utf8"),
    ) as { scripts: Record<string, string> };
    expect(rootPackage.scripts["demo:start"]).toBe("bun run demo serve --observability");
    expect(rootPackage.scripts["demo:stop"]).toBe("bun run demo down");
  });

  test("fresh bring-up materializes the built SDK before runtime canaries", async () => {
    const script = await fs.readFile(
      path.join(import.meta.dir, "../../../solana/scripts/e2e/clean-e2e.sh"),
      "utf8",
    );
    const install = script.indexOf(
      "npm ci --workspace=@fhevm/sdk-dev --workspace=@fhevm/sdk --include-workspace-root=false",
    );
    const build = script.indexOf("npm run clean && npm run build:esm && npm run build:types");
    const refresh = script.indexOf("bun install --force --frozen-lockfile");
    const materialize = script.indexOf('solana/scripts/e2e/materialize-test-sdk.sh');
    const canary = script.indexOf('node --input-type=module -e "await import(\'@fhevm/sdk/solana\')"');
    const bunCanary = script.indexOf('bun -e "await import(\'@fhevm/sdk/solana\')"');
    expect(install).toBeGreaterThan(-1);
    expect(build).toBeGreaterThan(-1);
    expect(build).toBeGreaterThan(install);
    expect(refresh).toBeGreaterThan(build);
    expect(materialize).toBeGreaterThan(refresh);
    expect(canary).toBeGreaterThan(materialize);
    expect(bunCanary).toBeGreaterThan(materialize);
    expect(script).toContain('[ ! -L "$FHEVM/node_modules/@fhevm/sdk/_esm/solana/index.js" ]');
  });

  test("local SDK consumers own the materialized package dependency graph", async () => {
    const sdkPackage = JSON.parse(
      await fs.readFile(path.join(import.meta.dir, "../../../sdk/js-sdk/src/package.json"), "utf8"),
    ) as { dependencies: Record<string, string>; exports: Record<string, unknown> };
    const consumerLock = Bun.JSONC.parse(
      await fs.readFile(path.join(import.meta.dir, "../bun.lock"), "utf8"),
    ) as {
      packages: Record<string, [string, { dependencies?: Record<string, string> }]>;
    };
    const demoDappLock = Bun.JSONC.parse(
      await fs.readFile(path.join(import.meta.dir, "../../../solana/demo-dapp/bun.lock"), "utf8"),
    ) as {
      packages: Record<string, [string, { dependencies?: Record<string, string> }]>;
    };
    const workflow = await fs.readFile(
      path.join(import.meta.dir, "../../../.github/workflows/solana-e2e.yml"),
      "utf8",
    );
    const twoHolderTransfer = await fs.readFile(
      path.join(import.meta.dir, "../src/solana/two-holder-transfer.ts"),
      "utf8",
    );
    const demoViteConfig = await fs.readFile(
      path.join(import.meta.dir, "../../../solana/demo-dapp/vite.config.ts"),
      "utf8",
    );
    const cleanE2e = await fs.readFile(
      path.join(import.meta.dir, "../../../solana/scripts/e2e/clean-e2e.sh"),
      "utf8",
    );
    // Every `@fhevm/sdk` subpath a runtime canary imports must exist in the SDK's exports map.
    // The guard this replaces counted canaries instead of validating them, so it stayed green
    // while the workflow imported `@fhevm/sdk/solana/vault` — a subpath deleted when the vault
    // module moved into the demo dapp, which fails the e2e job only after the whole stack is up.
    const exportedSubpaths = Object.keys(sdkPackage.exports);
    // Matches every spelling a canary can use — dynamic `import(...)` with or without a space,
    // `require(...)`, a static `import '...'`, and `from '...'` — so rewording a canary cannot
    // silently drop it out of this guard's sight.
    const canaryImports = (source: string): string[] =>
      [
        ...source.matchAll(
          /(?:\bimport\s*\(|\brequire\s*\(|\bfrom\s+|\bimport\s+)['"]@fhevm\/sdk(\/[^'"]*)?['"]/g,
        ),
      ].map((match) => `.${match[1] ?? ""}`);
    for (const [name, source] of [
      ["solana-e2e.yml", workflow],
      ["clean-e2e.sh", cleanE2e],
    ] as const) {
      const imports = canaryImports(source);
      expect(imports.length, `${name} has no @fhevm/sdk runtime canary`).toBeGreaterThan(0);
      for (const subpath of imports) {
        expect(exportedSubpaths, `${name} imports unexported subpath ${subpath}`).toContain(
          subpath,
        );
      }
    }
    expect(workflow).toContain("run: bun run demo reseed --direct");
    // bun, not node: the SDK worker imports the demo dapp's vault module (TS sources resolved
    // through tsconfig paths), which node's type-stripping cannot resolve.
    expect(twoHolderTransfer).toContain('run(["bun", SDK_WORKER]');
    expect(demoViteConfig).toContain("preserveSymlinks: true");
    expect(demoViteConfig).toContain("noExternal: ['@fhevm/sdk']");
    expect(workflow).not.toContain("--preserve-symlinks");
    expect(twoHolderTransfer).not.toContain("--preserve-symlinks");
    expect(consumerLock.packages["@fhevm/sdk"][1].dependencies).toEqual(sdkPackage.dependencies);
    expect(demoDappLock.packages["@fhevm/sdk"][1].dependencies).toEqual(sdkPackage.dependencies);
  });

  test("arm64 source builds use the canonical native Rust builder", async () => {
    const script = await fs.readFile(
      path.join(import.meta.dir, "../../../solana/scripts/e2e/clean-e2e.sh"),
      "utf8",
    );
    expect(script).toContain("ensure_native_rust_builders");
    expect(script).toContain("docker info --format '{{.Architecture}}'");
    expect(script).toContain(
      '"$ROOT/golden-container-images/rust-glibc"',
    );
    expect(script).toContain(
      '--build-arg "RUST_IMAGE_VERSION=$version"',
    );
    expect(script).toContain("org.zama.rust-glibc.recipe");
    expect(script).toContain("docker pull --platform linux/arm64");
    expect(script).toContain(
      'cache_image="fhevm-rust-glibc-local:$version-arm64-$recipe_short"',
    );
    expect(script).toContain("cleanup_native_rust_builder_aliases");
    expect(script).toContain('if [ "$local_arch" != "arm64" ]');
    expect(script).not.toContain("--platform linux/amd64");
  });

  test("native Rust builder cleanup preserves and restores local tags", () => {
    const result = Bun.spawnSync({
      cmd: [
        "bash",
        path.join(
          import.meta.dir,
          "../../../solana/scripts/e2e/native-rust-builders.test.sh",
        ),
      ],
      stdout: "pipe",
      stderr: "pipe",
    });
    if (result.exitCode !== 0) {
      throw new Error(
        `${result.stdout.toString()}\n${result.stderr.toString()}`,
      );
    }
  });

  test("local Rust overrides use the bundled frontend that honors native images", async () => {
    const dockerfiles = [
      "../../../kms-connector/connector-db/Dockerfile",
      "../../../kms-connector/Dockerfile.workspace",
      "../../../relayer/docker/relayer/Dockerfile",
      "../../../relayer/docker/relayer-migrate/Dockerfile",
    ];
    for (const dockerfile of dockerfiles) {
      const contents = await fs.readFile(
        path.join(import.meta.dir, dockerfile),
        "utf8",
      );
      expect(contents).not.toMatch(/^#\s*syntax=/m);
      expect(contents).toContain(
        "ghcr.io/zama-ai/fhevm/gci/rust-glibc:${RUST_IMAGE_VERSION}",
      );
    }
    const workspaceDockerfile = await fs.readFile(
      path.join(import.meta.dir, "../../../kms-connector/Dockerfile.workspace"),
      "utf8",
    );
    expect(workspaceDockerfile).not.toContain("COPY .git");
  });

  test("Solana setup keeps every lifecycle compose call on the per-boot project", async () => {
    // src/solana/deploy.ts derives the compose project through lifecycleComposeProject (its own
    // unit tests pin the validation); this pins that the registration passes the derived project,
    // never a hardcoded one.
    const deploy = await fs.readFile(
      path.join(import.meta.dir, "../src/solana/deploy.ts"),
      "utf8",
    );
    expect(deploy).toContain("lifecycleComposeProject(lifecycleDir)");
    expect(deploy).toContain("parameters.composeProject");
    expect(deploy).not.toMatch(/"-p",\s*\n?\s*"fhevm"/);
  });

  test("doctor checks commands used by collision and Solana bootstrap paths", () => {
    expect(DEMO_REQUIRED_COMMANDS).toEqual(
      expect.arrayContaining([
        "docker",
        "dirname",
        "id",
        "lsof",
        "pgrep",
        "pkill",
        "python3",
        "solana-keygen",
        "solana-test-validator",
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

  test("doctor checks Docker Compose and Apple Silicon Buildx capabilities", () => {
    const snapshot = {
      docker: {
        cpus: 8,
        memoryBytes: 16 * 1024 ** 3,
        osType: "linux",
        architecture: "aarch64",
      },
      dockerComposeError: "compose is not a docker command",
      dockerBuildxError: "buildx is not a docker command",
      coreManifestArchitectures: ["amd64"],
      missingKeypairs: [],
      runtimeWritable: true,
    };
    expect(doctorEnvironmentErrors(snapshot, "darwin", "arm64")).toEqual([
      "Docker Compose unavailable: compose is not a docker command",
      "Docker Buildx unavailable: buildx is not a docker command",
    ]);
    expect(doctorEnvironmentErrors(snapshot, "linux", "arm64")).toEqual([
      "Docker Compose unavailable: compose is not a docker command",
      "Docker Buildx unavailable: buildx is not a docker command",
      expect.stringContaining("has no linux/arm64 manifest"),
    ]);
  });
});
