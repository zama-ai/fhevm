#!/usr/bin/env node
import type { CheckCommand } from './base/command.ts';
import { renderCompletionScript } from './base/sh-completion.ts';
import { printReport } from './base/diagnostics.ts';
import { hasDetailedOutput } from './base/verbosity.ts';
import { type CommandName, parseCliOptions } from './cli-options.ts';
import { checkDependencies } from './commands/check-dependencies.ts';
import { checkFoundry } from './commands/check-foundry.ts';
import { checkJsonSchemas } from './commands/check-json-schemas.ts';
import { checkLintPolicy } from './commands/check-lint-policy.ts';
import { checkLockfiles } from './commands/check-lockfiles.ts';
import { checkManifestCoverage } from './commands/check-manifest-coverage.ts';
import { checkMirror } from './commands/check-mirror.ts';
import { checkNames } from './commands/check-names.ts';
import { checkOwnership } from './commands/check-ownership.ts';
import { checkPackageJsonPaths } from './commands/check-package-json-paths.ts';
import { checkPackageJson } from './commands/check-package-json.ts';
import { checkCleartextConfig } from './commands/check-cleartext-config.ts';
import { checkCommitScope } from './commands/check-commit-scope.ts';
import { checkScripts } from './commands/check-scripts.ts';
import { checkTscMode } from './commands/check-tsc-mode.ts';
import { checkTsconfigPaths } from './commands/check-tsconfig-paths.ts';
import { checkFhevmChainsOrigin } from './commands/check-fhevm-chains-origin.ts';
import { checkVendoredOrigin } from './commands/check-vendored-origin.ts';
import { syncFhevmChains } from './commands/sync-fhevm-chains.ts';
import { checkWorkspaces } from './commands/check-workspaces.ts';
import { generateChainConstantsCommand } from './commands/generate-chain-constants.ts';
import { generateCleartextConfigCommand } from './commands/generate-cleartext-config.ts';
import { generateExportsCommand } from './commands/generate-exports.ts';
import { cleanForgeDependencies } from './commands/clean-forge-dependencies.ts';
import { installForgeDependencies } from './commands/install-forge-dependencies.ts';
import { listPackages } from './commands/list-packages.ts';
import { listVersions } from './commands/list-versions.ts';
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
  'check-json-schemas': checkJsonSchemas,
  'check-lint-policy': checkLintPolicy,
  'check-manifest-coverage': checkManifestCoverage,
  'check-tsconfig-paths': checkTsconfigPaths,
  'check-tsc-mode': checkTscMode,
  'check-commit-scope': checkCommitScope,
  'check-cleartext-config': checkCleartextConfig,
};

async function main(): Promise<void> {
  const options = parseCliOptions(process.argv.slice(2));
  // Before the workspace manifest is loaded: this one is driven by a package's own export manifest and
  // needs nothing from npm-manifest.json.
  if (options.command === 'generate-exports') {
    generateExportsCommand({ manifestFile: options.exportManifestFile, check: options.check });
    return;
  }
  // Also manifest-free: it reads sdk/cleartext-config.json and writes the faces generated from it.
  if (options.command === 'generate-cleartext-config') {
    generateCleartextConfigCommand({ workspaceRoot: options.workspaceRoot, check: options.check });
    return;
  }
  if (options.command === 'generate-chain-constants') {
    generateChainConstantsCommand({ workspaceRoot: options.workspaceRoot, check: options.check });
    return;
  }
  // Both are manifest-free: they speak to the protocol registry and the workspace-root chains file.
  if (options.command === 'sync-fhevm-chains') {
    await syncFhevmChains({ workspaceRoot: options.workspaceRoot, commit: options.commit, latest: options.latest });
    return;
  }
  if (options.command === 'check-fhevm-chains-origin') {
    const report = await checkFhevmChainsOrigin({ workspaceRoot: options.workspaceRoot });
    printReport(report, options.verbosity);
    if (report.violations.length > 0) process.exitCode = 1;
    return;
  }
  if (options.command === 'sh-completion') {
    // Package keys are a convenience for selector arguments; completion still renders outside a workspace.
    let packageKeys: readonly string[] = [];
    try {
      packageKeys = Object.keys(loadNpmManifest(options.manifestFile).packages);
    } catch {
      // No manifest reachable: complete commands and flags only.
    }
    process.stdout.write(renderCompletionScript(options.shell, options.commands, packageKeys));
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
  if (options.command === 'list-versions') {
    await listVersions(options.workspaceRoot, manifest, { checkNpmjs: options.checkNpmjs, json: options.json });
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
