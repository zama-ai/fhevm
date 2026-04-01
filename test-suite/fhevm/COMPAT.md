# Compatibility

Compatibility exists for one narrow reason: keep explicitly supported older targets runnable when the CLI evolves.

In practice, this usually means `latest-supported`, or another target the project still intends to support, even though the current CLI may have changed:

- flags
- env names
- startup assumptions
- runtime config shape

The compatibility layer lives in [`src/compat/compat.ts`](src/compat/compat.ts).

## What "compat" means here

- **bundle**: the resolved set of image versions the CLI will boot
- **shim**: a small CLI-side adjustment that keeps an older supported bundle runnable
- **incompatibility rule**: a deliberate fail-fast check for a combination the project does not want to boot
- **support floor**: the oldest bundle or SHA the CLI still intends to support

## What `compat-smoke` actually checks

The PR workflow runs one narrow behavioral smoke:

- it renders the legacy `coprocessor` and `kms-connector` service definitions for `latest-supported`
- it probes the real legacy images with the generated command/env contract the CLI would pass to them
- it fails if those old binaries reject the generated flags or config shape that the smoke covers

It does **not** fully replay the rendered runtime. It does not try to prove every compose mount, `env_file`, or entrypoint wiring path.

So `compat-smoke` answers this question:

> Does the current CLI still speak a compatible command/env contract to the legacy images we explicitly support?

It does **not** answer this stronger question:

> Does the whole legacy stack still boot exactly like a full runtime replay?

For that, targeted runtime QA is still needed.

## What to do when compatibility breaks

When a supported old target breaks, there are only three valid responses:

1. **Add or update a shim** in [`src/compat/compat.ts`](src/compat/compat.ts)
2. **Add or update an incompatibility rule** in [`src/compat/compat.ts`](src/compat/compat.ts)
3. **Raise the support floor** in [`src/resolve/target.ts`](src/resolve/target.ts)

Use a **shim** when the old supported bundle can still run with a small CLI-side adjustment.

Examples:

- drop a new flag for an old binary
- add a legacy flag or env alias for an old binary

Use an **incompatibility rule** when the combination should fail early with a clear message instead of reaching boot.

Use a **support-floor increase** when the project no longer intends to support that bundle at all. Today that includes the simple-ACL floor and the later gw-listener drift-address floor in [`src/resolve/target.ts`](src/resolve/target.ts).

## When to think about compat

If you change any of these, assume compatibility may need to be touched:

- coprocessor flags or startup behavior
- kms-connector flags or startup behavior
- relayer / test-suite API coupling
- env names or required runtime config for older supported images

## Decision rule

- Old supported target still works with a small CLI-side tweak: add a shim
- Old supported target should fail fast with a clear message: add an incompatibility rule
- Old target is no longer meant to be supported: raise the support floor

Treat `compat-smoke` as a narrow contract probe, not as a full stack replay.
