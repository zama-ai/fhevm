#!/usr/bin/env node
import type { CheckCommand } from './base/command.ts';
import { printReport } from './base/diagnostics.ts';
import { hasDetailedOutput } from './base/verbosity.ts';
import { type CommandName, parseCliOptions } from './cli-options.ts';
import { checkDependencies } from './commands/check-dependencies.ts';
import { checkFoundry } from './commands/check-foundry.ts';
import { checkLintPolicy } from './commands/check-lint-policy.ts';
import { checkLockfiles } from './commands/check-lockfiles.ts';
import { checkManifestCoverage } from './commands/check-manifest-coverage.ts';
import { checkMirror } from './commands/check-mirror.ts';
import { checkNames } from './commands/check-names.ts';
import { checkOwnership } from './commands/check-ownership.ts';
import { checkPackageJsonPaths } from './commands/check-package-json-paths.ts';
import { checkPackageJson } from './commands/check-package-json.ts';
import { checkCommitScope } from './commands/check-commit-scope.ts';
import { checkScripts } from './commands/check-scripts.ts';
import { checkTscMode } from './commands/check-tsc-mode.ts';
import { checkTsconfigPaths } from './commands/check-tsconfig-paths.ts';
import { checkVendoredOrigin } from './commands/check-vendored-origin.ts';
import { checkWorkspaces } from './commands/check-workspaces.ts';
import { generateExportsCommand } from './commands/generate-exports.ts';
import { cleanForgeDependencies } from './commands/clean-forge-dependencies.ts';
import { installForgeDependencies } from './commands/install-forge-dependencies.ts';
import { listPackages } from './commands/list-packages.ts';
import { packTarballs } from './commands/pack-tarball.ts';
import { syncVendoredCommand } from './commands/sync-vendored.ts';
import { testConsumerRegeneratePackageLock } from './commands/test-consumer-regenerate-package-lock.ts';
import { testConsumer } from './commands/test-consumer.ts';
import { loadNpmManifest } from './manifest.ts';

const commands: Readonly<Record<CommandName, CheckCommand>> = {
  'check-names': checkNames,
  'check-dependencies': checkDependencies,
  'check-package-json': checkPackageJson,
  'check-package-json-paths': checkPackageJsonPaths,
  'check-workspaces': checkWorkspaces,
  'check-ownership': checkOwnership,
  'check-scripts': checkScripts,
  'check-lockfiles': checkLockfiles,
  'check-foundry': checkFoundry,
  'check-lint-policy': checkLintPolicy,
  'check-manifest-coverage': checkManifestCoverage,
  'check-tsconfig-paths': checkTsconfigPaths,
  'check-tsc-mode': checkTscMode,
  'check-commit-scope': checkCommitScope,
};

async function main(): Promise<void> {
  const options = parseCliOptions(process.argv.slice(2));
  // Before the workspace manifest is loaded: this one is driven by a package's own export manifest and
  // needs nothing from npm-manifest.json.
  if (options.command === 'generate-exports') {
    generateExportsCommand({ manifestFile: options.exportManifestFile, check: options.check });
    return;
  }
  const manifest = loadNpmManifest(options.manifestFile);
  if (options.command === 'sync-vendored') {
    const report = syncVendoredCommand({
      workspaceRoot: options.workspaceRoot,
      manifest,
      check: options.check,
      verbose: hasDetailedOutput(options.verbosity),
    });
    printReport(report, options.verbosity);
    if (report.violations.length > 0) process.exitCode = 1;
    return;
  }
  if (options.command === 'list-packages') {
    listPackages(manifest);
    return;
  }
  if (options.command === 'pack-tarball') {
    packTarballs({
      workspaceRoot: options.workspaceRoot,
      manifest,
      packageSelector: options.packageSelector,
      outDir: options.outDir,
      clean: options.clean,
    });
    return;
  }
  if (options.command === 'check-mirror') {
    const report = checkMirror({ workspaceRoot: options.workspaceRoot, manifest }, options.packageSelector);
    printReport(report, options.verbosity);
    if (report.violations.length > 0) process.exitCode = 1;
    return;
  }
  if (options.command === 'check-vendored-origin') {
    const report = checkVendoredOrigin({ workspaceRoot: options.workspaceRoot, manifest }, options.packageSelector);
    printReport(report, options.verbosity);
    if (report.violations.length > 0) process.exitCode = 1;
    return;
  }
  if (options.command === 'test-consumer') {
    testConsumer({
      workspaceRoot: options.workspaceRoot,
      manifest,
      packageSelector: options.packageSelector,
      output: options.output,
      testFile: options.testFile,
      force: options.force,
      buildLinkedDependencies: options.buildLinkedDependencies,
      run: options.run,
      list: options.list,
      ci: options.ci,
      verbosity: options.verbosity,
    });
    return;
  }
  if (options.command === 'test-consumer-regenerate-package-lock') {
    testConsumerRegeneratePackageLock({
      workspaceRoot: options.workspaceRoot,
      manifest,
      packageSelector: options.packageSelector,
      verbosity: options.verbosity,
    });
    return;
  }
  if (options.command === 'install-forge-dependencies') {
    installForgeDependencies({
      workspaceRoot: options.workspaceRoot,
      manifest,
      packageSelector: options.packageSelector,
    });
    return;
  }
  if (options.command === 'clean-forge-dependencies') {
    await cleanForgeDependencies({
      workspaceRoot: options.workspaceRoot,
      manifest,
      packageSelector: options.packageSelector,
      dryRun: options.dryRun,
      force: options.force,
    });
    return;
  }
  const report = commands[options.command]({
    workspaceRoot: options.workspaceRoot,
    manifest,
    sortPackageJson: options.sortPackageJson,
  });
  printReport(report, options.verbosity);
  if (report.violations.length > 0) process.exitCode = 1;
}

try {
  await main();
} catch (error) {
  console.error(error instanceof Error ? error.message : String(error));
  process.exitCode = 2;
}
