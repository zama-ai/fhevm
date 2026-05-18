#!/usr/bin/env node
import { spawnSync } from 'node:child_process';
import { existsSync } from 'node:fs';
import { dirname, isAbsolute, resolve } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

import { BUILD_PROFILES, KMS_MANIFEST, TFHE_MANIFEST } from '../../versionsManifest.js';

const scriptDir = dirname(fileURLToPath(import.meta.url));
const sdkRoot = resolve(scriptDir, '../..');

const INSTALLERS = Object.freeze({
  tfhe: Object.freeze({
    displayName: 'TFHE',
    manifest: TFHE_MANIFEST,
    packageName: 'tfhe',
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
  'Installs missing WASM package versions listed in versionsManifest.js.',
  'Manifest entries may set source to any npm install spec, including file: URLs.',
  '',
  'Options:',
  '  --profile <dev|prod|all>  Manifest profile to install. Defaults to BUILD_PROFILE or dev.',
  '  --lib <tfhe|tkms|kms|all>  Library to install. Defaults to all.',
  '  --force, -y               Reinstall versions even when destination directories exist.',
  '  --no-compress             Forward to TKMS wasm base64 generation.',
  '  --no-codegen              Do not regenerate source WASM loaders/API declarations after install.',
  '  --dry-run                 Print installer commands without running them.',
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
    profile: process.env.BUILD_PROFILE ?? 'dev',
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

    if (arg === '--profile') {
      args.profile = argv[++i];
      if (!args.profile) fail(`Missing value for ${arg}.\n\n${usage}`);
      continue;
    }

    if (arg === '--lib') {
      args.lib = argv[++i];
      if (!args.lib) fail(`Missing value for ${arg}.\n\n${usage}`);
      continue;
    }

    fail(`Unknown argument: ${arg}\n\n${usage}`);
  }

  if (![...BUILD_PROFILES, 'all'].includes(args.profile)) {
    fail(`Unknown profile '${args.profile}'. Expected one of: ${[...BUILD_PROFILES, 'all'].join(', ')}`);
  }

  if (args.lib === 'kms') {
    args.lib = 'tkms';
  }

  if (!['tfhe', 'tkms', 'all'].includes(args.lib)) {
    fail(`Unknown lib '${args.lib}'. Expected one of: tfhe, tkms, kms, all`);
  }

  return args;
}

function manifestEntries(manifest, profile) {
  const entries = profile === 'all' ? manifest : manifest.filter((entry) => entry.tags.includes(profile));
  const seen = new Set();
  const unique = [];

  for (const entry of entries) {
    if (seen.has(entry.version)) {
      continue;
    }

    seen.add(entry.version);
    unique.push(entry);
  }

  return unique;
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

function sourceForEntry(installer, entry) {
  if (entry.source !== undefined && typeof entry.source !== 'string') {
    fail(`${installer.displayName} v${entry.version} source must be a string when provided.`);
  }

  return normalizeSource(entry.source ?? `${installer.packageName}@${entry.version}`);
}

function defaultSourceForEntry(installer, entry) {
  return `${installer.packageName}@${entry.version}`;
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
  const libs = args.lib === 'all' ? ['tfhe', 'tkms'] : [args.lib];
  const installs = [];

  for (const lib of libs) {
    const installer = INSTALLERS[lib];
    const entries = manifestEntries(installer.manifest, args.profile);

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
  const renderedCommand = `BUILD_PROFILE=dev ${commandLine(process.execPath, codegenArgs)}`;

  if (args.dryRun) {
    console.log(`[wasm-install] ${renderedCommand}`);
    return;
  }

  console.log('[wasm-install] regenerating source WASM loaders/API declarations (profile=dev)');
  const result = spawnSync(process.execPath, codegenArgs, {
    cwd: sdkRoot,
    env: {
      ...process.env,
      BUILD_PROFILE: 'dev',
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

const args = parseArgs(process.argv.slice(2));
const installs = plannedInstalls(args);

if (installs.length === 0) {
  console.log(`[wasm-install] nothing to install for profile=${args.profile} lib=${args.lib}`);
} else {
  for (const install of installs) {
    runInstaller(install, args);
  }
}

runCodegen(args);
