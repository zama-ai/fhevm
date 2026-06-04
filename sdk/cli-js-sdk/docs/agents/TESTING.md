# Testing

Use pnpm scripts for verification.

```bash
pnpm run typecheck
pnpm run build
```

For CLI changes, also run relevant help or parser checks, for example:

```bash
pnpm --silent run cli --help
pnpm --silent run cli user-decrypt --help
pnpm --silent run cli delegated-user-decrypt --help
```

Avoid network-dependent checks unless the user asks for them or credentials/RPC access are available.
