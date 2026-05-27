import {
  setFhevmRuntimeConfig,
  createFhevmEncryptClient,
} from '../../../src/ethers/index.js';
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

// Minimal ABI for addEuint8 (calls TFHE.asEuint8 internally, which verifies the input proof)
const FHE_TEST_ABI = [
  {
    type: 'function',
    name: 'addEuint8',
    inputs: [
      { name: 'inputEuint8', type: 'bytes32', internalType: 'bytes32' },
      { name: 'inputProof', type: 'bytes', internalType: 'bytes' },
    ],
    outputs: [],
    stateMutability: 'nonpayable',
  },
];

async function run() {
  const apiKey = import.meta.env.VITE_ZAMA_API_KEY;
  const mnemonic = import.meta.env.VITE_MNEMONIC;

  if (!apiKey) {
    log('[SKIP] VITE_ZAMA_API_KEY not set');
    done('fail');
    return;
  }
  if (!mnemonic) {
    log('[SKIP] VITE_MNEMONIC not set — cannot submit on-chain tx');
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
    // 2. Create encrypt client
    //
    log('Creating encrypt client...');
    const client = createFhevmEncryptClient({
      chain: sepolia,
      provider,
    });
    log('[PASS] Encrypt client created');

    //
    // 3. Init (loads TFHE WASM + fetches global FHE key)
    //
    log('Initializing...');
    await client.init();
    log('[PASS] Client initialized');

    //
    // 4. Encrypt a uint8 value
    //
    const testValue = 42;
    log(`Encrypting uint8 value (${testValue})...`);
    const result = await client.encryptValue({
      contractAddress: FHE_TEST_ADDRESS,
      userAddress: wallet.address,
      value: { type: 'uint8', value: testValue },
    });

    log(`  encryptedValue: ${result.encryptedValue.slice(0, 20)}...`);
    log(`  inputProof: ${result.inputProof.slice(0, 40)}... (${result.inputProof.length} chars)`);
    log('[PASS] Encryption complete');

    //
    // 5. Submit encrypted input to FHETest.addEuint8() on-chain
    //    This internally calls TFHE.asEuint8(inputHandle, inputProof)
    //    which verifies the ZK proof on-chain
    //
    log('Submitting input to FHETest.addEuint8() on-chain...');
    const fheTestContract = new ethers.Contract(FHE_TEST_ADDRESS, FHE_TEST_ABI, wallet);
    const tx = await fheTestContract.getFunction('addEuint8')(result.encryptedValue, result.inputProof);

    log(`  tx hash: ${tx.hash}`);
    log('Waiting for confirmation...');
    const receipt = await tx.wait();

    if (!receipt || receipt.status !== 1) {
      throw new Error(`Transaction failed: status=${receipt?.status}`);
    }

    log(`  block: ${receipt.blockNumber}, gasUsed: ${receipt.gasUsed.toString()}`);
    log('[PASS] On-chain input verification succeeded');

    const elapsed = (performance.now() - t0).toFixed(0);
    log(`\nAll verify-input checks passed in ${elapsed}ms`);
    done('pass');
  } catch (err) {
    log(`[FAIL] ${err}`);
    done('fail');
  }
}

run();
