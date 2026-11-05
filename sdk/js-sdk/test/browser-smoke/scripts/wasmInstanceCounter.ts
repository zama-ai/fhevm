/**
 * Counts the WebAssembly modules compiled and instantiated in this realm (the main thread), per SDK module.
 *
 * Loaded by the page before any other script. Wraps every compile and instantiate entry point of `WebAssembly`
 * without changing what it returns, so `instanceof WebAssembly.Module` checks (wasm-bindgen's) still hold.
 * Workers are separate realms and are not counted.
 */

export type WasmKind = 'tfhe' | 'tkms' | 'other';

export type WasmCounts = {
  readonly compiled: Record<WasmKind, number>;
  readonly instantiated: Record<WasmKind, number>;
};

const counts: WasmCounts = {
  compiled: { tfhe: 0, tkms: 0, other: 0 },
  instantiated: { tfhe: 0, tkms: 0, other: 0 },
};

/** A copy of the counts so far. */
export function getWasmCounts(): WasmCounts {
  return { compiled: { ...counts.compiled }, instantiated: { ...counts.instantiated } };
}

const OriginalModule = WebAssembly.Module;

/** The SDK module a compiled module belongs to, told apart by an export only it has. */
function kindOf(module: WebAssembly.Module): WasmKind {
  const names = new Set(OriginalModule.exports(module).map((e) => e.name));
  if (names.has('__wbg_tfheconfig_free')) {
    return 'tfhe';
  }
  if (names.has('__wbg_privateenckeymlkem512_free')) {
    return 'tkms';
  }
  return 'other';
}

function countCompiled(module: WebAssembly.Module): void {
  counts.compiled[kindOf(module)] += 1;
}

function countInstantiated(module: WebAssembly.Module): void {
  counts.instantiated[kindOf(module)] += 1;
}

function isInstantiatedSource(value: unknown): value is WebAssembly.WebAssemblyInstantiatedSource {
  return typeof value === 'object' && value !== null && 'module' in value && 'instance' in value;
}

const originalCompile = WebAssembly.compile.bind(WebAssembly);
const originalCompileStreaming = WebAssembly.compileStreaming.bind(WebAssembly);
const originalInstantiate = WebAssembly.instantiate.bind(WebAssembly) as (
  source: BufferSource | WebAssembly.Module,
  imports?: WebAssembly.Imports,
) => Promise<WebAssembly.Instance | WebAssembly.WebAssemblyInstantiatedSource>;
const originalInstantiateStreaming = WebAssembly.instantiateStreaming.bind(WebAssembly);

WebAssembly.compile = async (bytes: BufferSource): Promise<WebAssembly.Module> => {
  const module = await originalCompile(bytes);
  countCompiled(module);
  return module;
};

WebAssembly.compileStreaming = async (source: Response | PromiseLike<Response>): Promise<WebAssembly.Module> => {
  const module = await originalCompileStreaming(source);
  countCompiled(module);
  return module;
};

WebAssembly.instantiate = (async (source: BufferSource | WebAssembly.Module, imports?: WebAssembly.Imports) => {
  const result = await originalInstantiate(source, imports);
  if (isInstantiatedSource(result)) {
    // From bytes: one compile and one instance.
    countCompiled(result.module);
    countInstantiated(result.module);
  } else {
    countInstantiated(source as WebAssembly.Module);
  }
  return result;
}) as typeof WebAssembly.instantiate;

WebAssembly.instantiateStreaming = async (
  source: Response | PromiseLike<Response>,
  imports?: WebAssembly.Imports,
): Promise<WebAssembly.WebAssemblyInstantiatedSource> => {
  const result = await originalInstantiateStreaming(source, imports);
  countCompiled(result.module);
  countInstantiated(result.module);
  return result;
};

// Proxies, not subclasses: `instanceof` resolves through the proxy to the original constructor.
WebAssembly.Module = new Proxy(OriginalModule, {
  construct(target, args: [BufferSource]) {
    const module = new target(...args);
    countCompiled(module);
    return module;
  },
});

WebAssembly.Instance = new Proxy(WebAssembly.Instance, {
  construct(target, args: [WebAssembly.Module, WebAssembly.Imports?]) {
    const instance = new target(...args);
    countInstantiated(args[0]);
    return instance;
  },
});
