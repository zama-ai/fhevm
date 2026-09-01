import type { FhevmBase } from '../types/coreFhevmClient.js';
import type { FhevmChain } from '../types/fhevmChain.js';
import type { FhevmClientFrozenContext } from '../types/fhevmClientFrozenContext-p.js';
import {
  getFrozenContext,
  getFrozenContextPromise,
  setFrozenContext,
  setFrozenContextPromise,
} from '../runtime/CoreFhevm-p.js';
import { resolveFhevmClientFrozenContext } from './resolveFhevmClientFrozenContext-p.js';

////////////////////////////////////////////////////////////////////////////////

/**
 * Resolves the client's complete version basis **exactly once** and returns it,
 * deduping concurrent callers onto a single in-flight resolution.
 *
 * The init functions run concurrently (`init()` fans them out with `Promise.all`),
 * so `_initBase` / `_initEncrypt` / `_initDecrypt` cannot read an
 * already-resolved context synchronously — the first one to reach here starts the
 * single resolution and the rest `await` the same promise. This is the
 * synchronization point that guarantees every tier branches off **one** snapshot.
 *
 * All state lives **on the client instance** (`getFrozenContext` /
 * `getFrozenContextPromise`), never in module scope — so it is scoped to the
 * client's lifecycle and safe to use per-request in a server component.
 *
 * Lifecycle — promise while resolving, data afterwards:
 * - If the context is already stored on the client, return it immediately (a
 *   later re-run, e.g. after `extend()`, is a no-op — no promise, no re-resolve).
 * - Otherwise resolve once; on success store the context on the client
 *   (`setFrozenContext`) so subsequent reads are synchronous, and drop the
 *   transient promise. The stored data — not this promise — is the single source
 *   of truth that actions and a future `refresh()` read.
 * - The transient promise is cleared once the resolution settles either way, so a
 *   transient RPC failure does not poison later attempts (frozen-context
 *   resolution is pure chain reads with no un-resettable side effects, unlike
 *   one-shot WASM module init).
 *
 * @internal
 */
export async function ensureFrozenContext(fhevm: FhevmBase<FhevmChain>): Promise<FhevmClientFrozenContext> {
  const existing = getFrozenContext(fhevm);
  if (existing !== undefined) {
    return existing;
  }

  let pending = getFrozenContextPromise(fhevm);
  if (pending === undefined) {
    pending = resolveFhevmClientFrozenContext(fhevm)
      .then((frozenContext) => {
        setFrozenContext(fhevm, frozenContext);
        return frozenContext;
      })
      .finally(() => {
        setFrozenContextPromise(fhevm, undefined);
      });
    setFrozenContextPromise(fhevm, pending);
  }

  return pending;
}
