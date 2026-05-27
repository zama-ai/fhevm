# Solana PoC Development Issues

Compact rolling issue ledger for the Solana FHEVM PoC. Keep this file inspectable:

- Soft cap: 260 lines.
- Hard cap: 320 lines.
- Keep full detail only for active work and the most recent resolved issues.
- Collapse older resolved issues into one-line entries.
- Do not paste long logs; record the command, result, and the decision it changed.

## Current Status

Latest verified issue:

- Issue 127: confidential-token receiver hooks must be paired with a matching same-transaction
  transfer instruction, so normal transfers cannot be retroactively settled as transfer-and-call.

Latest full runtime result:

```bash
cd ../kms-connector
cargo test -p kms-worker solana_
# 110 filtered Solana tests passed after Issue 126

cd ../kms-connector
cargo test -p kms-worker --lib
# 129 tests passed after Issue 126

cd ../kms-connector
cargo test -p tx-sender --lib
# 13 tests passed after Issue 109

cd ../coprocessor/fhevm-engine
cargo test -p host-listener solana_adapter::tests::
# 14 Solana adapter tests passed after Yellowstone source validation

cd ../solana
anchor build --ignore-keys
cargo test -p zama-solana-runtime-tests --test host_events
# 166 passed after Issue 127
```

Active unverified work:

- None from the progress files. Continue the broader audit loop and add a new active candidate only
  after a concrete mismatch is confirmed.
- Yellowstone web-source validation refreshed `solana/README.md`; implementation, reconnect,
  provider, and finality policy remain product work.

Open external/product items:

- External input verifier service integration, threshold policy, and real proof/transciphering.
- Live native-v0 KMS Core dispatch; current core client/protobuf remains EVM-shaped.
- Concrete tx-sender publication/routing target for native Solana response rows.
- Durable archival/compaction policy for ACL/material/delegation/replay evidence.
- Historical handle discovery conventions for apps/indexers.
- Production Yellowstone gRPC listener integration, provider finality/replay policy, reconnect
  behavior, and backfill plan for Solana event transport.
- Production role/governance names for public decrypt and grant authority.
- Final transfer-and-call product/API shape.
- Transient-session SDK ergonomics.
- Frozen native-v0 request/response encodings and unsupported-version behavior.
- Full production `acl_storage_spec` config-versioned ACL/material/overflow witness layout remains
  deferred until the encoding freeze.

## Issue Index

Older resolved/modelled issues, collapsed:

```text
1. Opaque FHE handles cannot be Solana account seeds.
2. App state and host ACL state need a strict trust boundary.
3. ACL roles need explicit flags.
4. Host protocol events and app-local token events must remain separate.
5. Single-instruction ERC7984 callback settlement exceeded SBF heap limits.
6. Transfer-and-call needed a Solana receiver CPI/return-data bridge and split settlement.
7. External input verification is production-shaped but not production-complete.
8. Random handle birth needed host-owned ACL records and bounded-random parity.
9. KMS/Gateway Solana live wiring remains external to this PoC branch.
10. Material commitments are required for decryptability.
11. Transient allow uses Solana-specific one-shot session semantics.
12. Rent cleanup differs by transient, operator, and durable proof account class.
13. ZamaHost IDL and generated listener code can drift.
14. LiteSVM log limits can hide useful failure lines.
15. Tests need both event replay and account-state checks.
16. Gateway-PoC KMS witness wiring is not the production native-v0 contract.
17. Native-v0 KMS admission needed a pure verifier boundary.
18. Native-v0 Ed25519 signatures and replay persistence needed deterministic helpers.
19. Native-v0 KMS responses needed request/certificate binding.
20. Native-v0 request/response encodings still need final freeze.
21. KMS Core dispatch is blocked by missing native-v0 Solana protobuf/API.
22. Native Solana response rows must not use the EVM Gateway tx-sender path.
23. Direct host FHE operations needed overflow permission witnesses.
24. Token disclosure needed ACL public release plus material/KMS proof.
25. Material commitment needed unsupported handle-metadata rejection.
26. Material commitment needed ACL-record sealing.
27. Delegation updates needed same-slot double-update guard.
28. Native-v0 admission needed same-key batch enforcement.
29. Native-v0 admission needed handle metadata and encrypted-bit limits.
30. Native-v0 live admission needed finalized-by-default commitment policy.
31. ACL role bits needed exact host/KMS validation.
32. Native-v0 request witness attachment needed extra-account rejection.
33. Runtime tests can load stale SBF after program changes.
34. SQLx-backed checks need offline metadata without a local database.
35. Native-v0 response storage needed DB payload/certificate checks.
36. Confidential-token vault operations needed canonical SPL vault account.
37. Confidential-token instructions needed canonical app token-account PDA checks.
38. ZamaHost deny-list updates needed idempotent no-op behavior.
39. ZamaHost pause updates needed idempotent transition behavior.
40. ZamaHost delegation rows needed stored PDA bump and length exactness.
41. ZamaHost transient sessions needed account length exactness.
42. ZamaHost overflow permission witnesses needed length and stored-bump exactness.
43. Confidential-token material witnesses needed exact account length.
44. Confidential-token operator rows needed stored-shape and length exactness.
45. Confidential-token callback settlement rows needed canonical key, bump, and length.
46. Confidential-token account rows needed stored-bump and length exactness.
47. Confidential-token mint rows needed exact length and self-domain checks.
48. ZamaHost HostConfig needed exact singleton shape checks.
49. ZamaHost ACL records needed exact length checks on all consumers.
50. ZamaHost deny-list grant witnesses needed exact length checks.
51. Confidential-token host ACL witnesses needed exact length checks.
52. Confidential-token host ACL witnesses needed canonical PDA and bump checks.
53. Confidential-token amount disclosure needed token-compute authority checks.
54. ZamaHost output ACL birth needed duplicate-subject rejection.
55. ZamaHost role checks needed inline-subject precedence over overflow permissions.
56. Host ACL consumers needed KMS-equivalent subject-slot exactness.
57. ZamaHost persistent grants needed exact overflow-witness consumption.
58. ZamaHost role witnesses needed to reject overflow accounts for inline subjects.
59. ZamaHost scalar binary ops needed unused RHS permission-witness rejection.
60. ZamaHost FheEval needed exact dynamic-account consumption.
61. ZamaHost non-dynamic instructions needed trailing-meta rejection.
62. Confidential-token fixed-account instructions needed trailing-meta rejection.
63. Sample confidential-token receiver needed trailing-meta rejection.
64. Manual PDA creation helpers needed fresh-target and post-create checks.
65. ZamaHost deny-list optional witnesses needed exact disabled-state semantics.
66. ZamaHost transient durable-output policy needed subject and role exactness.
67. Confidential-token self-transfer no-op needed exact unused output ACL metas.
68. Deny-record absence needed non-executable exactness.
69. Config flag setters needed idempotent transition events.
70. Transient close cleanup needed to work while paused.
71. Delegation events needed old and new expiration slots.
72. Deny-list events needed deny-record and slot context.
73. Public-decrypt events needed updated-slot context.
74. Persistent grant events needed authority, placement, and slot context.
75. ACL record birth state and event needed created-slot context.
76. HostConfig state and update events needed updated-slot context.
77. Material commit/seal needed separate audit events.
78. Mock input bind needed native-v0 handle metadata checks before ACL birth.
79. Host output birth/eval paths needed supported FHE-type gates.
80. Connector ACL witness decoder needed current AclRecord layout.
81. Operator transferFrom needed operator-scoped amount inputs.
82. Burn redemption needed public-decrypt release gate.
83. Token account init needed zero-only balance birth.
84. Trivial encrypt handle birth needed nonce binding.
85. Receiver hook calls needed one-shot replay markers.
86. Callback settlement needed hook-call causality.
87. Native-v0 request hashes needed spec domains.
88. Transient sessions needed same-transaction creation proof.
89. Native-v0 requests needed hard profile field gates.
90. HostConfig init needed nonzero profile fields.
91. Host-listener needed event version rejection.
92. Callback settlement needed token-local refund events.
93. Mint init needed nonzero KMS verifier authority.
94. Delegation needed nonzero delegate and app context.
95. Native-v0 public requests needed expiration enforcement.
96. Live native-v0 public requests needed max-validity enforcement.
97. Native-v0 material source needed spec registry value.
98. Native-v0 witnesses needed executable-bit binding.
99. Native-v0 KMS response needed request context binding.
100. Native-v0 response vectors needed current request hash.
101. Native-v0 response needed accepted-request hash re-derivation.
102. Native-v0 response needed accepted replay-key shape binding.
103. Native-v0 response storage needed publication consistency checks.
104. Native-v0 admission needed replay-key signer binding.
105. Native-v0 response rows needed originating request rows.
106. Callback refund settlement needed recipient-signature-free relay.
107. Native-v0 response FK needed correct table placement.
108. Native-v0 tx-sender needed explicit status decision coverage.
109. Native-v0 tx-sender GC needed native row coverage.
110. Native-v0 verifier needed inline-subject role finality.
111. Native-v0 witness creation slots needed observed-slot freshness.
112. Native-v0 response routes needed signed extra-data binding.
113. Native-v0 parser needed extra-data prefetch rejection.
114. Native-v0 live admission needed static prefetch rejection.
115. Confidential transfer current ACL needed token metadata checks.
116. Native-v0 parser/live admission needed signed entries-hash prefetch rejection.
117. Native-v0 live admission needed raw extra-data prefetch rejection.
118. ZamaHost binary outputs needed operator type gates.
119. ZamaHost unbounded random needed EVM type gate.
120. ZamaHost binary ops needed operand type gates.
121. Confidential TransferFrom needed caller-scoped amount ACLs.
```

## Recent Resolved Issues

### 122. Amount Disclosure Needed Token-Amount Label Gate

Resolution: `request_disclose_amount` and `disclose_amount` now accept only token amount ACL labels:
wrap, transfer, burn, burned, transferred, and callback refund amounts. Balance, total-supply, and
callback-success ACLs must use their own paths or remain undisclosable through the amount API.

Development issue: the amount disclosure shape checked FHE type, mint domain, canonical nonce, and
compute-signer authority, but it did not reject a balance-labeled ACL if the requester had public
decrypt authority. That let the generic amount path bypass the app-specific balance disclosure
shape.

Development issue encountered: the first full `host_events` run after the label gate failed because
three amount KMS tests were using Alice's current balance ACL as an amount fixture. The fixtures now
seed a transferred-amount-shaped ACL instead, so they continue to test amount disclosure.

Verification: `cargo fmt`; `anchor build --ignore-keys`; focused
`cargo test -p zama-solana-runtime-tests --test host_events request_disclose_amount
-- --nocapture`; focused `cargo test -p zama-solana-runtime-tests --test host_events
disclose_amount -- --nocapture`; full `cargo test -p zama-solana-runtime-tests --test host_events
-- --nocapture` passed with 160 tests.

### 123. Public Decrypt Release Needed Post-Birth Allow Path

Resolution: ZamaHost durable handle birth paths now reject `output_public_decrypt = true`. Trivial,
random, bounded-random, mock input, signed input, binary bind, ternary bind, and composed eval
durable outputs initialize ACL records with `public_decrypt = false`; releasing a handle for public
decrypt must go through `allow_for_decryption` after birth.

Development issue: the ACL design treats `public_decrypt` as mutable authorization state that flips
from false to true through the public-decrypt allow rule. Letting birth instructions set it directly
bypassed that dedicated authority check and release event.

Verification: `cargo fmt`; `anchor build --ignore-keys`; focused
`cargo test -p zama-solana-runtime-tests --test host_events public_decrypt_at_birth
-- --nocapture`; full `cargo test -p zama-solana-runtime-tests --test host_events -- --nocapture`
passed with 162 tests.

### 124. Existing ACL Admission Needed Handle Metadata Gate

Resolution: ZamaHost now revalidates the stored ACL record handle against the active host chain
before public-decrypt release, compute admission, or transient capability handoff. The check rejects
wrong chain ids, unsupported handle versions, and unsupported FHE type ids before the handle can be
released, used in a compute transcript, or handed to a one-shot transient receiver.

Development issue: the first patch applied the metadata gate to `allow_acl_subjects` as well. That
was too broad because durable grant extension is not a decrypt or compute admission boundary and the
grant-only tests intentionally use opaque handles. The final patch keeps grant extension
handle-opaque and gates the admission paths instead.

Development issue encountered: a first implementation re-read unchecked RHS ACL records after the
role check, which pushed the confidential-transfer budget snapshot over its compute-unit cap. The
final helper validates metadata during the existing unchecked-record read and keeps the budget test
under the current threshold.

Verification: `cargo fmt`; `anchor build --ignore-keys`; focused
`cargo test -p zama-solana-runtime-tests --test host_events unsupported_acl_handle_metadata
-- --nocapture`; full `cargo test -p zama-solana-runtime-tests --test host_events -- --nocapture`
passed with 164 tests.

### 125. Delegation Needed Reserved Wildcard Delegate Rejection

Resolution: ZamaHost now rejects `[0xff; 32]` as a user-decryption delegate, while preserving it as
the reserved wildcard app-context row key. Native-v0 KMS request verification and the branch-local
Gateway-PoC witness path reject the same sentinel before accepting delegated Solana decrypts.

Development issue: the host only rejected zero, self-delegation, and delegate/app equality. The KMS
verifier had the same gap for signed delegated requests. That diverged from the EVM
`DelegateCannotBeWildcard` rule and the native spec's zero/wildcard delegate misuse guard.

Verification: `cargo fmt`; `anchor build --ignore-keys`; focused
`cargo test -p zama-solana-runtime-tests --test host_events
delegation_rejects_wildcard_delegate_sentinel -- --nocapture`; focused
`cargo test -p kms-worker native_v0_rejects_wildcard_delegate_pubkey -- --nocapture`; focused
`cargo test -p kms-worker check_solana_chain_delegated_user_decryption_rejects_wildcard_delegate
-- --nocapture`; full
`cargo test -p zama-solana-runtime-tests --test host_events -- --nocapture` passed with 165 tests;
`cargo test -p kms-worker solana_ -- --nocapture` passed with 107 filtered Solana tests; `cargo
test -p kms-worker --lib -- --nocapture` passed with 126 tests.

### 126. Gateway-PoC Witness Needed Role Matrix Gates

Resolution: the branch-local Gateway-PoC Solana witness path now rejects direct
subject/app-account equality, wildcard app-context sentinels, reserved or zero delegates, and
delegated subject/delegate/app equality. The shared Solana ACL verifier also rejects invalid
delegation tuple shapes before accepting canonical synthetic rows.

Development issue: the Gateway-PoC path verified ACL memberships and material/delegation records,
but did not enforce the role-shape constraints already present in native-v0 request verification
and the EVM user-decryption/delegation surface. A single ACL subject could satisfy both direct
owner and app-context roles, or a delegated synthetic row could encode equal roles.

Verification: `cargo fmt`; focused
`cargo test -p kms-worker rejects_invalid_delegation_witnesses -- --nocapture`; focused
`cargo test -p kms-worker check_solana_chain_user_decryption_rejects_subject_app_equality
-- --nocapture`; focused
`cargo test -p kms-worker check_solana_chain_user_decryption_rejects_wildcard_app_context
-- --nocapture`; focused
`cargo test -p kms-worker check_solana_chain_delegated_user_decryption_rejects_delegate_app_equality
-- --nocapture`; focused
`cargo test -p kms-worker check_solana_chain_delegated_user_decryption_rejects_wildcard_delegate
-- --nocapture`; full `cargo test -p kms-worker solana_ -- --nocapture` passed with 110 filtered
Solana tests; full `cargo test -p kms-worker --lib -- --nocapture` passed with 129 tests.

### 127. Receiver Hooks Needed Same-Transaction Transfer Intent

Resolution: `confidential_call_transfer_receiver` and
`confidential_call_transfer_receiver_from` now require the immediately preceding instruction in the
same transaction to be a matching `confidential_transfer` or `confidential_transfer_from` whose
mint, source token account, destination token account, and transferred-amount ACL match the hook.

Development issue: the split Solana transfer-and-call flow used the normal transfer output as its
sent amount, but the receiver-hook marker could be created in a later transaction. That let a normal
transfer be retroactively treated as transfer-and-call and potentially refunded, unlike ERC7984's
separate `transfer` and atomic `transferAndCall` paths.

Verification: `cargo fmt`; `anchor build --ignore-keys`; focused
`cargo test -p zama-solana-runtime-tests --test host_events
transfer_receiver_hook_rejects_standalone_prior_transfer -- --nocapture`; focused
`cargo test -p zama-solana-runtime-tests --test host_events transfer_receiver_hook -- --nocapture`
passed 7 tests; focused
`cargo test -p zama-solana-runtime-tests --test host_events transfer_callback -- --nocapture`
passed 5 tests; full `cargo test -p zama-solana-runtime-tests --test host_events -- --nocapture`
passed with 166 tests.
