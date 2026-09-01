# scripts:

The lifecycle has three compile-side phases, each with its own script family:

```sh
npm run generate:pre    # source-level generation: exports, cleartext-config, contract-versions,
                        # compute-addresses, placeholders, signers (writes tracked files)
npm run compile:forge   # compile the Solidity; needs generate:pre output (placeholders) and
                        # produces ./out, which generate:post reads
npm run generate:post   # artifact-dependent generation: templates, then local-host-bytecode
npm run generate        # all of the above in order

npm run compile         # the publishable payload: the four tsc emits into pkg/ts/_esm, _cjs,
                        # _types, _types-cjs; compiles committed sources only, generates nothing
npm run build           # the everyday sweep: fmt:check, then lint, then compile
```

`compile` deliberately does not run forge: `./out` is an input to generation, the payload gates
(`check:contract-sizes`) and the template tests, not part of the published payload. It exists in any
worktree where `generate` has run — and `test` self-provides it by running `test:forge` first.

Everyday dev-loop commands (fmt, lint, test, check, generate, and the make targets) are listed in
`sdk/fhevm-npm-docs/GUIDE.md`.
