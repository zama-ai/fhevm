#!/usr/bin/env node
import { spawnSync } from 'node:child_process';
import { existsSync, readdirSync, rmSync } from 'node:fs';
import { dirname, isAbsolute, resolve } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

import { KMS_MANIFEST, TFHE_MANIFEST } from '../../versionsManifest.js';

const scriptDir = dirname(fileURLToPath(import.meta.url));
const sdkRoot = resolve(scriptDir, '../..');

// tfhe-rs 1.7.0 split the client-only wasm build into its own npm package
// (`tfhe-client`); versions below that ship the wasm under `tfhe`.
function tfhePackageNameForVersion(version) {
  const [major, minor] = version.split('-', 1)[0].split('.').map(Number);
  return major > 1 || (major === 1 && minor >= 7) ? 'tfhe-client' : 'tfhe';
}

const INSTALLERS = Object.freeze({
  tfhe: Object.freeze({
    displayName: 'TFHE',
    manifest: TFHE_MANIFEST,
    packageName: tfhePackageNameForVersion,
    script: resolve(scriptDir, 'tfhe/install-tfhe.sh'),
    destinationRoot: resolve(sdkRoot, 'src/wasm/tfhe'),
  }),
  tkms: Object.freeze({
    displayName: 'TKMS',
    manifest: KMS_MANIFEST,
    packageName: 'tkms',
    script: resolve(scriptDir, 'kms/install-tkms.sh'),
    destinationRoot: resolve(sdkRoot, 'src/wasm/tkms'),
  }),
});

const usage = [
  'Usage:',
  '  node scripts/wasm/install-from-manifest.mjs [options]',
  '',
  'Installs missing WASM package versions listed in versionsManifest.js, and',
  'removes installed version directories that are no longer listed there.',
  'Manifest entries may set source to any npm install spec, including file: URLs.',
  '',
  'Options:',
  '  --lib <tfhe|tkms|kms|all>  Library to install/prune. Defaults to all.',
  '  --force, -y               Reinstall versions even when destination directories exist.',
  '  --no-prune                Do not remove version directories missing from versionsManifest.js.',
  '  --no-compress             Forward to TKMS wasm base64 generation.',
  '  --no-codegen              Do not regenerate source WASM loaders/API declarations after install.',
  '  --dry-run                 Print installer/removal commands without running them.',
  '  --help, -h                Show this help.',
].join('\n');

function fail(message) {
  throw new Error(message);
}

function parseArgs(argv) {
  const args = {
    dryRun: false,
    force: false,
    lib: 'all',
    noCodegen: false,
    noCompress: false,
    noPrune: false,
  };

  for (let i = 0; i < argv.length; i++) {
    const arg = argv[i];

    if (arg === '--help' || arg === '-h') {
      console.log(usage);
      process.exit(0);
    }

    if (arg === '--dry-run') {
      args.dryRun = true;
      continue;
    }

    if (arg === '--force' || arg === '-y') {
      args.force = true;
      continue;
    }

    if (arg === '--no-compress') {
      args.noCompress = true;
      continue;
    }

    if (arg === '--no-codegen') {
      args.noCodegen = true;
      continue;
    }

    if (arg === '--no-prune') {
      args.noPrune = true;
      continue;
    }

    if (arg === '--lib') {
      args.lib = argv[++i];
      if (!args.lib) fail(`Missing value for ${arg}.\n\n${usage}`);
      continue;
    }

    fail(`Unknown argument: ${arg}\n\n${usage}`);
  }

  if (args.lib === 'kms') {
    args.lib = 'tkms';
  }

  if (!['tfhe', 'tkms', 'all'].includes(args.lib)) {
    fail(`Unknown lib '${args.lib}'. Expected one of: tfhe, tkms, kms, all`);
  }

  return args;
}

function manifestEntries(manifest) {
  const seen = new Set();
  const unique = [];

  for (const entry of manifest) {
    if (seen.has(entry.version)) {
      continue;
    }

    seen.add(entry.version);
    unique.push(entry);
  }

  return unique;
}

function installedVersionDirNames(installer) {
  if (!existsSync(installer.destinationRoot)) {
    return [];
  }

  return readdirSync(installer.destinationRoot, { withFileTypes: true })
    .filter((entry) => entry.isDirectory() && /^v[^/]+$/.test(entry.name))
    .map((entry) => entry.name);
}

function staleVersionDirNames(installer) {
  const manifestDirNames = new Set(manifestEntries(installer.manifest).map((entry) => `v${entry.version}`));
  return installedVersionDirNames(installer).filter((name) => !manifestDirNames.has(name));
}

function selectedLibs(args) {
  return args.lib === 'all' ? ['tfhe', 'tkms'] : [args.lib];
}

function pruneStaleVersions(args) {
  if (args.noPrune) {
    return;
  }

  for (const lib of selectedLibs(args)) {
    const installer = INSTALLERS[lib];

    for (const name of staleVersionDirNames(installer)) {
      const destination = resolve(installer.destinationRoot, name);
      const renderedCommand = commandLine('rm', ['-rf', destination]);

      if (args.dryRun) {
        console.log(`[wasm-install] ${renderedCommand}`);
        continue;
      }

      console.log(
        `[wasm-install] ${installer.displayName} ${name}: not listed in versionsManifest.js, removing (${destination})`,
      );
      rmSync(destination, { recursive: true, force: true });
    }
  }
}

function quoteShellArg(value) {
  return /^[A-Za-z0-9_./:=@+-]+$/.test(value) ? value : `'${value.replaceAll("'", "'\\''")}'`;
}

function commandLine(command, args) {
  return [command, ...args].map(quoteShellArg).join(' ');
}

function normalizeSource(source) {
  if (!source.startsWith('file:')) {
    return source;
  }

  if (source.startsWith('file://')) {
    return pathToFileURL(fileURLToPath(source)).href;
  }

  const path = source.slice('file:'.length);

  if (path.length === 0) {
    fail('file: sources must include a path.');
  }

  return pathToFileURL(isAbsolute(path) ? path : resolve(sdkRoot, path)).href;
}

function packageNameForEntry(installer, entry) {
  return typeof installer.packageName === 'function' ? installer.packageName(entry.version) : installer.packageName;
}

function sourceForEntry(installer, entry) {
  if (entry.source !== undefined && typeof entry.source !== 'string') {
    fail(`${installer.displayName} v${entry.version} source must be a string when provided.`);
  }

  return normalizeSource(entry.source ?? `${packageNameForEntry(installer, entry)}@${entry.version}`);
}

function defaultSourceForEntry(installer, entry) {
  return `${packageNameForEntry(installer, entry)}@${entry.version}`;
}

function installerArgs(lib, installer, entry, args) {
  const next = [entry.version];
  const source = sourceForEntry(installer, entry);

  if (source !== defaultSourceForEntry(installer, entry)) {
    next.push('--source', source);
  }

  if (lib === 'tkms' && args.noCompress) {
    next.push('--no-compress');
  }

  if (args.force) {
    next.push('--force');
  }

  return next;
}

function plannedInstalls(args) {
  const installs = [];

  for (const lib of selectedLibs(args)) {
    const installer = INSTALLERS[lib];
    const entries = manifestEntries(installer.manifest);

    for (const entry of entries) {
      const destination = resolve(installer.destinationRoot, `v${entry.version}`);

      if (!args.force && existsSync(destination)) {
        console.log(
          `[wasm-install] ${installer.displayName} v${entry.version}: already exists, skipping (${destination})`,
        );
        continue;
      }

      installs.push({ entry, lib, installer });
    }
  }

  return installs;
}

function runInstaller({ entry, lib, installer }, args) {
  const runArgs = [installer.script, ...installerArgs(lib, installer, entry, args)];
  const renderedCommand = commandLine('bash', runArgs);

  if (args.dryRun) {
    console.log(`[wasm-install] ${renderedCommand}`);
    return;
  }

  console.log(`[wasm-install] ${installer.displayName} v${entry.version}`);
  const result = spawnSync('bash', runArgs, {
    cwd: sdkRoot,
    env: {
      ...process.env,
      NODE: process.env.NODE ?? process.execPath,
    },
    stdio: 'inherit',
  });

  if (result.error) {
    throw result.error;
  }

  if (result.status !== 0) {
    process.exit(result.status ?? 1);
  }
}

function runCodegen(args) {
  if (args.noCodegen) {
    return;
  }

  const codegenArgs = ['scripts/build/codegen-loaders.mjs'];
  const renderedCommand = commandLine(process.execPath, codegenArgs);

  if (args.dryRun) {
    console.log(`[wasm-install] ${renderedCommand}`);
    return;
  }

  console.log('[wasm-install] regenerating source WASM loaders/API declarations');
  const result = spawnSync(process.execPath, codegenArgs, {
    cwd: sdkRoot,
    env: process.env,
    stdio: 'inherit',
  });

  if (result.error) {
    throw result.error;
  }

  if (result.status !== 0) {
    process.exit(result.status ?? 1);
  }
}

const args = parseArgs(process.argv.slice(2));

pruneStaleVersions(args);

const installs = plannedInstalls(args);

if (installs.length === 0) {
  console.log(`[wasm-install] nothing to install for lib=${args.lib}`);
} else {
  for (const install of installs) {
    runInstaller(install, args);
  }
}

runCodegen(args);
