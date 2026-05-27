import { setFhevmRuntimeConfig, createFhevmEncryptClient } from '../../../src/ethers/index.js';
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

const dummyContractAddress = '0x1234567890abcdef1234567890abcdef12345678';
const dummyUserAddress = '0xabcdefabcdefabcdefabcdefabcdefabcdefabcd';

async function run() {
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
    // 1. Create encrypt client
    //
    log('Creating encrypt client...');
    const client = createFhevmEncryptClient({
      chain: sepolia,
      provider: new ethers.JsonRpcProvider('https://ethereum-sepolia-rpc.publicnode.com'),
    });
    log('[PASS] Encrypt client created');

    //
    // 2. Init (loads TFHE WASM + fetches global FHE key)
    //
    log('Initializing (TFHE WASM + global FHE key)...');
    await client.init();
    log('[PASS] Encrypt client initialized');

    //
    // 3. Encrypt all supported FHE types
    //
    log('Encrypting values (bool, uint8, uint16, uint32, uint64, uint128, uint256, address)...');
    const result = await client.encryptValues({
      contractAddress: dummyContractAddress,
      userAddress: dummyUserAddress,
      values: [
        { type: 'bool', value: true },
        { type: 'uint8', value: 42 },
        { type: 'uint16', value: 1000 },
        { type: 'uint32', value: 100_000 },
        { type: 'uint64', value: 1_000_000_000n },
        { type: 'uint128', value: 340_282_366_920_938_463_463_374_607_431_768_211_455n },
        { type: 'uint256', value: 115_792_089_237_316_195_423_570_985_008_687_907_853_269_984_665_640_564_039_457_584_007_913_129_639_935n },
        { type: 'address', value: '0x37AC010c1c566696326813b840319B58Bb5840E4' },
      ],
    });
    log('[PASS] Encryption complete');

    //
    // 4. Validate results
    //
    const { encryptedValues, inputProof } = result;

    if (!encryptedValues || encryptedValues.length !== 8) {
      throw new Error(`Expected 8 encrypted values, got ${encryptedValues?.length ?? 0}`);
    }
    log(`  [OK] Got ${encryptedValues.length} encrypted values (handles)`);

    for (let i = 0; i < encryptedValues.length; i++) {
      const handle = encryptedValues[i];
      if (typeof handle !== 'string' || !handle.startsWith('0x') || handle.length < 10) {
        throw new Error(`Invalid handle at index ${i}: ${handle}`);
      }
      log(`  [handle ${i}] ${handle.slice(0, 20)}...`);
    }

    if (!inputProof || typeof inputProof !== 'string' || !inputProof.startsWith('0x')) {
      throw new Error(`Invalid inputProof: ${inputProof}`);
    }
    log(`  [inputProof] ${inputProof.slice(0, 40)}... (${inputProof.length} chars)`);

    const elapsed = (performance.now() - t0).toFixed(0);
    log(`\nAll encryption checks passed in ${elapsed}ms`);
    done('pass');
  } catch (err) {
    log(`[FAIL] ${err}`);
    done('fail');
  }
}

run();
