// EXEMPLAR — concrete helm wrappers.
//
// Thin wrapper for running helm subprocesses. Every function maps to a single `helm`
// CLI invocation and returns raw stdout (callers parse YAML/JSON). Rejects with stderr
// on non-zero exit. `helmTemplate` is the L0 acceptance path (helm-template golden diff);
// `helmUpgrade`/`helmUninstall` back Stack.up()/down().

import { spawn } from "node:child_process";

const HELM = process.env.HELM_BIN ?? "helm";
const MAX_BUFFER = 256 * 1024 * 1024;

export type HelmInstallOptions = {
  /** Kubernetes namespace for this release. */
  namespace: string;
  /** Paths to values files merged left-to-right (later files win). */
  valuesFiles?: string[];
  /** Individual --set key=value overrides applied after valuesFiles. */
  set?: Record<string, string>;
  /** Wait for all Pods to be Ready before returning. */
  wait?: boolean;
  /** Timeout passed to --timeout (e.g. "5m0s"). */
  timeout?: string;
};

export type HelmUpgradeOptions = HelmInstallOptions & {
  /** When true, runs `helm upgrade --install` (creates if not present). */
  install?: boolean;
};

export type HelmUninstallOptions = {
  namespace: string;
  /** When true, blocks until all resources are deleted. */
  wait?: boolean;
};

export type HelmTemplateOptions = {
  namespace?: string;
  valuesFiles?: string[];
  set?: Record<string, string>;
};

function runHelm(args: string[]): Promise<string> {
  return new Promise((resolve, reject) => {
    const child = spawn(HELM, args, { stdio: ["ignore", "pipe", "pipe"] });
    let stdout = "";
    let stderr = "";
    child.stdout.on("data", (d) => {
      stdout += d;
      if (stdout.length > MAX_BUFFER) {
        child.kill("SIGKILL");
        reject(new Error(`helm ${args[0]}: output exceeded ${MAX_BUFFER} bytes`));
      }
    });
    child.stderr.on("data", (d) => (stderr += d));
    child.on("error", reject);
    child.on("close", (code) =>
      code === 0
        ? resolve(stdout)
        : reject(new Error(`helm ${args.join(" ")} exited ${code}: ${stderr.trim() || stdout.trim()}`)),
    );
  });
}

const valuesArgs = (files?: string[]): string[] => (files ?? []).flatMap((f) => ["-f", f]);
const setArgs = (set?: Record<string, string>): string[] =>
  Object.entries(set ?? {}).flatMap(([k, v]) => ["--set", `${k}=${v}`]);

/** helmUpgrade — `helm upgrade [--install] <release> <chart> -n <ns> [flags]`. */
export async function helmUpgrade(
  releaseName: string,
  chartPath: string,
  options: HelmUpgradeOptions,
): Promise<string> {
  const args = ["upgrade"];
  if (options.install) args.push("--install");
  args.push(releaseName, chartPath, "-n", options.namespace);
  args.push(...valuesArgs(options.valuesFiles), ...setArgs(options.set));
  if (options.wait) args.push("--wait");
  if (options.timeout) args.push("--timeout", options.timeout);
  return runHelm(args);
}

/** helmUninstall — `helm uninstall <release> -n <ns> [--wait]`. */
export async function helmUninstall(releaseName: string, options: HelmUninstallOptions): Promise<string> {
  const args = ["uninstall", releaseName, "-n", options.namespace];
  if (options.wait) args.push("--wait");
  return runHelm(args);
}

/** helmTemplate — `helm template <release> <chart> [flags]`; rendered YAML for L0 diffs. */
export async function helmTemplate(
  releaseName: string,
  chartPath: string,
  options: HelmTemplateOptions = {},
): Promise<string> {
  const args = ["template", releaseName, chartPath];
  if (options.namespace) args.push("-n", options.namespace);
  args.push(...valuesArgs(options.valuesFiles), ...setArgs(options.set));
  return runHelm(args);
}
