import type { Auth } from '../types/auth.js';
import type { FhevmChain } from '../types/fhevmChain.js';
import type { FetchFeaturesReturnType } from '../modules/relayer/types.js';
import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';

////////////////////////////////////////////////////////////////////////////////

type Context = {
  readonly runtime: FhevmRuntime;
  readonly chain: FhevmChain;
  readonly client: NonNullable<object>;
  readonly options: { readonly batchRpcCalls: boolean };
};

////////////////////////////////////////////////////////////////////////////////

/**
 * The relayer capability snapshot a client resolves **once** and then pins for its lifetime
 * (currently just `supportsRouteV3`). It is the relayer-service analogue of the frozen context's
 * version basis — but a *different category*: fetched over HTTP from the relayer, not derived from
 * chain reads, so it lives beside {@link FhevmClientFrozenContext}, never inside it.
 *
 * **Auth-independent.** The feature list is a property of the relayer *deployment*, identical for
 * every caller; an `Auth` (optional API key) is only needed to *reach* the discovery endpoint, and
 * never changes the result. This is precisely what makes a single per-client resolution correct —
 * were features per-caller, a per-client cache would be a correctness bug.
 */
export type RelayerFeatures = FetchFeaturesReturnType;

////////////////////////////////////////////////////////////////////////////////

/**
 * Resolves the relayer feature list via the relayer module's `fetchFeatures`.
 *
 * Unlike `resolveFhevmClientFrozenContext` (pure chain reads, no inputs), this is a network call to
 * the relayer whose discovery endpoint requires an `Auth`. The `auth` is threaded through purely to
 * authorize the request; the returned {@link RelayerFeatures} does not depend on it (see the type
 * doc above), so the caller is free to resolve once and reuse across callers with different auths.
 *
 * @internal
 */
export async function resolveRelayerFeatures(
  fhevm: Context,
  parameters: { readonly auth?: Auth | undefined },
): Promise<RelayerFeatures> {
  return fhevm.runtime.relayer.fetchFeatures(fhevm, { options: { auth: parameters.auth } });
}
