import { TEST_TYPE_CONFIG } from "./manifest";
import type { CommandDeps } from "./command-contracts";
import type { TestOptions } from "./types";

type RelayerKeyUrls = {
  publicKeyUrl: string;
  crsUrl: string;
};

const TEST_SUITE_CONTAINER = "fhevm-test-suite-e2e-debug";
const RELAYER_KEYURL_DEFAULT_URL = "http://relayer:3000/v2";
// Local stack currently serves a single CRS key-size entry at 2048 bits.
const RELAYER_CRS_KEY_SIZE = "2048";
const RELAYER_KEYURL_READY_MAX_ATTEMPTS = 24;
const RELAYER_KEYURL_READY_RETRY_DELAY_SECONDS = 5;
const RELAYER_KEYURL_PROBE_TIMEOUT_SECONDS = 5;
const DEFAULT_PROOF_RETRY_ATTEMPTS = 3;
const DEFAULT_PROOF_RETRY_DELAY_SECONDS = 10;

function rewriteUrlHost(url: string, host: string): string {
  try {
    const parsed = new URL(url);
    parsed.hostname = host;
    return parsed.toString();
  } catch {
    return url;
  }
}

function buildRelayerKeyUrlEndpoint(relayerBaseUrl: string): string {
  const base = relayerBaseUrl.trim().replace(/\/+$/, "");
  if (base.endsWith("/v2/keyurl")) {
    return base;
  }
  return `${base}/keyurl`;
}

function parseRelayerKeyUrls(payloadRaw: string): RelayerKeyUrls | undefined {
  const payload = payloadRaw.trim();
  if (!payload) {
    return undefined;
  }

  const payloadCandidates = [payload, payload.replace(/\\"/g, "\"")];
  for (const candidate of payloadCandidates) {
    try {
      const parsed = JSON.parse(candidate) as {
        response?: {
          fheKeyInfo?: Array<{ fhePublicKey?: { urls?: string[] } }>;
          crs?: Record<string, { urls?: string[] }>;
        };
      };

      const publicKeyUrl = parsed.response?.fheKeyInfo?.[0]?.fhePublicKey?.urls?.[0];
      const crsMap = parsed.response?.crs ?? {};
      const crsUrl = crsMap[RELAYER_CRS_KEY_SIZE]?.urls?.[0] ?? Object.values(crsMap)[0]?.urls?.[0];
      if (typeof publicKeyUrl === "string" && publicKeyUrl.trim() !== "" && typeof crsUrl === "string" && crsUrl.trim() !== "") {
        return { publicKeyUrl: publicKeyUrl.trim(), crsUrl: crsUrl.trim() };
      }
    } catch {
      // Keep trying other payload candidates.
    }
  }

  return undefined;
}

function parseTestArgs(args: string[], usageError: (message: string) => never): { testType: string; options: TestOptions } {
  const testType = args[0] ?? "";
  const options: TestOptions = {
    verbose: false,
    network: "staging",
    noRelayer: false,
    noHardhatCompile: false,
  };

  let i = 1;
  while (i < args.length) {
    const arg = args[i];

    if (arg === "-v" || arg === "--verbose") {
      options.verbose = true;
      i += 1;
      continue;
    }

    if (arg === "-n" || arg === "--network") {
      const value = args[i + 1];
      if (!value || value.startsWith("-")) {
        usageError("Network argument missing");
      }
      options.network = value;
      i += 2;
      continue;
    }

    if (arg === "-g" || arg === "--grep") {
      const value = args[i + 1];
      if (!value || value.startsWith("-")) {
        usageError("Grep pattern missing");
      }
      options.grep = value;
      i += 2;
      continue;
    }

    if (arg === "-r" || arg === "--no-relayer") {
      options.noRelayer = true;
      i += 1;
      continue;
    }

    if (arg === "--no-hardhat-compile") {
      options.noHardhatCompile = true;
      i += 1;
      continue;
    }

    usageError(`Unknown option: ${arg}`);
  }

  return { testType, options };
}

function runHardhatTestWithProofRetry(
  runCommand: CommandDeps["runCommand"],
  sleep: (seconds: number) => void,
  logWarn: (message: string) => void,
  command: string[],
  maxAttempts: number,
  retryDelaySeconds: number,
): void {
  const attempts = Math.max(1, maxAttempts);

  for (let attempt = 1; attempt <= attempts; attempt += 1) {
    const result = runCommand(command, { capture: true, check: false, allowFailure: true });
    if (result.stdout) {
      process.stdout.write(result.stdout);
    }
    if (result.stderr) {
      process.stderr.write(result.stderr);
    }

    if (result.status === 0) {
      return;
    }

    const combinedOutput = [result.stdout.trim(), result.stderr.trim()].filter(Boolean).join("\n");
    const isProofRejected = /Proof Rejected/i.test(combinedOutput);
    if (isProofRejected && attempt < attempts) {
      logWarn(
        `Input-proof compute/decrypt test hit transient proof rejection. Retrying in ${retryDelaySeconds}s... (${attempt}/${attempts})`,
      );
      sleep(retryDelaySeconds);
      continue;
    }

    const cmd = command.join(" ");
    throw new Error(`Command failed (${result.status}): ${cmd}${combinedOutput ? `\n${combinedOutput}` : ""}`);
  }
}

export function createTestHandlers(deps: CommandDeps) {
  const {
    PROJECT,
    COLORS,
    runCommand,
    sleep,
    usageError,
    logWarn,
    color,
    resolveProjectContainerName,
  } = deps;

  function isResolvedContainerRunning(containerName: string): boolean {
    const result = runCommand(["docker", "ps", "--filter", `name=^${containerName}$`, "--format", "{{.Names}}"], {
      capture: true,
      check: true,
    });

    return result.stdout
      .split("\n")
      .map((line) => line.trim())
      .filter(Boolean)
      .includes(containerName);
  }

  function resolveRunningTestSuiteContainer(): string {
    const resolvedContainer = resolveProjectContainerName(TEST_SUITE_CONTAINER) ?? TEST_SUITE_CONTAINER;
    if (!isResolvedContainerRunning(resolvedContainer)) {
      throw new Error(
        `Test container '${resolvedContainer}' is not running for project '${PROJECT}'. Start stack first: bun run up`,
      );
    }
    return resolvedContainer;
  }

  function waitForRelayerKeyUrlsReady(): void {
    const maxAttempts = RELAYER_KEYURL_READY_MAX_ATTEMPTS;
    const retryDelaySeconds = RELAYER_KEYURL_READY_RETRY_DELAY_SECONDS;
    let lastMessage = "not started";

    for (let attempt = 1; attempt <= maxAttempts; attempt += 1) {
      const relayerContainer = resolveProjectContainerName("fhevm-relayer");
      const testSuiteContainer = resolveProjectContainerName(TEST_SUITE_CONTAINER);
      if (!relayerContainer || !isResolvedContainerRunning(relayerContainer)) {
        lastMessage = "fhevm-relayer container is not running";
      } else if (!testSuiteContainer || !isResolvedContainerRunning(testSuiteContainer)) {
        lastMessage = `${TEST_SUITE_CONTAINER} container is not running`;
      } else {
        const relayerUrlResult = runCommand(
          ["docker", "exec", testSuiteContainer, "sh", "-lc", "printf '%s' \"${RELAYER_URL:-}\""],
          { capture: true, check: false, allowFailure: true },
        );
        const relayerBaseUrl = relayerUrlResult.status === 0 && relayerUrlResult.stdout.trim() !== ""
          ? relayerUrlResult.stdout.trim()
          : RELAYER_KEYURL_DEFAULT_URL;

        let parsedRelayerUrl: URL;
        try {
          parsedRelayerUrl = new URL(relayerBaseUrl);
        } catch {
          lastMessage = `invalid RELAYER_URL in ${TEST_SUITE_CONTAINER}: ${relayerBaseUrl}`;
          if (attempt < maxAttempts) {
            logWarn(`Relayer keyurl readiness not met (${lastMessage}). Retrying in ${retryDelaySeconds}s... (${attempt}/${maxAttempts})`);
            sleep(retryDelaySeconds);
            continue;
          }
          break;
        }

        const keyurlEndpoint = buildRelayerKeyUrlEndpoint(parsedRelayerUrl.toString());
        const keyurlResult = runCommand(
          ["docker", "exec", testSuiteContainer, "curl", "-fsS", keyurlEndpoint],
          { capture: true, check: false, allowFailure: true },
        );

        if (keyurlResult.status !== 0) {
          const details = [keyurlResult.stdout.trim(), keyurlResult.stderr.trim()].filter(Boolean).join(" ");
          lastMessage = `failed to query ${keyurlEndpoint}: ${details || "unknown error"}`;
        } else {
          const resolvedKeyUrls = parseRelayerKeyUrls(keyurlResult.stdout);
          if (!resolvedKeyUrls) {
            lastMessage = `relayer keyurl payload from ${keyurlEndpoint} is missing required public key / CRS URLs`;
          } else {
            const unreachable = [resolvedKeyUrls.publicKeyUrl, resolvedKeyUrls.crsUrl].filter((url) => {
              const urlCandidates = [url];

              if (parsedRelayerUrl.hostname !== "") {
                const rewritten = rewriteUrlHost(url, parsedRelayerUrl.hostname);
                if (rewritten !== url) {
                  urlCandidates.push(rewritten);
                }
              }

              const reachable = urlCandidates.some((candidateUrl) => {
                const probe = runCommand(
                  [
                    "docker",
                    "exec",
                    testSuiteContainer,
                    "curl",
                    "-fsS",
                    "--max-time",
                    String(RELAYER_KEYURL_PROBE_TIMEOUT_SECONDS),
                    "-o",
                    "/dev/null",
                    candidateUrl,
                  ],
                  { capture: true, check: false, allowFailure: true },
                );
                return probe.status === 0;
              });

              if (reachable) {
                return false;
              }
              return true;
            });

            if (unreachable.length === 0) {
              return;
            }

            lastMessage = `unreachable key URLs: ${unreachable.join(", ")}`;
          }
        }
      }

      if (attempt < maxAttempts) {
        logWarn(`Relayer keyurl readiness not met (${lastMessage}). Retrying in ${retryDelaySeconds}s... (${attempt}/${maxAttempts})`);
        sleep(retryDelaySeconds);
      }
    }

    throw new Error(
      `Relayer key URLs are not reachable after ${maxAttempts} attempts (${lastMessage}). Check test-suite RELAYER_URL and relayer keyurl config, then rerun: bun run up --resume relayer`,
    );
  }

  function runTests(args: string[]): void {
    const { testType, options } = parseTestArgs(args, usageError);

    const config = TEST_TYPE_CONFIG[testType];
    if (!config) {
      usageError(`Unknown test type: ${testType}`);
    }

    if (config.debugShell) {
      const testSuiteContainer = resolveRunningTestSuiteContainer();
      console.log(color("[DEBUG] Starting debug session...", `${COLORS.lightBlue}${COLORS.bold}`));
      runCommand(["docker", "exec", "-it", testSuiteContainer, "bash"], { check: true });
      return;
    }

    const grepPattern = options.grep ?? config.grep;
    const shouldRunParallel = Boolean(config.parallel);
    const logMessage = config.logMessage ? color(config.logMessage, `${COLORS.lightBlue}${COLORS.bold}`) : "";

    if (logMessage) {
      console.log(logMessage);
    }

    if (!options.noRelayer) {
      waitForRelayerKeyUrlsReady();
    }

    const testSuiteContainer = resolveRunningTestSuiteContainer();
    const dockerExecPrefix = ["docker", "exec"];
    if (options.noRelayer) {
      dockerExecPrefix.push("-e", "NO_RELAYER=true");
    }
    dockerExecPrefix.push(testSuiteContainer);

    if (options.noHardhatCompile) {
      const hardhatArgs = ["npx", "hardhat", "test"];
      if (shouldRunParallel) {
        hardhatArgs.push("--parallel");
      }
      if (options.verbose) {
        hardhatArgs.push("--verbose");
      }
      hardhatArgs.push("--no-compile");
      if (grepPattern) {
        hardhatArgs.push("--grep", grepPattern);
      }
      hardhatArgs.push("--network", options.network);
      const hardhatCommand = [...dockerExecPrefix, ...hardhatArgs];
      if (config.retryOnProofRejected) {
        runHardhatTestWithProofRetry(
          runCommand,
          sleep,
          logWarn,
          hardhatCommand,
          config.retryAttempts ?? DEFAULT_PROOF_RETRY_ATTEMPTS,
          config.retryDelaySeconds ?? DEFAULT_PROOF_RETRY_DELAY_SECONDS,
        );
        return;
      }
      runCommand(hardhatCommand, { check: true });
      return;
    }

    const dockerArgs: string[] = ["./run-tests.sh"];
    if (options.verbose) {
      dockerArgs.push("-v");
    }
    dockerArgs.push("-n", options.network);
    if (shouldRunParallel) {
      dockerArgs.push("--parallel");
    }
    if (grepPattern) {
      dockerArgs.push("-g", grepPattern);
    }

    runCommand([...dockerExecPrefix, ...dockerArgs], { check: true });
  }

  return { runTests };
}
