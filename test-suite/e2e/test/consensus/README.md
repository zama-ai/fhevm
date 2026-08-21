# Materialization byte-consensus tests

Opt-in E2E gate proving that independent coprocessors running the **same
software revision on the same backend/hardware class** persist byte-identical
ciphertexts under transaction-boundary materialization:

- intra-transaction intermediates are forwarded in memory (never
  compress/decompress round-tripped) — the fixture's `selected → sum/diff`
  fan-out;
- cross-transaction consumers always read the producer's persisted canonical
  compressed bytes — the fixture's same-block `stageInputA → deriveFromAAndB`
  edge and the next-block `consumeFanout` reader;
- whole transactions execute and complete atomically (exact per-transaction
  computation counts, no errors).

The oracle compares, across every coprocessor database: raw ciphertext bytes,
type/version, producing operation, transaction/block provenance, gateway key
id, and the Keccak digest bindings up to the on-chain
`AddCiphertextMaterialConsensus` quorum. CPU and GPU topologies are never put
in one byte comparison; their shared oracle is user-decrypted plaintexts.

## Running

The gate is skipped unless explicitly enabled. It requires a running stack
with N (default 3) coprocessors sharing one host chain and gateway:

```sh
RUN_MATERIALIZATION_CONSENSUS=1 \
COPROCESSOR_COUNT=3 \
CONSENSUS_SOFTWARE_REVISION=$(git rev-parse HEAD) \
CONSENSUS_BACKEND_CLASS=cpu \
CONSENSUS_HARDWARE_CLASS=<runner class> \
GATEWAY_RPC_URL=... \
GATEWAY_CONFIG_ADDRESS=... \
CIPHERTEXT_COMMITS_ADDRESS=... \
DATABASE_URL_0=... DATABASE_URL_1=... DATABASE_URL_2=... \
npx hardhat test test/consensus/materializationConsensus.ts --network localCoprocessor
```

The gate also runs an **aliased-handle** scenario (`AliasFixture.sol`).
Under the minted-in-transaction handle discriminant (see
`docs/minted-discriminant-note.md`), sourcing is part of the handle: two
same-block transactions consuming the same persisted boundaries alias each
other and must persist identical bytes on every coprocessor, while a
transaction that recomputes the same inputs locally folds zero boundary bits
and mints a DISTINCT handle — representation-mixing aliases cannot collide.
The gate asserts both facts.

## The equivocation residual

Across a fork, handles collide only when the replacement block shares the
parent AND timestamp of the block it replaces: a slashable same-slot double
proposal on Ethereum L1, or an ordinary unsafe-head reorg on fixed-block-time
chains (OP-stack derives the timestamp from the height). Historically a
collision with mixed input sourcing was the one case canonical
materialization could not bridge by construction:
`decompress(compress(x))` is bit-inexact for noisy ciphertexts (see the
ignored `compression_round_trip_bit_exactness_survey` test), and the
consuming operation's PBS modulus-switch rounding absorbs that delta only
when it stays below a rounding boundary. The boundary bits close that case
deterministically — colliding handles now imply identical sourcing, hence
identical bytes.

Drift detection plus automatic revert (`DRIFT_AUTO_REVERT_ENABLED`) remains
as the general backstop for any other divergence; it has its own test
coverage and is deliberately not duplicated in this suite.

`helpers.test.ts` and `materializationFixture.test.ts` are plain unit tests of
the harness itself and run in the ordinary suite without a stack.

The multi-coprocessor topology launcher is provided separately (it is
infrastructure, not part of this test suite).
