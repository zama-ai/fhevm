# Architecture

The repository is a pnpm workspace. Use the existing boundaries when adding behavior.

## Packages

- `packages/toolkit` (`@cli-fhevm-sdk/toolkit`): the library layer. It wraps `@fhevm/sdk` and FHETest with importable flows and must not depend on CLI packages (`commander`, `consola`, `@pnpm/tabtab`).
- `packages/cli` (`cli-fhevm-sdk`): commander command registration and the `fhevm-sdk` binary. It consumes the toolkit through `@cli-fhevm-sdk/toolkit` package imports (root export or deep subpaths such as `@cli-fhevm-sdk/toolkit/flows/input-proof`), never through relative paths.
- `packages/load-test` (`@cli-fhevm-sdk/load-test`): a private operator application, not a publishable SDK library. It owns load models, durable pools, raw relayer wire validation, collectors, reports, baselines, and its Commander surface. It may use public `@fhevm/sdk` entry points directly for SDK-native workload journeys and proof workers, and uses the toolkit for shared network, account, contract, ACL, and value behavior. Load orchestration must not move into toolkit.

The toolkit's `exports` map points at TypeScript source; the CLI build (`tsdown`) bundles the toolkit into `packages/cli/dist/` via `deps.alwaysBundle` while keeping npm dependencies external. Because of that, the toolkit's runtime npm dependencies must also be declared in `packages/cli/package.json`.

## Toolkit boundaries (`packages/toolkit/src`)

- `flows`: orchestration only. Compose config, SDK, contract, ACL, transaction, and value helpers here. Group command families into subfolders such as `fhe-test`, `public-decrypt`, `user-decrypt`, `delegated-user-decrypt`, and `token`.
- `fhevm`: `@fhevm/sdk` adapters and SDK response normalization.
- `fhe-test`: FHETest ABI and contract reads/writes.
- `token`: ERC-7984 confidential token ABI and contract reads/writes.
- `acl`: ACL delegation reads/writes.
- `config`: network registry, runtime config, account loading, and client contexts.
- `values`: clear-value parsing, random values, and JSON serialization helpers.
- `shared`: cross-cutting helpers like progress and transaction waiting.
- `types.ts`: networks, FHE value types, and flow result types shared across packages.
- `index.ts`: the public API barrel; keep new public functions exported from here. Deep subpath exports stay available for lazy loading.

Keep SDK integration on entry points declared by `@fhevm/sdk`; do not resolve or
import files from its private `_esm`, `_cjs`, or internal source layout. Toolkit
permit APIs use seconds, matching `@fhevm/sdk`. Human-facing CLI options may use
days, but must convert at the CLI boundary.

Custom FHEVM chains must include the host-chain `protocolConfig` contract used
by the SDK to resolve the protocol and compatible WASM versions.

## CLI boundaries (`packages/cli`)

- `src/cli`: command registration, argument parsing, stdout/stderr behavior.
- `bin/completion-server.mjs`: static tab completion resolver. It must stay lightweight and must not import TypeScript runtime, SDK clients, config, or flows.

When adding a command, prefer:

1. CLI module in `packages/cli/src/cli/commands`.
2. Flow module in the matching `packages/toolkit/src/flows/<command-family>` folder.
3. Adapter changes only at the boundary being crossed.

Keep raw SDK response casts and raw contract calls out of CLI and flow code when practical.
Keep expensive flow and SDK imports inside command actions, not at CLI module top level.

## Load-test boundaries (`packages/load-test/src`)

- `scenario` and `suite`: validated workload data, exact flow allocation, pool planning, and sequential suite orchestration.
- `runner` and `flows`: scheduler lifecycle plus one executor per relayer flow.
- `relayer`: bounded, runtime-validated raw HTTP protocol adapters. Never trust wire data through TypeScript casts.
- `pool`: durable pool schemas, writers, cursors, and on-chain/off-chain preparation.
- `collectors` and `report`: optional observations, strict versioned report schema, rendering, comparison, and explicit baseline blessing.
- `cli`: argument parsing and lazy action wiring only.

Report and baseline reads must pass the versioned runtime schema. A missing
baseline may skip a comparison; a corrupt or incompatible baseline must fail.
Baseline mutation belongs only to the explicit `baseline bless` command and is
staged only after every suite report validates.
