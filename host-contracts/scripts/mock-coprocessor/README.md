# mock-coprocessor

A long-running TypeScript daemon that imitates the production FHEVM
coprocessor for the **purposes of mocked testnet experimentation**. It polls
the deployed `FHEVMExecutor` and `ConfidentialBridge` on every configured
chain, replays each FHE operation in plaintext, and stores the result in a
shared SQLite DB.

> **Scope.** No real cryptography, no FHE, no MPC. Operates on event logs +
> arithmetic + a small EIP-712-signed input-bundle builder that mirrors what
> the production relayer would emit. Suitable for end-to-end integration
> smoke tests; **not** suitable as a stand-in for a real coprocessor in any
> security context.

---

## Architecture

```
                          ┌───────────────────────────┐
  RPC: Sepolia ──poll──▶  │  chain-worker (sepolia)   │  ─┐
                          └───────────────────────────┘   │
                                                          ├─▶  MockDb (SQLite)
                          ┌───────────────────────────┐   │      handle → clearText
  RPC: Amoy    ──poll──▶  │  chain-worker (amoy)      │  ─┘
                          └───────────────────────────┘
```

- One **chain worker** per supported chain, polling `eth_getLogs` for the
  FHEVMExecutor + ConfidentialBridge addresses snapshotted in that chain's
  `<chain>/addresses/.env.host`.
- Logs are parsed against the same event ABI used by `test/coprocessorUtils.ts`
  (`abi/events.ts`) and dispatched to a handler:
  - `handlers/fhe-executor.ts` — full operator coverage (TrivialEncrypt,
    FheAdd/Sub/Mul/Div/Rem/BitAnd/BitOr/…/IfThenElse/Sum/IsIn/MulDiv/Rand/…).
    Ported from `test/coprocessorUtils.ts::insertHandleFromEvent`.
  - `handlers/bridge.ts` — on `HandleBridged` copies the source-chain
    clearText into the destination handle. Cross-chain races (chain B's
    `HandleBridged` seen before chain A's `TrivialEncrypt`) are resolved by a
    pending-mappings retry queue. Also handles `FallbackGrantedPlaintext`.
- A shared **SQLite** DB stores `(handle → clearText)` so the separate
  `pnpm mock:query` process can read them.
