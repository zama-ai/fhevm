# F2 fix proposal: remove persistent build stamps

## Decision

Robustness is more important than avoiding unnecessary builds. Remove persistent Make build stamps and make every
`build-*` target execute whenever it is requested.

Do not try to fix F2 by extending the current source-extension list. Any maintained input list can omit configuration
files, root files, newly introduced file types, or deleted files and can therefore silently skip a required build.

## Current design

```make
DIR_STAMPS := .make

SRC_V12 := $(call sources,$(DIR_V12))
SRC_V13 := $(call sources,$(DIR_V13))

.PHONY: build-cleartext-v12 build-cleartext-v13

build-cleartext-v12: $(DIR_STAMPS)/v12.build
build-cleartext-v13: $(DIR_STAMPS)/v13.build

$(DIR_STAMPS)/v12.build: $(SRC_COMMON) $(SRC_COMMON_VENDORED) $(SRC_V12) | $(DIR_STAMPS) check-npm-cli-pre-build
	@echo "==> build $(DIR_V12)"
	$(call run,$(W_V12),build)
	@touch $@

$(DIR_STAMPS)/v13.build: $(SRC_V13) $(DIR_STAMPS)/v12.build | $(DIR_STAMPS)
	@echo "==> build $(DIR_V13)"
	$(call run,$(W_V13),build)
	@touch $@
```

Make rebuilds only when an input represented in `SRC_*` is newer than its stamp. An omitted input or a deleted file
can leave the stamp current while the output is stale.

## Proposed design

```make
.PHONY: build-cleartext-v12 build-cleartext-v13

build-cleartext-v12: check-npm-cli-pre-build
	@echo "==> build $(DIR_V12)"
	$(call run,$(W_V12),build)

build-cleartext-v13: build-cleartext-v12
	@echo "==> build $(DIR_V13)"
	$(call run,$(W_V13),build)
```

The complete graph follows the same pattern:

```text
build-cleartext-v12
  -> build-cleartext-v13
    -> build-hh-v2-plugin
      -> build-hh-v2-template
      -> build-hh-v2-e2e
```

Every `build-*` target is phony and owns its build recipe directly. Make executes a phony target at most once during
one invocation, even when multiple downstream targets require it.

## Required Makefile changes

1. Move each build recipe from `.make/*.build` onto its public `build-*` target.
2. Replace dependencies on build stamps with dependencies on the corresponding `build-*` targets.
3. Add the appropriate `build-*` prerequisite to every `test-*` target.
4. Make artifact-dependent checks, including `check-generated-post`, depend on the required `build-*` targets.
5. Preserve the real package order through target dependencies.
6. Remove the persistent-stamp machinery:
   - `DIR_STAMPS`;
   - `SRC_*`;
   - `sources`;
   - `prune`;
   - `.make/*.build` and `.make/e2e.typechain` rules;
   - `clean-stamps`.
7. Keep `make ci` starting with `clean`, so CI validates a completely fresh build.

The e2e TypeChain step can remain a separate phony target if both e2e build and lint require it, but it must execute
whenever requested rather than rely on a persistent stamp.

## Expected behavior

```sh
make build-cleartext-v13
```

always runs:

```text
pre-build checks
-> build cleartext v12
-> build cleartext v13
```

Running the command again executes the same sequence again. Changes to any source, configuration, dependency,
root-level file, newly introduced file type, or file deletion cannot be hidden by an incomplete Make input list.

The underlying tools may still use their own incremental caches, but Make always asks them to build. `make ci` runs
`clean` first and therefore does not rely on those caches.

## Trade-off

Local commands will rebuild more often, and recursive Make phases may repeat builds. This cost is accepted explicitly:
an unnecessary build is slower, while a skipped required build can validate stale output and produce a false green
result.

## Verification criteria

- `make build` invokes every package build in dependency order on every run.
- A second unchanged `make build` still invokes the build commands.
- Every standalone `make test-*` target builds the output it consumes first.
- Every standalone artifact-dependent `make check-*` target builds the output it inspects first.
- `make -j` cannot run a consumer before the build target it depends on.
- No `.make` build stamp or maintained source-extension list remains.
- `make ci` still begins with `clean` and completes the full validation workflow.
