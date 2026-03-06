import { mkdir, rename } from "fs/promises";
import { ExitCode, FhevmCliError } from "../errors";
import type { FhevmConfig } from "./model";

export const DEFAULT_RELAYER_TEMPLATE_PATH = "test-suite/fhevm/config/relayer/local.yaml";
export const DEFAULT_RELAYER_OUTPUT_PATH = ".fhevm/config/relayer/local.yaml";

function dirnameOf(filePath: string): string {
  const index = filePath.lastIndexOf("/");
  if (index === -1) {
    return ".";
  }
  return filePath.slice(0, index) || ".";
}

function formatDbUrl(config: FhevmConfig): string {
  return `postgresql://${config.db.user}:${config.db.password}@${config.db.relayerHost}:${config.db.relayerPort}/${config.db.relayerDb}`;
}

function requireAddress(name: string, value: string | undefined): string {
  if (value) {
    return value;
  }
  throw new FhevmCliError({
    exitCode: ExitCode.CONFIG,
    step: "relayer-config",
    message: `missing ${name} contract address`,
  });
}

function replaceLine(yaml: string, pattern: RegExp, value: string, label: string): string {
  if (!pattern.test(yaml)) {
    throw new FhevmCliError({
      exitCode: ExitCode.CONFIG,
      step: "relayer-config",
      message: `unable to patch ${label} in relayer config template`,
    });
  }
  return yaml.replace(pattern, (_match, prefix: string, suffix: string) => `${prefix}${value}${suffix}`);
}

export function renderRelayerConfig(template: string, config: FhevmConfig): string {
  const decryptionAddress = requireAddress("decryption", config.contracts.decryption);
  const inputVerificationAddress = requireAddress("inputVerification", config.contracts.inputVerification);

  let output = template;
  output = replaceLine(
    output,
    /^(\s*http_url:\s*").*(".*)$/m,
    config.rpc.gatewayHttp,
    "gateway.blockchain_rpc.http_url",
  );
  output = replaceLine(
    output,
    /^(\s*read_http_url:\s*").*(".*)$/m,
    config.rpc.gatewayHttp,
    "gateway.blockchain_rpc.read_http_url",
  );
  output = replaceLine(output, /^(\s*chain_id:\s*)\d+(\s*)$/m, String(config.chainIds.gateway), "gateway.chain_id");
  output = replaceLine(
    output,
    /^(\s*decryption_address:\s*").*(".*)$/m,
    decryptionAddress,
    "gateway.contracts.decryption_address",
  );
  output = replaceLine(
    output,
    /^(\s*input_verification_address:\s*").*(".*)$/m,
    inputVerificationAddress,
    "gateway.contracts.input_verification_address",
  );
  output = replaceLine(
    output,
    /^(\s*sql_database_url:\s*").*(".*)$/m,
    formatDbUrl(config),
    "storage.sql_database_url",
  );
  output = replaceLine(
    output,
    /^(\s*private_key:\s*)0x[a-fA-F0-9]+(\s*)$/m,
    config.keys.txSender.privateKey,
    "tx_engine.private_key",
  );
  output = output.replaceAll('url: "ws://gateway-node:8546"', `url: "${config.rpc.gatewayWs}"`);

  // Patch keyurl URLs to point to MinIO where the actual key files are stored.
  // The template defaults point to the relayer-db-migration container (port 3001)
  // which exits after running migrations, making the key URLs unreachable.
  const bucket = config.minio.buckets.public;
  const minioBase = `http://fhevm-minio:${config.ports.minioApi}/${bucket}`;

  if (config.runtime.fheKeyId) {
    const publicKeyUrl = `${minioBase}/PUB/PublicKey/${config.runtime.fheKeyId}`;
    output = replaceLine(
      output,
      /^(\s*url:\s*")[^"]*publicKey[^"]*(".*)$/im,
      publicKeyUrl,
      "keyurl.fhe_public_key.url",
    );
  }

  if (config.runtime.crsKeyId) {
    const crsUrl = `${minioBase}/PUB/CRS/${config.runtime.crsKeyId}`;
    output = replaceLine(
      output,
      /^(\s*url:\s*")[^"]*crs[^"]*(".*)$/im,
      crsUrl,
      "keyurl.crs.url",
    );
  }

  return output;
}

export interface RelayerConfigPaths {
  templatePath?: string;
  outputPath?: string;
}

export async function generateRelayerConfigFile(
  config: FhevmConfig,
  paths: RelayerConfigPaths = {},
): Promise<string> {
  const templatePath = paths.templatePath ?? DEFAULT_RELAYER_TEMPLATE_PATH;
  const outputPath = paths.outputPath ?? DEFAULT_RELAYER_OUTPUT_PATH;
  const template = await Bun.file(templatePath).text();
  const content = renderRelayerConfig(template, config);
  await mkdir(dirnameOf(outputPath), { recursive: true });
  const tmpPath = `${outputPath}.tmp.${Date.now()}`;
  await Bun.write(tmpPath, content);
  await rename(tmpPath, outputPath);
  return outputPath;
}
