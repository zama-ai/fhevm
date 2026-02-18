import fs from "node:fs";
import path from "node:path";

type NetworkVersionRow = {
  name: string;
  registry: string;
  repository: string;
  version: string;
};

type NetworkVersionCache = {
  schema: 1;
  dashboardUrl?: string;
  fetchedAt: string;
  rows: Partial<Record<"testnet" | "mainnet", NetworkVersionRow[]>>;
};

type GrafanaDashboardPanel = {
  id?: number;
  title?: string;
};

type NetworkProfileDeps = {
  networkVersionCacheFile: string;
  runCommand: (
    args: string[],
    options?: { capture?: boolean; check?: boolean; allowFailure?: boolean },
  ) => { stdout: string; stderr: string; status: number };
  errorMessage: (error: unknown) => string;
  logWarn: (message: string) => void;
  logInfo: (message: string) => void;
};

const NETWORK_VERSION_CACHE_SCHEMA = 1;
const GRAFANA_QUERY_INTERVAL_MS = 60_000;
const GRAFANA_QUERY_MAX_DATA_POINTS = 500;
// Query one hour to reliably include at least one recent datapoint for all services.
const GRAFANA_QUERY_LOOKBACK_MS = 60 * 60 * 1000;

function curlJson(
  runCommand: NetworkProfileDeps["runCommand"],
  errorMessage: NetworkProfileDeps["errorMessage"],
  url: string,
  method: "GET" | "POST" = "GET",
  data?: unknown,
): any {
  const args = ["curl", "-sS", "-L", "-X", method, url];
  if (data !== undefined) {
    args.push("-H", "content-type: application/json", "--data", JSON.stringify(data));
  }

  const result = runCommand(args, { capture: true, check: false, allowFailure: true });
  if (result.status !== 0 || !result.stdout.trim()) {
    const details = [result.stdout.trim(), result.stderr.trim()].filter(Boolean).join("\n");
    throw new Error(`Failed HTTP request to ${url}.${details ? `\n${details}` : ""}`);
  }

  try {
    return JSON.parse(result.stdout);
  } catch (error) {
    throw new Error(`Invalid JSON response from ${url}: ${errorMessage(error)}`);
  }
}

function readNetworkVersionCache(networkVersionCacheFile: string): NetworkVersionCache | undefined {
  if (!fs.existsSync(networkVersionCacheFile)) {
    return undefined;
  }

  try {
    const parsed = JSON.parse(fs.readFileSync(networkVersionCacheFile, "utf8")) as NetworkVersionCache;
    if (parsed?.schema !== NETWORK_VERSION_CACHE_SCHEMA || !parsed?.rows || !parsed?.fetchedAt) {
      return undefined;
    }
    return parsed;
  } catch {
    return undefined;
  }
}

function writeNetworkVersionCache(networkVersionCacheFile: string, cache: NetworkVersionCache): void {
  const existing = readNetworkVersionCache(networkVersionCacheFile);
  const merged: NetworkVersionCache = {
    schema: NETWORK_VERSION_CACHE_SCHEMA,
    dashboardUrl: cache.dashboardUrl ?? existing?.dashboardUrl,
    fetchedAt: cache.fetchedAt,
    rows: {
      ...(existing?.rows ?? {}),
      ...(cache.rows ?? {}),
    },
  };

  fs.mkdirSync(path.dirname(networkVersionCacheFile), { recursive: true });
  fs.writeFileSync(networkVersionCacheFile, `${JSON.stringify(merged, null, 2)}\n`, "utf8");
}

function resolveNetworkVersionRowsViaPublicApi(
  runCommand: NetworkProfileDeps["runCommand"],
  errorMessage: NetworkProfileDeps["errorMessage"],
  dashboardUrl: string,
  panelTitle: string,
): NetworkVersionRow[] {
  const tokenMatch = dashboardUrl.match(/public-dashboards\/([a-zA-Z0-9]+)/);
  if (!tokenMatch || !tokenMatch[1]) {
    throw new Error(`Unsupported public dashboard URL format: ${dashboardUrl}`);
  }

  const token = tokenMatch[1];
  const dashboardEndpoint = `https://zamablockchain.grafana.net/api/public/dashboards/${token}`;
  const dashboardJson = curlJson(runCommand, errorMessage, dashboardEndpoint);
  const panels = Array.isArray(dashboardJson?.dashboard?.panels)
    ? (dashboardJson.dashboard.panels as GrafanaDashboardPanel[])
    : [];
  const normalizeTitle = (value: string): string => value.toLowerCase().replace(/\s+/g, " ").trim();
  const expectedTitle = normalizeTitle(panelTitle);
  const expectedTokens = expectedTitle.split(" ").filter(Boolean);
  const panel =
    panels.find((entry) => normalizeTitle(entry?.title ?? "") === expectedTitle)
    ?? panels.find((entry) => {
      const normalized = normalizeTitle(entry?.title ?? "");
      return expectedTokens.every((token) => normalized.includes(token));
    });
  if (!panel || typeof panel.id !== "number") {
    const available = panels
      .map((entry) => (entry?.title ?? "").trim())
      .filter(Boolean)
      .join(", ");
    throw new Error(`Could not find panel '${panelTitle}' in dashboard metadata${available ? ` (${available})` : ""}`);
  }

  const now = Date.now();
  const queryPayload = {
    intervalMs: GRAFANA_QUERY_INTERVAL_MS,
    maxDataPoints: GRAFANA_QUERY_MAX_DATA_POINTS,
    timeRange: {
      from: String(now - GRAFANA_QUERY_LOOKBACK_MS),
      to: String(now),
      timezone: "browser",
    },
  };
  const queryEndpoint = `https://zamablockchain.grafana.net/api/public/dashboards/${token}/panels/${panel.id}/query`;
  const queryJson = curlJson(runCommand, errorMessage, queryEndpoint, "POST", queryPayload);
  const frames = queryJson?.results?.A?.frames;
  if (!Array.isArray(frames) || frames.length === 0) {
    throw new Error(`Panel query for '${panelTitle}' returned no frames`);
  }

  const rows: NetworkVersionRow[] = [];
  const seen = new Set<string>();
  for (const frame of frames) {
    const fields = Array.isArray(frame?.schema?.fields) ? frame.schema.fields : [];
    const valueField = fields.find((field: any) => field?.type === "number" && field?.labels);
    const labels = valueField?.labels;
    const name = typeof labels?.container_name === "string" ? labels.container_name.trim() : "";
    const registry = typeof labels?.image_registry === "string" ? labels.image_registry.trim() : "";
    const repository = typeof labels?.image_repository === "string" ? labels.image_repository.trim() : "";
    const version = typeof labels?.image_tag === "string" ? labels.image_tag.trim() : "";
    if (!name || !version || seen.has(name)) {
      continue;
    }
    seen.add(name);
    rows.push({ name, registry, repository, version });
  }

  if (rows.length === 0) {
    throw new Error(`Panel query for '${panelTitle}' returned no version rows`);
  }

  return rows;
}

export function createNetworkProfileHandlers(deps: NetworkProfileDeps) {
  const { networkVersionCacheFile, runCommand, errorMessage, logWarn, logInfo } = deps;

  function resolveNetworkVersionRows(networkProfile: "testnet" | "mainnet"): NetworkVersionRow[] {
    const dashboardUrl = process.env.FHEVM_GRAFANA_PUBLIC_VERSIONS_URL
      ?? "https://zamablockchain.grafana.net/public-dashboards/4027c482ad1e44ddb1336ec04cc5a1db";
    const panelTitle = networkProfile === "testnet" ? "Testnet Currently Deployed Versions" : "Mainnet Currently Deployed Versions";

    try {
      const rows = resolveNetworkVersionRowsViaPublicApi(runCommand, errorMessage, dashboardUrl, panelTitle);
      writeNetworkVersionCache(networkVersionCacheFile, {
        schema: NETWORK_VERSION_CACHE_SCHEMA,
        dashboardUrl,
        fetchedAt: new Date().toISOString(),
        rows: { [networkProfile]: rows },
      });
      return rows;
    } catch (error) {
      const cached = readNetworkVersionCache(networkVersionCacheFile);
      const cachedRows = cached?.rows[networkProfile];
      if (cachedRows && cachedRows.length > 0) {
        logWarn(
          `Live dashboard scrape failed for '${networkProfile}' (${errorMessage(error)}). Using cached snapshot from ${cached.fetchedAt}.`,
        );
        return cachedRows;
      }
      throw error;
    }
  }

  function applyNetworkProfileVersions(networkProfile: "testnet" | "mainnet"): void {
    const rows = resolveNetworkVersionRows(networkProfile);
    if (rows.length === 0) {
      throw new Error(`No version rows found for network profile '${networkProfile}'`);
    }

    const serviceToEnvVar: Record<string, string> = {
      "coprocessor-db-migration": "COPROCESSOR_DB_MIGRATION_VERSION",
      "coprocessor-gw-listener": "COPROCESSOR_GW_LISTENER_VERSION",
      "coprocessor-host-listener-catchup-only": "COPROCESSOR_HOST_LISTENER_VERSION",
      "coprocessor-host-listener-poller": "COPROCESSOR_HOST_LISTENER_VERSION",
      "coprocessor-host-listener": "COPROCESSOR_HOST_LISTENER_VERSION",
      "coprocessor-sns-worker": "COPROCESSOR_SNS_WORKER_VERSION",
      "coprocessor-tfhe-worker": "COPROCESSOR_TFHE_WORKER_VERSION",
      "coprocessor-tx-sender": "COPROCESSOR_TX_SENDER_VERSION",
      "coprocessor-zkproof-worker": "COPROCESSOR_ZKPROOF_WORKER_VERSION",
      "kms-connector-db-migration": "CONNECTOR_DB_MIGRATION_VERSION",
      "kms-connector-gw-listener": "CONNECTOR_GW_LISTENER_VERSION",
      "kms-connector-kms-worker": "CONNECTOR_KMS_WORKER_VERSION",
      "kms-connector-tx-sender": "CONNECTOR_TX_SENDER_VERSION",
      "kms-core-enclave": "CORE_VERSION",
      "kms-core-service": "CORE_VERSION",
    };

    let sawCoprocessorDbMigration = false;
    let applied = 0;
    for (const row of rows) {
      const envVar = serviceToEnvVar[row.name];
      if (!envVar) {
        continue;
      }
      if (!row.version || row.version.trim() === "") {
        logWarn(`Skipping empty version for '${row.name}' from '${networkProfile}' dashboard row.`);
        continue;
      }
      process.env[envVar] = row.version.trim();
      if (envVar === "COPROCESSOR_DB_MIGRATION_VERSION") {
        sawCoprocessorDbMigration = true;
      }
      applied += 1;
    }

    if (applied === 0) {
      throw new Error(`No known service versions mapped for network profile '${networkProfile}'`);
    }

    if (!sawCoprocessorDbMigration) {
      const runtimeCoprocessorVersions = [
        process.env.COPROCESSOR_HOST_LISTENER_VERSION,
        process.env.COPROCESSOR_GW_LISTENER_VERSION,
        process.env.COPROCESSOR_TFHE_WORKER_VERSION,
        process.env.COPROCESSOR_SNS_WORKER_VERSION,
        process.env.COPROCESSOR_TX_SENDER_VERSION,
        process.env.COPROCESSOR_ZKPROOF_WORKER_VERSION,
      ].filter((value): value is string => typeof value === "string" && value.trim() !== "");

      if (runtimeCoprocessorVersions.length > 0) {
        const counts = new Map<string, number>();
        for (const version of runtimeCoprocessorVersions) {
          counts.set(version, (counts.get(version) ?? 0) + 1);
        }
        const fallbackVersion = [...counts.entries()].sort((a, b) => b[1] - a[1])[0][0];
        process.env.COPROCESSOR_DB_MIGRATION_VERSION = fallbackVersion;
        logInfo(`Dashboard has no coprocessor-db-migration row; using inferred version '${fallbackVersion}' for COPROCESSOR_DB_MIGRATION_VERSION.`);
      }
    }

    logInfo(`Applied ${applied} version overrides from '${networkProfile}' public dashboard snapshot.`);
  }

  return { applyNetworkProfileVersions };
}
