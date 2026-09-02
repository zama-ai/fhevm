# npm workspace rules — a secure workspace without tarballs

Rules for running `sdk/` on npm workspaces with **no `.tgz` in the dependency graph**, while keeping every guarantee a
tarball install used to provide. Numbered `<paragraph>.<sub-paragraph>.<rule>` and stable, so they can be cited. Each
was verified against this workspace, not assumed — npm and pnpm differ sharply here, and several rules exist because
the obvious spec is the unsafe one.

## Goal

- **No tarballs:** avoid creating, committing or depending on tarball files while still requiring real npm installs in
  isolated consumers.
- **Consumer fidelity:** prove each package works from a fresh, isolated npm installation using only its published
  contents, exports, types and declared dependencies.
- **Repository hygiene:** keep private tooling, published payloads and consumer fixtures clearly separated while
  several repositories are developed together.
- **No version drift:** ensure local development, published dependency ranges and consumer tests exercise compatible
  versions.
- **Automatic validation:** turn every enforceable rule into a static validator or an executable package-owned test.
- **CI-ready:** make checks deterministic, non-interactive and runnable from a clean checkout with conventional
  commands wherever possible.

## 1. Definitions

### 1.1 Package kinds

Seven kinds of `package.json` live under `sdk/`. Which kind a directory is decides every rule below, so name the kind
before arguing about a spec.

- **Published package** — the only thing that ever reaches npmjs.com. Not `private`, declares `exports`, and always
  sits in a `pkg/` subdirectory. Example: `hardhat/v2/plugin/pkg`, published as `@fhevm/hardhat-plugin`.

- **Dev package** — the member directory wrapping exactly one published package, holding its scripts, config and
  tests. `private: true`, named `…-dev`. Example: `hardhat/v2/plugin`, named `@fhevm/hardhat-plugin-v2-dev`.

- **Shared helper** — private code that several dev packages import, with no published counterpart of its own.
  `private: true`, named `…-dev`. Example: `common`, named `@fhevm/sdk-common-dev`.

- **Internal consumer** — a member that installs the published packages in order to test them, and ships nothing
  itself. `private: true`, named `…-dev`. Example: `hardhat/v2/e2e`, named `@fhevm/hardhat-plugin-v2-e2e-dev`.

- **Standalone project** — a consumer deliberately left out of `workspaces`, keeping its own lockfile and its own
  install. Example: `hardhat/v2/fhevm-hardhat-template`, mirrored to a public repo whose CI runs `npm ci`.

- **Non-package** — an unnamed `package.json` carrying directory-local metadata inside another package or project,
  without defining an independently installable package. It has no `name`, version, dependencies or scripts and is
  never a workspace member. It may set fields such as `type`, `main`, `module`, `types`, `typings`, `sideEffects` or
  `private`. Example: `host-contracts-cleartext/v13/pkg/ts/`.

- **Workspace root** — `sdk/` itself: the member list plus the toolchain every member shares. `private: true`, never
  published, and not a member of anything.

Every manifest entry also declares two capabilities independent of `kind`: `type` is `cjs`, `esm` or `dual`, and
`browser` states whether browser execution is supported. `kind` describes ownership and publication; these fields
describe runtime compatibility.

### 1.2 Version of a private package

**1.2.1 Every private package sets `"version": "0.0.0"`.** Nothing resolves one by range — a member links it by name
— so any other number is a claim nobody verifies and everybody has to keep in step with a release it does not have.

## 2. Topology

### 2.1 The member list

The policy never records a concrete workspace-member list. That is changing inventory data and belongs in
`npm-manifest.json` and the root `package.json`. The central validator derives the expected `workspaces` array from
every non-root manifest entry whose `member` is `true`, removes the leading `./` from each key, and requires exact set
equality with the root array. Adding or removing a package therefore changes inventory, not this policy.

```text
expected workspaces = manifest package keys where member is true, with "./" removed
actual workspaces   = sdk/package.json#workspaces

✅ both sets are equal
❌ either set contains a path absent from the other
```

**2.1.1 Members are explicit relative paths, never globs.** A glob silently absorbs a new directory into the
workspace, changing resolution for every package under it before anyone reviews the change.

```jsonc
// ✅ Every member spelled out, so adding a directory is a reviewed edit.
"workspaces": ["packages/dev-owner", "packages/shared-helper"]

// ❌ Also swallows every matching directory added later.
"workspaces": ["packages/*"]
```

**2.1.2 A dev owner stores its published payload in `pkg/`.** The dev-owner root is private and holds scripts, config
and tests; `pkg/` holds only what ships. Packing the dev-owner root publishes the development package by mistake.

```jsonc
// ✅ Delegates to the manifest-aware CLI: the payload comes from this owner's publishedRelPath, and
//    the tarball lands in npm-manifest.json#tarballs.relPath.
"pack:tarball": "node ../../fhevm-npm/fhevm-npm.ts pack-tarball ./host-contracts-cleartext/v12"

// ❌ Packs the member root, so foundry.toml, tests and internal/ all ship.
"pack:tarball": "npm pack"
```

**2.1.3 At most one workspace member has a given published package name.** When generations share a name, only the
active generation's payload sets `member: true`; the validator groups published entries by the `name` read from their
real `package.json` and rejects a group with two member entries. Which generation is active is inventory data.

```text
✅ Illustrative generations:
   previous dev owner       member: true
   previous published name  @scope/library  member: false
   active dev owner         member: true
   active published name    @scope/library  member: true

❌ Ambiguous same-name payloads:
   previous published name  @scope/library  member: true
   active published name    @scope/library  member: true
```

**2.1.4 A standalone consumer is never a member.** A project deliberately installed in place with its own
`package-lock.json` breaks the moment npm folds it into the root lockfile. A manifest-selected consumer copied outside
the workspace is a separate case covered by 6.1.1; its source may remain a member.

```jsonc
// ✅ Stable kind invariant; no concrete package path belongs in this rule.
{ "kind": "standalone", "member": false }

// ❌ Its own lockfile stops being authoritative after the root absorbs it.
{ "kind": "standalone", "member": true }
```

**2.1.5 Every literal local path named by an owned tsconfig exists.** `include`, `exclude`, `files`,
`references[].path` and relative `extends` entries are checked in root, member and non-package tsconfigs; globs are
not literal claims, and conventional transient-output names (`node_modules`, `out`, `cache`, …) may be absent.

```jsonc
// ✅ Existing source path and a glob that may currently match nothing.
{ "include": ["./src", "./generated/**/*.ts"] }

// ❌ A deleted literal path silently weakens TypeScript's coverage.
{ "exclude": ["./test/deleted-file.ts"] }
```

**2.1.6 Every local entrypoint target in an inventoried `package.json` exists.** The validator checks `main`, `module`,
`types`, `typings`, and every local leaf below `exports` and `imports`. An exact target must exist; for a target with
`*` or `?`, its non-wildcard directory must exist. External package targets below `imports` are not filesystem paths.

```jsonc
// ✅ Exact files exist, and the wildcard's directory exists.
{ "types": "./_types/index.d.ts", "exports": { "./abi/*": "./abi/*" } }

// ❌ The package advertises a file or directory that is absent.
{ "main": "./missing/index.js", "exports": { "./feature/*": "./missing/*" } }
```

**2.1.7 Every package.json declares its module type explicitly.** Every manifest package sets `type` to either
`module` or `commonjs`; Node's implicit CommonJS default is not accepted.

```jsonc
// ✅ Explicit consumer runtime semantics.
{ "type": "commonjs" } // test-consumer/cjs/package.json
{ "type": "module" } // test-consumer/esm/package.json

// ❌ Runtime semantics depend on Node's default or contradict the fixture path.
{} // test-consumer/cjs/package.json
{ "type": "commonjs" } // test-consumer/esm/package.json
```

**2.1.8 Published package metadata follows the central field list.** `npm-manifest.json#packageJson` maps package
kinds to `required` and `excluded` top-level fields. The published policy excludes `private` entirely;
`"private": false` is also forbidden so publishability has one canonical representation.

```jsonc
// ✅ Central, editable policy.
"packageJson": {
  "published": {
    "required": ["name", "version", "description", "license"],
    "excluded": ["private"]
  },
  "dev": {
    "required": [],
    "excluded": []
  }
}

// ❌ A published payload is missing required metadata or carries the forbidden field.
{ "name": "@scope/library", "version": "1.0.0", "private": false }
```

**2.1.9 Every inventory entry declares runtime compatibility.** `type` is `cjs`, `esm` or `dual` and must agree with
the real `package.json` type and entry points; `browser` is an explicit boolean capability claim, never an omission.

```jsonc
// ✅ A Node-only package with both CommonJS and ESM entry points.
{ "kind": "published", "type": "dual", "browser": false }

// ✅ A browser-capable ESM package.
{ "kind": "published", "type": "esm", "browser": true }
```

**2.1.10 Top-level package.json entries follow one canonical order.** The editable list in
`fhevm-npm/package-json-order.ts` defines the order for known fields; unknown fields sort alphabetically after them.
Nested ordering rules, such as alphabetical scripts, remain separate checks.

**2.1.11 Every published package carries the repository license.** Its `package.json` sets
`"license": "BSD-3-Clause-Clear"` exactly, and a regular file named exactly `LICENSE` sits beside that package.json.
The field supports registry metadata; the file ensures the published payload carries the license text.

**2.1.12 Node types match the supported Node runtime.** A package declaring `@types/node` also declares
`engines.node` as a minimum version such as `>=22`, and the `@types/node` major equals that minimum Node major.

**2.1.13 A `tsc` invocation matches its tsconfig's shape.** A solution-style tsconfig (empty `files` plus
`references`) must be driven with `tsc -b`: project mode (`tsc -p`, or bare `tsc`) loads it, checks zero files and
exits 0 — the silent failure TS18002 would catch, except `references` suppresses it. Leaves may use either mode.

```jsonc
// ✅ Build mode walks the references of the solution file.
"lint": "tsc -b ./tsconfig.json --noEmit"

// ❌ Project mode on { "files": [], "references": [...] } checks nothing and still passes.
"lint": "tsc -p ./tsconfig.json --noEmit"
```

## 3. Dependency specs

### 3.1 Depending on a workspace member

**3.1.1 A member depending on another member uses a plain version matching that member's version.** npm links the
member in preference to npmjs.com, even when the same version is published there. This holds in a published manifest
too: 4.3.1's consumer-facing ranges are for root-pinned third-party packages, never for a member.

A mirror-only consumer project is the narrow exception: it may use a relative `file:` directory link to the exact
manifest-listed candidate, because `test-consumer` copies the project outside the workspace before installing it. A
package whose distribution includes `npm` never uses `file:` — that local path would not exist for npm consumers.

```jsonc
// ✅ In hardhat/v2/e2e: resolves to a symlink to hardhat/v2/plugin/pkg, even though this version
//    exists on npmjs.com.
"@fhevm/hardhat-plugin": "0.4.2"

// ✅ In the mirror-only fhevm-hardhat-template consumer: resolves to the local candidate after the
//    project is copied outside the workspace by test-consumer.
"@fhevm/hardhat-plugin": "file:../../plugin/pkg"

// ❌ The member is 0.4.2, so no local package satisfies this range and npm goes to npmjs.com.
//    A range that drifts past the member's version is how a member stops testing the member.
"@fhevm/hardhat-plugin": "^0.5.0"

// ❌ A mirror-only consumer link must resolve to the manifest member having this package name.
"@fhevm/hardhat-plugin": "file:../../another-package/pkg"

// ❌ In an npm-distributed package: local filesystem paths cannot be consumed from npmjs.com.
"@fhevm/host-contracts-cleartext": "file:../../../host-contracts-cleartext/v13/pkg"
```

**3.1.2 Never use `file:*.tgz` for a name that is also a member.** npm links members by name before reading the spec,
so the tarball is ignored and the edge is marked `invalid` — after which the whole subtree stops resolving.

```jsonc
// ✅ Same package, linked by name instead of packed. No pack step, nothing to rebuild first.
"@fhevm/hardhat-plugin": "0.4.2"

// ❌ In hardhat/v2/e2e, while @fhevm/hardhat-plugin is a member at hardhat/v2/plugin/pkg. npm reports:
//    invalid: "file:...fhevm-hardhat-plugin-0.4.2.tgz" from hardhat/v2/e2e -> ./hardhat/v2/plugin/pkg
//    ...and then leaves this member's other dependencies unresolved, while `npm install` says "up to date".
"@fhevm/hardhat-plugin": "file:../../../tarballs/fhevm-hardhat-plugin-0.4.2.tgz"
```

### 3.2 Depending from outside the workspace

**3.2.1 A consumer installed outside the workspace links a directory on disk, not a version.** Rule 3.1.1 works only
while npm can find a workspace member with that name; in an isolated copy, a plain version goes to npmjs.com.

```jsonc
// ✅ In fhevm-hardhat-template: test-consumer resolves this relative link to the local candidate before
//    copying and installing the consumer outside the workspace.
"@fhevm/hardhat-plugin": "file:../../plugin/pkg"

// ❌ Identical to 3.1.1's ✅, but with no member to link. npm fetches npmjs.com's own 0.4.2, which peers
//    on @zama-fhe/relayer-sdk and @fhevm/solidity ^0.11.1 — a different generation, installed silently.
"@fhevm/hardhat-plugin": "0.4.2"
```

### 3.3 Declaring what you use

**3.3.1 Declare every package whose API or executable you use.** This includes packages imported by source files and
tools invoked by npm scripts; script-only tools belong in `devDependencies`. A package resolved only through hoisting
is a phantom dependency: it disappears under a different install strategy, and non-node tools never find it at all.

```jsonc
// ✅ hardhat/v2/e2e imports chai in 45 test files and hardhat-ethers in 20, so it declares both.
"devDependencies": { "@nomicfoundation/hardhat-ethers": "^3.1.3", "chai": "^4.5.0", "hardhat": "^2.28.6" }

// ❌ The same imports, undeclared — the real state of that manifest before this rule was applied.
//    They resolved only because another member's copies sat hoisted in sdk/node_modules.
"devDependencies": { "hardhat": "^2.28.6" }

// ✅ A script invokes eslint, and the package declares the tool itself.
{ "scripts": { "lint": "eslint" }, "devDependencies": { "eslint": "^10.0.2" } }

// ❌ eslint happens to be hoisted from the workspace root but is undeclared here.
{ "scripts": { "lint": "eslint" } }
```

**3.3.2 Tools that do not walk up must be told where to look.** `forge` searches only the directories in `libs`, so a
hoisted `@openzeppelin` is invisible to it unless the workspace root's `node_modules` is listed explicitly.

```toml
# ✅ The workspace root's node_modules is listed, so forge finds what npm hoisted there.
libs = ["dependencies", "node_modules", "../../node_modules"]

# ❌ Only this package's own node_modules. A hoisted @openzeppelin is invisible, and forge reports
#    "Source ... not found" naming only this directory — which looks like a missing install.
libs = ["dependencies", "node_modules"]
```

**3.3.3 Globally forbidden packages appear in no dependency section unless the exact package has an exception.** The
manifest lists forbidden names in `dependencies.forbidden`; a `dependencyExceptions` entry permits only that one
package, never siblings or children. An exception naming no declared dependency is stale and fails validation.

```jsonc
// ✅ One narrowly scoped exception.
{
  "dependencies": { "forbidden": ["legacy-linter"] },
  "packages": {
    "./standalone-template": { "dependencyExceptions": ["legacy-linter"] }
  }
}

// ❌ No package-specific exception permits this declaration.
"devDependencies": { "legacy-linter": "^1.0.0" }
```

## 4. Where a version lives

Which rules apply depends on the kind of package, as named in § 1.1.

### 4.1 Workspace root

**4.1.1 The workspace root pins what the whole workspace shares.** One hoisted copy means the root pin is what every
member actually compiles against, so a member declaring something else publishes a claim nobody verified.

```jsonc
// ✅ In sdk/package.json. Exact, because these are the two whose ranges already diverged once.
"devDependencies": { "ethers": "6.17.0", "viem": "2.55.19" }

// ❌ No root pin, so every member states its own and the copy they all build against is
//    whichever one npm happened to hoist.
"devDependencies": {}
```

**4.1.2 The manifest is the single source of truth for the Foundry version.** The repository declares one exact
version in `npm-manifest.json#foundry.version`, compared with `forge --version` by `check-foundry`. Packages must not
duplicate the pin in `.foundry-version` files.

```jsonc
// ✅ One repository-wide pin, checked automatically.
"foundry": { "version": "1.5.1-stable" }

// ❌ A package-local pin can drift from the repository policy.
sdk/some-package/.foundry-version
```

**4.1.3 Every Foundry project inherits the shared formatting policy.** Its `foundry.toml` declares `extends` for
`sdk/foundry.base.toml`; `check-foundry` compares every project's effective `[fmt]` values with the shared file.
Package-specific `[fmt].ignore` values are exempt.

```toml
# ✅ Package-local paths and compiler settings remain local; formatting policy is inherited.
[profile.default]
extends = "../../foundry.base.toml"

# ❌ No extends: future changes to the shared formatting policy would not reach this project.
[profile.default]
src = "pkg/src"
```

### 4.2 Private packages — dev package, shared helper, internal consumer

**4.2.1 A private package declares every root-pinned package it directly uses, at the root's exact version.** A range
or a different version would claim support for a version the workspace did not test; a package that does not use the
dependency does not declare it. Shared helpers use `dependencies` (importing them must install their runtime
requirements); dev owners and internal consumers use `devDependencies`, because neither ships as a runtime package.

```jsonc
// ✅ sdk/package.json is the version source of truth.
"devDependencies": { "ethers": "6.17.0" }

// ✅ hardhat/v2/e2e imports ethers, so its private manifest repeats the exact root pin.
"devDependencies": { "ethers": "6.17.0" }

// ✅ common is a shared helper whose exported runtime imports viem.
"dependencies": { "viem": "2.55.19" }

// ❌ Missing: this package imports ethers but relies silently on the root's hoisted copy.
"devDependencies": {}

// ❌ A range is not the exact version the private package actually tests.
"devDependencies": { "ethers": "^6.17.0" }

// ❌ A different floor makes a claim the workspace does not verify.
"devDependencies": { "ethers": "^6.16.0" }

// ✅ Published payloads remain ranges whose floor matches the pin, as required by 4.3.1.
"peerDependencies": { "ethers": "^6.17.0" }
```

A future import-scanning validator will compare each private package's bare imports with these declarations, keyed by
the manifest inventory, and report `4.2.1` on a missing, ranged or divergent declaration. Published entries stay under
4.3.1; standalone projects and non-packages under 4.4.1.

**4.2.2 A dependency that differs per dependency group stays in its member.** `hardhat` is `^2.x` for one group and
`^3.x` for the next, so it cannot be root-pinned; npm hoists the majority and nests the odd one out.

```jsonc
// ✅ In hardhat/v2/plugin and hardhat/v2/e2e; hardhat/v3/* will say ^3.0.0 instead.
"devDependencies": { "hardhat": "^2.28.6" }

// ❌ The same line in sdk/package.json. One root pin cannot serve two majors, and v3 would have
//    to redeclare it — breaking 4.2.1 to work around 4.1.1.
"devDependencies": { "hardhat": "^2.28.6" }
```

**4.2.3 Siblings inside one dependency group declare identical ranges.** Gated by comparing members against each
other rather than against a table, so the rule needs no edit when a dependency group is added. Membership in the
group is declared by the manifest's `dependencyGroup` field.

```jsonc
// ✅ hardhat/v2/plugin and hardhat/v2/e2e agree, so the hoisted copy is the one both declare.
"hardhat": "^2.28.6"

// ❌ Live drift today: plugin says ^2.28.4 and e2e says ^2.28.6. Both build against one hoisted
//    copy, so nothing fails until someone installs a member on its own.
"hardhat": "^2.28.4"
```

**4.2.4 A dev owner declares packages only in `devDependencies`.** The dev owner is a private build and test harness;
its separate published payload declares its own consumer-facing dependencies. Reusable runtime code belongs in a
shared helper instead of the dev owner.

```jsonc
// ✅ Tooling used by the private dev owner.
"devDependencies": { "ethers": "6.17.0" }

// ❌ A consumer never installs the dev owner as a runtime package.
"dependencies": { "ethers": "6.17.0" }
```

### 4.3 Published packages

**4.3.1 A published dep or peer range's floor equals the root pin.** The published counterpart of 4.2.1: these are the
declarations that must exist. A caret is fine — consumers should get patches — but its floor is the only version the
workspace ever built against.

```jsonc
// ✅ In hardhat/v2/plugin/pkg. Floor 6.17.0 is the root pin; the caret still lets consumers take patches.
"peerDependencies": { "ethers": "^6.17.0" }

// ❌ Floor 6.16.0, a version nothing here ever compiled against. This shipped once, and the first
//    consumer to install 6.16.0 would have been the first to find out.
"peerDependencies": { "ethers": "^6.16.0" }
```

### 4.4 Standalone projects and non-packages

**4.4.1 A standalone project inherits nothing, and a non-package declares no dependencies.** The first sits outside
`workspaces`, so every version it needs is declared in its own manifest and frozen in its own lockfile. The second is
directory-local metadata; installable behavior belongs in a real package.

```jsonc
// ✅ fhevm-hardhat-template states every version itself, because no root pin reaches it.
"devDependencies": { "hardhat": "^2.28.6", "typescript": "^5.9.3" }

// ❌ A non-package with dependencies. Nothing can depend on it — it has no `name` — so
//    whatever is declared here is installed for a package that does not exist.
{ "type": "module", "private": true, "devDependencies": { "typescript": "^6.0.2" } }
```

## 5. Published package hygiene — what replaces the tarball install

### 5.1 Naming and the private boundary

**5.1.1 Private packages are named `…-dev` and set `"private": true`.** The field is what npm enforces; the suffix is
what makes a leak visible at the import site, and machine-checkable without packing anything.

**5.1.2 Nothing under `pkg/` imports a `…-dev` package.** That import ships a specifier no consumer can resolve. This
is the one guarantee a tarball install used to hold alone, and 5.1.1 is what makes it a grep.

**5.1.3 Content-integrity capabilities have conventional check entry points.** A manifest entry declaring `vendored`
requires `check:vendored-origin`; one declaring `mirror` conventionally exposes `check:mirror`, OPTIONAL until the
mirror spec is implemented — requiring an unpassable script would be enforcement theater. The owner is the dev
package for a published entry (inverse of `publishedRelPath`), otherwise the entry itself; implementations are
package-owned, and CI invokes the conventions.

`mirror.repository` identifies an independent Git repository that must stay synchronized with the local entry.
Mirroring is a capability, not a kind; `check:mirror` owns the package-specific contract.

A vendored SOURCE may itself be generated: `common-vendored/src/cleartext-config.ts` is rendered from
`sdk/cleartext-config.json` by `fhevm-npm generate-cleartext-config` (common-vendored's `generate` script), then
`sync-vendored` fans the committed result out — regenerate before syncing; `make generate` runs both in order.
The same command also writes each generation's `FhevmCleartextConfig.sol` and `scripts/cleartext-config.sh`
directly — no sync hop, because nothing imports those files across packages.

```jsonc
// ✅ The manifest describes the copies without prescribing how they are checked.
"vendored": [
  {
    "relPath": "./src/internal/vendored",
    "source": "./sdk/common-vendored/src",
    "reason": "The published package cannot import a private workspace helper."
  }
]

// ✅ Mirroring is orthogonal to kind and records only the independent repository.
"mirror": { "repository": "https://github.com/example/project" }

// ✅ In the owner's real package.json. Each conventional command owns its implementation.
"scripts": {
  "check:vendored-origin": "node ./internal/check-vendored-origin.ts",
  "check:mirror": "node ./internal/check-mirror.ts"
}

// ❌ Per-element command paths couple inventory data to one check implementation.
"checkedBy": "./sdk/scripts/check-vendored-origin.ts"

// ❌ A declared capability exists, but CI has no conventional command to discover.
"scripts": {}
```

**5.1.4 Private source-owning workspace packages expose common hygiene scripts.** Every `dev`, `shared-helper` and
`internal-consumer` package defines non-empty `fmt`, `fmt:check`, `lint`, `prettier:check` and `prettier:write`;
`fmt`/`fmt:check` are the orchestrator's formatting verbs (prettier plus `forge fmt` where the package owns Solidity).

```jsonc
// ✅ The package chooses what is covered while exposing the shared entry points.
"scripts": {
  "fmt": "npm run prettier:write && npm run forge:fmt",
  "fmt:check": "npm run prettier:check && npm run forge:fmt:check",
  "lint": "eslint . && tsc --noEmit",
  "prettier:check": "prettier --check .",
  "prettier:write": "prettier --write ."
}

// ❌ An internal consumer with source files but no discoverable formatting checks.
"scripts": { "test": "hardhat test" }
```

**5.1.4a The artifact verb is `compile`; `build` is the optional gated sweep.** Every dev owner of a published payload
defines `compile` (writes only gitignored output, never tracked files). A `build` script on an in-contract package
must reach `fmt:check`, `lint` and `compile` through same-package `npm run` references — nothing else.

```jsonc
// ✅ compile emits artifacts; build gates them behind formatting and lint, in lifecycle order.
"scripts": {
  "build": "npm run fmt:check && npm run lint && npm run compile",
  "compile": "npm run compile:pkg:ts:cjs && npm run compile:pkg:ts:esm"
}

// ❌ A `build` that skips the gates silently redefines the sweep.
"scripts": { "build": "npm run compile" }
```

**5.1.4b A package that generates exposes the full round trip.** Any package with `generate:*` scripts also defines a
non-empty `generate` aggregate and a non-empty `clean:generated` deleting every file the generators write; the
reverse holds too — `generate` or `clean:generated` with no `generate:*` leaf is dead wiring and is rejected.

Wiring is checked by name: every `generate:*` leaf must be reachable from `generate` through same-package `npm run`
references (exempt: `generate:genesis`, a stateful anvil deploy, and `generate:patch-sites`, a committed baseline a
test compares against), and every `export.manifest.json` output must be deleted by `clean:generated`.

This exists because a regenerate-and-diff gate cannot see a generator that silently _stops_ emitting a file: the
committed copy sits untouched, so nothing looks dirty. Deleting first turns that into a visible deletion, and the
delete list lives beside the generators — the only place it stands a chance of being kept current.

```jsonc
// ✅ Every path the generate:* scripts write is removable, and one verb rewrites everything.
"scripts": {
  "clean:generated": "rm -rf pkg/abi pkg/templates pkg/ts/index.ts",
  "generate": "npm run generate:exports && npm run generate:templates",
  "generate:exports": "node ./internal/cli/generateExports.ts",
  "generate:templates": "node ./internal/cli/generateTemplates.ts"
}
```

```jsonc
// ❌ Generators with no way to prove they still emit everything.
"scripts": { "generate:exports": "node ./internal/cli/generateExports.ts" }
```

**5.1.4c A private source-owning package can delete its own build output.** Every `dev`, `shared-helper` and
`internal-consumer` package that is not a mirror owner defines a non-empty `clean`. Its content requirements are
derived from what the package actually does, never listed centrally:

- a package whose scripts invoke `tsc` must delete `*.tsbuildinfo`, because a surviving build-info file lets the
  next typecheck resume from stale state;
- a Forge project must delete the directories `forge config --json` reports (`cache_path`, `out`, `broadcast`).

Those directory names are **asked of forge, never assumed** — `hardhat/v2/e2e` sets `cache_path = "cache-forge"`, so
any rule hard-coding `cache` is already wrong for one project in three. Anywhere this repo needs a Foundry answer,
`forge config --json` (which resolves `extends`, profiles and defaults) is the source, never a parsed `foundry.toml`.

```jsonc
// ✅ Derived from what the package does: it runs tsc, and forge reports cache/out/broadcast.
"scripts": {
  "clean": "rm -rf cache out broadcast *.tsbuildinfo pkg/ts/_esm pkg/ts/_cjs",
  "compile:forge": "forge build --skip test"
}
```

```jsonc
// ❌ A rebuild resumes from stale incremental state.
"scripts": { "clean": "rm -rf out", "lint": "eslint && tsc -p ./tsconfig.json --noEmit" }
```

**5.1.5 Prettier scripts do not target Solidity.** `forge fmt` is the sole Solidity formatter, so `prettier:check`
and `prettier:write` must exclude `.sol`. A mirror-only payload preserves its upstream toolchain instead (Solidity
Prettier plugin, Solhint) and does not require `forge:fmt`, `forge:fmt:check` or `forge:lint` on its dev owner.

```jsonc
// ✅ Solidity is intentionally absent.
"prettier:check": "prettier --check \"**/*.{js,json,md,ts,yml}\""

// ❌ This gives Solidity two formatters with potentially different output.
"prettier:write": "prettier --write \"**/*.{js,json,md,sol,ts,yml}\""
```

**5.1.6 A non-published package running Prettier uses the workspace configuration.** It contains `prettier.config.js`
referencing `sdk/prettier.base.mjs` by relative path; the only accepted config filenames are those two. A dev owner
containing only `package.json` and `pkg/` has no local source to format, so any local Prettier config is forbidden.

```js
// ✅ In an ESM sdk/common/prettier.config.js.
export { default } from '../prettier.base.mjs';

// ✅ In a CommonJS package's prettier.config.js.
module.exports = import('../../../prettier.base.mjs').then((module) => module.default);
```

```text
❌ Alternate configuration filename:
.prettierrc.mjs
```

**5.1.7 ESLint configuration uses one conventional filename.** Every non-published package defining `lint` contains
`eslint.config.js`; every other ESLint config filename is forbidden, and the workspace root holds the sole
`eslint.base.mjs`. A dev owner containing only `package.json` and `pkg/` forbids any local ESLint config.

```text
✅ eslint.config.js
❌ eslint.config.mjs
❌ .eslintrc.json
```

### 5.2 Packaging checks

**5.2.1 Every dev owner of an npm-distributed payload defines `check:publint` and the `check` verb wrapping its
deliverable validations.** `check:publint` runs `publint --strict` and `attw --pack` against `pkg/`; every `check:*`
leaf must be reachable from `check`, except `check:mirror` (network-bound) and generator `--check` conveniences.

`attw --pack` resolves the packed types under node10, node16-cjs, node16-esm and bundler, so packing stays verified
with no `.tgz`. The manifest gate follows `publishedRelPath` and requires these scripts on the owning dev package;
mirror-only payloads are exempt.

```jsonc
// ✅ --strict fails on warnings, and attw packs the package to resolve its types for real.
"check:publint": "publint --strict ./pkg && attw --pack ./pkg"

// ❌ The required script entry is absent, so no uniform gate can discover the check.
"publint": "publint --strict ./pkg && attw --pack ./pkg"

// ❌ Warnings pass silently and no types are resolved, so a broken `exports` can ship.
"check:publint": "publint ./pkg"
```

**5.2.2 `exports` is the published package's whole public surface, and stays minimal.** What it omits is unreachable
— provided the consumer resolves in a mode that honours it.

**5.2.3 Consumer tests use the resolution modes the package supports.** A package typechecks and executes with the
same `module`, `moduleResolution`, runtime loader and TypeScript major its real consumers use. `nodenext` is required
only when it belongs to that contract; substituting it for a legacy mode proves a different package.

```jsonc
// ✅ Hardhat v2's real TypeScript consumer profile.
{ "module": "commonjs", "moduleResolution": "node10" }

// ❌ Export-aware and useful elsewhere, but not representative of a Hardhat v2 consumer.
{ "module": "nodenext", "moduleResolution": "nodenext" }
```

**5.2.4 A legacy resolution test asserts the resolved file, not merely successful compilation.** Modes such as
`node10` ignore `exports` and can silently fall back to shipped source. The isolated consumer therefore reads
`tsc --traceResolution` and proves that the installed declaration named by `types` was selected.

```text
✅ /tmp/fhevm-consumer-abc123/node_modules/@fhevm/hardhat-plugin/_types/index.d.ts
❌ /repo/sdk/hardhat/v2/plugin/pkg/src/index.ts
❌ /tmp/fhevm-consumer-abc123/node_modules/@fhevm/hardhat-plugin/src/index.ts
```

**5.2.5 Modern resolution checks supplement the real consumer profile.** `attw --pack` keeps its broad node10,
node16-cjs, node16-esm and bundler matrix; `test:consumer` adds runtime execution and the package's actual supported
toolchain. Neither substitutes its own convenient resolution mode for the consumer contract.

```text
✅ attw --pack: broad static resolution matrix
✅ test:consumer: supported toolchain, exact paths and runtime execution
❌ tsc under nodenext only: no proof that a Hardhat v2 consumer resolves the package
```

#### Hardhat v2 profile — proposed example

This profile is the concrete application of 5.2.3–5.2.5 to `@fhevm/hardhat-plugin`. It is an example until a shared
consumer-profile runner makes it executable policy; the package's `test:consumer` remains the required entry point.

```jsonc
{
  "runtime": { "loader": "commonjs", "node": ">=20" },
  "typescript": {
    "version": "5.9.x",
    "module": "commonjs",
    "moduleResolution": "node10",
  },
  "declaration": "node_modules/@fhevm/hardhat-plugin/_types/index.d.ts",
}
```

The profile performs all of these checks against the isolated installation from 5.3:

1. `require('@fhevm/hardhat-plugin')` succeeds from the temporary consumer.
2. The consumer compiles with `module: "commonjs"` and `moduleResolution: "node10"`.
3. `tsc --traceResolution` selects the installed `_types/index.d.ts`.
4. Resolution never reaches the installed `src/index.ts` or any path in the checkout.
5. The real Hardhat template compiles and runs its tests against the installed candidates.
6. `attw --pack` supplies the broader node10, node16-cjs, node16-esm and bundler matrix.

```text
✅ require -> temporary consumer/node_modules/@fhevm/hardhat-plugin/_cjs/index.js
✅ types   -> temporary consumer/node_modules/@fhevm/hardhat-plugin/_types/index.d.ts
❌ types   -> checkout/sdk/hardhat/v2/plugin/pkg/src/index.ts
```

### 5.3 Tarballs

**5.3.1 Every dev package that wraps an npm-distributed package defines `test:consumer`.** This is the stable public
command for proving the installed payload; optional stages share the stem (`test:consumer:run`, …) but callers invoke
only `test:consumer`. A mirror-only consumer project is already the consumer and is exempt.

The checked-in fixtures are format-specific: `test-consumer/cjs` for a CommonJS entry point, `test-consumer/esm` for
ESM, both for a dual package — derived from the published entry points, not duplicated in the manifest. When an
existing manifest-listed project is the real consumer, `consumerTests` maps the format to it instead.

```text
✅ dual package: test-consumer/cjs/ and test-consumer/esm/
✅ ESM-only package: test-consumer/esm/
❌ dual package: test-consumer/esm/ only
```

```jsonc
// ✅ The Hardhat template is the plugin's real CJS consumer.
{
  "type": "cjs",
  "consumerTests": {
    "cjs": "./hardhat/v2/fhevm-hardhat-template/pkg"
  }
}

// ❌ A boolean waiver would remove consumer coverage instead of locating it.
{ "consumerTests": false }
```

An overridden consumer must support the selected module format, define a non-empty `test` script, contain a committed
`package-lock.json`, and directly link the tested package through a directory `file:` dependency. The validator checks
all four properties; `test-consumer --ci` performs the isolated installation and execution.

```jsonc
// ✅ In the dev package that wraps ./pkg. One command owns the whole consumer test.
"scripts": { "test:consumer": "node ./internal/test-consumer.ts" }

// ❌ A package-specific name cannot be discovered or invoked uniformly.
"scripts": { "test:hardhat-template": "node ./internal/test-consumer.ts" }
```

**5.3.2 Every published package has exactly one dev owner carrying its applicable conventional checks.** The validator
resolves each dev entry's `publishedRelPath`, checks the inverse (every published entry has exactly one dev owner),
and requires `check`, `check:publint` and `test:consumer` on owners of npm-distributed payloads only.

```jsonc
// ✅ In npm-manifest.json, the dev entry identifies the published payload it owns.
{
  "./hardhat/v2/plugin": {
    "kind": "dev",
    "publishedRelPath": "./hardhat/v2/plugin/pkg",
  },
}
```

```jsonc
// ✅ In hardhat/v2/plugin/package.json, the owning dev package exposes the conventional checks.
"scripts": {
  "check": "npm run check:publint && npm run check:vendored-origin",
  "check:publint": "publint --strict ./pkg && attw --pack ./pkg",
  "test:consumer": "node ./internal/test-consumer.ts"
}

// ❌ Package-specific names are not the conventional entry points the validator requires.
"scripts": {
  "check:package": "publint --strict ./pkg",
  "test:hardhat-template": "node ./internal/test-consumer.ts"
}
```

**5.3.3 A consumer test runs outside every workspace.** It creates a fresh temporary directory outside the checkout
and copies or generates the consumer there. A non-member nested below `sdk/` is not isolated: Node can still walk up
to `sdk/node_modules` and satisfy an undeclared dependency.

```text
✅ /tmp/fhevm-consumer-abc123
❌ /repo/sdk/hardhat/v2/plugin/test/consumer
```

**5.3.4 Candidate directories are installed with `--install-links`.** The temporary manifest replaces each candidate
dependency with an absolute `file:` path to its `pkg/`; outside a workspace, npm packs such a directory and installs
the packed contents, exercising `files` and ignore rules without putting a `.tgz` in the dependency graph.

```jsonc
// ✅ Generated only in the temporary consumer; npm installs the packed directory contents.
"@fhevm/hardhat-plugin": "file:/repo/sdk/hardhat/v2/plugin/pkg"

// ❌ A plain workspace version resolves to the workspace link, not the packed payload.
"@fhevm/hardhat-plugin": "0.4.2"
```

**5.3.5 Packages released together are installed together.** One clean install names every candidate in the release
set, so a local plugin cannot silently test against an older dependency fetched from npmjs.com.

```jsonc
// ✅ Both candidates are under test in the same dependency graph.
"devDependencies": {
  "@fhevm/hardhat-plugin": "file:/repo/sdk/hardhat/v2/plugin/pkg",
  "@fhevm/host-contracts-cleartext": "file:/repo/sdk/host-contracts-cleartext/v13/pkg"
}

// ❌ The plugin candidate can resolve the host-contracts package from the registry.
"devDependencies": { "@fhevm/hardhat-plugin": "file:/repo/sdk/hardhat/v2/plugin/pkg" }
```

**5.3.6 The gate proves that installation is physical and isolated.** Each candidate in `node_modules` must not be a
symlink, and its real path must stay below the temporary consumer. The test starts without an inherited lockfile or
`node_modules`.

```text
✅ /tmp/fhevm-consumer-abc123/node_modules/@fhevm/hardhat-plugin
❌ /repo/sdk/hardhat/v2/plugin/pkg
```

**5.3.7 The consumer exercises the published surface by package name.** It typechecks the supported resolution modes,
loads the runtime entry points and runs the real consumer suite. A relative import bypasses package resolution and is
not a consumer test.

```ts
require('@fhevm/hardhat-plugin'); // ✅ Exercises the installed manifest, exports and files.
require('../../plugin/pkg/src/index'); // ❌ Reaches source directly and proves nothing about the installed package.
```

**5.3.8 Packaging linters supplement the consumer test.** `publint --strict` and `attw --pack` remain mandatory for
metadata and type-resolution coverage, but neither replaces runtime execution of the isolated installed payload.
`pack:tarball` remains a release-artifact command, never a prerequisite of the daily workspace loop.

```jsonc
// ✅ Static packaging checks and the installed-artifact test cover different failures.
"scripts": {
  "check:publint": "publint --strict ./pkg && attw --pack ./pkg",
  "test:consumer": "node ./internal/test-consumer.ts"
}

// ❌ Type-resolution analysis alone does not execute the installed runtime artifact.
"scripts": { "check:publint": "publint --strict ./pkg && attw --pack ./pkg" }
```

**5.3.9 No test-consumer parallelism is allowed.** Consumer packages, CJS/ESM fixtures and test files execute one at
a time; a fixture using Node's test runner sets `--test-concurrency=1`, and other runners use their serial mode.
Distinct ports do not permit an exception to this rule.

## 6. Lockfiles and recovery

### 6.1 Lockfiles

**6.1.1 One authoritative lockfile per installation root.** Ordinary members have none (the root lock covers them); a
standalone project keeps its own, and so does a `consumerTests`-referenced package, because its isolated copy is an
independent installation root. Consumer lockfiles are immutable except through `test-consumer-regenerate-package-lock`.

```text
✅ ordinary workspace member                    sdk/package-lock.json only
✅ standalone consumer                         <consumer>/package-lock.json
✅ workspace member selected by consumerTests  <consumer>/package-lock.json for its isolated copy
❌ ordinary workspace member                    nested package-lock.json
```

`test-consumer` normally removes the lock only from its temporary copy and runs `npm install --install-links`, testing
fresh compatible resolution. With `--ci`, it requires the committed lock and runs `npm ci --install-links` instead.

### 6.2 Recovering a broken tree

**6.2.1 A member rename or a `workspaces` edit invalidates the tree, not just the lock.** npm compares against
`node_modules/.package-lock.json`, so it can report "up to date" while dependencies are missing.

**6.2.2 When npm says "up to date" and packages are absent, delete the tree.** Reinstalling cannot fix it: npm
believes there is nothing to do. Remove `node_modules` (and the lockfile only if placement is stale), then install.

**6.2.3 `.npmrc` is read from the local prefix, not from ancestors.** A nested workspace root does not inherit the
outer repo's `.npmrc`, so settings like `install-strategy` silently do not apply.

```sh
# ✅ Asked from the nested root, so the answer is the one that governs its installs.
cd sdk && npm config get install-strategy

# ❌ Asked from the repo root, reporting the outer .npmrc's value — which sdk/ never reads.
npm config get install-strategy
```

## 7. Inventory

### 7.1 The manifest

**7.1.1 The inventory universe is every SDK source `package.json` at or below `sdk/`.** Discovery is
repository-aware — tracked files plus untracked-unignored ones — so ignore rules exclude installed and generated
trees without this rule enumerating names; `inventory.exclude` removes explicitly out-of-scope trees.

The validator's own package tree is tooling, excluded by its resolved installation path. A standalone entry
additionally includes its own `package.json` even when the parent repository ignores it. Discovery never derives its
set from `workspaces`, names, `private` or kind — members, non-members, standalone projects and non-packages are all
in scope; a `package.json` outside `sdk/` or inside an ignored tree without a standalone entry is not.

```text
# ✅ Both files are in scope; being outside workspaces does not hide the second one.
sdk/package.json
sdk/examples/standalone/package.json

# ✅ The validator identifies and excludes its own autonomous package tree.
sdk/<validator-installation>/package.json

# ✅ A separately managed tree is excluded explicitly, including all of its descendants.
"inventory": { "exclude": ["./separately-managed-sdk"] }

# ✅ Ignore rules exclude generated and installed trees without hard-coding their names here.
sdk/node_modules/dependency/package.json       # ignored dependency installation
sdk/cache/tool/package.json                    # ignored generated cache
sdk/tarballs/unpacked/package.json             # ignored release output

# ✅ An explicit standalone entry includes its own ignored root manifest, but not its generated children.
sdk/examples/standalone/package.json           # included
sdk/examples/standalone/cache/package.json     # still ignored

# ❌ Discovering only workspace members misses valid non-member package.json files.
sdk/package.json -> workspaces[]

# ❌ A fixed prune list becomes incomplete whenever another tool creates a differently named output tree.
prune = ["node_modules", ".git"]
```

**7.1.2 `sdk/npm-manifest.json` classifies every in-scope manifest exactly once.** An entry records the package kind
and the inventory facts that `package.json` cannot express, such as dev-to-published ownership, dependency grouping
and mirroring.

```jsonc
// ✅ Manifest fragment keyed by the directory relative to sdk/.
"./family/active/pkg": { "kind": "published", "name": "@scope/library", "member": true }

// ❌ A package name is neither a filesystem identity nor guaranteed to be unique.
"@scope/library": { "kind": "published", "name": "@scope/library", "member": true }
```

**7.1.3 The inventory and discovery sets are equal.** The TypeScript validator canonicalizes the files found by 7.1.1,
removes the trees listed by `inventory.exclude`, adds each non-excluded explicitly declared standalone root, and
compares that set with `npm-manifest.json#packages`. A missing or stale key is a `7.1.3` error.

**7.1.4 A manifest path never contains `.` or `..` as a segment and never escapes its declared root.** The schema
enforces safe lexical forms for every path field; the validator additionally resolves each local path against its
declared root, follows symlinks with `realpath`, and rejects a result outside that root.

```jsonc
// ✅ Canonical paths stay below their documented roots.
"publishedRelPath": "./hardhat/v2/plugin/pkg"
"source": "./sdk/common-vendored/src"

// ❌ Traversal is rejected even when normalisation would produce an in-tree path.
"publishedRelPath": "./hardhat/v2/../v3/plugin/pkg"
"source": "./sdk/../outside"

// ❌ A vendored filename is one basename, not a path or traversal segment.
"files": ["..", "nested/file.ts"]
```

### 7.2 Enforcement boundary

**7.2.1 JSON Schema enforces entry-local invariants.** It validates canonical keys, field shapes, package-kind
constraints, safe lexical paths and mirror repository URLs. It also reserves `.` for the workspace root and restricts
ownership metadata to the kinds that can carry it.

**7.2.2 The TypeScript validator enforces external and graph invariants.** It compares entries with real
`package.json` files and `workspaces`, resolves `publishedRelPath`, checks one dev owner per published package, and
requires the conventional scripts selected by each entry's capabilities. These checks run only after 7.1.3 proves the
inventory complete.

> **Implemented.** `fhevm-npm check-manifest-coverage` performs repository-aware discovery, excludes its own autonomous package
> tree and the paths listed by `inventory.exclude`, adds explicit standalone roots, compares the result with the
> manifest and checks realpath containment.
