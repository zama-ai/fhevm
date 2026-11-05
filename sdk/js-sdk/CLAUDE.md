# js-sdk

## Feature docs

Features are documented in markdown files kept in a `docs/` folder next to the code they describe:

```
<folder>/docs/<kind>-<feature>.md
```

- `<folder>`: the module folder that owns the feature (or its `test/` folder for test docs).
- `<feature>`: a kebab-case name shared by all docs of the same feature (e.g. `relayer-http-request`).
- `<kind>`: what the doc covers:

| Kind           | Content                                                                                                  |
| -------------- | -------------------------------------------------------------------------------------------------------- |
| `design`       | How the feature or concept works: architecture, data flow, key decisions and their rationale.            |
| `requirements` | What the feature must guarantee: rules, invariants, contracts and edge-case behaviour. The source of truth when code and docs disagree. |
| `test`         | How the feature is tested: test strategy, scenarios covered, fixtures, and how to run the suite.          |

Other kinds may be added when needed; follow the same `<kind>-<feature>.md` pattern.

Read the relevant docs before changing a feature or its tests.

## Keeping docs up to date

Every time a feature is added or modified, its docs must be created or updated in the same change: `design` and `requirements` for the behaviour, `test` for the test suite.

Existing features that have no docs yet are exempt until the developers add them. Once a feature has a doc, that doc must be kept up to date.

## The `general` feature

`general` is the repo-wide feature: conventions, rules and design that apply to the whole js-sdk rather than to one module. Its docs live in `src/docs/`, the root of the source tree:

- `src/docs/design-general.md`
- `src/docs/requirements-general.md`
- `src/docs/test-general.md`

`general` docs apply to every feature. A feature's own docs may refine them but must not contradict them.

The top-level `docs/` folder is the published user documentation and is not part of this system; do not put feature docs there.

## Documented features

### general

See [The `general` feature](#the-general-feature).

### relayer-http-request

- [design-relayer-http-request.md](src/core/modules/relayer/module/docs/design-relayer-http-request.md)
- [requirements-relayer-http-request.md](src/core/modules/relayer/module/docs/requirements-relayer-http-request.md)
- [test-relayer-http-request.md](src/core/modules/relayer/module/test/docs/test-relayer-http-request.md)
