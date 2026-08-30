# Review of npm workspace rules

Review target: [`npm-rules.md`](./npm-rules.md), checked against [`../npm-manifest.json`](../npm-manifest.json),
[`../npm-manifest.schema.json`](../npm-manifest.schema.json) and relevant package manifests.

This is a review snapshot, not normative policy. Findings use stable rule identifiers so edits to the policy do not
invalidate their references.

## Open findings

### 1. High — Target policy is presented as verified current state

The introduction says every rule was verified, while the note after rule 7.1.4 says no gate reads the manifest yet.
Several normative rules describe a target state that package manifests do not currently satisfy, including private
version `0.0.0` and conventional consumer or vendoring scripts.

**Proposed resolution:** Identify the document explicitly as target policy. Replace the claim that every rule was
verified with the narrower statement that npm behavior claims were experimentally verified. Generate current
compliance status separately.

### 2. High — `vendored` is defined as byte-for-byte although derived copies exist

The schema describes every vendored element as a byte-for-byte copy, but an inventoried copy intentionally rewrites an
import. Rule 5.1.3 and `test:vendored` can support transformations, but the data model's definition cannot.

**Proposed resolution:** Define vendored content as copied or deterministically derived, or add a small field such as:

```json
{ "mode": "exact" }
{ "mode": "derived" }
```

### 3. Resolved — Unnamed nested manifests use the `non-package` kind

The former kind was too narrow because some unnamed nested manifests also declare entry-point and type metadata. The
`non-package` kind now classifies any manifest without an independent npm identity while allowing directory-local
metadata.

### 4. High — The manifest is incomplete under the defined inventory universe

Rules 7.1.1 and 7.1.2 now define repository-aware discovery and require every discovered source manifest to be
classified. Repository-aware discovery currently finds source manifests absent from `npm-manifest.json`.

**Proposed resolution:** Populate the inventory completely. If the intended scope is narrower, define stable general
discovery roots rather than relying on undocumented omissions.

### 5. Medium — Rules 5.2.1 and 5.3.2 disagree about implementation freedom

Rule 5.2.1 mandates particular `publint` and `attw` commands. Rule 5.3.2 says the validator does not prescribe script
implementations. Checking only for a non-empty script cannot prove the specific tools ran, while matching command text
would violate the flexibility promised by 5.3.2.

**Proposed resolution:** Separate discovery from semantics. The central validator checks script presence and
ownership; CI executes the script; the normative rule specifies required outcomes. If the exact tools are mandatory,
state that their execution requires a dynamic check and cannot be proven from script presence.

### 6. Medium — The co-release candidate set in rule 5.3.5 is undefined

Rule 5.3.5 requires packages released together to be installed together, but neither the manifest nor another stated
input defines co-release membership. A package-owned consumer test could silently omit a candidate.

**Proposed resolution:** Derive the minimum set from the local published-dependency closure rooted at the payload and
combine it with an explicit release-plan input supplied by CI. Require the consumer test to expose its installed
candidate set so the central runner can compare it with the expected set.

### 7. Medium — Rule 4.3.1 appears to require a root pin for every published dependency

The wording is universal, although a published package may legitimately have a dependency that is neither shared nor
root-pinned.

**Proposed resolution:** Rephrase the rule as:

> When a published dependency or peer is also root-pinned, its range floor equals that root pin.

Define a separate policy if particular dependency names must always be root-pinned.

### 8. Medium — The proposed import scanner has no deterministic source or syntax boundary

Rule 4.2.1 says the future validator collects imports from TypeScript and JavaScript sources, but does not define file
ownership or whether type-only imports, `require`, dynamic imports, package subpaths, generated files and nested
published payloads count.

**Proposed resolution:** Define an AST-based algorithm, derive owned files from checked-in TypeScript configurations
or another explicit source rule, exclude generated output and nested manifest roots, recognize the supported import
forms, and normalize package subpaths to their package names.

## Resolved findings

### Rules 3.3.1 and 4.2.1 were logically incompatible

Resolved by requiring a private package that directly uses a root-pinned dependency to repeat the exact root version.

### Inventory scope was undefined

Resolved by rule 7.1.1's repository-aware discovery definition. Inventory completeness remains open as finding 4.

### Path traversal was accepted

Resolved lexically by traversal-safe schema patterns and structurally by the proposed `realpath` containment check in
rule 7.1.4.

### Schema descriptions claimed invariants the schema did not enforce

Substantially resolved by strengthening kind-local schema constraints and documenting which filesystem, cross-entry
and package-manifest checks belong to the future TypeScript validator. Findings 2 and 3 remain semantic mismatches.

### Rule 2.1.2 contradicted the package taxonomy

Resolved by changing the rule from “a member is a dev package” to “a dev owner stores its published payload in
`pkg/`.” Section 2.1 now derives workspace membership from manifest data rather than embedding a concrete member list.
