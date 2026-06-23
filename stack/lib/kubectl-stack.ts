// EXEMPLAR — concrete Stack over kind + Helm + kubectl.
//
// KubectlStack implements the engine-agnostic Stack interface (./stack.ts) by wiring each
// method to the real helm/kubectl primitives (./helm, ./kubectl). The runtime/read/chaos
// methods (exec, sql, logs, waitForLog, until, chain, stop/start/restart, discovery, pin)
// are exactly the kubectl/cast operations the live fhevm-p2 boot used (verified 2026-06-17).
//
// up() renders the SAME chart topology as stack/.migration/run-l0.sh (anvil-node →
// contracts → coprocessor×N → kms-connector → listener) but with `helm upgrade --install`.
// NOTE (chart staleness): the live boot proved coprocessor/kms-connector via raw v0.13
// manifests because charts/{coprocessor,kms-connector} are v0.11; until those charts are
// bumped to v0.13.0-6, up() boots the chart versions as-is. The detailed, discovery-gated
// boot proven live is encoded separately in ./recipe.ts (bootStack) — that is the faithful
// path; this up() is the straight "real charts on kind" engine the goal names.

import { helmUninstall, helmUpgrade } from "./helm";
import {
  kubectlDelete,
  kubectlExec,
  kubectlGet,
  kubectlLogs,
  kubectlPatchConfigMap,
  kubectlRolloutRestart,
  kubectlRun,
  kubectlScale,
} from "./kubectl";
import type {
  CleanOptions,
  ContractAddresses,
  ContractTaskOptions,
  LogsOptions,
  PinOptions,
  Stack,
  StackState,
  TestOptions,
  UpgradeOptions,
  UpOptions,
} from "./stack";
import { writeFile } from "node:fs/promises";

export type KubectlStackConfig = {
  namespace: string;
  /** Repo-root charts directory (charts/). */
  chartsDir: string;
  /** stack/values directory. */
  valuesDir: string;
  /** Host-chain JSON-RPC URL (informational; chain() execs cast in the pod, see below). */
  rpcUrl: string;
  /** Host anvil pod for chain() — the CLI runs host-side, so on-chain calls exec `cast rpc`
   *  inside the anvil pod (in-cluster) rather than fetching the unreachable in-cluster RPC. */
  anvilPod: string;
  /** Image for host/gateway contract Jobs (gateway-contracts / host-contracts). */
  contractsImage: { host: string; gateway: string };
  /** Image for the e2e test runner Job. */
  testImage?: string;
  /** Coprocessor release count (centralized = 1). */
  numCoprocessors?: number;
};

/** Chart deploy order — mirrors run-l0.sh (the L0 render topology). */
const CHART_ORDER = ["anvil-node", "contracts", "coprocessor", "kms-connector", "listener"] as const;

export class KubectlStack implements Stack {
  private readonly state_: StackState = { releases: [], contractAddresses: {}, imageTags: {} };

  constructor(private readonly cfg: KubectlStackConfig) {}

  private valuesFor(chart: string, options: UpOptions): string[] {
    const files = [`${this.cfg.valuesDir}/kind-local.yaml`];
    if (chart === "kms-connector") {
      // kms-connector uses a separate per-topology file (avoids commonConfig collision).
      files.push(`${this.cfg.valuesDir}/kms-connector-${options.scenario ?? "default"}.yaml`);
    } else if ((chart === "contracts" || chart === "coprocessor") && options.lockFile) {
      files.push(options.lockFile);
    }
    if (options.valuesFile) files.push(options.valuesFile);
    for (const o of options.overrides ?? []) if (o.group === chart && o.valuesFile) files.push(o.valuesFile);
    return files;
  }

  async up(options: UpOptions): Promise<void> {
    if (options.dryRun) {
      // --dry-run: resolve the plan, make no cluster changes. A full impl renders
      // each chart via helmTemplate and prints it; here we surface the chart set.
      console.log(`[dry-run] would helm upgrade --install: ${CHART_ORDER.join(", ")}`);
      return;
    }
    const n = this.cfg.numCoprocessors ?? 1;
    for (const chart of CHART_ORDER) {
      const releases =
        chart === "coprocessor" ? Array.from({ length: n }, (_, i) => `coprocessor-${i}`) : [chart];
      for (const release of releases) {
        await helmUpgrade(release, `${this.cfg.chartsDir}/${chart}`, {
          namespace: this.cfg.namespace,
          install: true,
          wait: true,
          valuesFiles: this.valuesFor(chart, options),
        });
        this.state_.releases.push(release);
      }
    }
  }

  async down(): Promise<void> {
    // Stop the whole stack — uninstall helm releases + delete the raw-manifest workloads.
    // KEEP PVCs (persistent state); that is what distinguishes `down` from `clean`
    // (compose: `down` stops containers but does not `docker volume rm`).
    for (const release of [...this.state_.releases].reverse()) {
      await helmUninstall(release, { namespace: this.cfg.namespace, wait: true }).catch(() => {});
    }
    this.state_.releases = [];
    for (const kind of ["deployment", "statefulset", "job"]) {
      await kubectlDelete(kind, { all: true }, { namespace: this.cfg.namespace });
    }
  }

  async clean(options?: CleanOptions): Promise<void> {
    // Kind analog of compose `clean` (= down + `docker volume rm` + rm .fhevm). After the
    // teardown, WIPE the persistent state: the PVCs (kms-keys, minio-data, host-addr, anvil
    // chain-data, postgres) and the generated ConfigMaps (sc-addresses + the *-env configs).
    // This is the reset a stale kms-core vault / lost-key situation needs — plain `down`
    // keeps PVCs and would NOT clear it. (Secrets like registry-credentials are preserved;
    // the nuclear equivalent is `kubectl delete namespace`.)
    await this.down();
    await kubectlDelete("pvc", { all: true }, { namespace: this.cfg.namespace });
    await kubectlDelete("configmap", { all: true }, { namespace: this.cfg.namespace });
    // Container images live on the kind node, not the namespace, so options.keepImages is a
    // no-op on kind (kept for interface parity with the compose engine).
    void options;
  }

  async upgrade(group: string, options?: UpgradeOptions): Promise<void> {
    const tag = options?.lockFile ? undefined : this.state_.imageTags[group];
    await helmUpgrade(group, `${this.cfg.chartsDir}/${group}`, {
      namespace: this.cfg.namespace,
      install: false,
      wait: true,
      set: tag ? { "image.tag": tag } : undefined,
      valuesFiles: options?.lockFile ? [options.lockFile] : undefined,
    });
    await kubectlRolloutRestart(group, { namespace: this.cfg.namespace, timeout: "5m" }).catch(() => {});
  }

  async hostTask(command: string, options?: ContractTaskOptions): Promise<void> {
    await this.contractTask("host", this.cfg.contractsImage.host, command, options);
  }

  async gatewayTask(command: string, options?: ContractTaskOptions): Promise<void> {
    await this.contractTask("gateway", this.cfg.contractsImage.gateway, command, options);
  }

  private async contractTask(
    side: "host" | "gateway",
    image: string,
    command: string,
    options?: ContractTaskOptions,
  ): Promise<void> {
    // One-shot Job pod running the contracts image; e.g. `npx hardhat <command>`.
    const jobName = `${side}-task-${command.replace(/[^a-z0-9]+/gi, "-").slice(0, 30).toLowerCase()}`;
    await kubectlRun(jobName, {
      namespace: this.cfg.namespace,
      image,
      env: options?.env,
      command: ["/bin/sh", "-c", `npx hardhat ${command}`],
      wait: true,
      timeout: "10m0s",
    });
  }

  async snapshotContracts(surface: "host" | "gateway"): Promise<void> {
    const task = surface === "host" ? this.hostTask.bind(this) : this.gatewayTask.bind(this);
    await task("task:deploy", { env: { SNAPSHOT: "true" } });
  }

  async test(profile = "erc20", options?: TestOptions): Promise<void> {
    if (!this.cfg.testImage) throw new Error("KubectlStack.test: testImage not configured");
    await kubectlRun(`test-${profile}`, {
      namespace: this.cfg.namespace,
      image: this.cfg.testImage,
      env: {
        TEST_PROFILE: profile,
        NETWORK: options?.network ?? "staging",
        PARALLEL: options?.parallel ? "1" : "",
      },
      wait: true,
      timeout: "30m0s",
    });
  }

  async state(): Promise<StackState> {
    return this.state_;
  }

  async discovery(): Promise<ContractAddresses> {
    const json = await kubectlGet("configmap", "sc-addresses", {
      namespace: this.cfg.namespace,
      output: "jsonpath={.data}",
    });
    const raw: Record<string, string> = json.trim() ? JSON.parse(json) : {};
    const out: ContractAddresses = {};
    for (const [k, v] of Object.entries(raw)) out[k] = v;
    this.state_.contractAddresses = out;
    return out;
  }

  async refreshDiscovery(): Promise<void> {
    // Re-read sc-addresses; a full impl patches each Deployment's env from it.
    await this.discovery();
  }

  async pin(options: PinOptions): Promise<string> {
    // JSON is valid YAML — sufficient for a Helm values overlay.
    await writeFile(options.outputFile, JSON.stringify(options.values, null, 2) + "\n", "utf8");
    return options.outputFile;
  }

  async exec(pod: string, command: string[]): Promise<string> {
    return kubectlExec(pod, command, { namespace: this.cfg.namespace });
  }

  async sql(pod: string, query: string, db?: string): Promise<string> {
    const args = ["psql", "-U", "postgres", ...(db ? ["-d", db] : []), "-tAc", query];
    return kubectlExec(pod, args, { namespace: this.cfg.namespace });
  }

  async patchConfigMap(name: string, data: Record<string, string>): Promise<void> {
    await kubectlPatchConfigMap(name, data, { namespace: this.cfg.namespace });
  }

  async serviceClusterIP(name: string): Promise<string> {
    const ip = (
      await kubectlGet("service", name, { namespace: this.cfg.namespace, output: "jsonpath={.spec.clusterIP}" })
    ).trim();
    if (!ip || ip === "None") throw new Error(`service ${name} has no ClusterIP (got "${ip}")`);
    return ip;
  }

  async stop(deploymentName: string): Promise<void> {
    await kubectlScale(deploymentName, 0, { namespace: this.cfg.namespace });
  }

  async start(deploymentName: string, replicas = 1): Promise<void> {
    await kubectlScale(deploymentName, replicas, { namespace: this.cfg.namespace });
  }

  async restart(deploymentName: string): Promise<void> {
    await kubectlRolloutRestart(deploymentName, { namespace: this.cfg.namespace, timeout: "5m" });
  }

  async logs(pod: string, options?: LogsOptions): Promise<string> {
    return kubectlLogs(pod, { namespace: this.cfg.namespace, tail: options?.tail, follow: options?.follow });
  }

  async waitForLog(pod: string, pattern: RegExp, timeoutMs = 120_000): Promise<void> {
    await this.until(async () => pattern.test(await this.logs(pod, { tail: 500 })), timeoutMs, 2_000);
  }

  async waitForJob(name: string, timeoutMs = 300_000): Promise<void> {
    // Dedicated loop (NOT until(), which swallows throws): fail FAST if the Job fails, rather
    // than retrying until timeout. The Job may not exist for a moment after apply → tolerate
    // get errors as "not yet".
    const deadline = Date.now() + timeoutMs;
    for (;;) {
      const s = (
        await kubectlGet("job", name, {
          namespace: this.cfg.namespace,
          output: "jsonpath={.status.succeeded}|{.status.failed}",
        }).catch(() => "|")
      ).trim();
      const [succeeded, failed] = s.split("|");
      if (Number(succeeded) > 0) return;
      if (Number(failed) > 0) throw new Error(`job ${name} failed`);
      if (Date.now() >= deadline) throw new Error(`waitForJob ${name}: timed out after ${timeoutMs}ms`);
      await new Promise((r) => setTimeout(r, 3_000));
    }
  }

  async chain<T = unknown>(method: string, params: unknown[] = []): Promise<T> {
    // Host-side CLI → in-cluster RPC is unreachable; exec `cast rpc` inside the anvil pod
    // (foundry image ships cast) against its own localhost. Covers the recipe's calls
    // (eth_chainId readiness, anvil_setBalance funding) — both string/no-arg params.
    const args = ["cast", "rpc", method, ...params.map((p) => String(p)), "--rpc-url", "http://127.0.0.1:8545"];
    const out = (await kubectlExec(this.cfg.anvilPod, args, { namespace: this.cfg.namespace })).trim();
    return (out === "" || out === "null" ? null : out) as T;
  }

  async until(predicate: () => Promise<boolean>, timeoutMs = 120_000, intervalMs = 1_000): Promise<void> {
    const deadline = Date.now() + timeoutMs;
    let lastErr: unknown;
    for (;;) {
      // A readiness poll: the resource being checked may not exist yet (pod still scheduling,
      // table not created), so a thrown predicate means "not ready" → retry, not "fail".
      try {
        if (await predicate()) return;
        lastErr = undefined;
      } catch (e) {
        lastErr = e;
      }
      if (Date.now() >= deadline) {
        const tail = lastErr ? ` (last: ${lastErr instanceof Error ? lastErr.message : String(lastErr)})` : "";
        throw new Error(`until: not satisfied within ${timeoutMs}ms${tail}`);
      }
      await new Promise((r) => setTimeout(r, intervalMs));
    }
  }
}
