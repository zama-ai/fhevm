import { setFhevmRuntimeConfig, createFhevmClient } from '../../../src/ethers/index.js';
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

async function run() {
  try {
    log('Setting runtime config (base64 WASM)...');
    setFhevmRuntimeConfig({
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
    // 1. Call createFhevmClient
    //
    log('Creating client...');
    const client = createFhevmClient({
      chain: sepolia,
      provider: new ethers.JsonRpcProvider('https://ethereum-sepolia-rpc.publicnode.com'),
    });
    log('[PASS] Client created');

    //
    // 2. Manually init client
    //
    log('Initializing (WASM + workers + global FHE key)...');
    await client.init();
    log('[PASS] Client initialized');

    //
    // 3. Display TFHE module infos
    //
    const tfheInfo = client.runtime.encrypt.getTfheModuleInfo();
    if (!tfheInfo) {
      throw new Error('TFHE module not initialized after client.init()');
    }
    log(`  [TFHE Module] threads: ${tfheInfo.numberOfThreads} (available: ${tfheInfo.threadsAvailable})`);
    log(`  [TFHE Module] wasmUrl: ${tfheInfo.wasmUrl ?? 'base64'}`);
    log(`  [TFHE Module] workerUrl: ${tfheInfo.workerUrl ?? 'base64'}`);

    //
    // 4. Display TKMS module infos
    //
    const tkmsInfo = client.runtime.decrypt.getTkmsModuleInfo();
    if (!tkmsInfo) {
      throw new Error('TKMS module not initialized after client.init()');
    }
    log(`  [TKMS Module] wasmUrl: ${tkmsInfo.wasmUrl ?? 'base64'}`);

    const elapsed = (performance.now() - t0).toFixed(0);
    log(`\nAll checks passed in ${elapsed}ms`);
    done('pass');
  } catch (err) {
    log(`[FAIL] ${err}`);
    done('fail');
  }
}

run();
