# Package script verbs — the orchestrator contract

Status: IMPLEMENTED (v4) — all migration steps executed, plus one post-implementation rename:
the compile verb was `build` through v3; `build` is now the everyday sweep
(`fmt:check && lint && compile`), enforced by check-scripts wherever it exists on in-contract
kinds. Lifecycle mentions of "build" as the artifact step read as `compile`. `build:forge` is
`compile:forge`, `build:pkg:ts:*` are `compile:pkg:ts:*`, and the Makefile's `build-*` targets are
`compile-*` (with `compile-package` as the fhevm-npm bridge).
Scope: every package in the sdk workspace; `check-scripts` enforcement; the Makefile rewrite that follows.

## Goal

The Makefile speaks to packages only through a small set of high-level verbs. Everything else —
phases, leaf generators, per-tool scripts — is package-internal. The Makefile keeps only what is
genuinely workspace-level: the graph between packages, the workspace tools (`sync-vendored`, the
fhevm-npm check fleet, consumer rehearsal), the `build-package` bridge that `fhevm-npm
test-consumer` drives (`runMakeBuildPackage`), the formatting of the two non-member directories
(`fhevm-npm` via --prefix, `scripts/` via npx prettier — omitting them once let unformatted files
reach ci), and the regeneration gate's assert.

## Lifecycle (per-package)

```
clean → [sync-common-vendored] → generate → fmt:check → lint → build → check → test
```

- The order is a PER-PACKAGE contract, not a workspace-wide phase wall. Type-aware lint resolves
  dependency declarations through built pkg output (plugin lint reads v13's `pkg/ts/_types*`,
  template lint needs the plugin built, e2e lint needs typechain), so the Makefile keeps its
  lint → dependency-build cross-package edges. "All lint before all build" is unsatisfiable here,
  as the Makefile header already states.
- `fmt:check` after `generate` gates generator output formatting, not just human edits.
- `check` after `build`: its prerequisite is that **`generate` and `build` have run in this
  worktree** — contract-sizes reads forge `./out` (produced by `generate`'s internal forge step),
  publint reads `pkg/ts/_*` (produced by `build`). No verb exists to provide `./out` alone: once
  `generate` has run, it exists by construction, and a worktree where it never ran fails loudly,
  not silently. The Makefile never runs `generate` implicitly (it mutates tracked files).
- `test` is self-sufficient on a generated-and-built tree; packages that need `./out` order
  `test:forge` first so a fresh clone works.
- CI is the same pipeline with one substitution: `generate` becomes the regeneration gate below.

## The regeneration gate (CI)

One spelling, sync inside the diff window:

```
[precondition: spotless worktree]
clean:generated (per package) → sync-common-vendored → generate (per package)
→ assert `git status --porcelain` is empty
```

- `git status --porcelain`, NOT `git diff`: a generator that starts emitting a NEW file must fail
  the gate; `git diff` ignores untracked files. (The current Makefile already learned this.)
- The gate starts by deleting committed files, so it REQUIRES a clean worktree; the target keeps
  the existing recoverability guard and states the precondition loudly. The old `check-generated`
  worked on a dirty tree via snapshots — that property is deliberately traded for the stronger
  dropped-emitter detection (today's CI never runs `clean-generated`, so this is new coverage).
- The gate subsumes `check:exports` and `check:cleartext-config` (they are `--check` modes of
  generators the gate runs for real). The leaf `--check` scripts stay as package-internal dev
  conveniences with better per-file diagnostics; no orchestrator calls them.
- Subsumption holds ONLY with complete `clean:generated` coverage — see the v13 gap below.

## The verbs

```
compile          REQUIRED on dev and internal-consumer kinds; OPTIONAL on shared-helper (only when
                 there is a payload to emit).
                 Produce the package payload from committed + generated sources; writes only
                 gitignored output dirs, never tracked files.

clean            REQUIRED on dev/shared-helper/internal-consumer kinds, and on ANY owner that
                 defines `build` (the mirror-only template owner's clean is load-bearing: its
                 hardhat build leaves pkg/artifacts the workspace clean relies on).
                 Delete everything build/test leave behind (tsbuildinfo, forge dirs, emit dirs,
                 scratch); never touches tracked files.

clean:generated  REQUIRED when the package has non-exempt generate:* leaves.
                 Delete every tracked file the generators write; `clean:generated && generate`
                 must round-trip byte-identically.

fmt              REQUIRED on dev/shared-helper/internal-consumer kinds.
                 Rewrite formatting: prettier, plus forge fmt where the package owns Solidity.

fmt:check        REQUIRED wherever fmt is.
                 Read-only twin of fmt; runs after generate in CI to gate generator output
                 formatting.

lint             REQUIRED on dev/shared-helper/internal-consumer kinds.
                 All static analysis (eslint + every tsc --noEmit project + forge lint where
                 Solidity); may read dependency build output (cross-package edges stay in Make).

generate         REQUIRED when the package has any generator.
                 Full regeneration, internally ordered pre → forge compile → post; assumes
                 vendored content is already in sync.

test             REQUIRED on dev and internal-consumer kinds that have suites (a dev owner whose
                 suite lives in its payload delegates to it); OPTIONAL on shared-helper.
                 Self-sufficient on a generated AND built tree; orders test:forge first when it
                 needs ./out.

check            REQUIRED on dev owners of npm-distributed payloads; OPTIONAL elsewhere (define
                 when a package-specific validation exists).
                 Deliverable validations (publint/attw, contract-sizes, zama-config,
                 check:vendored-origin); prerequisite: generated AND built tree. No network.

check:publint    REQUIRED on dev owners of npm-distributed payloads; wired as a member of check.
                 publint --strict + attw on the payload — the packaging-correctness gate (5.2.1),
                 kept explicit rather than absorbed into check's description.

check:mirror     OPTIONAL until the mirror spec lands (rename of test:mirror); standalone forever
                 — never a member of check (it clones upstream, a network dependency check must
                 not acquire).
                 Verifies the mirrored payload against upstream plus declared adaptations.

pack:tarball     REQUIRED on dev owners of npm-distributed payloads.
                 Produce the publishable tarball. Callers are release operators and enforcement —
                 by documented design never part of the daily Makefile loop (npm-rules.md).

test:consumer    REQUIRED on dev owners of npm-distributed payloads (mirror-only payloads exempt).
                 Install the packed payload into a real consumer project and run its suite — the
                 only proof the tarball works.

build            OPTIONAL everyday sweep on in-contract kinds; when present it MUST reach
                 fmt:check, lint and compile (enforced by name-wiring). Overloads the ecosystem's
                 most-typed name in the safe direction: it still produces every artifact, gated.
```

Out of contract entirely: published payloads carry no scripts (rule 2.1.2); standalone consumer
fixtures stay minimal (their committed `--ci` lockfiles must not churn); workspace-root and
non-package entries follow their own rules.

Package-internal and orchestrator-invisible: `build:forge` (the forge compile leaf inside
`generate`), `check:vendored-origin` (rename of test:vendored — a required leaf where vendored content
is declared, byte-comparing it against its pinned source of truth via local git, wired as a
member of `check`; the orchestrator never calls it directly), `build:pkg:ts:*`, `generate:pre`,
`generate:post`,
`generate:*` leaves, `clean:*` splits, `check:*` leaves (including the `--check` conveniences),
`prettier:*`, `forge:*`. They may exist freely; they are enforced only through reachability from
the public verbs.

## Decisions of record (points 1–5 of the design discussion)

1. **`check` is conditional and post-build.** No `check:pre`: `check:exports` and
   `check:cleartext-config` are subsumed by the gate, `check:foundry` is workspace-level, the one
   survivor (`check:zama-config`) rides inside `check`. Reintroducing `check:pre` later is purely
   additive; it is created by the first concrete need (gate too slow for CI, accumulation of
   build-independent checks, pre-generate input validation), not speculatively.
2. **`generate` ⟺ `clean:generated` is bidirectional** — see enforcement below.
3. **`test:vendored` → `check:vendored-origin`**: it compares, it does not execute; member of `check`
   (verified network-free: local `git show`/`forge fmt` only).
4. **`test:mirror` → `check:mirror`, optional for now**: the mirror spec is unimplemented and the
   real check reports false violations; requiring an unpassable script is enforcement theater.
5. **`test:consumer` keeps exactly one exemption** (mirror-only payloads). Everything else
   npm-distributed needs the consumer rehearsal — publint/attw inspect metadata; only a real
   install catches a missing `files` entry, a broken runtime `exports` target, or an unbundled
   dependency.

## Enforcement changes (fhevm-npm check-scripts) — land atomically with the renames

1. Kind-scope the universal set with machine-checkable predicates (not prose): `build` and `test`
   required on dev and internal-consumer kinds; shared-helper exempt from both; a mirror-only
   owner satisfies `test` by delegating to its payload's test; `clean` required wherever `build`
   exists; no scripts demanded of published payloads or standalone fixtures.
2. Bidirectional generate rule: any `generate:*` leaf ⟹ both `generate` and `clean:generated`
   exist; `generate` or `clean:generated` with no `generate:*` leaf ⟹ violation (dead wiring).
3. Reachability — honestly scoped: `expandNpmRunReferences` verifies NAME-WIRING ONLY (that every
   `generate:*` / non-exempt `check:*` leaf name appears in its verb's expansion). It cannot see
   generator OUTPUT paths (those live in export manifests and TS code), so deletion coverage is
   enforced only where machine-readable: the export manifest's `outputs` must all appear in
   `clean:generated`'s expansion. Everything else relies on the gate itself. Harden the expansion
   regex first: it must not capture `--flags` as script names and must not resolve
   `npm run x --prefix ./other` in the caller's namespace.
4. Rename enforcement in the same change as the renames (`check:vendored-origin` required with vendored
   content; `check:mirror` optional) — sequential landing leaves `make ci` red in between.
5. Require `check` on dev owners of npm-distributed payloads, AND keep 5.2.1's `check:publint`
   requirement explicit on the same kind — plus a reachability assertion that `check:publint` is
   a member of `check`.
6. Exempt list for the generate rules: `generate:genesis` (stateful anvil deploy, deliberately
   outside the gate), `generate:patch-sites` (committed baseline a test compares against;
   auto-regenerating it would make the test unfalsifiable). Neither appears in `clean:generated`.

## Known collateral the migration MUST carry (from review)

- **v13 `clean:generated` coverage gap**: it does not delete `test-consumer/esm/src/export.ts`
  and `test-consumer/cjs/src/export.ts`, both written by `generate:exports`. The v12-style split
  port fixes this; without it the gate's dropped-emitter guarantee silently excludes those files.
- **`test:mirror` rename collateral**: the mirror spec injects the old name into the expected
  payload manifest (`fhevm-npm/base/mirrors/hardhat-template-v2.ts:53`) and the committed
  `fhevm-hardhat-template/pkg/package.json`; also `scripts/clone-hardhat-template-v2.ts:182`,
  the `cli-options.ts` check-scripts help text, `npm-rules.md` rule 5.1.3, and
  `fhevm-npm/test/hardhat-template-mirror.test.ts`.
- **hardhat plugin test-consumer caller**: `hardhat/v2/plugin/internal/test-consumer.ts:64-68`
  invokes v13's `build:cjs`/`build:esm`/`build:types` by name; the v13 `build:pkg:ts:*` rename
  updates it in the same change.
- **Orphaned `check:foundry` leaves** in v12/v13 package.json: delete (workspace-level check runs
  once via `run-fhevm-npm`); otherwise they violate the `check:* ⊆ check` reachability rule.
- **Orphaned `check:publint` on the template owner**: no Makefile caller (check-publint runs only
  v12/v13/plugin) and the mirror-only owner is not required to have `check` — delete it (publint
  on a never-npm-published mirror payload), or define an optional `check` containing it.
- **`test:vendored` rename doc collateral**: `host-contracts-cleartext/v12/README.md:139,306`,
  `v13/README.md:139,333`, and `fhevm-npm/npm-rules.md:507,532`, in addition to the mirror items.
- **`runMakeBuildPackage` bridge**: `fhevm-npm/base/test-consumer.ts` drives `make build-package
  PACKAGE=…`; the Makefile rewrite keeps that dispatcher.
- **Three non-verb call sites, resolved as follows**:
  (a) `typechain` on e2e — e2e's `lint` self-provides it (`npm run typechain && eslint && …`),
  the same self-provisioning pattern as `test` running `test:forge` first; its output is
  gitignored derived types, so lint writing it touches no tracked files. The standalone
  `typechain-hh-v2-e2e` target dies; the plugin-built-before-e2e-lint cross-package edge stays.
  (b) `test:anvil` on e2e — becomes an OPERATOR TARGET: a named Makefile category for manual
  entry points that may call a leaf by name because they are not lifecycle orchestration (needs
  an externally running anvil node; already excluded from `ci` and from `test`). Kept for
  `make help` discoverability.
  (c) `test` on the template PAYLOAD (`W_HH_V2_TEMPLATE_PKG`) — give the template owner a
  delegating `test` in step 2 so the orchestrator speaks to the owner, per the verb contract.

## Migration steps (ordered; renames and their enforcement land together)

1. Port v12's script organization to v13: `build:pkg:ts:*` renames (updating the hardhat
   test-consumer caller in the same change), clean splits (closing the `clean:generated`
   coverage gap above), `lint:internal`, BUILD.md.
2. Add the missing verbs across all in-contract packages: `fmt`, `fmt:check`, `check`, and the
   template owner's delegating `test`; rename `test:vendored` → `check:vendored-origin` and
   `test:mirror` → `check:mirror` WITH all listed collateral (docs included) AND the
   check-scripts enforcement updates in one change; delete the orphaned `check:foundry` leaves
   and the template owner's orphaned `check:publint`.
3. Reorder `test` in v12/v13 (`test:forge` first) so `test` self-provides `./out`.
4. Implement the remaining enforcement changes (kind scoping, bidirectional generate rule,
   scoped reachability with the hardened regex).
5. Rewrite the Makefile: verb calls only — with the named exceptions: the two non-member
   formatting lines (fhevm-npm --prefix, scripts/ prettier) and the `test:anvil` OPERATOR TARGET;
   typechain folds into e2e's `lint` (its standalone target dies); the single regeneration gate as specified
   above (porcelain assert, clean-worktree precondition, sync inside the window); keep workspace
   tools, the `build-package` bridge, and the lint→dependency-build edges; delete
   `generate-exports`, `generate-contracts`, `generate-templates`, `build-forge-cleartext`,
   `check-generated-post`, and `check-exports` as separate targets (`check`/`test` inputs come
   from `generate` having run, per the lifecycle).

## Accepted risks and trade-offs

- The gate compiles forge; CI's cheapest-first ordering shifts, total work unchanged (forge once,
  tsc once). The dropped-emitter detection is new coverage CI does not have today.
- `make ci` acquires a clean-worktree precondition (stated, guarded).
- Standalone post-build targets on a fresh clone rely on the documented "generate and build have
  run" prerequisite instead of auto-wired build dependencies; violating it fails loudly (missing
  `./out` or `pkg/ts/_*`), never silently.
- `pkg/state/genesis.json` is generated SHIPPED content with no gate beyond its sha sidecar
  (`generate:genesis` is exempt by design). Accepted; revisit if genesis drift ever bites.
- Reachability enforcement verifies name-wiring, not behavior; a leaf invoked by its underlying
  command (`node internal/cli/x.ts`) inside an aggregate reads as unreachable, and a textual
  mention counts as reachable. It is an anti-rot heuristic, not a proof — the gate is the proof.
