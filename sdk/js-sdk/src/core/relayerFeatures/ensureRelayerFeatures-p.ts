import type { Auth } from '../types/auth.js';
import type { FhevmChain } from '../types/fhevmChain.js';
import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import {
  getRelayerFeatures,
  getRelayerFeaturesPromise,
  setRelayerFeatures,
  setRelayerFeaturesPromise,
} from '../runtime/CoreFhevm-p.js';
import { resolveRelayerFeatures, type RelayerFeatures } from './resolveRelayerFeatures-p.js';

////////////////////////////////////////////////////////////////////////////////

type Context = {
  readonly runtime: FhevmRuntime;
  readonly chain: FhevmChain;
  readonly client: NonNullable<object>;
  readonly options: { readonly batchRpcCalls: boolean };
};

////////////////////////////////////////////////////////////////////////////////

/**
 * Resolves the client's relayer feature list **exactly once** and returns it, deduping concurrent
 * callers onto a single in-flight fetch. The relayer-service sibling of `ensureFrozenContext`.
 *
 * ### Why this lives beside the frozen context, not inside it
 *
 * The frozen context is the *version basis*, resolved at `init()` from pure chain reads with **no
 * inputs**. Relayer features are a *service capability*, fetched over HTTP and — critically —
 * requiring an `Auth` that only arrives with a public action call, never at `init()`. So they
 * cannot share the frozen context's resolution point, and they are resolved **lazily** here: at the
 * first relayer action that carries an `Auth`.
 *
 * ### Auth is used only on the first resolution
 *
 * The result is Auth-independent (a deployment capability — see {@link RelayerFeatures}). The first
 * caller's `auth` authorizes the one discovery request; the result is then pinned on the client and
 * every later caller — regardless of its own `auth` — reads the same snapshot. This is what keeps a
 * single per-client cache correct while avoiding dynamic drift.
 *
 * ### Lifecycle — promise while resolving, data afterwards (mirrors `ensureFrozenContext`)
 *
 * - Already stored on the client → return it immediately (no re-fetch).
 * - Otherwise resolve once; on success store it (`setRelayerFeatures`) so later reads are
 *   synchronous, and drop the transient promise.
 * - The transient promise is cleared once resolution settles either way (`.finally`), so a first
 *   attempt that fails — e.g. an auth-less call to an auth-required endpoint — does not poison later
 *   auth'd calls: the next caller retries, and the first success wins.
 *
 * All state lives on the client instance (`getRelayerFeatures` / `getRelayerFeaturesPromise`), so it
 * is scoped to the client's lifecycle and safe to use per-request in a server component.
 *
 * NOTE: this depends on the client-instance accessors `getRelayerFeatures` / `setRelayerFeatures` /
 * `getRelayerFeaturesPromise` / `setRelayerFeaturesPromise` being added to `CoreFhevm-p.ts`, exactly
 * parallel to the frozen-context slot (`#frozenContext` / `#frozenContextPromise` + their symbols).
 * Wiring those into `CoreFhevm-p.ts` is intentionally out of scope of this folder.
 *
 * @internal
 */
export async function ensureRelayerFeatures(
  fhevm: Context,
  parameters: { readonly auth?: Auth | undefined },
): Promise<RelayerFeatures> {
  const existing = getRelayerFeatures(fhevm);
  if (existing !== undefined) {
    return existing;
  }

  let pending = getRelayerFeaturesPromise(fhevm);
  if (pending === undefined) {
    pending = resolveRelayerFeatures(fhevm, parameters)
      .then((relayerFeatures) => {
        setRelayerFeatures(fhevm, relayerFeatures);
        return relayerFeatures;
      })
      .finally(() => {
        setRelayerFeaturesPromise(fhevm, undefined);
      });
    setRelayerFeaturesPromise(fhevm, pending);
  }

  return pending;
}
