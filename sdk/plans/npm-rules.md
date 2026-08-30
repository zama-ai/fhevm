# npm workspace rules — a secure workspace without tarballs

Rules for running `sdk/` on npm workspaces with **no `.tgz` in the dependency graph**, while keeping every guarantee a
tarball install used to provide. Numbered `<paragraph>.<sub-paragraph>.<rule>` and stable, so they can be cited. Each
was verified against this workspace, not assumed — npm and pnpm differ sharply here, and several rules exist because
the obvious spec is the unsafe one.

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

- **Module-type marker** — a `package.json` holding only `type`, to make one directory CommonJS or ESM. It has no
  `name`, so it is not a package at all and nothing can depend on it. Example: `scripts/`, `plugin/pkg/_cjs/`.

- **Workspace root** — `sdk/` itself: the member list plus the toolchain every member shares. `private: true`, never
  published, and not a member of anything.

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
and tests; `pkg/` holds only what ships. Other member kinds do not acquire a `pkg/` merely because they are workspace
members. Packing the dev-owner root publishes the development package by mistake.

```jsonc
// ✅ The shared script defaults to <member>/pkg, so the published package is what gets packed.
"pack:tarball": "\"$(npm prefix)/scripts/pack-tarball.ts\""

// ❌ Packs the member root, so foundry.toml, tests and internal/ all ship.
"pack:tarball": "npm pack"
```

**2.1.3 At most one workspace member has a given published package name.** When multiple generations publish under
one name, their dev owners remain members but only the active generation's published payload sets `member: true`.
Which generation is active is inventory data; the rule is the name-uniqueness invariant. The validator groups
published entries by the `name` read from their real `package.json` and rejects a group with two member entries.

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

**2.1.4 A standalone consumer is never a member.** A project that must keep its own `package-lock.json` — a mirrored
template, anything running `npm ci` in its own CI — breaks the moment npm folds it into the root lockfile.

```jsonc
// ✅ Stable kind invariant; no concrete package path belongs in this rule.
{ "kind": "standalone", "member": false }

// ❌ Its own lockfile stops being authoritative after the root absorbs it.
{ "kind": "standalone", "member": true }
```

## 3. Dependency specs

### 3.1 Depending on a workspace member

**3.1.1 A member depending on another member uses a plain version matching that member's version.** npm links the
member in preference to npmjs.com, even when the same version is published there.

```jsonc
// ✅ Resolves to a symlink to hardhat/v2/plugin/pkg, even though this version exists on npmjs.com.
"@fhevm/hardhat-plugin": "0.4.2"

// ❌ The member is 0.4.2, so no local package satisfies this range and npm goes to npmjs.com.
//    A range that drifts past the member's version is how a member stops testing the member.
"@fhevm/hardhat-plugin": "^0.5.0"
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

**3.2.1 A project outside the workspace links a directory on disk, not a version.** Rule 3.1.1 works only because npm
finds a member with that name. Outside the workspace there is none, so the same spec goes to npmjs.com instead.

```jsonc
// ✅ In fhevm-hardhat-template, which is not a member: a relative path npm can only satisfy from disk.
"@fhevm/hardhat-plugin": "file:../plugin/pkg"

// ❌ Identical to 3.1.1's ✅, but with no member to link. npm fetches npmjs.com's own 0.4.2, which peers
//    on @zama-fhe/relayer-sdk and @fhevm/solidity ^0.11.1 — a different generation, installed silently.
"@fhevm/hardhat-plugin": "0.4.2"
```

### 3.3 Declaring what you use

**3.3.1 Declare every package you import.** A package resolved only because npm hoisted it for someone else is a
phantom dependency: it disappears under a different install strategy, and non-node tools never find it at all.

```jsonc
// ✅ hardhat/v2/e2e imports chai in 45 test files and hardhat-ethers in 20, so it declares both.
"devDependencies": { "@nomicfoundation/hardhat-ethers": "^3.1.3", "chai": "^4.5.0", "hardhat": "^2.28.6" }

// ❌ The same imports, undeclared — the real state of that manifest before this rule was applied.
//    They resolved only because another member's copies sat hoisted in sdk/node_modules.
"devDependencies": { "hardhat": "^2.28.6" }
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

### 4.2 Private packages — dev package, shared helper, internal consumer

**4.2.1 A private package declares every root-pinned package it directly uses, at the root's exact version.** The
package's manifest stays truthful about its own imports, while the root remains the version source of truth. A range
or a different version is forbidden: either would claim support for a version that the workspace did not test. A
private package that does not use the dependency does not declare it. Published packages state a consumer-facing
range instead; rule 4.3.1 requires that range's floor to equal the root pin.

```jsonc
// ✅ sdk/package.json is the version source of truth.
"devDependencies": { "ethers": "6.17.0" }

// ✅ hardhat/v2/e2e imports ethers, so its private manifest repeats the exact root pin.
"devDependencies": { "ethers": "6.17.0" }

// ❌ Missing: this package imports ethers but relies silently on the root's hoisted copy.
"devDependencies": {}

// ❌ A range is not the exact version the private package actually tests.
"devDependencies": { "ethers": "^6.17.0" }

// ❌ A different floor makes a claim the workspace does not verify.
"devDependencies": { "ethers": "^6.16.0" }

// ✅ Published payloads remain ranges whose floor matches the pin, as required by 4.3.1.
"peerDependencies": { "ethers": "^6.17.0" }
```

A future TypeScript validator will use `npm-manifest.json` as the package inventory instead of inventing scan roots.
For every entry whose kind is `dev`, `shared-helper` or `internal-consumer`, it will read the actual `package.json`,
collect bare package imports from its TypeScript and JavaScript sources, and compare any root-pinned import with the
root's actual exact declaration. It will report the stable rule number `4.2.1` when a declaration is missing, is a
range, or differs from the root. Published entries are excluded from this check and handled by 4.3.1; module markers
and standalone projects follow 4.4.1. The manifest supplies scope and package kind, while the real package manifests
remain the source of dependency data.

**4.2.2 A dependency that differs per generation stays in its member.** `hardhat` is `^2.x` for one generation and
`^3.x` for the next, so it cannot be root-pinned; npm hoists the majority and nests the odd one out.

```jsonc
// ✅ In hardhat/v2/plugin and hardhat/v2/e2e; hardhat/v3/* will say ^3.0.0 instead.
"devDependencies": { "hardhat": "^2.28.6" }

// ❌ The same line in sdk/package.json. One root pin cannot serve two majors, and v3 would have
//    to redeclare it — breaking 4.2.1 to work around 4.1.1.
"devDependencies": { "hardhat": "^2.28.6" }
```

**4.2.3 Siblings inside one generation declare identical ranges.** Gated by comparing members against each other
rather than against a table, so the rule needs no edit when a generation is added.

```jsonc
// ✅ hardhat/v2/plugin and hardhat/v2/e2e agree, so the hoisted copy is the one both declare.
"hardhat": "^2.28.6"

// ❌ Live drift today: plugin says ^2.28.4 and e2e says ^2.28.6. Both build against one hoisted
//    copy, so nothing fails until someone installs a member on its own.
"hardhat": "^2.28.4"
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

### 4.4 Standalone projects and module-type markers

**4.4.1 A standalone project inherits nothing, and a module-type marker declares nothing.** The first sits outside
`workspaces`, so no root pin reaches it and every version it needs is declared in its own manifest and frozen in its
own lockfile. The second exists to set `type`; anything else placed in one belongs in a real package.

```jsonc
// ✅ fhevm-hardhat-template states every version itself, because no root pin reaches it.
"devDependencies": { "hardhat": "^2.28.6", "typescript": "^5.9.3" }

// ❌ A module-type marker with dependencies. Nothing can depend on it — it has no `name` — so
//    whatever is declared here is installed for a package that does not exist.
{ "type": "module", "private": true, "devDependencies": { "typescript": "^6.0.2" } }
```

## 5. Published package hygiene — what replaces the tarball install

### 5.1 Naming and the private boundary

**5.1.1 Private packages are named `…-dev` and set `"private": true`.** The field is what npm enforces; the suffix is
what makes a leak visible at the import site, and machine-checkable without packing anything.

**5.1.2 Nothing under `pkg/` imports a `…-dev` package.** That import ships a specifier no consumer can resolve. This
is the one guarantee a tarball install used to hold alone, and 5.1.1 is what makes it a grep.

**5.1.3 Vendored content has one conventional `test:vendored` entry point.** When a manifest entry declares
`vendored`, the central validator requires a non-empty `scripts.test:vendored` in its test owner. The test owner is the
dev package for a published entry, found through the inverse of `publishedRelPath`; for any other entry, it is that
package itself. The script checks every vendored element owned by that entry, but its implementation is package-owned
and unrestricted. CI invokes the convention; the manifest records only each copy's destination, source and reason.

```jsonc
// ✅ The manifest describes the copies without prescribing how they are checked.
"vendored": [
  {
    "relPath": "./src/internal/vendored",
    "source": "./sdk/common-vendored/src",
    "reason": "The published package cannot import a private workspace helper."
  }
]

// ✅ In the test owner's real package.json. One command may check every element above.
"scripts": { "test:vendored": "node ./internal/check-vendored.ts" }

// ❌ Per-element command paths couple inventory data to one check implementation.
"checkedBy": "./sdk/scripts/check-vendored.ts"

// ❌ A vendored entry exists, but CI has no conventional command to discover.
"scripts": {}
```

### 5.2 Packaging checks

**5.2.1 Every dev package that wraps a published package defines `test:publint`.** The script runs `publint --strict`
and `attw --pack` against its `pkg/`. `attw` packs the payload and resolves its types under node10, node16-cjs,
node16-esm and bundler, so packing stays verified with no `.tgz` dependency. The published manifest itself does not
carry this development script; the manifest gate follows `publishedRelPath` and requires it on the owning dev package.

```jsonc
// ✅ --strict fails on warnings, and attw packs the package to resolve its types for real.
"test:publint": "publint --strict ./pkg && attw --pack ./pkg"

// ❌ The required script entry is absent, so no uniform gate can discover the check.
"publint": "publint --strict ./pkg && attw --pack ./pkg"

// ❌ Warnings pass silently and no types are resolved, so a broken `exports` can ship.
"test:publint": "publint ./pkg"
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

**5.3.1 Every dev package that wraps a published package defines `test:consumer`.** This is the stable public command
for proving the installed payload. Optional implementation stages use the same stem, such as `prepare:consumer`,
`test:consumer:run` and `clean:consumer`; callers invoke only `test:consumer`.

```jsonc
// ✅ In the dev package that wraps ./pkg. One command owns the whole consumer test.
"scripts": { "test:consumer": "node ./internal/test-consumer.ts" }

// ❌ A package-specific name cannot be discovered or invoked uniformly.
"scripts": { "test:hardhat-template": "node ./internal/test-consumer.ts" }
```

**5.3.2 Every published package has exactly one dev owner carrying both conventional checks.** The central validator
follows each dev entry's `publishedRelPath` and verifies that it identifies an existing published entry. It also checks
the inverse: every published entry is referenced by exactly one dev entry. The validator then reads that dev package's
actual `package.json` and requires non-empty `scripts.test:publint` and `scripts.test:consumer` entries. It does not
prescribe their implementations or generate their tests; CI executes the package-owned scripts. Other package kinds
do not declare these development scripts merely to satisfy the validator.

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
// ✅ In hardhat/v2/plugin/package.json, the owning dev package exposes both conventional checks.
"scripts": {
  "test:publint": "publint --strict ./pkg && attw --pack ./pkg",
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
dependency with an absolute `file:` path to its `pkg/`, then runs `npm install --install-links`. Outside a workspace,
npm packs such a directory and installs its packed contents instead of linking its source tree, exercising `files`,
ignore rules and the packed manifest without putting a `.tgz` in the dependency graph.

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
  "test:publint": "publint --strict ./pkg && attw --pack ./pkg",
  "test:consumer": "node ./internal/test-consumer.ts"
}

// ❌ Type-resolution analysis alone does not execute the installed runtime artifact.
"scripts": { "test:publint": "publint --strict ./pkg && attw --pack ./pkg" }
```

## 6. Lockfiles and recovery

### 6.1 Lockfiles

**6.1.1 One lockfile per workspace root.** Members have none — the root lock covers them. A standalone consumer
(2.1.4) keeps its own, and that lockfile is regenerated by its own install, never by the root's.

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

**7.1.1 The inventory universe is every source `package.json` at or below `sdk/`.** Repository-aware discovery includes
tracked files and untracked files that are not ignored, honoring every applicable `.gitignore`; it never descends into
version-control metadata. Ignore rules therefore exclude dependency installations, caches, tarball output and other
generated trees without requiring this rule to enumerate every possible directory name. A standalone entry in
`npm-manifest.json` additionally includes that entry's own `package.json`, even when its embedded project is ignored by
the parent repository. This exception can only broaden discovery; no manifest field can exclude a discovered file.

Discovery does not derive its source set from `workspaces`, package names, `private` or package kind. Workspace
members, non-members, standalone projects and module-type markers can therefore all be in scope. A `package.json`
outside `sdk/`, or inside an ignored generated tree without an explicit standalone entry, is out of scope.

```text
# ✅ Both files are in scope; being outside workspaces does not hide the second one.
sdk/package.json
sdk/examples/standalone/package.json

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

**7.1.2 `sdk/npm-manifest.json` classifies every file in that universe exactly once.** Each discovered file is keyed by
its containing directory relative to `sdk/`: `.` for `sdk/package.json`, otherwise a leading `./` with no trailing
slash. `package.json` has no field for which of the seven kinds a directory is, nor for which dev package wraps which
published one. That belongs here, in one file a gate can read — and so does anything else npm gives a package no way
to state.

```jsonc
// ✅ Keyed by relative path, so a key doubles as the identity `publishedRelPath` points at.
{
  "$schema": "./npm-manifest.schema.json",
  "packages": {
    ".": {
      "kind": "workspace-root",
      "name": "workspace",
      "private": true,
      "member": false
    },
    "./hardhat/v2/plugin": {
      "kind": "dev",
      "name": "@fhevm/hardhat-plugin-v2-dev",
      "private": true,
      "member": true,
      "publishedRelPath": "./hardhat/v2/plugin/pkg"
    },
    "./hardhat/v2/plugin/pkg": {
      "kind": "published",
      "name": "@fhevm/hardhat-plugin",
      "member": true
    }
  }
}

// ❌ Keyed by package name. Five module-type markers have none, and v12/pkg and v13/pkg both
//    publish as @fhevm/host-contracts-cleartext, so keying by name drops one of them silently.
{
  "packages": {
    ".": { "kind": "workspace-root", "name": "workspace", "private": true, "member": false },
    "@fhevm/host-contracts-cleartext": {
      "kind": "published",
      "name": "@fhevm/host-contracts-cleartext",
      "member": true
    }
  }
}
```

`sdk/npm-manifest.schema.json` defines the shape, documents every field and enforces every invariant available from one
manifest entry alone. In particular, `.` is the sole workspace root; every entry states `member`; private workspace
packages are members named `…-dev`; standalone projects and module-type markers are non-members; only a dev entry may
carry `publishedRelPath`, which it must carry and which ends in `/pkg`; and module-type markers carry neither a name
nor package ownership or generation metadata. Generation identifiers are safe open paths, not a hard-coded list.

JSON Schema cannot inspect the filesystem or another JSON file, nor can it express graph cardinality across arbitrary
object keys. The TypeScript validator therefore checks the remaining invariants: discovered paths equal manifest keys,
manifest fields agree with each real `package.json`, `member` agrees with `workspaces`, every `publishedRelPath`
selects a nested published entry, and every published entry has exactly one dev owner with the two conventional test
scripts. For an entry declaring `vendored`, it also resolves the test owner defined by 5.1.3 and requires
`test:vendored`. Keeping this boundary explicit prevents a schema description from claiming checks the schema cannot
perform.

**7.1.3 A future TypeScript validator enforces inventory completeness before applying package rules.** It obtains
tracked and untracked-but-not-ignored `package.json` files under `sdk/`, adds the root manifest of each explicitly
declared standalone entry, converts every result to the canonical key from 7.1.2, and compares that set with
`npm-manifest.json#packages`. A discovered key missing from the manifest and a manifest key with no in-scope
`package.json` are both errors reported as rule `7.1.3`. Only after the sets are equal does it use each entry's `kind`,
`member` and `publishedRelPath` to select the other checks. The manifest classifies discovery and may add a standalone
root, but it cannot shrink the set obtained from repository-aware discovery.

**7.1.4 A manifest path never contains `.` or `..` as a segment and never escapes its declared root.** The schema
enforces safe lexical forms for package keys, `publishedRelPath`, vendored `relPath`, local `source`, external `from`
and vendored filenames. The TypeScript validator will additionally resolve every local path against the root named by
its field, follow existing symlinks with `realpath`, and reject a result outside that root. External `from` paths
receive the lexical check before they are joined to a future checkout.

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

> **Written, not yet enforced.** Both files exist and the manifest validates, but nothing reads it. Until the 7.1.3
> gate compares filesystem discovery with the manifest—and then checks the declared member subset against
> `workspaces`—each gate keeps its own idea of the layout: `check-dep-versions.ts` derives scan roots from `workspaces`,
> `check-lint-policy.sh` hardcodes its exclusions.
