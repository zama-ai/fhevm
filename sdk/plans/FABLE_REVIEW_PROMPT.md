# Build-orchestration review prompt

A reusable prompt for an independent full review of this workspace's build orchestration — the
`Makefile` and the `fhevm-npm` CLI, and nothing else. Kept verbatim so a review can be re-run against a
later state, and so the framing that produces the findings is inspectable rather than buried in a
transcript.

It is deliberately free of point-in-time context. Do not paste "what changed recently" or "known
issues" into the prompt itself — a reviewer told what to skip stops looking there. If a finding is
already known, discard it on the way out, not on the way in.

Past outputs: `FABLE_REVIEW.md`, `MAKEFILE_ORCHESTRATION_REVIEW.md`,
`F1_FIX_PROPOSAL_REVIEW_FABLE.md`.

---

Review the build orchestration of the npm workspace at `/Users/alex/src/me/zama-ai/fhevm/sdk`.

Scope is the orchestration only: the `Makefile` and the `fhevm-npm` CLI (its checks, its rules, and the
`npm-manifest.json` they are driven by). Package source code, contract logic and test contents are out
of scope except where they reveal something about how the build is wired.

**Do not modify anything.** Read-only analysis. `make -n`, `make -q`, `git status`, `git diff`, reading
files, and running individual read-only `fhevm-npm` check subcommands are all fine. Do **not** run
`make ci`, `make clean`, `make rebuild`, `make generate`, `make clean-generated` or `make test` — they
mutate a tree that is actively being worked in. To learn whether something would run, use `make -n`.

## The governing philosophy

Four principles, in priority order. Where they conflict, the earlier wins.

### 1. Robustness over speed

> It is fine to build too much. It is not fine to miss a build or a step. Better to rebuild than to
> risk forgetting to build.

Any optimization that can cause a step to be **skipped** is a defect, even when it is correct today.
An incremental mechanism whose input list is not provably exhaustive is the canonical example: it
converts "slow" into "silently wrong", which is the wrong trade in every case. Unnecessary work is
cheap and visible; a skipped step is expensive and invisible.

Do not reward cleverness here. If something is fast because it takes a shortcut that could go stale,
that is a finding, not a feature.

### 2. Pre-checks and post-checks exist to absorb human error

The orchestration's real job is to make it hard for a person to get this wrong. Assume a competent,
busy developer who forgets things: to regenerate, to rebuild, to commit generated output, to wire a new
script in, to add a new package to a list.

So evaluate the **gates**, not just the graph:

- **Pre-checks** — everything verifiable before a build. Are they positioned early enough to fail fast
  and cheap? Is anything checked late that could have been checked early?
- **Post-checks** — everything that needs build output to be verifiable. Do they actually depend on
  that output, or are they merely _ordered_ after it by luck of invocation sequence?
- Is there a class of human error with no gate at all?

A gate that cannot fail is worse than no gate, because it is believed.

### 3. Automate every check that can be automated

If a rule is written in a document, a comment, or a code-review habit, it is not enforced. Prefer a
check that runs unattended. Prefer a check **derived** from what the code actually does over one
**declared** in a list, because a declared list is a thing a developer forgets to update and its
staleness is silent.

Where a list genuinely cannot be avoided, judge it by which way it fails:

- a list that can only **miss** something (a check list) should be replaced by something exhaustive;
- a list that can **destroy** something (a delete list) must stay explicit, but its consequences must
  be made recoverable.

### 4. Low complexity — it must be easy to understand

This constrains the other three. An over-optimized Makefile is unreadable, and unreadable means
unmaintainable and eventually wrong. Prefer the obvious mechanism over the clever one. A rule someone
can hold in their head and verify by eye beats a subtle one that is technically stronger.

Say so when the current design is _too_ intricate for what it buys, or when a proposed improvement would
cost more comprehensibility than it returns in safety. Complexity that exists only to save time is the
first thing that should go, by principle 1.

Also assess the division of labour: the Makefile should own only what genuinely crosses package
boundaries. Anything a package knows about itself belongs in that package.

## What to examine

1. **Atomic scripts.** Does each npm script do one job? Or do scripts chain clean + build + check, or
   repeat work another phase already did? A script whose name understates what it does is a finding.
2. **Ownership of orchestration.** Is sequencing genuinely in the Makefile, or hidden inside npm
   scripts, lifecycle hooks (`prepare`, `postinstall`, `postcompile`) or shell wrappers?
3. **Phase correctness.** Are generation, formatting, linting, building, checking and testing wired
   with real dependency edges? Test each: would this still be correct if invoked directly, out of
   order, or under `make -j`? An edge that only works because `ci` happens to call things in sequence is
   not an edge.
4. **`make ci` from scratch.** Verify it genuinely starts from nothing and that the gate sequence is
   complete. What can a developer break that no stage would catch?
5. **The `fhevm-npm` rule set.** Which rules are enforced mechanically versus merely documented? Is any
   rule's trigger condition derived, or is it a hardcoded list of packages or paths? Is any rule
   satisfiable by a string that looks right while the underlying behaviour is wrong?
6. **New-thing coverage.** Trace what happens when a developer adds a new package, a new generator, a
   new build output, or a new file type. How many places must be edited by hand, and which of those
   omissions would be silent?

## What to report

Findings ordered by severity, most severe first. For each:

- file and line;
- what is wrong, concretely;
- which principle it bears on (1–4 above, or one of the six examination areas);
- the failure scenario in specific terms: what a developer does, and what silently goes wrong;
- **whether you verified it against the code or are inferring it** — say which, explicitly. A verified
  finding and a suspicion are both welcome; conflating them is not.

Then explicit verdicts — SATISFIED / PARTIALLY SATISFIED / VIOLATED, with reasoning — on:

- atomic scripts;
- Makefile owns orchestration;
- pre-checks and post-checks catch human error;
- checks are automated and derived rather than declared;
- complexity is low enough to understand;
- `make ci` does everything from scratch.

Finally, answer the question that matters most:

> **Where can a developer forget something and have it pass silently?**

Be adversarial. Hunt the silent pass, not the loud failure — a loud failure is already doing its job.
One verified silent-pass path is worth more than ten stylistic observations, so prioritise depth over
breadth, and state plainly when you could not verify something rather than filling the gap with a guess.
