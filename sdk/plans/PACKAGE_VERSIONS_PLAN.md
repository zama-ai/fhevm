# Managing the versions of every package

Version: **0.2** (2026-09-03). 0.2: bumps go through `npm version`. A proposal, not yet executed: a list of one-line features, to be picked,
ordered and turned into fhevm-npm commands and rules.

## Goals

- Minimize error risk: a version is written ONCE and every other occurrence is derived or checked.
- Checking: every derived occurrence is gated by `fhevm-npm` in the pre-build battery and in CI.
- Centralized: one file in the workspace answers "what version is each payload", and one command reads
  it back from the payloads and from npmjs.
- Every npmjs-published payload carries a `_version.ts` at its source root, generated like js-sdk's
  `src/core/_version.ts` (`export const version`, `export const <name>` from `package.json`), so the
  running code can say which version it is.
- Bumps go through `npm version`, never a hand edit: npm writes `package.json`, validates the semver, and
  runs the payload's `version` lifecycle script, which is where `_version.ts` is regenerated (js-sdk's
  `"version": "node ./scripts/generate-version.cjs"` is exactly this).

## Where versions live today

`fhevm-npm list-versions --check-npmjs` (landed 2026-09-03) already reads the payloads and npmjs:

| payload                                 | version | distribution | npmjs                                           |
| --------------------------------------- | ------- | ------------ | ----------------------------------------------- |
| `hardhat/v2/fhevm-hardhat-template/pkg` | 0.4.2   | mirror       | —                                               |
| `hardhat/v2/plugin/pkg`                 | 0.4.2   | npm          | published; registry repository is `fhevm-mocks` |
| `hardhat/v3/plugin/pkg`                 | 0.13.0  | npm          | never published                                 |
| `host-contracts-cleartext/v12/pkg`      | 0.12.0  | npm + mirror | never published under this name                 |
| `host-contracts-cleartext/v13/pkg`      | 0.13.0  | npm + mirror | never published under this name                 |

Besides each payload's `package.json`, a version is repeated in: the e2e `devDependencies` pins
(`hardhat/v2/e2e`, `hardhat/v3/e2e`), the consumer-fixture `file:` links and their lockfiles, the
`npm-manifest.json` notes, the tarball names, and the plugins' `npmPackage` ids. Nothing ties them
together yet; the hardhat 3 rename to `@fhevm/hardhat-plugin-v3` `0.13.0` was done by hand across all of them.

## Proposed features (one line each)

1. `sdk/versions.json` — the single source of truth: `{ "<payload key>": "x.y.z" }` for every `published` manifest entry, nothing else.
2. `fhevm-npm check-versions` — every payload's `package.json` version equals `versions.json`; a payload missing from it, or an entry without a payload, is a violation.
3. `fhevm-npm check-versions` — every workspace dependency on a payload (e2e pins, dev owners, sibling payloads) is exactly the payload's version, or a `file:` link.
4. `fhevm-npm generate-version` — renders `<payload>/src/_version.ts` from a committed `_version.ts.template` with `@VERSION@` and `@NAME@`, as js-sdk does; `--check` compares.
5. Rule: a `published` payload must contain `src/_version.ts` and export it; `check-versions` fails without it and when it lags `package.json`.
6. `make generate` runs `generate-version` before `sync-vendored`; `make check-generated` proves the tree is spotless afterwards.
7. `fhevm-npm set-version <payload> <x.y.z>` — the ONLY way to bump: runs `npm version <x.y.z> --no-git-tag-version` in the payload (npm writes `package.json` and fires its `version` script, which regenerates `_version.ts`), then writes `versions.json`, every pin and the consumer lockfiles, and prints the diff to commit.
8. `set-version` refuses a bump that is not strictly greater than the current version and, with `--check-npmjs`, one already published on npmjs; `npm version` itself rejects anything that is not valid semver.
9. Rule: every `published` payload declares a `version` script that regenerates `_version.ts`, so a bare `npm version` in a payload directory can never leave the file stale; `--no-git-tag-version` is the convention because payloads are workspace members, and the tag belongs to the release commit, not the bump.
10. `list-versions --check-npmjs` in CI (informational) — a table of what npmjs holds vs the tree, with the publication's `gitHead` and repository.
11. Rule: the registry's `repository` for the latest published version must equal the payload's `package.json` repository, else a violation (today: the v2 plugin says `fhevm-mocks`).
12. Rule: a version bump commit touches only the files `set-version` writes; `check-commit-scope` learns the shape so a hand edit stands out.
13. Tarball names are derived (`<name>-<version>.tgz`), and `pack-tarball` refuses a stale tarball of another version in `tarballs/`.
14. Protocol coupling: a plugin version's `major.minor` must equal the `@fhevm/solidity` and `@fhevm/host-contracts-cleartext` minor it targets (`0.13.x` ↔ `0.13.x`), checked, so the "version carries the protocol line" convention cannot drift.
15. `publish-check <payload>` — the pre-publish gate: `check-versions`, `_version.ts` fresh, `--check-npmjs` says unpublished, tarball packs, consumer fixture runs from the lockfile; publishing is still a human command.
16. Every check above is a `check-*` in `check-npm-cli-pre-build`, so `make check` catches a stray version before CI does.

## Open questions

- One `versions.json` per workspace, or per installation root (`hardhat/v2`, `hardhat/v3`) — the first
  keeps one file, the second matches how installs are scoped.
- `_version.ts` for mirror-only payloads (the template) — required for symmetry, or npm-only as the goal
  states?
- Whether `set-version` also writes the CHANGELOG entry, or only asserts one exists.
