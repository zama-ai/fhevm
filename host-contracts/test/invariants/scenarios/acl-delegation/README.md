# ACL Delegation Scenario

Focus: user decryption delegation state machine and delegated-access coherence.

## Actions
- `delegateForUserDecryption`
- `revokeTrackedDelegationAsDelegator`
- `revokeTrackedDelegationAsCaller`
- `pause` / `unpause`

## Model state
- Per tuple `(delegator, delegate, contractAddress)`: tracked `handle` and status (`ACTIVE` / `REVOKED`).
- Violation flags: paused bypass and non-delegator revoke success.

## Invariants (IDs)
- `ACL-DEL-001`: tuples marked `REVOKED` have expiration `0` and no delegated access.
- `ACL-DEL-002`: tuples marked `ACTIVE` grant delegated access for their tracked handle.
- `ACL-DEL-003`: delegation/revocation cannot succeed while ACL is paused.
- `ACL-DEL-004`: non-delegators cannot revoke tracked delegations.

## Known non-goals
- No assertion on event payloads or delegation counters.
