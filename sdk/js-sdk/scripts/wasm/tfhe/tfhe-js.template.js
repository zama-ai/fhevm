/* @ts-self-types="./tfhe.d.ts" */
import { startWorkers, getTfheWorkers, terminateWorkers, setWorkerUrlConfig } from '__TFHE_START_WORKERS_IMPORT__';

/* __TFHE_JS_BODY__ */

function getWasmInfo() {
  const memory = wasm?.memory;
  return {
    name: 'tfhe',
    version: __TFHE_VERSION_JSON__,
    downloadFiles: __TFHE_DOWNLOAD_FILES_JSON__,
    memory:
      memory === undefined
        ? undefined
        : {
            byteLength: memory.buffer.byteLength,
            pages: memory.buffer.byteLength / 65536,
          },
  };
}

export { initSync, getTfheWorkers, terminateWorkers, setWorkerUrlConfig, getWasmInfo, __wbg_init as initAsync };
export default __wbg_init;
