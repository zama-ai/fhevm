# Architecture

The repository is a pnpm workspace. Use the existing boundaries when adding behavior.

## Packages

- `packages/toolkit` (`@cli-fhevm-sdk/toolkit`): the library layer. It wraps `@fhevm/sdk` and FHETest with importable flows and must not depend on CLI packages (`commander`, `consola`, `@pnpm/tabtab`).
- `packages/cli` (`cli-fhevm-sdk`): commander command registration and the `fhevm-sdk` binary. It consumes the toolkit through `@cli-fhevm-sdk/toolkit` package imports (root export or deep subpaths such as `@cli-fhevm-sdk/toolkit/flows/input-proof`), never through relative paths.

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

## CLI boundaries (`packages/cli`)

- `src/cli`: command registration, argument parsing, stdout/stderr behavior.
- `bin/completion-server.mjs`: static tab completion resolver. It must stay lightweight and must not import TypeScript runtime, SDK clients, config, or flows.

When adding a command, prefer:

1. CLI module in `packages/cli/src/cli/commands`.
2. Flow module in the matching `packages/toolkit/src/flows/<command-family>` folder.
3. Adapter changes only at the boundary being crossed.

Keep raw SDK response casts and raw contract calls out of CLI and flow code when practical.
Keep expensive flow and SDK imports inside command actions, not at CLI module top level.
