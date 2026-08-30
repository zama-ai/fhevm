import { Command, Option } from 'commander';
import { resolve } from 'node:path';

import { defaultWorkspaceRoot } from './base/paths.ts';

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
  'check-tsconfig-paths',
] as const;
export type CommandName = (typeof commandNames)[number];

export type CliOptions = {
  readonly command:
    | CommandName
    | 'check-mirror'
    | 'check-vendored'
    | 'generate-exports'
    | 'install-forge-dependencies'
    | 'list-packages'
    | 'sync-vendored'
    | 'test-consumer'
    | 'test-consumer-regenerate-package-lock';
  readonly workspaceRoot: string;
  readonly manifestFile: string;
  readonly verbose: boolean;
  readonly sortPackageJson: boolean;
} & (
  | { readonly command: CommandName }
  | { readonly command: 'check-mirror'; readonly packageSelector: string }
  | { readonly command: 'check-vendored'; readonly packageSelector?: string }
  | { readonly command: 'generate-exports'; readonly exportManifestFile: string; readonly check: boolean }
  | { readonly command: 'install-forge-dependencies'; readonly packageSelector?: string }
  | { readonly command: 'list-packages' }
  | { readonly command: 'sync-vendored'; readonly check: boolean }
  | { readonly command: 'test-consumer-regenerate-package-lock'; readonly packageSelector?: string }
  | {
      readonly command: 'test-consumer';
      readonly packageSelector?: string;
      readonly output?: string;
      readonly testFile?: string;
      readonly force: boolean;
      readonly buildPackage: boolean;
      readonly run: boolean;
      readonly list: boolean;
      readonly ci: boolean;
    }
);

type RawOptions = {
  readonly root: string;
  readonly verbose: boolean;
};

type RawTestConsumerOptions = {
  readonly output?: string;
  readonly testFile?: string;
  readonly force: boolean;
  readonly buildPackage: boolean;
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
    .option('-v, --verbose', 'print a success summary', false);

  let selected: CommandName | undefined;
  let generateExports: { readonly exportManifestFile: string; readonly check: boolean } | undefined;
  let sortPackageJson = false;
  let vendoredPackageSelector: string | undefined;
  let checkAllVendored = false;
  let mirrorPackageSelector: string | undefined;
  let forgeDependencyPackageSelector: string | undefined;
  let installAllForgeDependencies = false;
  let listPackagesSelected = false;
  let syncVendored: { readonly check: boolean } | undefined;
  let regenerateConsumerPackageLocks = false;
  let regenerateConsumerPackageLockSelector: string | undefined;
  let testConsumer:
    | {
        readonly packageSelector?: string;
        readonly output?: string;
        readonly testFile?: string;
        readonly force: boolean;
        readonly buildPackage: boolean;
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
    .description('Check that local paths exposed by package.json exist.')
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
  build          Required on every dev owner of a published package.
  clean          Required on every dev owner of a published package.
  forge:fmt      Required on every package owning Solidity; published payloads use their dev owner.
  forge:fmt:check Required on every package owning Solidity; published payloads use their dev owner.
  forge:lint     Required on every package owning Solidity; published payloads use their dev owner.
  lint           Required on every dev package, shared helper and internal consumer.
  eslint.config.js Required beside every non-published package that owns a lint script; a "type": "commonjs" package uses eslint.config.mjs instead. Alternate names are forbidden.
  prettier:check Required on every dev package, shared helper and internal consumer; must exclude Solidity.
  prettier:write Required on every dev package, shared helper and internal consumer; must exclude Solidity.
  .prettierrc.mjs Required beside every non-published package that owns a Prettier script; re-exports prettier.base.mjs.
  test:publint   Required on every dev owner of a published package.
  test:consumer  Required on every dev owner of a published package.
  test:vendored  Required when the package declares vendored content.
  test:mirror    Required when the package declares a mirror.
  test           Required in each expected test-consumer/cjs or test-consumer/esm fixture.
`,
    )
    .action(() => {
      selected = 'check-scripts';
    });
  program
    .command('check-lockfiles')
    .description('Check workspace and standalone lockfile placement.')
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
    .command('check-vendored [package]')
    .description(
      'Check vendored sources against their declared sources of truth for one package, or for every ' +
        'package that declares vendored content when omitted.',
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
    .command('generate-exports <manifest>')
    .description("Render a package's export manifest into its index and consumer export tests.")
    .option('--check', 'compare the outputs against the manifest instead of writing them', false)
    .action((manifest: string, options: { readonly check: boolean }) => {
      generateExports = { exportManifestFile: resolve(manifest), check: options.check };
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
    .command('install-forge-dependencies [package]')
    .description('Install Soldeer dependencies for one package, or for all manifest packages when omitted.')
    .action((packageSelector: string | undefined) => {
      forgeDependencyPackageSelector = packageSelector;
      installAllForgeDependencies = packageSelector === undefined;
    });
  program
    .command('list-packages')
    .description('List every manifest package relative path and its kind.')
    .action(() => {
      listPackagesSelected = true;
    });
  program
    .command('test-consumer [package]')
    .description('Install one checked-in consumer fixture for manual inspection and testing.')
    .option('-l, --list', 'list available consumer fixtures', false)
    .option('-o, --output <path>', 'persistent installation directory')
    .option('--test-file <path>', "select one fixture-relative file for the consumer's 'test:file' script")
    .option('--build-package', "run the package owner's 'build' script before installation", false)
    .option('--run', "run the consumer's 'test' script after installation", false)
    .option('--ci', 'install from the committed consumer lockfile with npm ci', false)
    .option('-f, --force', 'replace an existing output directory', false)
    .action((packageSelector: string | undefined, options: RawTestConsumerOptions) => {
      testConsumer = {
        packageSelector,
        output: options.output,
        testFile: options.testFile,
        force: options.force,
        buildPackage: options.buildPackage,
        run: options.run,
        list: options.list,
        ci: options.ci,
      };
    });
  program
    .command('test-consumer-regenerate-package-lock [package]')
    .description('Regenerate and validate consumer package-lock.json files; defaults to every fixture.')
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
    !listPackagesSelected &&
    !regenerateConsumerPackageLocks &&
    testConsumer === undefined &&
    syncVendored === undefined &&
    generateExports === undefined
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
      verbose: options.verbose,
      sortPackageJson: false,
      ...syncVendored,
    };
  }
  if (generateExports !== undefined) {
    return {
      command: 'generate-exports',
      workspaceRoot,
      manifestFile: resolve(workspaceRoot, 'npm-manifest.json'),
      verbose: options.verbose,
      sortPackageJson: false,
      ...generateExports,
    };
  }
  if (regenerateConsumerPackageLocks) {
    return {
      command: 'test-consumer-regenerate-package-lock',
      workspaceRoot,
      manifestFile: resolve(workspaceRoot, 'npm-manifest.json'),
      verbose: options.verbose,
      sortPackageJson: false,
      packageSelector: regenerateConsumerPackageLockSelector,
    };
  }
  if (mirrorPackageSelector !== undefined) {
    return {
      command: 'check-mirror',
      workspaceRoot,
      manifestFile: resolve(workspaceRoot, 'npm-manifest.json'),
      verbose: options.verbose,
      sortPackageJson: false,
      packageSelector: mirrorPackageSelector,
    };
  }
  if (checkAllVendored || vendoredPackageSelector !== undefined) {
    return {
      command: 'check-vendored',
      workspaceRoot,
      manifestFile: resolve(workspaceRoot, 'npm-manifest.json'),
      verbose: options.verbose,
      sortPackageJson: false,
      packageSelector: vendoredPackageSelector,
    };
  }
  if (installAllForgeDependencies || forgeDependencyPackageSelector !== undefined) {
    return {
      command: 'install-forge-dependencies',
      workspaceRoot,
      manifestFile: resolve(workspaceRoot, 'npm-manifest.json'),
      verbose: options.verbose,
      sortPackageJson: false,
      packageSelector: forgeDependencyPackageSelector,
    };
  }
  if (testConsumer !== undefined) {
    return {
      command: 'test-consumer',
      workspaceRoot,
      manifestFile: resolve(workspaceRoot, 'npm-manifest.json'),
      verbose: options.verbose,
      sortPackageJson: false,
      ...testConsumer,
    };
  }
  if (listPackagesSelected) {
    return {
      command: 'list-packages',
      workspaceRoot,
      manifestFile: resolve(workspaceRoot, 'npm-manifest.json'),
      verbose: options.verbose,
      sortPackageJson: false,
    };
  }
  if (selected === undefined) throw new Error('unreachable');
  return {
    command: selected,
    workspaceRoot,
    manifestFile: resolve(workspaceRoot, 'npm-manifest.json'),
    verbose: options.verbose,
    sortPackageJson,
  };
}
