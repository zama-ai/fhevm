# ABI Compatibility

ABI coverage is derived from each package's `upgrade-manifest.json`, plus the contracts in
`PREDECESSOR_CONTRACTS` in [`config.ts`](./config.ts). Such a contract replaces a different contract
behind an existing proxy, and the check compares it against that predecessor until the baseline has it.
Stable-surface policy lives in [`config.ts`](./config.ts): initializer, ownership, upgrade,
and other intentionally non-public entrypoints are excluded there.

Compare the stable contract ABI surface between two refs locally with:

```bash
bun ci/abi-compat/list.ts --from v0.11.1 --to v0.12.0-0
```

Limit the scope with:

```bash
bun ci/abi-compat/list.ts --from v0.11.1 --to v0.12.0-0 --package host-contracts
bun ci/abi-compat/list.ts --from v0.11.1 --to v0.12.0-0 --package gateway-contracts
```

Use the lower-level checker when both package directories are already prepared:

```bash
bun ci/abi-compat/check.ts <baseline-pkg-dir> <target-pkg-dir> <host-contracts|gateway-contracts>
```
