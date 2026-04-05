import path from "node:path";
import fs from "node:fs/promises";

import { PreflightError } from "../errors";
import {
  DEFAULT_GATEWAY_RPC_PORT,
  DEFAULT_SOLANA_HOST_FAUCET_PORT,
  DEFAULT_SOLANA_HOST_WS_PORT,
  REPO_ROOT,
  SOLANA_CONFIDENTIAL_TOKEN_PROGRAM_ID,
  SOLANA_HOST_PROGRAM_ID,
  SOLANA_TEST_INPUT_PROGRAM_ID,
  envPath,
  hostChainAddressesPath,
  hostChainKind,
  type HostChainRuntime,
} from "../layout";
import { ensureDir, readEnvFile, readEnvFileIfExists, readJson, withHexPrefix, writeEnvFile } from "../utils/fs";
import { run } from "../utils/process";

const SOLANA_ROOT = path.join(REPO_ROOT, "solana-host-contracts");
const SOLANA_EXAMPLE_ENV = path.join(SOLANA_ROOT, ".env.example");
const SOLANA_LOCAL_CLI_MANIFEST = path.join(SOLANA_ROOT, "local-cli", "Cargo.toml");
const SOLANA_AUTHORITY_KEYPAIR = path.join(SOLANA_ROOT, "tests", "fixtures", "anchor-authority.json");
const SOLANA_TOKEN_RECIPIENT_KEYPAIR = path.join(SOLANA_ROOT, "tests", "fixtures", "confidential-token-recipient.json");
const SOLANA_HOST_PROGRAM_SO = path.join(SOLANA_ROOT, "target", "deploy", "solana_host_contracts.so");
const SOLANA_TEST_INPUT_PROGRAM_SO = path.join(SOLANA_ROOT, "target", "deploy", "solana_test_input_program.so");
const SOLANA_CONFIDENTIAL_TOKEN_PROGRAM_SO = path.join(
  SOLANA_ROOT,
  "target",
  "deploy",
  "solana_confidential_token_program.so",
);
const SOLANA_SOURCE_ROOTS = [
  path.join(SOLANA_ROOT, "core"),
  path.join(SOLANA_ROOT, "program"),
  path.join(SOLANA_ROOT, "test-input-program"),
  path.join(SOLANA_ROOT, "confidential-token-program"),
];

const parseEnvU64 = (value: string | undefined, label: string) => {
  if (!value || !/^\d+$/.test(value)) {
    throw new PreflightError(`Missing numeric ${label} in generated env`);
  }
  return value;
};

const solanaRpcUrl = (chain: Pick<HostChainRuntime, "rpcPort">) => `http://127.0.0.1:${chain.rpcPort}`;
const solanaWsUrl = () => `ws://127.0.0.1:${DEFAULT_SOLANA_HOST_WS_PORT}`;

const newestFileMtime = async (target: string): Promise<number> => {
  const stat = await fs.stat(target);
  if (!stat.isDirectory()) {
    return stat.mtimeMs;
  }
  let newest = stat.mtimeMs;
  for (const entry of await fs.readdir(target, { withFileTypes: true })) {
    newest = Math.max(newest, await newestFileMtime(path.join(target, entry.name)));
  }
  return newest;
};

const fileMtimeOrZero = async (target: string) => {
  try {
    return (await fs.stat(target)).mtimeMs;
  } catch {
    return 0;
  }
};

const solanaProgramsNeedBuild = async () => {
  const artifactTimes = await Promise.all([
    fileMtimeOrZero(SOLANA_HOST_PROGRAM_SO),
    fileMtimeOrZero(SOLANA_TEST_INPUT_PROGRAM_SO),
    fileMtimeOrZero(SOLANA_CONFIDENTIAL_TOKEN_PROGRAM_SO),
  ]);
  if (artifactTimes.some((mtime) => mtime === 0)) {
    return true;
  }
  const newestSource = Math.max(...(await Promise.all(SOLANA_SOURCE_ROOTS.map((dir) => newestFileMtime(dir)))));
  return artifactTimes.some((mtime) => newestSource > mtime);
};

const ensureSolanaProgramsBuilt = async () => {
  if (!(await solanaProgramsNeedBuild())) {
    return;
  }
  await run(["make", "-C", SOLANA_ROOT, "build-sbf"], { cwd: REPO_ROOT });
};

/** Detects whether a host chain uses the Solana runtime. */
export const isSolanaChain = (chain: Pick<HostChainRuntime, "chainKind">) => hostChainKind(chain) === "solana";

/** Waits until the Solana RPC answers a basic getVersion request. */
export const waitForSolanaRpc = async (url: string) => {
  for (let attempt = 0; attempt <= 60; attempt += 1) {
    try {
      const response = await fetch(url, {
        method: "POST",
        headers: { "content-type": "application/json" },
        body: JSON.stringify({ jsonrpc: "2.0", id: 1, method: "getVersion", params: [] }),
      });
      if (response.ok) {
        const payload = (await response.json()) as { result?: unknown; error?: unknown };
        if (payload.result && !payload.error) {
          return;
        }
      }
    } catch {
      // retry
    }
    if (attempt === 60) {
      throw new PreflightError(`Solana RPC ${url} was not ready after 60s`);
    }
    await Bun.sleep(1_000);
  }
};

const bootstrapEnv = async () => {
  const example = await readEnvFile(SOLANA_EXAMPLE_ENV);
  const gatewayEnv = await readEnvFile(envPath("gateway-sc"));
  const hostEnv = await readEnvFileIfExists(envPath("host-sc"));
  const merged = { ...example, ...hostEnv, ...gatewayEnv };
  for (const [key] of Object.entries(merged)) {
    if (key.startsWith("COPROCESSOR_SIGNER_ADDRESS_")) {
      delete merged[`PRIVATE_KEY_COPROCESSOR_ACCOUNT_${key.slice("COPROCESSOR_SIGNER_ADDRESS_".length)}`];
    }
    if (key.startsWith("KMS_SIGNER_ADDRESS_")) {
      delete merged[`PRIVATE_KEY_KMS_SIGNER_${key.slice("KMS_SIGNER_ADDRESS_".length)}`];
    }
  }
  return merged;
};

const keypairPubkey = async (file: string) => (await run(["solana-keygen", "pubkey", file])).stdout.trim();

const dockerAirdrop = async (container: string, rpcUrl: string, amount: string, recipient: string) => {
  await run(["docker", "exec", container, "solana", "airdrop", amount, recipient, "--url", rpcUrl]);
};

const initLocalArgs = (
  rpcUrl: string,
  wsUrl: string,
  addressesEnvFile: string,
  addressesJsonFile: string,
) => [
  "run",
  "--quiet",
  "--manifest-path",
  SOLANA_LOCAL_CLI_MANIFEST,
  "--",
  "init-local",
  "--rpc-url",
  rpcUrl,
  "--ws-url",
  wsUrl,
  "--output-rpc-url",
  rpcUrl,
  "--output-ws-url",
  wsUrl,
  "--payer-keypair",
  SOLANA_AUTHORITY_KEYPAIR,
  "--program-id",
  SOLANA_HOST_PROGRAM_ID,
  "--test-input-program-id",
  SOLANA_TEST_INPUT_PROGRAM_ID,
  "--confidential-token-program-id",
  SOLANA_CONFIDENTIAL_TOKEN_PROGRAM_ID,
  "--addresses-env",
  addressesEnvFile,
  "--addresses-json",
  addressesJsonFile,
];

type SolanaLocalnetJson = {
  rpc_url?: string;
  ws_url?: string;
  host_kind?: string;
  program_id?: string;
  state_pda?: string;
  session_pda?: string;
  acl_program_id?: string;
  test_input_program_id?: string;
  test_input_state_pda?: string;
  confidential_token_program_id?: string;
  confidential_token_state_pda?: string;
  authority?: string;
  token_recipient?: string;
  host_chain_id?: number;
  gateway_chain_id?: number;
  input_verification_address?: string;
  decryption_address?: string;
  coprocessor_threshold?: number;
  public_decryption_threshold?: number;
  coprocessor_signers?: string[];
  kms_signers?: string[];
};

const augmentSolanaAddresses = async (
  file: string,
  jsonFile: string,
  bootstrapValues: Record<string, string>,
) => {
  const env = await readEnvFileIfExists(file);
  const json = (await readJson<SolanaLocalnetJson>(jsonFile).catch(() => ({}))) ?? {};
  const acl = env.SOLANA_HOST_ACL_PROGRAM_ID ?? env.SOLANA_HOST_PROGRAM_ID ?? SOLANA_HOST_PROGRAM_ID;
  const next = {
    SOLANA_HOST_RPC_URL: env.SOLANA_HOST_RPC_URL ?? json.rpc_url ?? bootstrapValues.SOLANA_HOST_OUTPUT_RPC_URL ?? "",
    SOLANA_HOST_WS_URL: env.SOLANA_HOST_WS_URL ?? json.ws_url ?? bootstrapValues.SOLANA_HOST_OUTPUT_WS_URL ?? "",
    SOLANA_HOST_KIND: env.SOLANA_HOST_KIND ?? json.host_kind ?? "solana",
    SOLANA_HOST_PROGRAM_ID: env.SOLANA_HOST_PROGRAM_ID ?? json.program_id ?? SOLANA_HOST_PROGRAM_ID,
    SOLANA_HOST_STATE_PDA: env.SOLANA_HOST_STATE_PDA ?? json.state_pda ?? "",
    SOLANA_HOST_SESSION_PDA: env.SOLANA_HOST_SESSION_PDA ?? json.session_pda ?? "",
    SOLANA_HOST_ACL_PROGRAM_ID: env.SOLANA_HOST_ACL_PROGRAM_ID ?? json.acl_program_id ?? SOLANA_HOST_PROGRAM_ID,
    SOLANA_TEST_INPUT_PROGRAM_ID: env.SOLANA_TEST_INPUT_PROGRAM_ID ?? json.test_input_program_id ?? SOLANA_TEST_INPUT_PROGRAM_ID,
    SOLANA_TEST_INPUT_STATE_PDA: env.SOLANA_TEST_INPUT_STATE_PDA ?? json.test_input_state_pda ?? "",
    SOLANA_CONFIDENTIAL_TOKEN_PROGRAM_ID:
      env.SOLANA_CONFIDENTIAL_TOKEN_PROGRAM_ID ?? json.confidential_token_program_id ?? SOLANA_CONFIDENTIAL_TOKEN_PROGRAM_ID,
    SOLANA_CONFIDENTIAL_TOKEN_STATE_PDA: env.SOLANA_CONFIDENTIAL_TOKEN_STATE_PDA ?? json.confidential_token_state_pda ?? "",
    SOLANA_HOST_AUTHORITY: env.SOLANA_HOST_AUTHORITY ?? json.authority ?? "",
    SOLANA_TOKEN_RECIPIENT: env.SOLANA_TOKEN_RECIPIENT ?? json.token_recipient ?? "",
    SOLANA_HOST_CHAIN_ID:
      env.SOLANA_HOST_CHAIN_ID ?? String(json.host_chain_id ?? bootstrapValues.SOLANA_HOST_CHAIN_ID ?? ""),
    CHAIN_ID_GATEWAY:
      env.CHAIN_ID_GATEWAY ?? String(json.gateway_chain_id ?? bootstrapValues.CHAIN_ID_GATEWAY ?? ""),
    INPUT_VERIFICATION_ADDRESS:
      env.INPUT_VERIFICATION_ADDRESS ?? json.input_verification_address ?? bootstrapValues.INPUT_VERIFICATION_ADDRESS ?? "",
    DECRYPTION_ADDRESS:
      env.DECRYPTION_ADDRESS ?? json.decryption_address ?? bootstrapValues.DECRYPTION_ADDRESS ?? "",
    NUM_COPROCESSORS: env.NUM_COPROCESSORS ?? bootstrapValues.NUM_COPROCESSORS ?? "",
    COPROCESSOR_THRESHOLD:
      env.COPROCESSOR_THRESHOLD ??
      String(json.coprocessor_threshold ?? bootstrapValues.COPROCESSOR_THRESHOLD ?? ""),
    NUM_KMS_NODES: env.NUM_KMS_NODES ?? bootstrapValues.NUM_KMS_NODES ?? "",
    PUBLIC_DECRYPTION_THRESHOLD:
      env.PUBLIC_DECRYPTION_THRESHOLD ??
      String(json.public_decryption_threshold ?? bootstrapValues.PUBLIC_DECRYPTION_THRESHOLD ?? ""),
    ...env,
    HOST_CHAIN_KIND: "solana",
    ACL_CONTRACT_ADDRESS: env.ACL_CONTRACT_ADDRESS ?? acl,
    FHEVM_EXECUTOR_CONTRACT_ADDRESS: env.FHEVM_EXECUTOR_CONTRACT_ADDRESS ?? (env.SOLANA_HOST_PROGRAM_ID ?? SOLANA_HOST_PROGRAM_ID),
    KMS_VERIFIER_CONTRACT_ADDRESS: env.KMS_VERIFIER_CONTRACT_ADDRESS ?? (env.SOLANA_HOST_PROGRAM_ID ?? SOLANA_HOST_PROGRAM_ID),
    INPUT_VERIFIER_CONTRACT_ADDRESS:
      env.INPUT_VERIFIER_CONTRACT_ADDRESS ?? env.SOLANA_TEST_INPUT_PROGRAM_ID ?? env.INPUT_VERIFICATION_ADDRESS ?? "",
    PAUSER_SET_CONTRACT_ADDRESS: env.PAUSER_SET_CONTRACT_ADDRESS ?? (env.SOLANA_HOST_PROGRAM_ID ?? SOLANA_HOST_PROGRAM_ID),
  };
  for (const [index, signer] of (json.coprocessor_signers ?? []).entries()) {
    next[`COPROCESSOR_SIGNER_ADDRESS_${index}`] = env[`COPROCESSOR_SIGNER_ADDRESS_${index}`] ?? signer;
  }
  for (const [index, signer] of (json.kms_signers ?? []).entries()) {
    next[`KMS_SIGNER_ADDRESS_${index}`] = env[`KMS_SIGNER_ADDRESS_${index}`] ?? signer;
  }
  await writeEnvFile(file, next);
  return next;
};

/** Bootstraps Solana address/state artifacts against a Dockerized validator. */
export const bootstrapSolanaHost = async (chain: HostChainRuntime) => {
  await ensureSolanaProgramsBuilt();
  const runtimeEnv = await bootstrapEnv();
  const rpcUrl = solanaRpcUrl(chain);
  const wsUrl = solanaWsUrl();
  const chainDir = path.dirname(hostChainAddressesPath(chain.key));
  const addressesEnvFile = hostChainAddressesPath(chain.key);
  const addressesJsonFile = path.join(chainDir, "localnet.json");
  await ensureDir(chainDir);

  const mergedEnv = {
    ...process.env,
    ...runtimeEnv,
    SOLANA_HOST_CHAIN_ID: chain.chainId,
    CHAIN_ID_GATEWAY: parseEnvU64(runtimeEnv.CHAIN_ID_GATEWAY, "CHAIN_ID_GATEWAY"),
    INPUT_VERIFICATION_ADDRESS: withHexPrefix(runtimeEnv.INPUT_VERIFICATION_ADDRESS),
    DECRYPTION_ADDRESS: withHexPrefix(runtimeEnv.DECRYPTION_ADDRESS),
    SOLANA_HOST_OUTPUT_RPC_URL: rpcUrl,
    SOLANA_HOST_OUTPUT_WS_URL: wsUrl,
    NUM_COPROCESSORS: runtimeEnv.NUM_COPROCESSORS ?? "1",
    COPROCESSOR_THRESHOLD: runtimeEnv.COPROCESSOR_THRESHOLD ?? "1",
    NUM_KMS_NODES: runtimeEnv.NUM_KMS_NODES ?? "1",
    PUBLIC_DECRYPTION_THRESHOLD: runtimeEnv.PUBLIC_DECRYPTION_THRESHOLD ?? "1",
  };

  const authorityPubkey = await keypairPubkey(SOLANA_AUTHORITY_KEYPAIR);
  const recipientPubkey = await keypairPubkey(SOLANA_TOKEN_RECIPIENT_KEYPAIR);
  await dockerAirdrop(chain.node, rpcUrl, runtimeEnv.SOLANA_HOST_AIRDROP_SOL ?? "20", authorityPubkey);
  await dockerAirdrop(chain.node, rpcUrl, "5", recipientPubkey);

  await run(["cargo", ...initLocalArgs(rpcUrl, wsUrl, addressesEnvFile, addressesJsonFile)], {
    cwd: REPO_ROOT,
    env: mergedEnv,
  });
  return augmentSolanaAddresses(addressesEnvFile, addressesJsonFile, mergedEnv);
};
