---
name: list-contract-calls
description: List every call the js-sdk makes to an Ethereum contract (which contract, which function, which arguments) as an exhaustive table with a confidence tag per row. Use when the user asks which on-chain calls / contract reads / eth_calls the SDK performs, wants an inventory or audit of contract interactions, or asks what the SDK calls on a given contract. Not for writing or changing contract calls.
---

# List Ethereum contract calls

Produce an **exhaustive** table of every call the SDK makes to an Ethereum contract. Missing a call is the worst outcome: be systematic, then cross-check. When a real ambiguity remains, **ask the user** instead of guessing (see "When to ask").

This skill names **no SDK-internal file, function, or variable**: those get renamed. Rely only on stable anchors:
- the external libraries' APIs (ethers, viem);
- JSON-RPC method names;
- the ABI formats (JSON and human-readable);
- Solidity 4-byte selectors.

Discover the SDK's own names fresh on every run (step 1). **Grep results are candidates, not proof of completeness.** Read the code around them. If an anchor below matches nothing, report it under "Stale anchors"; don't conclude there are no calls.

## Scope

- Default scope: production source under `src/`. Exclude:
  - tests;
  - generated / build output;
  - vendored code and WASM bindings;
  - `node_modules`.
- Top-level `test/`, `examples/`, `scripts/` and Solidity sources are out of scope unless the user asks.
- State the scope you used in one line above the tables. If the user names a subset (a contract, a module, an action), restrict to it but stay exhaustive inside it.

**Set up the filters first** and apply them to **every** grep below. Source extensions and generated directories change over time, so discover them; don't assume `.ts`:

```bash
find src -type f -not -path '*/node_modules/*' | sed -E 's/.*\.//' | sort | uniq -c   # source extensions present
# also read the build config (tsconfig / package.json / bundler config) for generated / output dirs
EXT='ts'                                                   # alternation of source extensions found, e.g. 'ts|tsx|mts'
EXCL='\.(test|spec)\.|/__tests__/|/node_modules/'          # append '|/<generated-or-vendored-dir>/' for each one found
```

Every grep below is then written as `grep -rnIE "<pattern>" src | grep -E "^[^:]+\.($EXT):" | grep -vE "$EXCL"`, shortened to `G "<pattern>"`. Define it as a shell function: `G(){ grep -rnIE "$1" src | grep -E "^[^:]+\.($EXT):" | grep -vE "$EXCL"; }`.

Patterns must work in BSD grep, GNU grep and ugrep (`grep` may be any of them). Use `(^|[^A-Za-z0-9_])`, not `\b` inside an alternation like `(\.|\b)`, which ugrep rejects.

## What counts as a contract call

Any code path that ends in a JSON-RPC `eth_call`, `eth_estimateGas` or `eth_sendTransaction` / `eth_sendRawTransaction` whose target is a contract address. That includes:
- typed reads: ABI + function name + args;
- raw calls: hand-built calldata = 4-byte selector + ABI-encoded args;
- transactions.

The target is not always a protocol contract (e.g. an ERC-1271 `isValidSignature` call on a user's smart-contract wallet still counts).

**Not** contract calls. Exclude them, but count them in "Excluded" so the user sees they were considered:
- chain / account RPCs (`eth_chainId`, `eth_blockNumber`, `eth_getBalance`, ...);
- signing, hashing and signature recovery. This includes `signTypedData` handed to the user's signer / wallet client: it is **not** a contract call and **not** suspected, even if a smart-account signer could make calls internally (user decision);
- ABI encode / decode;
- key or account generation;
- HTTP requests to off-chain services.

**Suspected calls: when unsure, flag, never drop.** If you can't prove whether a code path reaches a contract, don't decide silently either way. Tag it `[SUSPECTED]`. Typical cases:
- a provider / client passed to a third-party library or callback whose internals you can't see (signing operations excepted: see above);
- a dynamic or computed method on a contract-like object;
- a generic "send RPC request" helper whose method name is a variable;
- an `eth_call` whose target can't be shown to be a contract.

**ERC-1271 `isValidSignature` on the user's address is always `[SUSPECTED]`** (user decision; don't ask about it). It is a raw `eth_call` to the **user's own address**, sent to check a user signature. Whether it reaches a contract depends on the user's account at run time:
- **Smart-contract wallet** (e.g. a Safe): the call executes `isValidSignature(bytes32 hash, bytes signature)`. The wallet returns the magic value `0x1626ba7e` to accept the signature. That is a real contract call.
- **EOA** (no code at the address): the `eth_call` "succeeds" with empty return data, and the SDK reads that as "this is an EOA" and rejects. No contract code runs.
- The call is usually skipped entirely when the signature can be checked locally (65-byte signature whose recovered signer equals the user address).

When writing the row:
- In the Notes column, state these three cases briefly: called only when local verification doesn't succeed; contract call only if the user is a smart-contract wallet; empty return data means EOA.
- Under "Needs checking", record "user decision: kept as [SUSPECTED]" instead of a question.
- Name the contract `ERC-1271 wallet`.

A concurrency or batching helper that runs many calls in parallel is **not** multicall: each item is still its own `eth_call`. Only an on-chain `Multicall` contract (or a viem `multicall`) bundles calls, and then each inner call is a row.

## Procedure

1. **Discover the transport layer (don't assume names).**
   - Locate the modules that use the Ethereum libraries. Imports can span lines and can be re-exported, so search for the library name, not a full import line:
     ```bash
     G "['\"](ethers|viem)(/[^'\"]*)?['\"]"
     ```
     Use these hits only to locate adapter modules and re-export points. Read in full only the files that also match the next grep.
   - Find the library-level calls (stable external names):
     ```bash
     G "\b(readContract|simulateContract|writeContract|multicall|staticCall|getFunction|sendTransaction|estimateGas|getContract)\b|new Contract\(|\.call\(|['\"]eth_(call|sendTransaction|sendRawTransaction|estimateGas)['\"]"
     ```
   - For every `new Contract(` / `getContract(` result, **follow the variable**. Each method called on it is a contract call (ethers exposes ABI functions as dynamic methods, which no grep anchors).
   - For each library call, find the **SDK wrapper function** that contains it (the SDK's own "read contract" / "raw call" / "send tx" methods), and the **interface or object** that exposes those wrappers to the rest of the code.
   - Write down:
     - the discovered wrapper method names;
     - the property / module name they hang off;
     - the files that **implement** them. These are adapters: their library calls are not call sites.

     Every later step greps for **these discovered names**, not names remembered from a previous run.
   - Find **every implementation** of that interface: one per library, plus any mock / alternative runtime shipped in production code. Each may add its own calls.
   - **List every method of the discovered interface** (and of its alternative-runtime subtypes), each classified as contract call or excluded. This list feeds the "Excluded" section.

2. **Enumerate call sites of the discovered wrappers.**
   - Grep for calls of each wrapper method with **any receiver**, **bare** (destructured), and with **generic type arguments**:
     ```bash
     G "(^|[^A-Za-z0-9_])(<method1>|<method2>)(<[^>]*>)?\(" | grep -v 'prototype\.'
     ```
   - Code often stores the module in a local variable or casts it to a subtype. Grep for the module name **alone** (`G "\b<module name>\b"`), since an alias assignment may span lines. Check that each alias's calls are in your list.
   - Also grep for the discovered names used as values: passed as callbacks, destructured, re-exported, renamed on import, or accessed as **computed members** (`obj[name](…)`, `obj['<method>']`). Follow each one to its calls.
   - **Baseline** = the hits of the wrapper-call grep, minus:
     - hits inside the adapter files from step 1;
     - comment lines;
     - method declarations in interfaces and types.

     Record the raw hit count and the baseline separately.

3. **Find every ABI and selector.**
   - JSON ABIs (shared or inline):
     ```bash
     G "type: ?['\"]function['\"]|stateMutability"
     ```
   - Human-readable ABIs:
     ```bash
     G "parseAbi(Item)?\(|new Interface\(\[|['\"](function|event) [A-Za-z_]"
     ```
   - Hand-built selectors and calldata: 8-hex-digit constants, selector / magic-value constants (searched case-insensitively), and hash-derived selectors. A hash-derived selector is a `keccak256` / `id` / `toFunctionSelector` of a signature string, including through renamed imports, or a `slice(0, 10)` on a hash:
     ```bash
     grep -rnIiE "0x[0-9a-f]{8}\b|selector|magic|toFunctionSelector|keccak256|\bid\(" src | grep -E "^[^:]+\.($EXT):" | grep -vE "$EXCL"
     ```
     This is noisy (constants, examples, compression headers). Inspect each hit; keep only calldata.
   - List **every ABI fragment one by one** (not file by file), with the contract name if a comment, constant name or type nearby states it.

4. **For each call site, read the enclosing function** (not just the grep line) and record:
   - **Function**: the Solidity function name. Resolve indirections (e.g. `someAbi[0].name`) by opening the ABI. For raw calldata, resolve the selector to a fragment or a known standard (ERC-20, ERC-1271, EIP-712 / EIP-5267, ...).
   - **Arguments**: name and Solidity type of each argument from the ABI, plus where the SDK value comes from (user input, config, result of a previous read, ...). Write `—` for none.
   - **Contract**:
     1. Take the candidate contract(s) from labels near the ABI (comments, names, types).
     2. Then **trace the target address up the callers** until it reaches a concrete source: a chain / config field, an address returned by another contract read, or user input. This confirms the contract and records the address source.
     3. Several labels, or an "any contract" label, on one fragment mean fan-out (step 5). So does an address that can come from different deployments (e.g. a contract and its replica / mirror).
     4. Record the **address origin** in Notes as one of:
        - `← preset`: a constant or built-in chain preset in the SDK;
        - `← public argument`: it enters through a public API argument, including a chain / deployment description passed by the caller;
        - `← contract read`: returned by another contract call;
        - `← user account`: the user's own address.

        Several paths means listing every origin. **The origin does not affect the confidence tag** (user decision): who supplies the address is recorded, not judged. A public-argument origin is never a doubt to ask about.
   - **Returns**: output type(s) from the ABI.
   - **Call site**: `path:line` of the wrapper call.
   - **Triggered by**: the **outermost exported SDK entry point** a user would call (public action, client method), not the nearest internal helper. Follow callers up to the package's public exports. If there is none, write "no exported entry" and tag `unused` in Notes.
   - **Confidence**: see "Confidence tags".

5. **Follow wrappers and fan-out.**
   - If `abi`, function name or address is a **parameter** of a helper or local closure, enumerate **every caller** of that helper / closure **recursively**: a wrapper of a wrapper also forwards it. Stop when each one is a concrete constant. Emit one variant per resolved (contract, function).
   - If one helper is called with different addresses (e.g. a version / domain / threshold getter shared by several contracts), emit one variant per reachable (contract, function).
   - If a call runs once per item (per input, per signer, ...), say so in Notes: "one call per <item>, batched".

6. **Grouping rules (settled here; don't ask about them).**
   - **Exactly one row per (contract, full signature)**, where the full signature is the name plus argument types. Overloads are separate rows. Put **all** call sites of that pair in the row's Call site cell. Never drop a site.
   - **Unused**: a helper that performs a call but is never reached from an exported entry point is still listed, with `unused` in Notes.
   - **Fallbacks**: a raw call that replays a typed read (e.g. to tell a revert from a transport error) is not a separate row. Add its site to the typed-read row and note "raw-call fallback at <site>".
   - **Cleartext (and other alternative) modes are fully separated.** A call is **cleartext** when it is reached only when the SDK runs in cleartext / mock mode, i.e. in code that only that mode's runtime or modules execute. Identify that code from step 1's "every implementation" list and by following which runtime selects it, not from file names alone.
     - Every cleartext call goes in the separate **Cleartext mode** part of the output (see "Output"). That includes cleartext calls to regular contracts (e.g. a cleartext path reading the same ACL getter as production): give them their own row there, and **never merge them** into the production row.
     - Cleartext-only contracts are never merged into a production contract with a similar name.
     - **ACL in cleartext mode** (user decision): the contract at the ACL address is `CleartextACL` **or** `CleartextForgeACL`, depending on the deployment context. Write the contract as `CleartextACL / CleartextForgeACL` and note "depends on deployment context". This is a known fact, not a doubt: don't tag it `[UNCERTAIN]` and don't ask about it.
     - **Cleartext uncertainty is left as is** (user decision). If a cleartext call's contract or function can't be established from the code by the procedure, keep it `[UNCERTAIN]`, and its contract `Unknown` if undetermined.
       - Don't resolve it by analogy with other calls or by reading other sources.
       - Don't ask the user about it.
       - Still list it under "Needs checking".
     - Code shared by both modes (reached in production too) stays in the production part. If the cleartext runtime also reaches it, write "also cleartext" in Notes; don't copy the row.
     - The same split applies to any other alternative mode (mock, test double shipped in `src/`): one separate part per mode.

7. **Cross-check before answering.**
   - Every hit of steps 1–3 maps to a row, or is explained in one line (adapter implementation, interface declaration, non-contract call, encoding only, noise).
   - Every ABI fragment from step 3 is either:
     - used by a row;
     - reported as "ABI with no call site";
     - reported as "encoding only" (used with encode / decode, never called).
   - Every hand-built selector maps to a row or is explained.
   - Reconcile **three counts separately**, explaining each difference in one line:
     - call sites (the step-2 baseline plus raw / aliased sites found later);
     - expanded variants (after wrapper and fan-out expansion);
     - rows (after grouping).
   If a check fails, go back; don't paper over it.

## When to ask (do not guess)

Ask the user only when, after reading the ABI labels **and** tracing callers:
- you can't tell whether a code path is a contract call at all (`[SUSPECTED]`). Phrase it as: "I'm not sure, this is probably a contract call: please check `<file:line>` (<why>)"; or
- more than one contract is still possible, or a selector / dynamic ABI / function name / target address can't be resolved (`[UNCERTAIN]`); or
- the user's request implies a scope different from the default (e.g. they mention tests or examples) and it changes the result.

Don't ask about anything the defaults or step 6 already settle. In particular, cleartext `[UNCERTAIN]` rows are left as is (step 6), even though `[UNCERTAIN]` is otherwise always asked about.

Put all doubts into one question round. Use the AskUserQuestion tool if available. Otherwise ask in plain text, then stop and wait for the answer. Finish the tables after the answer.

## Output

State in one line each:
- the scope and filters used;
- the discovered transport wrapper names.

### Summary tables (always first)

Then write **one summary table per mode**, in this order:
1. `#### Summary: Production`;
2. `#### Summary: Cleartext`, if any cleartext call exists;
3. one `#### Summary: <Mode>` per other alternative mode, alphabetically.

Together they list **every** row of the detailed tables below. They answer "which contract, which function, which arguments" at a glance, and are the tables used to **compare results between agents and runs**. So their content and formatting must be deterministic:

#### Summary: Production

| Confidence | Contract | Function | Arguments |
| --- | --- | --- | --- |
| [CONFIRMED] | ACL | `isAllowedForDecryption` | `handle: bytes32` |

#### Summary: Cleartext

| Confidence | Contract | Function | Arguments |
| --- | --- | --- | --- |
| [CONFIRMED] | CleartextACL / CleartextForgeACL | `getFHEVMExecutorAddress` | — |

(Format illustration only; verify every row against the code.)

- **Rows**: exactly one per detailed-table row of that mode, no more, no less. Each summary's row count equals its part's row total.
- **Sort order**, **case-insensitive** (compare lowercased strings; ties broken by the original string in ASCII order):
  1. Contract. `Unknown` goes last.
  2. Function. Overloads are ordered by their Arguments cell.
- **Confidence**: the same tag as the detailed row.
- **Contract**: the same name as the detailed section heading; `Unknown` if undetermined. A target that is not a protocol contract is named by its standard, as `<Standard> wallet` for a user account (e.g. `ERC-1271 wallet`), or `<Standard> contract` otherwise.
- **Function**: the bare Solidity function name in backticks. No return type, no parentheses.
- **Arguments**:
  - `name: type` in ABI order, comma-separated, in one pair of backticks;
  - use the ABI's argument names; if the ABI has none, use `arg0`, `arg1`, …;
  - write `—` for no arguments;
  - **tuple types are always expanded** to their component types, recursively: `(address,bytes)`, `(bytes32,address)[]`. Never write `tuple`, `tuple[]` or a struct name.
- Nothing else in the cells: no links, sources, notes or emphasis. Those belong in the detailed tables.

Then write the detailed tables in **separate parts, one per mode**:
1. `## Production`;
2. `## Cleartext mode`, if any cleartext call exists;
3. one `## <Mode> mode` per other alternative mode (see step 6).

Inside each part, write one `### <Contract>` section per contract (sorted by name), each with a table sorted by function. Calls whose contract can't be determined go in a final `### Unknown contract` section of that part.

### <Contract>

| Confidence | Function → returns | Arguments (name: type ← source) | Call site | Triggered by | Notes |
| --- | --- | --- | --- | --- | --- |
| [CONFIRMED] | `someFunction(bytes32) → bool` | `id: bytes32` ← user input | [file.ts:42](#placeholder); [other.ts:7](#placeholder) | `somePublicAction` | address ← config field; one call per input, batched |

(Format placeholder only.)

### Confidence tags

Every row gets **exactly one** tag. The tag answers only one question: **did the agent identify this call correctly?** Pick it from the **weakest** of the row's identification claims:
- that the code path is a contract call;
- the contract;
- the function;
- the argument sources;
- that the call is reachable.

The **address origin** (preset / public argument / contract read / user account) is **not** an identification claim. It goes in Notes and never lowers the tag (see step 4).

| Tag | Meaning | Required evidence |
| --- | --- | --- |
| `[CONFIRMED]` | Certain. | The call site was read. The function was resolved from an ABI or selector. The contract is named by a label near the ABI **and** the traced address path names the same contract, whatever the address's origin. For example: a contract-named field such as `contracts.<name>.address`, a variable or parameter named after it, or a getter that returns it. |
| `[LIKELY]` | Very probably right; one claim rests on inference. | The call is certain, but the contract comes from the ABI label alone (no name anywhere along the address path, e.g. a bare `address` argument), or one argument source was inferred. Notes says which. |
| `[UNCERTAIN]` | A contract call, but the contract or function is ambiguous. | Several candidates, an unresolved selector, or a dynamic name. Notes lists the candidates. Always asked about, except in cleartext mode (left as is, see step 6). |
| `[SUSPECTED]` | Possibly not a contract call at all. | See "Suspected calls". Always asked about. |
| `[USER-CONFIRMED]` | Was `[UNCERTAIN]` / `[SUSPECTED]`; the user confirmed it. | Notes holds the user's answer. |

If the user says a suspected call is **not** a contract call, remove its row and keep it in "Needs checking" with the verdict.

**Notes** holds only:
- the address source and its origin (`← preset` / `← public argument` / `← contract read` / `← user account`);
- fan-out / per-item;
- `unused`;
- "also cleartext" (shared code also reached in cleartext mode);
- raw-call fallback;
- the call kind if not a typed read (raw call / tx);
- the reason a row is below `[CONFIRMED]`.

After the tables:
- **Totals**, **per part** (Production, Cleartext mode, ...) and overall:
  - rows per confidence tag;
  - rows, distinct contracts, distinct (contract, function) pairs;
  - the three reconciled counts from step 7 (call sites → expanded variants → rows).
- **Needs checking**: one line per `[UNCERTAIN]` / `[SUSPECTED]` row, and per suspected site the user rejected. Give `file:line`, why it is in doubt, and what would settle it. Write the user's verdict next to each one once answered. Write "none" if empty; never omit this section.
- **Excluded**: the non-contract interface methods from step 1, with call counts.
- **ABIs with no call site**, and "encoding only" ABIs.
- **Stale anchors**: any grep in this skill that matched nothing (a sign the code or this skill has drifted).

Call sites are clickable markdown links `[file.ts:42](path#L42)`, never wrapped in backticks.
