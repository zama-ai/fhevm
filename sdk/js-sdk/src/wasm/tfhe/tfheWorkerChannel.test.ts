import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { Worker } from 'node:worker_threads';

import { describe, expect, it } from 'vitest';

////////////////////////////////////////////////////////////////////////////////
// npx vitest run --config src/vitest.config.ts src/wasm/tfhe/tfheWorkerChannel.test.ts
////////////////////////////////////////////////////////////////////////////////

const here = dirname(fileURLToPath(import.meta.url));
const templatePath = resolve(here, '../../../scripts/wasm/tfhe/tfhe-worker.template.mjs');
const template = readFileSync(templatePath, 'utf8');

const versions = ['v1.5.3', 'v1.6.0-dev', 'v1.6.2'] as const;

/**
 * Channel bootstrap above the wasm-bindgen body: ___getTarget, ___waitForMsgType,
 * the auto-start handshake, and wbg_rayon_start_worker.
 */
function extractChannelBootstrap(source: string): string {
  const marker = '////////////////////////////////////////////////////////////////////////////////\n// Internal wasmbindgen tools';
  const idx = source.indexOf(marker);
  if (idx < 0) {
    throw new Error('missing wasmbindgen body marker');
  }
  return source.slice(0, idx);
}

/** Just ___getTarget + ___waitForMsgType, without the auto-start that calls `tfhe`. */
function extractChannelHelpers(source: string): string {
  const bootstrap = extractChannelBootstrap(source);
  const autoStart = bootstrap.indexOf('___getTarget().then');
  if (autoStart < 0) {
    throw new Error('missing ___getTarget().then auto-start');
  }
  return bootstrap.slice(0, autoStart);
}

function runInWorker<T>(code: string, timeoutMs = 5_000): Promise<T> {
  return new Promise((resolvePromise, reject) => {
    const worker = new Worker(code, { eval: true });
    let settled = false;
    const finish = (fn: () => void): void => {
      if (settled) return;
      settled = true;
      clearTimeout(timer);
      void worker.terminate();
      fn();
    };
    const timer = setTimeout(() => {
      finish(() => reject(new Error(`worker timed out after ${String(timeoutMs)}ms`)));
    }, timeoutMs);
    worker.on('message', (msg: T) => {
      finish(() => resolvePromise(msg));
    });
    worker.on('error', (err) => {
      finish(() => reject(err));
    });
    worker.on('exit', (code) => {
      finish(() => reject(new Error(`worker exited with code ${String(code)}`)));
    });
  });
}

describe('tfhe worker message channel', () => {
  it('keeps generated worker bootstraps in sync with tfhe-worker.template.mjs', () => {
    const expected = extractChannelBootstrap(template);
    for (const version of versions) {
      const generated = readFileSync(resolve(here, version, 'tfhe-worker.mjs'), 'utf8');
      expect(extractChannelBootstrap(generated), version).toBe(expected);
    }
  });

  it('startWorkers.js embeds the on-disk worker bytes (sha256)', () => {
    for (const version of versions) {
      const workerBytes = readFileSync(resolve(here, version, 'tfhe-worker.mjs'));
      const startWorkers = readFileSync(resolve(here, version, 'startWorkers.js'), 'utf8');
      const sha = createHash('sha256').update(workerBytes).digest('hex');
      expect(startWorkers, version).toContain(`const _workerUrlSha256 = ${JSON.stringify(sha)}`);
    }
  });

  it('listens on parentPort even when addEventListener exists (bun 1.4 worker_threads)', async () => {
    const helpers = extractChannelHelpers(template);
    const result = await runInWorker<{
      readonly usesParentPort: boolean;
      readonly usesSelf: boolean;
    }>(`
      if (typeof addEventListener !== 'function') {
        globalThis.addEventListener = () => {};
        globalThis.removeEventListener = () => {};
      }
      ${helpers}
      ___getTarget().then((target) => {
        const { parentPort } = require('worker_threads');
        parentPort.postMessage({
          usesParentPort: target === parentPort,
          usesSelf: target === globalThis,
        });
      });
    `);
    expect(result.usesParentPort).toBe(true);
    expect(result.usesSelf).toBe(false);
  });

  it('receives wasm_bindgen_worker_init on parentPort under a bun-like EventTarget leak', async () => {
    const helpers = extractChannelHelpers(template);
    const workerCode = `
      if (typeof addEventListener !== 'function') {
        globalThis.addEventListener = () => {};
        globalThis.removeEventListener = () => {};
      }
      ${helpers}
      ___getTarget().then((target) => {
        const { parentPort } = require('worker_threads');
        parentPort.postMessage({ stage: 'listening', targetIsParentPort: target === parentPort });
        return ___waitForMsgType(target, 'wasm_bindgen_worker_init');
      }).then((data) => {
        const { parentPort } = require('worker_threads');
        parentPort.postMessage({ stage: 'got-init', type: data.type });
      });
    `;

    const msgs = await new Promise<readonly Record<string, unknown>[]>((resolvePromise, reject) => {
      const worker = new Worker(workerCode, { eval: true });
      const collected: Record<string, unknown>[] = [];
      let settled = false;
      const finish = (fn: () => void): void => {
        if (settled) return;
        settled = true;
        clearTimeout(timer);
        void worker.terminate();
        fn();
      };
      const timer = setTimeout(() => {
        finish(() => reject(new Error(`handshake timed out; msgs=${JSON.stringify(collected)}`)));
      }, 5_000);
      worker.on('message', (msg: Record<string, unknown>) => {
        collected.push(msg);
        if (msg.stage === 'listening') {
          worker.postMessage({ type: 'wasm_bindgen_worker_init', init: { ok: true }, receiver: 1 });
        }
        if (msg.stage === 'got-init') {
          finish(() => resolvePromise(collected));
        }
      });
      worker.on('error', (err) => {
        finish(() => reject(err));
      });
    });

    expect(msgs).toEqual([
      { stage: 'listening', targetIsParentPort: true },
      { stage: 'got-init', type: 'wasm_bindgen_worker_init' },
    ]);
  });
});
