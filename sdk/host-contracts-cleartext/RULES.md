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
(rules 15 and 17), and what the standalone repo of rule 3 contains (rule 16). Most
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

6.  Everything under `src/contracts/` is **vendored from `host-contracts/`** and must be **byte-for-byte
    identical** to it at one **declared fhevm tag**, recorded in the package. The tag must sit on the
    package's own major.minor line (rule 5). Cleartext-specific contracts live in `src/cleartext/` and are
    out of scope.

    The tag is declared, not derived: patch numbering on a line is not reliably monotonic — `v0.13.3` is
    an _ancestor_ of `v0.13.2` — so choosing it is a manual decision, made once per release and then
    recorded.

    Only the files cleartext actually vendors are covered; it carries a subset, and adopting a new
    upstream file is a deliberate decision. No local edits of any kind, not even a "do not edit" header —
    that is what lets a plain `diff` against the declared tag enforce the rule, so `src/contracts/` must
    be excluded from anything that rewrites files (`forge fmt`, prettier).

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

20. **`v13/` is the reference implementation**, and `v11/` and `v12/` are derived from it. Each is a copy
    of `v13/` carrying the **smallest possible set of edits**: a divergence is legitimate only when the
    protocol generation forces it, and anything else is drift.

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
    reason that is **not** generation-specific must be propagated to `v11/` and `v12/` rather than left to
    diverge, and a bug found while working in `v11/` or `v12/` is fixed in `v13/` first and then copied
    down.

21. **The oldest supported generation has no upgrade path.** Today that is `v11/`: there is no cleartext
    v10 to migrate from, so v11 ships no update function, no upgrade e2e, and no previous-generation
    fixture. The property follows the floor — if v11 is ever dropped, v12 inherits it.

    Note this is a _scope_ decision, not an upstream fact: fhevm does have a 0.10 line (`v0.10.0`,
    `v0.10.1`, `release/0.10.x`, and lines back to 0.7). What does not exist, and is not planned, is a
    cleartext package for it.

    **This is an explicit exception to rule 10**, which requires `ts/` to expose at least a `deploy` and
    an update function. The oldest generation exposes `deploy` only; there is nothing for an update to
    migrate from, and a stub that reverts would be worse than its absence.

    Under rule 20 this is the largest legitimate divergence in the v11 diff, so it should read as
    expected rather than as drift. v11 **deletes** rather than re-points: `pkg/ts/upgrade.ts` and its
    `updateV12ToV13` export, `internal/prepareTestV12Consumer.ts`, `internal/runUpgradeE2e.ts`,
    `test/ts/upgrade-e2e.test.ts`, `test/ts/vitest.e2e.config.ts`, and the `FhevmAddressesV12` /
    `UpdateV12ToV13MigrationConfig` types. `internal/listUpgradeOps.ts` is the exception worth keeping —
    it takes the previous generation as an argument, so in v11 it simply has nothing to point at rather
    than being broken.
