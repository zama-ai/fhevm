# CI Scripts

## Contract ABI compatibility

Compare the stable contract ABI surface between two refs locally with:

```bash
bun ci/list-abi-compat.ts --from v0.11.1 --to v0.12.0-0
```

Limit the scope with:

```bash
bun ci/list-abi-compat.ts --from v0.11.1 --to v0.12.0-0 --package host-contracts
bun ci/list-abi-compat.ts --from v0.11.1 --to v0.12.0-0 --package gateway-contracts
```

Use the lower-level checker when both package directories are already prepared:

```bash
bun ci/check-abi-compat.ts <baseline-pkg-dir> <target-pkg-dir> <host-contracts|gateway-contracts>
```
