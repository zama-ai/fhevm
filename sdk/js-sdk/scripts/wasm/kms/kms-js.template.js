/* @ts-self-types="./kms_lib.d.ts" */

/* __KMS_JS_BODY__ */

function getWasmInfo() {
  const memory = wasm?.memory;
  return {
    name: 'tkms',
    version: '__KMS_VERSION__',
    downloadFiles: __KMS_DOWNLOAD_FILES_JSON__,
    memory:
      memory === undefined
        ? undefined
        : {
            byteLength: memory.buffer.byteLength,
            pages: memory.buffer.byteLength / 65536,
          },
  };
}

export { initSync, getWasmInfo, __wbg_init as initAsync };
export default __wbg_init;
