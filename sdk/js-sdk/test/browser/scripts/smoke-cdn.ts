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

// TFHE + TKMS .wasm binaries are fetched cross-origin from jsdelivr. The
// TFHE worker (.mjs) stays local: browsers refuse to instantiate a
// cross-origin module worker, and the file is SDK-specific anyway.
const WASM_URLS: Record<string, URL> = {
  'tfhe_bg.v1.5.3.wasm': new URL('https://cdn.jsdelivr.net/npm/tfhe@1.5.3/tfhe_bg.wasm'),
  'tfhe-worker.v1.5.3.mjs': new URL('/src/wasm/tfhe/tfhe-worker.v1.5.3.mjs', location.origin),
  'kms_lib_bg.v0.13.10.wasm': new URL('https://cdn.jsdelivr.net/npm/tkms@0.13.10/kms_lib_bg.wasm'),
};

async function run() {
  try {
    log('Setting runtime config (CDN-based WASM)...');
    setFhevmRuntimeConfig({
      locateFile: (file: string): URL => {
        const url = WASM_URLS[file];
        if (!url) {
          throw new Error(`Unknown WASM file: ${file}`);
        }
        return url;
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
