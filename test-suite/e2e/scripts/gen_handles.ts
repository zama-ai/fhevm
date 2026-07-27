/**
 * gen_handles.ts — generate decryptable ciphertext handles for the given private key via via the
 * real on-chain input flow:
 *
 *   encryptUint64 -> SmokeTestInput.add42ToInput64(handle, proof) -> resUint64()
 *
 * add42ToInput64 grants both FHE.allow(res, msg.sender) and FHE.makePubliclyDecryptable(res), so
 * each handle is valid for user- and public-decryption and is committed + ACL-authorized on-chain
 * by construction.
 * We decrypt each handle (public + user) as a readiness check, then PRINT the handles and
 * contract address for you to paste into the gateway-stress config.
 *
 * Designed to run INSIDE the test-suite e2e container, which is attached to the stack's docker
 * network and loads the stack env files.
 *
 *   docker exec fhevm-test-suite-e2e-debug bash -c \
 *     'PRIVATE_KEY=<hex> npx hardhat run scripts/gen_handles.ts --network staging'
 */
import { ethers, network } from 'hardhat';

/** Read a required env var (loaded from the stack env files by the container). */
const requireEnv = (key: string): string => {
  const value = process.env[key];
  if (!value) throw new Error(`${key} is not set in the environment`);
  return value;
};

// $ZAMA (mock ZamaOFT / ERC20) surface we use; mint() is open on the local stack.
const ZAMA_ABI = [
  'function mint(address to, uint256 amount)',
  'function approve(address spender, uint256 amount) returns (bool)',
  'function balanceOf(address account) view returns (uint256)',
  'function allowance(address owner, address spender) view returns (uint256)',
];

/**
 * Fund the PRIVATE_KEY signer with $ZAMA so its on-chain decryption requests can
 * pay the ProtocolPayment fee (1e18 per request, pulled via transferFrom).
 */
async function provisionGatewayZama(privateKey: string): Promise<void> {
  const zamaAddress = requireEnv('ZAMA_OFT_ADDRESS');
  const paymentAddress = requireEnv('PROTOCOL_PAYMENT_ADDRESS');

  const gatewayUrl = requireEnv('GATEWAY_RPC_URL');
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
    throw new Error(`PRIVATE_KEY signer ${gwWallet.address} has 0 gas on the gateway chain.`);
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

async function main(): Promise<void> {
  const privateKey = requireEnv('PRIVATE_KEY');

  // test/instance reads its addresses/urls from process.env at import time; the
  // container already loads them from the stack env files.
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
        'This account (PRIVATE_KEY) must be funded on the host chain to submit the\n' +
        'input transaction. Fund it, or point PRIVATE_KEY at a funded key.',
    );
  }

  await provisionGatewayZama(privateKey);

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

  // Real readiness check: decrypt before printing the config values.
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

  // Print the verified values to paste into test-suite/gateway-stress/config/config.toml.
  console.log('\n=== gateway-stress config values ===');
  console.log(`allowed_contract = "${contractAddress}"`);
  console.log('\n[[public_ct]]');
  console.log(`handle = "${publicHandle}"`);
  console.log('\n[[user_ct]]');
  console.log(`handle = "${userHandle}"`);
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
