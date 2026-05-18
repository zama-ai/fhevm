function ___isBrowserLike() {
  return (
    typeof addEventListener === 'function' &&
    typeof removeEventListener === 'function'
  );
}

async function ___getTarget() {
  if (___isBrowserLike()) return self;
  const nodeModuleName = 'worker_threads';
  const nodeModuleId = `node:${nodeModuleName}`;
  const { parentPort } = await import(/* @vite-ignore */ nodeModuleId);
  return parentPort;
}

function ___waitForMsgType(target, type) {
  return new Promise((resolve) => {
    if (typeof target.on === 'function') {
      // Node: EventEmitter, data passed directly
      target.on('message', function onMsg(data) {
        if (data?.type !== type) return;
        target.off('message', onMsg);
        resolve(data);
      });
    } else {
      // Browser: DOM events, data wrapped in MessageEvent
      target.addEventListener('message', function onMsg({ data }) {
        if (data?.type !== type) return;
        target.removeEventListener('message', onMsg);
        resolve(data);
      });
    }
  });
}

___getTarget().then((target) =>
  ___waitForMsgType(target, 'wasm_bindgen_worker_init').then(
    async ({ init, receiver }) => {
      const pkg = await Promise.resolve().then(function () {
        return tfhe;
      });
      await pkg.default(init);
      target.postMessage({ type: 'wasm_bindgen_worker_ready' });
      pkg.wbg_rayon_start_worker(receiver);
    },
  ),
);

/**
 * @param {number} receiver
 */
function wbg_rayon_start_worker(receiver) {
  wasm.wbg_rayon_start_worker(receiver);
}

////////////////////////////////////////////////////////////////////////////////
// Internal wasmbindgen tools
////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////
//
// Imports:
// __wbg_get_imports
//
////////////////////////////////////////////////////////////////////////////////

function __wbg_get_imports(memory) {
    const import0 = {
        __proto__: null,
        __wbg_BigInt_65bcea251e788083: function (arg0) {
            const ret = BigInt(arg0);
            return ret;
        },
        __wbg_BigInt_ccbca793f456e582: function () {
            return handleError(function (arg0) {
                const ret = BigInt(arg0);
                return ret;
            }, arguments);
        },
        __wbg_Error_960c155d3d49e4c2: function (arg0, arg1) {
            const ret = Error(getStringFromWasm0(arg0, arg1));
            return ret;
        },
        __wbg___wbindgen_bigint_get_as_i64_3d3aba5d616c6a51: function (arg0, arg1) {
            const v = arg1;
            const ret = typeof (v) === 'bigint' ? v : undefined;
            getDataViewMemory0().setBigInt64(arg0 + 8 * 1, isLikeNone(ret) ? BigInt(0) : ret, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, !isLikeNone(ret), true);
        },
        __wbg___wbindgen_bit_and_96f696c56b4950d5: function (arg0, arg1) {
            const ret = arg0 & arg1;
            return ret;
        },
        __wbg___wbindgen_bit_or_d564c2751fdac6c9: function (arg0, arg1) {
            const ret = arg0 | arg1;
            return ret;
        },
        __wbg___wbindgen_debug_string_ab4b34d23d6778bd: function (arg0, arg1) {
            const ret = debugString(arg1);
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg___wbindgen_is_function_3baa9db1a987f47d: function (arg0) {
            const ret = typeof (arg0) === 'function';
            return ret;
        },
        __wbg___wbindgen_is_object_63322ec0cd6ea4ef: function (arg0) {
            const val = arg0;
            const ret = typeof (val) === 'object' && val !== null;
            return ret;
        },
        __wbg___wbindgen_is_string_6df3bf7ef1164ed3: function (arg0) {
            const ret = typeof (arg0) === 'string';
            return ret;
        },
        __wbg___wbindgen_is_undefined_29a43b4d42920abd: function (arg0) {
            const ret = arg0 === undefined;
            return ret;
        },
        __wbg___wbindgen_jsval_eq_d3465d8a07697228: function (arg0, arg1) {
            const ret = arg0 === arg1;
            return ret;
        },
        __wbg___wbindgen_lt_78bab382628fb48f: function (arg0, arg1) {
            const ret = arg0 < arg1;
            return ret;
        },
        __wbg___wbindgen_memory_dfa12096f400c9bd: function () {
            const ret = wasm.memory;
            return ret;
        },
        __wbg___wbindgen_module_b5e6fb95dbdb7d7e: function () {
            const ret = wasmModule;
            return ret;
        },
        __wbg___wbindgen_neg_8d39d23ef65c9fdb: function (arg0) {
            const ret = -arg0;
            return ret;
        },
        __wbg___wbindgen_shl_c7ea4173387a1669: function (arg0, arg1) {
            const ret = arg0 << arg1;
            return ret;
        },
        __wbg___wbindgen_shr_436553cbaef41a66: function (arg0, arg1) {
            const ret = arg0 >> arg1;
            return ret;
        },
        __wbg___wbindgen_string_get_7ed5322991caaec5: function (arg0, arg1) {
            const obj = arg1;
            const ret = typeof (obj) === 'string' ? obj : undefined;
            var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg___wbindgen_throw_6b64449b9b9ed33c: function (arg0, arg1) {
            throw new Error(getStringFromWasm0(arg0, arg1));
        },
        __wbg_call_a24592a6f349a97e: function () {
            return handleError(function (arg0, arg1, arg2) {
                const ret = arg0.call(arg1, arg2);
                return ret;
            }, arguments);
        },
        __wbg_crypto_38df2bab126b63dc: function (arg0) {
            const ret = arg0.crypto;
            return ret;
        },
        __wbg_error_a6fa202b58aa1cd3: function (arg0, arg1) {
            let deferred0_0;
            let deferred0_1;
            try {
                deferred0_0 = arg0;
                deferred0_1 = arg1;
                console.error(getStringFromWasm0(arg0, arg1));
            }
            finally {
                wasm.__wbindgen_free(deferred0_0, deferred0_1, 1);
            }
        },
        __wbg_getRandomValues_c44a50d8cfdaebeb: function () {
            return handleError(function (arg0, arg1) {
                arg0.getRandomValues(arg1);
            }, arguments);
        },
        __wbg_getTime_da7c55f52b71e8c6: function (arg0) {
            const ret = arg0.getTime();
            return ret;
        },
        __wbg_instanceof_Window_cc64c86c8ef9e02b: function (arg0) {
            let result;
            try {
                result = arg0 instanceof Window;
            }
            catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_length_9f1775224cf1d815: function (arg0) {
            const ret = arg0.length;
            return ret;
        },
        __wbg_msCrypto_bd5a034af96bcba6: function (arg0) {
            const ret = arg0.msCrypto;
            return ret;
        },
        __wbg_new_0_4d657201ced14de3: function () {
            const ret = new Date();
            return ret;
        },
        __wbg_new_227d7c05414eb861: function () {
            const ret = new Error();
            return ret;
        },
        __wbg_new_with_length_8c854e41ea4dae9b: function (arg0) {
            const ret = new Uint8Array(arg0 >>> 0);
            return ret;
        },
        __wbg_node_84ea875411254db1: function (arg0) {
            const ret = arg0.node;
            return ret;
        },
        __wbg_process_44c7a14e11e9f69e: function (arg0) {
            const ret = arg0.process;
            return ret;
        },
        __wbg_prototypesetcall_a6b02eb00b0f4ce2: function (arg0, arg1, arg2) {
            Uint8Array.prototype.set.call(getArrayU8FromWasm0(arg0, arg1), arg2);
        },
        __wbg_randomFillSync_6c25eac9869eb53c: function () {
            return handleError(function (arg0, arg1) {
                arg0.randomFillSync(arg1);
            }, arguments);
        },
        __wbg_require_b4edbdcf3e2a1ef0: function () {
            return handleError(function () {
                const ret = module.require;
                return ret;
            }, arguments);
        },
        __wbg_stack_3b0d974bbf31e44f: function (arg0, arg1) {
            const ret = arg1.stack;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_startWorkers_8b582d57e92bd2d4: function (arg0, arg1, arg2) {
            handleError(function () {
                throw new Error('startWorkers not supported from a worker thread');
            });
            // const ret = startWorkers(arg0, arg1, wbg_rayon_PoolBuilder.__wrap(arg2));
            // return ret;
        },
        __wbg_static_accessor_GLOBAL_8cfadc87a297ca02: function () {
            const ret = typeof global === 'undefined' ? null : global;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_static_accessor_GLOBAL_THIS_602256ae5c8f42cf: function () {
            const ret = typeof globalThis === 'undefined' ? null : globalThis;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_static_accessor_SELF_e445c1c7484aecc3: function () {
            const ret = typeof self === 'undefined' ? null : self;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_static_accessor_WINDOW_f20e8576ef1e0f17: function () {
            const ret = typeof window === 'undefined' ? null : window;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_subarray_f8ca46a25b1f5e0d: function (arg0, arg1, arg2) {
            const ret = arg0.subarray(arg1 >>> 0, arg2 >>> 0);
            return ret;
        },
        __wbg_toString_6dc1a94e0bdba378: function (arg0) {
            const ret = arg0.toString();
            return ret;
        },
        __wbg_toString_c96dc76d5547a715: function (arg0, arg1, arg2) {
            const ret = arg1.toString(arg2);
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_versions_276b2795b1c6a219: function (arg0) {
            const ret = arg0.versions;
            return ret;
        },
        __wbindgen_cast_0000000000000001: function (arg0) {
            // Cast intrinsic for `F64 -> Externref`.
            const ret = arg0;
            return ret;
        },
        __wbindgen_cast_0000000000000002: function (arg0, arg1) {
            // Cast intrinsic for `I128 -> Externref`.
            const ret = (BigInt.asUintN(64, arg0) | (arg1 << BigInt(64)));
            return ret;
        },
        __wbindgen_cast_0000000000000003: function (arg0) {
            // Cast intrinsic for `I64 -> Externref`.
            const ret = arg0;
            return ret;
        },
        __wbindgen_cast_0000000000000004: function (arg0, arg1) {
            // Cast intrinsic for `Ref(Slice(U8)) -> NamedExternref("Uint8Array")`.
            const ret = getArrayU8FromWasm0(arg0, arg1);
            return ret;
        },
        __wbindgen_cast_0000000000000005: function (arg0, arg1) {
            // Cast intrinsic for `Ref(String) -> Externref`.
            const ret = getStringFromWasm0(arg0, arg1);
            return ret;
        },
        __wbindgen_cast_0000000000000006: function (arg0, arg1) {
            // Cast intrinsic for `U128 -> Externref`.
            const ret = (BigInt.asUintN(64, arg0) | (BigInt.asUintN(64, arg1) << BigInt(64)));
            return ret;
        },
        __wbindgen_cast_0000000000000007: function (arg0) {
            // Cast intrinsic for `U64 -> Externref`.
            const ret = BigInt.asUintN(64, arg0);
            return ret;
        },
        __wbindgen_init_externref_table: function () {
            const table = wasm.__wbindgen_externrefs;
            const offset = table.grow(4);
            table.set(0, undefined);
            table.set(offset + 0, undefined);
            table.set(offset + 1, null);
            table.set(offset + 2, true);
            table.set(offset + 3, false);
        },
        memory: memory || new WebAssembly.Memory({ initial: 21, maximum: 16384, shared: true }),
    };
    return {
        __proto__: null,
        "./tfhe_bg.js": import0,
    };
}

////////////////////////////////////////////////////////////////////////////////
// addToExternrefTable0
////////////////////////////////////////////////////////////////////////////////

function addToExternrefTable0(obj) {
    const idx = wasm.__externref_table_alloc();
    wasm.__wbindgen_externrefs.set(idx, obj);
    return idx;
}

////////////////////////////////////////////////////////////////////////////////
// debugString
////////////////////////////////////////////////////////////////////////////////

function debugString(val) {
    // primitive types
    const type = typeof val;
    if (type == 'number' || type == 'boolean' || val == null) {
        return `${val}`;
    }
    if (type == 'string') {
        return `"${val}"`;
    }
    if (type == 'symbol') {
        const description = val.description;
        if (description == null) {
            return 'Symbol';
        }
        else {
            return `Symbol(${description})`;
        }
    }
    if (type == 'function') {
        const name = val.name;
        if (typeof name == 'string' && name.length > 0) {
            return `Function(${name})`;
        }
        else {
            return 'Function';
        }
    }
    // objects
    if (Array.isArray(val)) {
        const length = val.length;
        let debug = '[';
        if (length > 0) {
            debug += debugString(val[0]);
        }
        for (let i = 1; i < length; i++) {
            debug += ', ' + debugString(val[i]);
        }
        debug += ']';
        return debug;
    }
    // Test for built-in
    const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
    let className;
    if (builtInMatches && builtInMatches.length > 1) {
        className = builtInMatches[1];
    }
    else {
        // Failed to match the standard '[object ClassName]'
        return toString.call(val);
    }
    if (className == 'Object') {
        // we're a user defined class or Object
        // JSON.stringify avoids problems with cycles, and is generally much
        // easier than looping through ownProperties of `val`.
        try {
            return 'Object(' + JSON.stringify(val) + ')';
        }
        catch (_) {
            return 'Object';
        }
    }
    // errors
    if (val instanceof Error) {
        return `${val.name}: ${val.message}\n${val.stack}`;
    }
    // TODO we could test for more things here, like `Set`s and `Map`s.
    return className;
}

////////////////////////////////////////////////////////////////////////////////
// getArrayU8FromWasm0
////////////////////////////////////////////////////////////////////////////////

function getArrayU8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint8ArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
}

////////////////////////////////////////////////////////////////////////////////
// cachedDataViewMemory0
////////////////////////////////////////////////////////////////////////////////

let cachedDataViewMemory0 = null;

////////////////////////////////////////////////////////////////////////////////
// getDataViewMemory0
////////////////////////////////////////////////////////////////////////////////

function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer !== wasm.memory.buffer) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

////////////////////////////////////////////////////////////////////////////////
// getStringFromWasm0
////////////////////////////////////////////////////////////////////////////////

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return decodeText(ptr, len);
}

////////////////////////////////////////////////////////////////////////////////
// cachedUint8ArrayMemory0
////////////////////////////////////////////////////////////////////////////////

let cachedUint8ArrayMemory0 = null;

////////////////////////////////////////////////////////////////////////////////
// getUint8ArrayMemory0
////////////////////////////////////////////////////////////////////////////////

function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.buffer !== wasm.memory.buffer) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

////////////////////////////////////////////////////////////////////////////////
// handleError
////////////////////////////////////////////////////////////////////////////////

function handleError(f, args) {
    try {
        return f.apply(this, args);
    }
    catch (e) {
        const idx = addToExternrefTable0(e);
        wasm.__wbindgen_exn_store(idx);
    }
}

////////////////////////////////////////////////////////////////////////////////
// isLikeNone
////////////////////////////////////////////////////////////////////////////////

function isLikeNone(x) {
    return x === undefined || x === null;
}

////////////////////////////////////////////////////////////////////////////////
// passStringToWasm0
////////////////////////////////////////////////////////////////////////////////

function passStringToWasm0(arg, malloc, realloc) {
    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }
    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;
    const mem = getUint8ArrayMemory0();
    let offset = 0;
    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F)
            break;
        mem[ptr + offset] = code;
    }
    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
        const ret = cachedTextEncoder.encodeInto(arg, view);
        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }
    WASM_VECTOR_LEN = offset;
    return ptr;
}

////////////////////////////////////////////////////////////////////////////////
// cachedTextDecoder
////////////////////////////////////////////////////////////////////////////////

let cachedTextDecoder = (typeof TextDecoder !== 'undefined' ? new TextDecoder('utf-8', { ignoreBOM: true, fatal: true }) : undefined);

if (cachedTextDecoder)
    cachedTextDecoder.decode();

////////////////////////////////////////////////////////////////////////////////
// MAX_SAFARI_DECODE_BYTES
////////////////////////////////////////////////////////////////////////////////

const MAX_SAFARI_DECODE_BYTES = 2146435072;

////////////////////////////////////////////////////////////////////////////////
// numBytesDecoded
////////////////////////////////////////////////////////////////////////////////

let numBytesDecoded = 0;

////////////////////////////////////////////////////////////////////////////////
// decodeText
////////////////////////////////////////////////////////////////////////////////

function decodeText(ptr, len) {
    numBytesDecoded += len;
    if (numBytesDecoded >= MAX_SAFARI_DECODE_BYTES) {
        cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
        cachedTextDecoder.decode();
        numBytesDecoded = len;
    }
    return cachedTextDecoder.decode(getUint8ArrayMemory0().slice(ptr, ptr + len));
}

////////////////////////////////////////////////////////////////////////////////
// cachedTextEncoder
////////////////////////////////////////////////////////////////////////////////

const cachedTextEncoder = (typeof TextEncoder !== 'undefined' ? new TextEncoder() : undefined);

if (cachedTextEncoder) {
    cachedTextEncoder.encodeInto = function (arg, view) {
        const buf = cachedTextEncoder.encode(arg);
        view.set(buf);
        return {
            read: arg.length,
            written: buf.length
        };
    };
}

////////////////////////////////////////////////////////////////////////////////
// WASM_VECTOR_LEN is a module-level variable that stores the byte length of
// the data just written into WASM memory. It acts as an out-parameter.
////////////////////////////////////////////////////////////////////////////////

let WASM_VECTOR_LEN = 0;

////////////////////////////////////////////////////////////////////////////////
// WASM module state
////////////////////////////////////////////////////////////////////////////////

let wasmModule, wasm;

////////////////////////////////////////////////////////////////////////////////
// Init:
// __wbg_finalize_init
////////////////////////////////////////////////////////////////////////////////

function __wbg_finalize_init(instance, module, thread_stack_size) {
    wasm = instance.exports;
    wasmModule = module;
    cachedDataViewMemory0 = null;
    cachedUint8ArrayMemory0 = null;
    if (typeof thread_stack_size !== 'undefined' && (typeof thread_stack_size !== 'number' || thread_stack_size === 0 || thread_stack_size % 65536 !== 0)) {
        throw new Error('invalid stack size');
    }
    wasm.__wbindgen_start(thread_stack_size);
    return wasm;
}

////////////////////////////////////////////////////////////////////////////////
// Init:
// __wbg_load
////////////////////////////////////////////////////////////////////////////////

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);
            }
            catch (e) {
                const validResponse = module.ok && expectedResponseType(module.type);
                if (validResponse && module.headers.get('Content-Type') !== 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);
                }
                else {
                    throw e;
                }
            }
        }
        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);
    }
    else {
        const instance = await WebAssembly.instantiate(module, imports);
        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };
        }
        else {
            return instance;
        }
    }
    function expectedResponseType(type) {
        switch (type) {
            case 'basic':
            case 'cors':
            case 'default': return true;
        }
        return false;
    }
}

////////////////////////////////////////////////////////////////////////////////
// Init:
// __wbg_init
////////////////////////////////////////////////////////////////////////////////

async function __wbg_init(module_or_path, memory) {
    if (wasm !== undefined)
        return wasm;
    let thread_stack_size;
    if (module_or_path !== undefined) {
        if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
            ({ module_or_path, memory, thread_stack_size } = module_or_path);
        }
        else {
            console.warn('using deprecated parameters for the initialization function; pass a single object instead');
        }
    }
    //   if (module_or_path === undefined) {
    //     module_or_path = new URL('tfhe_bg.wasm', import.meta.url);
    //   }
    const imports = __wbg_get_imports(memory);
    //   if (
    //     typeof module_or_path === 'string' ||
    //     (typeof Request === 'function' && module_or_path instanceof Request) ||
    //     (typeof URL === 'function' && module_or_path instanceof URL)
    //   ) {
    //     module_or_path = fetch(module_or_path);
    //   }
    const { instance, module } = await __wbg_load(await module_or_path, imports);
    return __wbg_finalize_init(instance, module, thread_stack_size);
}

////////////////////////////////////////////////////////////////////////////////
//
// The 'tfhe' global object
// ========================
// Final tfhe object global declaration called by 'waitForMsgType' only
//
////////////////////////////////////////////////////////////////////////////////

var tfhe = /*#__PURE__*/ Object.freeze({
  __proto__: null,
  default: __wbg_init,
  wbg_rayon_start_worker: wbg_rayon_start_worker,
});

