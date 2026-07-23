# Agent Guidance

This is a TypeScript/Node.js pnpm workspace providing a CLI for `@fhevm/sdk` viem flows against FHETest: input proof, public decrypt, user decrypt, delegated user decrypt, and FHETest setup.

Workspace packages:

- `packages/toolkit` (`@cli-fhevm-sdk/toolkit`): importable library layer (config, encryption, decrypt flows, FHETest helpers). No CLI dependencies.
- `packages/cli` (`cli-fhevm-sdk`): commander glue and the `fhevm-sdk` binary; depends on the toolkit via `workspace:*`.

Use pnpm scripts for project commands. Read `docs/agents/ENGINE.md` before package-manager or TypeScript commands.

Focused guidance:

- CLI behavior: `docs/agents/CLI.md`
- Testing: `docs/agents/TESTING.md`
- Architecture: `docs/agents/ARCHITECTURE.md`
