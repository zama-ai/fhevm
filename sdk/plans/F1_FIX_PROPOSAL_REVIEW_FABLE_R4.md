# Fable review round 4 — F1_FIX_PROPOSAL.md

Confirmation pass, not a fresh review. Round 3 declared the document ready subject to five edits; all
five were applied, and this round verifies they landed correctly and left nothing incoherent behind.
Earlier rounds: `F1_FIX_PROPOSAL_REVIEW_FABLE.md`, `_R2.md`, `_R3.md`.

## Verdict

**Ship as-is.** Nothing of substance found, having looked specifically where the deletion could have
broken something. The document is finished — implement it.

## The five round-3 edits

| #   | Edit                         | Status                                                                                                                                                                                                             |
| --- | ---------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| 1   | Mirror-owner eligibility     | **Landed correctly.** Uses the exact recommended formulation, with a matching verification criterion. Verified against `npm-manifest.json`: the one package this mattered for resolves the right way.              |
| 2   | Empty commands               | **Landed correctly.** Rule 3 now _includes_ empty-command scripts in selection and rule 4 rejects them as errors. Select-then-reject is the right shape: error, never omission.                                    |
| 3   | Duplicate-identity criterion | **Resolved via the permitted second branch** — deleted with ordinals. Identity is now `(package, full name)`, so no dedup rule is needed and none remains.                                                         |
| 4   | Snapshot element type        | **Landed.** Defined twice, consistently, as the pair `(package-relative-path, full-script-name)` with the phase included, canonical sort order stated, and a matching criterion.                                   |
| 5   | Ordinal cut                  | **Landed cleanly.** Reduced to exactly the recommended residue: independence stated, sort explicitly "for reproducibility, not to express dependencies", `^\d+-` reserved and rejected, future mechanism deferred. |

## The deletion left no orphans

The highest-risk edit was removing three interlocking rules. Grepped for every removed concept —
`ordinal`, `effective identity`, `fixed-width`, `near-miss`, `dedup` — **zero hits**. Rule 5's sort key
is coherent without ordinals and matches the snapshot's canonical sort. The one surviving "non-empty"
applies to `clean:generated`, which is unrelated and correct. The reserved-syntax criterion matches
`^\d+-` exactly.

**On honesty about enforcement:** the text never claims phase-independence is machine-enforced. It says
there is no ordering mechanism, reserves the syntax so an ordering assumption cannot be smuggled in, and
records the human verification for the current eight generators in migration steps 3–4. That is the same
acknowledged-boundary pattern the document already uses for the TypeScript-entrypoint and
acceptance-naming limits.

## Consistency sweep

Clean. Eligibility is stated identically in the Decision section, the validator rules and the criteria;
the mirror exemption carves out the same object everywhere; `sync-vendored` in `generate-pre` is
consistent with "explicitly Make-managed" (Make lists it, discovery does not); the parity claim of eight
names per version package matches the proposed list (5 pre + 3 post). No term carries two meanings.

## Review series outcome

| Round | Verdict                    | Substantive findings                                                    |
| ----- | -------------------------- | ----------------------------------------------------------------------- |
| 1     | Adopt with changes         | 10, incl. mirror eligibility, unenforceable aggregate ban, gate scope   |
| 2     | Adopt with changes         | 7, incl. the ordinal sort-key contradiction and ineligible-package hole |
| 3     | Ready with 2 textual fixes | 4, all textual; recommended cutting ordinals                            |
| 4     | **Ship as-is**             | none                                                                    |

The document grew 230 → 294 → 346 lines across the first three rounds and then shrank to 342 once
ordinals were cut — the only decrease, and the point at which specification growth stopped outpacing
design clarity.
