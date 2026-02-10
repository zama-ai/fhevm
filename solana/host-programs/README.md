# Solana Host Program (PoC scaffold)

This Anchor workspace is a minimal program-first scaffold for the Solana host listener PoC.

It currently exposes:

1. `request_add(lhs, rhs, is_scalar)` -> emits `OpRequestedAdd`
2. `allow(handle, account)` -> emits `HandleAllowed`
3. `request_add_cpi(lhs, rhs, is_scalar)` -> emits `OpRequestedAdd` via `emit_cpi!`
4. `allow_cpi(handle, account)` -> emits `HandleAllowed` via `emit_cpi!`

This aligns with:

- `docs/protocol/explorations/solana-host-listener/INTERFACE_V0.md`

Notes:

1. `result_handle` derivation is placeholder only.
2. No persistence/state accounts are implemented in this scaffold.
3. Listener integration is expected to consume emitted events from finalized RPC logs.
