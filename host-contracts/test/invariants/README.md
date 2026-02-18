# Host Contracts Invariants

Targeted, scenario-based stateful invariant suites for host contracts.

## Scenario map

- `scenarios/signer-governance/`: signer-set and threshold safety (`InputVerifier`, `KMSVerifier`).
- `scenarios/acl-permissions/`: ACL permission composition, pause, deny-list, and privilege enforcement.
- `scenarios/acl-delegation/`: delegation/revocation state machine and delegated-access coherence.

Each scenario folder has its own `README.md` with invariant IDs and intent.

## Runtime guardrails

- `host-contracts/foundry.toml` uses short deterministic defaults under `[profile.default.invariant]`.
- Handlers use explicit selector allowlists and bounded domains for reproducible runtime.

## Repro commands

```bash
cd host-contracts && forge test --match-path 'test/invariants/scenarios/**/*.sol' -vv
```

Stress sweep:

```bash
cd host-contracts && FOUNDRY_PROFILE=invariant_stress forge test --match-path 'test/invariants/scenarios/**/*.sol' -vv
```
