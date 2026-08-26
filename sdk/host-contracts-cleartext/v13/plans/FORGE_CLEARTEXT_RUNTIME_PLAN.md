# Forge-specific cleartext contracts

Status: **proposal**. Scope: `pkg/forge/src/FhevmDeploy.sol` only.

## 1. Scope

`FhevmDeploy.sol` deploys the Forge cleartext contracts. **Nothing else changes** — no node
detection, no flag, no `create2-deploy` option, no TypeScript parameter.

That is not a reduced version of a larger feature; it is the only place the feature can work.
Cheatcodes are handled by an inspector inside **forge's own EVM**, and forge grants them only to
contracts it created during the test. `FhevmDeploy` runs in-process in a forge test and creates the
whole stack there, so its contracts qualify. Every other path broadcasts transactions to a node, and
a contract that lives on a node does not.

What that would have cost, if the other paths were included:

| calling into a Forge-variant stack | `vmSafe.pauseGasMetering()` |
| --- | --- |
| `forge test`, stack created by the test — **`FhevmDeploy`** | works |
| anvil over RPC — `cast`, js-sdk, hardhat | **reverts** |
| `forge test --fork-url`, stack from forked state | **reverts** — *cheatcodes are not enabled for `<addr>`* |
| the same, after `vm.allowCheatcodes(PROXY address)` | works |

The revert is not a no-op: `0x7109…dD12D` has no code outside forge, and Solidity emits an
`extcodesize` check before a high-level call that returns nothing. Every FHE operation would fail,
because every override in `CleartextForgeFHEVMExecutor` pauses metering. A deployed Forge stack is
therefore usable only from forge — a constraint no deploy flag can enforce, since the tooling cannot
know who will call the stack tomorrow. Confining the variants to the one path that is *always* forge
removes the question.

## 2. The change

`FhevmDeploy.sol` consumes pre-compiled blobs from `_internal/LocalHostBytecode.sol`. So:

1. **`internal/generateLocalHostBytecode.ts`** — emit two additional blobs,
   `CLEARTEXT_FORGE_FHEVM_EXECUTOR_CREATION_CODE` and `CLEARTEXT_FORGE_ARITHMETIC_CREATION_CODE`.
   Additional, not replacements — see the warning below.
2. **`pkg/forge/src/FhevmDeploy.sol`** — import and deploy those two in place of
   `CLEARTEXT_FHEVM_EXECUTOR_CREATION_CODE` and `CLEARTEXT_ARITHMETIC_CREATION_CODE`.

> **`DeployLocalStack.s.sol` must keep the standard blobs.** It is described in-repo as *"the
> broadcast twin of `FhevmDeploy.sol`: same phases, same order, same blobs"* — and that shared-blob
> arrangement is exactly the trap here. It broadcasts to a node, so a Forge blob would give it a stack
> that reverts on every RPC call. The two paths stop sharing these two blobs, and that divergence is
> the point of the change rather than an accident of it.

**No address moves.** `FhevmDeploy` derives everything from `CREATE(deployer, nonce)`, so swapping an
implementation's bytecode leaves the whole layout untouched — including the three addresses RULES.md
rules 15 and 17 require `ZamaConfig.sol` to find.

## 3. Tests

1. A forge test deploying via `FhevmDeploy`, running one FHE operation, and asserting the `gasleft()`
   delta is far below the unmetered figure. Measured on a proxied contract during this investigation:
   **~29400 unpaused vs ~840 paused**, so the assertion has a wide margin and will not be flaky.
   Assert on the delta, never on a cheatcode call's success flag — that flag reads `true` on anvil
   where nothing happened.
2. A test that `CleartextForgeArithmetic` produces varying values across runs, where the standard
   contract's `keccak(randType, seed, "randValue")` is fixed. This is the observable behaviour change,
   and the one most likely to surprise: any existing test asserting a specific cleartext random value
   will now fail under `FhevmDeploy`.
3. The existing `DeployLocalStack` / js-sdk suites, unchanged, as the regression guard for the
   blob-sharing warning above.

## 4. Port to v12

Mechanical, and only after v13's tests pass so a wrong answer is not copied.

1. Copy `CleartextForgeFHEVMExecutor.sol` and `CleartextForgeArithmetic.sol` into
   `v12/pkg/src/cleartext/`, adjusting for v12/v13 divergence: v12 has no `ProtocolConfig` or
   `KMSGeneration`, `FHEVMExecutor`'s override set may differ, and `CleartextArithmetic`'s two
   `virtual` randomness hooks must exist there in the same shape.
2. Mirror the `generateLocalHostBytecode.ts` change and regenerate.
3. Mirror the `FhevmDeploy.sol` change; leave v12's `DeployLocalStack` alone.
4. Re-run v12's deploy and e2e suites. RULES.md's v11/v12 floor applies: v12 must keep working for its
   own js-sdk consumers, and this touches contracts they call.

**Check first:** v12's deterministic randomness may not match v13's, and the Forge variant discards it
in both. Any v12 test asserting a specific cleartext random value breaks exactly as in (3.2).

## 5. Open question

**Duplication.** `CleartextForgeFHEVMExecutor` is a byte-identical copy of `CleartextFHEVMExecutor`
apart from the metering calls, and nothing checks they stay in sync. The copy is structurally forced —
`super` performs the op and the record together, so the metering cannot be added by inheritance — but
a CI diff modulo the `pauseGasMetering`/`resumeGasMetering` lines is cheap and would catch drift.

`CleartextForgeArithmetic` needs no such check: it inherits and overrides two `virtual` hooks, which is
why it is 20 lines where the executor is 150. If `FHEVMExecutor` ever grew an equivalent hook — a
`_afterOp` the cleartext recording hangs off — the executor variant could shrink the same way and the
question would disappear.

Minor, and worth doing while the files are open: `CleartextForgeArithmetic` has four unused imports
(`cleartextDbAdd`, `FheTypeBitWidth`, `ICleartextDB`, `ICleartextArithmetic`), all flagged by
`forge build`.
