### Functions used by the WASM -> JS imports

Functions: `getXXXFromWasm0` are only called by imports

- getStringFromWasm0 (only called by imports)
- getDataViewMemory0 (only called by imports)
- getArrayU8FromWasm0 (only called by imports)

- passStringToWasm0 (only called by imports)
- handleError (only called by imports)
- isLikeNone (only called by imports)
- addToExternrefTable0 (only called by imports + handleError)
- debugString (only called by imports) - wbg.\_\_wbg_wbindgendebugstring
- getUint8ArrayMemory0

Local vars:

- cachedUint8ArrayMemory0
- cachedTextDecoder
- numBytesDecoded
- cachedDataViewMemory0
- WASM_VECTOR_LEN
- MAX_SAFARI_DECODE_BYTES
- EXPECTED_RESPONSE_TYPES

### About WASM memory

- getUint8ArrayMemory0 returns the wasm memory.buffer as a Uint8Array, used by:
  - passStringToWasm0
  - getStringFromWasm0 (via decodeText)
  - getArrayU8FromWasm0

- getDataViewMemory0 returns the wasm memory.buffer as a DataView, used by:
  - imports only

- getArrayU8FromWasm0 returns the wasm memory.buffer as a Uint8Array, used by:
  - imports only

### How to

- copy function `__wbg_get_imports() {...}`
- copy all functions `__wbg_xxx`

-`__wbg_init`

and functions called by `__wbg_init`: -`__wbg_init_memory` (no more in v1.5.3) -`__wbg_finalize_init` -`__wbg_load`
