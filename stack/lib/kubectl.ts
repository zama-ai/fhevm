// EXEMPLAR — interface skeleton, not a working implementation

/**
 * Thin wrapper for running kubectl subprocesses.
 *
 * Every function maps to a single `kubectl` CLI invocation.  All functions
 * reject with an Error if the subprocess exits non-zero.
 */

export type KubectlExecOptions = {
  namespace?: string;
  /** Container name inside the pod; omit to use the default container. */
  container?: string;
  /** When true, allocates a pseudo-TTY (kubectl exec -t). */
  tty?: boolean;
};

export type KubectlScaleOptions = {
  namespace?: string;
};

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
};

export type KubectlGetOptions = {
  namespace?: string;
  /** Output format passed to -o (e.g. "json", "yaml"). */
  output?: string;
};

export type KubectlRunOptions = {
  namespace?: string;
  /** Container image for the one-shot Job pod. */
  image: string;
  /** Environment variables injected into the Job container. */
  env?: Record<string, string>;
  /** Wait for the Job to complete before returning. */
  wait?: boolean;
  /** Timeout for --timeout (e.g. "10m0s"). */
  timeout?: string;
};

/**
 * kubectlExec — `kubectl exec <pod> [flags] -- <command>` and resolves
 * with combined stdout.
 *
 * TODO: spawn kubectl, capture stdout, reject on non-zero exit.
 */
export async function kubectlExec(pod: string, command: string[], options?: KubectlExecOptions): Promise<string> {
  // TODO: build argv from options, spawn kubectl exec, capture stdout
  throw new Error(`kubectlExec(${pod}, ${command.join(" ")}) — TODO`);
}

/**
 * kubectlScale — `kubectl scale deployment/<name> --replicas=<n>` and
 * resolves when the API server has accepted the request.
 *
 * TODO: spawn kubectl scale, reject on non-zero exit.
 */
export async function kubectlScale(
  deploymentName: string,
  replicas: number,
  options?: KubectlScaleOptions,
): Promise<void> {
  // TODO: build argv from options, spawn kubectl scale
  throw new Error(`kubectlScale(${deploymentName}, ${replicas}) — TODO`);
}

/**
 * kubectlRolloutRestart — `kubectl rollout restart deployment/<name>` then
 * `kubectl rollout status deployment/<name> --timeout=<t>`.
 *
 * TODO: chain the two kubectl invocations sequentially.
 */
export async function kubectlRolloutRestart(deploymentName: string, options?: KubectlRolloutOptions): Promise<void> {
  // TODO: spawn kubectl rollout restart, then kubectl rollout status
  throw new Error(`kubectlRolloutRestart(${deploymentName}) — TODO`);
}

/**
 * kubectlLogs — `kubectl logs <pod> [flags]` and resolves with the log
 * text string.  When `options.follow` is true the function streams stdout
 * line-by-line until the pod exits or the caller aborts.
 *
 * TODO: spawn kubectl logs, buffer or stream stdout.
 */
export async function kubectlLogs(pod: string, options?: KubectlLogsOptions): Promise<string> {
  // TODO: build argv from options, spawn kubectl logs, capture or stream
  throw new Error(`kubectlLogs(${pod}) — TODO`);
}

/**
 * kubectlGet — `kubectl get <resource> <name> [-o <format>]` and resolves
 * with the raw output string.  Callers parse JSON/YAML as needed.
 *
 * TODO: spawn kubectl get, capture stdout.
 */
export async function kubectlGet(resource: string, name: string, options?: KubectlGetOptions): Promise<string> {
  // TODO: build argv from options, spawn kubectl get, capture stdout
  throw new Error(`kubectlGet(${resource}, ${name}) — TODO`);
}

/**
 * kubectlRun — `kubectl run <name> --image=<image> --restart=Never [flags]`
 * as a one-shot Job pod; resolves when the pod completes (exit 0).
 *
 * Used by hostTask / gatewayTask to launch sc-deploy / scUpgrade Jobs.
 *
 * TODO: spawn kubectl run, optionally wait for pod completion.
 */
export async function kubectlRun(jobName: string, options: KubectlRunOptions): Promise<void> {
  // TODO: build argv from options, spawn kubectl run, optionally wait
  throw new Error(`kubectlRun(${jobName}) — TODO`);
}
