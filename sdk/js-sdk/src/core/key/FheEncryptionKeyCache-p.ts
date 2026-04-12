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
function _assertRelayerUrlMatch(
  actualRelayerUrl: string,
  expectedRelayerUrl: string,
): void {
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
    throw new Error(
      "Debug: resolvedKind is 'wasm' but pendingKind is " +
        JSON.stringify(entry.pendingKind),
    );
  }
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Global cache for FHE public key parameters.
 *
 * Keyed by relayer URL. Each entry is a live mutable placeholder that callers
 * hold a reference to and observe state changes across awaits.
 */
export class FheEncryptionKeyCache {
  readonly #cache = new Map<string, CacheEntryImpl>();

  ////////////////////////////////////////////////////////////////////////////////

  get(relayerUrl: string): CacheEntry | undefined {
    return this.#cache.get(relayerUrl);
  }

  has(relayerUrl: string): boolean {
    return this.#cache.has(relayerUrl);
  }

  remove(relayerUrl: string): boolean {
    const entry = this.#cache.get(relayerUrl);
    if (entry !== undefined) {
      this.#invalidate(entry);
    }
    return this.#cache.delete(relayerUrl);
  }

  clear(): void {
    for (const entry of this.#cache.values()) {
      this.#invalidate(entry);
    }
    this.#cache.clear();
  }

  /** Stale-ify an entry so in-flight callbacks hit the guard and get swallowed. */
  #invalidate(entry: CacheEntryImpl): void {
    entry.ready = Promise.resolve();
    entry.pendingKind = undefined;
    entry.pendingOp = undefined;
  }

  ////////////////////////////////////////////////////////////////////////////////

  setBytes(
    owner: FhevmRuntime,
    relayerUrl: string,
    bytes: FheEncryptionKeyBytes,
  ): void {
    // First write wins: if an entry already exists (resolved or in-flight),
    // skip to avoid creating a closure over the 50MB bytes unnecessarily.
    if (this.#cache.has(relayerUrl)) {
      return;
    }
    _assertRelayerUrlMatch(bytes.metadata.relayerUrl, relayerUrl);
    // The closure over `bytes` is ephemeral: ensureBytes calls the fetcher
    // immediately, the promise resolves synchronously, and the closure becomes
    // unreachable. No extra copy of the 50MB data is retained.
    this.ensureBytes({
      owner,
      relayerUrl,
      fetcher: () => Promise.resolve(bytes),
      metadata: bytes.metadata,
    });
  }

  ////////////////////////////////////////////////////////////////////////////////

  /**
   * Ensures the cache has an entry for the given relayer URL.
   *
   * **First write wins**: if an entry already exists (resolved or in-flight,
   * bytes or wasm), it is kept as-is and the fetcher is never called.
   * Only when no entry exists is the fetcher invoked to start a new fetch.
   *
   * To force a re-fetch, call `remove(relayerUrl)` before `ensureBytes`.
   */
  ensureBytes(parameters: {
    readonly owner: FhevmRuntime;
    readonly relayerUrl: string;
    readonly fetcher: () => Promise<FheEncryptionKeyBytes>;
    readonly metadata: FheEncryptionKeyMetadata;
  }): void {
    const { owner, relayerUrl, fetcher, metadata } = parameters;
    if (this.#cache.has(relayerUrl)) {
      return;
    }

    const entry = new CacheEntryImpl(owner, metadata);
    const pendingValue = fetcher();

    entry.pendingKind = 'bytes';
    entry.pendingOp = 'fetching';
    entry._pending = pendingValue;
    entry.ready = this.#makeReady(relayerUrl, entry, 'bytes', pendingValue);

    this.#cache.set(relayerUrl, entry);
  }

  ////////////////////////////////////////////////////////////////////////////////

  /**
   * Upgrade a bytes entry to wasm in-place by deserializing.
   * If already wasm (pending or resolved), returns the existing entry.
   * Chains from `_pending` so it works whether bytes are resolved or still pending.
   * Idempotent: repeated calls return the same entry.
   */
  ensureWasm(parameters: {
    readonly owner: FhevmRuntime;
    readonly relayerUrl: string;
    readonly deserializeFn: (
      bytes: FheEncryptionKeyBytes,
    ) => Promise<FheEncryptionKeyWasm>;
  }): void {
    const { relayerUrl, deserializeFn } = parameters;

    const entry = this.#cache.get(relayerUrl);
    if (entry === undefined) {
      return;
    }

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
    entry.ready = this.#makeReady(relayerUrl, entry, 'wasm', pendingValue);
  }

  /**
   * Get bytes from the cache, converting from wasm if necessary.
   *
   * - No entry → `undefined`
   * - Resolved bytes → return immediately
   * - Resolved wasm → serialize to bytes and return
   * - Pending bytes → await, return resolved bytes
   * - Pending wasm → await, serialize to bytes and return
   */
  async resolveBytes(parameters: {
    readonly relayerUrl: string;
    readonly serializeFn?:
      | ((parameters: FheEncryptionKeyWasm) => Promise<FheEncryptionKeyBytes>)
      | undefined;
  }): Promise<FheEncryptionKeyBytes | undefined> {
    const { relayerUrl, serializeFn } = parameters;

    const entry = this.#cache.get(relayerUrl);
    if (entry === undefined) {
      return undefined;
    }

    // Bytes already available — return immediately, don't wait for pending wasm
    if (entry.resolvedKind === 'bytes') {
      return entry.value as FheEncryptionKeyBytes;
    }

    // Wasm already available — serialize to bytes
    if (entry.resolvedKind === 'wasm') {
      if (serializeFn === undefined) {
        throw new Error(
          'Cannot convert wasm to bytes: serialize function not provided',
        );
      }
      return serializeFn(entry.value as FheEncryptionKeyWasm);
    }

    if (entry.pendingKind === undefined) {
      return undefined;
    }

    await entry.ready;

    // Re-read after potential await — entry is mutable across await boundaries.
    // Cast needed: TS narrows out "bytes" from the early return above,
    // but the entry can become "bytes" during the await.
    const kind = entry.resolvedKind as 'bytes' | 'wasm' | undefined;

    if (kind === 'bytes') {
      return entry.value as FheEncryptionKeyBytes;
    }

    if (kind === 'wasm') {
      if (serializeFn === undefined) {
        throw new Error(
          'Cannot convert wasm to bytes: serialize function not provided',
        );
      }
      return serializeFn(entry.value as FheEncryptionKeyWasm);
    }

    return undefined;
  }

  ////////////////////////////////////////////////////////////////////////////////

  /**
   * Create a `ready` promise that updates entry state on resolution,
   * self-evicts on rejection (if still current), and swallows stale errors.
   */
  #makeReady(
    relayerUrl: string,
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
        // Verify the resolved value's metadata matches the cache key.
        // Catches bugs where a fetcher/deserializer returns data for a different relayer.
        _assertRelayerUrlMatch(resolved.metadata.relayerUrl, relayerUrl);
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
        if (this.#cache.get(relayerUrl) === entry) {
          this.#cache.delete(relayerUrl);
        }
        throw err;
      },
    );
    return ready;
  }
}

////////////////////////////////////////////////////////////////////////////////

export const globalFheEncryptionKeyCache = new FheEncryptionKeyCache();
