// `fhe_runtime` was an off-chain cleartext arithmetic simulator used only by the pre-migration
// `token_mollusk.rs` wrap/burn cleartext-value assertions. It referenced ACL-rewrite-deleted
// `zama_host` event types (e.g. `FheRandBoundedEvent`) and is unused by the migrated suite, which
// asserts lineage/ACL structure rather than replaying cleartext arithmetic. Left on disk (not
// deleted, since it is out of this pass's scope) but no longer compiled.
