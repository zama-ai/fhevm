/* __TFHE_DTS_BODY__ */

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
export function getTfheWorkers(): object[];
export function terminateWorkers(): Promise<unknown>;
export type WasmAssetLoadMode =
  | 'embedded-base64'
  | 'verified-blob'
  | 'precheck-direct-url'
  | 'trusted-direct-url'
  | 'auto';
export function setWorkerUrlConfig(parameters?: {
  readonly workerUrl?: URL | undefined;
  readonly wasmAssetLoadMode?: WasmAssetLoadMode | undefined;
  // Required: the SDK injects the resolved runtime kind (browser vs Node); the
  // worker bootstrap no longer detects it itself.
  readonly isBrowserLike: boolean;
  readonly logger?:
    | {
        debug: (message: string) => void;
        error: (message: string, cause: unknown) => void;
      }
    | undefined;
}): void;
