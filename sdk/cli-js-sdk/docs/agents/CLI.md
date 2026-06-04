# CLI

Use these tools for CLI work:

- `commander` for CLI interface
- `@commander-js/extra-typings` for TypeScript types
- `consola` for Elegant Console Wrapper

Behavioral guidance:

- Keep progress and status logs on stderr.
- Keep the final machine-readable response on stdout as JSON.
- Global options are passed before the subcommand.
- FHETest is the only contract target.
- Networks may target different host chains; do not assume Ethereum Sepolia for every network.
- Keep `fresh` and `cached` naming consistent across decrypt workflows.
- `fresh` creates or stores a new FHETest handle before decrypting.
- `cached` reads an existing FHETest handle from account/type or accepts direct `--handle` values.
- Public decrypt `fresh` stores with `makePublic=true`.
- User decrypt and delegated user decrypt `fresh` store with `makePublic=false`.
- Delegated flows use `PRIVATE_KEY`/`MNEMONIC` for the delegate and `DELEGATOR_PRIVATE_KEY`/`DELEGATOR_MNEMONIC` for the encrypted data owner.
