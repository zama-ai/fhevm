// sdk/js-sdk/src/wasm/tkms/type-check.ts

import type * as V_0_13_10 from '../../src/wasm/tkms/v0.13.10/kms_lib.js';
import type * as V_0_13_20_0 from '../../src/wasm/tkms/v0.13.20-0/kms_lib.js';

type IfEquals<X, Y, A = true, B = false> = (<T>() => T extends X ? 1 : 2) extends <T>() => T extends Y ? 1 : 2 ? A : B;
type Assert<T extends true> = T;
type PublicShape<T> = { [K in keyof T]: T[K] };

// --- Classes (public structural shape) ---
type _Client = Assert<IfEquals<PublicShape<V_0_13_10.Client>, PublicShape<V_0_13_20_0.Client>>>;
type _PubEnc = Assert<
  IfEquals<PublicShape<V_0_13_10.PublicEncKeyMlKem512>, PublicShape<V_0_13_20_0.PublicEncKeyMlKem512>>
>;
type _PrivEnc = Assert<
  IfEquals<PublicShape<V_0_13_10.PrivateEncKeyMlKem512>, PublicShape<V_0_13_20_0.PrivateEncKeyMlKem512>>
>;
type _ServerIdAddr = Assert<IfEquals<PublicShape<V_0_13_10.ServerIdAddr>, PublicShape<V_0_13_20_0.ServerIdAddr>>>;
type _TypedPlain = Assert<IfEquals<PublicShape<V_0_13_10.TypedPlaintext>, PublicShape<V_0_13_20_0.TypedPlaintext>>>;

// --- Free functions you actually import in api-p.ts ---
type _ml_kem_pke_keygen = Assert<IfEquals<typeof V_0_13_10.ml_kem_pke_keygen, typeof V_0_13_20_0.ml_kem_pke_keygen>>;
type _ml_kem_pke_get_pk = Assert<IfEquals<typeof V_0_13_10.ml_kem_pke_get_pk, typeof V_0_13_20_0.ml_kem_pke_get_pk>>;
type _ml_kem_pke_pk_to_u8vec = Assert<
  IfEquals<typeof V_0_13_10.ml_kem_pke_pk_to_u8vec, typeof V_0_13_20_0.ml_kem_pke_pk_to_u8vec>
>;
type _ml_kem_pke_sk_to_u8vec = Assert<
  IfEquals<typeof V_0_13_10.ml_kem_pke_sk_to_u8vec, typeof V_0_13_20_0.ml_kem_pke_sk_to_u8vec>
>;
type _u8vec_to_ml_kem_pke_sk = Assert<
  IfEquals<typeof V_0_13_10.u8vec_to_ml_kem_pke_sk, typeof V_0_13_20_0.u8vec_to_ml_kem_pke_sk>
>;
type _new_server_id_addr = Assert<IfEquals<typeof V_0_13_10.new_server_id_addr, typeof V_0_13_20_0.new_server_id_addr>>;
type _new_client = Assert<IfEquals<typeof V_0_13_10.new_client, typeof V_0_13_20_0.new_client>>;
type _process_user_decryption_resp_from_js = Assert<
  IfEquals<
    typeof V_0_13_10.process_user_decryption_resp_from_js,
    typeof V_0_13_20_0.process_user_decryption_resp_from_js
  >
>;

// Bundle everything so noUnusedLocals doesn't flag them.
export type _NeverUse = _Client &
  _PubEnc &
  _PrivEnc &
  _ServerIdAddr &
  _TypedPlain &
  _ml_kem_pke_keygen &
  _ml_kem_pke_get_pk &
  _ml_kem_pke_pk_to_u8vec &
  _ml_kem_pke_sk_to_u8vec &
  _u8vec_to_ml_kem_pke_sk &
  _new_server_id_addr &
  _new_client &
  _process_user_decryption_resp_from_js &
  never;
