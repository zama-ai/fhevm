const { copyFileSync, existsSync, mkdirSync, readFileSync } = require("node:fs");
const { join, resolve } = require("node:path");
const { execFileSync, spawn } = require("node:child_process");
const net = require("node:net");

const ROOT = resolve(__dirname, "..");
const TEST_SUITE_STAGING_ENV_DIR = resolve(ROOT, "../test-suite/fhevm/env/staging");
const HOST_STACK_ENV = join(TEST_SUITE_STAGING_ENV_DIR, ".env.host-sc.local");
const GATEWAY_STACK_ENV = join(TEST_SUITE_STAGING_ENV_DIR, ".env.gateway-sc.local");
const HOST_PROGRAM_ID = "5TeWSsjg2gbxCyWVniXeCmwM7UtHTCK7svzJr5xYJzHf";
const TEST_INPUT_PROGRAM_ID = "5MaDNrtMTmYccr1ASgE1i2LZgbnyBPeDR7tN8Q8ewXTv";
const CONFIDENTIAL_TOKEN_PROGRAM_ID = "Cjb3AVoxxKmG4TGWX5gzSjCNwtxN6gneVsWB7f9i8Csx";
const DOCKER_HOST_NODE_CONTAINER =
  process.env.SOLANA_DOCKER_HOST_NODE_CONTAINER || "host-node";
const DEFAULT_RPC_URL = "http://127.0.0.1:18999";
const DEFAULT_WS_URL = "ws://127.0.0.1:19000";
const ENV_OVERRIDE_KEYS = new Set([
  "SOLANA_HOST_CHAIN_ID",
  "CHAIN_ID_GATEWAY",
  "INPUT_VERIFICATION_ADDRESS",
  "DECRYPTION_ADDRESS",
  "NUM_KMS_NODES",
  "PUBLIC_DECRYPTION_THRESHOLD",
  "NUM_COPROCESSORS",
  "COPROCESSOR_THRESHOLD",
  "HCU_CAP_PER_BLOCK",
  "MAX_HCU_DEPTH_PER_TX",
  "MAX_HCU_PER_TX",
]);

function parseEnvFile(path) {
  const parsed = {};
  const text = readFileSync(path, "utf8");
  for (const rawLine of text.split(/\r?\n/)) {
    const line = rawLine.trim();
    if (!line || line.startsWith("#")) {
      continue;
    }
    const idx = line.indexOf("=");
    if (idx === -1) {
      continue;
    }
    const key = line.slice(0, idx).trim();
    let value = line.slice(idx + 1).trim();
    if (
      (value.startsWith('"') && value.endsWith('"')) ||
      (value.startsWith("'") && value.endsWith("'"))
    ) {
      value = value.slice(1, -1);
    }
    parsed[key] = value;
  }
  return parsed;
}

function mergeEnvOverrides(envConfig) {
  const merged = { ...envConfig };
  for (const [key, value] of Object.entries(process.env)) {
    if (!value) {
      continue;
    }
    if (
      ENV_OVERRIDE_KEYS.has(key) ||
      key.startsWith("PRIVATE_KEY_KMS_SIGNER_") ||
      key.startsWith("PRIVATE_KEY_COPROCESSOR_ACCOUNT_") ||
      key.startsWith("KMS_SIGNER_ADDRESS_") ||
      key.startsWith("COPROCESSOR_SIGNER_ADDRESS_")
    ) {
      merged[key] = value;
    }
  }
  return merged;
}

function loadLocalStackEnvOverrides() {
  const merged = {};
  for (const path of [GATEWAY_STACK_ENV, HOST_STACK_ENV]) {
    if (!existsSync(path)) {
      continue;
    }
    Object.assign(merged, parseEnvFile(path));
  }
  return merged;
}

function preferConfiguredSignerAddresses(mergedEnv, overrides, privateKeyPrefix, addressPrefix) {
  for (const key of Object.keys(overrides)) {
    if (!key.startsWith(addressPrefix)) {
      continue;
    }
    const suffix = key.slice(addressPrefix.length);
    const privateKeyKey = `${privateKeyPrefix}${suffix}`;
    if (!(privateKeyKey in overrides)) {
      delete mergedEnv[privateKeyKey];
    }
  }
}

function deriveWsUrl(rpcUrl) {
  try {
    const url = new URL(rpcUrl);
    url.protocol = url.protocol === "https:" ? "wss:" : "ws:";
    if (url.port) {
      url.port = String(Number(url.port) + 1);
    }
    return url.toString().replace(/\/$/, "");
  } catch {
    return DEFAULT_WS_URL;
  }
}

function run(cmd, args, options = {}) {
  return execFileSync(cmd, args, {
    cwd: ROOT,
    stdio: options.stdio ?? "pipe",
    encoding: options.encoding ?? "utf8",
    env: options.env ?? process.env,
  });
}

async function runWithRetry(fn, label, attempts = 30, delayMs = 1_000) {
  let lastError = null;
  for (let attempt = 1; attempt <= attempts; attempt += 1) {
    try {
      return fn();
    } catch (error) {
      lastError = error;
      if (attempt === attempts) {
        break;
      }
      await sleep(delayMs);
    }
  }
  throw new Error(`${label} failed after ${attempts} attempts: ${lastError}`);
}

function initLocalArgs({
  anchorWallet,
  providerUrl,
  providerWsUrl,
  outputRpcUrl,
  outputWsUrl,
  addressesEnv,
  addressesJson,
}) {
  return [
    "run",
    "--quiet",
    "--manifest-path",
    join(ROOT, "local-cli/Cargo.toml"),
    "--",
    "init-local",
    "--rpc-url",
    providerUrl,
    "--ws-url",
    providerWsUrl,
    "--output-rpc-url",
    outputRpcUrl,
    "--output-ws-url",
    outputWsUrl,
    "--payer-keypair",
    anchorWallet,
    "--program-id",
    HOST_PROGRAM_ID,
    "--test-input-program-id",
    TEST_INPUT_PROGRAM_ID,
    "--confidential-token-program-id",
    CONFIDENTIAL_TOKEN_PROGRAM_ID,
    "--addresses-env",
    addressesEnv,
    "--addresses-json",
    addressesJson,
  ];
}

function airdropViaHost(amount, recipient, providerUrl) {
  run("solana", ["airdrop", amount, recipient, "--url", providerUrl], {
    stdio: "inherit",
  });
}

function airdropViaDocker(amount, recipient, providerUrl) {
  run(
    "docker",
    [
      "exec",
      DOCKER_HOST_NODE_CONTAINER,
      "solana",
      "airdrop",
      amount,
      recipient,
      "--url",
      providerUrl,
    ],
    {
      stdio: "inherit",
    },
  );
}

function ensureDir(path) {
  mkdirSync(path, { recursive: true });
}

function localnetConfig() {
  const envFile = join(ROOT, ".env.example");
  const stackOverrides = loadLocalStackEnvOverrides();
  const envConfig = mergeEnvOverrides({
    ...parseEnvFile(envFile),
    ...stackOverrides,
  });
  preferConfiguredSignerAddresses(
    envConfig,
    stackOverrides,
    "PRIVATE_KEY_COPROCESSOR_ACCOUNT_",
    "COPROCESSOR_SIGNER_ADDRESS_",
  );
  preferConfiguredSignerAddresses(
    envConfig,
    stackOverrides,
    "PRIVATE_KEY_KMS_SIGNER_",
    "KMS_SIGNER_ADDRESS_",
  );
  const anchorWallet =
    process.env.ANCHOR_WALLET || resolve(ROOT, "tests/fixtures/anchor-authority.json");
  const bobWallet = resolve(ROOT, "tests/fixtures/confidential-token-recipient.json");
  const providerUrl = process.env.ANCHOR_PROVIDER_URL || DEFAULT_RPC_URL;
  const providerWsUrl = deriveWsUrl(providerUrl);
  const outputRpcUrl = envConfig.SOLANA_HOST_OUTPUT_RPC_URL || providerUrl;
  const outputWsUrl = envConfig.SOLANA_HOST_OUTPUT_WS_URL || deriveWsUrl(outputRpcUrl);
  const airdropSol = envConfig.SOLANA_HOST_AIRDROP_SOL || "20";
  const addressesDir = join(ROOT, "addresses");
  const addressesEnv = join(addressesDir, ".env.host");
  const addressesEnvLocal = join(addressesDir, ".env.local");
  const addressesJson = join(addressesDir, "localnet.json");

  return {
    envConfig,
    anchorWallet,
    bobWallet,
    providerUrl,
    providerWsUrl,
    outputRpcUrl,
    outputWsUrl,
    airdropSol,
    addressesDir,
    addressesEnv,
    addressesEnvLocal,
    addressesJson,
  };
}

async function bootstrapCommon(airdrop) {
  const {
    envConfig,
    anchorWallet,
    bobWallet,
    providerUrl,
    providerWsUrl,
    outputRpcUrl,
    outputWsUrl,
    airdropSol,
    addressesDir,
    addressesEnv,
    addressesEnvLocal,
    addressesJson,
  } = localnetConfig();
  const commandEnv = { ...process.env, ...envConfig };
  preferConfiguredSignerAddresses(
    commandEnv,
    envConfig,
    "PRIVATE_KEY_COPROCESSOR_ACCOUNT_",
    "COPROCESSOR_SIGNER_ADDRESS_",
  );
  preferConfiguredSignerAddresses(
    commandEnv,
    envConfig,
    "PRIVATE_KEY_KMS_SIGNER_",
    "KMS_SIGNER_ADDRESS_",
  );

  ensureDir(addressesDir);

  const authorityPubkey = run("solana-keygen", ["pubkey", anchorWallet]).trim();
  await runWithRetry(
    () => airdrop(airdropSol, authorityPubkey, providerUrl),
    `airdrop to authority ${authorityPubkey}`,
  );
  const bobPubkey = run("solana-keygen", ["pubkey", bobWallet]).trim();
  await runWithRetry(
    () => airdrop("5", bobPubkey, providerUrl),
    `airdrop to bob ${bobPubkey}`,
  );

  run(
    "cargo",
    initLocalArgs({
      anchorWallet,
      providerUrl,
      providerWsUrl,
      outputRpcUrl,
      outputWsUrl,
      addressesEnv,
      addressesJson,
    }),
    {
      stdio: "inherit",
      env: commandEnv,
    },
  );

  copyFileSync(addressesEnv, addressesEnvLocal);

  if (!existsSync(addressesEnv) || !existsSync(addressesJson)) {
    throw new Error("anchor localnet setup did not write address artifacts");
  }

  console.log("Anchor localnet setup complete");
  console.log(`host_program_id=${HOST_PROGRAM_ID}`);
  console.log(`test_input_program_id=${TEST_INPUT_PROGRAM_ID}`);
  console.log(`confidential_token_program_id=${CONFIDENTIAL_TOKEN_PROGRAM_ID}`);
  console.log(`rpc_url=${outputRpcUrl}`);
  console.log(`ws_url=${outputWsUrl}`);
  console.log(`addresses_env=${addressesEnv}`);
  console.log(`addresses_json=${addressesJson}`);
}

async function bootstrapLocalnet() {
  await bootstrapCommon(airdropViaHost);
}

async function bootstrapDockerHostNode() {
  await bootstrapCommon(airdropViaDocker);
}

async function rpcRequest(rpcUrl, method, params = []) {
  const response = await fetch(rpcUrl, {
    method: "POST",
    headers: { "content-type": "application/json" },
    body: JSON.stringify({
      jsonrpc: "2.0",
      id: 1,
      method,
      params,
    }),
  });

  if (!response.ok) {
    throw new Error(`rpc request failed with status ${response.status}`);
  }

  const payload = await response.json();
  if (payload.error) {
    throw new Error(payload.error.message || `rpc request failed for ${method}`);
  }
  return payload.result;
}

function portFromUrl(urlString) {
  try {
    const url = new URL(urlString);
    const port =
      url.port === "" ? (url.protocol === "https:" ? 443 : 80) : Number(url.port);
    return {
      host: url.hostname,
      port,
    };
  } catch {
    return null;
  }
}

function isPortInUse(host, port, timeoutMs = 1_000) {
  return new Promise((resolve) => {
    const socket = net.createConnection({ host, port });
    let settled = false;

    const finish = (value) => {
      if (settled) {
        return;
      }
      settled = true;
      socket.destroy();
      resolve(value);
    };

    socket.setTimeout(timeoutMs);
    socket.once("connect", () => finish(true));
    socket.once("timeout", () => finish(false));
    socket.once("error", () => finish(false));
  });
}

async function describeExistingLocalnet(rpcUrl) {
  try {
    await rpcRequest(rpcUrl, "getVersion");
    return [
      `a validator is already responding on ${rpcUrl}`,
      "reuse it with `make localnet-test-existing` or refresh address artifacts with `make localnet-bootstrap`",
      "if you intended to start a fresh validator, stop the existing one first",
    ].join("\n");
  } catch {
    const target = portFromUrl(rpcUrl);
    if (target && (await isPortInUse(target.host, target.port))) {
      return [
        `the configured RPC port for anchor localnet is already in use: ${target.port}`,
        "free that port or point Anchor at a different local RPC port",
        "if this is an existing validator for this workspace, use `make localnet-test-existing` or `make localnet-bootstrap` instead",
      ].join("\n");
    }
    return null;
  }
}

function sleep(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

async function waitForProgramDeployment(rpcUrl, programId, timeoutMs) {
  const deadline = Date.now() + timeoutMs;

  while (Date.now() < deadline) {
    try {
      const accountInfo = await rpcRequest(rpcUrl, "getAccountInfo", [
        programId,
        { encoding: "base64" },
      ]);
      if (accountInfo?.value?.executable) {
        return;
      }
    } catch {
      // Keep polling until the validator is ready and the program is deployed.
    }
    await sleep(1_000);
  }

  throw new Error(`timed out waiting for ${programId} to be deployed on ${rpcUrl}`);
}

async function runPersistentLocalnet() {
  const { providerUrl } = localnetConfig();
  const existingLocalnetMessage = await describeExistingLocalnet(providerUrl);

  if (existingLocalnetMessage) {
    throw new Error(existingLocalnetMessage);
  }

  const child = spawn("anchor", ["localnet"], {
    cwd: ROOT,
    env: process.env,
    stdio: ["pipe", "inherit", "inherit"],
  });

  let shuttingDown = false;
  const forwardSignal = (signal) => {
    if (shuttingDown) {
      return;
    }
    shuttingDown = true;
    if (child.stdin && !child.stdin.destroyed) {
      child.stdin.write("\n");
      child.stdin.end();
      return;
    }
    if (!child.killed) {
      child.kill(signal);
    }
  };

  process.on("SIGINT", () => forwardSignal("SIGINT"));
  process.on("SIGTERM", () => forwardSignal("SIGTERM"));

  const childExit = new Promise((resolve, reject) => {
    child.once("exit", (code, signal) => {
      if (shuttingDown && (signal === "SIGINT" || code === 0 || code === 130)) {
        resolve();
        return;
      }
      if (code === 0) {
        resolve();
        return;
      }
      reject(
        new Error(
          `anchor localnet exited before bootstrap completed (code=${code}, signal=${signal})`,
        ),
      );
    });
    child.once("error", reject);
  });

  try {
    await Promise.race([
      (async () => {
        await waitForProgramDeployment(providerUrl, HOST_PROGRAM_ID, 180_000);
        await waitForProgramDeployment(providerUrl, TEST_INPUT_PROGRAM_ID, 180_000);
        await waitForProgramDeployment(providerUrl, CONFIDENTIAL_TOKEN_PROGRAM_ID, 180_000);
      })(),
      childExit,
    ]);
    await bootstrapLocalnet();
    console.log("Anchor localnet is ready and will keep running until you stop it.");
    await childExit;
  } catch (error) {
    forwardSignal("SIGINT");
    throw error;
  }
}

async function main() {
  const mode = process.argv[2] || "bootstrap";

  if (mode === "live") {
    await runPersistentLocalnet();
    return;
  }

  if (mode === "bootstrap") {
    await bootstrapLocalnet();
    return;
  }

  if (mode === "bootstrap-docker") {
    await bootstrapDockerHostNode();
    return;
  }

  throw new Error(`unknown mode: ${mode}`);
}

main().catch((error) => {
  console.error(error instanceof Error ? error.message : String(error));
  process.exit(1);
});
