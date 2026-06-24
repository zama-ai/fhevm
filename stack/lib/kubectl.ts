// EXEMPLAR — concrete kubectl wrappers.
//
// Thin wrapper for running kubectl subprocesses. Every function maps to a single
// `kubectl` CLI invocation and rejects with an Error (including stderr) on non-zero
// exit. These are the exact invocations the live fhevm-p2 boot used (verified 2026-06-17);
// the higher-level Stack (recipe.ts phases) composes them.

import { spawn } from "node:child_process";

const KUBECTL = process.env.KUBECTL_BIN ?? "kubectl";
const DEFAULT_NAMESPACE = process.env.FHEVM_NAMESPACE ?? "fhevm";
const MAX_BUFFER = 512 * 1024 * 1024; // kms-core logs can be large

export type KubectlExecOptions = {
  namespace?: string;
  /** Container name inside the pod; omit to use the default container. */
  container?: string;
  /** When true, allocates a pseudo-TTY (kubectl exec -t). */
  tty?: boolean;
  /** Data piped to the process stdin (e.g. a SQL query for psql -f -). */
  stdin?: string;
};

export type KubectlScaleOptions = { namespace?: string };

export type KubectlRolloutOptions = {
  namespace?: string;
  /** Timeout for `kubectl rollout status --timeout`. */
  timeout?: string;
};

export type KubectlLogsOptions = {
  namespace?: string;
  container?: string;
  tail?: number;
  follow?: boolean;
  /** Return logs since this duration (e.g. "5m", "1h"). */
  since?: string;
  /** Select by label instead of an exact pod name (kubectl logs -l <selector>). */
  selector?: string;
};

export type KubectlGetOptions = {
  namespace?: string;
  /** Output format passed to -o (e.g. "json", "yaml", "jsonpath=..."). */
  output?: string;
};

export type KubectlRunOptions = {
  namespace?: string;
  /** Container image for the one-shot Job pod. */
  image: string;
  /** Environment variables injected into the Job container. */
  env?: Record<string, string>;
  /** Command + args to run (after `--`). */
  command?: string[];
  /** Wait for the pod to complete before returning. */
  wait?: boolean;
  /** Timeout for --timeout (e.g. "10m0s"). */
  timeout?: string;
};

/**
 * runKubectl — spawn `kubectl <args>`, capture stdout, reject on non-zero exit with
 * stderr in the message. No shell is used (argv array), so values need no escaping.
 */
export function runKubectl(args: string[], opts: { stdin?: string; timeoutMs?: number } = {}): Promise<string> {
  return new Promise((resolve, reject) => {
    const child = spawn(KUBECTL, args, { stdio: ["pipe", "pipe", "pipe"] });
    let stdout = "";
    let stderr = "";
    let killed = false;
    const timer = opts.timeoutMs
      ? setTimeout(() => {
          killed = true;
          child.kill("SIGKILL");
        }, opts.timeoutMs)
      : undefined;

    child.stdout.on("data", (d) => {
      stdout += d;
      if (stdout.length > MAX_BUFFER) {
        child.kill("SIGKILL");
        reject(new Error(`kubectl ${args[0]}: output exceeded ${MAX_BUFFER} bytes`));
      }
    });
    child.stderr.on("data", (d) => (stderr += d));
    child.on("error", (err) => {
      if (timer) clearTimeout(timer);
      reject(err);
    });
    child.on("close", (code) => {
      if (timer) clearTimeout(timer);
      if (killed) return reject(new Error(`kubectl ${args.join(" ")} timed out`));
      if (code === 0) return resolve(stdout);
      reject(new Error(`kubectl ${args.join(" ")} exited ${code}: ${stderr.trim() || stdout.trim()}`));
    });

    if (opts.stdin !== undefined) {
      child.stdin.write(opts.stdin);
    }
    child.stdin.end();
  });
}

const ns = (namespace?: string): string[] => ["-n", namespace ?? DEFAULT_NAMESPACE];

export type KubectlDeleteTarget = { name?: string; all?: boolean; selector?: string };
export type KubectlDeleteOptions = { namespace?: string; wait?: boolean; ignoreNotFound?: boolean };

/**
 * kubectlDelete — `kubectl delete <resource> (<name>|--all|-l <selector>) [flags]`.
 * Handles cluster-scoped resources (namespace/pv/node): no `-n` is added for those.
 */
export async function kubectlDelete(
  resource: string,
  target: KubectlDeleteTarget,
  options: KubectlDeleteOptions = {},
): Promise<string> {
  const clusterScoped = ["namespace", "ns", "node", "pv", "persistentvolume"].includes(resource);
  const args = ["delete", resource];
  if (target.name) args.push(target.name);
  if (target.all) args.push("--all");
  if (target.selector) args.push("-l", target.selector);
  if (!clusterScoped) args.push(...ns(options.namespace));
  if (options.wait === false) args.push("--wait=false");
  if (options.ignoreNotFound ?? true) args.push("--ignore-not-found");
  return runKubectl(args, { timeoutMs: 300_000 });
}

/**
 * kubectlApply — `kubectl apply -f <path|->`. Pass a file path, or `manifest` to pipe
 * YAML via stdin (`-f -`). This is the raw-manifest engine path the live boot used for
 * every layer except anvil-node (whose chart is current); the v0.11 coprocessor/connector
 * charts are stale, so v0.13 components are applied as manifests until the charts are bumped.
 */
export async function kubectlApply(
  source: { path?: string; manifest?: string },
  options: { namespace?: string } = {},
): Promise<string> {
  if (source.manifest !== undefined) {
    return runKubectl(["apply", ...ns(options.namespace), "-f", "-"], { stdin: source.manifest });
  }
  if (source.path) {
    return runKubectl(["apply", ...ns(options.namespace), "-f", source.path]);
  }
  throw new Error("kubectlApply: provide either source.path or source.manifest");
}

/** kubectlPatchConfigMap — merge `data` into a ConfigMap (`kubectl patch configmap --type merge`). */
export async function kubectlPatchConfigMap(
  name: string,
  data: Record<string, string>,
  options: { namespace?: string } = {},
): Promise<string> {
  const patch = JSON.stringify({ data });
  return runKubectl(["patch", "configmap", name, ...ns(options.namespace), "--type", "merge", "-p", patch]);
}

/** kubectlExec — `kubectl exec [-n ns] [-c container] [-it] <pod> -- <command>`. */
export async function kubectlExec(pod: string, command: string[], options: KubectlExecOptions = {}): Promise<string> {
  const flags = [...ns(options.namespace)];
  if (options.container) flags.push("-c", options.container);
  if (options.stdin !== undefined) flags.push("-i");
  if (options.tty) flags.push("-t");
  return runKubectl(["exec", ...flags, pod, "--", ...command], { stdin: options.stdin });
}

/** kubectlScale — `kubectl scale deployment/<name> --replicas=<n> [-n ns]`. */
export async function kubectlScale(
  deploymentName: string,
  replicas: number,
  options: KubectlScaleOptions = {},
): Promise<void> {
  await runKubectl(["scale", `deployment/${deploymentName}`, `--replicas=${replicas}`, ...ns(options.namespace)]);
}

/**
 * kubectlRolloutRestart — `kubectl rollout restart deployment/<name>` then
 * `kubectl rollout status deployment/<name> --timeout=<t>`.
 */
export async function kubectlRolloutRestart(
  deploymentName: string,
  options: KubectlRolloutOptions = {},
): Promise<void> {
  await runKubectl(["rollout", "restart", `deployment/${deploymentName}`, ...ns(options.namespace)]);
  const statusArgs = ["rollout", "status", `deployment/${deploymentName}`, ...ns(options.namespace)];
  if (options.timeout) statusArgs.push(`--timeout=${options.timeout}`);
  await runKubectl(statusArgs);
}

/** kubectlLogs — `kubectl logs (<pod>|-l <selector>) [flags]`; returns captured text. */
export async function kubectlLogs(pod: string, options: KubectlLogsOptions = {}): Promise<string> {
  const args = ["logs", ...ns(options.namespace)];
  if (options.selector) args.push("-l", options.selector);
  else args.push(pod);
  if (options.container) args.push("-c", options.container);
  if (options.tail !== undefined) args.push(`--tail=${options.tail}`);
  if (options.since) args.push(`--since=${options.since}`);
  if (options.follow) args.push("-f");
  return runKubectl(args);
}

/** kubectlGet — `kubectl get <resource> [<name>] [-o <format>] [-n ns]`. */
export async function kubectlGet(resource: string, name: string, options: KubectlGetOptions = {}): Promise<string> {
  const args = ["get", resource];
  if (name) args.push(name);
  args.push(...ns(options.namespace));
  if (options.output) args.push("-o", options.output);
  return runKubectl(args);
}

/**
 * kubectlRun — `kubectl run <name> --image=<image> --restart=Never [--env ...] [-- cmd]`,
 * optionally waiting for completion via `kubectl wait`.
 */
export async function kubectlRun(jobName: string, options: KubectlRunOptions): Promise<void> {
  const args = ["run", jobName, `--image=${options.image}`, "--restart=Never", ...ns(options.namespace)];
  for (const [k, v] of Object.entries(options.env ?? {})) args.push("--env", `${k}=${v}`);
  if (options.command?.length) args.push("--command", "--", ...options.command);
  await runKubectl(args);
  if (options.wait) {
    const waitArgs = [
      "wait",
      `pod/${jobName}`,
      "--for=condition=Ready=false",
      "--for=jsonpath={.status.phase}=Succeeded",
      ...ns(options.namespace),
    ];
    if (options.timeout) waitArgs.push(`--timeout=${options.timeout}`);
    await runKubectl(waitArgs);
  }
}
