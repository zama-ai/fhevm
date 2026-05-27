# CLI

Tools to use for making the CLI:

- `commander` for CLI interface
- `@commander-js/extra-typings` for TypeScript types
- `consola` for Elegant Console Wrapper

Behavioral guidance:

- Keep progress and status logs on stderr.
- Keep the final machine-readable response on stdout as JSON.
- Global options are passed before the subcommand.
- FHETest v2 is the only contract target.
