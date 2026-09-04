1. Format `kms_lib.d.ts` using Prettier

2. In `kms_lib.d.ts` add:

```ts
////////////////////////////////////////////////////////////////////////////////

export function getWasmInfo(): { name: string; version: string };
```

3. In `kms_lib.js`, add `getWasmInfo` implementation:

Replace:

```ts
export { initSync, __wbg_init as default };
```

With:

```ts
function getWasmInfo() {
  return {
    name: 'tkms',
    version: '0.13.10',
  };
}

export { initSync, getWasmInfo, __wbg_init as default };
```

Note: specify the right `tkms` version.

4. In `kms_lib.js`:

Replace:

```js
if (module_or_path === undefined) {
  module_or_path = new URL('kms_lib_bg.wasm', import.meta.url);
}
const imports = __wbg_get_imports();

if (
  typeof module_or_path === 'string' ||
  (typeof Request === 'function' && module_or_path instanceof Request) ||
  (typeof URL === 'function' && module_or_path instanceof URL)
) {
  module_or_path = fetch(module_or_path);
}
```

With:

```js
// if (module_or_path === undefined) {
//     module_or_path = new URL('kms_lib_bg.wasm', import.meta.url);
// }
const imports = __wbg_get_imports();

// if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
//     module_or_path = fetch(module_or_path);
// }
```

5. In both `kms_lib.js` and `kms_lib.d.ts`

Replace original function signature:

```js
export function process_user_decryption_resp_from_js(
  client: Client,
  request: any,
  eip712_domain: any,
  agg_resp: any,
  enc_pk: PublicEncKeyMlKem512,
  enc_sk: PrivateEncKeyMlKem512,
  verify: boolean,
): TypedPlaintext[];
```

With:

```js
export function process_user_decryption_resp_from_js(
  client: Client,
  request: any,
  eip712_domain: any,
  agg_resp: any,
  enc_pk: PublicEncKeyMlKem512,
  enc_sk: PrivateEncKeyMlKem512,
  // Not in original version. Make it compatible with v0.13.20
  threshold: number | null | undefined,
  verify: boolean,
): TypedPlaintext[];
```

- We add `threshold` as an `unused` argument to make the function signature identical to v0.13.20-0
- Add a comment next to `threshold: number | null | undefined` to explain why it is important

6. I want a script that

- in a tmp folder: install tkms@<version>
- copy <tmp>/node_modules/tkms into sdk/js-sdk/src/wasm/tkms/<version>
- generates the wasm base64 js file
- patches `sdk/js-sdk/src/wasm/tkms/<version>/kms_lib.js` and `sdk/js-sdk/src/wasm/tkms/<version>/kms_lib.d.ts` according to the specs
- put the installer in `sdk/js-sdk/scripts/wasm/kms/install-tkms.sh`
- keep KMS-specific source transforms in `sdk/js-sdk/scripts/wasm/kms/build-kms.ts`
