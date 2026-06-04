# CLI

Use these tools for CLI work:

- `commander` for CLI interface
- `@commander-js/extra-typings` for TypeScript types
- `consola` for Elegant Console Wrapper

Behavioral guidance:

- Keep progress and status logs on stderr.
- Keep the final machine-readable response on stdout as JSON.
- Update `README.md` examples and this guidance when changing command behavior, options, outputs, defaults, or supported flows.
- Global options are passed before the subcommand.
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
