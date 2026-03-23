import type { ChecksummedAddress, UintBigInt } from "../types/primitives.js";
import type { TrustedClient } from "../modules/ethereum/types.js";
import type { FhevmRuntime } from "../types/coreFhevmRuntime.js";
import { KmsVerifierPartialAbi } from "../host-contracts/abi/kmsVerifier.js";
import { asChecksummedAddress } from "../base/address.js";

/**
 * Per-context signer cache and current context ID fetcher.
 *
 * **Lifecycle:** Created eagerly by the runtime (lightweight — stores
 * address + runtime refs only). No RPC calls until `getSignersForContext()`
 * or `getCurrentContextId()` is first called.
 *
 * **Signer cache (no TTL):** Signer sets are immutable per context on-chain.
 * Once fetched, they are cached permanently. If this assumption is ever violated,
 * add a TTL parameter to `KmsContextCache.create()`.
 *
 * **Context ID cache (TTL):** The current context ID changes on context switch.
 * Cached with a short TTL (default 5s) to balance RPC cost vs. staleness.
 *
 * **Cache survives provider reconnections and chain reorgs.** This is correct
 * given the signer-immutability assumption.
 *
 * @internal Not exported from the public SDK API.
 */

const CONTEXT_ID_RETRY_DELAY_MS = 200;

export class KmsContextCache {
  readonly #runtime: FhevmRuntime;
  readonly #kmsContractAddress: ChecksummedAddress;
  readonly #hostPublicClient: TrustedClient;

  // Signer cache: contextId → Promise<ChecksummedAddress[]>
  // Stores in-flight promises for concurrent dedup. On success, the resolved
  // promise is kept (no TTL — signer sets are immutable per context).
  // On rejection, the entry is removed so subsequent calls can retry.
  readonly #signerCache = new Map<bigint, Promise<ChecksummedAddress[]>>();

  // Context ID cache
  #currentContextIdPromise: Promise<UintBigInt> | null = null;
  #cachedContextId: UintBigInt | null = null;
  #cachedContextIdTimestamp = 0;
  readonly #contextIdTtlMs: number;

  private constructor(parameters: {
    runtime: FhevmRuntime;
    kmsContractAddress: ChecksummedAddress;
    hostPublicClient: TrustedClient;
    contextIdTtlMs?: number;
  }) {
    this.#runtime = parameters.runtime;
    this.#kmsContractAddress = parameters.kmsContractAddress;
    this.#hostPublicClient = parameters.hostPublicClient;
    this.#contextIdTtlMs = parameters.contextIdTtlMs ?? 5000;
  }

  /**
   * Factory — sole creation path.
   */
  public static create(parameters: {
    runtime: FhevmRuntime;
    kmsContractAddress: ChecksummedAddress;
    hostPublicClient: TrustedClient;
    contextIdTtlMs?: number;
  }): KmsContextCache {
    return new KmsContextCache(parameters);
  }

  /**
   * Response side — fetches or returns cached signers for a specific context.
   *
   * `contextId` is `bigint` (`uint256`) matching the contract's internal type.
   * Throws on RPC failure — callers must NOT silently swallow errors.
   *
   * Concurrent calls for the same context ID share one RPC call (dedup).
   * On rejection, the entry is evicted so subsequent calls can retry.
   */
  public async getSignersForContext(
    contextId: UintBigInt,
  ): Promise<ChecksummedAddress[]> {
    const cached = this.#signerCache.get(contextId);
    if (cached !== undefined) {
      return cached;
    }

    const promise = this.#fetchSignersForContext(contextId);
    this.#signerCache.set(contextId, promise);

    // On rejection, evict so the next caller can retry
    promise.catch(() => {
      if (this.#signerCache.get(contextId) === promise) {
        this.#signerCache.delete(contextId);
      }
    });

    return promise;
  }

  async #fetchSignersForContext(
    contextId: UintBigInt,
  ): Promise<ChecksummedAddress[]> {
    let signers: unknown;
    try {
      signers = await this.#runtime.ethereum.readContract(
        this.#hostPublicClient,
        {
          address: this.#kmsContractAddress,
          abi: KmsVerifierPartialAbi,
          functionName: "getSignersForKmsContext",
          args: [contextId],
        },
      );
    } catch (error) {
      throw new Error(`Failed to fetch signers for KMS context ${contextId}`, {
        cause: error,
      });
    }

    if (!Array.isArray(signers)) {
      throw new Error(
        `Invalid signers response for KMS context ${contextId}: expected array`,
      );
    }

    const checksummedSigners: ChecksummedAddress[] = [];
    for (const signer of signers) {
      if (typeof signer !== "string") {
        throw new Error(
          `Invalid signer address for KMS context ${contextId}: expected string`,
        );
      }
      checksummedSigners.push(asChecksummedAddress(signer));
    }

    return checksummedSigners;
  }

  /**
   * Request side — returns the current KMS context ID for building extraData.
   *
   * Cached with TTL; on RPC failure with valid stale cache, returns stale value.
   * On RPC failure with no cache, retries once (200ms delay) then throws.
   *
   * If an in-flight promise exists, concurrent callers always join it regardless
   * of TTL state. TTL only governs whether a *new* fetch is initiated when
   * there is no in-flight promise.
   */
  public async getCurrentContextId(): Promise<UintBigInt> {
    // If there's an in-flight promise, join it (dedup)
    if (this.#currentContextIdPromise !== null) {
      return this.#currentContextIdPromise;
    }

    // Check TTL — return cached value if fresh
    if (
      this.#cachedContextId !== null &&
      Date.now() - this.#cachedContextIdTimestamp < this.#contextIdTtlMs
    ) {
      return this.#cachedContextId;
    }

    // Initiate a new fetch with retry logic inside the deduped promise
    const promise = this.#fetchCurrentContextIdWithRetry();
    this.#currentContextIdPromise = promise;

    // Clean up in-flight promise and update cache on settle.
    // Use .then(onFulfilled, onRejected) to avoid creating an unhandled
    // rejected promise (which .then().catch() would do on rejection).
    promise.then(
      (contextId) => {
        if (this.#currentContextIdPromise === promise) {
          this.#currentContextIdPromise = null;
        }
        this.#cachedContextId = contextId;
        this.#cachedContextIdTimestamp = Date.now();
      },
      () => {
        if (this.#currentContextIdPromise === promise) {
          this.#currentContextIdPromise = null;
        }
      },
    );

    return promise;
  }

  async #fetchCurrentContextIdWithRetry(): Promise<UintBigInt> {
    try {
      const result = await this.#runtime.ethereum.readContract(
        this.#hostPublicClient,
        {
          address: this.#kmsContractAddress,
          abi: KmsVerifierPartialAbi,
          functionName: "getCurrentKmsContextId",
          args: [],
        },
      );

      if (typeof result !== "bigint") {
        throw new Error(
          `Invalid context ID response: expected bigint, got ${typeof result}`,
        );
      }

      return result as UintBigInt;
    } catch (firstError) {
      // On RPC failure with valid stale cached value, return stale value
      if (this.#cachedContextId !== null) {
        return this.#cachedContextId;
      }

      // Cold start — retry once after delay
      await new Promise((resolve) =>
        setTimeout(resolve, CONTEXT_ID_RETRY_DELAY_MS),
      );

      try {
        const result = await this.#runtime.ethereum.readContract(
          this.#hostPublicClient,
          {
            address: this.#kmsContractAddress,
            abi: KmsVerifierPartialAbi,
            functionName: "getCurrentKmsContextId",
            args: [],
          },
        );

        if (typeof result !== "bigint") {
          throw new Error(
            `Invalid context ID response: expected bigint, got ${typeof result}`,
          );
        }

        return result as UintBigInt;
      } catch (_secondError) {
        throw new Error(
          `Failed to fetch current KMS context ID (both attempts failed)`,
          { cause: firstError },
        );
      }
    }
  }
}
