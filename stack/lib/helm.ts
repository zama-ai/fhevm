// EXEMPLAR — interface skeleton, not a working implementation

/**
 * Thin wrapper for running helm subprocesses.
 *
 * Every function here maps to a single `helm` CLI invocation and returns the
 * raw stdout string.  Callers are responsible for parsing YAML/JSON output.
 */

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

/**
 * helmUpgrade — runs `helm upgrade [--install] <release> <chart> [flags]`
 * and resolves with stdout when the command exits 0.
 *
 * TODO: implement by spawning `helm upgrade` with the assembled flag list.
 */
export async function helmUpgrade(
  releaseName: string,
  chartPath: string,
  options: HelmUpgradeOptions,
): Promise<string> {
  // TODO: build argv from options, spawn helm, capture stdout, reject on non-zero exit
  throw new Error(`helmUpgrade(${releaseName}, ${chartPath}) — TODO`);
}

/**
 * helmUninstall — runs `helm uninstall <release> -n <namespace>` and
 * resolves with stdout when the command exits 0.
 *
 * TODO: implement by spawning `helm uninstall` with the assembled flag list.
 */
export async function helmUninstall(releaseName: string, options: HelmUninstallOptions): Promise<string> {
  // TODO: build argv from options, spawn helm, capture stdout, reject on non-zero exit
  throw new Error(`helmUninstall(${releaseName}) — TODO`);
}

/**
 * helmTemplate — runs `helm template <release> <chart> [flags]` and
 * resolves with the rendered YAML string.  Used for L0 golden-master diffs.
 *
 * TODO: implement by spawning `helm template` with the assembled flag list.
 */
export async function helmTemplate(
  releaseName: string,
  chartPath: string,
  options?: HelmTemplateOptions,
): Promise<string> {
  // TODO: build argv from options, spawn helm, capture stdout, reject on non-zero exit
  throw new Error(`helmTemplate(${releaseName}, ${chartPath}) — TODO`);
}
