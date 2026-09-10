# Encrypted State and MMR reviewer map

This describes the current RFC35 State implementation. Historical per-value decisions are retained in DESIGN_DECISIONS.md; they are not alternate supported APIs.

## Identity and current state

The host owns `EncryptedState` at `["encrypted-state", program, authority, scope]`. Creation proves the authority is a PDA of the program. The state has at most 32 keyed current handles and one shared MMR. A token account controls its own state, scoped to its mint; each JoinRecord controls its contribution state, scoped to its batch. Slot keys are not PDA seeds.

`State.get(key)` supplies a current handle and requires the controlling signature. `State.set(key)` compares the expected previous handle. State outputs also compare the previous shared leaf count; mismatches revert all transaction effects. Slot handles arise only from validated execution outputs.

## Decrypt history

An output may update a slot, append allowed-key leaves, append a public leaf, or combine those actions. A fresh result can acquire decrypt history without occupying a slot. Leaves bind the exact State and handle, plus allowed key for private access. Allows append in declaration order, then public permission. State contains peaks and count; the listener records leaves and returns untrusted inclusion proofs.

Historical private decryption remains valid after a slot changes. Current-slot publication additionally checks the slot key and expected handle. Adding new permissions to a history-only handle is deferred to [#2007](https://github.com/zama-ai/fhevm-internal/issues/2007); no current API silently re-registers arbitrary handles.

## Composition and returns

The initiating State authority opens canonical scratch at `["transient", initiating_state]`. A producer grants its exact output to a consumer State; the consumer’s authority signs when using it. Grants expire through the exact final top-level close, refunding the recorded payer. Missing or invalid finalization rolls back the transaction. Grants authorize compute, not KMS decrypt, although authorized compute can produce persistently decryptable results.

The token returns the transferred result; the batcher adds it to its own contribution. There is no TransferReceipt or permanent transferred-result register. Burn retains a result slot and PendingBurn because redeem/cancel spans transactions.

`FheExecuteArgs.returned_results` selects `(step_index, output_index)` entries independently of step count. Current operators require output index 0. At most 32 handles fit the return channel; order and duplicates follow the list. An empty selection returns no handles. `build_returning` requests one typed result. Return bytes convey no permission.

## Off-chain trust and recovery

The listener reconstructs every executed operation and leaf from the same transaction, regardless of return selection. Compute records, leaves and checkpoint commit atomically. KMS validates proofs against its deciding on-chain State snapshot, not listener-proof-endpoint assertions. Public consumers verify the certificate and exact-handle public proof on-chain. Generic disclosure carries no authenticated token-kind label.

An incomplete local history refuses proofs. Advancing its cursor does not repair it; retention recovery and coverage must be checked separately. A fetched proof can become stale before submission; the consumer rejects it and a retry must fetch fresh evidence. Old acceptance does not prove current freshness.

## Resources and verification

The 32-step maximum is not a promise that every shape fits the default heap or packet budget. Use the measured runtime boundary and cost snapshots; #1872 records the accepted approach. Scratch rent is reclaimed but CPI/compute costs remain. See TESTING.md for unit/runtime/live layers and INVARIANTS.md for the security register.
