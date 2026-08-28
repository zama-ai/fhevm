// The interfaces an `*EthereumLib.ts` adapter implements. THIS FILE IS THE SOURCE OF TRUTH for them.
//
// Declared here rather than in the package so an adapter is self-contained: copy an adapter and this
// file into your project and nothing else is needed. sdk/scripts/sync-vendored-ts.ts copies it to every
// destination in vendored/manifest.json — including each generation pkg/ts/types/, whose public.ts
// re-exports from the copy rather than declaring these types itself. So there is one definition, and
// the gate proves every copy is byte-identical to it.

export type EncodeCallParameters = {
  readonly abi: readonly unknown[];
  readonly functionName: string;
  readonly args?: readonly unknown[];
};

export type DeployParameters = {
  readonly abi?: readonly unknown[];
  readonly bytecode: string;
  readonly args?: readonly unknown[];
};

export interface AbstractEthereumUtils {
  getContractAddress(parameters: { readonly from: string; readonly nonce: bigint }): `0x${string}`;

  // Pure ABI encoding. No signer/caller/msg.sender.
  encodeCall(parameters: EncodeCallParameters): Promise<`0x${string}`>;

  // ---------------------------------------------------------------------------------------
  // CREATE2 prediction (`precomputeCreate2Addresses`)
  //
  // Three pure primitives, all synchronous, all a one-liner over viem or ethers. They are here rather
  // than on a separate optional interface because a CREATE2 address is not an optional capability of the
  // deterministic path — without them `precomputeCreate2Addresses` cannot answer at all, so a "partial"
  // implementation would only be able to report that it could not work.
  // ---------------------------------------------------------------------------------------

  /** keccak256 of raw bytes. Hashes both the salt preimage and the init code. */
  keccak256(parameters: { readonly bytes: string }): `0x${string}`;

  /**
   * `abi.encode(...)` — the standard, offset-carrying encoding, not the packed one.
   *
   * Standard specifically: the CREATE2 salt is `keccak256(abi.encode(prefix, version, deploymentId,
   * role))` over four DYNAMIC strings, so the offsets are part of the preimage. `encodePacked` over the
   * same four values hashes to something else entirely, and the resulting addresses would look perfectly
   * plausible while matching nothing the deploy scripts produce.
   */
  encodeAbiParameters(parameters: {
    readonly types: readonly string[];
    readonly values: readonly unknown[];
  }): `0x${string}`;

  /** `keccak256(0xff ++ from ++ salt ++ initCodeHash)[12:]`, EIP-1014. */
  getCreate2Address(parameters: {
    readonly from: string;
    readonly salt: string;
    readonly initCodeHash: string;
  }): `0x${string}`;
}

export interface AbstractEthereumProvider {
  // Privileged dev-node RPC operation, not signer-based.
  setCodeAt(parameters: { readonly address: string; readonly bytecode: string }): Promise<void>;

  // Pure ABI encoding. No signer/caller/msg.sender.
  getCodeAt(parameters: { readonly address: string }): Promise<string>;

  // Read-only contract call (eth_call). No signer/caller/msg.sender. Returns the decoded output.
  readContract(parameters: {
    readonly address: string;
    readonly abi: readonly unknown[];
    readonly functionName: string;
    readonly args?: readonly unknown[];
  }): Promise<unknown>;

  // Number of transactions sent from `address` at the latest block (its next nonce). Used to
  // precompute deterministic deploy addresses when the caller does not supply them.
  getTransactionCount(parameters: { readonly address: string }): Promise<number>;
}

export type DeployReturnType = { contractAddress: string };

/**
 * The web3-library adapter this package sends transactions through. An implementation owns the whole
 * transaction lifecycle: nonce selection, submission, and waiting for inclusion.
 *
 * ## Nonces must be contiguous, and the adapter must supply them
 *
 * Every host address is `CREATE(deployer, startNonce + k)`, and each implementation's creation bytecode
 * is patched with those addresses *before* it is deployed. One skipped or reused nonce moves the whole
 * stack out from under bytecode that cannot adapt. So every transaction sent from a given signer must
 * occupy that signer's next nonce, in order, with no gaps and no reuse.
 *
 * An implementation MUST NOT rely on its web3 library picking a correct nonce per send. Libraries
 * differ, and one of them gets this wrong for this workload:
 *
 *   - ethers v6 `AbstractProvider` caches `eth_getTransactionCount` for `cacheTimeout` — 250 ms of
 *     wall clock, which mining a block does NOT invalidate. A local `deploy()` sends roughly 26
 *     transactions in ~2 s, so consecutive sends fall inside the previous window and receive the same
 *     stale count. An ethers adapter MUST therefore supply the nonce itself.
 *   - viem re-reads it per send (`prepareTransactionRequest` with `blockTag: 'pending'`) and disables
 *     request dedupe for block-tag queries, so its nonces come out right without help. That is
 *     current behaviour, not a promise of this interface — and it changes if the account carries a
 *     `nonceManager`.
 *
 * The safe implementation either way is to read the count once per signer and advance it locally,
 * sending each transaction with an explicit nonce. It applies to `writeContract` as much as to
 * `deploy`: `setupACLOwner` sends three owner-gated calls back to back, the tightest-spaced sends in
 * the flow.
 *
 * Getting this wrong surfaces as `nonce has already been used` on an early transaction — or, on a
 * slower network where the windows never overlap, not at all, which makes it look like a flake.
 *
 * Note this package reads the deployer's nonce exactly once, to derive the addresses. It never
 * re-reads it to check progress, precisely because that read is subject to the same cache. Drift is
 * detected by comparing deployed addresses instead (`assertDeployedAddress`), which no cache affects.
 *
 * ## Sends must be awaited to inclusion
 *
 * Both methods must resolve only once the transaction is mined. This package reads state written by a
 * previous send — an address's code, `ACL.owner()` — so an implementation that resolves at submission
 * time produces write-then-read races.
 */
export interface AbstractEthereumSigner {
  // Signer/account address. Used as msg.sender-equivalent ownership input where deployment calldata needs it.
  getAddress(): Promise<string>;

  // Signer/account-based transaction. Deployer is msg.sender in constructor.
  // Must send with the signer's next nonce and resolve only once mined; see the interface doc above.
  deploy(parameters: DeployParameters): Promise<DeployReturnType>;

  // Signer/account-based transaction. msg.sender is the signer/account.
  // Same nonce and inclusion requirements as `deploy`; see the interface doc above.
  writeContract(parameters: unknown): Promise<unknown>;
}
