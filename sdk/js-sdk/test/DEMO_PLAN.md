In a separate folder of /Users/alex/src/me/zama-ai/fhevm/sdk/js-sdk/test/multi-wasm
I want a similar browser test that involves the real navigator UI (Chrome or Safari)
With a big text box where I can type a value
a big button to encrypt
a progress message + spinner
Then a big Decrypt button to decrypt.
I Also want a few checkboxes to specify the wams loading options

- location: /Users/alex/src/me/zama-ai/fhevm/sdk/js-sdk/test/browser-ui

- asset source: local / jsdelivr / unpkg
- TFHE version: 1.5.3 / 1.6.1 / 1.6.0-dev
- KMS version: 0.13.10 / 0.13.20-0
- As in /Users/alex/src/me/zama-ai/fhevm/sdk/js-sdk/test/multi-wasm use FHETest as the source of handles
- threaded (workers + SAB) vs single-thread
- manual init vs lazy init
- encrypt/decrypt a uint64
- decryption flavour: both: publicDecrypt and userDecrypt (use a checkbox for that)
- Chain target is a list: testnet, localstack, localcleartext
- Signer: use a private key defined in .env MNEMONIC
- Dropdown menu: embedded-base64, verified-blob, precheck-direct-url, trusted-direct-url, auto
- Radio button for TFHE version
- Radio button for KMS version
- Radio: jsdelivr | unpkg | local
- find RPC URLS the same way as for all other tests (parse .env.xxxx)
- localstack is the docker version (see existing test folder)

  "assetUrlSets": {
  "local": {
  "tfheWasm": "/src/wasm/tfhe/v{tfhe}/tfhe_bg.wasm",
  "tfheWorker": "/**raw_wasm/src/wasm/tfhe/v{tfhe}/tfhe-worker.mjs",
  "kmsWasm": "/src/wasm/tkms/v{kms}/kms_lib_bg.wasm"
  },
  "jsdelivr": {
  "tfheWasm": "https://cdn.jsdelivr.net/npm/tfhe@{tfhe}/tfhe_bg.wasm",
  "tfheWorker": "/**raw_wasm/src/wasm/tfhe/v{tfhe}/tfhe-worker.mjs",
  "kmsWasm": "https://cdn.jsdelivr.net/npm/tkms@{kms}/kms_lib_bg.wasm"
  },
  "unpkg": {
  "tfheWasm": "https://unpkg.com/tfhe@{tfhe}/tfhe_bg.wasm",
  "tfheWorker": "/\_\_raw_wasm/src/wasm/tfhe/v{tfhe}/tfhe-worker.mjs",
  "kmsWasm": "https://unpkg.com/tkms@{kms}/kms_lib_bg.wasm"
  }
  }

/\*\*

- Worker load mode security guarantees:
-
- embedded-base64 Integrity: build-time. Inherits the JS bundle's integrity.
- verified-blob Integrity: runtime SHA-256 of fetched bytes; executed bytes
-                    are the verified bytes themselves.
- precheck-direct-url No integrity guarantee. The SDK fetches the URL once and
-                      validates SHA-256, then the runtime fetches the URL a
-                      second time and executes those (unverified) bytes. Use
-                      for fail-fast on misconfigured URLs / wrong builds, not
-                      for protection against on-path or CDN-edge tampering.
- trusted-direct-url No integrity check. Use only when the URL is fully trusted
-                    (e.g., same-origin static asset).
- auto Tries verified-blob if workerUrl is set, falls back to
-                    embedded-base64 on any non-SHA-256 error. SHA-256 mismatch
-                    is always fatal and never falls back.
  \*/
