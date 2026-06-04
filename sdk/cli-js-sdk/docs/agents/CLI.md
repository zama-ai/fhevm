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
- The CLI ships as the `fhevm-sdk` binary (`bin/fhevm-sdk.mjs`, exposed via `pnpm link --global`); `pnpm run cli` remains equivalent.
- The project-level `.env` is loaded by `src/env.ts` relative to the repository, not the working directory; shell variables take precedence.
- `completion-server` is a hidden command invoked by tabtab's shell templates; keep its stdout limited to completion items.
- Completion suggestions come from walking the Commander tree in `src/cli/completion.ts`; define option value choices with Commander's `.choices()` so completion and `--help` stay in sync.
- FHETest is the only contract target.
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
- `fhe-test op` exposes FHETest operator demos as explicit subcommands, not as a generic `--type` flag. Keep operation names aligned with the underlying behavior, such as `add-uint64` and `xor-uint256`.
