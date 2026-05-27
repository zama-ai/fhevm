interface ImportMetaEnv {
  readonly VITE_ZAMA_API_KEY?: string;
  readonly VITE_MNEMONIC?: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}
