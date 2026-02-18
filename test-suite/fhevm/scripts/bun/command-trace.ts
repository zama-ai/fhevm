import path from "node:path";
import { TELEMETRY_REQUIRED_JAEGER_SERVICES } from "./manifest";
import type { CommandDeps } from "./command-contracts";

type ComposePsEntry = {
  Name?: string;
  Service?: string;
  State?: string;
  Status?: string;
};

const JAEGER_QUERY_PORT = 16686;
const JAEGER_OTLP_GRPC_PORT = 4317;
const JAEGER_OTLP_HTTP_PORT = 4318;
const PROMETHEUS_PORT = 9090;
const TELEMETRY_SMOKE_MAX_ATTEMPTS = 6;
const TELEMETRY_SMOKE_RETRY_DELAY_SECONDS = 5;

function telemetryServiceAliases(requiredService: string): string[] {
  switch (requiredService) {
    case "txn-sender":
      return ["transaction-sender", "kms-connector-tx-sender"];
    default:
      return [];
  }
}

export function createTraceHandlers(deps: CommandDeps) {
  const {
    PROJECT,
    COMPOSE_DIR,
    runCommand,
    sleep,
    cliError,
    logInfo,
    logWarn,
    isContainerRunningExact,
    resolveProjectContainerName,
    ensureHostPortAssignments,
    runComposeUp,
  } = deps;

  function tracingComposeFile(): string {
    return path.resolve(COMPOSE_DIR, "tracing-docker-compose.yml");
  }

  async function traceUp(): Promise<void> {
    await ensureHostPortAssignments();
    const compose = tracingComposeFile();
    const result = runComposeUp(["docker", "compose", "-p", PROJECT, "-f", compose, "up", "-d"]);
    if (result.status !== 0) {
      throw new Error("Failed to start tracing stack. Free conflicting ports or rerun deploy with --no-tracing.");
    }
  }

  function traceDown(): void {
    const compose = tracingComposeFile();
    runCommand(["docker", "compose", "-p", PROJECT, "-f", compose, "down", "-v"], {
      check: false,
      allowFailure: true,
    });
  }

  function parsePublishedHostPort(raw: string): string | undefined {
    const lines = raw
      .split("\n")
      .map((line) => line.trim())
      .filter(Boolean);

    for (const line of lines) {
      const match = line.match(/:(\d+)\s*$/);
      if (match) {
        return match[1];
      }
      if (/^\d+$/.test(line)) {
        return line;
      }
    }

    return undefined;
  }

  function resolvePublishedHostPort(
    logicalContainerName: string,
    containerPort: number,
    protocol: "tcp" | "udp",
    fallback: string,
  ): string {
    const containerName = resolveProjectContainerName(logicalContainerName) ?? logicalContainerName;
    const result = runCommand(
      ["docker", "port", containerName, `${containerPort}/${protocol}`],
      { capture: true, check: false, allowFailure: true },
    );
    if (result.status !== 0) {
      return fallback;
    }
    return parsePublishedHostPort(result.stdout) ?? fallback;
  }

  function parseComposePsJson(raw: string): ComposePsEntry[] {
    const trimmed = raw.trim();
    if (!trimmed) {
      return [];
    }

    const parseObject = (value: unknown): ComposePsEntry | null => {
      if (!value || typeof value !== "object") {
        return null;
      }
      return value as ComposePsEntry;
    };

    try {
      const parsed = JSON.parse(trimmed);
      if (Array.isArray(parsed)) {
        return parsed.map(parseObject).filter((entry): entry is ComposePsEntry => entry !== null);
      }
      const objectEntry = parseObject(parsed);
      return objectEntry ? [objectEntry] : [];
    } catch {
      // docker compose may output one JSON object per line depending on version
    }

    return trimmed
      .split("\n")
      .map((line) => line.trim())
      .filter(Boolean)
      .map((line) => {
        try {
          const parsed = JSON.parse(line);
          return parseObject(parsed);
        } catch {
          return null;
        }
      })
      .filter((entry): entry is ComposePsEntry => entry !== null);
  }

  function traceStatus(): void {
    const compose = tracingComposeFile();
    const result = runCommand(
      ["docker", "compose", "-p", PROJECT, "-f", compose, "ps", "--format", "json", "jaeger", "prometheus"],
      { capture: true, check: false, allowFailure: true },
    );

    if (result.status !== 0) {
      runCommand(["docker", "compose", "-p", PROJECT, "-f", compose, "ps", "jaeger", "prometheus"], { check: true });
      return;
    }

    const entries = parseComposePsJson(result.stdout);
    if (entries.length === 0) {
      runCommand(["docker", "compose", "-p", PROJECT, "-f", compose, "ps", "jaeger", "prometheus"], { check: true });
      return;
    }

    const jaeger = entries.find((entry) => entry.Service === "jaeger");
    const prometheus = entries.find((entry) => entry.Service === "prometheus");

    const jaegerQueryPort = resolvePublishedHostPort(
      "jaeger",
      JAEGER_QUERY_PORT,
      "tcp",
      process.env.JAEGER_QUERY_PORT?.trim() || String(JAEGER_QUERY_PORT),
    );
    const jaegerGrpcPort = resolvePublishedHostPort(
      "jaeger",
      JAEGER_OTLP_GRPC_PORT,
      "tcp",
      process.env.JAEGER_OTLP_GRPC_PORT?.trim() || String(JAEGER_OTLP_GRPC_PORT),
    );
    const jaegerHttpPort = resolvePublishedHostPort(
      "jaeger",
      JAEGER_OTLP_HTTP_PORT,
      "tcp",
      process.env.JAEGER_OTLP_HTTP_PORT?.trim() || String(JAEGER_OTLP_HTTP_PORT),
    );
    const prometheusPort = resolvePublishedHostPort(
      "prometheus",
      PROMETHEUS_PORT,
      "tcp",
      process.env.PROMETHEUS_PORT?.trim() || String(PROMETHEUS_PORT),
    );

    const printService = (name: string, entry?: ComposePsEntry): void => {
      if (!entry) {
        console.log(`- ${name}: not running`);
        return;
      }
      const status = entry.Status?.trim() || entry.State?.trim() || "unknown";
      console.log(`- ${name}: ${status}`);
    };

    console.log(`Tracing services for project '${PROJECT}':`);
    printService("jaeger", jaeger);
    if (jaeger) {
      console.log(`  UI: http://localhost:${jaegerQueryPort}`);
      console.log(`  OTLP gRPC: localhost:${jaegerGrpcPort}`);
      console.log(`  OTLP HTTP: localhost:${jaegerHttpPort}`);
    }
    printService("prometheus", prometheus);
    if (prometheus) {
      console.log(`  UI: http://localhost:${prometheusPort}`);
    }
  }

  function stackStatus(): void {
    runCommand(["docker", "compose", "-p", PROJECT, "ps"], { check: true });
  }

  function fetchJaegerServices(): string[] {
    const jaegerPort = resolvePublishedHostPort(
      "jaeger",
      JAEGER_QUERY_PORT,
      "tcp",
      process.env.JAEGER_QUERY_PORT?.trim() || String(JAEGER_QUERY_PORT),
    );
    const result = runCommand(["curl", "-fsS", `http://localhost:${jaegerPort}/api/services`], {
      capture: true,
      check: false,
      allowFailure: true,
    });

    if (result.status !== 0) {
      throw new Error(
        `Unable to query Jaeger services API at http://localhost:${jaegerPort}/api/services. Ensure tracing stack is running.`,
      );
    }

    const payload = result.stdout.trim();
    const payloadCandidates = [payload, payload.replace(/\\"/g, "\"")];
    let parsed: unknown;
    let parsedOk = false;
    for (const candidate of payloadCandidates) {
      try {
        parsed = JSON.parse(candidate);
        parsedOk = true;
        break;
      } catch {
        // Try normalized candidate next.
      }
    }
    if (!parsedOk) {
      throw new Error("Jaeger services API returned invalid JSON");
    }

    if (!parsed || typeof parsed !== "object" || !("data" in parsed) || !Array.isArray((parsed as { data: unknown }).data)) {
      throw new Error("Jaeger services API response does not contain a data array");
    }

    const services = (parsed as { data: unknown[] }).data.filter((entry): entry is string => typeof entry === "string");
    return services;
  }

  function runTelemetrySmokeCheck(strict: boolean): void {
    if (!isContainerRunningExact("jaeger")) {
      const message = "Jaeger container is not running. Start it with: ./fhevm-cli trace up";
      if (strict) {
        cliError("E_JAEGER_NOT_RUNNING", message);
      }
      logWarn(message);
      return;
    }

    const maxAttempts = TELEMETRY_SMOKE_MAX_ATTEMPTS;
    const retryDelaySeconds = TELEMETRY_SMOKE_RETRY_DELAY_SECONDS;
    let lastMessage = "";

    for (let attempt = 1; attempt <= maxAttempts; attempt += 1) {
      try {
        const services = fetchJaegerServices();
        const missing = TELEMETRY_REQUIRED_JAEGER_SERVICES.filter((service) => {
          const acceptedNames = [service, ...telemetryServiceAliases(service)];
          return !acceptedNames.some((candidate) => services.includes(candidate));
        });
        if (missing.length === 0) {
          logInfo(`Telemetry smoke check passed. Found services: ${TELEMETRY_REQUIRED_JAEGER_SERVICES.join(", ")}`);
          return;
        }
        lastMessage = `Missing Jaeger services: ${missing.join(", ")}`;
      } catch (error) {
        const message = error instanceof Error ? error.message : String(error);
        lastMessage = `Jaeger query failed: ${message}`;
      }

      if (attempt < maxAttempts) {
        logWarn(`Telemetry smoke attempt ${attempt}/${maxAttempts} not ready (${lastMessage}). Retrying in ${retryDelaySeconds}s...`);
        sleep(retryDelaySeconds);
        continue;
      }
    }

    const message = `Telemetry smoke check failed after ${maxAttempts} attempts. ${lastMessage}. Check OTEL_EXPORTER_OTLP_ENDPOINT and coprocessor/kms-connector startup logs.`;
    if (strict) {
      cliError("E_TELEMETRY_SMOKE_FAILED", message);
    }
    logWarn(message);
  }

  return {
    traceUp,
    traceDown,
    traceStatus,
    stackStatus,
    runTelemetrySmokeCheck,
  };
}
