import { setFhevmRuntimeConfig, createFhevmDecryptClient } from '../../../src/ethers/index.js';
import { sepolia } from '../../../src/core/chains/index.js';
import { ethers } from 'ethers';

const logEl = document.getElementById('log')!;
const t0 = performance.now();

function log(msg: string) {
  const elapsed = (performance.now() - t0).toFixed(0);
  logEl.textContent += `[${elapsed}ms] ${msg}\n`;
}

function done(status: 'pass' | 'fail') {
  const el = document.createElement('div');
  el.id = 'result';
  el.dataset.status = status;
  el.className = status;
  el.textContent = status.toUpperCase();
  document.body.appendChild(el);
}

const WASM_URLS: Record<string, string> = {
  'tfhe_bg.v1.5.3.wasm': '/src/wasm/tfhe/tfhe_bg.v1.5.3.wasm',
  'tfhe-worker.v1.5.3.mjs': '/src/wasm/tfhe/tfhe-worker.v1.5.3.mjs',
  'kms_lib_bg.v0.13.10.wasm': '/src/wasm/tkms/kms_lib_bg.v0.13.10.wasm',
};

// Sepolia FHETest contract address
const FHE_TEST_ADDRESS = '0x1E7eA8fE4877E6ea5dc8856f0dA92da8d5066241';

// Known Sepolia handle: ebool (publicly decryptable, owned by deployer)
const TEST_HANDLE = '0xf6751d547a5c06123575aad93f22f76b7d841c4cacff0000000000aa36a70000';
const EXPECTED_TYPE = 'bool';
const EXPECTED_VALUE = false;

async function run() {
  const apiKey = import.meta.env.VITE_ZAMA_API_KEY;
  const mnemonic = import.meta.env.VITE_MNEMONIC;

  if (!apiKey) {
    log('[SKIP] VITE_ZAMA_API_KEY not set');
    done('fail');
    return;
  }
  if (!mnemonic) {
    log('[SKIP] VITE_MNEMONIC not set — cannot derive signer wallet');
    done('fail');
    return;
  }

  try {
    log('Setting runtime config...');
    setFhevmRuntimeConfig({
      locateFile: (file: string): URL => {
        const path = WASM_URLS[file];
        if (!path) {
          throw new Error(`Unknown WASM file: ${file}`);
        }
        return new URL(path, location.origin);
      },
      auth: {
        type: 'ApiKeyHeader',
        value: apiKey,
      },
      logger: {
        debug: (message: string) => log(`  [debug] ${message}`),
        error: (message: string, cause: unknown) => {
          log(`  [error] ${message}`);
          if (cause !== undefined) {
            log(`  [error] ${cause}`);
          }
        },
      },
    });
    log('[PASS] Runtime config set');

    //
    // 1. Derive wallet from mnemonic
    //
    log('Deriving wallet from mnemonic...');
    const provider = new ethers.JsonRpcProvider('https://ethereum-sepolia-rpc.publicnode.com');
    const wallet = ethers.HDNodeWallet.fromMnemonic(
      ethers.Mnemonic.fromPhrase(mnemonic),
    ).connect(provider);
    log(`[PASS] Wallet address: ${wallet.address}`);

    //
    // 2. Create decrypt client
    //
    log('Creating decrypt client...');
    const client = createFhevmDecryptClient({
      chain: sepolia,
      provider,
    });
    log('[PASS] Decrypt client created');

    //
    // 3. Init (loads TKMS WASM + TFHE WASM)
    //
    log('Waiting for client.ready (TKMS + TFHE WASM)...');
    await client.ready;
    log('[PASS] Decrypt client ready');

    //
    // 4. Generate transport keypair (exercises TKMS WASM)
    //
    log('Generating transport keypair...');
    const transportKeypair = await client.generateTransportKeypair();
    if (!transportKeypair) {
      throw new Error('generateTransportKeypair returned undefined');
    }
    log('[PASS] Transport keypair generated');

    //
    // 5. Sign decryption permit (EIP-712 signing)
    //
    log('Signing decryption permit...');
    const signedPermit = await client.signDecryptionPermit({
      transportKeypair,
      contractAddresses: [FHE_TEST_ADDRESS],
      durationDays: 1,
      startTimestamp: Math.floor(Date.now() / 1000),
      signerAddress: wallet.address,
      signer: wallet,
    });
    if (!signedPermit) {
      throw new Error('signDecryptionPermit returned undefined');
    }
    log(`[PASS] Decryption permit signed (isDelegated: ${signedPermit.isDelegated})`);

    //
    // 6. Decrypt value (exercises full decrypt pipeline: relayer + TKMS)
    //
    log(`Decrypting handle ${TEST_HANDLE.slice(0, 20)}...`);
    const typedValue = await client.decryptValue({
      contractAddress: FHE_TEST_ADDRESS,
      encryptedValue: TEST_HANDLE,
      signedPermit,
      transportKeypair,
    });

    log(`  type: ${typedValue.type}, value: ${String(typedValue.value)}`);

    if (typedValue.type !== EXPECTED_TYPE) {
      throw new Error(`Type mismatch: expected "${EXPECTED_TYPE}", got "${typedValue.type}"`);
    }

    if (typedValue.value !== EXPECTED_VALUE) {
      throw new Error(`Value mismatch: expected ${String(EXPECTED_VALUE)}, got ${String(typedValue.value)}`);
    }

    log('[PASS] Decrypted value matches expected');

    const elapsed = (performance.now() - t0).toFixed(0);
    log(`\nAll user decrypt checks passed in ${elapsed}ms`);
    done('pass');
  } catch (err) {
    log(`[FAIL] ${err}`);
    done('fail');
  }
}

run();
