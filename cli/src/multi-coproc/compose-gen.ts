import { mkdir, rename } from "fs/promises";

import { dirnameOf } from "../utils/path";

import type { CoprocessorInstance } from "./topology";

interface HealthCheckTemplate {
  test: string;
  interval: string;
  timeout: string;
  retries: number;
}

export interface CoprocessorComposeTemplate {
  suffix: string;
  dockerTarget: string;
  image: string;
  versionVar: string;
  command: string[];
  dependsOnMigration: boolean;
  isCacheWriter: boolean;
  environment?: string[];
  healthCheck?: HealthCheckTemplate;
  volumes?: string[];
}

export const COPROCESSOR_SERVICE_TEMPLATES: readonly CoprocessorComposeTemplate[] = [
  {
    suffix: "db-migration",
    dockerTarget: "db-migration",
    image: "ghcr.io/zama-ai/fhevm/coprocessor/db-migration",
    versionVar: "COPROCESSOR_DB_MIGRATION_VERSION",
    environment: ["KEY_ID=${FHE_KEY_ID}"],
    command: ["/initialize_db.sh"],
    dependsOnMigration: false,
    isCacheWriter: false,
    volumes: ["keys-cache:/fhevm-keys"],
  },
  {
    suffix: "host-listener",
    dockerTarget: "host-listener",
    image: "ghcr.io/zama-ai/fhevm/coprocessor/host-listener",
    versionVar: "COPROCESSOR_HOST_LISTENER_VERSION",
    command: [
      "host_listener",
      "--database-url=${DATABASE_URL}",
      "--acl-contract-address=${ACL_CONTRACT_ADDRESS}",
      "--tfhe-contract-address=${FHEVM_EXECUTOR_CONTRACT_ADDRESS}",
      "--url=${RPC_WS_URL}",
      "--initial-block-time=1",
    ],
    dependsOnMigration: true,
    isCacheWriter: false,
  },
  {
    suffix: "host-listener-poller",
    dockerTarget: "host-listener",
    image: "ghcr.io/zama-ai/fhevm/coprocessor/host-listener",
    versionVar: "COPROCESSOR_HOST_LISTENER_VERSION",
    command: [
      "host_listener_poller",
      "--database-url=${DATABASE_URL}",
      "--acl-contract-address=${ACL_CONTRACT_ADDRESS}",
      "--tfhe-contract-address=${FHEVM_EXECUTOR_CONTRACT_ADDRESS}",
      "--url=${RPC_HTTP_URL}",
    ],
    dependsOnMigration: true,
    isCacheWriter: false,
  },
  {
    suffix: "gw-listener",
    dockerTarget: "gw-listener",
    image: "ghcr.io/zama-ai/fhevm/coprocessor/gw-listener",
    versionVar: "COPROCESSOR_GW_LISTENER_VERSION",
    command: [
      "gw_listener",
      "--database-url=${DATABASE_URL}",
      "--database-pool-size=16",
      "--verify-proof-req-database-channel=event_zkpok_new_work",
      "--gw-url=${GATEWAY_WS_URL}",
      "--input-verification-address=${INPUT_VERIFICATION_ADDRESS}",
      "--kms-generation-address=${KMS_GENERATION_ADDRESS}",
      "--error-sleep-initial-secs=1",
      "--error-sleep-max-secs=10",
    ],
    dependsOnMigration: true,
    isCacheWriter: false,
    healthCheck: {
      test: "curl -f http://localhost:8080/liveness || exit 1",
      interval: "10s",
      timeout: "5s",
      retries: 3,
    },
  },
  {
    suffix: "tfhe-worker",
    dockerTarget: "tfhe-worker",
    image: "ghcr.io/zama-ai/fhevm/coprocessor/tfhe-worker",
    versionVar: "COPROCESSOR_TFHE_WORKER_VERSION",
    command: [
      "tfhe_worker",
      "--run-bg-worker",
      "--database-url=${DATABASE_URL}",
      "--pg-pool-max-connections=10",
      "--worker-polling-interval-ms=1000",
      "--work-items-batch-size=10",
      "--key-cache-size=32",
      "--coprocessor-fhe-threads=8",
      "--tokio-threads=4",
    ],
    dependsOnMigration: true,
    isCacheWriter: true,
  },
  {
    suffix: "zkproof-worker",
    dockerTarget: "zkproof-worker",
    image: "ghcr.io/zama-ai/fhevm/coprocessor/zkproof-worker",
    versionVar: "COPROCESSOR_ZKPROOF_WORKER_VERSION",
    command: [
      "zkproof_worker",
      "--database-url=${DATABASE_URL}",
      "--pg-listen-channel=event_zkpok_new_work",
      "--pg-notify-channel=event_zkpok_computed",
      "--pg-polling-interval=5",
      "--pg-pool-connections=5",
      "--worker-thread-count=4",
    ],
    dependsOnMigration: true,
    isCacheWriter: false,
  },
  {
    suffix: "sns-worker",
    dockerTarget: "sns-worker",
    image: "ghcr.io/zama-ai/fhevm/coprocessor/sns-worker",
    versionVar: "COPROCESSOR_SNS_WORKER_VERSION",
    command: [
      "sns_worker",
      "--database-url=${DATABASE_URL}",
      "--pg-listen-channels",
      "event_pbs_computations",
      "event_ciphertext_computed",
      "--pg-notify-channel",
      "event_ciphertext128_computed",
      "--work-items-batch-size=20",
      "--pg-polling-interval=30",
      "--pg-pool-connections=10",
      "--bucket-name-ct64=ct64",
      "--bucket-name-ct128=ct128",
      "--s3-max-concurrent-uploads=100",
      "--s3-max-retries-per-upload=100",
      "--s3-max-backoff=10s",
      "--s3-max-retries-timeout=120s",
      "--s3-recheck-duration=2s",
      "--s3-regular-recheck-duration=120s",
      "--enable-compression",
    ],
    dependsOnMigration: true,
    isCacheWriter: false,
  },
  {
    suffix: "transaction-sender",
    dockerTarget: "transaction-sender",
    image: "ghcr.io/zama-ai/fhevm/coprocessor/tx-sender",
    versionVar: "COPROCESSOR_TX_SENDER_VERSION",
    command: [
      "transaction_sender",
      "--database-url=${DATABASE_URL}",
      "--gateway-url=${GATEWAY_WS_URL}",
      "--private-key=${TX_SENDER_PRIVATE_KEY}",
      "--ciphertext-commits-address=${CIPHERTEXT_COMMITS_ADDRESS}",
      "--input-verification-address=${INPUT_VERIFICATION_ADDRESS}",
      "--multichain-acl-address=${MULTICHAIN_ACL_ADDRESS}",
      "--database-pool-size=10",
      "--database-polling-interval-secs=20",
      "--verify-proof-resp-database-channel=event_zkpok_computed",
      "--add-ciphertexts-database-channel=event_ciphertexts_uploaded",
      "--verify-proof-resp-batch-limit=128",
      "--verify-proof-resp-max-retries=15",
      "--verify-proof-remove-after-max-retries",
      "--signer-type=private-key",
      "--host-chain-url=${RPC_WS_URL}",
      "--delegation-block-delay=10",
      "--delegation-clear-after-n-blocks=648000",
      "--delegation-fallback-polling=30",
      "--delegation-max-retry=100000",
      "--retry-immediately-on-nonce-error=2",
    ],
    dependsOnMigration: true,
    isCacheWriter: false,
  },
] as const;

function renderService(instance: CoprocessorInstance, template: CoprocessorComposeTemplate): string {
  const serviceName = `${instance.servicePrefix}-${template.suffix}`;
  const migrationService = `${instance.servicePrefix}-db-migration`;
  const envFilePath = `../env/${instance.envFileName}.env`;
  const lines: string[] = [
    `  ${serviceName}:`,
    `    image: ${template.image}:\${${template.versionVar}}`,
    "    build:",
    "      context: ../../..",
    "      dockerfile: coprocessor/fhevm-engine/Dockerfile.workspace",
    `      target: ${template.dockerTarget}`,
    "      args:",
    "        CARGO_PROFILE: ${FHEVM_CARGO_PROFILE:-release}",
    "      cache_from:",
    "        - ${FHEVM_CACHE_FROM_COPROCESSOR:-type=gha}",
  ];

  if (template.isCacheWriter && instance.index === 0) {
    lines.push("      cache_to:", "        - ${FHEVM_CACHE_TO_COPROCESSOR:-type=gha,mode=max}");
  }

  if (template.healthCheck) {
    lines.push(
      "    healthcheck:",
      `      test: [\"CMD-SHELL\", \"${template.healthCheck.test}\"]`,
      `      interval: ${template.healthCheck.interval}`,
      `      timeout: ${template.healthCheck.timeout}`,
      `      retries: ${template.healthCheck.retries}`,
    );
  }

  lines.push("    env_file:", `      - ${envFilePath}`);

  if (!instance.usesBaseCompose && template.suffix === "gw-listener") {
    lines.push("    ports:", `      - ${instance.gwListenerPort}:8080`);
  }

  if (template.environment?.length) {
    lines.push("    environment:");
    for (const value of template.environment) {
      lines.push(`      - ${value}`);
    }
  }

  if (template.command.length > 0) {
    lines.push("    command:");
    for (const commandPart of template.command) {
      lines.push(`      - ${commandPart}`);
    }
  }

  if (template.dependsOnMigration) {
    lines.push(
      "    depends_on:",
      `      ${migrationService}:`,
      "        condition: service_completed_successfully",
    );
  }

  if (template.volumes?.length) {
    lines.push("    volumes:");
    for (const volume of template.volumes) {
      lines.push(`      - ${volume}`);
    }
  }

  return lines.join("\n");
}

export function generateComposeYaml(instance: CoprocessorInstance): string {
  const services = COPROCESSOR_SERVICE_TEMPLATES.map((template) => renderService(instance, template)).join(
    "\n\n",
  );

  return [
    `# Auto-generated by fhevm-cli for coprocessor instance ${instance.displayIndex}`,
    "# Do not edit. Regenerated by `fhevm-cli up`.",
    "services:",
    services,
    "",
    "volumes:",
    "  keys-cache:",
    "    external: true",
    "",
  ].join("\n");
}

export async function writeComposeFile(instance: CoprocessorInstance): Promise<string> {
  if (instance.usesBaseCompose) {
    return instance.composeFile;
  }

  await mkdir(dirnameOf(instance.composeFile), { recursive: true });
  const content = generateComposeYaml(instance);
  const tmpPath = `${instance.composeFile}.tmp.${Date.now()}`;
  await Bun.write(tmpPath, content);
  await rename(tmpPath, instance.composeFile);
  return instance.composeFile;
}
