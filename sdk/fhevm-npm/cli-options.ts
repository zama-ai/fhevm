import { Command, Option } from 'commander';
import { resolve } from 'node:path';

import type { CompletionCommand, CompletionShell } from './base/sh-completion.ts';
import { defaultWorkspaceRoot } from './base/paths.ts';
import { type Verbosity, increaseVerbosity } from './base/verbosity.ts';

export const commandNames = [
  'check-names',
  'check-dependencies',
  'check-package-json',
  'check-package-json-paths',
  'check-workspaces',
  'check-ownership',
  'check-scripts',
  'check-lockfiles',
  'check-manifest-coverage',
  'check-foundry',
  'check-lint-policy',
  'check-tsconfig-paths',
  'check-tsc-mode',
  'check-commit-scope',
  'check-cleartext-config',
] as const;
export type CommandName = (typeof commandNames)[number];

export type CliOptions = {
  readonly command:
    | CommandName
    | 'check-fhevm-chains-origin'
    | 'check-mirror'
    | 'check-vendored-origin'
    | 'clean-forge-dependencies'
    | 'sh-completion'
    | 'generate-cleartext-config'
    | 'generate-exports'
    | 'install-forge-dependencies'
    | 'list-packages'
    | 'pack-tarball'
    | 'sync-fhevm-chains'
    | 'sync-vendored'
    | 'test-consumer'
    | 'test-consumer-regenerate-package-lock';
  readonly workspaceRoot: string;
  readonly manifestFile: string;
  readonly verbosity: Verbosity;
  readonly sortPackageJson: boolean;
} & (
  | { readonly command: CommandName }
  | { readonly command: 'check-mirror'; readonly packageSelector: string }
  | { readonly command: 'check-vendored-origin'; readonly packageSelector?: string }
  | {
      readonly command: 'sh-completion';
      readonly shell: CompletionShell;
      readonly commands: readonly CompletionCommand[];
    }
  | { readonly command: 'generate-cleartext-config'; readonly check: boolean }
  | { readonly command: 'generate-exports'; readonly exportManifestFile: string; readonly check: boolean }
  | { readonly command: 'install-forge-dependencies'; readonly packageSelector?: string }
  | {
      readonly command: 'clean-forge-dependencies';
      readonly packageSelector?: string;
      readonly dryRun: boolean;
      readonly force: boolean;
    }
  | { readonly command: 'list-packages' }
  | {
      readonly command: 'pack-tarball';
      readonly packageSelector?: string;
      readonly outDir?: string;
      readonly clean: boolean;
    }
  | { readonly command: 'check-fhevm-chains-origin' }
  | { readonly command: 'sync-fhevm-chains'; readonly commit?: string; readonly latest: boolean }
  | { readonly command: 'sync-vendored'; readonly check: boolean }
  | { readonly command: 'test-consumer-regenerate-package-lock'; readonly packageSelector?: string }
  | {
      readonly command: 'test-consumer';
      readonly packageSelector?: string;
      readonly output?: string;
      readonly testFile?: string;
      readonly force: boolean;
      readonly buildLinkedDependencies: boolean;
      readonly run: boolean;
      readonly list: boolean;
      readonly ci: boolean;
    }
);

type RawOptions = {
  readonly root: string;
  readonly verbose: Verbosity;
};

type RawTestConsumerOptions = {
  readonly output?: string;
  readonly testFile?: string;
  readonly force: boolean;
  readonly buildLinkedDependencies: boolean;
  readonly run: boolean;
  readonly list: boolean;
  readonly ci: boolean;
};

export function parseCliOptions(argv: readonly string[]): CliOptions {
  const program = new Command()
    .name('fhevm-npm')
    .description('Validate the npm workspace policy from npm-manifest.json.')
    .showHelpAfterError()
    .showSuggestionAfterError()
    .addOption(new Option('-r, --root <path>', 'sdk workspace root').default(defaultWorkspaceRoot))
    .option(
      '-v, --verbose',
      'increase verbosity; repeat up to -vvvv (-vv preserves the previous verbose behavior)',
      increaseVerbosity,
      0,
    );

  let selected: CommandName | undefined;
  let generateExports: { readonly exportManifestFile: string; readonly check: boolean } | undefined;
  let generateCleartextConfig: { readonly check: boolean } | undefined;
  let completion: { readonly shell: CompletionShell; readonly commands: readonly CompletionCommand[] } | undefined;
  let sortPackageJson = false;
  let vendoredPackageSelector: string | undefined;
  let checkAllVendored = false;
  let mirrorPackageSelector: string | undefined;
  let forgeDependencyPackageSelector: string | undefined;
  let installAllForgeDependencies = false;
  let cleanForgeDependencies:
    { readonly packageSelector?: string; readonly dryRun: boolean; readonly force: boolean } | undefined;
  let listPackagesSelected = false;
  let packTarball: { readonly packageSelector?: string; readonly outDir?: string; readonly clean: boolean } | undefined;
  let syncVendored: { readonly check: boolean } | undefined;
  let syncFhevmChains: { readonly commit?: string; readonly latest: boolean } | undefined;
  let checkFhevmChainsOrigin = false;
  let regenerateConsumerPackageLocks = false;
  let regenerateConsumerPackageLockSelector: string | undefined;
  let testConsumer:
    | {
        readonly packageSelector?: string;
        readonly output?: string;
        readonly testFile?: string;
        readonly force: boolean;
        readonly buildLinkedDependencies: boolean;
        readonly run: boolean;
        readonly list: boolean;
        readonly ci: boolean;
      }
    | undefined;
  program
    .command('check-names')
    .description('Check package names, privacy, and the required -dev suffix.')
    .action(() => {
      selected = 'check-names';
    });
  program
    .command('check-dependencies')
    .description('Check source and npm-script dependencies, workspace specs, and dependency-version rules.')
    .action(() => {
      selected = 'check-dependencies';
    });
  program
    .command('check-package-json')
    .description('Check package.json hygiene.')
    .option(
      '--sort',
      "sort top-level entries and each package.json 'scripts' field; 'workspaces' order is left alone",
      false,
    )
    .action((options: { readonly sort: boolean }) => {
      selected = 'check-package-json';
      sortPackageJson = options.sort;
    });
  program
    .command('check-package-json-paths')
    .description('Check that local paths exposed by package.json exist; packages must be built first.')
    .addHelpText(
      'after',
      `
Prerequisite:
  Build the project first because package.json entry points may reference generated files.
`,
    )
    .action(() => {
      selected = 'check-package-json-paths';
    });
  program
    .command('check-workspaces')
    .description('Check workspace membership and published-name uniqueness.')
    .action(() => {
      selected = 'check-workspaces';
    });
  program
    .command('check-ownership')
    .description('Check dev-owner and published-payload relationships.')
    .action(() => {
      selected = 'check-ownership';
    });
  program
    .command('check-scripts')
    .description('Check conventional package-owned validation scripts.')
    .addHelpText(
      'after',
      `
Checked scripts:
  compile        Required on every dev owner of a published package.
  build          Optional everyday sweep; when present it must reach fmt:check, lint and compile.
  clean          Required on every dev owner of a published package.
  forge:fmt      Required on every package owning Solidity except mirror-only payloads; published payloads use their dev owner.
  forge:fmt:check Required on every package owning Solidity except mirror-only payloads; published payloads use their dev owner.
  forge:lint     Required on every package owning Solidity except mirror-only payloads; published payloads use their dev owner.
  lint           Required on every dev package, shared helper and internal consumer.
  pack:tarball   Required on every dev owner of an npm-distributed package.
  eslint.config.js The only package-level ESLint config filename; required beside every non-published package that owns a lint script.
  prettier:check Required on every dev package, shared helper and internal consumer; must exclude Solidity.
  prettier:write Required on every dev package, shared helper and internal consumer; must exclude Solidity.
  prettier.config.js The only package-level Prettier config filename; references the root prettier.base.mjs.
  check:publint  Required on every dev owner of an npm-distributed package.
  test:consumer  Required on every dev owner of an npm-distributed package; mirror-only consumer projects are exempt.
  fmt            Required on every dev package, shared helper and internal consumer.
  fmt:check      Required wherever fmt is.
  check          Required on every dev owner of an npm-distributed package.
  check:vendored-origin Required when the package declares vendored content.
  check:mirror   Optional until the mirror spec lands.
  test           Required in each expected test-consumer/cjs or test-consumer/esm fixture.
`,
    )
    .action(() => {
      selected = 'check-scripts';
    });
  program
    .command('check-lockfiles')
    .description('Check workspace and isolated-consumer lockfile placement.')
    .action(() => {
      selected = 'check-lockfiles';
    });
  program
    .command('check-foundry')
    .description('Check the installed forge version against the central manifest pin.')
    .action(() => {
      selected = 'check-foundry';
    });
  program
    .command('check-lint-policy')
    .description('Check that Forge is the only Solidity linter outside declared mirror-only packages.')
    .action(() => {
      selected = 'check-lint-policy';
    });
  program
    .command('check-manifest-coverage')
    .description('Check filesystem discovery, manifest completeness, and path containment.')
    .action(() => {
      selected = 'check-manifest-coverage';
    });
  program
    .command('check-mirror <package>')
    .description("Compare one package's tracked mirror files with a fresh upstream clone.")
    .action((packageSelector: string) => {
      mirrorPackageSelector = packageSelector;
    });
  program
    .command('check-vendored-origin [package]')
    .description(
      'Check that each local vendored folder matches its declared origin (git commit), for one ' +
        'package or for every package that declares vendored content when omitted.',
    )
    .action((packageSelector: string | undefined) => {
      vendoredPackageSelector = packageSelector;
      checkAllVendored = packageSelector === undefined;
    });
  program
    .command('check-tsconfig-paths')
    .description('Check that literal paths named by owned tsconfigs exist.')
    .action(() => {
      selected = 'check-tsconfig-paths';
    });
  program
    .command('check-commit-scope')
    .description(
      'Check that every pending git change (staged, unstaged, untracked) is inside the sdk workspace — ' +
        'nothing outside it may be touched by a commit from here.',
    )
    .action(() => {
      selected = 'check-commit-scope';
    });
  program
    .command('check-cleartext-config')
    .description(
      'Check that every generated face of sdk/cleartext-config.json matches it (read-only twin of ' +
        'generate-cleartext-config --check).',
    )
    .action(() => {
      selected = 'check-cleartext-config';
    });
  program
    .command('check-tsc-mode')
    .description("Check that no 'tsc -p' or bare 'tsc' script invocation targets a solution-style tsconfig.")
    .addHelpText(
      'after',
      `
Why:
  A solution-style tsconfig (empty 'files' plus 'references') only orchestrates other projects. Project
  mode loads it, checks zero files, and exits 0, so the script passes without type-checking anything.
  Build mode ('tsc -b') is the only driver that walks the references.
`,
    )
    .action(() => {
      selected = 'check-tsc-mode';
    });
  program
    .command('generate-exports <manifest>')
    .description("Render a package's export manifest into its index and consumer export tests.")
    .option('--check', 'compare the outputs against the manifest instead of writing them', false)
    .action((manifest: string, options: { readonly check: boolean }) => {
      generateExports = { exportManifestFile: resolve(manifest), check: options.check };
    });
  program
    .command('sh-completion <shell>')
    .description('Print a tab-completion script for zsh or bash, rendered from the live command list.')
    .action((shell: string) => {
      if (shell !== 'zsh' && shell !== 'bash') {
        throw new Error(`sh-completion: unsupported shell '${shell}' — expected zsh or bash`);
      }
      // Introspected from the registry itself, so a new command or option is completed without any
      // hand-kept list. Flags are the bare tokens; the first positional's name selects value completion.
      completion = {
        shell,
        commands: program.commands.map((cmd) => ({
          name: cmd.name(),
          description: cmd.description(),
          flags: cmd.options.flatMap((option) => option.flags.split(/[,\s]+/).filter((t) => t.startsWith('-'))),
          argument: cmd.registeredArguments[0]?.name(),
        })),
      };
    });
  program
    .command('generate-cleartext-config')
    .description(
      'Render sdk/cleartext-config.json into every file generated from it: the TypeScript face in ' +
        "common-vendored/src (copied to each generation's pkg/ts by sync-vendored), and each " +
        "generation's FhevmCleartextConfig.sol and scripts/cleartext-config.sh.",
    )
    .option('--check', 'compare the outputs against the JSON instead of writing them', false)
    .action((options: { readonly check: boolean }) => {
      generateCleartextConfig = { check: options.check };
    });
  program
    .command('sync-vendored')
    .description(
      'Write every vendored destination from its source of truth: the shared TypeScript from ' +
        'common-vendored/manifest.json, and the pinned Solidity plus its provenance from npm-manifest.json.',
    )
    .option('--check', 'compare instead of writing, and fail on any difference', false)
    .action((options: { readonly check: boolean }) => {
      syncVendored = { check: options.check };
    });
  program
    .command('sync-fhevm-chains')
    .description(
      'Write fhevm-chains.config.json — every fhevm host-contract and gateway address on mainnet and ' +
        "testnet — from the protocol registry at a pinned commit (default: the file's recorded pin).",
    )
    .option('--latest', "pin to the registry's current HEAD", false)
    .option('--commit <sha>', 'pin to an explicit registry commit (full 40-hex sha)')
    .action((options: { readonly latest: boolean; readonly commit?: string }) => {
      syncFhevmChains = { commit: options.commit, latest: options.latest };
    });
  program
    .command('check-fhevm-chains-origin')
    .description(
      "Check that fhevm-chains.config.json is current with the head of the protocol registry's main " +
        '(read-only; registry commits touching no fhevm address stay green).',
    )
    .action(() => {
      checkFhevmChainsOrigin = true;
    });
  program
    .command('install-forge-dependencies [package]')
    .description('Install Soldeer dependencies for one package, or for all manifest packages when omitted.')
    .action((packageSelector: string | undefined) => {
      forgeDependencyPackageSelector = packageSelector;
      installAllForgeDependencies = packageSelector === undefined;
    });
  program
    .command('clean-forge-dependencies [package]')
    .description(
      'Delete the Forge dependency directories `forge config --json` reports (libs minus node_modules), ' +
        'after showing the list and asking for confirmation.',
    )
    .option('--dry-run', 'list what would go, delete nothing', false)
    .option('-f, --force', 'skip the confirmation prompt; required when stdin is not a terminal', false)
    .action((packageSelector: string | undefined, options: { readonly dryRun: boolean; readonly force: boolean }) => {
      cleanForgeDependencies = { packageSelector, dryRun: options.dryRun, force: options.force };
    });
  program
    .command('list-packages')
    .description('List every manifest package relative path and its kind.')
    .action(() => {
      listPackagesSelected = true;
    });
  program
    .command('pack-tarball [package]')
    .description(
      'Pack one npm-distributed payload (or all of them when omitted) into the manifest-declared ' +
        "tarballs directory. The payload comes from the dev owner's publishedRelPath.",
    )
    .option('-o, --out-dir <dir>', 'override npm-manifest.json#tarballs.relPath')
    .option('--clean', 'delete existing *.tgz in the output directory first', false)
    .action((packageSelector: string | undefined, options: { readonly outDir?: string; readonly clean: boolean }) => {
      packTarball = { packageSelector, outDir: options.outDir, clean: options.clean };
    });
  program
    .command('test-consumer [package]')
    .description('Install one checked-in consumer fixture or manifest-listed consumer project.')
    .option('-l, --list', 'list available consumer fixtures and projects', false)
    .option('-o, --output <path>', 'persistent installation directory')
    .option('--test-file <path>', "select one fixture-relative file for the consumer's 'test:file' script")
    .option(
      '--build-linked-dependencies',
      'ask the SDK Makefile to build dev owners of direct and recursively discovered local candidates',
      false,
    )
    .option('--run', "run the consumer's 'test' script after installation", false)
    .option('--ci', 'install from the committed consumer lockfile with npm ci', false)
    .option('-f, --force', 'replace an existing output directory', false)
    .action((packageSelector: string | undefined, options: RawTestConsumerOptions) => {
      testConsumer = {
        packageSelector,
        output: options.output,
        testFile: options.testFile,
        force: options.force,
        buildLinkedDependencies: options.buildLinkedDependencies,
        run: options.run,
        list: options.list,
        ci: options.ci,
      };
    });
  program
    .command('test-consumer-regenerate-package-lock [package]')
    .description('Regenerate and validate consumer package-lock.json files; defaults to every conventional fixture.')
    .action((packageSelector: string | undefined) => {
      regenerateConsumerPackageLocks = true;
      regenerateConsumerPackageLockSelector = packageSelector;
    });

  program.parse([...argv], { from: 'user' });
  if (
    selected === undefined &&
    mirrorPackageSelector === undefined &&
    !checkAllVendored &&
    vendoredPackageSelector === undefined &&
    !installAllForgeDependencies &&
    forgeDependencyPackageSelector === undefined &&
    cleanForgeDependencies === undefined &&
    !listPackagesSelected &&
    packTarball === undefined &&
    !regenerateConsumerPackageLocks &&
    testConsumer === undefined &&
    syncVendored === undefined &&
    generateExports === undefined &&
    generateCleartextConfig === undefined &&
    completion === undefined &&
    syncFhevmChains === undefined &&
    !checkFhevmChainsOrigin
  ) {
    program.help({ error: true });
    throw new Error('unreachable');
  }
  const options = program.opts<RawOptions>();
  const workspaceRoot = resolve(options.root);
  if (syncVendored !== undefined) {
    return {
      command: 'sync-vendored',
      workspaceRoot,
      manifestFile: resolve(workspaceRoot, 'npm-manifest.json'),
      verbosity: options.verbose,
      sortPackageJson: false,
      ...syncVendored,
    };
  }
  if (syncFhevmChains !== undefined) {
    return {
      command: 'sync-fhevm-chains',
      workspaceRoot,
      manifestFile: resolve(workspaceRoot, 'npm-manifest.json'),
      verbosity: options.verbose,
      sortPackageJson: false,
      ...syncFhevmChains,
    };
  }
  if (checkFhevmChainsOrigin) {
    return {
      command: 'check-fhevm-chains-origin',
      workspaceRoot,
      manifestFile: resolve(workspaceRoot, 'npm-manifest.json'),
      verbosity: options.verbose,
      sortPackageJson: false,
    };
  }
  if (completion !== undefined) {
    return {
      command: 'sh-completion',
      workspaceRoot,
      manifestFile: resolve(workspaceRoot, 'npm-manifest.json'),
      verbosity: options.verbose,
      sortPackageJson: false,
      ...completion,
    };
  }
  if (generateCleartextConfig !== undefined) {
    return {
      command: 'generate-cleartext-config',
      workspaceRoot,
      manifestFile: resolve(workspaceRoot, 'npm-manifest.json'),
      verbosity: options.verbose,
      sortPackageJson: false,
      ...generateCleartextConfig,
    };
  }
  if (generateExports !== undefined) {
    return {
      command: 'generate-exports',
      workspaceRoot,
      manifestFile: resolve(workspaceRoot, 'npm-manifest.json'),
      verbosity: options.verbose,
      sortPackageJson: false,
      ...generateExports,
    };
  }
  if (regenerateConsumerPackageLocks) {
    return {
      command: 'test-consumer-regenerate-package-lock',
      workspaceRoot,
      manifestFile: resolve(workspaceRoot, 'npm-manifest.json'),
      verbosity: options.verbose,
      sortPackageJson: false,
      packageSelector: regenerateConsumerPackageLockSelector,
    };
  }
  if (mirrorPackageSelector !== undefined) {
    return {
      command: 'check-mirror',
      workspaceRoot,
      manifestFile: resolve(workspaceRoot, 'npm-manifest.json'),
      verbosity: options.verbose,
      sortPackageJson: false,
      packageSelector: mirrorPackageSelector,
    };
  }
  if (checkAllVendored || vendoredPackageSelector !== undefined) {
    return {
      command: 'check-vendored-origin',
      workspaceRoot,
      manifestFile: resolve(workspaceRoot, 'npm-manifest.json'),
      verbosity: options.verbose,
      sortPackageJson: false,
      packageSelector: vendoredPackageSelector,
    };
  }
  if (cleanForgeDependencies !== undefined) {
    return {
      command: 'clean-forge-dependencies',
      workspaceRoot,
      manifestFile: resolve(workspaceRoot, 'npm-manifest.json'),
      verbosity: options.verbose,
      sortPackageJson: false,
      ...cleanForgeDependencies,
    };
  }
  if (installAllForgeDependencies || forgeDependencyPackageSelector !== undefined) {
    return {
      command: 'install-forge-dependencies',
      workspaceRoot,
      manifestFile: resolve(workspaceRoot, 'npm-manifest.json'),
      verbosity: options.verbose,
      sortPackageJson: false,
      packageSelector: forgeDependencyPackageSelector,
    };
  }
  if (testConsumer !== undefined) {
    return {
      command: 'test-consumer',
      workspaceRoot,
      manifestFile: resolve(workspaceRoot, 'npm-manifest.json'),
      verbosity: options.verbose,
      sortPackageJson: false,
      ...testConsumer,
    };
  }
  if (listPackagesSelected) {
    return {
      command: 'list-packages',
      workspaceRoot,
      manifestFile: resolve(workspaceRoot, 'npm-manifest.json'),
      verbosity: options.verbose,
      sortPackageJson: false,
    };
  }
  if (packTarball !== undefined) {
    return {
      command: 'pack-tarball',
      workspaceRoot,
      manifestFile: resolve(workspaceRoot, 'npm-manifest.json'),
      verbosity: options.verbose,
      sortPackageJson: false,
      ...packTarball,
    };
  }
  if (selected === undefined) throw new Error('unreachable');
  return {
    command: selected,
    workspaceRoot,
    manifestFile: resolve(workspaceRoot, 'npm-manifest.json'),
    verbosity: options.verbose,
    sortPackageJson,
  };
}
