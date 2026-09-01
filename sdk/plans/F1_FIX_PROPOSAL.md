# F1 fix proposal: derive generation from a strict script namespace

## Problem

`check-generated` currently compares `git status --porcelain` before and after generation, so it does not maintain a
list of generated output paths. However, the generation command itself still maintains a list of generator scripts:

```make
generate-contracts:
	$(call run,$(W_V12),generate:cleartext-config)
	$(call run,$(W_V13),generate:cleartext-config)
	# ...more manually listed generators...
```

If a developer adds and commits a new `generate:foo` script but forgets to wire it into the Makefile, neither
`make generate` nor `check-generated` executes it. Its committed output can later drift while CI remains green.

The fix must make namespace membership, not a Makefile list, determine which generators execute.

## Decision

Adopt a strict, phase-aware namespace for every command that generates committed source:

```text
generate:pre:<name>
generate:post:<name>
```

- `pre` generators do not require build artifacts.
- `post` generators consume build artifacts and run only after the required packages build.
- Commands that intentionally refresh local or acceptance state are not generators and must use another namespace.

`fhevm-npm` discovers and executes every conforming generator from an explicitly eligible subset of the
manifest-listed package inventory. Make owns the phase order but never lists individual generators.

The runner and validator must use one shared eligibility predicate. Eligible generator owners are packages of kind
`dev`, `shared-helper`, or `internal-consumer`. Every package outside those three kinds—mirror payloads, published
payloads, standalone consumer fixtures, non-package entries, and the workspace root—is ineligible for execution. A
`dev` package whose payload is a mirror remains eligible because its own scripts are repository-authored. Trees
excluded by the inventory remain outside the mechanism entirely.

Execution eligibility and validation coverage are deliberately different:

- eligible packages have their `generate:*` scripts validated and executed;
- ineligible manifest-listed packages must not define any `generate:*` script;
- mirror-only packages are exempt from that prohibition because their package contents are upstream-controlled;
- inventory-excluded trees are outside discovery, validation, and execution.

This makes an accidentally misplaced generator fail loudly instead of becoming invisible. In particular, scripts in
a byte-for-byte mirror must neither be validated as repository generators nor executed by `make generate`.

## Proposed script names

The current cleartext scripts would become:

```text
generate:pre:cleartext-config
generate:pre:compute-addresses
generate:pre:contract-versions
generate:pre:exports
generate:pre:placeholders

generate:post:local-host-bytecode
generate:post:signers
generate:post:templates
```

The current aggregate `generate:templates` must become atomic: it should generate templates only. Signers and local
host bytecode remain separate post-build generators discovered by the same mechanism.

The two deliberately manual commands leave the `generate:*` namespace:

```text
generate:genesis     -> refresh:genesis
generate:patch-sites -> accept:patch-sites
```

This prevents automatic generation and CI checks from running operations whose purpose is manual refresh or
acceptance.

## New fhevm-npm command

Add one command that executes a complete phase:

```sh
node ./fhevm-npm/fhevm-npm.ts run-generators pre
node ./fhevm-npm/fhevm-npm.ts run-generators post
```

The command:

1. Loads `npm-manifest.json` and every manifest-listed package.
2. Applies the shared generator-owner eligibility predicate.
3. Selects every script key matching exactly `generate:<phase>:<name>`, including scripts with empty commands.
4. Rejects malformed generator names and empty commands as errors rather than silently ignoring them.
5. Sorts selected scripts deterministically by full generator name and then package path.
6. Executes them serially and stops at the first failure.
7. Returns a non-zero exit code when any generator fails, including when the failed generator changed no file.
8. Prints the selected package and script at normal progress verbosity.
9. Supports the existing `-v` through `-vvvv` output policy.
10. Supports `--list`, which prints the exact selected set without executing it.

Serial execution is intentional. Generation writes committed shared state, and robustness is more important than
parallel speed.

Generators within one phase must be independent; there is no dependency-ordering mechanism. The deterministic
full-name and package-path sort exists for reproducibility, not to express dependencies. Generator names whose
`<name>` component matches `^\d+-` are reserved and rejected, so authors cannot accidentally encode an ordering
assumption. An ordering mechanism may be designed later when a concrete dependency requires one.

Every generator is invoked through a common runner wrapper. F6 may later extend that wrapper with an output contract
without changing generator discovery or execution; this proposal does not implement that output contract.

## Validator rules

Extend `check-scripts` so that:

- every manifest-listed package is inspected for `generate:*` scripts unless it is mirror-only;
- an ineligible non-mirror package defining any `generate:*` script fails validation;
- eligible packages use the shared parser for all remaining generator rules;
- every automatically executed generator matches `generate:(pre|post):<name>`;
- bare `generate:<name>` scripts are forbidden;
- empty generator commands are forbidden;
- a generator script contains one command only and cannot contain shell composition or delegation through
  `npm run`;
- packages defining generators still require a non-empty `clean:generated` script;
- the known manual commands use their required `refresh:*` or `accept:*` names.

The validator and runner must share the same parser and selector. There must not be one implementation deciding what
is valid and another deciding what executes.

The single-command rule must use a strict, fail-closed allowed-command grammar. Initially, the accepted form is one
direct Node invocation:

```text
node <entrypoint> [arguments...]
```

Shell composition, redirection, pipelines, command substitution, environment-assignment prefixes, and delegation
through `npm run` are rejected. A command requiring a new construct must first extend the grammar and its tests. A
general-purpose shell tokenizer is explicitly rejected because a parser bug could accept unsupported composition in
the unsafe direction. Static validation still cannot detect a single TypeScript entrypoint that imports and runs
multiple generators; that remains a code-review boundary.

Likewise, no validator can infer that an arbitrary future command is a manual refresh or acceptance operation. It can
enforce the names of known commands and the namespace grammar, while the rule that acceptance operations must never
enter `generate:*` remains a documented authoring convention.

A package containing only `accept:*` or `refresh:*` commands does not derive a `clean:generated` requirement. Those
commands intentionally update local state or acceptance baselines whose outputs must survive `clean:generated`.

## Makefile after the change

Make retains cross-phase orchestration but contains no generator inventory:

```make
.PHONY: generate generate-pre generate-post

generate: ## Write every generated file (then commit the result)
	$(MAKE) generate-pre
	$(MAKE) build-cleartext-v12 build-cleartext-v13
	$(MAKE) generate-post

generate-pre:
	$(call run-fhevm-npm,run-generators pre)
	$(call run-fhevm-npm,sync-vendored)

generate-post:
	$(call run-fhevm-npm,run-generators post)
```

`check-generated` and `check-generated-post` invoke the same phase targets inside a clean-worktree gate:

```make
check-generated:
	$(call assert-clean-generation,generate-pre)

check-generated-post: build-cleartext-v12 build-cleartext-v13
	$(call assert-clean-generation,generate-post)
```

With the stamp-free F2 proposal, the post-build prerequisites always ask both packages to build before artifact-based
generation is checked.

Each phase invocation is a separate recipe command. Its exit status must propagate directly to Make; it must never be
joined to the cleanliness checks using `;` or otherwise depend on global shell `-e` behavior. Therefore, a generator
failure fails the check even when it exits before modifying the worktree.

The existing hand-maintained `generate-exports`, `generate-contracts`, and `generate-templates` recipe lists are
removed. A targeted development command can be supported by `run-generators --package <path>` or
`--name <generator>` without reintroducing Makefile lists.

## Clean-worktree requirement

The generated-output checks must require the entire Git repository to be clean before running generation and require
it to remain clean afterward. They must not compare only the text of `git status --porcelain` before and after.

This repository-wide scope is intentional. `sdk/` is a subdirectory of the monorepo, so an unscoped `git status`
also detects changes outside `sdk/`. A path-scoped `git status -- .` is explicitly rejected because a narrower scope
can only miss generator side effects. The error message must therefore refer to the repository, not merely the SDK.

Status-text comparison is insufficient on an already dirty tree. If a generator changes a file that was already
modified, both captures can contain the same status line:

```text
Before:  M generated.ts
After:   M generated.ts
```

The contents changed, but the status strings are equal. Existing untracked files have the same weakness.

The robustness-first gate is therefore:

```make
define assert-clean-generation
	@[ -z "$$(git status --porcelain)" ] || { \
	  echo "Refusing generation check: the worktree is not clean."; \
	  exit 1; \
	}
	$(MAKE) --no-print-directory $(1)
	@[ -z "$$(git status --porcelain)" ] || { \
	  git status --short; \
	  echo "Generation changed the worktree — run 'make generate' and commit the result."; \
	  exit 1; \
	}
endef
```

`git status` is read-only for this purpose. This gate creates no commit, tree, stash, ref, or persistent snapshot in
the Git object database. The status text exists only for the duration of each shell command.

The trade-off is explicit: developers must commit or stash unrelated work anywhere in the repository before running
generated-output checks. That inconvenience is accepted because a dirty-worktree comparison can produce a false
green result. A content-fingerprint mode for dirty trees is deliberately not included; the clean-tree gate is simpler
and more robust.

## Why this closes F1

Adding a correctly named generator automatically changes the set selected by:

- `make generate`;
- `check-generated` or `check-generated-post`;
- real CI.

There is no second Makefile edit to remember. The same namespace parser validates and executes the generator.

No tool can infer from arbitrary code that a command writes committed files. The enforceable policy boundary is:
every command that generates committed source must use the strict `generate:pre:*` or `generate:post:*` namespace.
Once that convention is followed, forgetting Makefile wiring is impossible.

The namespace governs eligible npm package scripts only. `sync-common-vendored` remains an explicitly Make-managed
operation unless it is separately migrated. Trees excluded by `inventory.exclude`, currently including `js-sdk`, are
outside discovery, validation, and execution.

## Relationship with F6

This proposal closes generator **execution coverage** only. It does not prove that `clean:generated` removes every
output.

F6 should be solved separately by giving each generator one authoritative output declaration that is consumed by
both the generator framework and cleanup. Adding another documentation-only output list would reproduce the same
problem rather than solve it.

## Migration sequence

1. Add the shared eligibility predicate, generator-name parser, and phase selector to `fhevm-npm`.
2. Add `run-generators <phase> --list` and tests without removing the existing Makefile inventory.
3. Extract the current `generate:templates` aggregate into three atomic post-build scripts. Record that the resulting
   alphabetical order—local host bytecode, signers, templates—is safe because those generators are independent.
4. Rename current automatic scripts into the `generate:pre:*` and `generate:post:*` namespaces, temporarily updating
   the existing explicit Makefile calls to the new names while retaining the old inventory structure. Record that the
   resulting pre-phase order is safe: those generators read committed templates, manifests, and contract sources
   rather than one another's generated output.
5. Rename manual commands to `refresh:genesis` and `accept:patch-sites`.
6. Extend `check-scripts` using the shared eligibility predicate, parser, and selector.
7. Add a parity snapshot that pins the exact legacy selection: eight automatic generator names for each of v12 and
   v13. Compare `run-generators --list` with the still-explicit Makefile inventory by exact set equality before
   switching orchestration.
8. Replace Makefile generator lists with `generate-pre` and `generate-post` calls only after the parity test passes.
9. Keep the parity snapshot as a permanent high-security approval gate. Adding or removing a generator requires an
   explicit snapshot update, but forgetting that update fails CI loudly and never silently skips execution.
10. Replace before/after status comparison with the repository-wide clean-worktree precondition and postcondition.
11. Run `make generate` from a clean repository and require a zero diff.
12. Update documentation, shell diagnostics, generator banners, and error messages that mention old generator names.
13. Require `git grep -n 'npm run generate:'` to contain only valid `generate:pre:*` and `generate:post:*` references.

The permanent parity snapshot is intentionally an asymmetric safety gate. It is a second deliberate approval edit,
but not a second execution-wiring edit: discovery still includes a conforming generator automatically, and stale
snapshot state fails loudly rather than allowing a generator to disappear silently. This cost is accepted because
robustness is more important than convenience.

The snapshot is a regression witness, never an execution inventory:

- validation compares the discovered set and snapshot using exact set equality, so additions and removals both fail;
- each snapshot element is the pair `(package-relative-path, full-script-name)`; the full script name includes its
  phase;
- the committed representation is canonical and sorted deterministically by full script name and then package path;
- the snapshot is a static committed file and is never regenerated automatically by CI;
- neither `run-generators` nor any selector, parser, or execution path may read the snapshot;
- generator selection is computed exclusively from the manifest, package metadata, eligibility predicate, and script
  namespace;
- the mismatch diagnostic explains that the snapshot records the approved generator set and that changing it
  explicitly approves an inventory change;
- CI runs the snapshot validation as a pre-build check before executing either generator phase.

The snapshot does not prove that discovery found every conceptual generator; discovery and validation deliberately
share the same parser. It protects against later regressions in that shared selection logic by witnessing changes to
the previously approved result. A test must additionally prove that `--list` and actual execution consume the same
selected array: inject a fake executor, compare its calls with `--list`, and do not independently recompute either
set.

## Verification criteria

- Adding a new `generate:pre:*` script makes it run in `make generate` without editing the Makefile.
- Adding a new `generate:post:*` script makes it run after the required build without editing the Makefile.
- Both generator phases stop at the first failed script.
- A generator that exits non-zero without changing any file makes the generated-output check fail.
- Bare or malformed `generate:*` names fail `check-scripts`.
- An empty command whose key matches the generator namespace fails both validation and direct runner selection.
- Generator names beginning with digits followed by `-` fail validation because ordering syntax is reserved.
- The known `refresh:genesis` and `accept:patch-sites` commands are not selected by either generator phase.
- Mirror-only packages are neither selected nor subjected to repository generator validation.
- A `dev` owner of a mirror payload remains eligible for generator validation and execution.
- Any `generate:*` script in an ineligible non-mirror manifest package, including a published payload, standalone
  consumer, non-package entry, or workspace root, fails validation.
- Inventory-excluded trees are outside discovery, validation, and execution.
- `make generate`, `check-generated`, and `check-generated-post` use the same phase selection implementation.
- A fixture test proves that `run-generators --list` and actual execution consume the same selected array.
- Generated-output checks refuse to run when any path in the Git repository is dirty.
- A successful generated-output check leaves `git status --porcelain` empty.
- Code review confirms that generated-output checks call only read-only Git status operations and create no Git
  object, stash, commit, tree, or ref.
- `make -n generate-pre generate-post` contains no hand-maintained list of individual generator script names.
- Adding an eligible manifest-listed package with conforming generator scripts automatically selects it and makes a
  stale parity snapshot fail until the new selection is explicitly approved.
- Before the Makefile inventory is removed, `run-generators --list` matches the legacy automatic generator set.
- After migration, the permanent parity snapshot matches the complete selected generator set.
- Every parity-snapshot element is a canonical `(package-relative-path, full-script-name)` pair.
- The parity check uses exact set equality and cannot update its own snapshot.
- `run-generators`, the shared selector, and all generator execution paths contain no code path that reads the parity
  snapshot.
- CI validates the parity snapshot before running either generator phase.
