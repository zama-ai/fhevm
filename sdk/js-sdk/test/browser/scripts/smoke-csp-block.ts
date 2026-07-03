import { setFhevmRuntimeConfig, createFhevmClient } from '../../../src/ethers/index.js';
import { sepolia } from '../../../src/core/chains/index.js';
import { ethers } from 'ethers';
import { createLogger } from './common.js';

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

// Capture browser-level CSP violation reports for diagnostics.
document.addEventListener('securitypolicyviolation', (e) => {
  log(`[CSP-violation] directive=${e.violatedDirective} blocked=${e.blockedURI || e.sourceFile}`);
});

const WASM_URLS: Record<string, URL> = {
  'tfhe_bg.v1.5.3.wasm': new URL('/__raw_wasm/src/wasm/tfhe/v1.5.3/tfhe_bg.wasm', location.origin),
  'tfhe-worker.v1.5.3.mjs': new URL('/__raw_wasm/src/wasm/tfhe/v1.5.3/tfhe-worker.mjs', location.origin),
  'tfhe_bg.v1.6.2.wasm': new URL('/__raw_wasm/src/wasm/tfhe/v1.6.2/tfhe_bg.wasm', location.origin),
  'tfhe-worker.v1.6.2.mjs': new URL('/__raw_wasm/src/wasm/tfhe/v1.6.2/tfhe-worker.mjs', location.origin),
  'kms_lib_bg.v0.13.10.wasm': new URL('/__raw_wasm/src/wasm/tkms/v0.13.10/kms_lib_bg.wasm', location.origin),
  'kms_lib_bg.v0.13.20-0.wasm': new URL('/__raw_wasm/src/wasm/tkms/v0.13.20-0/kms_lib_bg.wasm', location.origin),
};

// Heuristic match for "WASM compile blocked by CSP" across Chromium / Firefox / WebKit.
const CSP_WASM_BLOCK_RE =
  /Wasm code generation disallowed|wasm-unsafe-eval|Content Security Policy|CompileError|disallowed by embedder|unsafe-eval/i;

async function run() {
  log('Setting runtime config...');
  setFhevmRuntimeConfig({
    locateFile: (file: string): URL => {
      const url = WASM_URLS[file];
      if (!url) {
        throw new Error(`Unknown WASM file: ${file}`);
      }
      return url;
    },
    logger: createLogger(log),
  });

  log('Creating client...');
  const client = createFhevmClient({
    chain: sepolia,
    provider: new ethers.JsonRpcProvider('https://ethereum-sepolia-rpc.publicnode.com'),
  });

  log("Calling client.init() — expecting failure because the page's CSP omits 'wasm-unsafe-eval'...");
  try {
    await client.init();
  } catch (err) {
    const msg = err instanceof Error ? `${err.name}: ${err.message}` : String(err);
    log(`Caught error: ${msg}`);
    if (CSP_WASM_BLOCK_RE.test(msg)) {
      log('[PASS] WASM compilation was rejected by the browser as expected');
      done('pass');
      return;
    }
    log('[FAIL] An error was thrown, but it did not look like a CSP/WASM-compile rejection');
    done('fail');
    return;
  }

  log('[FAIL] client.init() resolved successfully — CSP did NOT block WASM compilation');
  done('fail');
}

run();
