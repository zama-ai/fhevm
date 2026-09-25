# fhEVM test engine

A manifest-driven, Cucumber-based engine for fhEVM end-to-end scenarios.

- **Declarative.** Every scenario is a folder with a `scenario.yaml` manifest and a Gherkin `.feature` file. Adding a test means adding a folder. Removing it means deleting the folder or setting `enabled: false`. There is no central registry.
- **Isolated.** Each scenario runs in its own Node.js process, so no Cucumber global and no open resource can leak into another scenario.
- **Auditable.** Every run produces the native Cucumber message stream, a Cucumber HTML report, the console log of each scenario, and a consolidated `report.json` / `report.html`.

The engine runs on Node.js (>= 22) and TypeScript compiled with `tsc`. No TypeScript loader is involved at runtime. Cucumber always loads plain JavaScript from `dist/`.

> **Status:** phase 0 (foundation). The engine is fully functional, but no fhEVM scenario has been ported yet. The only scenario is `smoke.arithmetic`, which has no dependency on the fhEVM stack. Environment profiles, capabilities, the chain/FHE modules and evidence capture come in the next phases.

## Quick start

```bash
cd test-suite/test-engine
npm ci
npm run build                       # compiles src/, scenarios/ and test/ to dist/

npm run engine -- list              # discover and validate every scenario
npm run engine -- dry-run           # validate step bindings without executing anything
npm run engine -- run               # run every enabled scenario
npm run engine -- run smoke.arithmetic
npm run engine -- run --tags "@smoke and not @slow"
```

`npm run engine -- <args>` is the same as `node dist/src/cli/main.js <args>`.

> Rebuild after changing any `.ts` file, including step definitions: the engine only loads compiled JavaScript. If a scenario's support code has not been compiled, `list` and `run` report it.

## Terminology

| Term                 | Meaning                                                                                                                                    |
| -------------------- | ------------------------------------------------------------------------------------------------------------------------------------------ |
| **Scenario**         | A folder with a `scenario.yaml`. This is the unit that the engine selects, isolates, times out and reports.                                |
| **Gherkin scenario** | A `Scenario:` (or one `Examples` row of a `Scenario Outline:`) inside the feature file. A scenario contains one or more Gherkin scenarios. |
| **Support code**     | The TypeScript step definitions (and optional World) of a scenario.                                                                        |

## Adding a scenario

1. Create a folder under `scenarios/`, e.g. `scenarios/my-area/my-scenario/`.
2. Add `scenario.yaml`:

   ```yaml
   apiVersion: fhevm.zama.ai/scenario/v1
   kind: Scenario

   metadata:
     id: my-area.my-scenario
     name: What this scenario verifies, in a few words
     owner: my-team
     description: A sentence or two describing what is verified and why.
     tags: [my-area]

   spec:
     runtime: cucumber
     entrypoint: ./my-scenario.feature
     supportCode:
       - ./steps.ts
     timeoutSeconds: 120
   ```

3. Add the feature file and the step definitions it needs (see `scenarios/smoke/` for a complete example).
4. Run `npm run build && npm run engine -- dry-run my-area.my-scenario`. This checks the manifest, the Gherkin syntax, and that every step is bound to exactly one definition.
5. Run it: `npm run engine -- run my-area.my-scenario`.

### Rules for support code

- **Do no work at import time.** Support code is imported even by `dry-run`. Connecting to an RPC, reading environment variables or creating SDK clients belongs in steps, hooks or the World, never at module top level.
- **Keep state in the World.** Cucumber creates a fresh World for every Gherkin scenario. Do not keep module-level mutable state.
- **Gherkin scenarios are independent.** A Gherkin scenario must not rely on state left by another one. Put shared setup in a `Background`.
- **Scenario globals are safe.** `setWorldConstructor`, `setDefaultTimeout` and `defineParameterType` only affect the calling scenario, because each scenario has its own process. A scenario that calls `setDefaultTimeout` overrides the default derived from `timeoutSeconds`.

## Manifest reference

The schema is [`schema/scenario.v1.schema.json`](schema/scenario.v1.schema.json) (JSON Schema draft-07). Point your editor's YAML extension at it to get validation and completion.

**Every field is enforced at runtime, and unknown fields are rejected**, so a manifest never carries a field that does nothing. Fields planned for later phases (`targets`, `requires.capabilities`, …) will be added to the schema together with the code that applies them.

| Field                  | Required | Description                                                                                                            |
| ---------------------- | -------- | ---------------------------------------------------------------------------------------------------------------------- |
| `apiVersion`           | yes      | `fhevm.zama.ai/scenario/v1`                                                                                            |
| `kind`                 | yes      | `Scenario`                                                                                                             |
| `metadata.id`          | yes      | Unique, stable id: lowercase words separated by `.` or `-` (e.g. `erc20.transfer`). Used for selection and in reports. |
| `metadata.name`        | yes      | Human-readable name.                                                                                                   |
| `metadata.owner`       | yes      | Owning team.                                                                                                           |
| `metadata.description` | yes      | What the scenario verifies.                                                                                            |
| `metadata.tags`        | yes      | Selection tags, written **without** `@` (may be empty).                                                                |
| `metadata.origin`      | no       | The original test this scenario was ported from (for traceability of behavioural equivalence).                         |
| `spec.runtime`         | yes      | `cucumber`                                                                                                             |
| `spec.entrypoint`      | yes      | Feature file, relative to the scenario folder. Must stay inside the folder.                                            |
| `spec.supportCode`     | no       | TypeScript files, relative to the scenario folder. The engine loads their compiled `.js`. Default `[]`.                |
| `spec.timeoutSeconds`  | yes      | Wall-clock budget of the scenario (integer, 1–86400). See [Timeouts](#timeouts).                                       |
| `spec.enabled`         | no       | `false` keeps the scenario on disk but always skips it, with the reason shown in the report. Default `true`.           |

Discovery also rejects duplicate ids, missing files, paths that escape the scenario folder, and support code that has not been compiled. If any manifest is invalid, `run` and `dry-run` refuse to start (exit code 2) and `list` prints every problem.

## CLI reference

```
test-engine <command> [scenario-id...] [options]

Commands:
  list       List discovered scenarios (validates every manifest)
  run        Run the selected scenarios, one isolated process per scenario
  dry-run    Validate the selected scenarios without executing any step
  help       Show the usage

Options:
  -t, --tags <expr>          Cucumber tag expression on manifest tags, e.g. "@smoke and not @slow"
      --scenarios-dir <dir>  Directory scanned for scenario.yaml files (default: <package>/scenarios)
  -o, --output-dir <dir>     Run output directory (default: <package>/reports/<run-id>)
      --json                 list: print JSON instead of a table
  -v, --version              Print the engine version
```

**Selection.** Without ids or tags, every scenario is selected. Ids and `--tags` can be combined, and a scenario must match both. Tag expressions use the [Cucumber syntax](https://github.com/cucumber/tag-expressions) (`and`, `or`, `not`, parentheses), with `@` in front of each manifest tag. An unknown id, an invalid expression, or an empty selection is an error.

**Exit codes**

| Code | Meaning                                                                                                         |
| ---- | --------------------------------------------------------------------------------------------------------------- |
| 0    | Every selected scenario passed (dry-run: is valid). Skipped scenarios do not fail a run.                        |
| 1    | At least one scenario failed, timed out or errored (dry-run: is invalid).                                       |
| 2    | Usage or configuration error (bad option, invalid manifest, unknown id, empty selection). Nothing was executed. |
| 130  | Interrupted (Ctrl+C / SIGTERM). The report of what already ran is still written.                                |

### `dry-run`

`dry-run` is a static validator. For each selected scenario, it validates the manifest, parses the feature file, loads the support code, and checks that every step matches exactly one step definition. It **executes no step and no hook**. It also shows the plan: what would run and what would be skipped, and why.

A scenario is **invalid** when it has a Gherkin parse error, an undefined step, an ambiguous step, or an undefined parameter type. The engine applies this rule itself, because Cucumber reports these cases as successful in dry-run mode.

`dry-run` validates the shape of a scenario, not its behaviour. Wrong values, network failures and failed assertions only show up with `run`.

## Outputs

Each `run` / `dry-run` writes a run directory (default `reports/<run-id>/`, ignored by git):

```
reports/20260925-131935-run-42be/
├── report.json                    # consolidated, machine-readable result (contract below)
├── report.html                    # self-contained human-readable report
└── scenarios/
    └── smoke.arithmetic/
        ├── messages.ndjson        # native Cucumber message stream (canonical source)
        ├── cucumber-report.html   # native Cucumber HTML report
        └── console.log            # the scenario's console output, without ANSI colours
```

`report.json` (`schemaVersion: 1`, type `RunReport` in [`src/report/report.ts`](src/report/report.ts)) contains:

- `engine`: engine name and version, Node version;
- `run`: id, mode, start and end times, duration, selection filters;
- `summary`: counts per outcome and the overall `success`;
- `scenarios[]`: manifest metadata, `outcome`, `reason`, timings, worker exit code/signal, artifact paths (relative to the run directory), and the parsed Cucumber execution. The Cucumber execution lists each Gherkin scenario with its steps (keyword, text, line, status, duration, error) and the problems found (parse errors, undefined or ambiguous steps).

Scenario outcomes:

| Outcome     | Meaning                                                                                               |
| ----------- | ----------------------------------------------------------------------------------------------------- |
| `passed`    | Cucumber reported success (dry-run: valid).                                                           |
| `failed`    | A step failed, or a step is undefined or ambiguous, or the feature does not parse (dry-run: invalid). |
| `skipped`   | Not executed. `reason` says why (e.g. `spec.enabled: false`, run interrupted).                        |
| `timed-out` | The scenario exceeded its budget and was stopped. The partial message stream is kept.                 |
| `error`     | The worker could not run Cucumber, e.g. support code throwing at import. `reason` holds the error.    |

## Timeouts

`spec.timeoutSeconds` is applied at two levels:

1. **Default step timeout.** It becomes Cucumber's default step timeout inside the scenario process. A single step that hangs fails with a clear Cucumber timeout error, and the scenario is reported as `failed`.
2. **Scenario budget.** If the whole scenario is still running `timeoutSeconds + 5 s` after it started (for example, many slow steps that each fit in the step timeout), the engine sends `SIGTERM`. The worker flushes what it has recorded and exits, and the scenario is reported as `timed-out`. A worker that does not exit within 3 s (e.g. a step blocking the event loop) receives `SIGKILL`.

The 5 s grace lets Cucumber report a hanging step itself before the hard stop.

## Architecture

```
            ┌──────────────────────────── orchestrator process ─────────────────────────────┐
 argv ──▶  CLI ──▶ Discovery ──▶ Manifest ──▶ Planner ──▶ Runner (orchestrator) ──▶ Report
            │        (scan)      (schema)    (select,        │  one child process      (report.json,
            │                                 run/skip)      │  per scenario           report.html)
            └────────────────────────────────────────────────┼───────────────────────────────┘
                                                             ▼
                                            ┌──── worker process (per scenario) ────┐
                                            │ Cucumber API (loadConfiguration +     │
                                            │ runCucumber): engine support code +   │
                                            │ scenario support code + feature       │
                                            │ → messages.ndjson, cucumber-report.html│
                                            └───────────────────────────────────────┘
```

Each component depends only on the contracts (types) of the components before it:

| Component | Code                         | Contract (input → output)                                                                                                          |
| --------- | ---------------------------- | ---------------------------------------------------------------------------------------------------------------------------------- |
| Manifest  | `src/manifest/`              | YAML text → `ScenarioManifest`, or `ManifestError` listing every schema violation. Pure.                                           |
| Discovery | `src/discovery/`             | scenarios directory → `DiscoveryResult` (valid `DiscoveredScenario`s with absolute paths + one error per invalid manifest).        |
| Planner   | `src/planner/`               | scenarios + `SelectionFilters` → `ExecutionPlan` (each item `run` or `skip` + reason). Pure.                                       |
| Runner    | `src/runner/orchestrator.ts` | `ExecutionPlan` → `ScenarioRunResult[]`, exactly one per plan item. Never throws for scenario failures.                            |
| Worker    | `src/runner/worker.ts`       | `WorkerJob` (JSON argument) → Cucumber artifacts + exit code (`0` passed, `1` failed/invalid, `2` engine error, `143` terminated). |
| Report    | `src/report/`                | plan + results + message streams → `RunReport` (`report.json`) → `report.html`.                                                    |
| CLI       | `src/cli/`                   | argv → exit code + printed artifact paths.                                                                                         |

Engine-wide support code (`src/cucumber/support/`) is loaded into every scenario before the scenario's own support code. Today it only applies the default step timeout.

## Development

```bash
npm run build          # clean build to dist/
npm test               # node:test suites (unit + CLI integration), on the compiled output
npm run tsc            # type-check only
npm run prettier:check # formatting (same rules as the repository root)
```

The CLI integration tests run the engine against the fixture scenarios in `test/fixtures/scenarios/`. Those fixtures cover a passing scenario, a failing step, undefined and ambiguous steps, a Gherkin parse error, support code crashing at import, step and scenario timeouts, a disabled scenario, and two scenarios that would clash if they shared a process. `test/fixtures/invalid-scenarios/` holds deliberately invalid manifests.
