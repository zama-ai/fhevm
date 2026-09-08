# fhevm-npm

Autonomous Node-native TypeScript CLI for validating [`FHEVM_NPM_RULES.md`](../fhevm-npm-docs/FHEVM_NPM_RULES.md). It owns its dependencies,
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
node ./fhevm-npm.ts check-json-schemas
node ./fhevm-npm.ts check-manifest-coverage
node ./fhevm-npm.ts check-published-files
node ./fhevm-npm.ts check-tsconfig-paths
node ./fhevm-npm.ts check-tsc-mode
node ./fhevm-npm.ts check-commit-scope
node ./fhevm-npm.ts check-cleartext-config
node ./fhevm-npm.ts check-generations
node ./fhevm-npm.ts sync-fhevm-chains --latest
node ./fhevm-npm.ts check-fhevm-chains-origin
node ./fhevm-npm.ts generate-chain-constants
node ./fhevm-npm.ts generate-chain-constants --check
node ./fhevm-npm.ts version check
node ./fhevm-npm.ts version apply --dry-run
node ./fhevm-npm.ts version apply --check-npmjs
node ./fhevm-npm.ts version list
node ./fhevm-npm.ts version list --check-npmjs
node ./fhevm-npm.ts version list --check-npmjs --json
node ./fhevm-npm.ts publish order
node ./fhevm-npm.ts publish render ./hardhat/v3/plugin/pkg
node ./fhevm-npm.ts publish render ./hardhat/v3/plugin/pkg --json
node ./fhevm-npm.ts publish pack ./hardhat/v3/plugin/pkg
node ./fhevm-npm.ts publish check ./hardhat/v3/plugin/pkg
node ./fhevm-npm.ts publish check ./hardhat/v3/plugin/pkg --check-npmjs
node ./fhevm-npm.ts test-consumer --list
node ./fhevm-npm.ts test-consumer ./host-contracts-cleartext/v12
node ./fhevm-npm.ts test-consumer ./host-contracts-cleartext/v12 --build-linked-dependencies --run
node ./fhevm-npm.ts test-consumer ./hardhat/v2/fhevm-hardhat-template/pkg --build-linked-dependencies --run --ci
node ./fhevm-npm.ts test-consumer --all --build-linked-dependencies --run --ci
```

The package exposes the same entry point as the `fhevm-npm` bin, so an installed or linked copy uses:

```sh
fhevm-npm [options] <command>
```

Tab completion is rendered from the live command registry, so it never drifts from the CLI:

```sh
# zsh — one of (regenerate after the CLI gains a command):
fhevm-npm sh-completion zsh > ~/.zfunc/_fhevm-npm-cli     # with ~/.zfunc in $fpath, before compinit
source <(fhevm-npm sh-completion zsh)                     # in ~/.zshrc, after compinit

# bash:
source <(fhevm-npm sh-completion bash)                    # in ~/.bashrc
```

Verbosity is global and cumulative:

- no `-v`: print violations and command-specific result banners; successful npm subprocesses are silent;
- `-v`: add concise progress and success summaries while npm subprocess output remains silent;
- `-vv`: print detailed successes, timings, and normal npm subprocess output (the former `-v` behavior);
- `-vvv`: additionally run npm with `--loglevel verbose`;
- `-vvvv`: run npm with `--loglevel silly`.

Captured npm output is always printed when a silent subprocess fails.

Check commands have no command-specific options. `test-consumer` accepts a package selector plus options for listing,
building linked dependencies, running and choosing or replacing its persistent output directory. Each consumer is
installed in isolation the way its own state dictates: `npm ci --install-links` when it carries a committed lockfile,
`npm install --install-links` when it is a workspace member (its installation root locks it, so it has none by policy)
or when an isolated consumer lacks one, which prints a warning. `--ci` turns that warning into an error and changes
nothing else. Without `--run`, it prints and preserves the
resulting directory for manual inspection. With
`--run`, it logs each directory below `${TMPDIR}/fhevm-npm-test-consumer/<consumer>/<format>`, executes `npm test` and
removes the marked format directory afterward; an explicit `--output` remains
available for inspection. The consumers are exactly those registered in `npm-manifest.json#consumerTests`: nothing is
discovered from directory names, scripts or dependencies. A selector names a consumer (key or package name), a published
payload, or that payload's dev owner; a payload or owner selector runs every suite it registers, and a consumer several
payloads register runs once; `--all` runs every registered consumer serially in source order. The CLI
recursively substitutes manifest-listed published runtime dependencies when their exact name and version match a local
candidate. The committed consumer manifest therefore only needs its genuine direct `file:` candidate. The CLI always
delegates linked package builds to the SDK Makefile, which owns prerequisite ordering and incremental build stamps. It
reads the manifest from `<root>/npm-manifest.json`; run
`fhevm-npm test-consumer --help` for details.

Exit code `0` means the check passed, `1` means policy violations were found, and `2` means the CLI, manifest or
workspace could not be loaded.
