# solana-listener (PoC scaffold)

This crate is a minimal scaffold for Solana host ingestion aligned with:

- `docs/protocol/explorations/solana-host-listener/INTERFACE_V0.md`

Current intent:

1. finalized event source -> canonical mapping
2. canonical mapping -> existing DB ingestion contracts
3. replay-safe cursor updates

The source implementation is intentionally mocked in this first scaffold; next step is wiring real finalized RPC log retrieval.
