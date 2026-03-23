/**
 * Renders runtime env maps from resolved versions, scenario topology, discovery outputs, and compat policy.
 */
import {
  compatPolicyForState,
  requiresLegacyRelayerUrl,
  requiresMultichainAclAddress,
} from "../compat/compat";
import type { StackSpec } from "../stack-spec/stack-spec";
import {
  CHAIN_B_ID,
  CHAIN_B_PORT,
  COPROCESSOR_WALLET_INDICES,
  DEFAULT_CHAIN_ID,
  DEFAULT_TENANT_API_KEY,
  MINIO_INTERNAL_URL,
  POSTGRES_HOST,
} from "../layout";
import type { State } from "../types";
import { predictedCrsId, predictedKeyId } from "../utils/fs";
import { interpolateString } from "./compose";

export type WalletMaterial = {
  address: string;
  privateKey: string;
};

const HAS_PLACEHOLDER = /(?<!\$)\$\{[A-Z0-9_]+\}/;

/** Resolves env interpolation until placeholders disappear or a fixed point is reached. */
export const resolveEnvMap = (env: Record<string, string>) => {
  const unresolvedKeys = () =>
    Object.entries(env)
      .filter(([, value]) => HAS_PLACEHOLDER.test(value))
      .map(([key]) => key);
  for (let attempt = 0; attempt < 4; attempt += 1) {
    let changed = false;
    for (const [key, raw] of Object.entries(env)) {
      const value = typeof raw === "string" ? raw : "";
      const next = interpolateString(value, env);
      if (next !== value) {
        env[key] = next;
        changed = true;
      }
    }
    if (!changed) {
      const unresolved = unresolvedKeys();
      if (unresolved.length) {
        throw new Error(`Unresolved env interpolation for ${unresolved.join(", ")}`);
      }
      return env;
    }
  }
  const unresolved = unresolvedKeys();
  if (unresolved.length) {
    throw new Error(`Unresolved env interpolation for ${unresolved.join(", ")}`);
  }
  return env;
};

/** Applies contract addresses into a component env map when values are available. */
const updateContracts = (env: Record<string, string>, values: Record<string, string>) => {
  for (const [key, value] of Object.entries(values)) {
    if (value !== undefined) {
      env[key] = value;
    }
  }
};

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

  updateContracts(envs["gateway-sc"], state.discovery.gateway);
  updateContracts(envs["gateway-mocked-payment"], {
    PROTOCOL_PAYMENT_ADDRESS: state.discovery.gateway.PROTOCOL_PAYMENT_ADDRESS,
  });
  updateContracts(envs["host-sc"], {
    DECRYPTION_ADDRESS: state.discovery.gateway.DECRYPTION_ADDRESS,
    INPUT_VERIFICATION_ADDRESS: state.discovery.gateway.INPUT_VERIFICATION_ADDRESS,
    ACL_CONTRACT_ADDRESS: state.discovery.host.ACL_CONTRACT_ADDRESS,
    PAUSER_SET_CONTRACT_ADDRESS: state.discovery.host.PAUSER_SET_CONTRACT_ADDRESS,
  });
  envs["gateway-sc"].HOST_CHAIN_FHEVM_EXECUTOR_ADDRESS_0 = state.discovery.host.FHEVM_EXECUTOR_CONTRACT_ADDRESS;
  envs["gateway-sc"].HOST_CHAIN_ACL_ADDRESS_0 = state.discovery.host.ACL_CONTRACT_ADDRESS;
  updateContracts(envs["coprocessor"], {
    ACL_CONTRACT_ADDRESS: state.discovery.host.ACL_CONTRACT_ADDRESS,
    FHEVM_EXECUTOR_CONTRACT_ADDRESS: state.discovery.host.FHEVM_EXECUTOR_CONTRACT_ADDRESS,
    INPUT_VERIFIER_ADDRESS: state.discovery.host.INPUT_VERIFIER_CONTRACT_ADDRESS,
    INPUT_VERIFICATION_ADDRESS: state.discovery.gateway.INPUT_VERIFICATION_ADDRESS,
    CIPHERTEXT_COMMITS_ADDRESS: state.discovery.gateway.CIPHERTEXT_COMMITS_ADDRESS,
    ...(requiresMultichainAclAddress(plan) ? { MULTICHAIN_ACL_ADDRESS: state.discovery.gateway.MULTICHAIN_ACL_ADDRESS } : {}),
    KMS_GENERATION_ADDRESS: state.discovery.gateway.KMS_GENERATION_ADDRESS,
  });
  updateContracts(envs["kms-connector"], {
    KMS_CONNECTOR_DECRYPTION_CONTRACT__ADDRESS: state.discovery.gateway.DECRYPTION_ADDRESS,
    KMS_CONNECTOR_GATEWAY_CONFIG_CONTRACT__ADDRESS: state.discovery.gateway.GATEWAY_CONFIG_ADDRESS,
    KMS_CONNECTOR_KMS_GENERATION_CONTRACT__ADDRESS: state.discovery.gateway.KMS_GENERATION_ADDRESS,
    KMS_CONNECTOR_HOST_CHAINS: JSON.stringify([
      {
        url: state.discovery.endpoints.hostHttp,
        chain_id: Number(envs["coprocessor"].CHAIN_ID ?? DEFAULT_CHAIN_ID),
        acl_address: state.discovery.host.ACL_CONTRACT_ADDRESS,
      },
    ]),
  });
  updateContracts(envs["relayer"], {
    APP_GATEWAY__CONTRACTS__DECRYPTION_ADDRESS: state.discovery.gateway.DECRYPTION_ADDRESS,
    APP_GATEWAY__CONTRACTS__INPUT_VERIFICATION_ADDRESS: state.discovery.gateway.INPUT_VERIFICATION_ADDRESS,
  });
  updateContracts(envs["test-suite"], {
    DECRYPTION_ADDRESS: state.discovery.gateway.DECRYPTION_ADDRESS,
    INPUT_VERIFICATION_ADDRESS: state.discovery.gateway.INPUT_VERIFICATION_ADDRESS,
    KMS_VERIFIER_CONTRACT_ADDRESS: state.discovery.host.KMS_VERIFIER_CONTRACT_ADDRESS,
    ACL_CONTRACT_ADDRESS: state.discovery.host.ACL_CONTRACT_ADDRESS,
    INPUT_VERIFIER_CONTRACT_ADDRESS: state.discovery.host.INPUT_VERIFIER_CONTRACT_ADDRESS,
    FHEVM_EXECUTOR_CONTRACT_ADDRESS: state.discovery.host.FHEVM_EXECUTOR_CONTRACT_ADDRESS,
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
    next.DATABASE_URL = `postgresql://${envs.database.POSTGRES_USER}:${envs.database.POSTGRES_PASSWORD}@${POSTGRES_HOST}/coprocessor_${index}`;
    next.TX_SENDER_PRIVATE_KEY = wallet.privateKey;
    const instance = plan.coprocessor.instances.find((item) => item.index === index);
    Object.assign(next, instance?.env ?? {});
    instanceEnvs[`coprocessor.${index}`] = next;
  }
  return instanceEnvs;
};

/** Resolves interpolation across all component and per-instance env maps. */
const resolveAllEnvMaps = (
  envs: Record<string, Record<string, string>>,
  instanceEnvs: Record<string, Record<string, string>>,
) => {
  for (const env of [...Object.values(envs), ...Object.values(instanceEnvs)]) {
    resolveEnvMap(env);
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
  applyTopologyEnv(envs, plan);
  applyBaseRuntimeEnv(envs, state);
  applyCompatEnv(envs, plan);
  applyDiscoveryEnv(envs, state, plan);
  const instanceEnvs = await buildInstanceEnvs(envs, plan, deriveWallet);

  if (plan.multiChain) {
    const hostBHttp = `http://host-node-b:${CHAIN_B_PORT}`;
    const hostBWs = `ws://host-node-b:${CHAIN_B_PORT}`;

    envs["gateway-sc"].NUM_HOST_CHAINS = "2";
    envs["gateway-sc"].HOST_CHAIN_CHAIN_ID_1 = CHAIN_B_ID;
    envs["gateway-sc"].HOST_CHAIN_FHEVM_EXECUTOR_ADDRESS_1 =
      state.discovery?.hostB?.FHEVM_EXECUTOR_CONTRACT_ADDRESS ??
      envs["gateway-sc"].HOST_CHAIN_FHEVM_EXECUTOR_ADDRESS_0 ?? "";
    envs["gateway-sc"].HOST_CHAIN_ACL_ADDRESS_1 =
      state.discovery?.hostB?.ACL_CONTRACT_ADDRESS ??
      envs["gateway-sc"].HOST_CHAIN_ACL_ADDRESS_0 ?? "";
    envs["gateway-sc"].HOST_CHAIN_NAME_1 = "";
    envs["gateway-sc"].HOST_CHAIN_WEBSITE_1 = "";

    instanceEnvs["host-node-b"] = {
      ...envs["host-node"],
      HOST_NODE_CONTAINER_NAME: "host-node-b",
      HOST_NODE_PORT: String(CHAIN_B_PORT),
      HOST_NODE_CHAIN_ID: CHAIN_B_ID,
    };

    const hostScB = { ...envs["host-sc"] };
    hostScB.RPC_URL = hostBHttp;
    hostScB.CHAIN_ID = CHAIN_B_ID;
    hostScB.HOST_SC_DEPLOY_CONTAINER_NAME = "host-sc-b-deploy";
    hostScB.HOST_SC_PAUSERS_CONTAINER_NAME = "host-sc-b-add-pausers";
    hostScB.NUM_COPROCESSORS = String(plan.topology.count);
    hostScB.COPROCESSOR_THRESHOLD = String(plan.topology.threshold);
    for (let i = 0; i < plan.topology.count; i += 1) {
      const signer = envs["host-sc"][`COPROCESSOR_SIGNER_ADDRESS_${i}`];
      if (signer) {
        hostScB[`COPROCESSOR_SIGNER_ADDRESS_${i}`] = signer;
      }
    }
    instanceEnvs["host-sc-b"] = hostScB;

    for (let index = 0; index < plan.topology.count; index += 1) {
      const baseKey = index === 0 ? "coprocessor" : `coprocessor.${index}`;
      const baseEnv = index === 0 ? envs["coprocessor"] : instanceEnvs[baseKey];
      if (!baseEnv) continue;
      const coproB = { ...baseEnv };
      coproB.RPC_HTTP_URL = hostBHttp;
      coproB.RPC_WS_URL = hostBWs;
      coproB.CHAIN_ID = CHAIN_B_ID;
      if (state.discovery?.hostB) {
        coproB.ACL_CONTRACT_ADDRESS = state.discovery.hostB.ACL_CONTRACT_ADDRESS;
        coproB.FHEVM_EXECUTOR_CONTRACT_ADDRESS = state.discovery.hostB.FHEVM_EXECUTOR_CONTRACT_ADDRESS;
        coproB.INPUT_VERIFIER_ADDRESS = state.discovery.hostB.INPUT_VERIFIER_CONTRACT_ADDRESS;
      }
      instanceEnvs[`coprocessor-b.${index}`] = coproB;
    }

    envs["test-suite"].RPC_URL_CHAIN_B = hostBHttp;
    envs["test-suite"].CHAIN_ID_HOST_B = CHAIN_B_ID;

    if (state.discovery) {
      const chainAChainId = Number(envs["coprocessor"].CHAIN_ID ?? DEFAULT_CHAIN_ID);
      const chainBAcl = state.discovery.hostB?.ACL_CONTRACT_ADDRESS ?? state.discovery.host.ACL_CONTRACT_ADDRESS;
      envs["kms-connector"].KMS_CONNECTOR_HOST_CHAINS = JSON.stringify([
        {
          url: state.discovery.endpoints.hostHttp,
          chain_id: chainAChainId,
          acl_address: state.discovery.host.ACL_CONTRACT_ADDRESS,
        },
        {
          url: hostBHttp,
          chain_id: Number(CHAIN_B_ID),
          acl_address: chainBAcl,
        },
      ]);
    }
  }

  resolveAllEnvMaps(envs, instanceEnvs);

  return {
    componentEnvs: envs,
    instanceEnvs,
    versionsEnv: plan.versions.env,
  };
};
