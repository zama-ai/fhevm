import type { FhevmProtocolContext, ProtocolVersionResolution, PubKeyCrsVersionResolution } from '../types/coreFhevmClient.js';
import type { HostContractName, HostContractVersion } from '../types/hostContract.js';
import type { TfheVersion, TkmsVersion } from '../types/moduleVersions.js';

////////////////////////////////////////////////////////////////////////////////

/**
 * Brand for {@link FhevmClientFrozenContext}. Prevents a structurally-similar
 * plain object from being passed where a genuine, SDK-produced context is
 * expected — instances are only ever created by the SDK.
 */
export declare const fhevmClientFrozenContextBrand: unique symbol;

////////////////////////////////////////////////////////////////////////////////
// FhevmClientFrozenContext (exposed contract)
////////////////////////////////////////////////////////////////////////////////

/**
 * An immutable, self-consistent snapshot of the **version basis** a single FHEVM
 * operation resolves against: the raw host-contract versions (ACL, KMSVerifier,
 * InputVerifier, ProtocolConfig, …) plus the versions derived from them
 * (protocol, PubKey/CRS, TFHE, TKMS).
 *
 * ### Why this exists
 *
 * These versions are otherwise read from several places with mismatched
 * lifetimes: `protocol`/`tfhe`/`tkms` are resolved once per client and frozen,
 * while the host-contract `getVersion()` reads are TTL-cached, so they can drift
 * out of sync within a single client (e.g. a KMSVerifier read refreshing past
 * the protocol version the loaded WASM was selected for). This object is the
 * single source of truth: resolve the needed subset once, freeze it, and thread
 * it through every internal function **instead of** passing individual
 * `TfheVersion` / `TkmsVersion` / protocol-version arguments. Everything then
 * branches off one coherent snapshot.
 *
 * ### IMPORTANT — a public action must never call another public action
 *
 * The frozen context is resolved **once**, at a public (`fhevm`-first) action's
 * entry point, and threaded through `context` into internal (`context`-taking)
 * helpers only. This is safe — and needs no re-entrancy / reuse machinery —
 * *precisely because* no public action calls another public action: every action
 * delegates to internal implementations instead, so a nested public action can
 * never be reached mid-operation and the context is therefore built exactly once
 * and never rebuilt.
 *
 * Preserve this invariant. If one public action needs another's behavior,
 * extract the shared logic into a `context`-taking internal helper and call that
 * from both — never call the public action from an action. (The `host`
 * chain-discovery tier is the sole exception: `resolveFhevmConfig` fans out to
 * sibling host actions, but host actions run before a chain is resolved and never
 * carry a frozen context, so the invariant is unaffected there.)
 *
 * ### Scope (intentional)
 *
 * This holds the version basis **only** — quasi-immutable data that changes only
 * on a contract upgrade. It deliberately does NOT hold per-block-mutable state
 * (KMS context id / epoch, signer sets, ACL allowances): that state has a
 * different lifetime and is resolved where it is used. Keeping the two apart is
 * what lets this snapshot ignore reorgs.
 *
 * ### Partial resolution
 *
 * A context may carry only the versions an operation needs. The plain accessors
 * (`protocolVersion`, `tfheVersion`, `hostContractVersion(name)`, …) **throw** if
 * the requested version was not resolved, so a mis-scoped call fails fast rather
 * than silently proceeding on an absent value. Use the `has*` predicates or the
 * `try*` accessors to probe without throwing.
 */
export type FhevmClientFrozenContext = {
  readonly [fhevmClientFrozenContextBrand]: never;

  //////////////////////////////////////////////////////////////////////////////
  // Host contract versions (raw on-chain getVersion() results)
  //////////////////////////////////////////////////////////////////////////////

  hasHostContractVersion(name: HostContractName): boolean;
  /** @throws If `name`'s version was not resolved in this context. */
  hostContractVersion<name extends HostContractName>(name: name): HostContractVersion<name>;
  tryHostContractVersion<name extends HostContractName>(name: name): HostContractVersion<name> | undefined;

  //////////////////////////////////////////////////////////////////////////////
  // Protocol version
  //////////////////////////////////////////////////////////////////////////////

  readonly hasProtocolVersion: boolean;
  /** @throws If the protocol version was not resolved in this context. */
  readonly protocolVersion: ProtocolVersionResolution;
  readonly tryProtocolVersion: ProtocolVersionResolution | undefined;

  //////////////////////////////////////////////////////////////////////////////
  // PubKey/CRS version
  //////////////////////////////////////////////////////////////////////////////

  readonly hasPubKeyCrsVersion: boolean;
  /** @throws If the PubKey/CRS version was not resolved in this context. */
  readonly pubKeyCrsVersion: PubKeyCrsVersionResolution;
  readonly tryPubKeyCrsVersion: PubKeyCrsVersionResolution | undefined;

  //////////////////////////////////////////////////////////////////////////////
  // Protocol context (protocol + PubKey/CRS bundle)
  //////////////////////////////////////////////////////////////////////////////

  readonly hasProtocolContext: boolean;
  /** @throws If either the protocol or PubKey/CRS version was not resolved. */
  readonly protocolContext: FhevmProtocolContext;

  //////////////////////////////////////////////////////////////////////////////
  // TFHE / TKMS module versions
  //////////////////////////////////////////////////////////////////////////////

  readonly hasTfheVersion: boolean;
  /** @throws If the TFHE version was not resolved in this context. */
  readonly tfheVersion: TfheVersion;
  readonly tryTfheVersion: TfheVersion | undefined;

  readonly hasTkmsVersion: boolean;
  /** @throws If the TKMS version was not resolved in this context. */
  readonly tkmsVersion: TkmsVersion;
  readonly tryTkmsVersion: TkmsVersion | undefined;

  //////////////////////////////////////////////////////////////////////////////

  toJSON(): Record<string, unknown>;
};
