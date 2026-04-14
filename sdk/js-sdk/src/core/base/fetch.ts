import { normalizeBytes } from './bytes.js';
import { abortableSleep } from './timeout.js';

/**
 * Extracts the response body as a Uint8Array.
 *
 * Uses `Response.bytes()` when available, falling back to `Response.arrayBuffer()`
 * for compatibility. The `bytes()` method is a newer addition to the Fetch API
 * and may not be supported in all environments. (older browsers, some JS runtimes, or polyfills).
 *
 * @see https://developer.mozilla.org/en-US/docs/Web/API/Response/bytes
 */
export async function getResponseBytes(response: Response): Promise<Uint8Array> {
  const bytes: Uint8Array =
    typeof response.bytes === 'function'
      ? normalizeBytes(await response.bytes())
      : normalizeBytes(await response.arrayBuffer());

  return bytes;
}

/*
  Nodejs Fetch:
  =============
  https://github.com/nodejs/undici/blob/8fd6c43c65952ebd2557964a726716879dea5506/lib/web/fetch/index.js#L135

  TypeError:
    -  webidl.argumentLengthCheck(arguments, 1, 'globalThis.fetch')
  
  Error:
    - new Request(input, init)  
      TypeError: 
        - throw new TypeError('Failed to parse URL from ' + input, { cause: err })
        - throw new TypeError(
          'Request cannot be constructed from a URL that includes credentials: ' +
            input
        )
        - throw new TypeError(`'window' option '${window}' must be null`)
        - throw new TypeError(`Referrer "${referrer}" is not a valid URL.`, { cause: err })
        - ...

  AbortError:
    - Abort signal

  TypeError (Nodejs undici):

    // See https://github.com/nodejs/undici/blob/8fd6c43c65952ebd2557964a726716879dea5506/lib/web/fetch/index.js#L237
    if (response.type === 'error') {
      p.reject(new TypeError('fetch failed', { cause: response.error }))
      return
    }

    "fetch failed" - general network failure
      - ECONNREFUSED (net.connect() / tls.connect())
      - ENOTFOUND (net.connect() / tls.connect())
      - UND_ERR_xxx (undici errors)

    Undici error is accessible via: `TypeError.cause`
*/

interface FetchErrorInfo {
  name: string;
  message: string;
  props: Record<string, string | number>;
}

/**
 * Extracts structured error information from a fetch error's cause chain.
 *
 * Node.js fetch (undici) wraps network errors in a TypeError with a `cause`
 * property pointing to the underlying error. This function traverses the
 * cause chain and extracts:
 * - `name`: The error name (e.g., "TypeError", "Error")
 * - `message`: The error message
 * - `props`: All string/number properties (e.g., code, errno, syscall, hostname)
 *
 * Uses duck typing (checks for `message` property) rather than `instanceof`
 * to handle cross-realm errors.
 *
 * @example
 * // For a DNS lookup failure:
 * // TypeError: fetch failed
 * //   cause: Error: getaddrinfo ENOTFOUND example.com
 * //          { code: 'ENOTFOUND', errno: -3008, syscall: 'getaddrinfo', hostname: 'example.com' }
 * //
 * // Returns:
 * // [
 * //   {
 * //     name: 'TypeError',
 * //     message: 'fetch failed',
 * //     props: {}
 * //   },
 * //   {
 * //     name: 'Error',
 * //     message: 'getaddrinfo ENOTFOUND example.com',
 * //     props: {
 * //       code: 'ENOTFOUND',
 * //       errno: -3008,
 * //       syscall: 'getaddrinfo',
 * //       hostname: 'example.com'
 * //     }
 * //   }
 * // ]
 */
const FETCH_ERROR_SKIP_PROPS = new Set(['message', 'cause', 'stack', 'name']);

function getFetchErrorInfo(error: unknown): FetchErrorInfo[] {
  const errors: FetchErrorInfo[] = [];
  let current: unknown = error;

  while (current !== null && typeof current === 'object') {
    const obj = current as Record<string, unknown>;

    if (typeof obj.message !== 'string') {
      break;
    }

    const name = typeof obj.name === 'string' ? obj.name : 'Error';
    const props: Record<string, string | number> = {};

    for (const key of Object.keys(obj)) {
      if (FETCH_ERROR_SKIP_PROPS.has(key)) continue;

      const value = obj[key];
      if (typeof value === 'string' || typeof value === 'number') {
        props[key] = value;
      }
    }

    errors.push({ name, message: obj.message, props });
    current = obj.cause;
  }

  return errors;
}

/**
 * Formats a fetch error into an array of human-readable messages.
 *
 * Traverses the error's cause chain and formats each error with its message
 * and properties. The root error uses standard "Name: message" format, while
 * nested causes are prefixed with "Cause: " and include name in the props.
 *
 * @param error - The error to format (typically from a failed fetch call)
 * @returns An array of formatted error messages, one per error in the cause chain
 *
 * @example
 * // For a DNS lookup failure (ENOTFOUND):
 * // Input error structure:
 * //   TypeError: fetch failed
 * //     cause: Error: getaddrinfo ENOTFOUND api.example.com
 * //            { code: 'ENOTFOUND', errno: -3008, syscall: 'getaddrinfo', hostname: 'api.example.com' }
 * //
 * // Output:
 * // [
 * //   "TypeError: fetch failed",
 * //   "Cause: getaddrinfo ENOTFOUND api.example.com [name=Error, code=ENOTFOUND, errno=-3008, syscall=getaddrinfo, hostname=api.example.com]"
 * // ]
 *
 * @example
 * // For a connection refused error (ECONNREFUSED):
 * // Output:
 * // [
 * //   "TypeError: fetch failed",
 * //   "Cause: connect ECONNREFUSED 127.0.0.1:3000 [name=Error, code=ECONNREFUSED, errno=-61, syscall=connect, address=127.0.0.1, port=3000]"
 * // ]
 */
export function formatFetchErrorMetaMessages(error: unknown): string[] {
  const infos = getFetchErrorInfo(error);
  if (infos.length === 0) {
    return [String(error)];
  }

  return infos.map((info, index) => {
    const isRoot = index === 0;
    const propEntries = Object.entries(info.props);

    if (isRoot) {
      // Root error: "TypeError: fetch failed [props...]" or "TypeError: fetch failed"
      const propsStr = propEntries.length > 0 ? ` [${propEntries.map(([k, v]) => `${k}=${v}`).join(', ')}]` : '';
      return `${info.name}: ${info.message}${propsStr}`;
    } else {
      // Cause errors: "Cause: message [name=Error, props...]"
      const allProps: Array<[string, string | number]> = [['name', info.name], ...propEntries];
      const propsStr = ` [${allProps.map(([k, v]) => `${k}=${v}`).join(', ')}]`;
      return `Cause: ${info.message}${propsStr}`;
    }
  });
}

export type FetchWithRetryParameters = {
  readonly init?: RequestInit | undefined;
  readonly retries?: number | undefined;
  readonly retryDelayMs?: number | undefined;
};

/**
 * Fetches a URL with automatic retry on network failures.
 *
 * Retries are triggered only for network-level errors (e.g., ECONNREFUSED, ENOTFOUND, UND_ERR_xxx (undici errors)
 * connection timeouts). HTTP error responses (4xx, 5xx) are NOT retried - the response
 * is returned as-is for the caller to handle.
 *
 * The operation is abortable via `init.signal`. If the signal is aborted, an AbortError
 * is thrown immediately without further retries.
 *
 * @param url - The URL to fetch
 * @param init - Optional fetch init options (method, headers, body, etc.)
 * @param retries - Number of retry attempts on network failure (default: 3)
 * @param retryDelayMs - Delay in milliseconds between retries (default: 1000)
 * @returns The fetch Response
 * @throws The last network error if all retries are exhausted
 * @throws {Error} An error with name 'AbortError' if the signal is aborted
 */
export async function fetchWithRetry(
  args: {
    url: string;
  } & FetchWithRetryParameters,
): Promise<Response> {
  let lastError: unknown;

  const retries = args.retries ?? 3;
  const retryDelayMs = args.retryDelayMs ?? 1000;
  const { url, init } = args;

  for (let attempt = 0; attempt <= retries; attempt++) {
    // Check if already aborted before fetching
    init?.signal?.throwIfAborted();

    try {
      return await fetch(url, init);
    } catch (error) {
      // AbortError should not be retried - propagate immediately
      if ((error as { name: string }).name === 'AbortError') {
        throw error;
      }

      lastError = error;

      if (attempt < retries) {
        // Abortable delay between retries
        await abortableSleep(retryDelayMs, init?.signal ?? undefined);
      }
    }
  }

  throw lastError;
}

/**
 * Smoke-tests whether `fetch("data:...base64,...")` works and returns correct bytes.
 *
 * This is critical because {@link isomorphicCompileWasmFromBase64} relies on
 * `fetch("data:application/octet-stream;base64,...")` to decode a base64 WASM
 * binary into an `ArrayBuffer` before passing it to `WebAssembly.compile`.
 * If any step in this pipeline fails, WASM compilation will silently produce
 * corrupt bytes or throw.
 *
 * Known failure cases:
 * - CSP `connect-src` without `data:` blocks data URL fetches
 * - Sandboxed iframes without `allow-same-origin`
 * - Older Node.js versions (< 18.13) with limited data URL fetch support
 *
 * When this returns `false`, callers should fall back to `atob` decoding
 * or URL-based WASM loading via {@link isomorphicCompileWasm}.
 */
let _isDataUrlFetchSupportedPromise: Promise<boolean> | undefined;
export function isDataUrlFetchSupported(): Promise<boolean> {
  _isDataUrlFetchSupportedPromise ??= _isDataUrlFetchSupported();
  return _isDataUrlFetchSupportedPromise;
}

async function _isDataUrlFetchSupported(): Promise<boolean> {
  try {
    const res = await fetch('data:text/plain;base64,YQ==');
    if (!res.ok) {
      return false;
    }
    const buf = await res.arrayBuffer();
    return buf.byteLength === 1 && new Uint8Array(buf)[0] === 0x61; // 'a'
  } catch {
    return false;
  }
}
