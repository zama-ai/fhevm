# Replace `prettier-plugin-solidity` with `forge fmt` + `forge lint`

Status: **proposal**. Scope: `v12` and `v13` — `foundry.toml`, `.prettierrc`, the `prettier:*` and
`lint` scripts, `devDependencies`, and repo VS Code settings.

**Decision: `forge` is the single formatter and linter for Solidity.** Prettier must not touch `.sol`
anywhere — CLI, CI or editor. Prettier keeps `js`, `json`, `md`, `ts`, `yml` and nothing else.

## 1. The constraint, and why it is principled

`pkg/src/contracts` is vendored from host-contracts at the commit named in
`pkg/package.json → fhevm.vendoredFrom` (RULES.md rule 6), gated by
`sdk/scripts/check-vendored-sources.sh`, which now runs inside `build`.

**The CONTENT of those files is upstream's decision, not ours** — host-contracts formats them with
`prettier-plugin-solidity` at `printWidth: 120`, per its own `.prettierrc`. This section originally
concluded that the directory must therefore be excluded from any formatter we adopt. **That
conclusion was wrong**, and §3.5 supersedes it: the files are now *stored* forge-formatted and the
gate normalises the upstream side instead. Exclusion left two styles in one tree and did not
actually protect the files (§3.4.1). Normalising does both.

### 1.1 `[fmt]` cannot be tuned to agree — searched and measured

The obvious hope is a `[fmt]` config under which `forge fmt` is a no-op on the vendored files, making
the exclusion unnecessary and the IDE formatter safe. **It does not exist.** Differing lines in
`ACL.sol`, sweeping every plausible key:

```
baseline                                   93
multiline_func_header=params_first_multi   72
  + prefer_compact=none                    56   ← best found
  + override_spacing=true                  56
  + line_length=140                        79
  + line_length=100                       108
```

Under that best config **7 of 7 vendored files still differ** — and, decisively, **9 files that
agree at baseline start disagreeing**. Net: 7 differing files becomes 16. The tuned config is
strictly worse than the defaults and neither reaches zero.

What survives tuning is not a setting but two opposite line-breaking philosophies:

```
forge:     UserDecryptionDelegation storage x =
               $.userDecryptionDelegations[msg.sender][delegate][contractAddress];
prettier:  UserDecryptionDelegation storage x = $.userDecryptionDelegations[msg.sender][delegate][
               contractAddress
           ];
```

forge breaks *before* the `=`; prettier breaks *inside* the index. Nested mappings differ the same
way. No `[fmt]` key controls either, and a search from the prettier side is equally futile: 20
prettier configurations all left exactly 93 differing lines, the number never moving. **Do not
re-run either search** — leave `[fmt]` at its defaults.

This is precisely why normalisation (§3.5) is the right answer rather than configuration: it does
not need the two formatters to agree, it erases prettier's style instead of matching it. A corollary
in an earlier draft — that the tree could never be uniformly `forge fmt`-clean — is now false;
`forge fmt --check` reports **0 unformatted files** in both packages.

## 2. Measured facts

| # | fact | evidence |
| --- | --- | --- |
| 1 | `forge fmt` disagrees with the current formatting on 30 files in v13, 29 in v12 | `forge fmt --check` |
| 2 | Of those, **9 of v13's 21 vendored files** and **7 of v12's 16** | e.g. `ACL.sol`, `FHEVMExecutor.sol`, `KMSVerifier.sol` |
| 3 | Prettier currently passes on the vendored files | so prettier's output **equals** the vendored bytes |
| 4 | Upstream host-contracts is prettier-formatted at width 120 | its `.prettierrc` |
| 5 | `forge fmt` scans `test/ts/node_modules` | 17–18 of the diffs are the tarball-consumer fixture |
| 6 | `foundry.toml`'s `skip` does **not** apply to `forge fmt` | fixture files appear in `--check` despite `skip = ["test/ts/node_modules/**"]` |
| 7 | `[fmt] ignore` works, and **must be a bare directory name** | see §2.1 — `dir/**` silently fails |
| 7b | With the ignore applied, the real reformat is **3 files in v13, 4 in v12** | 30 → 3, verified via `FOUNDRY_FMT_IGNORE` on the live tree |
| 8 | VS Code can drive `forge fmt` | `juanblanco.solidity` exposes `solidity.formatter: "none"｜"prettier"｜"forge"`, implemented at `out/src/client/formatter/forgeFormatter.js` |
| 9 | Both relevant extensions are already installed | `juanblanco.solidity-0.0.187`, `nomicfoundation.hardhat-solidity-0.8.29` |

Fact 6 is the easy one to miss: the fixture is *installed* Solidity from a tarball, and reformatting
it would corrupt a published artifact under test.

### 2.1 `ignore` syntax — `dir/**` is a silent footgun

`[fmt] ignore` entries are matched such that `**` requires **at least one intervening directory**, so
the obvious spelling protects nested files while leaving direct children exposed:

| spelling | direct children | nested | |
| --- | --- | --- | --- |
| `"src/protected"` | excluded | excluded | **correct — recursive** |
| `"src/protected/"` | excluded | excluded | equivalent |
| `"src/protected/*"` | excluded | excluded | equivalent |
| `"src/protected/**"` | **NOT excluded** | excluded | **broken** |
| `"**/protected/**"` | **NOT excluded** | excluded | broken |
| `"src/protected/*.sol"` | excluded | NOT excluded | direct only |

This matters exactly here: the vendored files sit *directly* in `pkg/src/contracts`, so
`"pkg/src/contracts/**"` would have protected only `shared/` and `immutable/` and let `forge fmt`
rewrite `ACL.sol`, `FHEVMExecutor.sol`, `KMSVerifier.sol` and the rest — silently, with a config that
looks correct. **Use the bare directory name.**

Verified on the live tree with `FOUNDRY_FMT_IGNORE` (an env override, so no file had to change):
`forge fmt --check` goes from 30 flagged files to 3, with no `pkg/src/contracts` or
`test/ts/node_modules` entry remaining.

## 3. Design

### 3.1 `foundry.toml`, both packages

```toml
[fmt]
# pkg/src/contracts is deliberately NOT ignored (§3.5): vendored sources are stored forge-formatted,
# so forge fmt must reach them and is a no-op on them.
# test/ts/node_modules IS ignored: installed tarball fixture; formatting it would rewrite the
# published artifact under test. `skip` applies to forge build, not forge fmt.
# If an entry is ever added back, use a BARE DIRECTORY NAME — "dir/**" silently misses direct
# children (§2.1).
ignore = ["test/ts/node_modules"]
```

As executed, the one-time reformat touched **11 files in v12** and **12 in v13** — 7 and 9 vendored
respectively, plus 3–4 of our own.

`[lint]` already exists in both files with a populated `exclude_lints`, so `forge lint` is partly
adopted already — it runs during `forge build` today. It becomes explicit in `lint`.

### 3.2 Scripts

```
prettier:check = prettier --check "**/*.{js,json,md,ts,yml}"      # no --plugin, no sol
prettier:write = prettier --write "**/*.{js,json,md,ts,yml}"
fmt:check      = forge fmt --check
fmt:write      = forge fmt
lint           = eslint && forge lint && tsc … (per the tsconfig plan)
build          = … prettier:check && fmt:check && lint …
```

Also delete the temporary comparison harness once the decision is locked in:
`prettier.forge-compat.mjs`, `scripts/fmt-compare.sh`, and the `fmt:prettier` / `fmt:forge` /
`fmt:compare` scripts. They exist only to let the prettier-vs-forge gap be re-measured by hand; once
prettier has no Solidity parser they cannot run anyway.

### 3.3 A problem this deletes

`v13/package.json`'s `//` block documents a real bug worked around today: the plugin is passed as
`--plugin=prettier-plugin-solidity` on the CLI rather than declared in `.prettierrc`, because an
editor's prettier extension resolves plugin *names* against its own module path, fails, and silently
falls back to prettier's defaults (`printWidth` 80, not 120) — so files saved in the editor differed
from what `prettier:check` demanded. Removing the plugin removes that failure mode entirely, and the
`.prettierrc`/CLI asymmetry with it. That comment block should be deleted, not migrated.

### 3.4 Disabling prettier for Solidity, everywhere

Three layers, in order of strength. The first is the one that actually matters.

**Layer 1 — remove the capability.** Dropping `prettier-plugin-solidity` does not merely turn
prettier off for `.sol`; it makes prettier *unable* to process Solidity at all. Measured:

```
$ prettier --no-config foo.sol
[error] No parser could be inferred for file "foo.sol".

$ prettier --support-info | grep -i solidity
(nothing — solidity is absent from prettier's language list; no parser named *sol*)
```

This holds identically for the CLI, CI, and `esbenp.prettier-vscode`, which resolves plugins from the
workspace's `node_modules`. No configuration can re-enable it. **This is the real mechanism** — the
settings below only make the intent legible and guard against the plugin ever returning.

**Layer 2 — `.prettierignore`.** Add `*.sol` in both packages. `esbenp.prettier-vscode` honours
`.prettierignore` (confirmed in its `extension.js`), so even a future reinstall of the plugin would
not let the editor format Solidity.

**Layer 3 — VS Code settings.** The repo `.vscode/settings.json` currently sets one global formatter
(`editor.formatOnSave: true`, `editor.defaultFormatter: esbenp.prettier-vscode`). Add:

```json
"[solidity]": { "editor.defaultFormatter": "JuanBlanco.solidity" },
"solidity.formatter": "forge",
"files.readonlyInclude": {
  "**/sdk/host-contracts-cleartext/*/pkg/src/contracts/**": true
}
```

Two facts that make this correct rather than hopeful, both verified from the extension manifests:

- **Both installed extensions register language id `solidity`** for `.sol`
  (`juanblanco.solidity-0.0.187` and `nomicfoundation.hardhat-solidity-0.8.29`). With two candidate
  formatters VS Code picks arbitrarily or prompts, so the `[solidity]` block is required, not
  optional — it names the winner.
- **They share the `solidity.formatter` key**, and both declare the same enum
  `["none", "prettier", "forge"]`. So the single setting is valid for whichever one is invoked;
  there is no conflict and no need to disable the other extension.

### 3.4.1 The IDE formatter CANNOT honour `[fmt] ignore` — measured

This was an open question; it is now settled, and the answer is the worst one. The extension's forge
backend (`out/src/client/formatter/forgeFormatter.js`) does:

```js
cp.execFile('forge', ['fmt', '--raw', '-'], { cwd: rootPath })
forge.stdin.write(documentText)
```

It pipes the buffer's **text** to stdin. **No path is passed**, so a path-based `ignore` rule has
nothing to match. Measured on `pkg/src/contracts/ACL.sol` with the ignore list active:

```
forge fmt --raw -  < ACL.sol   ->  87 lines changed   (ignore had no effect)
forge fmt --check    ACL.sol   ->  correctly ignored
```

So **one save of a vendored file breaks RULES.md rule 6**, and `[fmt] ignore` cannot prevent it.
Format-on-save for Solidity is therefore unsafe on its own, and disabling it is not sufficient
either — an explicit *Format Document* does the same damage.

The protection has to come from outside the formatter. In order of strength:

1. **`files.readonlyInclude`** (VS Code ≥ 1.74; this machine runs 1.135.0). Marks the vendored tree
   read-only in the editor, which blocks the formatter, quick-fixes and stray typing alike — the
   only option that addresses the whole class rather than one symptom:

   ```json
   "files.readonlyInclude": { "**/host-contracts-cleartext/*/pkg/src/contracts/**": true }
   ```

   It is editor-only and advisory — it does not protect `forge fmt` run from a terminal, which is
   what `[fmt] ignore` is for. The two are complements, not alternatives.
2. **`"[solidity]": { "editor.formatOnSave": false }`** — removes the accidental path, leaves the
   deliberate one open.
3. **`check:vendored` in `build`** — already exists, and is the real backstop. It catches the damage
   rather than preventing it, which is fine as long as it runs before anything is committed.

Adopt 1 and 3 together. 2 is redundant once 1 is in place, and costs format-on-save everywhere else
in the package for no benefit.

### 3.5 Option B — the normalised vendored gate (DECIDED)

The vendored files are stored **forge-formatted**, and `check-vendored-sources.sh` compares
`forge fmt(upstream)` against them. Consequences:

- `forge fmt --check` passes on `pkg/src/contracts`, so **no `[fmt] ignore` entry is needed for it** —
  only `test/ts/node_modules` remains in the ignore list.
- The IDE formatter becomes a genuine **no-op** on those files, so `files.readonlyInclude` is no
  longer load-bearing. Keep it anyway as defence in depth: it costs nothing and still blocks stray
  typing and quick-fixes, which the formatter change does not.
- The tree is uniformly forge-formatted — which is what "forge is the single formatter" means.

The comparison is `fmt(upstream)` vs **ours as stored**, not `fmt(upstream)` vs `fmt(ours)`. The
stricter form: if anyone reformats a vendored file by some other means, the gate fails rather than
silently normalising the difference away.

Measured properties this relies on:

| property | result |
| --- | --- |
| `forge fmt` idempotent on the vendored set | 0 / 16 non-idempotent |
| normalised gate passes today | 0 / 16 differ |
| catches `msg.sender` → `tx.origin` | yes |
| catches licence change | yes |
| catches `uint256` → `uint128` | yes |
| catches an inserted blank line | yes |
| cost | 0.4 s per side |

**Rule 6 changes meaning** and both wordings must be updated together: RULES.md and the header of
`check-vendored-sources.sh` currently say *byte-for-byte identical to upstream*. It becomes
*identical to `forge fmt`(upstream)*. The golden rule softens correspondingly, from "never edit" to
**"only ever transformed by `forge fmt`, never by hand"** — `ARCHITECTURE.md` §0 and invariant I0
need the same edit.

#### 3.5.1 Pinning forge is a precondition

`forge fmt` output can change between releases, and a change would make every stored vendored file
non-compliant at once. Nothing pins it today:

- `foundry.toml` has **no** forge-version key (`FOUNDRY_FORGE_VERSION` is silently ignored).
- No CI workflow covers `sdk/host-contracts-cleartext` at all yet.
- `foundryup` supports `--install <version>` and `--use <version>`, but that is a local action, not a
  declared constraint.

So the pin has to be a declared version plus a check that asserts it: record the version (currently
`1.5.1-stable`) in a `.foundry-version` file, and add a `check:forge-version` script that compares
`forge --version` against it and fails loudly. Wire it into `build` ahead of `check:vendored`, so a
forge upgrade is reported as "you changed formatter versions" rather than as 16 mysterious vendored
drifts.

## 4. Execution order

Ordered so that nothing is unprotected at any point. Steps 1–2 must land before step 4, which is the
first that rewrites a vendored file.

1. ~~**Pin forge**~~ **DONE.** `.foundry-version` = `1.5.1-stable` in both packages,
   `scripts/check-forge-version.sh`, `check:forge-version` script, wired into `build`.

   **Discovered while doing it: `check:vendored` was wired into nothing** — not `build`, not `test`,
   not CI. It was defined and manual-only, so the golden rule had *no* automatic enforcement, and
   this plan's §1 claim that it "runs inside `build`" was false. It is now wired in, immediately
   after `clean` and before any other work, in both packages. `npm run build` passes in v12 and v13
   with both gates active:

   ```
   ✅ forge 1.5.1-stable matches .foundry-version
   ✅ 16 vendored files identical to upstream   (v12)
   ✅ 21 vendored files identical to upstream   (v13)
   ```

   This had to be true *before* step 4 rewrites vendored files — otherwise the reformat would happen
   with nothing checking the result.
2. ~~**Normalise the gate.**~~ **DONE.** `sdk/scripts/check-vendored-sources.sh` now compares
   `forge fmt(upstream)` against the stored file (upstream side only). RULES.md rule 6,
   `ARCHITECTURE.md` §0 and invariant I0 all updated. The gate failed exactly as predicted before
   step 4 — 7 of 16 in v12, 9 of 21 in v13.
3. ~~`[fmt] ignore = ["test/ts/node_modules"]`~~ **DONE** in both `foundry.toml`s, with
   `pkg/src/contracts` deliberately absent (§3.5).
4. ~~**Run `forge fmt`.**~~ **DONE.** 11 files rewritten in v12, 12 in v13. Verified after:

   ```
   check:vendored          exit 0   16 / 21 vendored files identical to forge fmt(upstream)
   forge fmt --check       0 files unformatted, both packages
   forge fmt --raw ACL.sol NO-OP  <- an IDE save can no longer damage a vendored file
   npm run build           exit 0, both
   forge test              18 passed (v12), 42 passed (v13)
   ```
5. Switch the scripts (§3.2). Delete the comparison harness (`prettier.forge-compat.mjs`,
   `scripts/fmt-compare.sh`, `fmt:prettier` / `fmt:forge` / `fmt:compare`).
6. **Remove `prettier-plugin-solidity` from the whole workspace** — three manifests:
   `sdk/package.json:44`, `v12/package.json:78`, `v13/package.json:87`; the four `--plugin=` script
   flags (`v12` 63–64, `v13` 74–75); and the `//` comment block in both packages (§3.3), which
   documents a workaround that no longer exists. Add `.prettierignore` with `*.sol`.
7. `npm install` at `sdk/`, then `npm run build` in both packages.
8. VS Code settings (§3.4 layer 3).

## 5. Open questions

1. ~~How are the vendored files protected from the IDE?~~ **DECIDED: Option B** — store them
   forge-formatted and normalise the gate. Designed in §3.5, sequenced in §4.
2. ~~**Does `forge lint` overlap or conflict with `solhint`/eslint anywhere?**~~ **DECIDED: no
   overlap, and solhint is now banned outright** — the same shape as the `prettier-plugin-solidity`
   decision in question 3, and for the same reason. Measured:

   - **solhint was never installed in this workspace.** No dependency in any manifest, no
     `.solhint*` config, no binary under any `node_modules/.bin`. Nothing to un-configure.
   - **eslint does not conflict**: it has no Solidity parser here and lints only `.ts`/`.js`.
   - **Four inert `solhint-disable-next-line` comments survived** in code we own — `CleartextDB.sol`
     and `create2-deploy/script/FhevmCreate2Base.s.sol`, in both generations — annotations for a
     linter that has not run in this workspace at any point. Removed.
   - **Vendored sources keep theirs and are exempt.** `pkg/src/contracts/**` carries upstream's
     directives, rule 6 forbids editing it, and a gate that flagged them could never go green.
     Same carve-out `[lint] ignore` already makes.

   Enforced by `sdk/scripts/check-lint-policy.sh` (`check:lint-policy`), wired into `build` in both
   packages ahead of `forge:lint`, so a reintroduction is reported as policy rather than as a
   confusing second opinion about a file. `[lint] exclude_lints` is now the single place a Solidity
   rule may be turned off — which is the whole point of the ban. Recorded as invariant I14.

   **Still true, and not yet resolved: `forge lint` is not a gate.** It exits 0 on a `note`, so the
   one finding it currently reports (`screaming-snake-case-immutable` on `ACLOwner.acl`, both
   generations) does not fail `build`. Making it fail is a separate decision from banning solhint.
3. ~~Should the root toolchain keep `prettier-plugin-solidity`?~~ **DECIDED: no — remove it from the
   entire `sdk/` workspace.** Three manifests and four script flags; see §4 step 6. Prettier then has
   no Solidity parser anywhere in the workspace, which is the enforcement mechanism (§3.4 layer 1).
