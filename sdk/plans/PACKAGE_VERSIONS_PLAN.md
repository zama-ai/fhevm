# Managing the versions of every package

Version: **0.10** (2026-09-04). 0.10 makes direct editing of `sdk/versions.json` the normal operator
workflow and removes `version set`. The version commands only prepare and validate the repository for
`npm publish`; they never publish or mutate the registry. A proposal, not yet executed.

## Goals

- We must have one centralized authority for every managed package version; no package-local file may
  independently define or override a version.
- A manually added package cannot bypass that authority: fhevm-npm must detect both a package missing
  from the manifest and a published manifest entry missing from `sdk/versions.json`.
- A maintainer declares package versions in one central model, never by visiting package directories.
- `sdk/versions.json` is the only authoritative store. Versions in `package.json`, dependency pins,
  lockfiles and runtime faces are derived state.
- One invocation may change one payload or a coordinated set. Both paths validate and reconcile the
  complete version graph; a batch is not implemented as independent public operations.
- The pre-build battery and CI reject every divergence from the central model.
- Version tooling only prepares and validates repository state for publication. It never commits,
  creates Git or npm tags, pushes, authenticates to npm, or publishes; CI owns `npm publish`.
- `version apply` accepts a clean Git tree or exactly one pre-existing unstaged change:
  `sdk/versions.json`. Every other staged, unstaged or untracked path is rejected, so all derived
  changes remain attributable to the operation and reviewable with ordinary Git tools.
- Safe defaults must not prevent legitimate release workflows. Optional behavior is explicit and
  narrow; no flag may bypass central authority, coverage, SemVer, graph-policy or clean-tree checks.

## Where versions live today

`fhevm-npm list-versions --check-npmjs` (landed 2026-09-03) already reads the payloads and npmjs:

| payload                                 | version | distribution | npmjs                                           |
| --------------------------------------- | ------- | ------------ | ----------------------------------------------- |
| `hardhat/v2/fhevm-hardhat-template/pkg` | 0.4.2   | mirror       | —                                               |
| `hardhat/v2/plugin/pkg`                 | 0.4.2   | npm          | published; registry repository is `fhevm-mocks` |
| `hardhat/v3/plugin/pkg`                 | 0.13.0  | npm          | never published                                 |
| `host-contracts-cleartext/v12/pkg`      | 0.12.0  | npm + mirror | never published under this name                 |
| `host-contracts-cleartext/v13/pkg`      | 0.13.0  | npm + mirror | never published under this name                 |

The repeated machine-owned values are payload versions, dependency pins, owning-workspace lockfile
entries, isolated-consumer lockfiles, generated runtime version faces and tarball names. Human prose,
protocol directories such as `v0.13.0`, Solidity remapping prefixes and fixture data are not version
pins merely because they contain the same digits; propagation must never be a global replacement.

## Decision: one authoritative version graph

`sdk/versions.json` is committed and contains exactly one entry for every `kind: published` package in
`sdk/npm-manifest.json`:

```json
{
  "schemaVersion": 1,
  "packages": {
    "./hardhat/v2/fhevm-hardhat-template/pkg": "0.4.2",
    "./hardhat/v2/plugin/pkg": "0.4.2",
    "./hardhat/v3/plugin/pkg": "0.13.0",
    "./host-contracts-cleartext/v12/pkg": "0.12.0",
    "./host-contracts-cleartext/v13/pkg": "0.13.0"
  }
}
```

Manifest keys are the identities. Package names are not unique: the v12 and v13 host-contract
payloads deliberately share an npm name. Entries appear in manifest order for stable diffs.

Maintainers edit versions only in `sdk/versions.json`; they never hand-edit derived `package.json`
versions. `fhevm-npm version apply` treats the current central file as the complete candidate graph,
validates it, and reconciles every affected derived field. It accepts either a single unstaged edit to
`versions.json` or a clean tree containing an already committed central change, typically after a
merge or rebase.

Coverage is bidirectional. The existing manifest-coverage rule reports a package directory that was
added without an `npm-manifest.json` entry. Once registered as `kind: published`, the version-coverage
rule requires exactly one matching `versions.json` key. A missing central entry is never inferred from
the new package's local `package.json`, because doing so would let derived state become authoritative.
It is initialized explicitly by adding its initial version to `versions.json`. An extra central key
whose manifest package no longer exists is also a violation and is never removed automatically.

## Why `npm version` is not the orchestrator

`npm version` is package-local. It writes one package, may update a nearby lockfile, runs lifecycle
hooks by default and, in Git, normally creates a commit and tag. When invoked inside a workspace member,
npm can implicitly enter workspace mode and reify the installation root. It has no model of the other
payloads, cross-package pins, shared lockfiles, runtime faces or an all-or-nothing multi-package bump.
It is also not transactional: a lifecycle or later orchestration failure can leave earlier writes in
place.

Flags such as `--workspaces=false --no-git-tag-version --ignore-scripts` reduce those side effects, but
the operation remains one package mutation at a time. Wrapping that command in a loop would make
`package.json` the effective authority during the update and recreate the synchronization problem this
plan is meant to remove.

`npm version` can update a package-local lockfile or, in workspace mode, cause npm to update the owning
workspace lock. That timing is wrong for this graph: it happens while changing one package, before
fhevm-npm has reconciled other selected payloads and dependent manifests. For example, the hardhat v3
lock could be regenerated while `e2e/package.json` still pins the previous plugin version. The final
lockfiles would need another regeneration anyway.

Therefore fhevm-npm does **not** invoke `npm version`. It uses a strict SemVer library for parsing and
comparison, updates only schema-selected JSON fields while preserving repository formatting, and runs
npm only for designated lockfile regeneration and pack/consumer verification. Plain `npm version` is
not a supported maintenance path; if someone uses it, `fhevm-npm version check` fails because the
derived package version no longer matches `versions.json`.

The behavior that motivates this decision is documented by
[npm-version](https://docs.npmjs.com/cli/v11/commands/npm-version/). The repository does not currently
pin npm, so lockfile integration tests must either run against every supported npm major or the
migration must declare one supported major.

## Manifest-owned propagation

The manifest describes where each central payload version is allowed to propagate:

- the payload `package.json` version field;
- owning development packages whose version deliberately follows the payload;
- workspace and consumer dependency fields that pin the payload by name;
- owning installation roots and isolated consumers whose lockfiles must be regenerated;
- an optional runtime version face: template, output path and required export path;
- an optional protocol line for payloads whose release policy deliberately follows one.

Every destination is an exact parsed field or generated file. The reconciler does not replace
version-shaped strings in source, documentation, Solidity paths, manifest notes or arbitrary lockfile
text. Runtime version faces are declared only for published JavaScript APIs that expose one, not
blindly at `src/_version.ts` for Solidity or mirror-only payloads with another layout.

Protocol coupling is opt-in policy. The hardhat v3 plugin may follow protocol line `0.13`; the v2
plugin's independent `0.4.x` release line must not be rejected for failing to equal protocol `0.13.x`.

## Operator interface

```
fhevm-npm version list [--check-npmjs] [--json]
fhevm-npm version check [payload...] [--check-npmjs]
fhevm-npm version apply [--check-npmjs] [--dry-run]
```

### Strict invariants and explicit flexibility

The operator may change one or many entries directly in `sdk/versions.json`. Stable and prerelease
versions use the same canonical SemVer representation:

```json
{
  "packages": {
    "./hardhat/v3/plugin/pkg": "0.14.0-alpha.0"
  }
}
```

This central edit is the explicit release intent, so no separate `--prerelease` flag is necessary.
`version apply --dry-run` validates the complete edited graph and previews every derived change without
writing. `version apply` performs the same validation and then reconciles the derived files.

The following invariants have no override flag:

- before `version apply`, the only permitted pre-existing change is an unstaged `sdk/versions.json`;
- every target is canonical SemVer and is strictly greater than its current central version;
- manifest and central coverage are complete;
- protocol-line and cross-package policies hold for the final graph;
- `--check-npmjs` proves every changed exact `name@version` is unpublished;
- only planned derived fields and lockfiles change.

In particular, there is no general `--force`, `--allow-downgrade` or `--allow-dirty` escape hatch.
Flexibility comes from editing one or many central entries, choosing stable or prerelease SemVer,
choosing whether to query npmjs, and previewing with `--dry-run`—not from disabling integrity checks.

The prerelease identifier is repository version data; the npm dist-tag is a CI publication choice.
For example, fhevm-npm prepares `0.14.0-alpha.0`, while CI later runs `npm publish --tag alpha`.
No fhevm-npm version command accepts npm credentials or invokes `npm publish`.

### Simple operations for each command

`fhevm-npm version list [--check-npmjs] [--json]`

1. Load `sdk/npm-manifest.json`.
2. Load `sdk/versions.json`.
3. Match every central key to a published payload.
4. Read each derived `package.json` version.
5. With `--check-npmjs`, query npmjs for each npm-distributed package.
6. Print the central, package and optional npmjs versions.
7. Exit without changing files.

`fhevm-npm version check [payload...] [--check-npmjs]`

1. Load `sdk/npm-manifest.json`.
2. Load `sdk/versions.json`.
3. Check that every published payload has one central version.
4. Resolve each optional payload selector, such as `hardhat/v3/plugin/pkg`.
5. Validate every central version as strict SemVer.
6. Compare derived `package.json` versions with the central versions.
7. Compare dependency pins and lockfiles with the central versions.
8. Compare runtime version faces with the central versions.
9. Validate protocol-line and cross-package policies.
10. With `--check-npmjs`, require at least one payload selector and verify that each selected exact
    `name@version` intended for npm distribution is not already published. Registry access is
    read-only.
11. Print violations or one success summary. When only `versions.json` is modified and the central
    graph is valid, distinguish expected pending derived mismatches and suggest `version apply
--dry-run`.
12. Exit without changing files.

`fhevm-npm version apply [--check-npmjs] [--dry-run]`

1. Inspect `git status --porcelain`. Accept either a clean tree or exactly one unstaged modification to
   `sdk/versions.json`. Reject staged changes, untracked files and every other modified path.
2. Load `sdk/npm-manifest.json` and the current authoritative `sdk/versions.json`.
3. Check that the complete central graph has exactly one canonical SemVer for every published payload
   and no extra key. Stable versions and prereleases such as `0.14.0-alpha.0` are both valid.
4. If `versions.json` differs from `HEAD`, identify every edited entry. For an existing entry, require
   its new version to be strictly greater than the version in `HEAD`. Allow a new entry only for a
   published manifest payload that had no central entry in `HEAD`.
5. Validate protocol-line and cross-package policies against the complete final graph.
6. With `--check-npmjs`, query every changed npm-distributed exact `name@version` and require it to be
   unpublished. Registry access is read-only.
7. Compute in memory every derived field and file that must change. For a Hardhat v3 plugin update,
   the plan may contain:
   - `hardhat/v3/plugin/pkg/package.json`: the package version;
   - `hardhat/v3/e2e/package.json`: the `@fhevm/hardhat-plugin-v3` dependency pin;
   - `hardhat/v3/package-lock.json`: the owning workspace lockfile regeneration target;
   - `hardhat/v3/plugin/pkg/src/_version.ts`: the generated runtime version.

   This step calculates expected contents and affected paths only; it does not write anything.

8. With `--dry-run`, print the central transitions and complete reconciliation plan, then exit without
   changing any file itself or running npm; the operator's existing `versions.json` edit remains.
9. Reconcile `package.json` versions, dependency pins and runtime faces from `versions.json`.
10. Before running npm, verify that every planned lockfile exists at its manifest-declared installation
    root and is still unchanged from `HEAD`.
11. For the Hardhat v3 installation root, run exactly:

```bash
npm --prefix sdk/hardhat/v3 install \
  --package-lock-only \
  --ignore-scripts \
  --no-audit \
  --no-fund
```

Run the equivalent command once for every other affected installation root, using its manifest
path as `--prefix`.

12. After running npm, require the set of modified `package-lock.json` files to equal the planned set.
    Parse each modified lockfile, verify its package versions and dependency pins against the central
    graph and reconciled manifests, and reject unrelated lockfile changes.
13. Run the global version and generated-file checks.
14. Print the central transitions and changed files. On failure, report the failed phase and leave the
    Git diff for inspection.

`version apply` never reads a new version from a package-local file and never copies derived state back
into the central graph. Editing one entry and editing several entries follow exactly the same workflow;
the command validates the complete final graph before performing any derived write.

## One controlled central-file operation over the complete graph

`version apply`, including `--dry-run`, performs these phases:

1. **Classify Git state.** Run `git status --porcelain`. Accept a clean tree for reconciliation of an
   already committed central change. For a new version change, accept exactly one unstaged modification
   to `sdk/versions.json`. Reject staged changes, untracked files and every other modified path.
   `version list` and `version check` remain usable with arbitrary worktree changes because they are
   read-only.
2. **Load the candidate graph.** Load the manifest and current `versions.json` once. The current central
   file—not a package-local file or command-line assignment—is the complete candidate graph.
3. **Validate the graph and transition.** Prove that the manifest and central file contain exactly the
   same published payload keys and that every value is canonical SemVer. When the central file is the
   permitted uncommitted change, compare it with `HEAD`, require every changed existing version to
   increase, and allow a new key only for a newly registered published payload. Validate all
   protocol-line and cross-package policies against the complete final graph.
4. **Plan without writes.** From the current central graph and manifest metadata, render every affected
   `package.json` field, dependency pin and runtime face in memory. Calculate the union of managed
   files and deduplicate owning workspace roots and consumer locks. Detect conflicting destination
   writes before mutation.
5. **Registry guard.** With `--check-npmjs`, query every changed `name@targetVersion` whose distribution
   includes npm. Refuse the complete operation if any target exists. A registry failure is an error,
   not "unpublished"; mirror-only payloads skip the query. With a clean tree, check the payloads whose
   committed central versions still require reconciliation.
6. **Preview.** Print every central transition, policy-derived transition and deduplicated destination. With
   `--dry-run`, stop here without writing a file or running npm.
7. **Reconcile derived state.** Apply all precomputed JSON field updates and runtime faces from the
   central graph. No destination becomes an authority or an input to another destination.
8. **Check lockfiles before npm.** Require every planned lockfile to exist at its manifest-declared
   installation root and still match `HEAD`. Read the planned lockfile paths and exact `HEAD` contents
   as a read-only validation baseline for the post-npm comparison.
9. **Regenerate lockfiles.** After every manifest has its final derived values, regenerate each
   affected owning-workspace lockfile once by passing that installation root explicitly to npm. For
   the Hardhat v3 workspace, the exact command is:

   ```bash
   npm --prefix sdk/hardhat/v3 install \
     --package-lock-only \
     --ignore-scripts \
     --no-audit \
     --no-fund
   ```

   `--package-lock-only` asks npm to update `package-lock.json` from the complete final manifest graph
   without reconciling `node_modules`. Isolated consumers use their existing designated lock
   regeneration command. Never hand-edit lockfile internals.

10. **Check lockfiles after npm.** Collect the modified `package-lock.json` paths from Git and require
    that set to equal the planned lockfile set: every expected lockfile changed and no unexpected
    lockfile changed. Parse each result and prove that its package versions and dependency pins match
    the central graph and reconciled manifests. Compare its exact and parsed before/after content and
    reject unrelated dependency, integrity, metadata or formatting churn.

11. **Postflight.** Run global `version check`, generated-file checks and affected package checks.
    Print the exact transitions and files for review and commit.

On failure, report the phase and affected files, then leave the Git diff untouched for inspection. The
operation does not attempt rollback. Because the only permitted input diff was `versions.json`, the
operator can distinguish that central edit from every derived change. A subsequent `version apply`
refuses to start until all partial derived changes are resolved and only the central edit remains. The
tool never runs destructive Git commands and never commits for the maintainer.

## Look and feel

Single payload:

```
$ $EDITOR sdk/versions.json
# Change ./hardhat/v3/plugin/pkg from 0.13.0 to 0.13.1.

$ fhevm-npm version apply --check-npmjs --dry-run
central edit
  @fhevm/hardhat-plugin-v3  0.13.0 → 0.13.1

would reconcile
  hardhat/v3/plugin/pkg/package.json
  hardhat/v3/e2e/package.json
  hardhat/v3/package-lock.json
  hardhat/v3/plugin/pkg/src/_version.ts

$ fhevm-npm version apply --check-npmjs
✅ 1 central edit; 4 derived files reconciled; global version checks passed.

```

Coordinated batch:

```
$ $EDITOR sdk/versions.json
# Change both central entries in the same JSON edit.

$ fhevm-npm version apply --check-npmjs --dry-run
central edits
  @fhevm/hardhat-plugin-v3          0.13.0 → 0.13.1
  @fhevm/host-contracts-cleartext   0.13.0 → 0.13.2

would reconcile
  hardhat/v3/plugin/pkg/package.json
  hardhat/v3/e2e/package.json
  hardhat/v3/package-lock.json
  host-contracts-cleartext/v13/pkg/package.json
  host-contracts-cleartext/v13/package.json
  package-lock.json

$ fhevm-npm version apply --check-npmjs
✅ 2 central edits; 6 derived files reconciled; global version checks passed.
```

The important UX distinction is "central edits" followed by "reconciled files". The operator changes
versions once in the authoritative JSON file; the tool reports and applies the derived consequences.

New-package coverage failure:

```
$ fhevm-npm version check
❌ [version-coverage] ./new-package/pkg: published payload is missing from sdk/versions.json
   Add its initial version explicitly to sdk/versions.json, then run `fhevm-npm version apply`.
```

If the directory has not been registered yet, the earlier manifest check reports that first:

```
❌ [manifest-coverage] ./new-package/pkg/package.json: package is missing from sdk/npm-manifest.json
```

## Checks and invariants

`fhevm-npm version check [payload...]` is read-only. With no selectors it checks the complete graph;
selectors filter diagnostics but never weaken global graph validation. It enforces:

1. Manifest coverage reports every package directory not registered in `npm-manifest.json`; version
   coverage reports every `kind: published` manifest key missing from `versions.json`, every extra
   central key, and unstable key order.
2. Every central value is strict canonical SemVer.
3. Every payload and declared owner `package.json` version equals its central value.
4. Every manifest-declared dependency pin equals its target's central version or declared `file:`
   policy; undeclared occurrences are reported, never silently rewritten.
5. Owning-workspace and isolated-consumer lockfiles agree with their manifests and linked payloads.
6. Every declared runtime version face is fresh and exported from the published surface.
7. Every declared protocol-line policy holds; packages without that policy remain independent.
8. Tarball names derive from the central name/version pair. Stale tarballs are reported or removed only
   through the existing explicit tarball-clean operation.
9. Local package name/repository metadata matches the manifest. Registry metadata is reported
   separately and becomes a failure only under explicit policy because published metadata is immutable.

The complete local check joins the pre-build battery. Registry checks remain a separate CI or release
gate so normal offline builds are deterministic.

## CI publication boundary

The terminal responsibility of `fhevm-npm version` is a repository that is ready to publish. Before
CI publishes a selected payload, `version check <payload> --check-npmjs` proves that its central
version, package manifest, dependency pins, lockfiles and runtime face agree and that the exact
`name@version` is not already on npmjs. This check may read the registry but never writes to it.

Authentication, provenance, access control and npm dist-tags belong exclusively to the CI action.
There is no fhevm-npm command that publishes a package or mutates an npm dist-tag. After the repository
checks, build, tests and package inspection pass, CI invokes npm directly from the published payload
directory:

```bash
# Stable release: npm uses the default "latest" dist-tag.
npm publish

# Prerelease: the CI workflow must choose an explicit non-latest dist-tag.
npm publish --tag alpha
```

The CI workflow, not fhevm-npm, is responsible for preventing a prerelease from being published as
`latest`. The SemVer identifier and npm dist-tag remain separate: fhevm-npm may prepare
`0.14.0-alpha.0`, while CI chooses a channel such as `alpha`, `next` or `canary`.

## Migration

1. **Bootstrap the authority once.** Generate the initial `versions.json` from today's payload
   `package.json` values, review it, and commit it with schema support. This is the only migration step
   that reads package versions into the central file; after this commit the data flow reverses.
2. **Make divergence visible.** Add read-only graph, package, pin, lock and runtime-face checks to the
   pre-build battery before introducing a writer.
3. **Add centralized writing.** Implement `version apply`, `--dry-run`, registry guards, central-file
   diff validation and integration tests for single edits, coordinated edits, invalid graph
   transitions, stable/prerelease transitions and shared lock roots.
4. **Add reconciliation.** Implement manifest-owned destinations and lockfile
   regeneration; test failures after partial mutation, verify that the Git diff remains inspectable,
   and prove derived state can never update the authority.
5. **Integrate CI publication.** Run the read-only version checks, build, tests and package inspection,
   then let the CI action invoke `npm publish` directly with the appropriate dist-tag.

During the CLI migration in `FHEVM_NPM_CLI_PLAN.md`, flat aliases may exist temporarily, but new
documentation uses `version list`, `version check` and `version apply`.

## Open questions

- Should a bump require a matching CHANGELOG entry, or create a structured empty entry to fill?
- Which published JavaScript APIs need a runtime version face, and what output/export path should each
  declare?
- Should lockfile regeneration allow network access, or require a complete npm cache and run offline?

## Day-to-day use

Start by asking the central authority what the workspace expects and whether every derived file agrees:

```
$ fhevm-npm version list --check-npmjs
payload                                  central   package   npmjs
hardhat/v2/plugin/pkg                    0.4.2     0.4.2     0.4.2
hardhat/v3/plugin/pkg                    0.13.0    0.13.0    —
host-contracts-cleartext/v13/pkg         0.13.0    0.13.0    —

$ fhevm-npm version check
✅ 5 central versions; package manifests, pins, lockfiles and runtime faces agree.
```

For an ordinary single-package release, edit the one authoritative value, preview the consequences,
then apply them:

```bash
$ $EDITOR sdk/versions.json
# Change "./hardhat/v3/plugin/pkg": "0.13.0" to "0.13.1".

$ fhevm-npm version apply --check-npmjs --dry-run
central edit
  @fhevm/hardhat-plugin-v3  0.13.0 → 0.13.1

would reconcile 4 derived files; no additional files written

$ fhevm-npm version apply --check-npmjs
✅ 1 central edit; 4 derived files reconciled; global version checks passed.

$ git diff -- sdk/versions.json sdk/hardhat/v3
# Review, test and commit through the normal repository workflow.
```

For a coordinated release, edit every target in the same JSON file. fhevm-npm validates the complete
final graph and regenerates shared lockfiles once:

```bash
$ $EDITOR sdk/versions.json
# Change hardhat/v3/plugin/pkg to 0.13.2 and host-contracts-cleartext/v13/pkg to 0.13.2.

$ fhevm-npm version apply --check-npmjs --dry-run
$ fhevm-npm version apply --check-npmjs
✅ 2 central edits; 6 derived files reconciled; global version checks passed.
```

For an alpha, put the exact prerelease SemVer in the same central file; no special fhevm-npm flag is
needed because the JSON edit itself is explicit release intent:

```bash
$ $EDITOR sdk/versions.json
# Change "./hardhat/v3/plugin/pkg": "0.13.2" to "0.14.0-alpha.0".

$ fhevm-npm version apply --check-npmjs --dry-run
$ fhevm-npm version apply --check-npmjs
✅ 1 central edit; prerelease 0.14.0-alpha.0 and derived files are consistent.

$ git add sdk/versions.json sdk/hardhat/v3
$ git commit -m "chore(hardhat-v3): prepare 0.14.0-alpha.0"

$ fhevm-npm version check hardhat/v3/plugin/pkg --check-npmjs
✅ @fhevm/hardhat-plugin-v3@0.14.0-alpha.0 is consistent and not yet published.

# The CI action now runs this from sdk/hardhat/v3/plugin/pkg:
$ npm publish --tag alpha
```

fhevm-npm stops after readiness validation; only the CI action has npm publication credentials and
chooses the `alpha` dist-tag.

When adding a package, register and commit it in `npm-manifest.json`; the normal check then forces
explicit initialization in the central authority. Next, add the initial version to `versions.json`
and use the same preview/apply workflow:

```bash
$ fhevm-npm version check
❌ [version-coverage] ./new-package/pkg: published payload is missing from sdk/versions.json

$ $EDITOR sdk/versions.json
# Add "./new-package/pkg": "0.1.0" in manifest order.

$ fhevm-npm version apply --dry-run
$ fhevm-npm version apply
✅ 1 central entry added; new payload derived files reconciled.
```

After a merge or rebase, check before doing new version work. If a committed central change arrived
without fresh derived files, reconcile outward from `versions.json`; never copy a package-local version
back into it:

```
$ fhevm-npm version check
❌ [version-package] ./hardhat/v3/plugin/pkg: package.json has 0.13.0; central version is 0.13.1

$ fhevm-npm version apply --dry-run
$ fhevm-npm version apply
✅ derived package manifests, pins, lockfiles and runtime faces reconciled from sdk/versions.json.
```

If an operation fails after writing derived files, it reports the failed phase and leaves its changes
visible. The next apply refuses to start until the operator has inspected the diff and restored the
state in which only `sdk/versions.json` is modified:

```
$ fhevm-npm version apply
❌ lockfile regeneration failed; partial changes remain for inspection

$ git status --short
 M sdk/versions.json
 M sdk/hardhat/v3/plugin/pkg/package.json
 M sdk/hardhat/v3/e2e/package.json

$ fhevm-npm version apply
❌ only sdk/versions.json may be modified before apply; resolve the partial derived changes first
```
