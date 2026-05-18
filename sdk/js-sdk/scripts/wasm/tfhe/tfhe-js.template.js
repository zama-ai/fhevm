/* @ts-self-types="./tfhe.d.ts" */
import { startWorkers, getTfheWorkers, terminateWorkers, setWorkerUrlConfig } from '__TFHE_START_WORKERS_IMPORT__';

/* __TFHE_JS_BODY__ */

function getWasmInfo() {
  return {
    name: 'tfhe',
    version: __TFHE_VERSION_JSON__,
    downloadFiles: __TFHE_DOWNLOAD_FILES_JSON__,
  };
}

export { initSync, getTfheWorkers, terminateWorkers, setWorkerUrlConfig, getWasmInfo };
export default __wbg_init;
