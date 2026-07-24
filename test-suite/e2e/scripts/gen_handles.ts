/**
 * gen_handles.ts — generate decryptable ciphertext handles for the
 * gateway-stress tool via the real on-chain input flow:
 *
 *   encryptUint64 -> SmokeTestInput.add42ToInput64(handle, proof) -> resUint64()
 *
 * add42ToInput64 grants both FHE.allow(res, msg.sender) and
 * FHE.makePubliclyDecryptable(res), so each handle is valid for user- and
 * public-decryption and is committed + ACL-authorized on-chain by construction.
 * We decrypt each handle (public + user) as a readiness check, then write the
 * handles and contract address into the gateway-stress config.
 *
 * We call the host contract as the gateway-stress `private_key`, so msg.sender
 * (hence the authorized address) matches the `userAddress` gateway-stress signs
 * with at decrypt time. That key must be funded on the host chain (checked
 * below) and holds $ZAMA on the gateway chain (see provisionGatewayZama).
 *
 * Run from test-suite/e2e:
 *   npx hardhat run scripts/gen_handles.ts --network staging
 */
import { readFileSync, writeFileSync } from 'node:fs';
import { resolve } from 'node:path';

import { ethers, network } from 'hardhat';

// cwd is the hardhat project root (test-suite/e2e) when run via `hardhat run`.
const REPO_ROOT = resolve(process.cwd(), '../..');
// Contract addresses + urls the deployed stack exposes for the test suites.
// Bind-mounted to the host; we remap the docker hostnames to localhost below.
const STACK_ENV_PATH = resolve(REPO_ROOT, '.fhevm/runtime/env/test-suite.env');
// $ZAMA token + payment contract addresses.
const GATEWAY_SC_ENV_PATH = resolve(REPO_ROOT, '.fhevm/runtime/env/gateway-sc.env');
const GATEWAY_STRESS_CONFIG_PATH = resolve(REPO_ROOT, 'test-suite/gateway-stress/config/config.toml');

/** Docker-internal hostnames the workers use, mapped to host-reachable ports. */
const HOST_REMAP: ReadonlyArray<[RegExp, string]> = [
  [/fhevm-relayer:3000/g, 'localhost:3000'],
  [/gateway-node:8546/g, 'localhost:8546'],
  [/host-node:8545/g, 'localhost:8545'],
  // The relayer's /keyurl response embeds minio:9000 URLs that the SDK fetches
  // directly; minio is published on localhost:9000.
  [/minio:9000/g, 'localhost:9000'],
];

const remapHost = (url: string): string =>
  HOST_REMAP.reduce((acc, [from, to]) => acc.replace(from, to), url);

/**
 * Patch global fetch to remap docker hostnames to localhost. Needed because the
 * relayer's /keyurl response embeds minio:9000 URLs the SDK fetches directly,
 * which are unreachable from the host.
 */
const installFetchHostRemap = (): void => {
  const original = globalThis.fetch;
  globalThis.fetch = ((input: RequestInfo | URL, init?: RequestInit) => {
    const url = typeof input === 'string' ? input : input instanceof URL ? input.href : input.url;
    const remapped = remapHost(url);
    if (remapped === url) return original(input, init);
    if (typeof input === 'string' || input instanceof URL) return original(remapped, init);
    return original(new Request(remapped, input), init);
  }) as typeof fetch;
};

/** Parse a `KEY=value` env file into a plain object (ignores comments/blanks). */
const parseEnvFile = (path: string): Record<string, string> => {
  const env: Record<string, string> = {};
  for (const raw of readFileSync(path, 'utf8').split('\n')) {
    const line = raw.trim();
    if (!line || line.startsWith('#') || !line.includes('=')) continue;
    const eq = line.indexOf('=');
    env[line.slice(0, eq).trim()] = line.slice(eq + 1).trim();
  }
  return env;
};

/**
 * Copy the addresses/urls `test/instance.ts` reads at import time from
 * test-suite.env into process.env, remapping docker hostnames. Must run BEFORE
 * `test/instance` is imported.
 */
const wireInstanceEnv = (): void => {
  const stackEnv = parseEnvFile(STACK_ENV_PATH);
  const need = (key: string): string => {
    const value = stackEnv[key];
    if (!value) throw new Error(`${key} not found in ${STACK_ENV_PATH}`);
    return value;
  };
  const set = (key: string, value: string): void => {
    process.env[key] = value;
  };

  set('ACL_CONTRACT_ADDRESS', need('ACL_CONTRACT_ADDRESS'));
  set('KMS_VERIFIER_CONTRACT_ADDRESS', need('KMS_VERIFIER_CONTRACT_ADDRESS'));
  set('INPUT_VERIFIER_CONTRACT_ADDRESS', need('INPUT_VERIFIER_CONTRACT_ADDRESS'));
  set('FHEVM_EXECUTOR_CONTRACT_ADDRESS', need('FHEVM_EXECUTOR_CONTRACT_ADDRESS'));
  set('PROTOCOL_CONFIG_CONTRACT_ADDRESS', need('PROTOCOL_CONFIG_CONTRACT_ADDRESS'));
  set('DECRYPTION_ADDRESS', need('DECRYPTION_ADDRESS'));
  set('INPUT_VERIFICATION_ADDRESS', need('INPUT_VERIFICATION_ADDRESS'));
  set('CHAIN_ID_GATEWAY', need('CHAIN_ID_GATEWAY'));
  set('CHAIN_ID_HOST', need('CHAIN_ID_HOST'));
  set('RELAYER_URL', remapHost(need('RELAYER_URL')));
};

/** Read the gateway-stress signer key from its config.toml (the userAddress). */
const readGatewayStressPrivateKey = (configText: string): string => {
  const match = configText.match(/^\s*private_key\s*=\s*"([^"]+)"/m);
  if (!match) throw new Error(`private_key not found in ${GATEWAY_STRESS_CONFIG_PATH}`);
  return match[1];
};

/** Read the gateway RPC url gateway-stress talks to (already host-reachable). */
const readGatewayUrl = (configText: string): string => {
  const match = configText.match(/^\s*gateway_url\s*=\s*"([^"]+)"/m);
  if (!match) throw new Error(`gateway_url not found in ${GATEWAY_STRESS_CONFIG_PATH}`);
  return match[1];
};

// $ZAMA (mock ZamaOFT / ERC20) surface we use; mint() is open on the local stack.
const ZAMA_ABI = [
  'function mint(address to, uint256 amount)',
  'function approve(address spender, uint256 amount) returns (bool)',
  'function balanceOf(address account) view returns (uint256)',
  'function allowance(address owner, address spender) view returns (uint256)',
];

/**
 * Fund the gateway-stress signer with $ZAMA so its on-chain decryption requests
 * can pay the ProtocolPayment fee (1e18 per request, pulled via transferFrom).
 * Without balance + allowance, requests revert with ERC20InsufficientAllowance
 * (selector 0xfb8f41b2). Mints a large buffer and grants an unbounded allowance;
 * idempotent — skips when already provisioned.
 */
async function provisionGatewayZama(configText: string, privateKey: string): Promise<void> {
  const gwEnv = parseEnvFile(GATEWAY_SC_ENV_PATH);
  const zamaAddress = gwEnv.ZAMA_OFT_ADDRESS;
  const paymentAddress = gwEnv.PROTOCOL_PAYMENT_ADDRESS;
  if (!zamaAddress || !paymentAddress) {
    throw new Error(`ZAMA_OFT_ADDRESS / PROTOCOL_PAYMENT_ADDRESS not found in ${GATEWAY_SC_ENV_PATH}`);
  }

  const gatewayUrl = readGatewayUrl(configText);
  const gwProvider = new ethers.JsonRpcProvider(gatewayUrl);
  const gwWallet = new ethers.Wallet(privateKey, gwProvider);
  console.log(`\n[gateway] $ZAMA prep on ${gatewayUrl} for signer=${gwWallet.address}`);

  // Gas on the gateway chain (anvil top-up if needed; no-op elsewhere).
  let gas = await gwProvider.getBalance(gwWallet.address);
  if (gas === 0n) {
    try {
      await gwProvider.send('anvil_setBalance', [gwWallet.address, '0x56bc75e2d63100000']); // 100 ETH
      gas = await gwProvider.getBalance(gwWallet.address);
    } catch {
      // Not an anvil node (or method disabled).
    }
  }
  if (gas === 0n) {
    throw new Error(`gateway-stress signer ${gwWallet.address} has 0 gas on the gateway chain.`);
  }

  const zama = new ethers.Contract(zamaAddress, ZAMA_ABI, gwWallet);
  const topUpThreshold = ethers.parseEther('1000'); // keep well above per-request 1 $ZAMA
  const mintAmount = ethers.parseEther('1000000'); // 1,000,000 $ZAMA — plenty for a long run

  const balance: bigint = await zama.balanceOf(gwWallet.address);
  if (balance < topUpThreshold) {
    console.log(`[gateway] minting ${ethers.formatEther(mintAmount)} $ZAMA (balance was ${ethers.formatEther(balance)}) ...`);
    await (await zama.mint(gwWallet.address, mintAmount)).wait();
  }

  const allowance: bigint = await zama.allowance(gwWallet.address, paymentAddress);
  if (allowance < topUpThreshold) {
    console.log(`[gateway] approving ProtocolPayment ${paymentAddress} (unbounded allowance) ...`);
    await (await zama.approve(paymentAddress, ethers.MaxUint256)).wait();
  }

  const finalBalance: bigint = await zama.balanceOf(gwWallet.address);
  console.log(`[gateway] $ZAMA ready: balance=${ethers.formatEther(finalBalance)} allowance=unbounded`);
}

/** Replace `handle = "..."` inside a `[[section]]` block, preserving formatting. */
const replaceSectionHandle = (configText: string, section: string, handle: string): string => {
  const re = new RegExp(`(\\[\\[${section}\\]\\][\\s\\S]*?handle\\s*=\\s*)"[^"]*"`);
  if (!re.test(configText)) throw new Error(`could not locate handle under [[${section}]]`);
  return configText.replace(re, `$1"${handle}"`);
};

const replaceAllowedContract = (configText: string, address: string): string => {
  const re = /^(allowed_contract\s*=\s*)"[^"]*"/m;
  if (!re.test(configText)) throw new Error('could not locate allowed_contract');
  return configText.replace(re, `$1"${address}"`);
};

async function main(): Promise<void> {
  wireInstanceEnv();
  installFetchHostRemap();

  // Imported dynamically: its module-level env reads must see wireInstanceEnv().
  const { createInstance, aclAddress, coprocessorAddress, kmsVerifierAddress } = await import(
    '../test/instance'
  );

  // Configure the @fhevm/sdk runtime before createInstance() (the shared test
  // wrapper only configures it `if (!hasFhevmRuntimeConfig())`, so we win).
  // `singleThread: true` spawns no worker threads, which silences the verbose
  // `[Worker #x] - ready` logs and lets the process exit cleanly instead of
  // hanging on live worker handles. No-op debug/warn logger; errors still surface.
  const { setFhevmRuntimeConfig, hasFhevmRuntimeConfig } = await import('@fhevm/sdk/ethers');
  if (!hasFhevmRuntimeConfig()) {
    setFhevmRuntimeConfig({
      singleThread: true,
      logger: {
        debug: () => {},
        warn: () => {},
        error: (message: string, cause?: unknown) => {
          console.error(`[fhevm-sdk] ${message}`, cause ?? '');
        },
      },
    });
  }

  const provider = ethers.provider;
  const net = await provider.getNetwork();
  console.log(`network=${network.name} chainId=${net.chainId} rpc=${(network.config as { url?: string }).url}`);

  const configText = readFileSync(GATEWAY_STRESS_CONFIG_PATH, 'utf8');
  const privateKey = readGatewayStressPrivateKey(configText);

  // Call as the gateway-stress signer so msg.sender matches the decrypt-time
  // userAddress (see file header).
  const wallet = new ethers.Wallet(privateKey, provider);
  let balance = await provider.getBalance(wallet.address);
  console.log(`host caller=${wallet.address} balance=${ethers.formatEther(balance)} ETH`);

  // The signer may lack host-chain gas. On a local anvil host, top it up
  // directly (also survives anvil restarts); elsewhere this is a no-op and we
  // fail loudly below.
  if (balance === 0n) {
    try {
      await provider.send('anvil_setBalance', [wallet.address, '0x56bc75e2d63100000']); // 100 ETH
      balance = await provider.getBalance(wallet.address);
      console.log(`funded host caller via anvil_setBalance -> ${ethers.formatEther(balance)} ETH`);
    } catch {
      // Not an anvil node (or method disabled) - leave balance at 0.
    }
  }
  if (balance === 0n) {
    throw new Error(
      `Host caller ${wallet.address} has 0 balance on the host chain.\n` +
        'This account (gateway-stress private_key) must be funded on the host chain\n' +
        'to submit the input transaction. Fund it, or point private_key at a funded key.',
    );
  }

  await provisionGatewayZama(configText, privateKey);

  // Deploy SmokeTestInput (or reuse an existing one via env).
  const factory = await ethers.getContractFactory('SmokeTestInput', wallet);
  let contractAddress = process.env.GEN_HANDLES_CONTRACT_ADDRESS;
  if (contractAddress) {
    console.log(`reusing SmokeTestInput at ${contractAddress}`);
  } else {
    console.log(`deploying SmokeTestInput (acl=${aclAddress} coprocessor=${coprocessorAddress} kms=${kmsVerifierAddress}) ...`);
    const deployed = await factory.deploy(aclAddress, coprocessorAddress, kmsVerifierAddress);
    await deployed.waitForDeployment();
    contractAddress = await deployed.getAddress();
    console.log(`deployed SmokeTestInput at ${contractAddress}`);
  }
  const contract = factory.attach(contractAddress) as any;

  const instance = await createInstance();

  // One handle per gateway-stress ciphertext slot, each verified in its mode.
  const generateHandle = async (label: string, value: bigint): Promise<string> => {
    console.log(`\n[${label}] encrypting input value=${value} ...`);
    const enc = await instance.encryptUint64({ contractAddress, userAddress: wallet.address, value });

    console.log(`[${label}] submitting add42ToInput64 ...`);
    const tx = await contract.add42ToInput64(enc.handles[0], enc.inputProof);
    const receipt = await tx.wait();
    if (receipt?.status !== 1) throw new Error(`[${label}] add42ToInput64 reverted (hash=${tx.hash})`);

    const handle: string = await contract.resUint64();
    console.log(`[${label}] result handle=${handle}`);
    return handle;
  };

  const expectedPublic = 7n + 42n;
  const expectedUser = 11n + 42n;
  const publicHandle = await generateHandle('public_ct', 7n);
  const userHandle = await generateHandle('user_ct', 11n);

  // Real readiness check: decrypt before writing config.
  console.log('\nverifying public decryption ...');
  const pub = await instance.publicDecrypt([publicHandle]);
  if (pub.clearValues[publicHandle] !== expectedPublic) {
    throw new Error(`public decrypt mismatch: got ${pub.clearValues[publicHandle]}, expected ${expectedPublic}`);
  }
  console.log(`public decrypt OK (${publicHandle} = ${expectedPublic})`);

  console.log('verifying user decryption ...');
  const userClear = await instance.userDecryptSingleHandle({
    handle: userHandle,
    contractAddress,
    signer: wallet,
  });
  if (userClear !== expectedUser) {
    throw new Error(`user decrypt mismatch: got ${userClear}, expected ${expectedUser}`);
  }
  console.log(`user decrypt OK (${userHandle} = ${expectedUser})`);

  // Wire the verified handles into the gateway-stress config.
  let updated = configText;
  updated = replaceSectionHandle(updated, 'public_ct', publicHandle);
  updated = replaceSectionHandle(updated, 'user_ct', userHandle);
  updated = replaceAllowedContract(updated, contractAddress);
  writeFileSync(GATEWAY_STRESS_CONFIG_PATH, updated);

  console.log(`\nUpdated ${GATEWAY_STRESS_CONFIG_PATH}:`);
  console.log(`  public_ct.handle   = ${publicHandle}`);
  console.log(`  user_ct.handle     = ${userHandle}`);
  console.log(`  allowed_contract   = ${contractAddress}`);
}

main()
  .then(() => {
    // SDK/provider handles may keep the event loop alive; work is done, so exit.
    process.exit(0);
  })
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
