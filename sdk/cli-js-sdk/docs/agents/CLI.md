# CLI

Use these tools for CLI work:

- `commander` for CLI interface
- `@commander-js/extra-typings` for TypeScript types
- `consola` for Elegant Console Wrapper
- `@pnpm/tabtab` for shell tab completion

Behavioral guidance:

- Keep progress and status logs on stderr.
- Keep the final machine-readable response on stdout as JSON.
- Update `README.md` examples and this guidance when changing command behavior, options, outputs, defaults, or supported flows.
- Global options may be passed before or after subcommands; use `optsWithGlobals()` from the command action context.
- The CLI ships as the `fhevm-sdk` binary (`packages/cli/bin/fhevm-sdk.mjs`, exposed via `pnpm link` run from `packages/cli`) and runs compiled `packages/cli/dist/index.mjs`; run `pnpm run build` before linking or using `pnpm run cli`.
- Use `pnpm run cli:dev` for source-mode CLI checks without rebuilding.
- The workspace-root `.env` is loaded by `packages/cli/src/env.ts` relative to the repository, not the working directory; shell variables take precedence.
- `completion-server` is invoked by tabtab's shell templates; the binary routes it to `packages/cli/bin/completion-server.mjs` before loading `tsx` or runtime flow modules. Keep its stdout limited to completion items.
- Keep completion metadata in `packages/cli/bin/completion-server.mjs` aligned with command help whenever changing commands, options, choices, or descriptions.
- Keep CLI command modules free of top-level flow imports. Runtime flow modules should be loaded with dynamic imports inside `.action()` handlers so help and completion startup stay fast.
- FHETest is the only contract target, except `token transfer`/`token balance`, which target ERC-7984 confidential tokens and require `--contract` since there is no per-network token default.
- Networks may target different host chains; do not assume Ethereum Sepolia for every network.
- Keep `fresh` and `cached` naming consistent across decrypt workflows.
- `fresh` creates or stores a new FHETest handle before decrypting.
- `cached` reads an existing FHETest handle from account/type or accepts direct `--handle` values.
- Public decrypt `fresh` stores with `makePublic=true`.
- User decrypt and delegated user decrypt `fresh` store with `makePublic=false`.
- Delegated flows use `PRIVATE_KEY`/`MNEMONIC` for the delegate and `DELEGATOR_PRIVATE_KEY`/`DELEGATOR_MNEMONIC` for the encrypted data owner.
- `fhe-test inspect` is read-only. Keep raw `--handle` inspection mutually exclusive with account/type inspection.
- `fhe-test inspect --type <type>` defaults `--account` to the wallet address loaded from `PRIVATE_KEY`/`MNEMONIC` when no account is provided.
- `fhe-test init --bulk` calls the contract-level all-types initializer and is mutually exclusive with `--type`.
- `fhe-test init --type <type>` may be repeated to initialize selected types without bulk mode.
- `fhe-test init` returns `transactionHashes` as an array because non-bulk initialization may send one transaction per initialized type.
- `fhe-test op` exposes FHETest operator demos as explicit subcommands, not as a generic `--type` flag. Keep operation names aligned with the underlying behavior, such as `add-uint64` and `xor-uint256`.
- `token transfer` uses `confidentialTransferFrom` when `--from` is set, spending an existing operator allowance instead of the loaded wallet's own balance.
- `token transfer` returns `transferredHandle` because ERC-7984 does not revert on insufficient balance; decrypt it with `user-decrypt` to confirm the transfer amount.
- `token balance --account` defaults to the wallet address loaded from `PRIVATE_KEY`/`MNEMONIC` when omitted.
