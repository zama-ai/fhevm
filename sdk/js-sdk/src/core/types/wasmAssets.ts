/*
 * verified-blob:
 * --------------
 * Creates a worker from the configured URL after SHA-256 verification.
 * 1. Reuse cached verified bytes.
 * 2. Execute those exact bytes as a Blob worker in browsers.
 * 3. Execute those exact bytes as an eval worker in Node.
 *
 * - Transport: url
 * - Blob: yes
 * - SHA256: yes
 *
 * precheck-direct-url:
 * --------------------
 * Creates a worker by passing the configured URL directly to the runtime, after a pre-flight SHA-256 probe.
 *
 * IMPORTANT: this is NOT an integrity check. The SDK fetches the URL once to validate
 * the hash, then hands the URL to the runtime, which fetches it a SECOND time and
 * executes those bytes. The two fetches are independent — the executed bytes are
 * never verified. Use only for fail-fast on misconfigured URLs / build mismatches.
 *
 * For an actual integrity guarantee, use `verified-blob` (requires CSP allowing blob: workers).
 *
 * 1. Fetch the URL and verify its SHA-256 against __TFHE_WORKER_URL_SHA256_JSON__ — fails fast on mismatch.
 * 2. Discard the verified bytes.
 * 3. Let the runtime fetch the same URL again and execute it (no verification on this fetch).
 *
 * - Transport: url
 * - Blob: no
 * - SHA256: yes
 *
 * trusted-direct-url:
 * -------------------
 * Creates a worker by passing the configured URL directly to the runtime.
 * 1. Require a configured worker URL.
 * 2. Do not perform SDK byte verification.
 * 3. Let the browser or Node runtime load and execute the URL directly.
 *
 * - Transport: url
 * - Blob: no
 * - SHA256: no
 *
 * embedded-base64:
 * ----------------
 * Creates a worker from the SDK-embedded base64 worker source.
 * 1. Read the base64-encoded JavaScript source baked into this module.
 * 2. Decode into a Blob URL and create a module Worker in browsers.
 * 3. Decode into UTF-8 JavaScript and create a worker_threads eval Worker in Node.
 *
 * - Transport: base64
 * - Blob: yes
 * - SHA256: no
 *
 * auto:
 * -----
 * if workerUrl is defined, try it using `verified-blob`.
 * if workerUrl failed or is undefined go for `embedded-base64`
 */
export type WasmAssetLoadMode =
  | 'embedded-base64'
  | 'verified-blob'
  | 'precheck-direct-url'
  | 'trusted-direct-url'
  | 'auto';
