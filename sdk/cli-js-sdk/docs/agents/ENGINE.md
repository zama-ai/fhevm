# TypeScript Engine

Default to Node.js with pnpm. The repository is a pnpm workspace (`packages/*`); run root scripts from the workspace root and use `pnpm --filter <package>` to target one package.

- Use `pnpm install` for dependency installation.
- Use `pnpm run <script>` for package scripts.
- Use `pnpm exec <binary>` for local binaries.
- Use `pnpm dlx <package> <command>` for one-off package execution.
- Use `tsx` for running TypeScript entry points in development.
- Use `tsdown` via `pnpm run build` for the compiled CLI artifact.
- Use `tsc --noEmit` for type-checking.

## APIs

- Read environment variables from `process.env`.
- Prefer built-in Node APIs for filesystem, path, URL, and process behavior.
- This project targets Node.js 22 or newer because `@fhevm/sdk` requires it.
