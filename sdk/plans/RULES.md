# fhevm/sdk — rules

## Context

This file is the canonical, normative list of rules for the `fhevm/sdk` workspace. `PLAN.md` holds the
problem statement, the folder proposal and the step-by-step plan; **RULES.md holds the invariants** —
the constraints any proposed layout, package split or release process must satisfy. When the two
disagree, this file wins.

Rules are numbered and the numbers are stable, so they can be cited from `PLAN.md`, from package.json
comments, from code and from PR descriptions ("required by rule 3"). Add new rules at the end rather
than renumbering.

The rules below concern **`host-contracts-cleartext` and its distribution chain**: where it has to be
available (rules 2–4), how it is versioned (rule 5), how faithful its vendored sources must be
(rules 6–7), what it may depend on (rules 8–9), what its optional TS layer must look like (rule 10), and
how it resolves inside a Foundry consumer (rule 11), where it must be deployable (rule 12), what it must not depend on
(rule 13), what may not live in the payload (rule 14), which addresses a local stack must use
(rules 15 and 17), and what the standalone repo of rule 3 contains (rule 16). Rules 20-22 govern the
relationship _between_ generations: which is the source of truth, which may lack an upgrade path, and how
quickly a change to the reference implementation has to reach the others. Rules 23–26 bind the
**workspace** rather than the package: where a value two languages must agree on is decided (rule 23),
what has to be re-validated whenever a folder is added to `sdk/` (rule 24), what a tsconfig may name
(rule 25), and how a package that is never published is named and marked (rule 26). Most
bind the package itself; rule 11 also binds
the layer that consumes it — `forge-fhevm` — because the config remapping can only be satisfied there.

The shape of the first group is deliberate: one editable source of truth inside the fhevm monorepo
(rule 1), plus several derived distributions (rules 2–4), because each downstream toolchain fetches
Solidity a different way and no single channel reaches all of them:

| Channel             | Reaches                                                            | Rule         |
| ------------------- | ------------------------------------------------------------------ | ------------ |
| npm                 | Hardhat projects, and Foundry projects that resolve `node_modules` | 2            |
| standalone git repo | Foundry projects using `forge install` (git submodules)            | 3            |
| Soldeer             | Foundry projects using the Soldeer package manager                 | 4 (optional) |

Consequences worth keeping in view: the package is Solidity-first, so a distribution is only useful if
the `.sol` sources arrive intact and resolvable — and resolving them is entirely the consuming layer's
job, since the package itself declares nothing (rules 11 and 16); and the same version must be
publishable to every channel from the single source of truth,
which means release automation, not hand-copying.

## Rules

1.  `host-contracts-cleartext` **must stay in the fhevm repo**, which is its single source of truth. It
    is the only editable copy; every distribution in rules 2–4 is a derived artifact, published from it
    and never edited in place.
2.  When released as a product, `host-contracts-cleartext` **must be published to npm** — for Hardhat
    projects, and for hybrid Foundry projects that resolve Solidity through `node_modules`.
3.  When released as a product, `host-contracts-cleartext` **must be available as a standalone GitHub
    repository** — for pure Foundry projects using the legacy `forge install` method. Intended location:
    `https://github.com/zama-ai/host-contracts-cleartext` (does not exist yet).
4.  Optional: when released as a product, `host-contracts-cleartext` **should also be available via
    Soldeer** — for pure Foundry projects using the Soldeer package manager.
5.  The `host-contracts-cleartext` **major and minor numbers must equal the fhevm repo version**; the
    **patch number is free**. So `host-contracts-cleartext` 0.13.x mirrors `host-contracts/` as it
    stands in fhevm v0.13.x — and 0.13.0, 0.13.1, 0.13.2 are successive releases of the package all
    targeting protocol 0.13.

    "fhevm repo version" means the **fhevm release version** — the repo's `vX.Y.Z` git tag — and _not_
    the `version` field of `host-contracts/package.json`, which tracks its own npm line and has read
    `0.10.0` across fhevm v0.12.0, v0.13.0 and v0.14.0 alike.

    Consequences: the patch number is the package's only free component, so it carries every
    cleartext-only fix, and it is the component to bump when the protocol has not moved; the fhevm patch
    level does not propagate, so fhevm v0.13.0 and v0.13.2 both map to cleartext 0.13.x; and several
    generations of the package are expected to exist side by side, each pinned to the protocol it
    targets.

6.  Everything under `src/contracts/` is **vendored from `host-contracts/`** and must be identical to
    **`forge fmt` of** it at one **declared fhevm tag**, recorded in the package. The tag must sit on the
    package's own major.minor line (rule 5). Cleartext-specific contracts live in `src/cleartext/` and are
    out of scope.

    The tag is declared, not derived: patch numbering on a line is not reliably monotonic — `v0.13.3` is
    an _ancestor_ of `v0.13.2` — so choosing it is a manual decision, made once per release and then
    recorded.

    Only the files cleartext actually vendors are covered; it carries a subset, and adopting a new
    upstream file is a deliberate decision.

    **`forge fmt` is the one transformation allowed, and no other.** Vendored sources are _stored_
    forge-formatted, and `scripts/check-vendored-sources.sh` normalises the upstream side before
    comparing. Everything else still holds: no hand edits, not even a "do not edit" header, and no
    other tool may rewrite these files.

    This is not a loosening for convenience. Upstream formats with `prettier-plugin-solidity` and this
    workspace formats with `forge`; the two cannot be reconciled by configuration — 20 prettier configs
    and 13 forge configs were measured and neither converges. A raw byte compare therefore forced the
    directory to be excluded from `forge fmt`, which left two Solidity styles in one tree _and_ left the
    files exposed: the VS Code formatter pipes buffer text to `forge fmt --raw` with no path, so
    `[fmt] ignore` cannot protect them and one save rewrote ~87 lines of `ACL.sol`. Storing them
    forge-formatted makes that save a **no-op**, which is a stronger guarantee than the exclusion it
    replaces.

    What the gate still catches: renamed identifiers, changed types, changed licences, inserted blank
    lines, added or removed code. What it no longer catches: a purely cosmetic upstream reflow.

    Because the comparison depends on `forge fmt` output, the forge version is pinned in
    `.foundry-version` and checked by `scripts/check-forge-version.sh` before this gate runs. A forge
    upgrade changes what rule 6 expects and must be a deliberate, separate step.

7.  The declared tag of rule 6 lives in the **published `package.json`**, under a namespaced `fhevm`
    field. npm ignores unknown top-level fields and carries them through `npm publish`, so the
    declaration ships with the package and is readable by tooling (`npm pkg get fhevm.vendoredFrom.tag`).

    ```json
    {
      "name": "@fhevm/host-contracts-cleartext",
      "version": "0.13.4",
      "fhevm": {
        "vendoredFrom": {
          "repository": "https://github.com/zama-ai/fhevm",
          "tag": "v0.13.2",
          "commit": "07fb05fb75f0aa6cea934088640ddb4539d0b1b9",
          "from": "host-contracts/contracts",
          "to": "src/contracts"
        }
      }
    }
    ```

    `commit` is required alongside `tag`: a tag can be moved or re-pointed, a commit cannot, so the SHA is
    what makes the declaration immutable and the check exactly reproducible. `from` and `to` make the
    mapping explicit rather than implied, so the check needs no knowledge of either layout. The field is
    namespaced under `fhevm` rather than a bare top-level key so it can never collide with a field npm or
    the registry adds later.

    This is what turns rule 6 into a CI gate — extract the declared path at the declared commit, diff it
    against the vendored path, fail on any output:

    ```bash
    git -C "$FHEVM" archive "$COMMIT" "$FROM" | tar -x -C "$TMP"
    diff -r "$TMP/$FROM" "$TO" && echo "vendored sources match $TAG"
    ```

    Note `version` and `vendoredFrom.tag` are independent: rule 5 fixes the package's major.minor to the
    line and frees its patch, so `0.13.4` vendored from `v0.13.2` is normal, not a mismatch.

8.  `host-contracts-cleartext` **`dependencies` may contain Solidity packages only** — packages whose
    payload is `.sol` sources — today `@openzeppelin/contracts` and `@openzeppelin/contracts-upgradeable`,
    and nothing else. The test is what a package _ships_, not what its name suggests. No TypeScript or
    JavaScript runtime package is acceptable there: `viem`, `hardhat`, `ethers` and the like must never
    appear in `dependencies`.

    `devDependencies` are unaffected — the harness needs the full TS toolchain to build and test, and
    `viem`, `vitest`, `tsx` and `typescript` already live there.

    The reason is rules 3–4: a pure Foundry consumer installing through `forge install` or Soldeer has no
    npm install step at all, and a Solidity-only project should never be made to pull a JavaScript
    dependency tree in order to obtain `.sol` files.

    The optional TS tooling under `ts/` is **library-free**: it declares no chain library — not as a
    dependency, not as a peer dependency — and imports none. Reaching a chain only through the abstract
    adapter interfaces the caller implements (rule 10) is what makes that possible. Verified: the
    published `ts/` source has no external imports at all. `viem` exists solely as a harness
    devDependency, for tests.

9.  The **published `host-contracts-cleartext` manifest must be pure**: it declares only consumer-facing
    fields — identity and metadata, entrypoints, `files` / `exports`, `dependencies`, and
    `peerDependencies` where applicable, plus the `fhevm` declaration of rule 7. It carries no
    `devDependencies`, no `scripts`, and no tooling configuration.

    This is the `sdk/js-sdk` mechanism: the harness manifest is private and holds every devDependency,
    every build and test script, and all tool config (`overrides`, `size-limit`, `imports`); the published
    manifest beside the payload holds none of them — it has no `scripts` field at all.

    `.npmignore` cannot achieve this: it filters files, not manifest fields, so a single mixed manifest
    ships its dev-time entries however much is excluded from the tarball. Only a separate payload manifest
    removes them — which is also what makes rule 8 meaningful: a Solidity-only package whose manifest
    advertises `viem` is not pure.

10. `host-contracts-cleartext` exposes a **`ts/` folder with at least a `deploy` and an update function**
    (the update is versioned, e.g. `updateV12ToV13`), so a Hardhat project or plain script can deploy and
    migrate a stack from TypeScript. These are **secondary helpers**: the Solidity is the product, `ts/`
    is optional convenience. One exception: the oldest supported generation exposes `deploy` only, having
    no predecessor to migrate from (rule 21).

    They must be declared with the **same abstract interface system as `sdk/js-sdk`'s ethereum module** —
    `src/core/modules/ethereum/types.ts` and `types-ct.ts` — in which every operation contributes a
    `XxxParameters` / `XxxReturnType` / `XxxModuleFunction` triple, those compose into `EthereumModule`
    and `CleartextEthereumModule`, and the concrete implementation is left to the consumer's web3 library
    (as `src/ethers/internal/ethereum.ts` and `src/viem/internal/ethereum.ts` do). **js-sdk's ethereum
    interface is the source of truth**; any divergence requires an explicit, documented exception.

    The declarations must be **re-declared, not imported**: rule 8 bars a TS dependency, so cleartext
    cannot depend on `@fhevm/sdk` to obtain them. Unlike the vendored Solidity of rule 6 there is no tag
    to diff against, so keeping the two in sync is a review obligation rather than a mechanical check.

11. **`fhevm-config-<version>/` is a deliberate config injection point**: the library imports a promised
    path and the consumer supplies the file, which is what makes compiling against consumer-chosen
    addresses possible. It must never be "simplified" into a relative import — relative imports resolve
    against the importing file and cannot be substituted, so that change would silently remove the
    capability. The version inside the prefix is load-bearing too: it lets two protocol generations
    coexist in one project without their address sets colliding.

    Satisfying it is **layered**, and `host-contracts-cleartext` is not the layer that does it:

    | Layer                                          | Declares                                                    |
    | ---------------------------------------------- | ----------------------------------------------------------- |
    | `host-contracts-cleartext`                     | nothing at all — it ships no `foundry.toml` (rule 16)       |
    | `forge-fhevm`, the dApp-facing Foundry library | `fhevm-config-<version>/`, and installs OpenZeppelin itself |
    | the dApp                                       | nothing; optionally one line to override addresses          |

    That works because `forge install` puts `forge-fhevm` at `lib/forge-fhevm/` — nesting level 1 from the
    default `libs = ['lib']` — and forge discovers a dependency's config at nesting **levels 1 and 2**,
    and only when a `foundry.toml` is present beside it (a bare `remappings.txt` is not read; verified as a
    clean A/B). Declared paths are rebased onto that directory, so
    `fhevm-config-0.13.0/=config/` becomes `fhevm-config-0.13.0/=lib/forge-fhevm/config/`. Verified: a
    dApp whose entire `foundry.toml` is `solc_version` + `evm_version`, with no `libs` and no
    `remappings`, compiles.

    Cleartext must not rely on that discovery **for the config prefix**. Published as `@fhevm/…`, the
    scope directory puts it at depth 2 under `node_modules` and its config is never read — so a
    `fhevm-config-<version>/` declaration would be honoured under `forge install` and silently ignored
    under npm, making the package work in some channels and not others.

    Cleartext need not declare its OpenZeppelin paths either, so it declares nothing whatsoever
    (rule 16). Both channels resolve OpenZeppelin without help: npm auto-maps
    `@openzeppelin/=node_modules/@openzeppelin/`, and a `forge install`-ed OpenZeppelin ships its own
    `remappings.txt` that forge reads and rebases. What cleartext must **not** do is vendor OpenZeppelin
    itself — that pushes it to nesting level 3, past where its self-declaration is read (rule 16).

    Constraint on the declaring layer: **the mapped path must be one it controls.** Owning the `config/`
    directory it maps to makes the mapping independent of how cleartext was installed, npm included. A
    mapping that instead points into cleartext's own install location is only as stable as that location —
    submodule and Soldeer paths are deterministic, whereas npm may hoist the package elsewhere and leave
    the relative path dangling. `forge install` fetches nested submodules recursively, so a submodule
    dependency arrives with the dApp's install.

    A dApp override still wins: `remappings = ['fhevm-config-0.13.0/=my-config/']` at the top level takes
    precedence over the library's. None of this touches Hardhat consumers — they never compile these
    sources, using the `ts/` deploy/update API with the prebuilt `templates/` and `abi/` instead (rule 10).

12. `host-contracts-cleartext` **must be deployable on any EVM chain**, with no node-side concessions.
    Concretely: every contract's runtime bytecode must fit the **EIP-170 limit of 24,576 B**, so the
    stack deploys on a stock node — no `--code-size-limit`, no raised cap, no reliance on a chain that
    happens to be lenient.

    This is a budget on the Solidity, not a setting to tune. When a contract outgrows it the answer is to
    shrink or split it — as the executor split already did — never to relax the node. A stack that needs
    a custom limit is deployable on a dev node and nowhere else, which defeats rules 2–4.

    Headroom today (`forge build --sizes`), largest first:

    | Contract                 | Runtime  | Margin  |
    | ------------------------ | -------- | ------- |
    | `CleartextFHEVMExecutor` | 22,994 B | 1,582 B |
    | `FHEVMExecutor`          | 21,535 B | 3,041 B |
    | `HCULimit`               | 20,231 B | 4,345 B |
    | `KMSGeneration`          | 19,914 B | 4,662 B |

    Nothing exceeds the limit, but the executors are close enough that this belongs in CI:
    `forge build --sizes` fails on a negative margin, so the check costs nothing to keep.

13. `host-contracts-cleartext` **must not depend on `encrypted-types`**.

    It is a Solidity package, so rule 8 would allow it — this is a separate, deliberate exclusion. The
    cleartext stack has no encrypted values to describe, and it already carries the only type enum it
    needs: `FheType` is declared in `src/contracts/shared/FheType.sol` (vendored with the rest of
    `src/contracts/`, rule 6) and imported relatively everywhere it is used. Taking `encrypted-types`
    would add a second, competing source for those types and push an unnecessary install onto every
    downstream Solidity consumer.

    Already satisfied: `dependencies` lists only the two OpenZeppelin packages, and nothing under `src/`,
    `ts/`, `abi/`, `templates/` or `config/` references `encrypted-types`. One stale trace remains — a
    sample remapping in the payload `README.md` (`encrypted-types/=dependencies/@encrypted-types-0.0.4/`)
    that should go.

14. **No tests in the payload.** Nothing under `pkg/` may be a test — no `*.t.sol`, no `*.test.ts`, no
    `test/` directory. Every test lives in the harness (`test/`, `test/ts/`), which is never published.

    The reason is mechanical, not tidiness: a test needs `forge-std` or `vitest`, and rule 8 forbids both
    in the payload's `dependencies`. A test shipped inside the package would therefore carry an import no
    consumer can resolve — and a Foundry consumer that sweeps the package directory for sources would try
    to compile it.

    Satisfied today: 0 test files under `pkg/`, and the 8 tests all sit in `test/` and `test/ts/`. Note
    the payload's `files` no longer carries the `!**/test/**/*` negation the original v13 had. That is
    consistent with this rule — there is nothing to exclude — but it removes the safety net, so the rule
    is now the only thing standing between a stray test file and the published tarball. Cheap to enforce:
    fail the build if `find pkg -name '*.t.sol' -o -name '*.test.ts'` returns anything.

15. in localhost mode (deployed on anvil with fhevm mnemonic), addresses MUST be:
    - ACLAddress: 0x50157CFfD6bBFA2DECe204a89ec419c23ef5755D,
    - CoprocessorAddress: 0xe3a9105a3a932253A70F126eb1E3b589C643dD24,
    - KMSVerifierAddress: 0x901F8942346f7AB3a01F6D7613119Bca447Bb030

16. The standalone repository of rule 3 **contains the contents of `pkg/` and nothing else**, with `pkg/`
    as the **repo root** — not as a subdirectory. So the repo root holds `src/`, `abi/`, `templates/`,
    `ts/`, `package.json` and `README.md`, and none of the harness: no `internal/`, `test/`, `scripts/`,
    `plans/`, no `foundry.toml`, no `tsconfig*.json`, no eslint or prettier config, no
    `package-lock.json`, `dependencies/`, `soldeer.lock`, `cache/` or `out/`.

    Root, not subdirectory, is load-bearing, and is **the only structural requirement**: forge places an
    installed dependency at `lib/<name>/` and auto-generates `<name>/=lib/<name>/src/`. With `src/` at the
    root a consumer writes `import "host-contracts-cleartext/contracts/ACL.sol"`; nested one level, every
    consumer import would carry a `pkg/` segment forever. It also puts `package.json` at the root, so the
    one repo serves npm (rule 2) and Soldeer (rule 4) without a publish-time move.

    Cleartext's sources import two things it does not ship, and **both belong to the consuming layer**
    (`forge-fhevm`), not here:

    | Import                                     | Supplied by                                                       |
    | ------------------------------------------ | ----------------------------------------------------------------- |
    | `@openzeppelin/contracts{,-upgradeable}/…` | whichever layer runs `forge install` for OpenZeppelin — see below |
    | `fhevm-config-<version>/addresses.sol`     | the consuming layer, through the injection point of rule 11       |

    OpenZeppelin needs no declaration from anyone: its repository ships a `remappings.txt` containing
    `@openzeppelin/contracts/=contracts/` beside a `foundry.toml`, so a `forge install`-ed OpenZeppelin
    **describes itself** and forge rebases it onto the install directory. Verified end to end on forge
    1.5.1 through a real three-layer chain (dApp → consuming layer → cleartext → OpenZeppelin): with
    cleartext completely bare, the dApp compiled while declaring nothing at all.

    Two consequences that decide this rule:

    - **Cleartext must ship no `foundry.toml`.** There is nothing for it to declare. OpenZeppelin
      self-describes, and the config prefix must _not_ be declared here (rule 11) — declaring it is what
      would defeat the injection point.
    - **Cleartext must ship no OpenZeppelin submodule either.** That is not merely redundant, it is
      broken: forge reads a dependency's own config at nesting levels 1 and 2 only, so OpenZeppelin
      vendored inside cleartext lands at level 3 in the real chain, where its self-declaration is never
      read. Verified as a clean A/B — with cleartext's own `foundry.toml` deleted and its OpenZeppelin
      submodule left in place, the `@openzeppelin/contracts/` mapping disappears entirely and the build
      fails. The two artifacts are a package deal, and the right number of them is zero.

    Rewriting those imports to a cleartext-owned prefix is not an option either — `src/contracts/` is
    vendored byte-for-byte (rule 6), so the import paths are upstream's to choose.

    Beyond remappings, a library cannot impose compilation settings on a dApp: only remappings merge
    upward from a dependency's config. Measured: a library-declared `fs_permissions` is ignored outright.
    So cleartext's two compile-time floors are the dApp's to satisfy, and are documentation plus a loud
    compiler error rather than anything the package can enforce — **solc ≥ 0.8.24** (31 of 32 payload
    files are `^0.8.24`; OpenZeppelin itself needs only `^0.8.20`) and **Cancun**, because
    `KMSVerifier.sol` uses `tstore`/`tload` in inline assembly. Verified: solc 0.8.24 with
    `evm_version = 'shanghai'` fails, with `cancun` compiles, and Cancun is already the default for
    solc 0.8.24 and later.

    The repo is a **derived, read-only mirror** (rule 1): it is published to, never developed in. Nothing
    in it can be regenerated from it — `templates/` and `abi/` come from `build:templates` in the
    harness, and the rule 6/7 vendoring gate is a harness script — so all verification happens in fhevm
    **before** the mirror is written. There is nothing to build and, by rule 14, nothing to test.

    Satisfied today: `pkg/` holds exactly `src/ abi/ templates/ ts/ package.json README.md` plus
    `.npmignore`, with `src/` at its root and no `foundry.toml` or submodules — which is precisely what
    this rule requires. The one cleanup is `.npmignore`, which still lists `internal`, `test`,
    `tsconfig*.json`, `eslint.config.js` and `PLAN.md`, paths that never exist under `pkg/` and cannot
    exist in the standalone repo either; rule 9's `files` allowlist already covers both channels.

17. The **default `deploy()`** of `host-contracts-cleartext` must always produce a stack whose addresses
    match the local configuration in **`library-solidity/config/ZamaConfig.sol`** — the
    `_getLocalConfig()` branch, gated on `block.chainid == 31337`:

    | `ZamaConfig` field   | cleartext contract | Address (rule 15)                            |
    | -------------------- | ------------------ | -------------------------------------------- |
    | `ACLAddress`         | `ACL`              | `0x50157CFfD6bBFA2DECe204a89ec419c23ef5755D` |
    | `CoprocessorAddress` | `FHEVMExecutor`    | `0xe3a9105a3a932253A70F126eb1E3b589C643dD24` |
    | `KMSVerifierAddress` | `KMSVerifier`      | `0x901F8942346f7AB3a01F6D7613119Bca447Bb030` |

    Note the naming: `ZamaConfig`'s `CoprocessorAddress` **is** the `FHEVMExecutor` address. The two
    names describe the same contract, and conflating them with some other component is the easiest way
    to get this wrong.

    `ZamaConfig.sol` is the **source of truth**, not this table — it is a library dApps inherit
    (`ZamaEthereumConfig` and friends), so those three literals are compiled into every consumer's
    bytecode. A dApp pointed at a local node cannot be reconfigured after the fact: if the stack lands
    anywhere else, every such dApp silently calls addresses holding no code. That is why this binds the
    _default_ path specifically — the one a developer gets by running the deploy with nothing specified.

    **Default** is the operative word. A caller who passes explicit precomputed addresses, or targets a
    chain id other than 31337, has opted out and is outside this rule. What may not happen is the
    zero-configuration local deploy drifting off these addresses.

    What makes it hold is CREATE determinism: a contract's address is fixed by the deployer and its
    nonce, so **the deploy order and the deployer's starting nonce are load-bearing**. Reordering
    deploys, inserting a transaction, or changing which account deploys will move every subsequent
    address. Any of those is a breaking change to this rule even when no address is written down
    anywhere, which is what makes it easy to break by accident during a refactor.

    Enforced today by `scripts/anvil.sh`, which deploys a fresh stack and verifies all three addresses
    hold code, carry an ERC-1967 implementation slot, and are wired to each other. Gap worth closing:
    that script is run by hand and is **not** part of `npm run test`, so nothing in CI currently fails
    if the default deploy drifts. `test/ts/deploy-v13.test.ts` asserts the wiring and each contract's
    `getVersion()`, but against derived addresses rather than these fixed ones.

18. The v12 manifest:

```json
"fhevm": {
    "vendoredFrom": {
        "repository": "https://github.com/zama-ai/fhevm",
        "tag": "v0.12.5",
        "commit": "ac18e49ea85dd3c26788fc66f9ac0ea7cfe48519",
        "from": "host-contracts/contracts",
        "to": "src/contracts"
    }
}
```

19. The v11 manifest:

```json
"fhevm": {
    "vendoredFrom": {
        "repository": "https://github.com/zama-ai/fhevm",
        "tag": "v0.11.4",
        "commit": "e3e12705a73cde2c485b1dca2e4668580f147971",
        "from": "host-contracts/contracts",
        "to": "src/contracts"
    }
}
```

20. **`v13/` is the reference implementation**, and every older generation is derived from it. Each is a
    copy of `v13/` carrying the **smallest possible set of edits**: a divergence is legitimate only when
    the protocol generation forces it, and anything else is drift.

    Built today: `v13/` and `v12/`. `v11/` is an empty directory — the rule anticipates it, and the
    manifest in rule 19 is ready for it, but nothing has been derived yet.

    This is a different axis from rule 1. Rule 1 makes the fhevm repo the source of truth for the
    _distribution copies_ of one generation; this rule makes `v13/` the source of truth _across_
    generations. So v11 and v12 are rebuilt from `v13/`, not patched forward from whatever shape they
    previously had.

    What a legitimate diff may contain — everything a generation genuinely fixes:

    - the vendored sources and their declared tag (rules 6–7, and the manifests in rules 18–19)
    - the package name and version (rule 5)
    - the config remapping prefix, `fhevm-config-<MAJOR>.<MINOR>.0/`
    - the address set that generation's contracts actually reference — 0.11 needs five symbols, 0.13 ten
    - which contracts exist at all — 0.11 vendors 16 files with no `ProtocolConfig` or `KMSGeneration`,
      0.13 vendors 21
    - the previous-generation wiring: the upgrade path, its e2e, and the fixture that builds it

    Everything else is copied verbatim: harness layout, the `pkg/` payload split, tsconfigs, eslint and
    prettier config, `scripts/`, the `internal/` generators, and the test structure.

    The point is reviewability. `diff -r v13 v12` should be short enough to read in full, with every hunk
    attributable to one of the causes above; a diff nobody can read is one nobody can audit. Two
    corollaries follow, and they are what keeps the property from decaying: a change made in `v13/` for a
    reason that is **not** generation-specific must be propagated to every derived generation rather than
    left to diverge, and a bug found while working in a derived generation is fixed in `v13/` first and
    then copied down.

    That second half is not ceremony. Deriving v12 surfaced defects that were latent in v13 —
    hand-copied `getVersion()` strings, a hardcoded ACLOwner nonce, twelve hardcoded loop bounds, and a
    test that contradicted the tarball directory it read. Each was correct in v13 by coincidence and
    wrong the moment the stack changed shape. Fixing them in v13 is what stops the next generation
    inheriting them.

21. **The oldest supported generation has no upgrade path.** The property follows the floor: whichever
    generation is oldest ships no update function, no upgrade e2e, and no previous-generation fixture,
    because it has nothing to migrate from.

    **Today that generation is `v12/`.** `v11/` is an empty directory — planned, not built — so v12 is
    the floor and carries the exemption. When v11 lands, the floor moves to it and v12 gains
    `updateV11ToV12` plus the fixture and e2e that go with it. Read every "v11" below as "the floor",
    and note that the concrete file list is the one v12 actually has today.

    Note this is a _scope_ decision, not an upstream fact: fhevm does have a 0.10 line (`v0.10.0`,
    `v0.10.1`, `release/0.10.x`, and lines back to 0.7). What does not exist, and is not planned, is a
    cleartext package for it.

    **This is an explicit exception to rule 10**, which requires `ts/` to expose at least a `deploy` and
    an update function. The oldest generation exposes `deploy` only; there is nothing for an update to
    migrate from, and a stub that reverts would be worse than its absence.

    Under rule 20 this is the largest legitimate divergence in the v11 diff, so it should read as
    expected rather than as drift. v11 **deletes** rather than re-points: `pkg/ts/upgrade.ts` and its
    `updateV12ToV13` export, `internal/runUpgradeE2e.ts`,
    `test/ts/upgrade-e2e.test.ts`, `test/ts/vitest.e2e.config.ts`, and the `FhevmAddressesV12` /
    `UpdateV12ToV13MigrationConfig` types. `internal/listUpgradeOps.ts` is the exception worth keeping —
    it takes the previous generation as an argument, so in v11 it simply has nothing to point at rather
    than being broken.

22. **Every change to `v13/` is ported to every derived generation in the same change.** Immediately, not
    "next time someone touches it" — and the port is part of the change, not a follow-up task.

    Rule 20 already says a non-generation-specific fix must be propagated rather than left to diverge.
    This rule adds the only thing that makes that hold in practice: **when.** "Must be propagated"
    without a deadline is a statement of intent that loses to whatever is being worked on; "in the same
    change" is checkable.

    The exception is narrow and has to be _stated_, not assumed: a change is not ported only when the
    protocol generation makes it impossible or meaningless — the contract does not exist there, the
    address set differs, the generation is the floor of rule 21. When that happens, the divergence goes
    in the register in `PLAN-v12.md` with its cause, so `diff -r v13 v12` stays attributable line by
    line. A difference that is in neither the register nor a legitimate cause is drift, by definition.

    "Not possible" is a much smaller category than it looks. Inert plumbing that a derived generation
    does not currently use is still ported: keeping the shared files byte-identical apart from the
    recorded deltas is what keeps the diff short enough to audit, and today's unused option becomes
    tomorrow's requirement when the next generation is derived. `create2-deploy/common.ts` is the worked
    example — v12 has no upgrade coordinator and reads none of the upgrade's option plumbing, and carries
    it anyway, marked as unused-in-this-generation with the reason.

    Why this is a rule and not a habit: **the two directions are not symmetric.** `v13/` is where the
    work happens, so it is also where the tests run, the e2e executes and the rehearsals are done. A
    derived generation that lags is not merely behind — it is _unexercised_, so its copies rot without
    anything failing. Every defect rule 20's second half describes was found that way, and each had been
    latent in `v13/` for as long as nobody had derived from it.

    The practical form: after changing a shared file in `v13/`, run

    ```sh
    diff -rq v13 v12 --exclude=node_modules --exclude=dependencies --exclude=out --exclude=cache \
      --exclude=tarball --exclude='.out*' --exclude=_cjs --exclude=_esm --exclude=_types \
      --exclude='*.tsbuildinfo' --exclude=broadcast --exclude=state --exclude=plans
    ```

    and account for every line of output before considering the change done.

23. **`sdk/cleartext-config.json` is the only source for a cleartext-stack constant.** Every value the
    cleartext stack's languages must agree on is decided there, once. Two faces are emitted from it and
    nothing else declares these values:

    | Face       | Path                                                                                        |
    | ---------- | ------------------------------------------------------------------------------------------- |
    | TypeScript | `<gen>/internal/cleartext-config.ts`, copied to `<gen>/pkg/ts/cleartext-config.ts` (rule 9) |
    | Solidity   | `<gen>/create2-deploy/script/FhevmCleartextConfig.sol`                                      |

    The JSON sits **above** every generation, at the `sdk/` root, because the generations share it — v12's
    and v13's TypeScript copies are byte-identical today. JSON rather than TypeScript because Solidity
    cannot read a `.ts` module, and neither can a shell script, a Rust crate, or a CI job that wants a
    chain id without installing a toolchain.

    Each face keeps every **name verbatim** — `CLEARTEXT_KMS_NODE_COUNT`, never a locally tidier
    `KMS_NODE_COUNT` — in **declaration order**, and every **value byte-for-byte**.

    The naming half is the part that gets argued with, so it is worth being explicit: inside a file that is
    entirely about the cleartext stack the `CLEARTEXT_` prefix reads as noise, and shortening it is the
    natural thing to do. Refuse it. The identical name is the only thing that makes a drift _findable_ —
    one `grep` has to reach all three files. A renamed face is a copy nobody will think to check, which is
    the same failure this document keeps describing under other headings.

    "Byte-for-byte" includes what looks like a mistake. The mnemonic paths end in `/` because
    `vm.deriveKey(mnemonic, path, index)` derives at `{path}{index}` by plain concatenation — a path
    written `m/44'/60'/0'/2` silently derives `m/44'/60'/0'/20` for index 0. That is a valid path, a real
    key and an entirely wrong signer set, and nothing else in the run notices: the stack deploys, verifies
    against itself, and fails only when the js-sdk relayer arrives with keys at the _documented_ path.
    `FhevmVerify.s.sol` carried exactly that, in a hand-copied constant, until this rule existed. Copy
    values; do not tidy them.

    **A derived value records its formula, not just its result.** Three of these are keccak-derived from a
    fixed string, and the string is the real decision — the hex is only what it hashes to. A hex literal is
    unverifiable by reading: a mistyped digit looks exactly like a correct one, survives review, and then
    every copy agrees with every other. So the JSON carries `formula`, `formulaKind` and `preimage`
    alongside the value, and the check recomputes them.

    Faces are emitted-and-checked-in rather than generated at build time, deliberately: an operator running
    `forge script` against a testnet from an unbuilt checkout must never be blocked on a generator. That is
    safe only because the copies are _checked_. `<gen>/test/cleartext-config-mirror.test.ts` requires each
    face to declare the same names in the same order with equal values, recomputes every formula, and
    requires every address to be EIP-55 checksummed — which Solidity needs anyway (error 9429). It also
    checks the literal _shape_ on the TypeScript side, since a `bigint` and a `number` compare equal
    numerically and behave differently at every call site.

    A constant that genuinely has no counterpart in the other language — a role name, a forge artifact
    path, an EIP-170 limit — is not covered by this rule and belongs wherever it is used. The rule is about
    the values two languages have to agree on, and those are exactly the ones that fail silently.

    **The same file's `localhost` block covers the fixed local-deploy addresses (rules 15 and 17).** Its
    faces are different — `<gen>/internal/constants.ts` and the generated
    `<gen>/pkg/forge/src/_internal/LocalHostAddresses.sol`, not `FhevmCleartextConfig.sol` — so the names
    there are _those_ files' names, including `MNEMONIC`, kept as-is despite reading ambiguously next to
    `FHEVM_MNEMONIC`. Renaming it would hide a real hazard from a `grep` rather than fix it: two mnemonics
    with two different jobs, and swapping them produces a stack whose addresses look right and whose
    signatures never verify.

    Two things differ from the `constants` block, and both follow from what these values are:

    - **The block is keyed by generation, and a generation may not read another's table.** Every address is
      `CREATE(deployer, nonce)`, so it is a function of position and nothing else. Reading a role out of the
      wrong table is therefore silent — every address stays a valid address, just of a different contract —
      which is why each generation selects its own from `FHEVM_CONFIG_REMAPPING_PREFIX` and a missing table
      is a failure rather than a fallback.
    - **Each table is split into `primary` and `secondary`, along the line the code already draws** between
      `HOST_NONCE_OFFSET` and the entries positioned against `HOST_NONCE_COUNT` in
      `<gen>/pkg/ts/addresses.ts`. `primary` is the protocol stack: the two empty-proxy implementations plus
      every contract in the `FhevmAddresses` type. `secondary` is what sits after it — the two
      cleartext-only contracts and `PauserSet`.

      The split is what makes the cross-generation divergence legible instead of a coincidence of
      numbering. Only the primary block changes shape: v13 adds `ProtocolConfig` and `KMSGeneration` to it,
      so the two generations agree up to nonce 6 and v13's entire secondary block shifts by two.
      `0x44aA028f…` is `PROTOCOL_CONFIG_ADDRESS` (primary) in v13 and `CLEARTEXT_ARITHMETIC_ADDRESS`
      (secondary) in v12 — the _same address_, a different contract, not even the same category. The
      categories are positional rather than labels, so the check asserts that `secondary` starts exactly
      where `primary` ends, that only the two empty implementations are unnamed, and that every
      generation's secondary list is the same three roles in the same order.

    - **Nothing in it is transcribed — it is re-derived.** The check derives the deployer from the mnemonic
      at its HD index, then every address from `CREATE(deployer, nonce)`, and compares the generated
      Solidity against the _result_. So the JSON is not trusted either. That matters more here than
      anywhere else in this document: `ZamaConfig.sol` is a library dApps **inherit**, so three of these
      addresses are compiled into consumer bytecode and cannot be reconfigured afterwards. A local deploy
      landing anywhere else leaves every such dApp calling addresses that hold no code — and the deploy
      still verifies, because it checks itself against whatever it produced.

24. **Adding a folder to the `sdk/` workspace is a change to every path-scoped gate, and the codebase
    must be re-validated as part of that change.** Not afterwards, and not when something breaks: a new
    directory is either deliberately inside each gate's scope or deliberately outside it, and both
    answers have to be _decided_ rather than inherited from a glob that happened to match.

    The mechanism is that **every gate in this workspace is scoped by hardcoded paths, and no scope is
    shared with any other.** `sdk/scripts/check-lint-policy.sh` scans
    `host-contracts-cleartext/v*` for banned directives and nothing else, so a new folder at the `sdk/`
    root is simply not looked at — the gate still prints `✅` and now means less than it did. That is
    the dangerous direction: a gate that goes _red_ announces itself, whereas a gate whose reach quietly
    shrank keeps passing and is indistinguishable from one that is working.

    The opposite direction is just as real and is what the workspace has actually been bitten by. Foundry
    applies three different scoping mechanisms to the same tree and **none of them inherits from
    another**: `skip` governs `forge build`, `[fmt] ignore` governs `forge fmt`, `[lint] ignore` governs
    `forge lint`. When the tarball-consumer fixture first appeared under `test/ts/node_modules`,
    `forge fmt` began rewriting the published artifact under test and needed its own `[fmt] ignore`
    entry; `forge lint` kept scanning the same directory long after that, emitting ~40
    `AST source not found` warnings per package — one per fixture file, because `skip` had kept them out
    of the build and left them with no AST — until `[lint] ignore` got an entry of its own. One folder,
    three declarations, each discovered by breakage rather than by design.

    **The register of scope-bearing declarations**, all of which a new folder may belong to or have to be
    excluded from:

    | declaration               | lives in                           | decides                                                          |
    | ------------------------- | ---------------------------------- | ---------------------------------------------------------------- |
    | `workspaces`              | `sdk/package.json`                 | whether npm sees the folder at all — explicit paths, never globs |
    | `skip`                    | `<gen>/foundry.toml`               | whether `forge build` compiles it                                |
    | `[fmt] ignore`            | `<gen>/foundry.toml`               | whether `forge fmt` rewrites it (bare directory names)           |
    | `[lint] ignore`           | `<gen>/foundry.toml`               | whether `forge lint` reports it (file globs — the opposite form) |
    | `libs`                    | `<gen>/foundry.toml`               | where forge resolves imports from (invariant I9)                 |
    | `.prettierignore`         | `<gen>/`                           | whether prettier touches it                                      |
    | `files`                   | `<gen>/pkg/package.json`           | whether it ships (rules 9 and 16)                                |
    | `fhevm.vendoredFrom.to`   | `<gen>/pkg/package.json`           | whether the rule 6 gate covers it                                |
    | scan roots and exclusions | `sdk/scripts/check-lint-policy.sh` | whether the linter ban reaches it                                |
    | `--exclude` list          | rule 22's `diff -rq`               | whether cross-generation drift in it stays visible               |
    | `clean`                   | `<gen>/package.json`               | whether its build output is removed                              |
    | `.gitignore`              | `sdk/`, `<gen>/`                   | whether it is committed                                          |

    **Validated means run, not reasoned about.** `npm run build` in every generation — which chains
    `check:forge-version`, `check:forge-fmt-config`, `check:vendored`, `prettier:check`,
    `forge:fmt:check`, `check:lint-policy`, `forge:lint` and `lint` — plus rule 22's `diff -rq` for a
    folder that exists in more than one generation. A gate that passes is only evidence once you have
    confirmed it actually reached the new directory; the cheap confirmation is to break something inside
    it on purpose and watch the gate fail.

    Two consequences follow. A gate whose scope is a glob over generations (`v*`) absorbs a new
    _generation_ for free but never a new _kind_ of folder, so `v11/` costs nothing here and
    `sdk/rust-sdk/` costs a decision per row of the table. And a folder holding installed or generated
    content — anything named `node_modules`, `dependencies`, `out`, `cache`, `broadcast`, `tarball` — is
    the case that needs exclusions rather than inclusions, which is precisely the case that fails
    silently by rewriting or flagging files nobody owns.

    Vendored sources stay exempt throughout: `pkg/src/contracts/**` is upstream's (rule 6), so a gate
    that would flag it must exclude it instead — a permanently red gate is not enforcement, it is noise
    that teaches everyone to ignore the output.

25. **A tsconfig may not name a path that does not exist.** Every literal entry in `include`, `exclude`,
    `files`, `references[].path` or a relative `extends`, in every `tsconfig.json` and
    `tsconfig.<task>.json`, must resolve to something on disk.

    This is not tidiness. A stale entry is **invisible**: TypeScript ignores an `include` that matches
    nothing as long as another entry matches, and an `exclude` for a deleted file is simply inert. So a
    project goes on reporting success while checking less than it claims — `test/ts/tsconfig.e2e.json`
    named five files, three of which had moved into `@fhevm/sdk-common-dev`, and `tsc` kept exiting 0.

    Prefer a rule to a list. `"exclude": ["./ts/**"]` states which project owns a directory and cannot go
    stale; enumerating the files in it must be re-edited on every rename, and drifts from the same
    enumeration in `eslint.config.js` — which is exactly what happened before this rule existed.

    Globs are exempt: `pkg/ts/**/*.test.ts` matching nothing today is legitimate. So are the build-output
    names in `INTENTIONAL` (`node_modules`, `out`, `cache`, `tarballs`, …) — being absent is the point of
    excluding them. Note that declaring `exclude` at all **replaces** tsc's default list, so
    `node_modules` has to be named again whenever you write one.

    Gated by `sdk/scripts/check-tsconfig-paths.ts`, which runs in the root `check` and therefore in
    `build`. Its scope is the root's own tsconfigs, every declared workspace member and `sdk/scripts` —
    derived from `workspaces`, so a new member is covered with no edit. `sdk/js-sdk` belongs to the outer
    repo workspace and is deliberately out of scope, exactly as `check-dep-versions.ts` scopes itself.

26. **A private package under `sdk/` is named `…-dev` and declares `"private": true`.** Both halves, on
    every manifest in the workspace and its subdirectories that is not itself a published payload.

    The two are not redundant, and neither substitutes for the other. `private: true` is the hard stop —
    npm refuses to publish the package at all — but it is invisible at every call site. The `-dev` suffix
    is the visible half: an import of `@fhevm/sdk-common-dev` from inside a `pkg/` reads as wrong on
    sight, with no manifest to open. That is exactly the boundary rule 14 draws and `test:tarball:run`
    catches after the fact — a published payload that imports a private helper ships an unresolvable
    specifier — and a name that announces itself is what lets a reviewer catch it first.

    The suffix is the **last** segment, with no exception for a generation tag: a per-generation harness
    is `@fhevm/host-contracts-cleartext-v12-dev`, not `…-dev-v12`. Position within the name carries no
    meaning to npm — a harness is reachable as `@fhevm/host-contracts-cleartext-v12-dev/pkg/...` either
    way, and only because harness manifests declare no `exports` (I2) — so the tail is free to be the one
    place the marker always sits. A suffix that moves is a suffix nobody can grep for.

    One carve-out: **a manifest with no `name` is a module-type marker, not a package**, so there is
    nothing to suffix — `sdk/scripts/` and `<gen>/test/ts/` exist only to set `type`. Outside a published
    payload they must still declare `private: true`; inside one (`pkg/_cjs/`, `pkg/_esm/`, `pkg/ts/`) they
    set `type` and nothing else, since npm reads only the payload's top-level manifest.

    The workspace root is exempt: `fhevm-sdk-workspace` is the workspace, not a member of it.

    Ungated. Unlike rules 24 and 25 nothing in `check` verifies this today, so it holds by review only —
    the cheap check is `npm pkg get name private --workspaces`.
