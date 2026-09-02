# Rules

- no git push
- no git pull
- no git commit
- no git command
- no access to internet
- you can only edit /Users/alex/tmp-zama-ai-fhevm/fhevm/sdk
- you can never edit anything outside of /Users/alex/tmp-zama-ai-fhevm/fhevm/sdk

# Symmetric hardhat generation clusters, workspace-native template (option E), and the v3 skeleton

Status: **IMPLEMENTED in this tree** (phases 1–3), with ONE step pending network access:
`npm --prefix hardhat/v3 install` — hardhat@3 tarballs are not in the offline cache. The cluster's
committed lockfile already pins hardhat 3.15.0 (generated offline from cached metadata), so the online
run is a plain `npm ci`-shaped install; the v3 plugin's tests skip themselves loudly until then. (v2: supersedes the asymmetric draft — every
hardhat generation is now a cluster; the root workspace holds no hardhat at all.)

The problems this plan solves, under one constraint:

1. Rebuild `hardhat/v2/fhevm-hardhat-template` on option E — the workspace copy is workspace-native,
   the public repo is a RENDER of it.
2. Create the `hardhat/v3` skeleton: `plugin` + `plugin/pkg` with a minimal hello-world hardhat 3
   plugin — with independent `fhevm-hardhat-template` and `e2e` siblings arriving later.

Constraint: **no hand-made symlinks anywhere.** npm's own workspace/`file:` linking is not banned
(it is how npm works); every `ln -s` we wrote must go, and no new one may appear.

## 0. Root cause, invariant, and the symmetry principle

Hardhat's environment is a per-module SINGLETON: plugins decorate the `hre` of whichever `hardhat`
directory they resolve — and a package reached through a link resolves its dependencies from its
REAL location. Every failure this week (broken `hre.fhevm`, TS2339 on `hre.ethers`) was one topology
mistake: a plugin and its consumer resolving two different `hardhat` directories. Hand-made symlink
shims papered over instances of it; banning them forces the topology itself to be right.

The invariant that replaces symlinks:

> **One npm installation root per hardhat major.** Inside one installation root, npm hoisting gives
> every package the same single `hardhat` instance — natively.

And the symmetry principle that applies it uniformly:

> **Every hardhat generation is a CLUSTER — its own installation root. The sdk root workspace holds
> no hardhat at all.** No generation is privileged; adding v4 is copying a shape; retiring v2 is
> deleting one directory.

## 1. Target topology

```
sdk/                          ROOT workspace (installation root #1) — no hardhat anywhere in it
│   package.json workspaces: common, common-vendored, host-contracts v12, v13, v13/pkg, (future js-sdk)
│   package-lock.json · node_modules (eslint10, tsc6, ethers/viem pins, NO hardhat)
│
├─ fhevm-npm/                 installation root #2 — the orchestrator CLI (unchanged)
│
├─ hardhat/v2/                installation root #3 — the v2 CLUSTER
│   │  package.json  workspaces: plugin, plugin/pkg, fhevm-hardhat-template, fhevm-hardhat-template/pkg, e2e
│   │  package-lock.json · node_modules (hardhat@2 hoisted ONCE, ethers6, its own toolchain pins)
│   ├─ plugin/                dev owner ── plugin/pkg  published @fhevm/hardhat-plugin
│   ├─ fhevm-hardhat-template/  dev owner + mirror.manifest.json (outbound render spec)
│   │   └─ pkg/               WORKSPACE-NATIVE (option E): no solhint, no own lock, file: → ../../plugin/pkg
│   └─ e2e/                   consumer project
│
└─ hardhat/v3/                installation root #4 — the v3 CLUSTER
    │  package.json  workspaces: plugin, plugin/pkg   (later: + fhevm-hardhat-template, e2e)
    │  package-lock.json · node_modules (hardhat@3 hoisted ONCE)
    └─ plugin/                dev owner ── plugin/pkg  hello-world hardhat 3 plugin
```

Cross-root edges (all npm-native, none hand-made):

- v2 `plugin/pkg` → `@fhevm/host-contracts-cleartext`: `file:../../../host-contracts-cleartext/v13/pkg`
- v2/v3 dev owners → `@fhevm/sdk-common-dev` / `@fhevm/sdk-vendored-dev`: `file:../../../common{,-vendored}`
- inside a cluster, members resolve each other by name/`file:` as today (template pkg → plugin).

Safe because only `hardhat` is a singleton, and it never crosses a root. Every cross-root `file:`
spec is enumerated in the render spec so the published forms map them to registry ranges.

**js-sdk lands in this architecture (planned, shapes the design now).** `@fhevm/sdk` joins the ROOT
workspace — it is a browser/node library with no hardhat dependency, so it belongs with the
contracts, not in a cluster. Every cluster consumes it the same cross-root way: v2/v3 plugins and
templates take `file:../../../js-sdk` specs (live, one-commit lockstep across sdk → plugin →
template — the ultimate goal), and the render maps them to `@fhevm/sdk@^x` for the published forms.
Nothing in this plan may assume a package list frozen at today's members; the N-root machinery of
phase 1 is what makes that landing a manifest edit, not an architecture change.

## 2. Phase 1 — fhevm-npm learns N installation roots (prerequisite for everything)

- Manifest schema: allow the `workspace-root` kind at non-`.` keys (`./hardhat/v2`, `./hardhat/v3`,
  `member: false`), each carrying its own `workspaces`; packages declare `memberOf` (default `"."`).
- Checks to teach, one by one: `check-workspaces` (each root's array ↔ its members),
  `check-lockfiles` (exactly one lock per installation root, none below), `check-names`
  (published-name uniqueness scoped PER ROOT — v2 and v3 payloads may share `@fhevm/hardhat-plugin`),
  `check-dependencies` (`dependencyGroup` per generation; cross-root `file:` specs legal and
  containment-checked), `check-manifest-coverage`, `check-scripts` (verbs identical for cluster
  members), `check-tsconfig-paths` / `check-tsc-mode` (path discovery per root).
- fhevm-npm test fixtures gain a two-root workspace.

## 3. Phase 2 — the v2 cluster, with the template gone workspace-native (option E)

### 3.1 Create the cluster and move the v2 family into it

- New `hardhat/v2/package.json` (private cluster root, the workspaces list above) — hardhat@2,
  and the v2 family's deps, hoist HERE from now on.
- Root `package.json` drops all `hardhat/v2/*` entries from `workspaces`; root lock shrinks; NO
  hardhat remains at the root.
- Dependency spec changes: v2 plugin/pkg's dep on v13/pkg becomes the cross-root `file:` above; dev
  owners' `@fhevm/sdk-common-dev` likewise. Same-cluster references unchanged.
- Makefile: a `run-cluster` macro (`npm --prefix <cluster> run --silent <script> -w <name>`);
  `HH_V2` targets switch to it; `install` adds `npm --prefix hardhat/v2 install`; `purge` adds
  `hardhat/v2/package-lock.json`; `uninstall` needs nothing (the cleaner finds every node_modules).

### 3.2 Undo ALL option-3 plumbing for the template

- Delete from `make install`: the pkg standalone-install line and the two hardhat-shim lines
  (`rm -rf` + `ln -s` — the banned symlink).
- `git rm hardhat/v2/fhevm-hardhat-template/pkg/package-lock.json` — cluster members carry no own
  lock; the PUBLIC lock is regenerated by publish-mirror at render time.
- Manifest: template pkg `member: false` stays, but `memberOf: "./hardhat/v2"`; note rewritten to
  the render doctrine. `purge` banner loses the pkg lock (replaced by the cluster lock).

### 3.3 First workspace-native divergences (only what reality already forced)

- **Drop solhint from the workspace form**: the devDependency, the `lint:sol` script (rechain
  `lint`), `.solhint.json`, `.solhintignore`. solhint's `import-path-check` resolves only the
  package-local `node_modules`; under hoisting it cannot pass without a symlink. The check moves to
  the clone-shaped layer: the RENDERED repo (which gets solhint + configs back) and its CI /
  `test-consumer`. This deliberately reverses the earlier "must call solhint" requirement.
- Consequences: revert this week's "sanctioned-by-mirror-only installed binary" exemption in
  `check-lint-policy` (dead code once no workspace form declares solhint); drop
  `dependencyExceptions: ["solhint"]` from the manifest entry.
- Record every divergence in `hardhat/v2/fhevm-hardhat-template/mirror.manifest.json` — the outbound
  render spec: `packageJson` (identity `fhevm-hardhat-template`, every `file:` → registry range,
  re-add solhint + `lint:sol`), `extras/` (`.solhint.json`, `.solhintignore`, `.github/`,
  `.vscode/`), `exclude`, `lockfile: regenerate`. The existing INBOUND patch table
  (`fhevm-npm/base/mirrors/hardhat-template-v2.ts`) holds the exact values to invert; it retires
  with the old `check-mirror` when `publish-mirror` lands (separate, already-proposed work).
- Everything else stays byte-identical for now — divergence grows only on demand, never for taste.
- Memory/doc updates: GUIDE (`install`/`purge`/reinstall sections: three make-owned locks — root,
  v2 cluster, v3 cluster), Makefile comments, and the `mirror-pkg-read-only` memory note — the
  READ-ONLY rule moves from the workspace copy to the RENDERED artifact.

### 3.4 Verification (phase 2)

From a wiped tree: `make install` (four installs, zero `ln -s` — grep proves it) → v2 cluster holds
exactly one `hardhat`; template compile + lint + TESTS green (runtime `hre.fhevm` proof); plugin and
e2e green; full `make lint`; complete fhevm-npm battery; `true | make uninstall` preview lists both
cluster trees; commit scope clean.

## 4. Phase 3 — the v3 cluster skeleton

- `hardhat/v3/package.json` (private cluster root, `workspaces: ["plugin", "plugin/pkg"]`), own
  lock, hardhat@3 hoisted once. NOT in root workspaces. Slots for `fhevm-hardhat-template` and
  `e2e` are reserved in naming only — both will be born workspace-native under E, never verbatim.
- `plugin/pkg`: ESM-only (`"type": "module"`), `peerDependencies: { "hardhat": "^3" }`, exports
  `./dist/index.js`. `src/index.ts` exports a `HardhatPlugin` object (`id: "fhevm"`) with one
  `hello` task printing a greeting — exact task-builder API confirmed against hardhat 3 docs at
  implementation time (v3 changed it: plugin object + hook handlers, no side-effect
  `extendEnvironment`).
- `plugin` (dev owner): the standard verb set — `compile` (tsc → `pkg/dist`), `lint`, `fmt`,
  `fmt:check`, `clean`, `test`. Test = `node:test` creating a programmatic HRE with the plugin
  loaded, asserting the `hello` task exists and runs, and asserting the singleton
  (`require.resolve('hardhat')` identity agrees across owner and pkg). Package-level
  `eslint.config.js` + `prettier.config.js` referencing the workspace bases.
- Makefile: `HH_V3` vars and `compile/lint/test-hh-v3-plugin` targets via `run-cluster`; `install`
  adds `npm --prefix hardhat/v3 install`; `purge` adds `hardhat/v3/package-lock.json`.
- Manifest entries for the cluster root, dev owner, payload (`dependencyGroup: "hardhat/v3"`).

### Verification (phase 3)

Cluster install from scratch; one `hardhat` directory under `hardhat/v3/node_modules` and none
below members; hello task runs; full battery green; `make install → build → test` end to end.

## 5. Order of execution

1. Phase 1: fhevm-npm N-root awareness (schema + checks + fixtures) — everything else depends on it.
2. Phase 2: the v2 cluster + option-E template (§3) — removes both shims, frees the template.
3. Phase 3: the v3 skeleton (§4).
4. Later, out of scope but slotted: `publish-mirror` (renders the public template repo from §3.3's
   spec), then `hardhat/v3/fhevm-hardhat-template` and `hardhat/v3/e2e` inside the v3 cluster.

## 6. Open questions

1. v3 plugin's published npm name: `@fhevm/hardhat-plugin` as a new major line (recommended — the
   per-root uniqueness scoping in phase 1 makes it legal) or a v3-suffixed name?
2. Which hardhat 3 version to pin the v3 cluster to (`^3.x` floor).
3. The public template repo's solhint CI: the render re-adds solhint — an `extras/.github/workflows`
   file in the render spec must actually run it there; who owns writing it?
