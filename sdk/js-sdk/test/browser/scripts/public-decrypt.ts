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

const WASM_URLS: Record<string, string> = {
  'tfhe_bg.v1.5.3.wasm': '/src/wasm/tfhe/tfhe_bg.v1.5.3.wasm',
  'tfhe-worker.v1.5.3.mjs': '/src/wasm/tfhe/tfhe-worker.v1.5.3.mjs',
  'kms_lib_bg.v0.13.10.wasm': '/src/wasm/tkms/kms_lib_bg.v0.13.10.wasm',
};

// Known Sepolia FHETest public handles (from handles.sepolia.json)
// These handles are publicly decryptable via makePubliclyDecryptable()
const PUBLIC_HANDLES = [
  {
    bytes32Hex: '0xf6751d547a5c06123575aad93f22f76b7d841c4cacff0000000000aa36a70000',
    fheType: 'ebool',
    expectedType: 'bool',
    expectedValue: false,
  },
  {
    bytes32Hex: '0x6f17228bda73a5e57b94511c5bab2665e6a2870399ff0000000000aa36a70200',
    fheType: 'euint8',
    expectedType: 'uint8',
    expectedValue: 171,
  },
  {
    bytes32Hex: '0x9797f8eb707b0a32c47a80ea86c0648df36bfe7cd0ff0000000000aa36a70300',
    fheType: 'euint16',
    expectedType: 'uint16',
    expectedValue: 15764,
  },
  {
    bytes32Hex: '0xf1673094de7c833604f1b62183cbcdf2cdc968db90ff0000000000aa36a70400',
    fheType: 'euint32',
    expectedType: 'uint32',
    expectedValue: 1083783185,
  },
] as const;

async function run() {
  const apiKey = import.meta.env.VITE_ZAMA_API_KEY;
  if (!apiKey) {
    log('[SKIP] VITE_ZAMA_API_KEY not set — cannot call relayer');
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
    // 1. Create client
    //
    log('Creating client...');
    const client = createFhevmClient({
      chain: sepolia,
      provider: new ethers.JsonRpcProvider('https://ethereum-sepolia-rpc.publicnode.com'),
    });
    log('[PASS] Client created');

    //
    // 2. Init (loads WASM modules + fetches global FHE key)
    //
    log('Initializing...');
    await client.init();
    log('[PASS] Client initialized');

    //
    // 3. Public decrypt each handle and verify
    //
    for (const entry of PUBLIC_HANDLES) {
      log(`Reading public value for ${entry.fheType} (${entry.bytes32Hex.slice(0, 20)}...)...`);

      const result = await client.readPublicValue({
        encryptedValue: entry.bytes32Hex,
      });

      log(`  type: ${result.type}, value: ${result.value}`);

      if (result.type !== entry.expectedType) {
        throw new Error(
          `Type mismatch for ${entry.fheType}: expected "${entry.expectedType}", got "${result.type}"`,
        );
      }

      if (entry.expectedType === 'bool') {
        if (result.value !== entry.expectedValue) {
          throw new Error(
            `Value mismatch for ${entry.fheType}: expected ${String(entry.expectedValue)}, got ${String(result.value)}`,
          );
        }
      } else {
        if (BigInt(result.value as number | bigint) !== BigInt(entry.expectedValue)) {
          throw new Error(
            `Value mismatch for ${entry.fheType}: expected ${entry.expectedValue}, got ${String(result.value)}`,
          );
        }
      }

      log(`[PASS] ${entry.fheType} decrypted correctly`);
    }

    //
    // 4. Batch public decrypt via readPublicValues
    //
    log('Reading all public values in a single batch call...');
    const batchResult = await client.readPublicValues({
      encryptedValues: PUBLIC_HANDLES.map((h) => h.bytes32Hex),
    });

    if (batchResult.length !== PUBLIC_HANDLES.length) {
      throw new Error(`Batch result length mismatch: expected ${PUBLIC_HANDLES.length}, got ${batchResult.length}`);
    }
    log(`[PASS] Batch readPublicValues returned ${batchResult.length} results`);

    const elapsed = (performance.now() - t0).toFixed(0);
    log(`\nAll public decrypt checks passed in ${elapsed}ms`);
    done('pass');
  } catch (err) {
    log(`[FAIL] ${err}`);
    done('fail');
  }
}

run();
