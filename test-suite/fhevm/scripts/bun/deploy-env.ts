import fs from "node:fs";
import path from "node:path";
import type { HostPortSpec } from "./types";

type DeployEnvHandlersDeps = {
  CONFIG_DIR: string;
  DEFAULT_OTEL_EXPORTER_OTLP_ENDPOINT: string;
  defaultRelayerPublicKeyId: string;
  defaultRelayerCrsId: string;
  HOST_PORT_SPECS: HostPortSpec[];
  baseEnvFile: (component: string) => string;
  localEnvFile: (component: string) => string;
  uniqueComponents: () => string[];
  uniqueVersionEnvVars: () => string[];
  upsertEnvValue: (filePath: string, key: string, value: string) => void;
  readEnvValue: (filePath: string, key: string) => string | undefined;
  trimTrailingSlashes: (value: string) => string;
  buildKmsPublicObjectPrefix: (rawPrefix: string | undefined) => string;
  extractTrailingPathSegment: (url: string | undefined, fallback: string) => string;
  isContainerRunningExact: (containerName: string) => boolean;
  cliCommand: (args?: string) => string;
  cliError: (code: string, message: string, options?: { showUsage?: boolean }) => never;
  logInfo: (message: string) => void;
  logWarn: (message: string) => void;
};

export function createDeployEnvHandlers(deps: DeployEnvHandlersDeps) {
  const {
    CONFIG_DIR,
    DEFAULT_OTEL_EXPORTER_OTLP_ENDPOINT,
    defaultRelayerPublicKeyId,
    defaultRelayerCrsId,
    HOST_PORT_SPECS,
    baseEnvFile,
    localEnvFile,
    uniqueComponents,
    uniqueVersionEnvVars,
    upsertEnvValue,
    readEnvValue,
    trimTrailingSlashes,
    buildKmsPublicObjectPrefix,
    extractTrailingPathSegment,
    isContainerRunningExact,
    cliCommand,
    cliError,
    logInfo,
    logWarn,
  } = deps;

  function prepareLocalEnvFile(component: string): string {
    const baseFile = baseEnvFile(component);
    const localFile = localEnvFile(component);

    if (!fs.existsSync(baseFile)) {
      throw new Error(`Base environment file for ${component} not found: ${baseFile}`);
    }

    logInfo(`Creating/updating local environment file for ${component}...`);
    fs.copyFileSync(baseFile, localFile);
    return localFile;
  }

  function prepareLocalConfigRelayer(): string {
    const baseFile = path.resolve(CONFIG_DIR, "relayer", "local.yaml");
    const localFile = path.resolve(CONFIG_DIR, "relayer", "local.yaml.local");

    if (!fs.existsSync(baseFile)) {
      throw new Error(`Base configuration file for relayer not found: ${baseFile}`);
    }

    logInfo("Creating/updating local configuration file for relayer...");
    fs.copyFileSync(baseFile, localFile);
    return localFile;
  }

  function prepareAllEnvFiles(): void {
    logInfo("Preparing all local environment files...");

    for (const component of uniqueComponents()) {
      prepareLocalEnvFile(component);
    }

    logInfo("All local environment files prepared successfully");
  }

  function syncVersionEnvVarsIntoLocalEnvFiles(): void {
    const versionKeys = uniqueVersionEnvVars();

    for (const component of uniqueComponents()) {
      const envFile = localEnvFile(component);
      if (!fs.existsSync(envFile)) {
        continue;
      }

      for (const key of versionKeys) {
        const value = process.env[key];
        if (!value || value.trim() === "") {
          continue;
        }
        upsertEnvValue(envFile, key, value.trim());
      }
    }
  }

  function syncHostPortEnvVarsIntoLocalEnvFiles(): void {
    const hostPortVars = HOST_PORT_SPECS.map((spec) => spec.envVar);

    for (const component of uniqueComponents()) {
      const envFile = localEnvFile(component);
      if (!fs.existsSync(envFile)) {
        continue;
      }

      for (const key of hostPortVars) {
        const value = process.env[key];
        if (!value || value.trim() === "") {
          continue;
        }
        upsertEnvValue(envFile, key, value.trim());
      }
    }
  }

  function ensureCoprocessorTelemetryEnv(validateReachability: boolean): void {
    const coprocessorLocal = localEnvFile("coprocessor");
    if (!fs.existsSync(coprocessorLocal)) {
      throw new Error(`Coprocessor local env file not found: ${coprocessorLocal}`);
    }

    const key = "OTEL_EXPORTER_OTLP_ENDPOINT";
    let endpoint = readEnvValue(coprocessorLocal, key);

    if (!endpoint) {
      endpoint = DEFAULT_OTEL_EXPORTER_OTLP_ENDPOINT;
      upsertEnvValue(coprocessorLocal, key, endpoint);
      logWarn(`Missing ${key} in ${path.basename(coprocessorLocal)}. Defaulting to ${endpoint}.`);
    }

    let parsed: URL;
    try {
      parsed = new URL(endpoint);
    } catch {
      throw new Error(`Invalid ${key} value in ${path.basename(coprocessorLocal)}: ${endpoint}`);
    }

    if (validateReachability && parsed.hostname === "jaeger" && !isContainerRunningExact("jaeger")) {
      cliError(
        "E_OTEL_JAEGER_REQUIRED",
        `Telemetry endpoint ${endpoint} is configured but Jaeger is not running. Start tracing first: ${cliCommand("trace up")}`,
      );
    }
  }

  function ensureCoprocessorLegacyEnvCompatibility(): void {
    const coprocessorLocal = localEnvFile("coprocessor");
    if (!fs.existsSync(coprocessorLocal)) {
      throw new Error(`Coprocessor local env file not found: ${coprocessorLocal}`);
    }

    const legacyInputVerifier = readEnvValue(coprocessorLocal, "INPUT_VERIFIER_ADDRESS");
    const currentInputVerifier = readEnvValue(coprocessorLocal, "INPUT_VERIFICATION_ADDRESS");
    if ((!legacyInputVerifier || legacyInputVerifier.trim() === "") && currentInputVerifier && currentInputVerifier.trim() !== "") {
      upsertEnvValue(coprocessorLocal, "INPUT_VERIFIER_ADDRESS", currentInputVerifier.trim());
    }
  }

  function ensureKmsConnectorLegacyEnvCompatibility(): void {
    const kmsConnectorLocal = localEnvFile("kms-connector");
    if (!fs.existsSync(kmsConnectorLocal)) {
      throw new Error(`KMS connector local env file not found: ${kmsConnectorLocal}`);
    }

    const chainId = readEnvValue(kmsConnectorLocal, "KMS_CONNECTOR_CHAIN_ID");
    if (chainId && chainId.trim() !== "") {
      return;
    }

    const gatewayChainId = readEnvValue(kmsConnectorLocal, "KMS_CONNECTOR_GATEWAY_CHAIN_ID");
    if (gatewayChainId && gatewayChainId.trim() !== "") {
      upsertEnvValue(kmsConnectorLocal, "KMS_CONNECTOR_CHAIN_ID", gatewayChainId.trim());
    }
  }

  function ensureRelayerKeyUrlEnvCompatibility(): void {
    const coreLocal = localEnvFile("core");
    const relayerLocal = localEnvFile("relayer");

    if (!fs.existsSync(coreLocal)) {
      throw new Error(`Core local env file not found: ${coreLocal}`);
    }

    if (!fs.existsSync(relayerLocal)) {
      throw new Error(`Relayer local env file not found: ${relayerLocal}`);
    }

    const endpoint = trimTrailingSlashes(readEnvValue(coreLocal, "S3_ENDPOINT") ?? "http://minio:9000");
    const bucket = (readEnvValue(coreLocal, "KMS_CORE__PUBLIC_VAULT__STORAGE__S3__BUCKET") ?? "kms-public").trim();
    const rawPrefix = readEnvValue(coreLocal, "KMS_CORE__PUBLIC_VAULT__STORAGE__S3__PREFIX");
    const objectPrefix = buildKmsPublicObjectPrefix(rawPrefix);

    const currentPublicKeyUrl = readEnvValue(relayerLocal, "APP_KEYURL__FHE_PUBLIC_KEY__URL");
    const currentCrsUrl = readEnvValue(relayerLocal, "APP_KEYURL__CRS__URL");
    const publicKeyId = extractTrailingPathSegment(currentPublicKeyUrl, defaultRelayerPublicKeyId);
    const crsId = extractTrailingPathSegment(currentCrsUrl, defaultRelayerCrsId);

    const nextPublicKeyUrl = `${endpoint}/${bucket}/${objectPrefix}/PublicKey/${publicKeyId}`;
    const nextCrsUrl = `${endpoint}/${bucket}/${objectPrefix}/CRS/${crsId}`;

    upsertEnvValue(relayerLocal, "APP_KEYURL__FHE_PUBLIC_KEY__URL", nextPublicKeyUrl);
    upsertEnvValue(relayerLocal, "APP_KEYURL__CRS__URL", nextCrsUrl);
  }

  function ensureTestSuiteRelayerUrlEnvCompatibility(): void {
    const testSuiteLocal = localEnvFile("test-suite");
    if (!fs.existsSync(testSuiteLocal)) {
      throw new Error(`Test-suite local env file not found: ${testSuiteLocal}`);
    }

    const key = "RELAYER_URL";
    const defaultRelayerUrl = "http://relayer:3000/v2";
    const current = readEnvValue(testSuiteLocal, key);
    if (!current || current.trim() === "") {
      upsertEnvValue(testSuiteLocal, key, defaultRelayerUrl);
      logWarn(`Missing ${key} in ${path.basename(testSuiteLocal)}. Defaulting to ${defaultRelayerUrl}.`);
    } else {
      let parsed: URL;
      try {
        parsed = new URL(current);
      } catch {
        throw new Error(`Invalid ${key} value in ${path.basename(testSuiteLocal)}: ${current}`);
      }

      if (parsed.hostname === "fhevm-relayer") {
        parsed.hostname = "relayer";
        upsertEnvValue(testSuiteLocal, key, parsed.toString());
        logWarn(`Updated legacy ${key} hostname in ${path.basename(testSuiteLocal)} from fhevm-relayer to relayer.`);
      }
    }
  }

  return {
    prepareAllEnvFiles,
    prepareLocalConfigRelayer,
    syncVersionEnvVarsIntoLocalEnvFiles,
    syncHostPortEnvVarsIntoLocalEnvFiles,
    ensureCoprocessorTelemetryEnv,
    ensureCoprocessorLegacyEnvCompatibility,
    ensureKmsConnectorLegacyEnvCompatibility,
    ensureRelayerKeyUrlEnvCompatibility,
    ensureTestSuiteRelayerUrlEnvCompatibility,
  };
}
