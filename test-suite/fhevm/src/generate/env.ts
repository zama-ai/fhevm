/**
 * Renders runtime env maps from resolved versions, scenario topology, discovery outputs, and compat policy.
 */
import {
  compatPolicyForState,
  requiresLegacyRelayerUrl,
  requiresMultichainAclAddress,
} from "../compat/compat";
import { driftDatabaseName } from "../drift";
import type { StackSpec } from "../stack-spec/stack-spec";
import {
  COPROCESSOR_WALLET_INDICES,
  DEFAULT_SOLANA_HOST_FAUCET_PORT,
  DEFAULT_SOLANA_HOST_WS_PORT,
  DEFAULT_TENANT_API_KEY,
  MINIO_INTERNAL_URL,
  POSTGRES_HOST,
  SOLANA_CONFIDENTIAL_TOKEN_PROGRAM_ID,
  SOLANA_HOST_NODE_IMAGE,
  SOLANA_HOST_NODE_PLATFORM,
  SOLANA_HOST_PROGRAM_ID,
  SOLANA_TEST_INPUT_PROGRAM_ID,
  hostChainKind,
  hostChainRuntimes,
} from "../layout";
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

const solanaAclIdentity = (values: Record<string, string>) =>
  values.SOLANA_HOST_ACL_PROGRAM_ID ?? values.SOLANA_HOST_PROGRAM_ID ?? values.ACL_CONTRACT_ADDRESS ?? "";

const defaultGatewayHostCompatAddress = "0x05fD9B5EFE0a996095f42Ed7e77c390810CF660c";

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
  envs["gateway-sc"].NUM_COPROCESSORS = String(plan.topology.count);
  envs["gateway-sc"].COPROCESSOR_THRESHOLD = String(plan.topology.threshold);
  envs["host-sc"].NUM_COPROCESSORS = String(plan.topology.count);
  envs["host-sc"].COPROCESSOR_THRESHOLD = String(plan.topology.threshold);
};

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
  envs["coprocessor"].TENANT_API_KEY = DEFAULT_TENANT_API_KEY;
  envs["coprocessor"].COPROCESSOR_API_KEY = DEFAULT_TENANT_API_KEY;
  envs["coprocessor"].AWS_ENDPOINT_URL = state.discovery?.endpoints.minioExternal ?? MINIO_INTERNAL_URL;
  envs["coprocessor"].FHE_KEY_ID = fheKeyId;
  envs["coprocessor"].KMS_PUBLIC_KEY = `${minioInternal}/kms-public/${keyPrefix}/PublicKey/${fheKeyId}`;
  envs["coprocessor"].KMS_SERVER_KEY = `${minioInternal}/kms-public/${keyPrefix}/ServerKey/${fheKeyId}`;
  envs["coprocessor"].KMS_SNS_KEY = `${minioInternal}/kms-public/${keyPrefix}/SnsKey/${fheKeyId}`;
  envs["coprocessor"].KMS_CRS_KEY = `${minioInternal}/kms-public/${keyPrefix}/CRS/${crsKeyId}`;
  envs["relayer"].APP_KEYURL__FHE_PUBLIC_KEY__URL = `${minioInternal}/kms-public/${keyPrefix}/PublicKey/${fheKeyId}`;
  envs["relayer"].APP_KEYURL__CRS__URL = `${minioInternal}/kms-public/${keyPrefix}/CRS/${crsKeyId}`;
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
  if (state.discovery?.kmsSigner) {
    envs["gateway-sc"].KMS_SIGNER_ADDRESS_0 = state.discovery.kmsSigner;
    envs["host-sc"].KMS_SIGNER_ADDRESS_0 = state.discovery.kmsSigner;
  }
  if (!state.discovery) {
    return;
  }

  const chains = hostChainRuntimes(plan.hostChains);
  const defaultChain = chains[0];
  if (!defaultChain) {
    return;
  }
  const primaryHost = state.discovery.hosts[defaultChain.key] ?? {};

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
    KMS_GENERATION_ADDRESS: state.discovery.gateway.KMS_GENERATION_ADDRESS,
  });

  const kmsHostChains = chains.map((chain) => {
    const hostAddresses = state.discovery!.hosts[chain.key] ?? {};
    const endpoints = state.discovery!.endpoints.hosts[chain.key];
    const base = {
      url: endpoints?.http ?? `http://${chain.node}:${chain.rpcPort}`,
      chain_id: Number(chain.chainId),
      acl_address: hostChainKind(chain) === "solana" ? solanaAclIdentity(hostAddresses) : hostAddresses.ACL_CONTRACT_ADDRESS ?? "",
    };
    return hostChainKind(chain) === "solana"
      ? {
          ...base,
          chain_kind: "solana",
          state_pda: hostAddresses.SOLANA_HOST_STATE_PDA ?? "",
        }
      : base;
  });
  updateContracts(envs["kms-connector"], {
    KMS_CONNECTOR_DECRYPTION_CONTRACT__ADDRESS: state.discovery.gateway.DECRYPTION_ADDRESS,
    KMS_CONNECTOR_GATEWAY_CONFIG_CONTRACT__ADDRESS: state.discovery.gateway.GATEWAY_CONFIG_ADDRESS,
    KMS_CONNECTOR_KMS_GENERATION_CONTRACT__ADDRESS: state.discovery.gateway.KMS_GENERATION_ADDRESS,
    KMS_CONNECTOR_HOST_CHAINS: JSON.stringify(kmsHostChains),
  });
  updateContracts(envs["relayer"], {
    APP_GATEWAY__CONTRACTS__DECRYPTION_ADDRESS: state.discovery.gateway.DECRYPTION_ADDRESS,
    APP_GATEWAY__CONTRACTS__INPUT_VERIFICATION_ADDRESS: state.discovery.gateway.INPUT_VERIFICATION_ADDRESS,
  });
  updateContracts(envs["test-suite"], {
    DECRYPTION_ADDRESS: state.discovery.gateway.DECRYPTION_ADDRESS,
    INPUT_VERIFICATION_ADDRESS: state.discovery.gateway.INPUT_VERIFICATION_ADDRESS,
    KMS_VERIFIER_CONTRACT_ADDRESS: primaryHost.KMS_VERIFIER_CONTRACT_ADDRESS,
    ACL_CONTRACT_ADDRESS: primaryHost.ACL_CONTRACT_ADDRESS,
    INPUT_VERIFIER_CONTRACT_ADDRESS: primaryHost.INPUT_VERIFIER_CONTRACT_ADDRESS,
    FHEVM_EXECUTOR_CONTRACT_ADDRESS: primaryHost.FHEVM_EXECUTOR_CONTRACT_ADDRESS,
  });
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
  applyBaseRuntimeEnv(envs, state);
  applyCompatEnv(envs, plan);
  applyDiscoveryEnv(envs, state, plan);
  const defaultHost = state.discovery?.hosts[defaultChain.key] ?? {};
  const defaultHostHttp = `http://${defaultChain.node}:${defaultChain.rpcPort}`;
  const defaultHostWs =
    hostChainKind(defaultChain) === "solana"
      ? `ws://${defaultChain.node}:${DEFAULT_SOLANA_HOST_WS_PORT}`
      : `ws://${defaultChain.node}:${defaultChain.rpcPort}`;
  if (hostChainKind(defaultChain) === "solana") {
    envs["host-node"].SOLANA_HOST_NODE_IMAGE = SOLANA_HOST_NODE_IMAGE;
    envs["host-node"].SOLANA_HOST_NODE_PLATFORM = SOLANA_HOST_NODE_PLATFORM;
    envs["host-node"].SOLANA_HOST_RPC_PORT = String(defaultChain.rpcPort);
    envs["host-node"].SOLANA_HOST_WS_PORT = String(DEFAULT_SOLANA_HOST_WS_PORT);
    envs["host-node"].SOLANA_HOST_FAUCET_PORT = String(DEFAULT_SOLANA_HOST_FAUCET_PORT);
    envs["host-node"].SOLANA_HOST_PROGRAM_ID = SOLANA_HOST_PROGRAM_ID;
    envs["host-node"].SOLANA_TEST_INPUT_PROGRAM_ID = SOLANA_TEST_INPUT_PROGRAM_ID;
    envs["host-node"].SOLANA_CONFIDENTIAL_TOKEN_PROGRAM_ID = SOLANA_CONFIDENTIAL_TOKEN_PROGRAM_ID;
    envs["coprocessor"].SOLANA_HOST_LISTENER_RPC_URL = defaultHostHttp;
    envs["coprocessor"].SOLANA_HOST_LISTENER_PROGRAM_ID = defaultHost.SOLANA_HOST_PROGRAM_ID ?? SOLANA_HOST_PROGRAM_ID;
    envs["coprocessor"].SOLANA_HOST_LISTENER_HOST_CHAIN_ID = defaultChain.chainId;
    envs["coprocessor"].SOLANA_HOST_LISTENER_COMMITMENT = "confirmed";
    envs["coprocessor"].RPC_HTTP_URL = defaultHostHttp;
    envs["coprocessor"].RPC_WS_URL = defaultHostWs;
    envs["coprocessor"].CHAIN_ID = defaultChain.chainId;
    envs["coprocessor"].ACL_CONTRACT_ADDRESS = solanaAclIdentity(defaultHost);
    envs["kms-connector"].KMS_CONNECTOR_ETHEREUM_URL = "http://gateway-node:8546";
    envs["test-suite"].RPC_URL = defaultHostHttp;
    envs["test-suite"].CHAIN_ID_HOST = defaultChain.chainId;
    envs["test-suite"].SOLANA_HOST_RPC_URL = `http://localhost:${defaultChain.rpcPort}`;
  } else {
    envs["host-node"].RPC_URL = defaultHostHttp;
    envs["host-node"].HOST_NODE_PORT = String(defaultChain.rpcPort);
    envs["host-node"].HOST_NODE_CHAIN_ID = defaultChain.chainId;
    envs["host-sc"].RPC_URL = defaultHostHttp;
    envs["host-sc"].HOST_ADDRESS_DIR = defaultChain.key;
    envs["coprocessor"].RPC_HTTP_URL = defaultHostHttp;
    envs["coprocessor"].RPC_WS_URL = defaultHostWs;
    envs["kms-connector"].KMS_CONNECTOR_ETHEREUM_URL = defaultHostHttp;
    envs["test-suite"].RPC_URL = defaultHostHttp;
    envs["test-suite"].CHAIN_ID_HOST = defaultChain.chainId;
  }
  const instanceEnvs = await buildInstanceEnvs(envs, plan, deriveWallet);

  // Uniform per-chain gateway-sc indexed vars for ALL host chains.
  envs["gateway-sc"].NUM_HOST_CHAINS = String(chains.length);
  for (const chain of chains) {
    const chainIndex = chain.index;
    const hostAddresses = state.discovery?.hosts[chain.key] ?? {};
    const metadata = defaultHostChainMetadata(chain, chainIndex);
    envs["gateway-sc"][`HOST_CHAIN_CHAIN_ID_${chainIndex}`] = chain.chainId;
    const compatGatewayAddress =
      hostChainKind(chain) === "evm"
        ? hostAddresses.ACL_CONTRACT_ADDRESS ?? envs["gateway-sc"][`HOST_CHAIN_ACL_ADDRESS_${chainIndex}`]
        : envs["gateway-sc"].HOST_CHAIN_ACL_ADDRESS_0 ?? defaultGatewayHostCompatAddress;
    const executorAddress =
      hostChainKind(chain) === "evm"
        ? hostAddresses.FHEVM_EXECUTOR_CONTRACT_ADDRESS ?? compatGatewayAddress
        : compatGatewayAddress;
    if (executorAddress) {
      envs["gateway-sc"][`HOST_CHAIN_FHEVM_EXECUTOR_ADDRESS_${chainIndex}`] = executorAddress;
    }
    if (compatGatewayAddress) {
      envs["gateway-sc"][`HOST_CHAIN_ACL_ADDRESS_${chainIndex}`] = compatGatewayAddress;
    }
    envs["gateway-sc"][`HOST_CHAIN_NAME_${chainIndex}`] = metadata.name;
    envs["gateway-sc"][`HOST_CHAIN_WEBSITE_${chainIndex}`] = metadata.website;
  }

  // Non-default chain infrastructure: host-node, host-sc, coprocessor, and test-suite env files.
  for (const chain of chains.filter((item) => !item.isDefault)) {
    const chainIndex = chain.index;
    const hostHttp = `http://${chain.node}:${chain.rpcPort}`;
    const hostWs =
      hostChainKind(chain) === "solana"
        ? `ws://${chain.node}:${DEFAULT_SOLANA_HOST_WS_PORT}`
        : `ws://${chain.node}:${chain.rpcPort}`;
    const hostAddresses = state.discovery?.hosts[chain.key] ?? {};

    if (hostChainKind(chain) === "solana") {
      instanceEnvs[chain.node] = {
        ...envs["host-node"],
        SOLANA_HOST_NODE_IMAGE: SOLANA_HOST_NODE_IMAGE,
        SOLANA_HOST_NODE_PLATFORM: SOLANA_HOST_NODE_PLATFORM,
        SOLANA_HOST_RPC_PORT: String(chain.rpcPort),
        SOLANA_HOST_WS_PORT: String(DEFAULT_SOLANA_HOST_WS_PORT),
        SOLANA_HOST_FAUCET_PORT: String(DEFAULT_SOLANA_HOST_FAUCET_PORT),
        SOLANA_HOST_PROGRAM_ID: SOLANA_HOST_PROGRAM_ID,
        SOLANA_TEST_INPUT_PROGRAM_ID: SOLANA_TEST_INPUT_PROGRAM_ID,
        SOLANA_CONFIDENTIAL_TOKEN_PROGRAM_ID: SOLANA_CONFIDENTIAL_TOKEN_PROGRAM_ID,
      };
    } else {
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
      hostSc.HOST_SC_DEPLOY_CONTAINER_NAME = `${chain.sc}-deploy`;
      hostSc.HOST_SC_PAUSERS_CONTAINER_NAME = `${chain.sc}-add-pausers`;
      hostSc.NUM_COPROCESSORS = String(plan.topology.count);
      hostSc.COPROCESSOR_THRESHOLD = String(plan.topology.threshold);
      for (let i = 0; i < plan.topology.count; i += 1) {
        const signer = envs["host-sc"][`COPROCESSOR_SIGNER_ADDRESS_${i}`];
        if (signer) hostSc[`COPROCESSOR_SIGNER_ADDRESS_${i}`] = signer;
      }
      instanceEnvs[chain.sc] = hostSc;
    }

    for (let index = 0; index < plan.topology.count; index += 1) {
      const baseKey = index === 0 ? "coprocessor" : `coprocessor.${index}`;
      const baseEnv = index === 0 ? envs["coprocessor"] : instanceEnvs[baseKey];
      if (!baseEnv) continue;
      const coproChain = { ...baseEnv };
      coproChain.RPC_HTTP_URL = hostHttp;
      coproChain.RPC_WS_URL = hostWs;
      coproChain.CHAIN_ID = chain.chainId;
      if (hostChainKind(chain) === "solana") {
        coproChain.SOLANA_HOST_LISTENER_RPC_URL = hostHttp;
        coproChain.SOLANA_HOST_LISTENER_PROGRAM_ID = hostAddresses.SOLANA_HOST_PROGRAM_ID ?? SOLANA_HOST_PROGRAM_ID;
        coproChain.SOLANA_HOST_LISTENER_HOST_CHAIN_ID = chain.chainId;
        coproChain.SOLANA_HOST_LISTENER_COMMITMENT = "confirmed";
        coproChain.ACL_CONTRACT_ADDRESS = solanaAclIdentity(hostAddresses);
      } else if (hostAddresses.ACL_CONTRACT_ADDRESS) {
        coproChain.ACL_CONTRACT_ADDRESS = hostAddresses.ACL_CONTRACT_ADDRESS;
        coproChain.FHEVM_EXECUTOR_CONTRACT_ADDRESS = hostAddresses.FHEVM_EXECUTOR_CONTRACT_ADDRESS;
        coproChain.INPUT_VERIFIER_ADDRESS = hostAddresses.INPUT_VERIFIER_CONTRACT_ADDRESS;
      }
      instanceEnvs[`coprocessor-${chain.key}.${index}`] = coproChain;
    }

    envs["test-suite"][`HOST_CHAIN_${chainIndex}_RPC_URL`] = hostHttp;
    envs["test-suite"][`HOST_CHAIN_${chainIndex}_CHAIN_ID`] = chain.chainId;
    envs["test-suite"][`HOST_CHAIN_${chainIndex}_ACL_CONTRACT_ADDRESS`] =
      hostChainKind(chain) === "solana" ? solanaAclIdentity(hostAddresses) : hostAddresses.ACL_CONTRACT_ADDRESS ?? "";
    envs["test-suite"][`HOST_CHAIN_${chainIndex}_KMS_VERIFIER_CONTRACT_ADDRESS`] = hostAddresses.KMS_VERIFIER_CONTRACT_ADDRESS ?? "";
    envs["test-suite"][`HOST_CHAIN_${chainIndex}_INPUT_VERIFIER_CONTRACT_ADDRESS`] = hostAddresses.INPUT_VERIFIER_CONTRACT_ADDRESS ?? "";
    envs["test-suite"][`HOST_CHAIN_${chainIndex}_FHEVM_EXECUTOR_CONTRACT_ADDRESS`] = hostAddresses.FHEVM_EXECUTOR_CONTRACT_ADDRESS ?? "";
    if (hostChainKind(chain) === "solana") {
      envs["test-suite"].CHAIN_ID_HOST_SOLANA = chain.chainId;
      envs["test-suite"].SOLANA_HOST_RPC_URL = `http://localhost:${chain.rpcPort}`;
    }
  }

  validateEnvMaps(envs, instanceEnvs);
  const compat = compatPolicyForState(plan);

  return {
    componentEnvs: envs,
    instanceEnvs,
    versionsEnv: { ...plan.versions.env, ...compat.composeEnv },
  };
};
