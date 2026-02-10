# Solana Host Program (PoC scaffold)

This Anchor workspace is a minimal program-first scaffold for the Solana host listener PoC.

It currently exposes:

1. Full symbolic op request surface:
- `request_add`, `request_sub`
- `request_binary_op`, `request_unary_op`
- `request_if_then_else`, `request_cast`
- `request_trivial_encrypt`, `request_rand`, `request_rand_bounded`
2. ACL signal:
- `allow`
3. Matching `_cpi` variants for emission-mode comparison (`emit!` vs `emit_cpi!`).

This aligns with:

- `docs/protocol/explorations/solana-host-listener/INTERFACE_V0.md`

Notes:

1. `result_handle` derivation is placeholder only.
2. No persistence/state accounts are implemented in this scaffold.
3. Listener integration is expected to consume emitted events from finalized RPC logs.
4. Operation logic is shared between `emit!` and `emit_cpi!` entrypoints; wrappers differ only by emission mode.
