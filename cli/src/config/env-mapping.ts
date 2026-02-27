import type { FhevmConfig } from "./model";
import type { EnvFileName } from "./service-map";

export type EnvGenerator = (config: FhevmConfig) => Record<string, string>;

export interface CoprocessorEnvOverrides {
  databaseUrl?: string;
  txSenderPrivateKey?: string;
}

function formatDbUrl(config: FhevmConfig, host: string, port: number, database: string): string {
  return `postgresql://${config.db.user}:${config.db.password}@${host}:${port}/${database}`;
}

function valueOrEmpty(value: string | undefined): string {
  return value ?? "";
}

function kmsSignerAt(config: FhevmConfig, index: number): string {
  if (index === 0 && config.runtime.kmsSigner) {
    return config.runtime.kmsSigner;
  }
  return valueOrEmpty(config.keys.kmsNodes[index]?.signer.address);
}

function hostChainRecords(config: FhevmConfig): string {
  return JSON.stringify([
    {
      url: config.rpc.hostHttp,
      chain_id: config.chainIds.host,
      acl_address: valueOrEmpty(config.contracts.acl),
    },
  ]);
}

export function generateCoprocessorEnvWithOverrides(
  config: FhevmConfig,
  overrides: CoprocessorEnvOverrides = {},
): Record<string, string> {
  const txSender =
    overrides.txSenderPrivateKey ??
    config.keys.coprocessors[0]?.txSender.privateKey ??
    config.keys.txSender.privateKey;
  const databaseUrl =
    overrides.databaseUrl ??
    formatDbUrl(config, config.db.host, config.db.port, config.db.coprocessorDb);
  const awsEndpoint = config.runtime.minioIp
    ? `http://${config.runtime.minioIp}:${config.ports.minioApi}`
    : config.minio.endpoint;

  return {
    DATABASE_URL: databaseUrl,
    ACL_CONTRACT_ADDRESS: valueOrEmpty(config.contracts.acl),
    FHEVM_EXECUTOR_CONTRACT_ADDRESS: valueOrEmpty(config.contracts.fhevmExecutor),
    RPC_WS_URL: config.rpc.hostWs,
    RPC_HTTP_URL: config.rpc.hostHttp,
    GATEWAY_WS_URL: config.rpc.gatewayWs,
    INPUT_VERIFICATION_ADDRESS: valueOrEmpty(config.contracts.inputVerification),
    KMS_GENERATION_ADDRESS: valueOrEmpty(config.contracts.kmsGeneration),
    TX_SENDER_PRIVATE_KEY: txSender,
    CIPHERTEXT_COMMITS_ADDRESS: valueOrEmpty(config.contracts.ciphertextCommits),
    MULTICHAIN_ACL_ADDRESS: valueOrEmpty(config.contracts.multichainAcl),
    FHE_KEY_ID: valueOrEmpty(config.runtime.fheKeyId),
    AWS_ENDPOINT_URL: awsEndpoint,
    AWS_ACCESS_KEY_ID: config.minio.accessKey,
    AWS_SECRET_ACCESS_KEY: config.minio.secretKey,
    AWS_REGION: config.minio.region,
    RUST_LOG: "info",
    CHAIN_ID: String(config.chainIds.host),
    // Legacy env vars required by older db-migration images (pre-v0.12).
    TENANT_API_KEY: "a1503fb6-d79b-4e9e-826d-44cf262f3e05",
    INPUT_VERIFIER_ADDRESS: valueOrEmpty(config.contracts.inputVerification),
  };
}

export function generateCoprocessorEnv(config: FhevmConfig): Record<string, string> {
  return generateCoprocessorEnvWithOverrides(config);
}

export function generateKmsConnectorEnv(config: FhevmConfig): Record<string, string> {
  const dbUrl = formatDbUrl(config, config.db.host, config.db.port, config.db.kmsConnectorDb);
  return {
    // DATABASE_URL is needed by the db-migration init script (raw sqlx).
    DATABASE_URL: dbUrl,
    KMS_CONNECTOR_DATABASE_URL: dbUrl,
    KMS_CONNECTOR_GATEWAY_URL: config.rpc.gatewayHttp,
    KMS_CONNECTOR_GATEWAY_CHAIN_ID: String(config.chainIds.gateway),
    KMS_CONNECTOR_KMS_CORE_ENDPOINTS: config.rpc.kmsCore,
    KMS_CONNECTOR_PRIVATE_KEY:
      config.keys.kmsNodes[0]?.txSender.privateKey ?? config.keys.txSender.privateKey,
    KMS_CONNECTOR_DECRYPTION_CONTRACT__ADDRESS: valueOrEmpty(config.contracts.decryption),
    KMS_CONNECTOR_GATEWAY_CONFIG_CONTRACT__ADDRESS: valueOrEmpty(config.contracts.gatewayConfig),
    KMS_CONNECTOR_KMS_GENERATION_CONTRACT__ADDRESS: valueOrEmpty(config.contracts.kmsGeneration),
    KMS_CONNECTOR_HOST_CHAINS: hostChainRecords(config),
    KMS_CONNECTOR_S3_CONFIG__ENDPOINT: config.minio.endpoint,
    KMS_CONNECTOR_S3_CONFIG__BUCKET: config.minio.buckets.ct128,
    KMS_CONNECTOR_S3_CONFIG__REGION: config.minio.region,
    // Catch up on keygen/crsgen events emitted before kms-connector started.
    KMS_CONNECTOR_KMS_OPERATION_FROM_BLOCK_NUMBER: "0",
    KMS_CONNECTOR_DECRYPTION_FROM_BLOCK_NUMBER: "0",
  };
}

export function generateKmsCoreEnv(config: FhevmConfig): Record<string, string> {
  return {
    KMS_CORE__PUBLIC_VAULT__STORAGE__S3__BUCKET: config.minio.buckets.public,
    KMS_CORE__PUBLIC_VAULT__STORAGE__S3__PREFIX: "PUB",
    KMS_CORE__PRIVATE_VAULT__STORAGE__FILE__PATH: "./keys",
    S3_ENDPOINT: config.minio.endpoint,
    S3_REGION: config.minio.region,
  };
}

export function generateDatabaseEnv(config: FhevmConfig): Record<string, string> {
  return {
    POSTGRES_USER: config.db.user,
    POSTGRES_PASSWORD: config.db.password,
  };
}

export function generateMinioEnv(config: FhevmConfig): Record<string, string> {
  return {
    MINIO_ROOT_USER: config.minio.rootUser,
    MINIO_ROOT_PASSWORD: config.minio.rootPassword,
    MINIO_ENDPOINT: config.minio.endpoint,
    ACCESS_KEY: config.minio.accessKey,
    SECRET_KEY: config.minio.secretKey,
  };
}

export function generateGatewayScEnv(config: FhevmConfig): Record<string, string> {
  const env: Record<string, string> = {
    HARDHAT_NETWORK: "staging",
    CHAIN_ID_GATEWAY: String(config.chainIds.gateway),
    MNEMONIC: config.mnemonic,
    DEPLOYER_PRIVATE_KEY: config.keys.deployer.privateKey,
    NEW_OWNER_PRIVATE_KEY: config.keys.newOwner.privateKey,
    RPC_URL: config.rpc.gatewayHttp,
    PROTOCOL_NAME: config.protocol.name,
    PROTOCOL_WEBSITE: config.protocol.website,
    MPC_THRESHOLD: String(config.thresholds.mpc),
    PUBLIC_DECRYPTION_THRESHOLD: String(config.thresholds.publicDecryption),
    USER_DECRYPTION_THRESHOLD: String(config.thresholds.userDecryption),
    KMS_GENERATION_THRESHOLD: String(config.thresholds.kmsGeneration),
    COPROCESSOR_THRESHOLD: String(config.thresholds.coprocessor),
    NUM_KMS_NODES: String(config.topology.numKmsNodes),
    NUM_COPROCESSORS: String(config.topology.numCoprocessors),
    NUM_CUSTODIANS: String(config.topology.numCustodians),
    NUM_HOST_CHAINS: String(config.topology.numHostChains),
    NUM_PAUSERS: String(config.topology.numPausers),
    INPUT_VERIFICATION_PRICE: config.protocol.inputVerificationPrice,
    PUBLIC_DECRYPTION_PRICE: config.protocol.publicDecryptionPrice,
    USER_DECRYPTION_PRICE: config.protocol.userDecryptionPrice,
    ZAMA_OFT_ADDRESS: valueOrEmpty(config.contracts.zamaOft),
    FEES_SENDER_TO_BURNER_ADDRESS: valueOrEmpty(config.contracts.feesSenderToBurner),
    GATEWAY_CONFIG_ADDRESS: valueOrEmpty(config.contracts.gatewayConfig),
    KMS_GENERATION_ADDRESS: valueOrEmpty(config.contracts.kmsGeneration),
    PAUSER_SET_ADDRESS: valueOrEmpty(config.contracts.pauserSet),
    PAUSER_PRIVATE_KEY: valueOrEmpty(config.keys.pausers[0]?.privateKey),
    TX_SENDER_PRIVATE_KEY: config.keys.txSender.privateKey,
  };

  for (let i = 0; i < config.topology.numKmsNodes; i += 1) {
    env[`KMS_TX_SENDER_ADDRESS_${i}`] = valueOrEmpty(config.keys.kmsNodes[i]?.txSender.address);
    env[`KMS_SIGNER_ADDRESS_${i}`] = kmsSignerAt(config, i);
    env[`KMS_NODE_IP_ADDRESS_${i}`] = config.keys.kmsNodes[i]?.ipAddress ?? `127.0.0.${i + 1}`;
    env[`KMS_NODE_STORAGE_URL_${i}`] = `http://minio:${config.ports.minioApi}/${config.minio.buckets.public}`;
  }

  for (let i = 0; i < config.topology.numCoprocessors; i += 1) {
    env[`COPROCESSOR_TX_SENDER_ADDRESS_${i}`] = valueOrEmpty(config.keys.coprocessors[i]?.txSender.address);
    env[`COPROCESSOR_SIGNER_ADDRESS_${i}`] = valueOrEmpty(config.keys.coprocessors[i]?.signer.address);
    env[`COPROCESSOR_S3_BUCKET_URL_${i}`] = config.keys.coprocessors[i]?.s3BucketUrl ?? `s3://${config.minio.buckets.ct128}`;
  }

  for (let i = 0; i < config.topology.numCustodians; i += 1) {
    env[`CUSTODIAN_TX_SENDER_ADDRESS_${i}`] = valueOrEmpty(config.keys.custodians[i]?.txSender.address);
    env[`CUSTODIAN_SIGNER_ADDRESS_${i}`] = valueOrEmpty(config.keys.custodians[i]?.signer.address);
    env[`CUSTODIAN_ENCRYPTION_KEY_${i}`] = valueOrEmpty(config.keys.custodians[i]?.encryptionKey);
  }

  for (let i = 0; i < config.topology.numHostChains; i += 1) {
    env[`HOST_CHAIN_CHAIN_ID_${i}`] = String(config.chainIds.host + i);
    env[`HOST_CHAIN_FHEVM_EXECUTOR_ADDRESS_${i}`] = valueOrEmpty(config.contracts.fhevmExecutor);
    env[`HOST_CHAIN_ACL_ADDRESS_${i}`] = valueOrEmpty(config.contracts.acl);
    env[`HOST_CHAIN_NAME_${i}`] = `Host chain ${config.chainIds.host + i}`;
    env[`HOST_CHAIN_WEBSITE_${i}`] = `https://host-chain-${config.chainIds.host + i}.local`;
  }

  for (let i = 0; i < config.topology.numPausers; i += 1) {
    env[`PAUSER_ADDRESS_${i}`] = valueOrEmpty(config.keys.pausers[i]?.address);
  }

  return env;
}

export function generateHostScEnv(config: FhevmConfig): Record<string, string> {
  const env: Record<string, string> = {
    HARDHAT_NETWORK: "staging",
    MNEMONIC: config.mnemonic,
    CHAIN_ID_GATEWAY: String(config.chainIds.gateway),
    DEPLOYER_PRIVATE_KEY: config.keys.deployer.privateKey,
    DECRYPTION_ADDRESS: valueOrEmpty(config.contracts.decryption),
    INPUT_VERIFICATION_ADDRESS: valueOrEmpty(config.contracts.inputVerification),
    NUM_KMS_NODES: String(config.topology.numKmsNodes),
    PUBLIC_DECRYPTION_THRESHOLD: String(config.thresholds.publicDecryption),
    COPROCESSOR_THRESHOLD: String(config.thresholds.coprocessor),
    NUM_COPROCESSORS: String(config.topology.numCoprocessors),
    NUM_PAUSERS: String(config.topology.numPausers),
    RPC_URL: config.rpc.hostHttp,
    ACL_CONTRACT_ADDRESS: valueOrEmpty(config.contracts.acl),
    PAUSER_SET_CONTRACT_ADDRESS: valueOrEmpty(config.contracts.hostPauserSet),
    PAUSER_PRIVATE_KEY: valueOrEmpty(config.keys.pausers[0]?.privateKey),
    NEW_OWNER_PRIVATE_KEY: config.keys.newOwner.privateKey,
    ETHERSCAN_API_KEY: "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX",
  };

  for (let i = 0; i < config.topology.numKmsNodes; i += 1) {
    env[`KMS_SIGNER_ADDRESS_${i}`] = kmsSignerAt(config, i);
  }
  for (let i = 0; i < config.topology.numCoprocessors; i += 1) {
    env[`COPROCESSOR_SIGNER_ADDRESS_${i}`] = valueOrEmpty(config.keys.coprocessors[i]?.signer.address);
  }
  for (let i = 0; i < config.topology.numPausers; i += 1) {
    env[`PAUSER_ADDRESS_${i}`] = valueOrEmpty(config.keys.pausers[i]?.address);
  }

  return env;
}

export function generateRelayerEnv(config: FhevmConfig): Record<string, string> {
  return {
    DATABASE_URL: formatDbUrl(config, config.db.relayerHost, config.db.relayerPort, config.db.relayerDb),
    MAX_ATTEMPTS: "10",
  };
}

export function generateGatewayNodeEnv(config: FhevmConfig): Record<string, string> {
  return {
    MNEMONIC: config.mnemonic,
  };
}

export function generateHostNodeEnv(config: FhevmConfig): Record<string, string> {
  return {
    MNEMONIC: config.mnemonic,
  };
}

export function generateTestSuiteEnv(config: FhevmConfig): Record<string, string> {
  return {
    MNEMONIC: config.mnemonic,
    RPC_URL: config.rpc.hostHttp,
    RELAYER_URL: `${config.rpc.relayerHttp}/v2`,
    HARDHAT_NETWORK: "staging",
    ACL_CONTRACT_ADDRESS: valueOrEmpty(config.contracts.acl),
    FHEVM_EXECUTOR_CONTRACT_ADDRESS: valueOrEmpty(config.contracts.fhevmExecutor),
    KMS_VERIFIER_CONTRACT_ADDRESS: valueOrEmpty(config.contracts.kmsVerifier),
    INPUT_VERIFIER_CONTRACT_ADDRESS: valueOrEmpty(config.contracts.inputVerifier),
    DECRYPTION_ADDRESS: valueOrEmpty(config.contracts.decryption),
    INPUT_VERIFICATION_ADDRESS: valueOrEmpty(config.contracts.inputVerification),
    CHAIN_ID_GATEWAY: String(config.chainIds.gateway),
    CHAIN_ID_HOST: String(config.chainIds.host),
  };
}

export function generateGatewayMockedPaymentEnv(config: FhevmConfig): Record<string, string> {
  return {
    HARDHAT_NETWORK: "staging",
    RPC_URL: config.rpc.gatewayHttp,
    MNEMONIC: config.mnemonic,
    CHAIN_ID_GATEWAY: String(config.chainIds.gateway),
    DEPLOYER_PRIVATE_KEY: config.keys.deployer.privateKey,
    TX_SENDER_PRIVATE_KEY: config.keys.txSender.privateKey,
    PROTOCOL_PAYMENT_ADDRESS: valueOrEmpty(config.contracts.protocolPayment),
    ZAMA_OFT_ADDRESS: valueOrEmpty(config.contracts.zamaOft),
  };
}

export const ENV_GENERATORS: Readonly<Record<EnvFileName, EnvGenerator>> = {
  coprocessor: generateCoprocessorEnv,
  "kms-connector": generateKmsConnectorEnv,
  "kms-core": generateKmsCoreEnv,
  database: generateDatabaseEnv,
  minio: generateMinioEnv,
  "gateway-sc": generateGatewayScEnv,
  "host-sc": generateHostScEnv,
  relayer: generateRelayerEnv,
  "gateway-node": generateGatewayNodeEnv,
  "host-node": generateHostNodeEnv,
  "test-suite": generateTestSuiteEnv,
  "gateway-mocked-payment": generateGatewayMockedPaymentEnv,
};
