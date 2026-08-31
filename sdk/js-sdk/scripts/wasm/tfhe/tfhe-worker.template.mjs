async function ___getTarget() {
  // Prefer parentPort whenever this is a node:worker_threads worker.
  // `addEventListener` is not a browser signal: Bun exposes EventTarget APIs
  // inside worker_threads, but bun >= 1.4 delivers parent messages only to
  // parentPort (Node parity). Listening on `self` hangs startWorkers().
  try {
    const nodeModuleName = 'worker_threads';
    const nodeModuleId = `node:${nodeModuleName}`;
    const { parentPort } = await import(/* @vite-ignore */ nodeModuleId);
    if (parentPort) return parentPort;
  } catch {
    // Browser / edge: node:worker_threads is not importable.
  }
  return self;
}

function ___waitForMsgType(target, type) {
  return new Promise((resolve) => {
    if (typeof target.on === 'function') {
      // Node: EventEmitter, data passed directly
      target.on('message', function onMsg(data) {
        if (data?.type !== type) return;
        target.off('message', onMsg);
        resolve(data);
      });
    } else {
      // Browser: DOM events, data wrapped in MessageEvent
      target.addEventListener('message', function onMsg({ data }) {
        if (data?.type !== type) return;
        target.removeEventListener('message', onMsg);
        resolve(data);
      });
    }
  });
}

___getTarget().then((target) =>
  ___waitForMsgType(target, 'wasm_bindgen_worker_init').then(
    async ({ init, receiver }) => {
      const pkg = await Promise.resolve().then(function () {
        return tfhe;
      });
      await pkg.default(init);
      target.postMessage({ type: 'wasm_bindgen_worker_ready' });
      pkg.wbg_rayon_start_worker(receiver);
    },
  ),
);

/**
 * @param {number} receiver
 */
function wbg_rayon_start_worker(receiver) {
  wasm.wbg_rayon_start_worker(receiver);
}

////////////////////////////////////////////////////////////////////////////////
// Internal wasmbindgen tools
////////////////////////////////////////////////////////////////////////////////

/* __TFHE_WORKER_BODY__ */

////////////////////////////////////////////////////////////////////////////////
//
// The 'tfhe' global object
// ========================
// Final tfhe object global declaration called by 'waitForMsgType' only
//
////////////////////////////////////////////////////////////////////////////////

var tfhe = /*#__PURE__*/ Object.freeze({
  __proto__: null,
  default: __wbg_init,
  wbg_rayon_start_worker: wbg_rayon_start_worker,
});
