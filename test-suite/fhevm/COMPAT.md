# Compatibility

`src/compat/compat.ts` exists for one narrow job: keep explicitly supported old bundles runnable when a runtime contract changes.

The PR workflow runs one behavioral compat smoke:

- it renders the legacy `coprocessor` and `kms-connector` runtime service definitions for `latest-supported`
- it probes the real legacy images with the generated command/flag contract the CLI would pass to them
- it does **not** fully reproduce the rendered runtime with every compose mount, `env_file`, or entrypoint wiring
- it fails if the old binaries reject the rendered flags or required config shape the smoke checks today

If that smoke fails, do one of these:

1. Add or update a shim in `src/compat/compat.ts`
2. Add or update an explicit incompatibility rule in `src/compat/compat.ts`
3. Intentionally raise the support floor in `src/resolve/target.ts`

Use a shim when the old supported bundle can still run with a small CLI-side adjustment:

- drop a new flag for an old binary
- add a legacy flag or env alias for an old binary

Use an incompatibility rule when the combination should fail early with a clear message instead of reaching boot.

Raise the support floor when the project no longer intends to support that old bundle at all. Today that includes both the simple-ACL floor and the later gw-listener drift-address runtime floor in `src/resolve/target.ts`.

If you change a runtime flag, env contract, or startup assumption for:

- coprocessor
- kms-connector
- relayer / test-suite API coupling

assume you may need one of the three actions above.

Treat `compat-smoke` as a narrow contract probe, not as a full stack replay.
If a change depends on rendered mounts, compose-only wiring, or broader boot semantics, rely on targeted runtime QA in addition to `compat-smoke`.
