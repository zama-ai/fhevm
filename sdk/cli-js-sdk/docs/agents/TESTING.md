# Testing

Use pnpm scripts for verification.

```bash
pnpm run typecheck
pnpm run test
pnpm run build
```

For CLI changes, also run relevant help or parser checks, for example:

```bash
pnpm --silent run cli --help
pnpm --silent run cli user-decrypt --help
pnpm --silent run cli delegated-user-decrypt --help
```

For load-test changes, validate its complete private package and inspect the
affected command groups:

```bash
pnpm --filter @cli-fhevm-sdk/load-test run typecheck
pnpm --filter @cli-fhevm-sdk/load-test run test
pnpm load-test --help
pnpm load-test pool --help
pnpm load-test report --help
pnpm load-test baseline --help
```

Avoid network-dependent checks unless the user asks for them or credentials/RPC access are available.

Unit tests must cover unit conversions and SDK adapter request shapes. In
particular, assert permit durations are sent to `@fhevm/sdk` as
`durationSeconds`; type-checking alone does not catch semantic unit regressions.
