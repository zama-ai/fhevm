import type { FheEncryptionKeyDigests } from '../../../src/core/key/fheEncryptionKeyDigests-p.js';
import { isBlobWorkerSupported, runCodeInIsomorphicWorker } from '../../../src/core/base/isomorphicWorker.js';
import { bytesToHexNo0x } from '../../../src/core/base/bytes.js';
import { computeFheEncryptionKeyDigests } from '../../../src/core/key/fheEncryptionKeyDigests-p.js';
import { FHE_ENCRYPTION_KEY_DIGESTS_WORKER_CODE } from '../../../src/core/key/fheEncryptionKeyDigestsWorkerCode-p.js';

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

function bytesOf(length: number, seed: number): Uint8Array {
  const bytes = new Uint8Array(length);
  let x = seed;
  for (let i = 0; i < length; i++) {
    x = (Math.imul(x, 1103515245) + 12345) >>> 0;
    bytes[i] = x >>> 24;
  }
  return bytes;
}

async function run() {
  try {
    if (!(await isBlobWorkerSupported())) {
      log('[FAIL] Blob workers are not supported on this page');
      done('fail');
      return;
    }
    log('[PASS] Blob workers are supported');

    // Sizes of the real public key and CRS.
    const publicKey = bytesOf(33_000, 1);
    const crs = bytesOf(4_600_000, 2);

    const tWorker = performance.now();
    const inWorker = await runCodeInIsomorphicWorker<FheEncryptionKeyDigests>(
      FHE_ENCRYPTION_KEY_DIGESTS_WORKER_CODE,
      { publicKey, crs },
      5_000,
    );
    log(`Worker digests in ${(performance.now() - tWorker).toFixed(0)}ms`);

    const tHere = performance.now();
    const here = computeFheEncryptionKeyDigests({ publicKey, crs });
    log(`Calling-thread digests in ${(performance.now() - tHere).toFixed(0)}ms`);

    const actual = `${bytesToHexNo0x(inWorker.publicKey)} ${bytesToHexNo0x(inWorker.crs)}`;
    const expected = `${bytesToHexNo0x(here.publicKey)} ${bytesToHexNo0x(here.crs)}`;
    log(`worker:         ${actual}`);
    log(`calling thread: ${expected}`);
    if (actual !== expected) {
      log('[FAIL] The worker digests differ from the calling-thread ones');
      done('fail');
      return;
    }
    if (crs.byteLength !== 4_600_000) {
      log('[FAIL] The CRS buffer was detached');
      done('fail');
      return;
    }

    log('[PASS] The worker digests equal the calling-thread ones');
    done('pass');
  } catch (err) {
    log(`[FAIL] ${err instanceof Error ? `${err.name}: ${err.message}` : String(err)}`);
    done('fail');
  }
}

run();
