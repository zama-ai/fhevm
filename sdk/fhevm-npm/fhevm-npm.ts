#!/usr/bin/env node
import type { CheckCommand } from './base/command.ts';
import { printReport } from './base/diagnostics.ts';
import { type CommandName, parseCliOptions } from './cli-options.ts';
import { checkDependencies } from './commands/check-dependencies.ts';
import { checkFoundry } from './commands/check-foundry.ts';
import { checkLockfiles } from './commands/check-lockfiles.ts';
import { checkManifestCoverage } from './commands/check-manifest-coverage.ts';
import { checkMirror } from './commands/check-mirror.ts';
import { checkNames } from './commands/check-names.ts';
import { checkOwnership } from './commands/check-ownership.ts';
import { checkPackageJsonPaths } from './commands/check-package-json-paths.ts';
import { checkPackageJson } from './commands/check-package-json.ts';
import { checkScripts } from './commands/check-scripts.ts';
import { checkTsconfigPaths } from './commands/check-tsconfig-paths.ts';
import { checkVendored } from './commands/check-vendored.ts';
import { checkWorkspaces } from './commands/check-workspaces.ts';
import { generateExportsCommand } from './commands/generate-exports.ts';
import { installForgeDependencies } from './commands/install-forge-dependencies.ts';
import { listPackages } from './commands/list-packages.ts';
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
  'check-manifest-coverage': checkManifestCoverage,
  'check-tsconfig-paths': checkTsconfigPaths,
};

function main(): void {
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
      verbose: options.verbose,
    });
    printReport(report, options.verbose);
    if (report.violations.length > 0) process.exitCode = 1;
    return;
  }
  if (options.command === 'list-packages') {
    listPackages(manifest);
    return;
  }
  if (options.command === 'check-mirror') {
    const report = checkMirror({ workspaceRoot: options.workspaceRoot, manifest }, options.packageSelector);
    printReport(report, options.verbose);
    if (report.violations.length > 0) process.exitCode = 1;
    return;
  }
  if (options.command === 'check-vendored') {
    const report = checkVendored({ workspaceRoot: options.workspaceRoot, manifest }, options.packageSelector);
    printReport(report, options.verbose);
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
      buildPackage: options.buildPackage,
      run: options.run,
      list: options.list,
      ci: options.ci,
    });
    return;
  }
  if (options.command === 'test-consumer-regenerate-package-lock') {
    testConsumerRegeneratePackageLock({
      workspaceRoot: options.workspaceRoot,
      manifest,
      packageSelector: options.packageSelector,
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
  const report = commands[options.command]({
    workspaceRoot: options.workspaceRoot,
    manifest,
    sortPackageJson: options.sortPackageJson,
  });
  printReport(report, options.verbose);
  if (report.violations.length > 0) process.exitCode = 1;
}

try {
  main();
} catch (error) {
  console.error(error instanceof Error ? error.message : String(error));
  process.exitCode = 2;
}
