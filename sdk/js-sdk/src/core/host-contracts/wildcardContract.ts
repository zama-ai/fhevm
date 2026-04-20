/**
 * Wildcard address used with `ACL.delegateForUserDecryption`.
 * Passing this as `contractAddress` grants the delegate decryption rights
 * for every handle the delegator owns, regardless of which contract the
 * handle is associated with.
 *
 * This is `address(type(uint160).max)` — the maximum representable Ethereum
 * address. No real contract can ever be deployed here, so there is no
 * ambiguity with a legitimate contract.
 *
 * @see RFC-017: Wildcard Delegation via Wildcard Address
 */
export const WILDCARD_CONTRACT = '0xFFfFfFffFFfffFFfFFfFFFFFffFFFffffFfFFFfF' as const;

export type WildcardContractAddress = typeof WILDCARD_CONTRACT;

export function isWildcardContract(address: string): boolean {
  return address.toLowerCase() === WILDCARD_CONTRACT.toLowerCase();
}
