import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import type { FheEncryptionKeyWasm, FheEncryptionKeyBytes } from '../types/fheEncryptionKey.js';

////////////////////////////////////////////////////////////////////////////////

type FailureStage = 'fetch' | 'conversion to wasm';

/**
 * One key of the cache. Never changes once created: `bytes` is set at
 * creation, the wasm key is deserialized from it at most once. A caller
 * holding an entry gets its result, whatever happens to the cache slot.
 *
 * A failed entry removes itself from its slot, only if it still holds it.
 */
export class FheEncryptionKeyEntry {
  /**
   * The bytes, as fetched or provided. Kept once the wasm key is
   * deserialized: a re-serialization can re-encode them (tfhe upgrades old
   * serialization versions), and no longer be the bytes the relayer served.
   */
  readonly bytes: Promise<FheEncryptionKeyBytes>;
  readonly #onFailure: (stage: FailureStage, error: unknown) => void;
  #wasm: Promise<FheEncryptionKeyWasm> | undefined;

  constructor(bytes: Promise<FheEncryptionKeyBytes>, onFailure: (stage: FailureStage, error: unknown) => void) {
    this.bytes = bytes;
    this.#onFailure = onFailure;
    // Also marks `bytes` as handled: callers get the error when they await it.
    bytes.catch((error: unknown) => {
      onFailure('fetch', error);
    });
  }

  /** The wasm key, deserialized once by the first caller's `deserialize`. */
  wasm(deserialize: (bytes: FheEncryptionKeyBytes) => Promise<FheEncryptionKeyWasm>): Promise<FheEncryptionKeyWasm> {
    this.#wasm ??= this.bytes.then(async (bytes) => {
      try {
        return await deserialize(bytes);
      } catch (error) {
        this.#onFailure('conversion to wasm', error);
        throw error;
      }
    });
    return this.#wasm;
  }
}

////////////////////////////////////////////////////////////////////////////////

type CreateEntryParameters = {
  readonly owner: FhevmRuntime;
  readonly key: string;
  readonly relayerUrl: string;
  readonly fill: () => Promise<FheEncryptionKeyBytes>;
};

/**
 * Cache for FHE public key parameters, keyed by relayer URL: one
 * {@link FheEncryptionKeyEntry} per slot. A slot is filled once and only ever
 * emptied (`remove`, or a failed entry); the entries themselves never change.
 */
export class FheEncryptionKeyCache {
  readonly #cache = new Map<string, FheEncryptionKeyEntry>();

  get(key: string): FheEncryptionKeyEntry | undefined {
    return this.#cache.get(key);
  }

  has(key: string): boolean {
    return this.#cache.has(key);
  }

  /** Empties the slot. Callers holding its entry still get its result. */
  remove(key: string): boolean {
    return this.#cache.delete(key);
  }

  clear(): void {
    this.#cache.clear();
  }

  /**
   * The slot's entry, or a new one filled by `fill` when the slot is empty
   * (first write wins: `fill` is never called when an entry exists). To
   * force a re-fetch, `remove(key)` the entry first.
   */
  getOrCreate(parameters: CreateEntryParameters): FheEncryptionKeyEntry {
    const entry = this.#cache.get(parameters.key);
    if (entry !== undefined) {
      return entry;
    }
    const created = this.#create(parameters);
    this.#cache.set(parameters.key, created);
    return created;
  }

  /**
   * Seeds the relayer URL's slot with caller-provided bytes
   * (`options.fheEncryptionKey`). No-op if the slot holds an entry.
   */
  setBytes(owner: FhevmRuntime, relayerUrl: string, bytes: FheEncryptionKeyBytes): void {
    this.getOrCreate({ owner, key: relayerUrl, relayerUrl, fill: () => Promise.resolve(bytes) });
  }

  #create(parameters: CreateEntryParameters): FheEncryptionKeyEntry {
    const { key, relayerUrl, fill } = parameters;
    // Logging only: the cache must not keep a runtime alive.
    const owner = new WeakRef(parameters.owner);
    const entry: FheEncryptionKeyEntry = new FheEncryptionKeyEntry(
      fill().then((bytes) => {
        // Catches a fetcher returning another relayer's key for this slot.
        _assertRelayerUrlMatch(bytes.metadata.relayerUrl, relayerUrl);
        return bytes;
      }),
      (stage, error) => {
        if (this.#cache.get(key) === entry) {
          this.#cache.delete(key);
        }
        owner
          .deref()
          ?.config.logger?.error?.(
            `FheEncryptionKeyCache: ${stage} failed for key="${key}". Evicting cache entry; the next call will re-fetch.`,
            error,
          );
      },
    );
    return entry;
  }
}

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

////////////////////////////////////////////////////////////////////////////////

export const globalFheEncryptionKeyCache = new FheEncryptionKeyCache();
