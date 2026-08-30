# fhevm-npm

Autonomous Node-native TypeScript CLI for validating [`npm-rules.md`](./npm-rules.md). It owns its dependencies,
TypeScript configuration and tests, and imports no code from the surrounding SDK.

```sh
npm install
node ./fhevm-npm.ts check-names
node ./fhevm-npm.ts check-dependencies
node ./fhevm-npm.ts check-package-json
node ./fhevm-npm.ts check-package-json-paths
node ./fhevm-npm.ts check-workspaces
node ./fhevm-npm.ts check-ownership
node ./fhevm-npm.ts check-scripts
node ./fhevm-npm.ts check-lockfiles
node ./fhevm-npm.ts check-foundry
node ./fhevm-npm.ts check-manifest-coverage
node ./fhevm-npm.ts check-tsconfig-paths
node ./fhevm-npm.ts test-consumer --list
node ./fhevm-npm.ts test-consumer ./host-contracts-cleartext/v12
node ./fhevm-npm.ts test-consumer ./host-contracts-cleartext/v12 --build-package --run
```

The package exposes the same entry point as the `fhevm-npm` bin, so an installed or linked copy uses:

```sh
fhevm-npm [options] <command>
```

Check commands have no command-specific options. `test-consumer` accepts a package selector plus options for listing,
building, running and choosing or replacing its persistent output directory. It performs an isolated
`npm ci --install-links`. Without `--run`, it prints and preserves the resulting directory for manual inspection. With
`--run`, it logs each directory below `${TMPDIR}/fhevm-npm-test-consumer/<owner>/<format>`, executes `npm test` and
removes the marked format directory afterward; an explicit `--output` remains
available for inspection. A package selector runs every required `test-consumer/cjs` and `test-consumer/esm` variant
detected from the published package's entry points. The CLI always reads the manifest from `<root>/npm-manifest.json`; run
`fhevm-npm test-consumer --help` for details.

Exit code `0` means the check passed, `1` means policy violations were found, and `2` means the CLI, manifest or
workspace could not be loaded.
