// Per-handle access evidence, and where it comes from.
//
// None of it is signed. The permit says who may decrypt and for how long; the evidence says which
// account holds the value and, when the handle is no longer the current one, proves that the access
// existed at a past state. A substituted value here can make a request fail — it can never widen
// what the permit allows — which is why it travels beside the signature rather than under it.
//
// Resolution is a port rather than a client: reading `EncryptedValue` accounts is host RPC and
// fetching a proof for a handle an update has replaced is a call to the relayer-colocated proof
// service. Both
// belong to the application's transport, and the request builder must be testable without either.

/** One handle the caller wants decrypted, and whose value it is. */
export interface SolanaHandleRequest {
  /** The 32-byte ciphertext handle. */
  readonly handle: Uint8Array;
  /**
   * The 32-byte subject: the pubkey whose encrypted value this asks for — the requester itself on a
   * direct entry, the delegator on a delegated one.
   */
  readonly subject: Uint8Array;
}

/** One resolved entry: the handle, its subject, and the evidence that it may be read. */
export interface SolanaAccessEvidence extends SolanaHandleRequest {
  /** The 32-byte encrypted value id — the identity the request carries on the wire. */
  readonly encryptedValueId: Uint8Array;
  /**
   * The 32-byte pubkey of the account itself — the PDA the id names under the host program. Never
   * sent on the wire (the Connector re-derives it from the id); it exists because the
   * historical-access leaves bind the account, so the proof verification needs it, not the id.
   */
  readonly encryptedValueAccount: Uint8Array;
  /** The leaf count the proof was built against; `0n` when the access is current. */
  readonly proofLeafCount: bigint;
  /** Empty when the access is current; otherwise the bare borsh MMR proof. */
  readonly accessProof: Uint8Array;
  /**
   * The account's MMR peaks at `proofLeafCount`; empty when the access is current. Never sent on
   * the wire — they exist so a proof can be verified locally before it costs a submission, against
   * the same snapshot the proof came from.
   */
  readonly peaks: readonly Uint8Array[];
}

/**
 * Where evidence comes from: host state for the account, the proof service for a handle an update
 * has replaced.
 *
 * One handle at a time, because that is the granularity at which the answer differs — one entry of a
 * request can be current while another needs a proof — and because a batched port would have to
 * define what a partially answered batch means.
 */
export interface SolanaAccessEvidenceSource {
  resolve(request: SolanaHandleRequest): Promise<SolanaAccessEvidence>;
}

/**
 * Resolves evidence for every requested handle, in the order asked.
 *
 * Position is meaning: the response linker binds the ordered list the request carried, so a resolver
 * that reordered entries or dropped one would produce a request the user did not make — and one
 * whose answer would fail to bind, or worse, bind to something else. Duplicate occurrences keep
 * their positions but share one fetch: two lookups at different moments could straddle an update
 * and answer current for one occurrence and historical for the other, and a request that
 * contradicts itself about one handle is not the request the caller made either.
 *
 * Every unique fetch is started before any is awaited, so a full request costs the slowest fetch
 * rather than the sum of all of them. Awaiting is still in the order asked: the first entry that
 * cannot be resolved is the one reported, every time, not whichever of several in-flight failures
 * happened to land first.
 *
 * One failed entry fails the whole resolution: a request assembled from partial evidence asks for
 * less than the caller asked for, and nothing downstream can tell that this was deliberate.
 *
 * @param source - The port that answers for one handle.
 * @param requests - The handles, in the order they will be requested.
 */
export async function resolveSolanaAccessEvidence(
  source: SolanaAccessEvidenceSource,
  requests: readonly SolanaHandleRequest[],
): Promise<readonly SolanaAccessEvidence[]> {
  const inFlight = new Map<string, Promise<SolanaAccessEvidence>>();
  const perOccurrence = requests.map((request) => {
    const key = resolutionKey(request);
    let resolution = inFlight.get(key);
    if (resolution === undefined) {
      resolution = source.resolve(request);
      // A failure here is reported by the ordered await below, possibly after another failure has
      // already been thrown; this branch only keeps it from surfacing as an unhandled rejection.
      void resolution.catch(() => undefined);
      inFlight.set(key, resolution);
    }
    return resolution;
  });

  const resolved: SolanaAccessEvidence[] = [];
  for (const resolution of perOccurrence) {
    resolved.push(await resolution);
  }
  return resolved;
}

/**
 * What makes two occurrences the same question: the handle and the subject together. The same
 * handle under two subjects asks about two different pubkeys' access, so it is two fetches.
 *
 * @param request - One occurrence.
 */
function resolutionKey(request: SolanaHandleRequest): string {
  return `${hex(request.handle)}|${hex(request.subject)}`;
}

/**
 * Lowercase hex, for map keys only — wire encoding is the request builder's business.
 *
 * @param bytes - Any bytes.
 */
function hex(bytes: Uint8Array): string {
  let out = '';
  for (const byte of bytes) {
    out += byte.toString(16).padStart(2, '0');
  }
  return out;
}
