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
  COPROCESSOR_WALLET_INDICES,
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

/** Renders component and per-instance env maps from state, topology, and discovery. */
export const renderEnvMaps = async (
  state: Pick<State, "discovery">,
  plan: StackSpec,
  templateEnvs: Record<string, Record<string, string>>,
  deriveWallet: (mnemonic: string, index: number) => Promise<WalletMaterial>,
) => {
  const envs = structuredClone(templateEnvs);
  const compat = compatPolicyForState(plan);
  const scenario = plan.coprocessor;

  envs["gateway-sc"].NUM_COPROCESSORS = String(plan.topology.count);
  envs["gateway-sc"].COPROCESSOR_THRESHOLD = String(plan.topology.threshold);
  envs["host-sc"].NUM_COPROCESSORS = String(plan.topology.count);
  envs["host-sc"].COPROCESSOR_THRESHOLD = String(plan.topology.threshold);
  envs["coprocessor"].DATABASE_URL = `postgresql://${envs.database.POSTGRES_USER}:${envs.database.POSTGRES_PASSWORD}@${POSTGRES_HOST}/coprocessor`;
  envs["coprocessor"].TENANT_API_KEY = DEFAULT_TENANT_API_KEY;
  envs["coprocessor"].COPROCESSOR_API_KEY = DEFAULT_TENANT_API_KEY;
  envs["coprocessor"].AWS_ENDPOINT_URL = state.discovery?.endpoints.minioExternal ?? MINIO_INTERNAL_URL;
  const kp = state.discovery?.minioKeyPrefix ?? "PUB";
  const minioInt = state.discovery?.endpoints.minioInternal ?? MINIO_INTERNAL_URL;
  envs["coprocessor"].FHE_KEY_ID = state.discovery?.actualFheKeyId ?? state.discovery?.fheKeyId ?? predictedKeyId();
  envs["coprocessor"].KMS_PUBLIC_KEY = `${minioInt}/kms-public/${kp}/PublicKey/${envs["coprocessor"].FHE_KEY_ID}`;
  envs["coprocessor"].KMS_SERVER_KEY = `${minioInt}/kms-public/${kp}/ServerKey/${envs["coprocessor"].FHE_KEY_ID}`;
  envs["coprocessor"].KMS_SNS_KEY = `${minioInt}/kms-public/${kp}/SnsKey/${envs["coprocessor"].FHE_KEY_ID}`;
  envs["coprocessor"].KMS_CRS_KEY = `${minioInt}/kms-public/${kp}/CRS/${state.discovery?.actualCrsKeyId ?? state.discovery?.crsKeyId ?? predictedCrsId()}`;
  envs["relayer"].APP_KEYURL__FHE_PUBLIC_KEY__URL = `${minioInt}/kms-public/${kp}/PublicKey/${state.discovery?.actualFheKeyId ?? state.discovery?.fheKeyId ?? predictedKeyId()}`;
  envs["relayer"].APP_KEYURL__CRS__URL = `${minioInt}/kms-public/${kp}/CRS/${state.discovery?.actualCrsKeyId ?? state.discovery?.crsKeyId ?? predictedCrsId()}`;

  for (const [key, source] of Object.entries(compat.connectorEnv)) {
    if (envs["kms-connector"][source]) {
      envs["kms-connector"][key] = envs["kms-connector"][source];
    }
  }

  if (state.discovery?.kmsSigner) {
    envs["gateway-sc"].KMS_SIGNER_ADDRESS_0 = state.discovery.kmsSigner;
    envs["host-sc"].KMS_SIGNER_ADDRESS_0 = state.discovery.kmsSigner;
  }
  if (state.discovery) {
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
          chain_id: Number(envs["coprocessor"].CHAIN_ID ?? "12345"),
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
  }

  if (!requiresLegacyRelayerUrl(plan)) {
    const base = envs["test-suite"].RELAYER_URL ?? "";
    if (base && !base.endsWith("/v2")) {
      envs["test-suite"].RELAYER_URL = `${base}/v2`;
    }
  }

  const instanceEnvs: Record<string, Record<string, string>> = {};
  const baseInstance = scenario.instances.find((instance) => instance.index === 0);
  if (plan.topology.count > 1) {
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
      const instance = scenario.instances.find((item) => item.index === index);
      Object.assign(next, instance?.env ?? {});
      resolveEnvMap(next);
      instanceEnvs[`coprocessor.${index}`] = next;
    }
  } else if (baseInstance) {
    Object.assign(envs["coprocessor"], baseInstance.env);
  }

  for (const env of Object.values(envs)) {
    resolveEnvMap(env);
  }

  return {
    componentEnvs: envs,
    instanceEnvs,
    versionsEnv: plan.versions.env,
  };
};
