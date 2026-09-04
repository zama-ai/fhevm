# fhevm-npm: a CLI that stays small as the rules grow

Version: **0.1** (2026-09-03). A proposal for the look and feel of the `fhevm-npm` command line, not yet
executed. Today the CLI has 31 flat commands, 18 of them `check-*`; every new rule adds one more, plus
a Makefile line, a README line and a completion entry. This plan makes rules grow inside one command.

## Principles

- A **rule** is not a command. `check` runs every rule; a rule is added inside fhevm-npm and nowhere else.
- A **generator** is not a command either. `generate` runs every generator; its `--check` modes are rules.
- Commands are **verbs with a stable object**: `sync vendored`, `forge install`, `consumer test`,
  `version list`. A group appears only when it has at least two subcommands that will not merge.
- One manifest load per invocation. The battery is one process, not eighteen.
- Old names keep working during the migration as hidden aliases that print where to go, then leave.

## The commands

```
fhevm-npm check [--only <rule>[,<rule>…]] [--list]          every rule; the pre-build battery
fhevm-npm generate [--only <generator>[,…]] [--check]        every generator; --check = the matching rules
fhevm-npm sync vendored [--check]                            common-vendored → destinations
fhevm-npm sync fhevm-chains [--commit <sha> | --latest]      protocol registry → fhevm-chains.config.json
fhevm-npm forge install [package]                            forge dependency trees
fhevm-npm forge clean [package] [--dry-run] [--force]
fhevm-npm consumer test [package] [--run] [--ci] [--build-linked-dependencies] [--test-file <path>] [-o <dir>] [-f] [--list]
fhevm-npm consumer regenerate-lock [package]
fhevm-npm version list [--check-npmjs] [--json]              the payloads and what npmjs holds
fhevm-npm version check [package...] [--check-npmjs]         derived tree vs central versions.json
fhevm-npm version apply [--check-npmjs] [--dry-run]          reconcile from central versions.json
fhevm-npm list packages                                      manifest inventory
fhevm-npm pack-tarball [package] [-o <dir>] [--clean]
fhevm-npm sh-completion <zsh|bash>
```

Global options stay: `-v/-vv/-vvv/-vvvv`, `--manifest <file>`.

`version apply`, including its dry run, accepts a clean tree or exactly one unstaged modification to
`sdk/versions.json`; it rejects every other changed path. The read-only `version list` and `version
check` commands work on a dirty tree.

## Look and feel

`check` prints one block per rule that found something, and one summary line. Silence per rule is the
default; `-v` lists the rules that passed too.

```
$ fhevm-npm check
❌ [package-json-order] ./hardhat/v3/plugin/package.json: top-level entries must follow …
❌ [4.2.1] ./hardhat/v3/e2e: imports root-pinned package 'ethers' but does not declare it; add …
❌ check: 2 violation(s) in 2 rule(s) across 24 package(s) — 21 rules passed.
```

```
$ fhevm-npm check --only vendored-origin,mirror
✅ check: 0 violation(s) — 2 rules passed (vendored-origin, mirror).
```

```
$ fhevm-npm check --list
cleartext-config      sdk/cleartext-config.json faces byte-identical in every destination
commit-scope          a commit touches one package scope
dependencies          4.2.x root pins, 4.2.3 sibling ranges, 3.1.1 file: links
…
```

`generate` is the writer twin of `check`: same `--only`, same `--list`, and `generate --check` is
exactly `check --only <the generators' rules>` so the two can never disagree about what "fresh" means.

```
$ fhevm-npm generate
✅ generate: exports (4 files), cleartext-config (2 files), chain-constants (1 file) — nothing else changed.
```

Groups print their own usage when called bare (`fhevm-npm sync` lists `vendored` and `fhevm-chains`);
an unknown subcommand names the group's subcommands, never the whole CLI.

Rule names are the ones the reports already print in brackets (`[4.2.1]`, `[package-json-order]`),
so a violation line tells you what to pass to `--only`.

## Where the old names go

| today                                                                       | tomorrow                                    |
| --------------------------------------------------------------------------- | ------------------------------------------- |
| `check-names` … `check-mirror` (18)                                         | `check --only <rule>`; the battery: `check` |
| `generate-exports`, `generate-cleartext-config`, `generate-chain-constants` | `generate --only <generator>`               |
| `sync-vendored [--check]`                                                   | `sync vendored [--check]`                   |
| `sync-fhevm-chains`                                                         | `sync fhevm-chains`                         |
| `install-forge-dependencies`, `clean-forge-dependencies`                    | `forge install`, `forge clean`              |
| `test-consumer`, `test-consumer-regenerate-package-lock`                    | `consumer test`, `consumer regenerate-lock` |
| `list-versions`                                                             | `version list`                              |
| `list-packages`                                                             | `list packages`                             |
| `pack-tarball`, `sh-completion`                                             | unchanged                                   |

Package scripts keep their own names (`check:vendored-origin`, `check:mirror`, `pack:tarball`,
`test:consumer`, `generate:*`) — the rules that require them do not change; only the command each one
runs does.

## Migration, three commits

1. `refactor(fhevm-npm): register the grouped command line, old names as deprecated aliases` — parser,
   dispatcher, completion renderer (one level of nesting), tests. Aliases print
   `fhevm-npm check-names is now fhevm-npm check --only names` and run.
2. `chore(sdk): move every fhevm-npm call site to the grouped commands` — Makefile (the 18-line battery
   becomes `check`), the 21 package-script calls, README, GUIDE, RULES examples.
3. `refactor(fhevm-npm): drop the deprecated command aliases` — once nothing in the workspace calls them.

## Open questions

- `--only` by rule id (`4.2.1`) or by name (`dependencies`)? Both are printed today; accepting both is cheap.
- Should `check` exit non-zero on the first failing rule (fail fast) or run everything (the report is the
  point)? Proposal: run everything; `--fail-fast` for interactive use.
- Does `list packages` stay a group of one, or fold into `version list --all`?

## Alternative B: nouns first

The proposal above optimizes the commands people type most often: `check` and `generate` are short,
and the less common operations sit in small verb-first groups. An alternative is to make every
top-level command a thing that fhevm-npm manages, followed by the action to perform on it. It is a
little more verbose, but the grammar never changes as each area grows.

```
fhevm-npm rule run [rule…] [--fail-fast]                    no rule = the complete battery
fhevm-npm rule list [--json]
fhevm-npm rule explain <rule>
fhevm-npm generator run [generator…]                        no generator = generate everything
fhevm-npm generator check [generator…]
fhevm-npm generator list
fhevm-npm vendored sync [package]
fhevm-npm vendored check [package]
fhevm-npm chains sync [--commit <sha> | --latest]
fhevm-npm chains check
fhevm-npm forge install [package]
fhevm-npm forge clean [package] [--dry-run] [--force]
fhevm-npm consumer test [package] [--run] [--ci] [--build-linked-dependencies] [--test-file <path>] [-o <dir>] [-f] [--list]
fhevm-npm consumer lock [package]
fhevm-npm package list
fhevm-npm package versions [--check-npmjs] [--json]
fhevm-npm package version-check [package...] [--check-npmjs]
fhevm-npm package version-apply [--check-npmjs] [--dry-run]
fhevm-npm package pack [package] [-o <dir>] [--clean]
fhevm-npm completion <zsh|bash>
```

Selection is positional instead of an `--only` option. The common battery is consequently one word
longer, but selecting several rules reads naturally and completion knows exactly what belongs in each
position.

```
$ fhevm-npm rule run
  package-json-order  failed   1 violation
  dependencies        failed   1 violation
  vendored-origin     passed
  mirror              passed
  …

❌ 2 violations in 2 of 23 rules · 24 packages checked
```

```
$ fhevm-npm rule run vendored-origin mirror
  vendored-origin     passed
  mirror              passed

✅ 2 rules passed · 24 packages checked
```

Rules become inspectable objects rather than names that exist only in `--list` output. `explain`
connects the short terminal diagnostic to the longer policy and its remediation without making every
failure noisy.

```
$ fhevm-npm rule explain 4.2.1
dependencies (4.2.1)

Workspace packages must declare every imported root-pinned dependency. Sibling payloads use the
range prescribed by the manifest; generated consumer projects use their declared file: link.

Run:  fhevm-npm rule run dependencies
Docs: sdk/fhevm-npm/FHEVM_NPM_RULES.md#421
```

Generators have the same shape as rules. `run` writes; `check` performs the corresponding freshness
checks without writing. The names and summary stay identical between the two actions.

```
$ fhevm-npm generator check exports cleartext-config
✅ exports            fresh · 4 files
❌ cleartext-config   stale · 2 files

Run `fhevm-npm generator run cleartext-config` to update them.
```

Calling a noun without an action shows only that noun's surface:

```
$ fhevm-npm generator
Usage: fhevm-npm generator <action>

Actions:
  run [generator…]    write generated files
  check [generator…]  report stale generated files
  list                show registered generators
```

### Trade-off against the primary proposal

| Primary proposal                           | Alternative B                                   |
| ------------------------------------------ | ----------------------------------------------- |
| Short hot path: `fhevm-npm check`          | Uniform hot path: `fhevm-npm rule run`          |
| Selection is a flag: `check --only a,b`    | Selection is positional: `rule run a b`         |
| Verbs are prominent                        | Managed concepts are prominent                  |
| Related commands may use different grammar | Every namespace uses noun, then action          |
| Smaller migration and completion change    | Better room for `list`, `explain`, and `--json` |

Choose the primary proposal if the CLI is principally a pre-build battery and its shortest command
should express that. Choose Alternative B if fhevm-npm is expected to become an operator's toolbox
whose rules, generators and packages will each gain more introspection and actions.
