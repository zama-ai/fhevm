// AUTO-GENERATED FROM scripts/wasm/loaders/KmsLibApi.template.d.ts - DO NOT EDIT.
// Generator: scripts/build/codegen-loaders.mjs

// Shared opaque types from kms_lib. We pick v0.13.10 as the canonical source.
// The per-version `type-check.test.ts` files enforce that every supported
// version exposes the same public shape, so any of them would do.
//
// `export type` makes these declaration-only: no JS import statement is emitted
// and `noEmit`/`isolatedModules` builds produce zero runtime code from this file.
export type {
  Client,
  PrivateEncKeyMlKem512,
  PublicEncKeyMlKem512,
  ServerIdAddr,
  TypedPlaintext,
} from './v0.13.10/kms_lib.js';

/** The subset you actually use - the runtime contract callers depend on. */
export interface KmsLibApi {
  initAsync: typeof import('./v0.13.10/kms_lib.js').initAsync;
  getWasmInfo: typeof import('./v0.13.10/kms_lib.js').getWasmInfo;

  ml_kem_pke_keygen: typeof import('./v0.13.10/kms_lib.js').ml_kem_pke_keygen;
  ml_kem_pke_get_pk: typeof import('./v0.13.10/kms_lib.js').ml_kem_pke_get_pk;
  ml_kem_pke_pk_to_u8vec: typeof import('./v0.13.10/kms_lib.js').ml_kem_pke_pk_to_u8vec;
  new_server_id_addr: typeof import('./v0.13.10/kms_lib.js').new_server_id_addr;
  new_client: typeof import('./v0.13.10/kms_lib.js').new_client;
  process_user_decryption_resp_from_js: typeof import('./v0.13.10/kms_lib.js').process_user_decryption_resp_from_js;
  ml_kem_pke_sk_to_u8vec: typeof import('./v0.13.10/kms_lib.js').ml_kem_pke_sk_to_u8vec;
  u8vec_to_ml_kem_pke_sk: typeof import('./v0.13.10/kms_lib.js').u8vec_to_ml_kem_pke_sk;
}

// Default version
export type TkmsVersion = '0.13.10' | '0.13.20-0';

export type TkmsWasmBase64 = {
  readonly tkmsWasmBase64: string;
  readonly tkmsWasmBase64IsGzipped: boolean;
  readonly tkmsWasmBase64CompressionFormat: 'gzip' | 'deflate' | 'deflate-raw' | undefined;
};
