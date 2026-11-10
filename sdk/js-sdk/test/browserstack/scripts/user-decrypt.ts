import { setFhevmRuntimeConfig, createFhevmDecryptClient } from '../../../src/ethers/index.js';
import { sepolia } from '../../../src/core/chains/index.js';
import { ethers } from 'ethers';
import { createLogger, createPageHarness, fetchFheTestAddress } from './common.js';

const { log, done, elapsed } = createPageHarness();

// FheType.Bool, per contracts/src/v0.13.0/host-contracts/contracts/shared/FheType.sol
const FHE_TYPE_BOOL = 0;
const EXPECTED_TYPE = 'bool';
// FHETest.initFheTest() always sets the caller's ebool handle to `true`.
const EXPECTED_VALUE = true;

// Minimal ABI to self-provision and read back a per-wallet FHETest handle
const FHE_TEST_ABI = [
  'function hasHandleOf(address account, uint8 fheType) view returns (bool)',
  'function getHandleOf(address account, uint8 fheType) view returns (bytes32)',
  'function initFheTest(bool force) external',
];

async function run() {
  const mnemonic = import.meta.env.MNEMONIC;

  if (!mnemonic) {
    log('[SKIP] MNEMONIC not set — cannot derive signer wallet');
    done('fail');
    return;
  }

  try {
    log('Setting runtime config...');
    setFhevmRuntimeConfig({
      logger: createLogger(log),
    });
    log('[PASS] Runtime config set');

    //
    // 1. Resolve the Sepolia FHETest contract address
    //
    log('Resolving FHETest contract address...');
    const FHE_TEST_ADDRESS = await fetchFheTestAddress('sepolia');
    log(`[PASS] FHETest address: ${FHE_TEST_ADDRESS}`);

    //
    // 2. Derive wallet from mnemonic
    //
    log('Deriving wallet from mnemonic...');
    const provider = new ethers.JsonRpcProvider('https://ethereum-sepolia-rpc.publicnode.com');
    const wallet = ethers.HDNodeWallet.fromMnemonic(ethers.Mnemonic.fromPhrase(mnemonic)).connect(provider);
    log(`[PASS] Wallet address: ${wallet.address}`);

    //
    // 3. Ensure the wallet owns an FHETest ebool handle (self-provision if missing)
    //
    const fheTestContract = new ethers.Contract(FHE_TEST_ADDRESS, FHE_TEST_ABI, wallet);
    const hasHandle: boolean = await fheTestContract.getFunction('hasHandleOf')(wallet.address, FHE_TYPE_BOOL);
    if (!hasHandle) {
      log('No FHETest handle for this wallet yet — calling initFheTest()...');
      const initTx = await fheTestContract.getFunction('initFheTest')(false);
      log(`  tx hash: ${initTx.hash}`);
      await initTx.wait();
      log('[PASS] FHETest handles initialized');
    } else {
      log('[PASS] FHETest handle already exists for this wallet');
    }

    const testHandle: string = await fheTestContract.getFunction('getHandleOf')(wallet.address, FHE_TYPE_BOOL);
    log(`[PASS] Using handle ${testHandle.slice(0, 20)}...`);

    //
    // 4. Create decrypt client
    //
    log('Creating decrypt client...');
    const client = createFhevmDecryptClient({
      chain: sepolia,
      provider,
    });
    log('[PASS] Decrypt client created');

    //
    // 5. Init (loads TKMS WASM + TFHE WASM)
    //
    log('Waiting for client.ready (TKMS + TFHE WASM)...');
    await client.ready;
    log('[PASS] Decrypt client ready');

    //
    // 6. Generate transport keypair (exercises TKMS WASM)
    //
    log('Generating transport keypair...');
    const transportKeyPair = await client.generateTransportKeyPair();
    if (!transportKeyPair) {
      throw new Error('generateTransportKeyPair returned undefined');
    }
    log('[PASS] Transport keypair generated');

    //
    // 7. Sign decryption permit (EIP-712 signing)
    //
    log('Signing decryption permit...');
    const signedPermit = await client.signDecryptionPermit({
      transportKeyPair,
      contractAddresses: [FHE_TEST_ADDRESS],
      durationSeconds: 24 * 60 * 60,
      startTimestamp: Math.floor(Date.now() / 1000),
      signerAddress: wallet.address,
      signer: wallet,
    });
    if (!signedPermit) {
      throw new Error('signDecryptionPermit returned undefined');
    }
    log(`[PASS] Decryption permit signed (isDelegated: ${signedPermit.isDelegated})`);

    //
    // 8. Decrypt value (exercises full decrypt pipeline: relayer + TKMS)
    //
    log(`Decrypting handle ${testHandle.slice(0, 20)}...`);
    const typedValue = await client.decryptValue({
      contractAddress: FHE_TEST_ADDRESS,
      encryptedValue: testHandle,
      signedPermit,
      transportKeyPair,
    });

    log(`  type: ${typedValue.type}, value: ${String(typedValue.value)}`);

    if (typedValue.type !== EXPECTED_TYPE) {
      throw new Error(`Type mismatch: expected "${EXPECTED_TYPE}", got "${typedValue.type}"`);
    }

    if (typedValue.value !== EXPECTED_VALUE) {
      throw new Error(`Value mismatch: expected ${String(EXPECTED_VALUE)}, got ${String(typedValue.value)}`);
    }

    log('[PASS] Decrypted value matches expected');

    log(`\nAll user decrypt checks passed in ${elapsed()}ms`);
    done('pass');
  } catch (err) {
    log(`[FAIL] ${err}`);
    done('fail');
  }
}

run();
