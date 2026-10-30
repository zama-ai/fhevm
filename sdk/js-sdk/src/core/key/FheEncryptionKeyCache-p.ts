import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import type {
  FheEncryptionKeyWasm,
  FheEncryptionKeyBytes,
  FheEncryptionKeyMetadata,
} from '../types/fheEncryptionKey.js';

////////////////////////////////////////////////////////////////////////////////

/**
 * A live mutable cache entry (placeholder).
 *
 * Callers hold a reference and observe state changes across awaits.
 * - `resolvedKind`: kind of the currently available value ("bytes" | "wasm"),
 *   or `undefined` if no value has resolved yet.
 * - `value`: the resolved data, or `undefined` while no value is available.
 * - `pendingKind`: kind of the value being fetched/converted ("bytes" | "wasm"),
 *   or `undefined` if idle.
 * - `pendingOp`: type of in-flight operation ("fetching" | "converting"),
 *   or `undefined` if idle.
 * - `ready`: the promise to await. When it resolves, `value` is populated,
 *   `resolvedKind` is updated, and `pendingKind`/`pendingOp` are `undefined`.
 */
export interface CacheEntry {
  readonly resolvedKind: 'bytes' | 'wasm' | undefined;
  readonly value: FheEncryptionKeyBytes | FheEncryptionKeyWasm | undefined;
  readonly pendingKind: 'bytes' | 'wasm' | undefined;
  readonly pendingOp: 'fetching' | 'converting' | undefined;
  readonly ready: Promise<void>;
  readonly owner: WeakRef<FhevmRuntime>;
}

export class CacheEntryImpl implements CacheEntry {
  resolvedKind: 'bytes' | 'wasm' | undefined;
  value: FheEncryptionKeyBytes | FheEncryptionKeyWasm | undefined;
  pendingKind: 'bytes' | 'wasm' | undefined;
  pendingOp: 'fetching' | 'converting' | undefined;
  metadata: FheEncryptionKeyMetadata;
  ready!: Promise<void>;
  owner!: WeakRef<FhevmRuntime>;

  /** @internal promise on the future value being resolved. Do not use externally. */
  _pending!: Promise<FheEncryptionKeyBytes | FheEncryptionKeyWasm>;
  /** @internal promise on the bytes as fetched, kept after conversion to wasm. Do not use externally. */
  _bytes!: Promise<FheEncryptionKeyBytes>;
  /** @internal debug: true if `_pending` has been chained from (e.g. by ensureWasm). */
  _pendingChained: boolean;

  constructor(owner: FhevmRuntime, metadata: FheEncryptionKeyMetadata) {
    this.owner = new WeakRef(owner);
    this.resolvedKind = undefined;
    this.value = undefined;
    this.pendingKind = undefined;
    this.pendingOp = undefined;
    this.metadata = metadata;
    this._pendingChained = false;
  }
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Asserts that two relayerUrls match.
 * Prevents mixing keys from different relayers in the same cache slot.
 */
function _assertRelayerUrlMatch(actualRelayerUrl: string, expectedRelayerUrl: string): void {
  if (actualRelayerUrl !== expectedRelayerUrl) {
    throw new Error(
      `FheEncryptionKey relayerUrl mismatch: expected "${expectedRelayerUrl}" but got "${actualRelayerUrl}". ` +
        'The FheEncryptionKey must originate from the same relayer.',
    );
  }
}

/** Debug: if resolvedKind is "wasm", there must be no pending operation. */
function _assertWasmIsTerminal(entry: CacheEntry): void {
  if (entry.resolvedKind === 'wasm' && entry.pendingKind !== undefined) {
    throw new Error("Debug: resolvedKind is 'wasm' but pendingKind is " + JSON.stringify(entry.pendingKind));
  }
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Global cache for FHE public key parameters.
 *
 * Keyed by slot key: the relayer URL, or `placeholder:<relayer URL>` for
 * placeholder keys (see {@link getFheEncryptionKeySlot}). Each entry is a
 * live mutable placeholder that callers hold a reference to and observe state
 * changes across awaits.
 */
export class FheEncryptionKeyCache {
  readonly #cache = new Map<string, CacheEntryImpl>();

  ////////////////////////////////////////////////////////////////////////////////

  get(key: string): CacheEntry | undefined {
    return this.#cache.get(key);
  }

  has(key: string): boolean {
    return this.#cache.has(key);
  }

  remove(key: string): boolean {
    const entry = this.#cache.get(key);
    if (entry !== undefined) {
      this.#invalidate(entry);
    }
    return this.#cache.delete(key);
  }

  clear(): void {
    for (const entry of this.#cache.values()) {
      this.#invalidate(entry);
    }
    this.#cache.clear();
  }

  /**
   * Empties the slot if it still holds `entry`. Unlike `remove`, the entry is
   * not invalidated: callers holding it finish with it.
   */
  evict(key: string, entry: CacheEntry): void {
    if (this.#cache.get(key) === entry) {
      this.#cache.delete(key);
    }
  }

  /** Stale-ify an entry so in-flight callbacks hit the guard and get swallowed. */
  #invalidate(entry: CacheEntryImpl): void {
    entry.ready = Promise.resolve();
    entry.pendingKind = undefined;
    entry.pendingOp = undefined;
  }

  ////////////////////////////////////////////////////////////////////////////////

  /**
   * Stores bytes in an empty slot, unchecked: callers check them first.
   * First write wins: an existing entry (resolved or in-flight) is kept.
   * For tests only (e.g. the browser smoke harness).
   */
  setBytes(owner: FhevmRuntime, key: string, relayerUrl: string, bytes: FheEncryptionKeyBytes): void {
    // Skip to avoid creating a closure over the key bytes unnecessarily.
    if (this.#cache.has(key)) {
      return;
    }
    _assertRelayerUrlMatch(bytes.metadata.relayerUrl, relayerUrl);
    // The closure over `bytes` is ephemeral: ensureBytes calls the fetcher
    // immediately, the promise resolves synchronously, and the closure becomes
    // unreachable. The entry keeps `bytes` itself (not a copy), as served.
    this.ensureBytes({
      owner,
      key,
      fetcher: () => Promise.resolve(bytes),
      metadata: bytes.metadata,
    });
  }

  /**
   * Stores bytes in the slot, replacing any entry, unchecked: callers check
   * them first. A new entry object takes the slot, so callers holding the old
   * entry finish with the old key.
   *
   * @returns the new entry.
   */
  replaceBytes(owner: FhevmRuntime, key: string, relayerUrl: string, bytes: FheEncryptionKeyBytes): CacheEntry {
    // Before emptying the slot: a mismatch leaves the cache as is.
    _assertRelayerUrlMatch(bytes.metadata.relayerUrl, relayerUrl);
    this.#cache.delete(key);
    return this.ensureBytes({
      owner,
      key,
      fetcher: () => Promise.resolve(bytes),
      metadata: bytes.metadata,
    });
  }

  ////////////////////////////////////////////////////////////////////////////////

  /**
   * Ensures the cache has an entry for the given slot key.
   *
   * **First write wins**: if an entry already exists (resolved or in-flight,
   * bytes or wasm), it is kept as-is and the fetcher is never called.
   * Only when no entry exists is the fetcher invoked to start a new fetch.
   *
   * To force a re-fetch, call `remove(key)` before `ensureBytes`.
   *
   * @returns the slot's entry, kept or new.
   */
  ensureBytes(parameters: {
    readonly owner: FhevmRuntime;
    readonly key: string;
    readonly fetcher: () => Promise<FheEncryptionKeyBytes>;
    readonly metadata: FheEncryptionKeyMetadata;
  }): CacheEntry {
    const { owner, key, fetcher, metadata } = parameters;
    const existing = this.#cache.get(key);
    if (existing !== undefined) {
      return existing;
    }

    const entry = new CacheEntryImpl(owner, metadata);
    const pendingValue = fetcher();

    entry.pendingKind = 'bytes';
    entry.pendingOp = 'fetching';
    entry._pending = pendingValue;
    entry._bytes = pendingValue;
    entry.ready = this.#makeReady(key, entry, 'bytes', pendingValue);

    this.#cache.set(key, entry);
    return entry;
  }

  ////////////////////////////////////////////////////////////////////////////////

  /**
   * Upgrade a bytes entry to wasm in-place by deserializing.
   * If already wasm (pending or resolved), returns the existing entry.
   * Chains from `_pending` so it works whether bytes are resolved or still pending.
   * Idempotent: repeated calls return the same entry.
   *
   * Upgrades `entry`, the one the caller checked, even if another entry has
   * taken the slot `key` since.
   */
  ensureWasm(parameters: {
    readonly owner: FhevmRuntime;
    readonly key: string;
    readonly entry: CacheEntry;
    readonly deserializeFn: (bytes: FheEncryptionKeyBytes) => Promise<FheEncryptionKeyWasm>;
  }): void {
    const { key, deserializeFn } = parameters;
    // Entries only come from this cache.
    const entry = parameters.entry as CacheEntryImpl;

    if (entry.resolvedKind === 'wasm' || entry.pendingKind === 'wasm') {
      return;
    }

    _assertWasmIsTerminal(entry);

    if (entry._pendingChained) {
      throw new Error('Debug: _pending was already chained from');
    }
    entry._pendingChained = true;

    const pendingValue = entry._pending.then((bytes) => {
      _assertWasmIsTerminal(entry);
      return deserializeFn(bytes as FheEncryptionKeyBytes);
    });

    entry.pendingKind = 'wasm';
    entry.pendingOp = 'converting';
    entry._pending = pendingValue;
    entry.ready = this.#makeReady(key, entry, 'wasm', pendingValue);
  }

  /**
   * The bytes of `entry`, as fetched, whether or not it still holds its slot.
   *
   * Never serializes the wasm value back to bytes: tfhe upgrades older
   * serialization versions on load, so the result could differ from the
   * bytes the relayer served.
   */
  entryBytes(entry: CacheEntry): Promise<FheEncryptionKeyBytes> {
    // Entries only come from this cache.
    return (entry as CacheEntryImpl)._bytes;
  }

  ////////////////////////////////////////////////////////////////////////////////

  /**
   * Create a `ready` promise that updates entry state on resolution,
   * self-evicts on rejection (if still current), and swallows stale errors.
   */
  #makeReady(
    key: string,
    entry: CacheEntryImpl,
    resolvedKind: 'bytes' | 'wasm',
    pendingValue: Promise<FheEncryptionKeyBytes | FheEncryptionKeyWasm>,
  ): Promise<void> {
    // `ready` is referenced inside the callbacks before this assignment completes,
    // but callbacks are closures that capture the variable binding, not the value.
    // Promise callbacks always run as microtasks (asynchronously), so `ready`
    // is guaranteed to be assigned by the time any callback executes.
    const ready: Promise<void> = pendingValue.then(
      (resolved) => {
        if (entry.ready !== ready) {
          return; // stale: a newer operation replaced us
        }
        // Verify the resolved value's metadata matches the entry's relayer.
        // Catches bugs where a fetcher/deserializer returns data for a different relayer.
        _assertRelayerUrlMatch(resolved.metadata.relayerUrl, entry.metadata.relayerUrl);
        entry.resolvedKind = resolvedKind;
        entry.value = resolved;
        entry.pendingKind = undefined;
        entry.pendingOp = undefined;
        _assertWasmIsTerminal(entry);
      },
      (err: unknown) => {
        if (entry.ready !== ready) {
          return; // stale: swallow the error
        }
        // Clean up pending state
        entry.pendingKind = undefined;
        entry.pendingOp = undefined;
        // Self-evict
        if (this.#cache.get(key) === entry) {
          this.#cache.delete(key);
        }
        entry.owner
          .deref()
          ?.config.logger?.error?.(
            `FheEncryptionKeyCache: ${resolvedKind === 'bytes' ? 'fetch' : 'conversion to wasm'} failed for key="${key}". Evicting cache entry; the next call will re-fetch.`,
            err,
          );
        throw err;
      },
    );
    // Callers get the error through `_bytes` or by awaiting `ready`: never
    // leave `ready` unhandled when no one awaits it.
    ready.catch(() => undefined);
    return ready;
  }
}

////////////////////////////////////////////////////////////////////////////////

export const globalFheEncryptionKeyCache = new FheEncryptionKeyCache();
