/* __KMS_DTS_BODY__ */

////////////////////////////////////////////////////////////////////////////////

export function initAsync(...args: Parameters<typeof __wbg_init>): ReturnType<typeof __wbg_init>;

export function getWasmInfo(): {
  name: string;
  version: string;
  downloadFiles: readonly {
    filename: string;
    sha256: string;
  }[];
  memory?: {
    byteLength: number;
    pages: number;
  };
};
