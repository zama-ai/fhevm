// What to do when a request does not come back with an answer.
//
// Three actions exist, and classification exists to pick one: resolve the evidence again and submit a
// new request, submit the same request again later, or stop. None of them involves the wallet —
// evidence lives outside the signed permit, so a rebuilt proof and a bounded retry are new requests
// under the signature the user already gave.
//
// The Connector has its own taxonomy for why it refused a request, with the same three actions in it.
// That classification does not reach a client: when the Connector refuses, no response is produced,
// and what the SDK observes is a request that was accepted and never answered. So the action is
// inferred here from what is observable — the relayer's machine-readable label, the HTTP-level
// overload signal, and the absence of an answer — rather than read off the wire. The one case where
// the inference matters is the unanswered request: both of its evidence-related causes, a proof that
// an append has moved past and a handle an update has replaced, are repaired by resolving the evidence
// again, which is why they need not be told apart.

/** What the transport saw instead of an answer. */
export type SolanaUserDecryptRejection =
  /** The relayer refused the submission: it never became a job. `label` is its machine-readable one. */
  | { readonly kind: 'refused'; readonly label: string; readonly message?: string }
  /** The relayer is over capacity and named a delay, in seconds. */
  | { readonly kind: 'overloaded'; readonly retryAfterSeconds: number }
  /** The job ran and ended in failure. */
  | { readonly kind: 'failed'; readonly label: string; readonly message?: string }
  /**
   * The job was accepted and produced nothing. This is what a Connector refusal looks like from
   * here, and the two evidence-related causes behind it are the ones a client can act on.
   */
  | { readonly kind: 'unanswered' };

/** What to do about a rejection. */
export type SolanaUserDecryptRecovery =
  /** Resolve the evidence again and submit the request the new evidence makes. */
  | { readonly action: 'resolve-again' }
  /** Submit the same bytes again, no sooner than this many seconds from now. */
  | { readonly action: 'retry-unchanged'; readonly afterSeconds: number }
  /** Nothing this layer can do repairs it. */
  | { readonly action: 'give-up' };

/** How long to wait when the relayer refused for a reason of its own and named no delay. */
export const SOLANA_USER_DECRYPT_DEFAULT_RETRY_SECONDS = 2;

/**
 * The action each relayer label implies.
 *
 * Split by whether submitting the same bytes again could ever succeed. A label about the request —
 * its shape, its host chain, its funding — will say the same thing forever, and a client that keeps
 * retrying learns nothing while paying for the attempts. A label about the service — paused,
 * unreachable, timed out — is the same request meeting a different moment.
 *
 * A label not in this table is given up on. That is the safer of the two mistakes: an unknown label
 * treated as retryable becomes a client that hammers the relayer over something no delay repairs.
 */
export const SOLANA_USER_DECRYPT_LABEL_ACTIONS: Readonly<Record<string, 'retry-unchanged' | 'give-up'>> = {
  malformed_json: 'give-up',
  missing_fields: 'give-up',
  validation_failed: 'give-up',
  request_error: 'give-up',
  not_allowed_on_host_acl: 'give-up',
  host_chain_id_not_supported: 'give-up',
  not_found: 'give-up',
  insufficient_balance: 'give-up',
  insufficient_allowance: 'give-up',
  // A failure to read the host ACL, not a refusal by it: `not_allowed_on_host_acl` above is the
  // ACL saying no, this is the relayer not getting an answer — its own taxonomy marks it retryable.
  host_acl_failed: 'retry-unchanged',
  rate_limited: 'retry-unchanged',
  protocol_paused: 'retry-unchanged',
  internal_server_error: 'retry-unchanged',
  gateway_not_reachable: 'retry-unchanged',
  readiness_check_timed_out: 'retry-unchanged',
  response_timed_out: 'retry-unchanged',
};

/**
 * Picks the action for one rejection.
 *
 * @param rejection - What the transport saw.
 */
export function classifySolanaUserDecryptRejection(rejection: SolanaUserDecryptRejection): SolanaUserDecryptRecovery {
  switch (rejection.kind) {
    // A refusal and a failed job carry the same labels and mean the same thing for the next attempt;
    // where they differed — a job exists — is not something the action depends on.
    case 'refused':
    case 'failed':
      return actionForLabel(rejection.label);
    case 'overloaded':
      return {
        action: 'retry-unchanged',
        afterSeconds:
          rejection.retryAfterSeconds > 0 ? rejection.retryAfterSeconds : SOLANA_USER_DECRYPT_DEFAULT_RETRY_SECONDS,
      };
    case 'unanswered':
      return { action: 'resolve-again' };
  }
}

/**
 * The recovery a label maps to, with the unknown label given up on.
 *
 * @param label - The relayer's machine-readable label, exactly as sent.
 */
function actionForLabel(label: string): SolanaUserDecryptRecovery {
  const action = SOLANA_USER_DECRYPT_LABEL_ACTIONS[label];
  if (action === 'retry-unchanged') {
    return { action: 'retry-unchanged', afterSeconds: SOLANA_USER_DECRYPT_DEFAULT_RETRY_SECONDS };
  }
  return { action: 'give-up' };
}
