import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import type {
  FheEncryptionKeyBytes,
  FheEncryptionKeyMetadata,
  FheEncryptionKeyWasm,
} from '../types/fheEncryptionKey.js';
import type { TfheVersion } from '../types/moduleVersions.js';

////////////////////////////////////////////////////////////////////////////////

type AuthenticatedBytesEntry = {
  readonly identityKey: string;
  readonly promise: Promise<FheEncryptionKeyBytes>;
};

/**
 * Maximum number of ~50 MB authenticated key bundles retained globally.
 *
 * Entries are LRU-ordered by trust scope. A new identity replaces the previous
 * identity in the same scope immediately, and inserting a fifth scope evicts the
 * least-recently-used one. Eviction never cancels promises already held by callers.
 */
const MAX_AUTHENTICATED_BYTES_SCOPES = 4;

/**
 * A bounded promise cache containing authenticated serialized key material only.
 *
 * Cleartext test mode is the isolated exception: it clones and admits bytes under
 * a cleartext-only scope without cryptographic authentication. That scope cannot
 * collide with any real-encryption policy mode.
 *
 * There are deliberately no bytes/WASM state transitions here. Rejected entries
 * self-evict, rotated identities replace their predecessor, and forced refreshes
 * replace the current promise without affecting callers that already hold it.
 */
export class AuthenticatedFheEncryptionKeyBytesCache {
  readonly #entries = new Map<string, AuthenticatedBytesEntry>();

  getOrCreate(parameters: {
    readonly scopeKey: string;
    readonly identityKey: string;
    readonly metadata: FheEncryptionKeyMetadata;
    readonly ignoreCache?: boolean | undefined;
    readonly fetcher: () => Promise<FheEncryptionKeyBytes>;
    readonly onRejected?: ((error: unknown) => void) | undefined;
  }): Promise<FheEncryptionKeyBytes> {
    const { scopeKey, identityKey, metadata, fetcher, onRejected } = parameters;
    const current = this.#entries.get(scopeKey);

    if (parameters.ignoreCache !== true && current?.identityKey === identityKey) {
      this.#touch(scopeKey, current);
      return current.promise;
    }

    const promise = Promise.resolve()
      .then(fetcher)
      .then((bytes) => {
        assertFheEncryptionKeyMetadata(bytes.metadata, metadata);
        return bytes;
      });
    const entry = { identityKey, promise };

    // Replacing the scope explicitly evicts a rotated identity (or a forced
    // refresh of the same identity). Existing callers still own the old promise.
    this.#entries.delete(scopeKey);
    this.#entries.set(scopeKey, entry);
    this.#trim();

    void promise.catch((error: unknown) => {
      if (this.#entries.get(scopeKey) === entry) {
        this.#entries.delete(scopeKey);
        try {
          onRejected?.(error);
        } catch {
          // Rejection diagnostics must not create a second, unhandled rejection.
        }
      }
    });

    return promise;
  }

  clear(): void {
    this.#entries.clear();
  }

  /** Exposed only for focused cache lifecycle tests. */
  get size(): number {
    return this.#entries.size;
  }

  #touch(scopeKey: string, entry: AuthenticatedBytesEntry): void {
    this.#entries.delete(scopeKey);
    this.#entries.set(scopeKey, entry);
  }

  #trim(): void {
    while (this.#entries.size > MAX_AUTHENTICATED_BYTES_SCOPES) {
      const oldestScope = this.#entries.keys().next().value;
      if (oldestScope === undefined) {
        return;
      }
      this.#entries.delete(oldestScope);
    }
  }
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Runtime-owned native wrappers indexed weakly by both runtime and raw bytes.
 *
 * The outer WeakMap cannot keep a runtime alive. The inner WeakMap cannot keep a
 * superseded raw bundle alive, so its resolved native WASM wrapper is collectible
 * as soon as neither the byte cache nor a caller owns that bundle. TFHE versions
 * remain separate within a live runtime.
 */
export class RuntimeFheEncryptionKeyWasmCache {
  #byRuntime = new WeakMap<
    FhevmRuntime,
    Map<TfheVersion, WeakMap<FheEncryptionKeyBytes, Promise<FheEncryptionKeyWasm>>>
  >();

  getOrCreate(parameters: {
    readonly runtime: FhevmRuntime;
    readonly tfheVersion: TfheVersion;
    readonly bytes: FheEncryptionKeyBytes;
    readonly deserialize: () => Promise<FheEncryptionKeyWasm>;
  }): Promise<FheEncryptionKeyWasm> {
    const { runtime, tfheVersion, bytes, deserialize } = parameters;
    let byVersion = this.#byRuntime.get(runtime);
    if (byVersion === undefined) {
      byVersion = new Map();
      this.#byRuntime.set(runtime, byVersion);
    }

    let byBytes = byVersion.get(tfheVersion);
    if (byBytes === undefined) {
      byBytes = new WeakMap();
      byVersion.set(tfheVersion, byBytes);
    }

    const current = byBytes.get(bytes);
    if (current !== undefined) {
      return current;
    }

    const promise = Promise.resolve().then(deserialize);
    byBytes.set(bytes, promise);
    void promise.catch(() => {
      if (byBytes.get(bytes) === promise) {
        byBytes.delete(bytes);
      }
    });
    return promise;
  }

  clear(): void {
    this.#byRuntime = new WeakMap();
  }
}

////////////////////////////////////////////////////////////////////////////////

function assertFheEncryptionKeyMetadata(actual: FheEncryptionKeyMetadata, expected: FheEncryptionKeyMetadata): void {
  if (actual.relayerUrl !== expected.relayerUrl || actual.chainId !== expected.chainId) {
    throw new Error(
      `FheEncryptionKey metadata mismatch: expected chain ${expected.chainId.toString()} at "${expected.relayerUrl}" ` +
        `but got chain ${actual.chainId.toString()} at "${actual.relayerUrl}".`,
    );
  }
}

////////////////////////////////////////////////////////////////////////////////

export const globalFheEncryptionKeyCache = new AuthenticatedFheEncryptionKeyBytesCache();
export const globalFheEncryptionKeyWasmCache = new RuntimeFheEncryptionKeyWasmCache();
