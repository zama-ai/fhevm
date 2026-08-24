// One wallet confirmation, as many reveals as the permit window allows.
//
// A permit is signed once and stays reusable for its whole validity window, so asking the wallet
// again for every private-balance view spends the user's attention on a signature that changes
// nothing. The cache holds one signed session per question the permit answers — the wallet, the
// chain, the ACL domain scope and the KMS route — and hands it back until the window is close
// enough to expiry that a decrypt started now could outlive it. A different wallet, a reseeded
// deployment (new mints, new KMS pair) or an expired window all miss the cache and prompt again.
//
// Only the signed session is cached — never a clear balance: what a reveal decrypts is still
// fetched and decrypted on every view.

import type { SolanaPermitSession } from '@fhevm/sdk/solana';

/** The identity a cached permit answers for: one wallet, one domain scope, one KMS route. */
export type PermitCacheKey = {
  readonly walletAddress: string;
  readonly chainId: string;
  readonly domainKey: string;
  readonly kmsContextId: string;
  readonly kmsEpochId: string;
};

/**
 * How close to expiry a permit stops being reused, in seconds. A decrypt started inside this margin
 * could still be in flight when the window closes, and would fail authorization mid-run; signing a
 * fresh permit is cheaper than explaining that failure.
 */
export const PERMIT_REUSE_SAFETY_MARGIN_SECONDS = 60n;

const cache = new Map<string, SolanaPermitSession>();

const cacheKeyOf = (key: PermitCacheKey): string =>
  [key.walletAddress, key.chainId, key.domainKey, key.kmsContextId, key.kmsEpochId].join('|');

/** Whether the permit's own signed window still covers now, with the safety margin to spare. */
export const permitSessionCoversNow = (session: SolanaPermitSession, nowSeconds: bigint): boolean => {
  const fields = session.signedPermit.fields;
  return (
    nowSeconds >= fields.startTimestamp &&
    nowSeconds + PERMIT_REUSE_SAFETY_MARGIN_SECONDS < fields.startTimestamp + fields.durationSeconds
  );
};

/**
 * Returns the cached permit session for the key, or signs a new one through `sign` and caches it.
 *
 * The validity check reads the signed permit's own fields — the window the wallet actually
 * confirmed — rather than a timestamp recorded beside it.
 */
export const permitSessionFor = async (
  key: PermitCacheKey,
  sign: () => Promise<SolanaPermitSession>,
  nowSeconds: bigint = BigInt(Math.floor(Date.now() / 1000)),
): Promise<SolanaPermitSession> => {
  const cacheKey = cacheKeyOf(key);
  const cached = cache.get(cacheKey);
  if (cached !== undefined && permitSessionCoversNow(cached, nowSeconds)) return cached;
  cache.delete(cacheKey);
  const session = await sign();
  cache.set(cacheKey, session);
  return session;
};

/** Drops every cached permit; a disconnect or a test boundary. */
export const clearPermitCache = (): void => {
  cache.clear();
};
