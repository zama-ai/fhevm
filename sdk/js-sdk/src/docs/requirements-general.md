# General requirements

## TypeScript

- Never use `enum`. Use a union of string literals or an `as const` object.
- No generators (`function*`, `yield`) unless strictly needed.
- Code must always be bundler-friendly: tree-shakeable, with no top-level side effects.
