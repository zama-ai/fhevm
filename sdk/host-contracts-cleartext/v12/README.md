# Install

```sh
# remove all node_modules and dependencies etc., only keep package-lock.json
# re-install all
./scripts/install.sh --reset --lockfile=keep
```

# Test

```sh
npm run test
```

# Anvil

Start a new anvil with a fresh deployed cleartext

```sh
./scripts/anvil.sh
```

# Consuming `pkg/forge` from Foundry

`pkg/forge/` holds the forge-only artifacts — everything needed to stand the stack up from Foundry:

| Path                  | Contents                                                                        |
| --------------------- | ------------------------------------------------------------------------------- |
| `src/FhevmDeploy.sol` | the deploy tool, and the **only** file a consumer imports                       |
| `script/`             | forge scripts (`*.s.sol`) — run by path, not imported, so outside the remapping |
| `src/_internal/`      | everything generated — addresses, bytecode blobs, bootstrap args, interfaces    |

It is the Foundry counterpart to `pkg/ts/`: both are optional conveniences, and the Solidity under
`pkg/src/` is still the product. The layout is deliberately Foundry-shaped — `src` and `script` where the
toolchain expects them — so `forge build` and `forge script` work with default config from inside
`pkg/forge`. There is no `test/` here: rule 14 keeps tests out of the payload, so the forge tests for these
tools live in the harness at `test/FhevmDeploy.t.sol`.

The two `LocalHost*.sol` files are halves of one artifact — the bytecode is compiled against exactly those
addresses — so they are regenerated together and `test/templates.test.ts` fails if they drift apart. The
deploy order is not a convenience listing: each address is fixed by `CREATE(deployer, nonce)`, so deploying
in a different order, or from a different account or start nonce, moves every address while the bytecode
keeps pointing at the old ones. The nonces with no named address are the empty-proxy implementations each
proxy is constructed over.

It sits **outside** `foundry.toml`'s `src`, deliberately. Forge therefore never compiles it here, so these
files cannot become inputs to the build that produces them, their pragma is free of the harness's pinned
solc, and a consumer sweeping `src/` does not compile ~139 KB of hex it may never use.

The cost is that forge's automatic `<name>/` → `lib/<name>/src/` mapping cannot reach outside `src/`, so
the consuming layer (`forge-fhevm`) declares one more remapping:

```toml
[profile.default]
remappings = [
    'fhevm-config-0.12.0/=config/',
    'host-contracts-cleartext-forge/=lib/host-contracts-cleartext/forge/src/',
]
```

Then:

```solidity
import {FhevmDeploy, IACL, ACL_ADDRESS} from "host-contracts-cleartext-forge/FhevmDeploy.sol";

contract MyTest is Test, FhevmDeploy {
    function setUp() public { deployFhevm(); }
    function test_x() public view { IACL(ACL_ADDRESS).getVersion(); }
}
```

Everything comes through `FhevmDeploy.sol`: Solidity re-exports imported symbols, so the interfaces and
address constants it pulls in are reachable from it, and `ACL_ADDRESS` stays a compile-time constant rather
than a getter. `src/_internal/` is not API — reaching into it works, since Solidity has no directory
visibility, but nothing there is stable. `LocalHostBytecode.sol` especially: those blobs are pre-compiled
against the canonical addresses, so deploying them by hand bypasses `FhevmDeploy`'s nonce-ordering guards
and produces a stack whose bytecode points at addresses nothing lives at.

Note `_internal/LocalHostAddresses.sol` declares the same constant names as the
`fhevm-config-<version>/addresses.sol` you supply for compiling `pkg/src` — import one or the other into a
given file, or alias.

Two constant flavors, and the difference matters:

- **`_CREATION_CODE`** must be deployed — the constructor either takes arguments or writes storage.
  Deploying these in order from account index 5 of the anvil mnemonic, starting at nonce 0, reproduces
  exactly the addresses in the file's header (rule 17). The bytecode and the addresses are two halves of
  one artifact; mixing in another deployer silently breaks every baked-in reference.
- **`_RUNTIME_CODE`** may be etched at its address, being equivalent to constructing the contract. Only
  `PauserSet` qualifies: everything else either takes constructor arguments, carries an immutable, or calls
  `_disableInitializers()` — a storage write that etching skips, leaving an implementation directly
  initializable where a constructed one is not.

Regenerate with `npm run generate:local-host-bytecode`, which also runs as the last step of
`npm run build:templates`. `test/templates.test.ts` checks the committed output against the templates, so a
stale file fails the suite.

# How to migrate to new host-contracts

The obvious path for any new version. Exceptions are the norm — treat this as the checklist you start
from, not one you can follow blindly.

## 1. Pick the upstream tag and resolve it to a commit

Pick the upstream tag and resolve it to a commit

This package lives _inside_ the fhevm repo, so plain `git` here already targets the right repository.

```sh
# stable tags on the 0.12 line (the [0-9] pattern skips prereleases like v0.12.5-1)
git tag --list 'v0.12.[0-9]'
#   v0.12.0  v0.12.1  v0.12.2  v0.12.3  v0.12.4  v0.12.5

# resolve the chosen tag to the commit it points at
git rev-list -n 1 v0.12.5
#   ac18e49ea85dd3c26788fc66f9ac0ea7cfe48519
```

Both values go into `pkg/package.json` → `fhevm.vendoredFrom` (step 8). The commit matters because a
tag can be moved or re-pointed later, so the commit is what makes the record verifiable (rule 7).

Choosing _which_ tag is a manual decision: numbering on a line is not reliably monotonic — on the 0.13
line `v0.13.3` is an **ancestor** of `v0.13.2`, so the highest patch number is not necessarily the newest
code (rule 6). Confirm before trusting the ordering:

```sh
git merge-base --is-ancestor v0.12.4 v0.12.5 && echo "v0.12.4 is behind v0.12.5"
```

## 2. Sync the vendored sources

Sync the vendored sources

Copy `host-contracts/contracts` into `pkg/src/contracts` — but only the files already vendored here.
Cleartext may carry a **subset**, so adopting a new upstream file is a decision, not a side effect of the
copy (rule 6).

On the 0.12 line there is nothing to leave out: upstream and vendored are both 16 files and
byte-identical. 0.13 adds five (`ProtocolConfig`, `KMSGeneration`, their two interfaces, and
`shared/Structs.sol`) for 21 — which is the shape of a sync that genuinely adopts new upstream files, and
where `npm run check:vendored-origin -- --verbose` checks the declared vendored sources.

## 3. Update the cleartext variants

Update the cleartext variants

Anything in `pkg/src/cleartext/` that extends a synced contract may need the same change —
`CleartextFHEVMExecutor extends FHEVMExecutor`, so an upstream signature change lands here too.

## 4. Addresses, if the host address set changed

Addresses, if the host address set changed

1. `pkg/src/addresses/FHEVMHostAddresses.sol` — add/remove the `*Add` aliases.
2. `ADDRESS_NAMES` in `internal/generateTemplates.ts` — the single definition; `generatePlaceholders.ts`
   imports it.
3. `TARGET_CONTRACTS` in `internal/generateTemplates.ts` — add/remove contracts to template.
4. `internal/placeholders/addresses.sol` is **generated** — do not hand-edit it. Run
   `npm run generate:placeholders` (also runs as the first step of `build:templates`).

## 5. If the protocol minor changed (0.12 → 0.13)

If the protocol minor changed (0.12 → 0.13)

The config remapping prefix is version-pinned, so find every occurrence rather than trusting a list:

```sh
grep -rn --exclude-dir=node_modules --exclude-dir=_cjs --exclude-dir=_esm --exclude-dir=_types \
  'fhevm-config-' .
```

Sweep the whole tree, not just `remappings.txt` and `pkg/src`: there are **four** executable sites —
those two plus `internal/constants.ts` (`FHEVM_CONFIG_REMAPPING_PREFIX`) and `scripts/deploy.sh`
(`CONFIG_PREFIX`) — and two more in prose, the `FOUNDRY_REMAPPINGS=` usage lines in
`pkg/forge/script/FhevmDeployScript.s.sol` and `VerifyFhevmDeploy.s.sol`. A narrower grep found only the
first two and left `deploy.sh` pointing at the previous generation, which fails at _run_ time rather than
compile time. The consuming layer (`forge-fhevm`) declares the new prefix for its own consumers — see
RULES.md rule 11.

## 6. Re-point the previous generation (V(N-1))

**Not applicable to this generation.** v12 is the oldest supported cleartext package, so it has no
previous generation to upgrade from: no `updateV11ToV12`, no upgrade e2e, no previous-generation fixture
(RULES.md rule 21, an explicit exception to rule 10). There is nothing here to re-point.

What that means concretely, and what to restore if v11 is ever added: `pkg/ts/upgrade.ts`,
`internal/prepareTestV11Consumer.ts`, `internal/runUpgradeE2e.ts`, `test/ts/upgrade-e2e.test.ts`,
`test/ts/vitest.e2e.config.ts`, `test/ts/tsconfig.e2e.json`, the `PREVIOUS_GENERATION_*` constants in
`internal/constants.ts`, the `test:upgrade-e2e` script, and a `FhevmAddressesV11` /
`UpdateV11ToV12MigrationConfig` type pair. v13 carries all of it for the v12→v13 path and is the model.

`internal/listUpgradeOps.ts` is deliberately **kept**: it takes the previous generation as a CLI argument
and hardcodes no version, so here it simply has nothing to point at rather than being broken.

## 7. Check the reinitializer versions (do not "fix" them)

Only relevant if existing deployments must be upgraded. This is a **verification**, not an edit: five of
the six contracts carrying a reinitializer are vendored, so rule 6 forbids touching them here — a bump
arrives with the upstream sync (step 2) or not at all.

```sh
grep -rl 'function reinitializeV' pkg/src --include='*.sol'
#   pkg/src/contracts/{ACL,FHEVMExecutor,HCULimit,InputVerifier,KMSVerifier}.sol  ← VENDORED
```

Note this generation's `CleartextArithmetic` declares no `reinitializeV*` at all — it has no predecessor
to be upgraded from. v13 adds one, for the v12→v13 re-point its new nary operators force.

`internal/listUpgradeOps.ts` does the comparison for you — point it at the previous generation and it
reports, per contract, whether the bytecode changed and whether the reinitializer moved:

```sh
npm run list:upgrade-ops -- ../v11   # no v11 exists yet; the helper takes any previous generation
#   contract                 bytecode  initializer                              verdict
#   ACL                      CHANGED   reinitializeV3 -> reinitializeV4         reinitialize
#   CleartextInputVerifier   same      reinitializeV2 -> reinitializeV2         no op
#   CleartextDB              same      - -> -                                   no op
#   KMSGeneration            -         initializeFrom{EmptyProxy,Migration}()   materialize
#   PauserSet                -                                                  not a proxy target
#
#   2 materializations, 5 reinitializations
```

It reads only committed JSON (`templates/` and `abi/`), so it needs no compilation and works against a
published tarball as easily as a sibling checkout. Two details make it trustworthy: it patches both
generations' placeholders to a **common** address set before comparing (the marker values differ per
generation, so a raw comparison would call everything changed), and it keys "is this a proxy target" off
`initializeFromEmptyProxy` rather than off the reinitializer — `CleartextDB`, `KMSGeneration` and
`ProtocolConfig` are proxy targets with no reinitializer at all.

Its output is a starting point, not a spec. It cannot tell you _which_ materializer to call, what
arguments to pass, or in what order — the worked example is v13's `updateV12ToV13`, which calls
`initializeFromMigration` on `ProtocolConfig` (seeding the migrated KMS context) but
`initializeFromEmptyProxy` on `KMSGeneration`. Compare its op list against the upgrade function of the
generation that consumes it (`../v13/pkg/ts/upgrade.ts`); this generation has no upgrade function of its
own (step 6).

The helper takes the previous generation as an argument and hardcodes no version, which is why it
survives a generation change untouched — only the path you pass changes.

A `⚠` verdict means the two signals disagree. `REINITIALIZER_VERSION` is a compile-time constant, and
the rule is two-sided:

| Bytecode changed? | Expected                                                   | If it is wrong                                                             | Reported as             |
| ----------------- | ---------------------------------------------------------- | -------------------------------------------------------------------------- | ----------------------- |
| yes               | `REINITIALIZER_VERSION` bumped, `reinitializeV<n>` renamed | upgrading that proxy has no replay guard and no on-chain generation marker | `⚠ CHANGED, NOT BUMPED` |
| no                | **both untouched**                                         | a gratuitous bump forces a pointless upgrade op and burns a version number | `⚠ BUMPED, UNCHANGED`   |

So a missing bump is not automatically a defect. `InputVerifier` is the worked example: between v12 and
v13 its source changed by 11 lines — five doc-comment lines, named mapping parameters, and one private
constant renamed with an identical value — none of which reach the bytecode. It kept
`MINOR_VERSION = 2` / `REINITIALIZER_VERSION = 3` and is therefore **deliberately absent** from
`updateV12ToV13`'s op list — `../v13/pkg/ts/upgrade.ts` says so directly: _"its v13 bytecode is identical
and its version did not bump"_.

What to check, per contract whose bytecode changed: that upstream bumped it, and that the op list in the
upgrade path matches. A contract whose bytecode did not change should not appear there at all.

Two things that trip people up:

- **The name and the counter differ by one.** `reinitializeV2` is gated by `reinitializer(3)`, because
  `initializeFromEmptyProxy` also consumes `reinitializer(REINITIALIZER_VERSION)`. The function name
  tracks the contract's _minor_ version (`MINOR_VERSION = 2`), not the counter.
- **The bodies are empty.** `function reinitializeV4() public virtual reinitializer(REINITIALIZER_VERSION) {}`
  initializes nothing; its only effect is advancing the counter. The bump is bookkeeping and a replay
  guard, not initialization — which is why an upgrade with empty `initData` is mechanically legal, just
  outside this codebase's convention (see `../v13/plans/FORGE_DEPLOY_SCRIPT_PLAN.md` — the design docs
  live in v13, which is the reference implementation).

## 8. Version and provenance

Version and provenance

- `pkg/package.json` `version`: major.minor must equal the fhevm line, patch is free (rule 5).
- `pkg/package.json` `fhevm.vendoredFrom`: update `tag` and `commit` to step 1 (rule 7).

## 9. Rebuild, then refresh the baseline

Rebuild, then refresh the baseline

Order matters — `generate:patch-sites` reads the templates that `build:templates` produces.

```sh
npm run build:templates      # forge build + abi/ + templates/ + ts/artifacts/ + signers
npm run generate:patch-sites # refresh internal/placeholders/patch-sites.json
```

Review the patch-sites diff rather than accepting it. A count falling to **0** for an address the
contracts still use means the deploy would bake in a placeholder.

## 10. Check the size budget

Check the size budget

New upstream code can breach EIP-170, and the margin is thin — `CleartextFHEVMExecutor` sits ~1.5 KB
under the 24,576 B cap. There is no `--code-size-limit` escape hatch (rule 12).

```sh
forge build --sizes
```

## 11. Verify

Verify

```sh
npm run lint
npm run test                 # includes the forge-vs-template equivalence test
./scripts/anvil.sh           # deploys, then checks the stack matches ZamaConfig.sol
```

The rule 6 gate — vendored sources byte-identical to the declared commit:

```sh
npm run check:vendored-origin             # validate npm-manifest.json vendored sources
#   🔎 rule 6: src/contracts must match host-contracts/contracts at v0.12.5 (ac18e49ea85d)
#      ✅ 16 vendored files identical to upstream (16 upstream files scanned)
```

It reads the tag and commit from `pkg/package.json` → `fhevm.vendoredFrom`, so it can never drift from
what the package claims. `--verbose` also lists upstream files that are not vendored here. It skips
(exit 0) outside a git checkout or when the commit is not fetched, and fails if either side of the
comparison is empty — an extraction that produced nothing would otherwise look like success.

The rules 15/17 gate — the localhost address set still being the one `ZamaConfig.sol` hands out:

```sh
npm run check:zama-config         # also runs inside `npm run build`
#   🔎 rules 15 and 17: ZAMA_LOCAL_CONFIG must match ZamaConfig.sol _getLocalConfig()
#      library-solidity/config/ZamaConfig.sol
#      ✅ ACLAddress           aclAddress             0x50157CFfD6bBFA2DECe204a89ec419c23ef5755D
#      ✅ CoprocessorAddress   fhevmExecutorAddress   0xe3a9105a3a932253A70F126eb1E3b589C643dD24
#      ✅ KMSVerifierAddress   kmsVerifierAddress     0x901F8942346f7AB3a01F6D7613119Bca447Bb030
```

This is the one direction the other address checks do not cover. `generateLocalHostBytecode.ts` asserts
the _derived_ addresses equal `ZAMA_LOCAL_CONFIG`, and `test/templates.test.ts` asserts the generated
forge constants do too — but all of them compare against that same hand-written constant in
`internal/constants.ts`. An upstream edit to `_getLocalConfig()` therefore leaves the whole chain
self-consistent and collectively wrong. This one parses the Solidity instead, so the transcription is
checked against its source.

It reads the file, compiles nothing, and refuses to pass vacuously: an absent `ZamaConfig.sol`, a 31337
branch that no longer routes to `_getLocalConfig()`, a renamed field, or a **new** field are all
failures rather than skips. A new field especially — that is an address the cleartext stack has to place,
and quietly ignoring it would narrow the check to the three fields we happen to know about.

Note it verifies the address _set_, not where a deploy actually lands; that is `./scripts/anvil.sh`
below (and still not part of `npm run test` — see RULES.md rule 17).

## Things that bite

- `test/templates.test.ts` has its own `ALTERNATE_ADDRESSES` fixture; a new host address must be added
  there too or the patching tests fail.
- The bootstrap config types (`BootstrapConfig` and friends in `pkg/ts/types/public.ts`) change
  whenever upstream adds an initializer parameter.
