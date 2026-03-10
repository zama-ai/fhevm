# fhevm-cli Architecture

This is the high-level shape of the Bun-based `fhevm-cli`.

```mermaid
flowchart TD
  A["fhevm-cli up"] --> B["Resolve target"]
  B --> B1["latest-main: walk main SHAs until complete image set, but not before acfa977"]
  B --> B2["latest-release: latest stable release"]
  B --> B3["sha: exact repo-owned SHA, fail if any package tag is missing"]
  B --> B4["devnet/testnet/mainnet: GitOps bundles"]
  B1 --> C["Lock resolved bundle"]
  B2 --> C
  B3 --> C
  B4 --> C

  C --> D["Preflight"]
  D --> E["Generate runtime files under .fhevm"]
  E --> E1["env/"]
  E --> E2["compose/"]
  E --> E3["locks/"]
  E --> E4["addresses/"]
  E --> E5["state.json"]

  E --> F["Boot pipeline"]
  F --> F1["base"]
  F1 --> F2["kms-signer"]
  F2 --> F3["gateway-deploy"]
  F3 --> F4["host-deploy"]
  F4 --> F5["discover"]
  F5 --> F6["regenerate + validate"]
  F6 --> F7["coprocessor + kms-connector"]
  F7 --> F8["bootstrap"]
  F8 --> F9["relayer"]
  F9 --> F10["test-suite"]

  G["Local overrides (group or runtime service)"] --> E
  H["Multicopro topology + per-instance overrides"] --> E
  I["Compatibility policy"] --> E
  I --> F7

  E5 --> J["resume / from-step"]
  J --> F

  K["fhevm-cli test"] --> F10
  L["fhevm-cli up --dry-run"] --> B
  L --> D
```

## Version Override (CI Integration)

After resolving a target bundle, `applyVersionEnvOverrides` overlays any matching `*_VERSION`
environment variables onto the bundle. This is the mechanism CI uses:

```
resolve target (e.g. latest-release)
  → baseline bundle with release tag for all repo-owned packages
  → applyVersionEnvOverrides(bundle, process.env)
  → env vars like COPROCESSOR_HOST_LISTENER_VERSION=<sha> replace baseline versions
  → lock file records overrides in its "sources" field
```

The merge queue workflow (`test-suite-orchestrate-e2e-tests.yml`) builds Docker images tagged
with the PR's HEAD SHA, exports them as env vars, then calls `./fhevm-cli up --target latest-release`.
The target provides companion defaults (CORE_VERSION, RELAYER_VERSION); the env vars provide
the SHA-tagged images for every component built from the PR.

## Notes

- Version selection is explicit. The CLI does not silently use a vague "latest".
- `latest-main` is modern-only by construction. If no complete bundle exists after the floor SHA, resolution fails.
- The resolved bundle is printed and locked before the real boot continues.
- `.fhevm` is the only mutable runtime area owned by the CLI.
- Discovery is not terminal output only. It feeds env regeneration before dependent services start.
- Resume is step-based via `state.json`, not "rerun the bash ritual and hope".
- `upgrade` is intentionally narrow: it only rebuilds and restarts active runtime override groups.
- `up --dry-run` exercises the same target-aware resolve and preflight path without mutating runtime state.
