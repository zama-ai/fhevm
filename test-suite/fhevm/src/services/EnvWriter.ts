import { Context, Effect, Layer } from "effect";
import fs from "node:fs/promises";
import path from "node:path";
import YAML from "yaml";
import { CommandRunner } from "./CommandRunner";
import type { State, VersionBundle } from "../types";
import {
  compatPolicyForState,
  requiresMultichainAclAddress,
  requiresLegacyRelayerReadinessConfig,
  requiresLegacyRelayerUrl,
} from "../compat";
import {
  COMPONENTS,
  TEMPLATE_ENV_DIR,
  ENV_DIR,
  envPath,
  versionsEnvPath,
  CONFIG_DIR,
  TEMPLATE_RELAYER_CONFIG,
  relayerConfigPath,
  ADDRESS_DIR,
  DEFAULT_TENANT_API_KEY,
} from "../layout";
import { readEnvFile, writeEnvFile, ensureDir, predictedKeyId, predictedCrsId } from "../utils";
import { interpolateString } from "../codegen";

const HAS_PLACEHOLDER = /(?<!\$)\$\{[A-Z0-9_]+\}/;

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

export const rewriteRelayerConfig = (
  config: Record<string, unknown>,
  state: Pick<State, "versions">,
) => {
  if (!requiresLegacyRelayerReadinessConfig(state)) {
    return config;
  }
  const gateway = config.gateway;
  if (!gateway || typeof gateway !== "object") {
    return config;
  }
  const readiness = (gateway as Record<string, unknown>).readiness_checker;
  if (!readiness || typeof readiness !== "object") {
    return config;
  }
  const current = readiness as Record<string, unknown>;
  (gateway as Record<string, unknown>).readiness_checker = Object.fromEntries(
    Object.entries({
      retry:
        current.retry ??
        (current.gw_ciphertext_check as Record<string, unknown> | undefined)?.retry ??
        (current.host_acl_check as Record<string, unknown> | undefined)?.retry,
      public_decrypt: current.public_decrypt,
      user_decrypt: current.user_decrypt,
      delegated_user_decrypt: current.delegated_user_decrypt,
    }).filter(([, value]) => value !== undefined),
  );
  return config;
};

const updateContracts = (env: Record<string, string>, values: Record<string, string>) => {
  for (const [key, value] of Object.entries(values)) {
    if (value !== undefined) {
      env[key] = value;
    }
  }
};

const ensureWritableDir = async (dir: string) => {
  await ensureDir(dir);
  await fs.chmod(dir, 0o777);
};

export class EnvWriter extends Context.Tag("EnvWriter")<
  EnvWriter,
  {
    readonly generateEnvFiles: (state: State) => Effect.Effect<void>;
    readonly writeVersionsEnv: (bundle: VersionBundle) => Effect.Effect<void>;
  }
>() {
  static Live = Layer.effect(
    EnvWriter,
    Effect.gen(function* () {
      const cmd = yield* CommandRunner;

      const deriveWallet = (mnemonic: string, index: number) =>
        Effect.gen(function* () {
          const [addrResult, keyResult] = yield* Effect.all(
            [
              cmd.run([
                "cast", "wallet", "address",
                "--mnemonic", mnemonic, "--mnemonic-index", String(index),
              ]),
              cmd.run([
                "cast", "wallet", "private-key",
                "--mnemonic", mnemonic, "--mnemonic-index", String(index),
              ]),
            ],
            { concurrency: 2 },
          );
          const address = addrResult.stdout.trim();
          const privateKey = keyResult.stdout.trim();
          if (!/^0x[a-fA-F0-9]{40}$/.test(address)) {
            return yield* Effect.die(
              new Error(`cast returned invalid address for wallet ${index}: ${address}`),
            );
          }
          if (!/^0x[a-fA-F0-9]{64}$/.test(privateKey)) {
            return yield* Effect.die(
              new Error(`cast returned invalid private key for wallet ${index}`),
            );
          }
          return { address, privateKey };
        }).pipe(
          Effect.catchTag("CommandError", (e) =>
            Effect.die(new Error(`cast wallet failed: ${e.stderr}`)),
          ),
        );

      return {
        generateEnvFiles: (state) =>
          Effect.gen(function* () {
            yield* Effect.promise(() =>
              Promise.all([
                ensureDir(ENV_DIR),
                ensureWritableDir(path.join(ADDRESS_DIR, "gateway")),
                ensureWritableDir(path.join(ADDRESS_DIR, "host")),
                ensureDir(CONFIG_DIR),
              ]),
            );

            const envs = yield* Effect.promise(async () =>
              Object.fromEntries(
                await Promise.all(
                  COMPONENTS.map(async (component) => [
                    component,
                    await readEnvFile(path.join(TEMPLATE_ENV_DIR, `.env.${component}`)),
                  ]),
                ),
              ) as Record<string, Record<string, string>>,
            );

            const compat = compatPolicyForState(state);

            // Inject topology
            envs["gateway-sc"].NUM_COPROCESSORS = String(state.topology.count);
            envs["gateway-sc"].COPROCESSOR_THRESHOLD = String(state.topology.threshold);
            envs["host-sc"].NUM_COPROCESSORS = String(state.topology.count);
            envs["host-sc"].COPROCESSOR_THRESHOLD = String(state.topology.threshold);
            envs["coprocessor"].DATABASE_URL = `postgresql://${envs.database.POSTGRES_USER}:${envs.database.POSTGRES_PASSWORD}@db:5432/coprocessor`;
            envs["coprocessor"].TENANT_API_KEY = DEFAULT_TENANT_API_KEY;
            envs["coprocessor"].COPROCESSOR_API_KEY = DEFAULT_TENANT_API_KEY;
            envs["coprocessor"].AWS_ENDPOINT_URL =
              state.discovery?.endpoints.minioExternal ?? "http://minio:9000";
            const kp = state.discovery?.minioKeyPrefix ?? "PUB";
            const minioInt = state.discovery?.endpoints.minioInternal ?? "http://minio:9000";
            envs["coprocessor"].FHE_KEY_ID =
              state.discovery?.actualFheKeyId ?? state.discovery?.fheKeyId ?? predictedKeyId();
            envs["coprocessor"].KMS_PUBLIC_KEY = `${minioInt}/kms-public/${kp}/PublicKey/${envs["coprocessor"].FHE_KEY_ID}`;
            envs["coprocessor"].KMS_SERVER_KEY = `${minioInt}/kms-public/${kp}/ServerKey/${envs["coprocessor"].FHE_KEY_ID}`;
            envs["coprocessor"].KMS_SNS_KEY = `${minioInt}/kms-public/${kp}/SnsKey/${envs["coprocessor"].FHE_KEY_ID}`;
            envs["coprocessor"].KMS_CRS_KEY = `${minioInt}/kms-public/${kp}/CRS/${state.discovery?.actualCrsKeyId ?? state.discovery?.crsKeyId ?? predictedCrsId()}`;
            envs["relayer"].APP_KEYURL__FHE_PUBLIC_KEY__URL = `${minioInt}/kms-public/${kp}/PublicKey/${state.discovery?.actualFheKeyId ?? state.discovery?.fheKeyId ?? predictedKeyId()}`;
            envs["relayer"].APP_KEYURL__CRS__URL = `${minioInt}/kms-public/${kp}/CRS/${state.discovery?.actualCrsKeyId ?? state.discovery?.crsKeyId ?? predictedCrsId()}`;

            // Compat connector env remapping
            for (const [key, source] of Object.entries(compat.connectorEnv)) {
              if (envs["kms-connector"][source]) {
                envs["kms-connector"][key] = envs["kms-connector"][source];
              }
            }

            // Discovery injection
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
              envs["gateway-sc"].HOST_CHAIN_FHEVM_EXECUTOR_ADDRESS_0 =
                state.discovery.host.FHEVM_EXECUTOR_CONTRACT_ADDRESS;
              envs["gateway-sc"].HOST_CHAIN_ACL_ADDRESS_0 =
                state.discovery.host.ACL_CONTRACT_ADDRESS;

              updateContracts(envs["coprocessor"], {
                ACL_CONTRACT_ADDRESS: state.discovery.host.ACL_CONTRACT_ADDRESS,
                FHEVM_EXECUTOR_CONTRACT_ADDRESS:
                  state.discovery.host.FHEVM_EXECUTOR_CONTRACT_ADDRESS,
                INPUT_VERIFIER_ADDRESS: state.discovery.host.INPUT_VERIFIER_CONTRACT_ADDRESS,
                INPUT_VERIFICATION_ADDRESS: state.discovery.gateway.INPUT_VERIFICATION_ADDRESS,
                CIPHERTEXT_COMMITS_ADDRESS: state.discovery.gateway.CIPHERTEXT_COMMITS_ADDRESS,
                ...(requiresMultichainAclAddress(state)
                  ? { MULTICHAIN_ACL_ADDRESS: state.discovery.gateway.MULTICHAIN_ACL_ADDRESS }
                  : {}),
                KMS_GENERATION_ADDRESS: state.discovery.gateway.KMS_GENERATION_ADDRESS,
              });
              updateContracts(envs["kms-connector"], {
                KMS_CONNECTOR_DECRYPTION_CONTRACT__ADDRESS:
                  state.discovery.gateway.DECRYPTION_ADDRESS,
                KMS_CONNECTOR_GATEWAY_CONFIG_CONTRACT__ADDRESS:
                  state.discovery.gateway.GATEWAY_CONFIG_ADDRESS,
                KMS_CONNECTOR_KMS_GENERATION_CONTRACT__ADDRESS:
                  state.discovery.gateway.KMS_GENERATION_ADDRESS,
                KMS_CONNECTOR_HOST_CHAINS: JSON.stringify([
                  {
                    url: state.discovery.endpoints.hostHttp,
                    chain_id: Number(envs["coprocessor"].CHAIN_ID ?? "12345"),
                    acl_address: state.discovery.host.ACL_CONTRACT_ADDRESS,
                  },
                ]),
              });
              updateContracts(envs["relayer"], {
                APP_GATEWAY__CONTRACTS__DECRYPTION_ADDRESS:
                  state.discovery.gateway.DECRYPTION_ADDRESS,
                APP_GATEWAY__CONTRACTS__INPUT_VERIFICATION_ADDRESS:
                  state.discovery.gateway.INPUT_VERIFICATION_ADDRESS,
              });
              updateContracts(envs["test-suite"], {
                DECRYPTION_ADDRESS: state.discovery.gateway.DECRYPTION_ADDRESS,
                INPUT_VERIFICATION_ADDRESS: state.discovery.gateway.INPUT_VERIFICATION_ADDRESS,
                KMS_VERIFIER_CONTRACT_ADDRESS:
                  state.discovery.host.KMS_VERIFIER_CONTRACT_ADDRESS,
                ACL_CONTRACT_ADDRESS: state.discovery.host.ACL_CONTRACT_ADDRESS,
                INPUT_VERIFIER_CONTRACT_ADDRESS:
                  state.discovery.host.INPUT_VERIFIER_CONTRACT_ADDRESS,
                FHEVM_EXECUTOR_CONTRACT_ADDRESS:
                  state.discovery.host.FHEVM_EXECUTOR_CONTRACT_ADDRESS,
              });
            }

            // Modern test-suite SDK (>= v0.11.0) expects RELAYER_URL to include /v2;
            // older SDKs append /v1/ internally, so the base URL must stay bare.
            if (!requiresLegacyRelayerUrl(state)) {
              const base = envs["test-suite"].RELAYER_URL ?? "";
              if (base && !base.endsWith("/v2")) {
                envs["test-suite"].RELAYER_URL = `${base}/v2`;
              }
            }

            // Multi-coprocessor wallet derivation
            const indices = [5, 8, 9, 10, 11];
            if (state.topology.count > 1) {
              const mnemonic = envs["gateway-sc"].MNEMONIC;
              if (!mnemonic) {
                return yield* Effect.die(
                  new Error("Missing gateway mnemonic for multicopro setup"),
                );
              }
              for (let index = 0; index < state.topology.count; index += 1) {
                const wallet = yield* deriveWallet(mnemonic, indices[index]);
                envs["gateway-sc"][`COPROCESSOR_TX_SENDER_ADDRESS_${index}`] = wallet.address;
                envs["gateway-sc"][`COPROCESSOR_SIGNER_ADDRESS_${index}`] = wallet.address;
                envs["gateway-sc"][`COPROCESSOR_S3_BUCKET_URL_${index}`] =
                  "http://minio:9000/ct128";
                envs["host-sc"][`COPROCESSOR_SIGNER_ADDRESS_${index}`] = wallet.address;
                if (index === 0) {
                  envs["coprocessor"].TX_SENDER_PRIVATE_KEY = wallet.privateKey;
                  continue;
                }
                const next = { ...envs["coprocessor"] };
                next.DATABASE_URL = `postgresql://${envs.database.POSTGRES_USER}:${envs.database.POSTGRES_PASSWORD}@db:5432/coprocessor_${index}`;
                next.TX_SENDER_PRIVATE_KEY = wallet.privateKey;
                const instance = state.topology.instances[`coprocessor-${index}`];
                Object.assign(next, instance?.env ?? {});
                resolveEnvMap(next);
                yield* Effect.promise(() => writeEnvFile(envPath(`coprocessor.${index}`), next));
              }
            }

            // Write all component env files
            yield* Effect.promise(async () => {
              for (const component of COMPONENTS) {
                resolveEnvMap(envs[component]);
                await writeEnvFile(envPath(component), envs[component]);
              }
              await writeEnvFile(versionsEnvPath, state.versions.env);

              // Write relayer config
              const relayerConfig = rewriteRelayerConfig(
                YAML.parse(
                  await fs.readFile(TEMPLATE_RELAYER_CONFIG, "utf8"),
                ) as Record<string, unknown>,
                state,
              );
              await fs.writeFile(relayerConfigPath, YAML.stringify(relayerConfig));
            });
          }),

        writeVersionsEnv: (bundle) =>
          Effect.promise(() => writeEnvFile(versionsEnvPath, bundle.env)),
      };
    }),
  );
}
