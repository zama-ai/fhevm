# Review — `plans/npm-rules.md`

**Reviewed:** 2026-08-29, by a Fable 5 agent, against `plans/npm-rules.md` at 673 lines / 40 rules.
**Toolchain used for experiments:** npm 11.7.0, node 22.20.0. Nothing in the repo was modified; experiments ran in
throwaway workspaces.

Two caveats on how to read this:

- **The document changed while the review ran.** §2.1 was rewritten to be manifest-driven mid-flight. The agent
  re-read the file and every citation below refers to the 40-rule version, but a finding may still be reacting to
  wording that has since moved.
- **Three findings are contested by first-hand evidence from the session that produced the document.** Those are
  adjudicated in the next section rather than left to stand or fall silently. Everything else is passed through as
  the agent reported it.

Verified correct and deliberately not discussed: 3.1.1, 3.2.1's registry claim, 3.3.2, 5.3.4, 6.2.3, and the
numbering (gapless, every `N.N.N` citation resolves).

## 0. Adjudication of contested findings

**Finding 1 (3.1.2) — CONTESTED, needs one experiment to settle.** The agent could not reproduce the `invalid` edge
and concludes npm honours the tgz. But that exact edge was observed directly in this workspace on the same machine:

```
@fhevm/hardhat-plugin@0.4.2 invalid: "file:../../../tarballs/fhevm-hardhat-plugin-0.4.2.tgz"
  from hardhat/v2/e2e -> ./hardhat/v2/plugin/pkg
```

The `-> ./hardhat/v2/plugin/pkg` proves npm resolved to the member and rejected the spec. The likely difference is
**lockfile state**: the repo had a stale, already-conflicted `package-lock.json`, whereas the agent used a fresh
workspace. If that is the explanation, both observations are right and the rule's mechanism is wrong in a different
way than the agent says — the failure needs a pre-existing lock, which the rule should state. Settle by re-running the
agent's experiment against a deliberately stale lock before rewriting 3.1.2.

**Finding 2 (4.2.2) — ACCEPTED, and it supersedes an earlier session result.** "Hoists the majority" was inferred
from an experiment where the majority major also happened to be placed first. The agent varied the order and the
minority won. "Whichever resolution npm places first" is the correct statement, and the agent is right that it
strengthens 4.2.3 rather than weakening it.

**Finding 3 (6.2.1/6.2.2) — PARTIALLY CONTESTED.** The agent could not reproduce "up to date over a broken tree", but
that symptom was observed in this workspace: `npm install` reported "up to date" while four of e2e's dependencies were
absent. What is probably wrong is the **attributed cause** — the trigger was the `file:`-vs-member conflict of 3.1.2,
not a rename or a `workspaces` edit on its own. Rewrite the cause, keep the recovery procedure.

## 1. Confirmed defects

**1. (3.1.2) The stated npm mechanism does not match observed behaviour.** See §0 — contested, but the rule's
justification is wrong either way. The prohibition stands: a tgz spec installs a stale duplicate and the member stops
being tested. Fix: rewrite the mechanism and pin the npm version behind any quoted transcript.

**2. (4.2.2) "npm hoists the majority and nests the odd one out" is false.** Hoisting follows placement order, not
vote count. Fix: "npm hoists whichever resolution it places first and nests the rest."

**3. (6.2.1/6.2.2) Recovery lore is mis-caused.** See §0. Fix: attribute correctly or capture the exact repro.

**4. (4.2.1 vs 4.1.1) Internally unsatisfiable.** 4.2.1 demands redeclaration "at the root's exact version" and flags
any declaration that "is a range" — but the root declares caret ranges for 10 of its 12 shared tools; only `ethers`
and `viem` are exact. A member using `typescript` cannot comply: matching the root byte-for-byte (`^6.0.2`) _is_ a
range, and no exact version is the root's declaration. The known ~28 redeclarations land here (v12: 10, v13: 10,
plugin: 8), including live drift — members' `prettier ^3.6.2` against the root's `^3.8.3` — unflagged, unlike
4.2.3's.
Fix: either 4.1.1 requires every root-shared declaration to be exact, or 4.2.1 requires byte-identical strings. Also
define "root-pinned": the doc never says whether it means all root devDependencies or only the exact ones.

**5. (4.2.1 ✅ example) Stated as fact, false.** "hardhat/v2/e2e imports ethers, so its private manifest repeats the
exact root pin" — e2e imports `ethers` across many test files and declares it nowhere. Today's e2e is the rule's own
❌ case.

**6. (3.2.1 ✅ example) Stated as current, false.** The template's real devDependencies are still the two
`file:...tgz` specs. The flagship no-tarball example describes a state the tree is not in.

**7. (4.2.3) "Gated by comparing members against each other" — no such gate exists.** `check-dep-versions.ts` checks
only ethers/viem, and only in non-published manifests. Fix: future tense, as elsewhere in the doc.

**8. (5.3.1, 5.3.2, 5.1.3) The conventional scripts do not exist.** Zero `test:consumer` or `test:vendored` in any
manifest. The closing "Written, not yet enforced" note covers only the manifest gates; extend it to disclose that a
working validator would fail every dev package today.

**9. (5.3.3) The current design is the rule's ❌ case, and it is load-bearing.** The consumer test today is
`<gen>/test/ts` + `prepare:tarball-consumer` / `test:tarball:run` — a non-member nested under `sdk/`, exactly what
5.3.3 declares not isolated. Worse: `RULES.md` rule 26 and `common/package.json` both name `test:tarball:run` as the
_only_ enforcement of the private-import boundary (5.1.2, itself ungated). So the sole enforcement of 5.1.2 is the
mechanism 5.3.3 outlaws. Fix: add a migration note — do not retire `test:tarball:run` before `test:consumer` exists.

**10. (5.3.8) "never a prerequisite of the daily workspace loop" — violated today.** `hardhat/v2/plugin`'s `build`
ends with `npm run pack:tarball`; v12/v13's `build` ends with `prepare:tarball-consumer`.

**11. (7.1.2 / 1.1) The inventory is incomplete, and an eighth kind exists.** Discovery as 7.1.1 defines it finds
**14 manifests under `sdk/js-sdk`** with no manifest entry: `js-sdk/package.json`, 11 nested `src/**` stubs, and
`js-sdk/test/browser-next`. None of the seven kinds fits — they are members of the _outer_ repo workspace. So "Seven
kinds of `package.json` live under `sdk/`" is factually wrong and 7.1.3's set-equality check can never pass. Fix: add
a kind (e.g. `outer-workspace-member`) rather than excluding the subtree — an exclusion is the silent scope-shrink
7.1.1 exists to prevent. Note `js-sdk/test/browser-next` and `test/manual-pack` carry their own lockfiles and are
standalone-shaped.

**12. (1.1 module-type marker) "holding only `type`" contradicts three artifacts.** The real `scripts/package.json`
holds `type` + `private` + a comment block; `npm-manifest.json` declares `private: true` on three marker entries; the
schema permits it; and `RULES.md` rule 26 _requires_ it outside a published package. Fix: markers outside `pkg/` also
set `private: true`; markers inside `pkg/` set `type` alone.

**13. (1.2.1) The rationale is backwards, and the rule retires a documented convention silently.** The stated reason
is that nothing resolves a private package by range — but per 3.1.1 the version _is_ load-bearing and verified by
every install, and `host-contracts-cleartext/v13/package.json`'s comment block documents that as deliberate ("each
harness version must equal its own ./pkg version… Loud is the point"). Adopting 1.2.1 deletes that mechanism. The
rule is defensible; the doc must acknowledge and retire the competing convention rather than ignore it.

**14. Cross-artifact policy conflicts left behind.** `sdk/package.json` ("ethers and viem … must not be redeclared by
a member") and `vendored/package.json` ("ethers/viem are NOT declared: the root pins them") both state the pre-fix
policy that 4.2.1 now inverts. Separately, `RULES.md` still calls itself "the canonical, normative list of rules for
the `fhevm/sdk` workspace" and overlaps npm-rules.md on naming (26 ↔ 5.1.1), the dev/published split (9 ↔ 2.1.2),
globs (24 ↔ 2.1.1) and enforcement. Fix: update the comments, and add a precedence line or scope `RULES.md` to
host-contracts-cleartext.

**15. The doc violates its own conventions.** "payload" appears 9 times (lines 69, 82, 93, 220, 325, 410, 429, 470,
507), three of them in the freshly rewritten §2.1, which also coins two undefined synonyms for "dev package": "dev
owner" (2.1.2, 2.1.3, 5.3.2) and "development package" (2.1.2). Bare "path" at line 55 (should be "member key") and
line 102. "harness" is clean. Also: §5.3 is titled "Tarballs" but holds the tarball-_replacement_ rules; 5.1.3
(vendoring) sits under "Naming and the private boundary"; grammar in 4.2.1 ("requires that range's floor to equal").

## 2. Suggestions

**16. Stale member pins are the biggest ungated risk 3.1.1 creates.** When `plugin/pkg` bumps to 0.4.3, e2e's
`"0.4.2"` silently resolves to registry 0.4.2 nested under e2e — the member drops out of testing with no failure. The
validator should assert that every spec naming a member's name is satisfied by that member's on-disk version.

**17. 3.3.1 has no gate and none planned.** An import satisfied by another member's hoisted copy is caught by nothing.
Extend the planned validator to compare all bare imports against the importer's own manifest.

**18. Peers in the isolated consumer.** npm ≥ 7 auto-installs peers at registry-latest. §5.3 never says the temporary
consumer pins `ethers` / `hardhat` / `hardhat-ethers` / `@fhevm/sdk` to the floors 4.3.1 advertises — without that,
`test:consumer` proves latest-peers, not the floor.

**19. The template's lockfile versus the public mirror.** 2.1.4 requires the template's own lockfile and `npm ci` in
the mirror's CI; 3.2.1 requires a `file:../plugin/pkg` spec. That spec cannot resolve in the mirrored public repo, and
a plain install of a `file:` directory outside a workspace symlinks rather than packs — so the template stops
exercising packed contents unless it installs with `--install-links`. The doc never reconciles spec ⇄ lockfile ⇄
mirror-patch; the tgz specs at least had a mirror-time patch story.

**20. `sdk/`'s install strategy is unpinned yet load-bearing.** The outer `.npmrc` sets `install-strategy=shallow`;
`sdk/` has none, so it runs hoisted — and 3.3.2's `../../node_modules` and 4.1.1's "one hoisted copy" both _depend_ on
hoisted. One well-intentioned `sdk/.npmrc` breaks both silently. Add a rule pinning it.

**21. Nothing wires the conventional scripts to execution.** The validator requires `test:publint` / `test:consumer`
to _exist_; no rule says CI runs them, and root scripts use `--if-present` — the silent-skip failure
`check-pack-scripts.ts`'s own header narrates. Make "and CI invokes them" a numbered rule.

**22. (5.2.1) Decide whether `--ignore-rules` is legal.** v12/v13's real `test:publint` is
`attw --pack ./pkg --ignore-rules false-esm`, exempting the node16-esm cell 5.2.5 leans on. Either the rule permits
documented per-package exemptions, or the workspace is quietly out of spec.

## 3. Suggested order of work

1. **Settle finding 1** with the stale-lock experiment — 3.1.2 is a load-bearing rule and its mechanism is currently
   unverified either way.
2. **Fix 4** (4.2.1 vs 4.1.1) — it is the only defect that makes a rule impossible to satisfy.
3. **Fix 11** — 7.1.3's check cannot pass until the js-sdk manifests are classified.
4. **Fix 5, 6, 7, 8, 10** by flagging them — cheap, and they stop the doc asserting things the tree contradicts.
5. **Fix 9 before retiring anything** — `test:tarball:run` is currently the only enforcement of 5.1.2.
6. Then 12–15, then the suggestions.
