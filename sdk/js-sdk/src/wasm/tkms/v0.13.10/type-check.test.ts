// sdk/js-sdk/src/wasm/tkms/v0.13.10/type-check.test.ts
//
// Verifies that this version's generated kms_lib module matches the canonical
// API contract declared in ../KmsLibApi.d.ts. Any drift here is a hard type
// error at build time, before the module ever ships.

import type * as Mod from './kms_lib.js';
import type {
  Client,
  KmsLibApi,
  PrivateEncKeyMlKem512,
  PublicEncKeyMlKem512,
  ServerIdAddr,
  TypedPlaintext,
} from '../KmsLibApi.js';

type IfEquals<X, Y, A = true, B = false> = (<T>() => T extends X ? 1 : 2) extends <T>() => T extends Y ? 1 : 2 ? A : B;
type Assert<T extends true> = T;
type PublicShape<T> = { [K in keyof T]: T[K] };

// --- Classes (public structural shape) ---
type _Client = Assert<IfEquals<PublicShape<Mod.Client>, PublicShape<Client>>>;
type _PubEnc = Assert<IfEquals<PublicShape<Mod.PublicEncKeyMlKem512>, PublicShape<PublicEncKeyMlKem512>>>;
type _PrivEnc = Assert<IfEquals<PublicShape<Mod.PrivateEncKeyMlKem512>, PublicShape<PrivateEncKeyMlKem512>>>;
type _ServerIdAddr = Assert<IfEquals<PublicShape<Mod.ServerIdAddr>, PublicShape<ServerIdAddr>>>;
type _TypedPlain = Assert<IfEquals<PublicShape<Mod.TypedPlaintext>, PublicShape<TypedPlaintext>>>;

// --- Free functions matching the KmsLibApi runtime contract ---
// Note: `default` (wasm-bindgen __wbg_init) is intentionally excluded — its
// InitInput / InitOutput shapes are version-specific by design.
type _getWasmInfo = Assert<IfEquals<typeof Mod.getWasmInfo, KmsLibApi['getWasmInfo']>>;
type _ml_kem_pke_keygen = Assert<IfEquals<typeof Mod.ml_kem_pke_keygen, KmsLibApi['ml_kem_pke_keygen']>>;
type _ml_kem_pke_get_pk = Assert<IfEquals<typeof Mod.ml_kem_pke_get_pk, KmsLibApi['ml_kem_pke_get_pk']>>;
type _ml_kem_pke_pk_to_u8vec = Assert<IfEquals<typeof Mod.ml_kem_pke_pk_to_u8vec, KmsLibApi['ml_kem_pke_pk_to_u8vec']>>;
type _ml_kem_pke_sk_to_u8vec = Assert<IfEquals<typeof Mod.ml_kem_pke_sk_to_u8vec, KmsLibApi['ml_kem_pke_sk_to_u8vec']>>;
type _u8vec_to_ml_kem_pke_sk = Assert<IfEquals<typeof Mod.u8vec_to_ml_kem_pke_sk, KmsLibApi['u8vec_to_ml_kem_pke_sk']>>;
type _new_server_id_addr = Assert<IfEquals<typeof Mod.new_server_id_addr, KmsLibApi['new_server_id_addr']>>;
type _new_client = Assert<IfEquals<typeof Mod.new_client, KmsLibApi['new_client']>>;
type _process_user_decryption_resp_from_js = Assert<
  IfEquals<typeof Mod.process_user_decryption_resp_from_js, KmsLibApi['process_user_decryption_resp_from_js']>
>;

// Bundle everything so noUnusedLocals doesn't flag them.
export type _NeverUse = _Client &
  _PubEnc &
  _PrivEnc &
  _ServerIdAddr &
  _TypedPlain &
  _getWasmInfo &
  _ml_kem_pke_keygen &
  _ml_kem_pke_get_pk &
  _ml_kem_pke_pk_to_u8vec &
  _ml_kem_pke_sk_to_u8vec &
  _u8vec_to_ml_kem_pke_sk &
  _new_server_id_addr &
  _new_client &
  _process_user_decryption_resp_from_js &
  never;
