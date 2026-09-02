# Everyday dev loop

From the `fhevm/sdk` root folder; Make owns the cross-package dependency graph and speaks to each
package only through its verbs (`fmt`, `lint`, `compile`, `check`, `test`, `generate`, `clean`) —
plus `build`, the everyday sweep.

```sh
make build                  # THE everyday command: fmt-check, then lint, then compile — everything,
                            # gated, each step run once
make compile                # artifacts only, in dependency order (no gates, no forge, no generation)
make test-cleartext-v12     # checks + compile + one package's tests (same shape: test-cleartext-v13,
                            # test-hh-v2-plugin, test-hh-v2-template, test-hh-v2-e2e)
make lint                   # every package's lint, compiling what each lint needs first
make fmt                    # rewrite formatting everywhere (fmt-check verifies without writing)
make check-post             # deliverable gates: publint/attw, contract sizes, configs, vendored
make clean                  # remove every package's build output; never touches tracked files
make rebuild                # clean, then compile from scratch (no gates)
make ci                     # every gate, from a clean tree
make pristine               # reset to a fresh-clone state: build outputs, node_modules, forge deps
                            # (shows each deletion list and asks first; explicit paths, never git clean)
make ci-from-scratch        # the strongest proof: pristine wipe, install, then every ci gate
make graph TARGET=<t>       # show what <t> would run, without running anything
make help                   # list every target
```

Generation is separate from the build, on purpose — generated files are committed build sources.
Regenerate ONLY after changing a generator input (a contract in pkg/src, an export.manifest.json,
cleartext config, a vendored pin), then inspect the diff and commit it.

```sh
# ALL packages — the complete, always-correct form: renders sdk/cleartext-config.json into its faces
# (common-vendored/src/cleartext-config.ts plus each generation's FhevmCleartextConfig.sol and
# scripts/cleartext-config.sh), syncs vendored content, then runs each generating package's own
# `generate` (pre-phase -> forge compile -> post-phase):
make generate

# ONE package — from the sdk root, or `npm run generate` from inside the package directory:
npm run generate -w @fhevm/host-contracts-cleartext-v12-dev
npm run generate -w @fhevm/host-contracts-cleartext-v13-dev

# A single package's `generate` assumes vendored content is already in sync (it is committed, so
# this holds unless you changed common-vendored/, a vendored pin, or sdk/cleartext-config.json —
# in that case regenerate the vendored sources and sync first):
./fhevm-npm-cli generate-cleartext-config
./fhevm-npm-cli sync-vendored

# To AUDIT vendored content without writing anything (fails on any difference): copies are current
# (sync would write nothing) AND each vendored folder matches its declared origin (git commit).
make check-vendored
make check-vendored FHEVM_NPM_ARGS=-vv

# To AUDIT the generated cleartext-config faces without writing anything: re-renders every face of
# sdk/cleartext-config.json in memory and fails if a committed one differs.
make check-cleartext-config

# Deployed chain addresses (sdk/fhevm-chains.config.json): every fhevm host-contract and gateway
# address on mainnet and testnet, rendered from the private protocol registry (source.commit records
# the revision of the last sync). Both need the network and an authenticated `gh`.
./fhevm-npm-cli sync-fhevm-chains --latest          # catch up with the registry's main head
make check-fhevm-chains                             # read-only: is the file current with that head?
                                                    # (registry commits that touch no fhevm address stay green)

# Finer-grained still, inside a package: one generator at a time, e.g.
# `npm run generate:exports` after editing export.manifest.json — no forge compile needed for
# pre-phase generators.

make check-generated        # the regeneration gate: deletes every generated file, regenerates,
                            # requires the tree to come back spotless (clean worktree only)
```

Never run `clean-generated` on its own in a dev loop: it deletes committed files and exists for the
gate above.

The fhevm-npm CLI has two short invocation forms from the sdk root — `./fhevm-npm-cli <command>`
(the launcher the Makefile itself uses), and the executable entry file `./fhevm-npm/fhevm-npm.ts
<command>`. The `./fhevm-npm-cli` spelling in the examples below is equivalent.

Tab completion (commands, flags, package selectors) is rendered from the live command registry —
`./fhevm-npm-cli sh-completion zsh > ~/.zfunc/_fhevm-npm-cli` (with `~/.zfunc` in `$fpath`), or
`source <(<sdk>/fhevm-npm-cli sh-completion zsh)` in `~/.zshrc` after `compinit`; `sh-completion bash` likewise.

Inside one package (from its directory), the same verbs work without orchestration — `npm run build`
(the sweep), `npm run fmt`, `npm run lint`, `npm run compile`, `npm run test`, `npm run check` — plus
its own finer-grained leaves (`test:forge`, `lint:internal`, ...) — list them with `npm run`.

# Changing a vendored origin (pin bump)

Vendored folders are frozen copies of upstream content; `npm-manifest.json` declares each one's
origin (`repository`, `tag`, `commit`, `from`). To move a pin to a new upstream release:

## 1. Make the target commit available locally

The pin resolves through local `git show` — nothing is fetched during checks — so after a new
upstream release:

```sh
git fetch --tags
```

## 2. Edit the pin in npm-manifest.json

Update the vendored entry's `commit` (authoritative) and `tag` (documentation — keep both on the
same release):

```jsonc
"source": {
  "repository": "https://github.com/zama-ai/fhevm",
  "tag": "v0.12.6",                                       // ← bump
  "commit": "<full sha of that tag>",                     // ← bump
  "from": "host-contracts/contracts"
}
```

## 3. Regenerate everything

The sync step rewrites the vendored folder from the new pin — it is the ONLY legal writer of
`pkg/src/contracts`, never edit it by hand — and everything derived from those sources regenerates
behind it:

```sh
make generate
```

## 4. Audit

Copies current AND matching the new declared origin:

```sh
make check-vendored
```

## 5. Review and commit as ONE change

The manifest pin, the re-vendored sources and the regeneration ripple belong in a single commit;
splitting them leaves the regeneration gate red in between.

```sh
git diff          # review: the upstream contract diff plus the regenerated files
git add -A && git commit
```

## 6. Validate

New upstream contracts can shift ABIs, versions and test expectations — expect real follow-up work,
not just churn:

```sh
make build
make test-cleartext-v12 test-cleartext-v13
make ci           # the full gate, now that the tree is committed
```

# Developing and running ESM/CJS consumer tests

## 1. To rebuild the tested package (not the test itself)

```sh
# From the fhevm/sdk root folder; Make owns the package dependency graph:
make compile-package PACKAGE=./host-contracts-cleartext/v13
```

or

```sh
# from the fhevm/sdk root folder
./fhevm-npm-cli test-consumer ./host-contracts-cleartext/v13 --build-linked-dependencies
```

or

```sh
# from the fhevm/sdk root folder
./fhevm-npm-cli test-consumer ./host-contracts-cleartext/v13/test-consumer/<type> --build-linked-dependencies
```

## 2. To run the 'esm' tests (without building the tested package)

```sh
# from the fhevm/sdk root folder
./fhevm-npm-cli test-consumer ./host-contracts-cleartext/v13/test-consumer/esm --run --ci
```

## 3. To run the 'cjs' tests (without building the tested package)

```sh
# from the fhevm/sdk root folder
./fhevm-npm-cli test-consumer ./host-contracts-cleartext/v13/test-consumer/cjs --run --ci
```

## 4. To run a single consumer test file

`--test-file` selects one test file; `--run` executes it.

```sh
# from the fhevm/sdk root folder
./fhevm-npm-cli test-consumer \
  ./host-contracts-cleartext/v13/test-consumer/esm \
  --test-file ./test/fhe-rand.test.ts \
  --run \
  --ci
```

## 5. To rebuild the tested package and run the tests (keep the lock file)

```sh
# esm + cjs
./fhevm-npm-cli test-consumer ./host-contracts-cleartext/v13 --build-linked-dependencies --run --ci
```

```sh
# esm only
./fhevm-npm-cli test-consumer ./host-contracts-cleartext/v13/test-consumer/esm --build-linked-dependencies --run --ci
```

## 6. To regenerate the lock files, rebuild the tested package and run the tests

```sh
# esm + cjs
./fhevm-npm-cli test-consumer-regenerate-package-lock ./host-contracts-cleartext/v13
./fhevm-npm-cli test-consumer ./host-contracts-cleartext/v13 --build-linked-dependencies --run --ci
```

```sh
# esm only
./fhevm-npm-cli test-consumer-regenerate-package-lock ./host-contracts-cleartext/v13/test-consumer/esm
./fhevm-npm-cli test-consumer ./host-contracts-cleartext/v13/test-consumer/esm --build-linked-dependencies --run --ci
```

## 7. To run a manifest-listed consumer project

The selected project must have a `package.json`, a `test` script, at least one manifest-listed `file:` dependency,
and a committed `package-lock.json` when `--ci` is used. `--build-linked-dependencies` asks the SDK Makefile to build
the dev owners of its linked published packages before copying, installing, and testing the consumer. Make remains
responsible for prerequisite ordering and incremental build stamps. Published runtime
dependencies are followed recursively when their exact package name and version identify another manifest candidate;
they do not need artificial direct `file:` declarations in the consumer's `package.json`.

```sh
# from the fhevm/sdk root folder
./fhevm-npm-cli test-consumer \
  ./hardhat/v2/fhevm-hardhat-template/pkg \
  --build-linked-dependencies \
  --run \
  --ci
```
