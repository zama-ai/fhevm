/**
 * Renders runtime env maps from resolved versions, scenario topology, discovery outputs, and compat policy.
 */
import {
  coprocessorUsesHostKmsGeneration,
  compatPolicyForState,
  kmsConnectorUsesHostKmsGeneration,
  requiresLegacyRelayerUrl,
  requiresMultichainAclAddress,
  requiresModernHostAddressArtifacts,
} from "../compat/compat";
import { driftDatabaseName } from "../drift";
import type { StackSpec } from "../stack-spec/stack-spec";
import {
  COPROCESSOR_WALLET_INDICES,
  DEFAULT_TENANT_API_KEY,
  KMS_NODE_WALLET_INDICES,
  MINIO_INTERNAL_URL,
  POSTGRES_HOST,
  hostChainRuntimes,
  realLzEndpointFor,
} from "../layout";
import { kmsConnectorDbName, kmsConnectorEnvName, kmsCoreName, kmsMpcPort, kmsPublicPrefix, kmsServicePort, reconstructionThreshold } from "../kms-party";
import type { State } from "../types";
import { predictedCrsId, predictedKeyId } from "../utils/fs";

export type WalletMaterial = {
  address: string;
  privateKey: string;
};


const HAS_PLACEHOLDER = /(?<!\$)\$\{[A-Z0-9_]+\}/;

const assertNoGeneratedPlaceholders = (env: Record<string, string>) => {
  const unresolved = Object.entries(env)
    .filter(([, value]) => HAS_PLACEHOLDER.test(value))
    .map(([key]) => key);
  if (unresolved.length) {
    throw new Error(`Unresolved env interpolation for ${unresolved.join(", ")}`);
  }
};

/** Applies contract addresses into a component env map when values are available. */
const updateContracts = (env: Record<string, string>, values: Record<string, string>) => {
  for (const [key, value] of Object.entries(values)) {
    if (value !== undefined) {
      env[key] = value;
    }
  }
};

/** Provides non-empty metadata defaults for host-chain registration tasks. */
const defaultHostChainMetadata = (chain: Pick<StackSpec["hostChains"][number], "name">, index: number) => ({
  name: chain.name ?? `Host chain ${index}`,
  website: `https://host-chain-${index}.com`,
});

/** Applies topology-driven values shared across generated component env files. */
const applyTopologyEnv = (
  envs: Record<string, Record<string, string>>,
  plan: Pick<StackSpec, "topology">,
) => {
  envs["test-suite"].COPROCESSOR_COUNT = String(plan.topology.count);
  envs["test-suite"].CONSENSUS_THRESHOLD = String(plan.topology.threshold);
  envs["test-suite"].COPROCESSOR_THRESHOLD = String(plan.topology.threshold);
  envs["gateway-sc"].NUM_COPROCESSORS = String(plan.topology.count);
  envs["gateway-sc"].COPROCESSOR_THRESHOLD = String(plan.topology.threshold);
  envs["host-sc"].NUM_COPROCESSORS = String(plan.topology.count);
  envs["host-sc"].COPROCESSOR_THRESHOLD = String(plan.topology.threshold);
};

/** Keeps host-contract deployment KMS inputs aligned with the gateway-side source of truth. */
const applyHostScKmsEnv = (envs: Record<string, Record<string, string>>) => {
  const gatewayEnv = envs["gateway-sc"];
  const hostEnv = envs["host-sc"];
  hostEnv.NUM_KMS_NODES = gatewayEnv.NUM_KMS_NODES;
  hostEnv.PUBLIC_DECRYPTION_THRESHOLD = gatewayEnv.PUBLIC_DECRYPTION_THRESHOLD;
  hostEnv.USER_DECRYPTION_THRESHOLD = gatewayEnv.USER_DECRYPTION_THRESHOLD;
  hostEnv.KMS_GEN_THRESHOLD = gatewayEnv.KMS_GENERATION_THRESHOLD;

  const numKmsNodes = Number(gatewayEnv.NUM_KMS_NODES ?? "0");
  for (let index = 0; index < numKmsNodes; index += 1) {
    const txSender = gatewayEnv[`KMS_TX_SENDER_ADDRESS_${index}`];
    const signer = gatewayEnv[`KMS_SIGNER_ADDRESS_${index}`];
    const storageUrl = gatewayEnv[`KMS_NODE_STORAGE_URL_${index}`];
    const ipAddress = gatewayEnv[`KMS_NODE_IP_ADDRESS_${index}`];
    if (txSender) hostEnv[`KMS_TX_SENDER_ADDRESS_${index}`] = txSender;
    if (signer) hostEnv[`KMS_SIGNER_ADDRESS_${index}`] = signer;
    if (storageUrl) hostEnv[`KMS_NODE_STORAGE_URL_${index}`] = storageUrl;
    if (ipAddress) hostEnv[`KMS_NODE_IP_${index}`] = ipAddress;
  }
};

const hostDeployKmsGenerationArgs = (plan: StackSpec, enabled: boolean) =>
  requiresModernHostAddressArtifacts(plan) ? `--with-kms-generation ${enabled}` : "";

/** Applies base runtime defaults before compat or discovery-specific rewrites. */
const applyBaseRuntimeEnv = (
  envs: Record<string, Record<string, string>>,
  state: Pick<State, "discovery">,
) => {
  const keyPrefix = state.discovery?.minioKeyPrefix ?? "PUB";
  const minioInternal = state.discovery?.endpoints.minioInternal ?? MINIO_INTERNAL_URL;
  const fheKeyId = state.discovery?.actualFheKeyId ?? state.discovery?.fheKeyId ?? predictedKeyId();
  const crsKeyId = state.discovery?.actualCrsKeyId ?? state.discovery?.crsKeyId ?? predictedCrsId();

  envs["coprocessor"].DATABASE_URL = `postgresql://${envs.database.POSTGRES_USER}:${envs.database.POSTGRES_PASSWORD}@${POSTGRES_HOST}/coprocessor`;
  // Local e2e stacks start from genesis with wave-2 semantics; tests may
  // restart workers after computations exist, so make the zero-cutover intent
  // explicit for the worker safety guard.
  envs["coprocessor"].FHEVM_ALLOW_ZERO_CUTOVER = "1";
  // E2E tests opt into automatic drift revert. Set via env (not CLI flag).
  envs["coprocessor"].DRIFT_AUTO_REVERT_ENABLED = "true";
  // Test-only: hold drift-revert signal in "reverting" state briefly so e2e
  // tests can observe the post-revert DB state before services resume.
  // No-op when no drift revert is in progress.
  envs["coprocessor"].DRIFT_REVERT_TEST_HOLD_SECS = "15";
  envs["coprocessor"].TENANT_API_KEY = DEFAULT_TENANT_API_KEY;
  envs["coprocessor"].COPROCESSOR_API_KEY = DEFAULT_TENANT_API_KEY;
  envs["coprocessor"].AWS_ENDPOINT_URL = state.discovery?.endpoints.minioExternal ?? MINIO_INTERNAL_URL;
  envs["coprocessor"].FHE_KEY_ID = fheKeyId;
  envs["coprocessor"].KMS_PUBLIC_KEY = `${minioInternal}/kms-public/${keyPrefix}/PublicKey/${fheKeyId}`;
  envs["coprocessor"].KMS_SERVER_KEY = `${minioInternal}/kms-public/${keyPrefix}/ServerKey/${fheKeyId}`;
  envs["coprocessor"].KMS_SNS_KEY = `${minioInternal}/kms-public/${keyPrefix}/SnsKey/${fheKeyId}`;
  envs["coprocessor"].KMS_CRS_KEY = `${minioInternal}/kms-public/${keyPrefix}/CRS/${crsKeyId}`;
};

/** Applies compatibility-driven env aliases and URL rewrites. */
const applyCompatEnv = (
  envs: Record<string, Record<string, string>>,
  plan: StackSpec,
) => {
  const compat = compatPolicyForState(plan);
  for (const [key, source] of Object.entries(compat.connectorEnv)) {
    if (envs["kms-connector"][source]) {
      envs["kms-connector"][key] = envs["kms-connector"][source];
    }
  }
  if (!requiresLegacyRelayerUrl(plan)) {
    const base = envs["test-suite"].RELAYER_URL ?? "";
    if (base && !base.endsWith("/v2")) {
      envs["test-suite"].RELAYER_URL = `${base}/v2`;
    }
  }
};

/** Applies discovery outputs such as deployed addresses and signer material. */
const applyDiscoveryEnv = (
  envs: Record<string, Record<string, string>>,
  state: Pick<State, "discovery">,
  plan: StackSpec,
) => {
  // One registered signer per party; centralized is just the single-signer case. Each address
  // is discovered from its PUB-p{i} prefix (party 1 = PUB for the centralized core).
  (state.discovery?.kmsSigners ?? []).forEach((address, index) => {
    envs["gateway-sc"][`KMS_SIGNER_ADDRESS_${index}`] = address;
    envs["host-sc"][`KMS_SIGNER_ADDRESS_${index}`] = address;
  });
  // Each node's serialized CA certificate (hex), discovered from its public vault
  // alongside the VerfAddress. The host ProtocolConfig deploy reads it as KMS_NODE_CA_CERT_i.
  (state.discovery?.kmsCaCerts ?? []).forEach((caCert, index) => {
    envs["host-sc"][`KMS_NODE_CA_CERT_${index}`] = caCert;
  });
  if (!state.discovery) {
    return;
  }

  const chains = hostChainRuntimes(plan.hostChains);
  const defaultChain = chains[0];
  if (!defaultChain) {
    return;
  }
  const primaryHost = state.discovery.hosts[defaultChain.key] ?? {};
  const gatewayKmsGenerationAddress = state.discovery.gateway.KMS_GENERATION_ADDRESS;
  const hostKmsGenerationAddress = primaryHost.KMS_GENERATION_CONTRACT_ADDRESS;
  const coprocessorKmsGenerationAddress = coprocessorUsesHostKmsGeneration(plan)
    ? hostKmsGenerationAddress
    : gatewayKmsGenerationAddress;
  const connectorKmsGenerationAddress = kmsConnectorUsesHostKmsGeneration(plan)
    ? hostKmsGenerationAddress
    : gatewayKmsGenerationAddress;
  const protocolConfigAddress = requiresModernHostAddressArtifacts(plan)
    ? primaryHost.PROTOCOL_CONFIG_CONTRACT_ADDRESS
    : "";

  updateContracts(envs["gateway-sc"], state.discovery.gateway);
  updateContracts(envs["gateway-mocked-payment"], {
    PROTOCOL_PAYMENT_ADDRESS: state.discovery.gateway.PROTOCOL_PAYMENT_ADDRESS,
  });
  updateContracts(envs["host-sc"], {
    DECRYPTION_ADDRESS: state.discovery.gateway.DECRYPTION_ADDRESS,
    INPUT_VERIFICATION_ADDRESS: state.discovery.gateway.INPUT_VERIFICATION_ADDRESS,
    ACL_CONTRACT_ADDRESS: primaryHost.ACL_CONTRACT_ADDRESS,
    PAUSER_SET_CONTRACT_ADDRESS: primaryHost.PAUSER_SET_CONTRACT_ADDRESS,
  });
  // Per-chain gateway-sc indexed vars are set uniformly in renderEnvMaps below.
  updateContracts(envs["coprocessor"], {
    ACL_CONTRACT_ADDRESS: primaryHost.ACL_CONTRACT_ADDRESS,
    FHEVM_EXECUTOR_CONTRACT_ADDRESS: primaryHost.FHEVM_EXECUTOR_CONTRACT_ADDRESS,
    INPUT_VERIFIER_ADDRESS: primaryHost.INPUT_VERIFIER_CONTRACT_ADDRESS,
    INPUT_VERIFICATION_ADDRESS: state.discovery.gateway.INPUT_VERIFICATION_ADDRESS,
    CIPHERTEXT_COMMITS_ADDRESS: state.discovery.gateway.CIPHERTEXT_COMMITS_ADDRESS,
    ...(requiresMultichainAclAddress(plan) ? { MULTICHAIN_ACL_ADDRESS: state.discovery.gateway.MULTICHAIN_ACL_ADDRESS } : {}),
    KMS_GENERATION_ADDRESS: coprocessorKmsGenerationAddress ?? "",
    PROTOCOL_CONFIG_ADDRESS: protocolConfigAddress,
    CONFIDENTIAL_BRIDGE_CONTRACT_ADDRESS: primaryHost.CONFIDENTIAL_BRIDGE_CONTRACT_ADDRESS ?? "",
  });

  const kmsHostChains = chains.map((chain) => {
    const hostAddresses = state.discovery!.hosts[chain.key] ?? {};
    const endpoints = state.discovery!.endpoints.hosts[chain.key];
    return {
      url: endpoints?.http ?? `http://${chain.node}:${chain.rpcPort}`,
      chain_id: Number(chain.chainId),
      acl_address: hostAddresses.ACL_CONTRACT_ADDRESS ?? "",
    };
  });
  updateContracts(envs["kms-connector"], {
    KMS_CONNECTOR_DECRYPTION_CONTRACT__ADDRESS: state.discovery.gateway.DECRYPTION_ADDRESS,
    KMS_CONNECTOR_GATEWAY_CONFIG_CONTRACT__ADDRESS: state.discovery.gateway.GATEWAY_CONFIG_ADDRESS,
    KMS_CONNECTOR_KMS_GENERATION_CONTRACT__ADDRESS: connectorKmsGenerationAddress ?? "",
    KMS_CONNECTOR_PROTOCOL_CONFIG_CONTRACT__ADDRESS: primaryHost.PROTOCOL_CONFIG_CONTRACT_ADDRESS,
    KMS_CONNECTOR_HOST_CHAINS: JSON.stringify(kmsHostChains),
  });
  updateContracts(envs["relayer"], {
    APP_GATEWAY__CONTRACTS__DECRYPTION_ADDRESS: state.discovery.gateway.DECRYPTION_ADDRESS,
    APP_GATEWAY__CONTRACTS__INPUT_VERIFICATION_ADDRESS: state.discovery.gateway.INPUT_VERIFICATION_ADDRESS,
  });
  updateContracts(envs["test-suite"], {
    GATEWAY_CONFIG_ADDRESS: state.discovery.gateway.GATEWAY_CONFIG_ADDRESS,
    DECRYPTION_ADDRESS: state.discovery.gateway.DECRYPTION_ADDRESS,
    INPUT_VERIFICATION_ADDRESS: state.discovery.gateway.INPUT_VERIFICATION_ADDRESS,
    KMS_VERIFIER_CONTRACT_ADDRESS: primaryHost.KMS_VERIFIER_CONTRACT_ADDRESS,
    ACL_CONTRACT_ADDRESS: primaryHost.ACL_CONTRACT_ADDRESS,
    INPUT_VERIFIER_CONTRACT_ADDRESS: primaryHost.INPUT_VERIFIER_CONTRACT_ADDRESS,
    FHEVM_EXECUTOR_CONTRACT_ADDRESS: primaryHost.FHEVM_EXECUTOR_CONTRACT_ADDRESS,
    PROTOCOL_CONFIG_CONTRACT_ADDRESS: primaryHost.PROTOCOL_CONFIG_CONTRACT_ADDRESS,
    KMS_GENERATION_CONTRACT_ADDRESS: primaryHost.KMS_GENERATION_CONTRACT_ADDRESS,
    CONFIDENTIAL_BRIDGE_CONTRACT_ADDRESS: primaryHost.CONFIDENTIAL_BRIDGE_CONTRACT_ADDRESS,
    LZ_ENDPOINT_ADDRESS: primaryHost.LZ_ENDPOINT_ADDRESS,
  });
  envs["test-suite"].BRIDGE_REAL_LZ = chains.some((chain) => realLzEndpointFor(chain.key)) ? "true" : "";
};

export type KmsParty = { party: number; endpoint: string; privateKey: string; dbName: string };

/**
 * ProtocolConfig context globals the host deploy reads, shared by both KMS modes.
 * mock_enclave skips PCR attestation, so zero PCRs suffice. softwareVersion must be valid semver
 * (the KMS core parses it) — fall back to a placeholder when CORE_VERSION is a git-SHA tag.
 */
const applyProtocolConfigKmsGlobals = (hostSc: Record<string, string>, plan: StackSpec) => {
  const zeroPcr = `0x${"00".repeat(48)}`;
  const coreVersion = (plan.versions.env.CORE_VERSION ?? "").replace(/^v/, "");
  hostSc.KMS_SOFTWARE_VERSION = /^\d+(\.\d+){0,2}(-[0-9A-Za-z.-]+)?$/.test(coreVersion) ? coreVersion : "0.1.0";
  hostSc.KMS_PCR_VALUES = JSON.stringify([{ pcr0: zeroPcr, pcr1: zeroPcr, pcr2: zeroPcr }]);
};

/**
 * Centralized mode: the single KMS node's ProtocolConfig params the threshold path sets per-node.
 * The rest (tx-sender, IP, storage URL, signer, CA cert) already come from the templates and
 * discovery. storagePrefix is "PUB" for the centralized core (PUB-p{i} is threshold-only), so it
 * tracks the discovered minioKeyPrefix. Must run after applyDiscoveryEnv.
 */
const applyKmsCentralizedHostEnv = (
  envs: Record<string, Record<string, string>>,
  plan: StackSpec,
  state: Pick<State, "discovery">,
) => {
  if (plan.kms.mode === "threshold") {
    return;
  }
  const hostSc = envs["host-sc"];
  applyProtocolConfigKmsGlobals(hostSc, plan);
  hostSc.KMS_NODE_PARTY_ID_0 = "1";
  hostSc.KMS_NODE_MPC_IDENTITY_0 = kmsCoreName(1);
  hostSc.KMS_NODE_STORAGE_PREFIX_0 = state.discovery?.minioKeyPrefix ?? "PUB";
};

/**
 * Threshold mode only: sets gateway-sc KMS counts/thresholds + per-party
 * tx-sender wallets, and points the base (party-1) connector at kms-core.
 * Returns the per-party connection info so connector instance envs for parties
 * 2..N can be cloned AFTER discovery has rewritten the base connector env.
 * Must run before applyHostScKmsEnv so host-sc inherits the counts/addresses.
 */
const applyKmsThresholdGatewayEnv = async (
  envs: Record<string, Record<string, string>>,
  plan: StackSpec,
  deriveWallet: (mnemonic: string, index: number) => Promise<WalletMaterial>,
): Promise<KmsParty[]> => {
  if (plan.kms.mode !== "threshold") {
    return [];
  }
  const { parties, threshold, committeeSize } = plan.kms;
  if (parties > KMS_NODE_WALLET_INDICES.length) {
    throw new Error(`KMS parties ${parties} exceeds supported ${KMS_NODE_WALLET_INDICES.length}`);
  }
  const gw = envs["gateway-sc"];
  const hostSc = envs["host-sc"];
  const mnemonic = gw.MNEMONIC;
  if (!mnemonic) {
    throw new Error("Missing gateway mnemonic for threshold-mode KMS setup");
  }
  // On-chain committee = the first committeeSize parties; cores beyond it are spares (still provisioned
  // below with a connector + signing key so a context switch can rotate one into the committee).
  gw.NUM_KMS_NODES = String(committeeSize);
  gw.MPC_THRESHOLD = String(threshold);
  // host-sc deploy reads MPC_THRESHOLD too (ProtocolConfig). applyHostScKmsEnv
  // mirrors the other KMS thresholds gateway->host but NOT MPC_THRESHOLD, and
  // the host-sc template default (1) only happens to match t=1. Set it here so
  // host and gateway agree for every topology (e.g. 7 parties / t=2). Scoped to
  // threshold mode so the centralized template default is left untouched.
  envs["host-sc"].MPC_THRESHOLD = String(threshold);
  // Decryption/keygen consensus needs 2t+1 matching responses (reconstruction
  // threshold; matches the KMS core-client `num_reconstruct`).
  const reconstruct = String(reconstructionThreshold(threshold));
  gw.PUBLIC_DECRYPTION_THRESHOLD = reconstruct;
  gw.USER_DECRYPTION_THRESHOLD = reconstruct;
  gw.KMS_GENERATION_THRESHOLD = reconstruct;

  applyProtocolConfigKmsGlobals(hostSc, plan);

  const result: KmsParty[] = [];
  for (let party = 1; party <= parties; party += 1) {
    const idx = party - 1;
    const wallet = await deriveWallet(mnemonic, KMS_NODE_WALLET_INDICES[idx]);
    gw[`KMS_TX_SENDER_ADDRESS_${idx}`] = wallet.address;
    // external_url: the core does url::Url::parse() and requires host+port, so it needs a scheme.
    gw[`KMS_NODE_IP_ADDRESS_${idx}`] = `http://${kmsCoreName(party)}:${kmsMpcPort(party)}`;
    gw[`KMS_NODE_STORAGE_URL_${idx}`] = `${MINIO_INTERNAL_URL}/kms-public`;
    // Per-node KmsNodeParams the host ProtocolConfig deploy reads. partyId is 1-based
    // (the env index is 0-based), mpcIdentity must match the node's TLS cert CN (gen-keys sets
    // --tls-subject to the core name), and storagePrefix is the node's public vault prefix. The
    // signer address and CA cert are discovered post-boot (applyDiscoveryEnv).
    hostSc[`KMS_NODE_PARTY_ID_${idx}`] = String(party);
    hostSc[`KMS_NODE_MPC_IDENTITY_${idx}`] = kmsCoreName(party);
    hostSc[`KMS_NODE_STORAGE_PREFIX_${idx}`] = kmsPublicPrefix(party);
    hostSc[`KMS_TX_SENDER_ADDRESS_${idx}`] = wallet.address;
    hostSc[`KMS_NODE_IP_${idx}`] = gw[`KMS_NODE_IP_ADDRESS_${idx}`];
    hostSc[`KMS_NODE_STORAGE_URL_${idx}`] = gw[`KMS_NODE_STORAGE_URL_${idx}`];
    // KMS_SIGNER_ADDRESS_{idx} comes from per-party signing-key discovery.
    const endpoint = `http://${kmsCoreName(party)}:${kmsServicePort(party)}`;
    const dbName = kmsConnectorDbName(party);
    if (party === 1) {
      envs["kms-connector"].KMS_CONNECTOR_KMS_CORE_ENDPOINTS = endpoint;
      envs["kms-connector"].KMS_CONNECTOR_PRIVATE_KEY = wallet.privateKey;
      envs["kms-connector"].KMS_CONNECTOR_DATABASE_URL = `postgresql://db:5432/${dbName}`;
      envs["kms-connector"].DATABASE_URL = `postgresql://db:5432/${dbName}`;
    }
    result.push({ party, endpoint, privateKey: wallet.privateKey, dbName });
  }
  return result;
};

const isKmsSwapTopology = (plan: StackSpec) =>
  plan.kms.mode === "threshold" && plan.kms.parties > plan.kms.committeeSize;

/** Node swap: maps the trailing committee slots onto the spare cores (parties beyond
 * committeeSize). For 5 parties / committee 4 / 1 spare this overwrites slot 3 (node 4) with the
 * spare at env index 4 (node 5), yielding committee {1,2,3,5}. */
const kmsSwapSlots = (plan: StackSpec): { slot: number; src: number }[] => {
  const { parties, committeeSize } = plan.kms;
  const spares = parties - committeeSize;
  return Array.from({ length: spares }, (_, k) => ({ slot: committeeSize - spares + k, src: committeeSize + k }));
};

/** host ProtocolConfig swap-committee env for `defineNewKmsContextAndEpoch`. host-sc carries every
 * provisioned party (committee + spares).
 * Returns undefined for non-swap topologies or before the spare's signer is discovered. */
export const buildHostScSwapEnv = (
  hostSc: Record<string, string>,
  plan: StackSpec,
): Record<string, string> | undefined => {
  if (!isKmsSwapTopology(plan)) return undefined;
  const swap = { ...hostSc };
  for (const { slot, src } of kmsSwapSlots(plan)) {
    if (!hostSc[`KMS_SIGNER_ADDRESS_${src}`]) return undefined;
    // The spare joins at the dropped node's MPC position, so KMS_NODE_PARTY_ID_{slot} stays the
    // positional id (1..committeeSize) — the core rejects party ids outside that range. Only the
    // node's identity (signer, tx-sender, cert), address and storage prefix move to the spare.
    swap[`KMS_TX_SENDER_ADDRESS_${slot}`] = hostSc[`KMS_TX_SENDER_ADDRESS_${src}`];
    swap[`KMS_SIGNER_ADDRESS_${slot}`] = hostSc[`KMS_SIGNER_ADDRESS_${src}`];
    swap[`KMS_NODE_IP_${slot}`] = hostSc[`KMS_NODE_IP_${src}`];
    swap[`KMS_NODE_STORAGE_URL_${slot}`] = hostSc[`KMS_NODE_STORAGE_URL_${src}`];
    swap[`KMS_NODE_MPC_IDENTITY_${slot}`] = hostSc[`KMS_NODE_MPC_IDENTITY_${src}`];
    swap[`KMS_NODE_CA_CERT_${slot}`] = hostSc[`KMS_NODE_CA_CERT_${src}`];
    swap[`KMS_NODE_STORAGE_PREFIX_${slot}`] = hostSc[`KMS_NODE_STORAGE_PREFIX_${src}`];
  }
  return swap;
};

/** gateway GatewayConfig swap-committee env for `updateKmsContext`. The Gateway KmsNode carries only
 * (txSender, signer, ip, storageUrl), all present in gateway-sc for every party. */
export const buildGatewayScSwapEnv = (
  gatewaySc: Record<string, string>,
  plan: StackSpec,
): Record<string, string> | undefined => {
  if (!isKmsSwapTopology(plan)) return undefined;
  const swap = { ...gatewaySc };
  for (const { slot, src } of kmsSwapSlots(plan)) {
    if (!gatewaySc[`KMS_SIGNER_ADDRESS_${src}`]) return undefined;
    swap[`KMS_TX_SENDER_ADDRESS_${slot}`] = gatewaySc[`KMS_TX_SENDER_ADDRESS_${src}`];
    swap[`KMS_SIGNER_ADDRESS_${slot}`] = gatewaySc[`KMS_SIGNER_ADDRESS_${src}`];
    swap[`KMS_NODE_IP_ADDRESS_${slot}`] = gatewaySc[`KMS_NODE_IP_ADDRESS_${src}`];
    swap[`KMS_NODE_STORAGE_URL_${slot}`] = gatewaySc[`KMS_NODE_STORAGE_URL_${src}`];
  }
  return swap;
};

/** Clones the (discovery-rewritten) base connector env into per-party instance envs. */
const buildKmsConnectorInstanceEnvs = (
  envs: Record<string, Record<string, string>>,
  kmsParties: KmsParty[],
): Record<string, Record<string, string>> => {
  const instanceEnvs: Record<string, Record<string, string>> = {};
  for (const { party, endpoint, privateKey, dbName } of kmsParties) {
    if (party === 1) continue; // party 1 uses the base kms-connector.env
    const next = { ...envs["kms-connector"] };
    next.KMS_CONNECTOR_KMS_CORE_ENDPOINTS = endpoint;
    next.KMS_CONNECTOR_PRIVATE_KEY = privateKey;
    next.KMS_CONNECTOR_DATABASE_URL = `postgresql://db:5432/${dbName}`;
    next.DATABASE_URL = `postgresql://db:5432/${dbName}`;
    instanceEnvs[kmsConnectorEnvName(party)] = next;
  }
  return instanceEnvs;
};

/** Builds per-instance coprocessor env maps and injects derived signer addresses. */
const buildInstanceEnvs = async (
  envs: Record<string, Record<string, string>>,
  plan: StackSpec,
  deriveWallet: (mnemonic: string, index: number) => Promise<WalletMaterial>,
) => {
  const instanceEnvs: Record<string, Record<string, string>> = {};
  const baseInstance = plan.coprocessor.instances.find((instance) => instance.index === 0);
  if (plan.topology.count === 1) {
    if (baseInstance) {
      Object.assign(envs["coprocessor"], baseInstance.env);
    }
    return instanceEnvs;
  }
  if (plan.topology.count > COPROCESSOR_WALLET_INDICES.length) {
    throw new Error(`Multicopro topology exceeds supported count ${COPROCESSOR_WALLET_INDICES.length}`);
  }
  const mnemonic = envs["gateway-sc"].MNEMONIC;
  if (!mnemonic) {
    throw new Error("Missing gateway mnemonic for multicopro setup");
  }
  for (let index = 0; index < plan.topology.count; index += 1) {
    const wallet = await deriveWallet(mnemonic, COPROCESSOR_WALLET_INDICES[index]);
    envs["gateway-sc"][`COPROCESSOR_TX_SENDER_ADDRESS_${index}`] = wallet.address;
    envs["gateway-sc"][`COPROCESSOR_SIGNER_ADDRESS_${index}`] = wallet.address;
    envs["gateway-sc"][`COPROCESSOR_S3_BUCKET_URL_${index}`] = `${MINIO_INTERNAL_URL}/ct128`;
    envs["host-sc"][`COPROCESSOR_SIGNER_ADDRESS_${index}`] = wallet.address;
    if (index === 0) {
      envs["coprocessor"].TX_SENDER_PRIVATE_KEY = wallet.privateKey;
      Object.assign(envs["coprocessor"], baseInstance?.env ?? {});
      continue;
    }
    const next = { ...envs["coprocessor"] };
    next.DATABASE_URL = `postgresql://${envs.database.POSTGRES_USER}:${envs.database.POSTGRES_PASSWORD}@${POSTGRES_HOST}/${driftDatabaseName(index)}`;
    next.TX_SENDER_PRIVATE_KEY = wallet.privateKey;
    const instance = plan.coprocessor.instances.find((item) => item.index === index);
    Object.assign(next, instance?.env ?? {});
    instanceEnvs[`coprocessor.${index}`] = next;
  }
  return instanceEnvs;
};

/** Validates that generated env files contain final values, not chained placeholders. */
const validateEnvMaps = (
  envs: Record<string, Record<string, string>>,
  instanceEnvs: Record<string, Record<string, string>>,
) => {
  for (const env of [...Object.values(envs), ...Object.values(instanceEnvs)]) {
    assertNoGeneratedPlaceholders(env);
  }
};

/** Renders component and per-instance env maps from state, topology, and discovery. */
export const renderEnvMaps = async (
  state: Pick<State, "discovery">,
  plan: StackSpec,
  templateEnvs: Record<string, Record<string, string>>,
  deriveWallet: (mnemonic: string, index: number) => Promise<WalletMaterial>,
) => {
  const envs = structuredClone(templateEnvs);
  const chains = hostChainRuntimes(plan.hostChains);
  const defaultChain = chains[0];
  if (!defaultChain) {
    throw new Error("Missing default host chain");
  }
  applyTopologyEnv(envs, plan);
  const kmsParties = await applyKmsThresholdGatewayEnv(envs, plan, deriveWallet);
  applyHostScKmsEnv(envs);
  applyBaseRuntimeEnv(envs, state);
  applyCompatEnv(envs, plan);
  applyDiscoveryEnv(envs, state, plan);
  applyKmsCentralizedHostEnv(envs, plan, state);
  envs["host-node"].RPC_URL = `http://${defaultChain.node}:${defaultChain.rpcPort}`;
  envs["host-node"].HOST_NODE_PORT = String(defaultChain.rpcPort);
  envs["host-node"].HOST_NODE_CHAIN_ID = defaultChain.chainId;
  envs["host-sc"].RPC_URL = `http://${defaultChain.node}:${defaultChain.rpcPort}`;
  envs["host-sc"].HOST_ADDRESS_DIR = defaultChain.key;
  envs["host-sc"].HOST_SC_DEPLOY_KMS_GENERATION_ARGS = hostDeployKmsGenerationArgs(plan, true);
  // Canonical host seeds ProtocolConfig fresh; non-canonical chains get this patched at deploy time
  // by the up flow (see `canonicalProtocolConfigSeedingArgs`) once the canonical address exists.
  envs["host-sc"].HOST_SC_DEPLOY_PROTOCOL_CONFIG_ARGS = "";
  envs["coprocessor"].RPC_HTTP_URL = `http://${defaultChain.node}:${defaultChain.rpcPort}`;
  envs["coprocessor"].RPC_WS_URL = `ws://${defaultChain.node}:${defaultChain.rpcPort}`;
  envs["kms-connector"].KMS_CONNECTOR_ETHEREUM_URL = `http://${defaultChain.node}:${defaultChain.rpcPort}`;
  envs["kms-connector"].KMS_CONNECTOR_ETHEREUM_CHAIN_ID = defaultChain.chainId;
  envs["test-suite"].RPC_URL = `http://${defaultChain.node}:${defaultChain.rpcPort}`;
  envs["test-suite"].CHAIN_ID_HOST = defaultChain.chainId;

  // Multi-chain seeding for the coprocessor dbMigration container.
  // HOST_CHAINS_COUNT + indexed HOST_CHAIN_<i>_{ID,NAME,ACL} drive
  // `seed_host_chains` in `initialize_db.sh`. Same shape the helm chart
  // renders from `.Values.chains` — keeps the e2e on the same code path
  // as production rather than bypassing it via direct SQL. Applied BEFORE
  // `buildInstanceEnvs` so multi-coprocessor topology instances inherit
  // these env vars when they clone `envs["coprocessor"]`.
  envs["coprocessor"].HOST_CHAINS_COUNT = String(chains.length);
  for (const chain of chains) {
    const chainIndex = chain.index;
    const hostAddresses = state.discovery?.hosts[chain.key] ?? {};
    envs["coprocessor"][`HOST_CHAIN_${chainIndex}_ID`] = chain.chainId;
    envs["coprocessor"][`HOST_CHAIN_${chainIndex}_NAME`] = chain.key;
    envs["coprocessor"][`HOST_CHAIN_${chainIndex}_ACL`] =
      hostAddresses.ACL_CONTRACT_ADDRESS ?? "";
  }

  const instanceEnvs = await buildInstanceEnvs(envs, plan, deriveWallet);
  envs["test-suite"].GATEWAY_DEPLOYER_PRIVATE_KEY = envs["gateway-sc"].DEPLOYER_PRIVATE_KEY;
  envs["test-suite"].GATEWAY_PAUSER_PRIVATE_KEY = envs["gateway-sc"].PAUSER_PRIVATE_KEY;
  envs["test-suite"].PRIORITY_COPROCESSOR_TX_SENDER_ADDRESS =
    envs["gateway-sc"].COPROCESSOR_TX_SENDER_ADDRESS_0;
  Object.assign(instanceEnvs, buildKmsConnectorInstanceEnvs(envs, kmsParties));

  // Propagate SNS-worker S3 migration configuration.
  // These are carried in version locks (see rollouts/.../versions.ts) so that
  // rollout phases can request "concurrent" etc. without touching the compose
  // command line (old pre-feature binaries must not see new --flags).
  // We write them into the coprocessor env files (base + all instance + chain copies)
  // so the clap parser (with env=) in the new binary picks them up.
  const migrationMode = plan.versions.env.S3_MIGRATION_MODE ?? "no";
  const cleanOld = plan.versions.env.CLEAN_OLD_S3_FORMAT_VERSION ?? "false";
  envs["coprocessor"].S3_MIGRATION_MODE = migrationMode;
  envs["coprocessor"].CLEAN_OLD_S3_FORMAT_VERSION = cleanOld;
  // Also push into any per-instance envs that were already built (they clone the base).
  for (const [name, inst] of Object.entries(instanceEnvs)) {
    if (name.startsWith("coprocessor")) {
      inst.S3_MIGRATION_MODE = migrationMode;
      inst.CLEAN_OLD_S3_FORMAT_VERSION = cleanOld;
    }
  }
  // The later host-chain coprocessor-*.N copies are built from the base + spreads
  // in the loop below, so they will inherit the values we just set on the base.

  // Uniform per-chain gateway-sc indexed vars for ALL host chains.
  envs["gateway-sc"].NUM_HOST_CHAINS = String(chains.length);
  for (const chain of chains) {
    const chainIndex = chain.index;
    const hostAddresses = state.discovery?.hosts[chain.key] ?? {};
    const metadata = defaultHostChainMetadata(chain, chainIndex);
    envs["gateway-sc"][`HOST_CHAIN_CHAIN_ID_${chainIndex}`] = chain.chainId;
    envs["gateway-sc"][`HOST_CHAIN_FHEVM_EXECUTOR_ADDRESS_${chainIndex}`] =
      hostAddresses.FHEVM_EXECUTOR_CONTRACT_ADDRESS ?? "";
    envs["gateway-sc"][`HOST_CHAIN_ACL_ADDRESS_${chainIndex}`] =
      hostAddresses.ACL_CONTRACT_ADDRESS ?? "";
    envs["gateway-sc"][`HOST_CHAIN_NAME_${chainIndex}`] = metadata.name;
    envs["gateway-sc"][`HOST_CHAIN_WEBSITE_${chainIndex}`] = metadata.website;
  }

  // Non-default chain infrastructure: host-node, host-sc, coprocessor, and test-suite env files.
  for (const chain of chains.filter((item) => !item.isDefault)) {
    const chainIndex = chain.index;
    const hostHttp = `http://${chain.node}:${chain.rpcPort}`;
    const hostWs = `ws://${chain.node}:${chain.rpcPort}`;
    const hostAddresses = state.discovery?.hosts[chain.key] ?? {};

    instanceEnvs[chain.node] = {
      ...envs["host-node"],
      HOST_NODE_CONTAINER_NAME: chain.node,
      HOST_NODE_PORT: String(chain.rpcPort),
      HOST_NODE_CHAIN_ID: chain.chainId,
    };

    const hostSc = { ...envs["host-sc"] };
    hostSc.RPC_URL = hostHttp;
    hostSc.CHAIN_ID = chain.chainId;
    hostSc.HOST_ADDRESS_DIR = chain.key;
    hostSc.HOST_SC_DEPLOY_KMS_GENERATION_ARGS = hostDeployKmsGenerationArgs(plan, false);
    hostSc.HOST_SC_DEPLOY_CONTAINER_NAME = `${chain.sc}-deploy`;
    hostSc.HOST_SC_PAUSERS_CONTAINER_NAME = `${chain.sc}-add-pausers`;
    hostSc.NUM_COPROCESSORS = String(plan.topology.count);
    hostSc.COPROCESSOR_THRESHOLD = String(plan.topology.threshold);
    for (let i = 0; i < plan.topology.count; i += 1) {
      const signer = envs["host-sc"][`COPROCESSOR_SIGNER_ADDRESS_${i}`];
      if (signer) hostSc[`COPROCESSOR_SIGNER_ADDRESS_${i}`] = signer;
    }
    instanceEnvs[chain.sc] = hostSc;

    for (let index = 0; index < plan.topology.count; index += 1) {
      const baseKey = index === 0 ? "coprocessor" : `coprocessor.${index}`;
      const baseEnv = index === 0 ? envs["coprocessor"] : instanceEnvs[baseKey];
      if (!baseEnv) continue;
      const coproChain = { ...baseEnv };
      coproChain.RPC_HTTP_URL = hostHttp;
      coproChain.RPC_WS_URL = hostWs;
      coproChain.CHAIN_ID = chain.chainId;
      if (hostAddresses.ACL_CONTRACT_ADDRESS) {
        coproChain.ACL_CONTRACT_ADDRESS = hostAddresses.ACL_CONTRACT_ADDRESS;
        coproChain.FHEVM_EXECUTOR_CONTRACT_ADDRESS = hostAddresses.FHEVM_EXECUTOR_CONTRACT_ADDRESS;
        coproChain.INPUT_VERIFIER_ADDRESS = hostAddresses.INPUT_VERIFIER_CONTRACT_ADDRESS;
      }
      coproChain.CONFIDENTIAL_BRIDGE_CONTRACT_ADDRESS = hostAddresses.CONFIDENTIAL_BRIDGE_CONTRACT_ADDRESS ?? "";
      instanceEnvs[`coprocessor-${chain.key}.${index}`] = coproChain;
    }

    envs["test-suite"][`HOST_CHAIN_${chainIndex}_RPC_URL`] = hostHttp;
    envs["test-suite"][`HOST_CHAIN_${chainIndex}_CHAIN_ID`] = chain.chainId;
    envs["test-suite"][`HOST_CHAIN_${chainIndex}_ACL_CONTRACT_ADDRESS`] = hostAddresses.ACL_CONTRACT_ADDRESS ?? "";
    envs["test-suite"][`HOST_CHAIN_${chainIndex}_KMS_VERIFIER_CONTRACT_ADDRESS`] = hostAddresses.KMS_VERIFIER_CONTRACT_ADDRESS ?? "";
    envs["test-suite"][`HOST_CHAIN_${chainIndex}_INPUT_VERIFIER_CONTRACT_ADDRESS`] = hostAddresses.INPUT_VERIFIER_CONTRACT_ADDRESS ?? "";
    envs["test-suite"][`HOST_CHAIN_${chainIndex}_FHEVM_EXECUTOR_CONTRACT_ADDRESS`] = hostAddresses.FHEVM_EXECUTOR_CONTRACT_ADDRESS ?? "";
    envs["test-suite"][`HOST_CHAIN_${chainIndex}_CONFIDENTIAL_BRIDGE_CONTRACT_ADDRESS`] = hostAddresses.CONFIDENTIAL_BRIDGE_CONTRACT_ADDRESS ?? "";
    envs["test-suite"][`HOST_CHAIN_${chainIndex}_LZ_ENDPOINT_ADDRESS`] = hostAddresses.LZ_ENDPOINT_ADDRESS ?? "";
    envs["test-suite"][`HOST_CHAIN_${chainIndex}_PROTOCOL_CONFIG_CONTRACT_ADDRESS`] = hostAddresses.PROTOCOL_CONFIG_CONTRACT_ADDRESS ?? "";
  }

  validateEnvMaps(envs, instanceEnvs);
  const compat = compatPolicyForState(plan);

  const versionsEnv: Record<string, string> = { ...plan.versions.env, ...compat.composeEnv };
  // Threshold + Test params: keygen/crsgen triggers read ${KEYGEN_PARAMS_TYPE}
  // from the compose env (versions.env) → ParamsType.Test (=1).
  if (plan.kms.mode === "threshold" && plan.kms.fheParams === "Test") {
    versionsEnv.KEYGEN_PARAMS_TYPE = "1";
  }
  return { componentEnvs: envs, instanceEnvs, versionsEnv };
};
