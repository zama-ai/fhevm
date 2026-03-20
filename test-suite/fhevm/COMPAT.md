# Compatibility

`src/compat.ts` exists for one narrow job: keep explicitly supported old bundles runnable when a runtime contract changes.

The PR workflow runs one behavioral compat smoke:

- it renders the legacy coprocessor commands for `latest-supported`
- it runs the real legacy images with those exact commands
- it fails if the old binaries reject the rendered flags

If that smoke fails, do one of these:

1. Add or update a shim in `src/compat.ts`
2. Add or update an explicit incompatibility rule in `src/compat.ts`
3. Intentionally raise the support floor in `src/resolve.ts`

Use a shim when the old supported bundle can still run with a small CLI-side adjustment:

- drop a new flag for an old binary
- add a legacy flag or env alias for an old binary

Use an incompatibility rule when the combination should fail early with a clear message instead of reaching boot.

Raise the support floor when the project no longer intends to support that old bundle at all.

If you change a runtime flag, env contract, or startup assumption for:

- coprocessor
- kms-connector
- relayer / test-suite API coupling

assume you may need one of the three actions above.
