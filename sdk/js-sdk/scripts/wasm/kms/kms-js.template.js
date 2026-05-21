/* @ts-self-types="./kms_lib.d.ts" */

/* __KMS_JS_BODY__ */

function getWasmInfo() {
  return {
    name: 'tkms',
    version: '__KMS_VERSION__',
    downloadFiles: __KMS_DOWNLOAD_FILES_JSON__,
  };
}

export { initSync, getWasmInfo, __wbg_init as initAsync };
export default __wbg_init;
