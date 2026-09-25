import { setFhevmRuntimeConfig, createFhevmClient } from '../../../src/ethers/index.js';
import { sepolia } from '../../../src/core/chains/index.js';
import { ethers } from 'ethers';
import { createLogger } from './common.js';
import { CANONICAL_WASM_VERSIONS } from '../../../src/core/runtime/WasmVersions-p.js';

const { tfhe: tfheVersion, kms: tkmsVersion } = CANONICAL_WASM_VERSIONS;

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
  [`tfhe_bg.v${tfheVersion}.wasm`]: new URL(`/__raw_wasm/src/wasm/tfhe/v${tfheVersion}/tfhe_bg.wasm`, location.origin),
  [`tfhe-worker.v${tfheVersion}.mjs`]: new URL(
    `/__raw_wasm/src/wasm/tfhe/v${tfheVersion}/tfhe-worker.mjs`,
    location.origin,
  ),
  [`kms_lib_bg.v${tkmsVersion}.wasm`]: new URL(
    `/__raw_wasm/src/wasm/tkms/v${tkmsVersion}/kms_lib_bg.wasm`,
    location.origin,
  ),
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
