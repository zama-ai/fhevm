# ACL Permissions Scenario

Focus: ACL permission, pause, deny-list, and privileged-operation safety.

## Actions
- `allow`
- `allowTransient`
- `allowForDecryption`
- `pause` / `unpause`
- `blockAccount` / `unblockAccount`

## Model state
- Successful persistent `allow(handle, account)` pairs.
- Successful `allowForDecryption(handle)` handles.
- Violation flags: privilege bypass, deny-list bypass, sender-permission bypass, paused bypass, transient visibility mismatch.

## Invariants (IDs)
- `ACL-PERM-001`: successful persistent/decryption writes remain visible in ACL storage.
- `ACL-PERM-002`: privilege, deny-list, sender-permission, and transient visibility guarantees are never violated.
- `ACL-PERM-003`: ACL mutating flows never succeed while paused.

## Known non-goals
- No cross-transaction invariant over `allowedTransient` (transient storage resets at tx boundary).
