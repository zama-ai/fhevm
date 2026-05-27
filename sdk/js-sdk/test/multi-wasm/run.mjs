#!/usr/bin/env node

import { spawnSync } from 'node:child_process';
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';

const WASM_ASSET_LOAD_MODES = ['auto', 'embedded-base64', 'verified-blob', 'precheck-direct-url', 'trusted-direct-url'];
const LOCAL_CDN = 'local';

const matrixPath = resolve('test/multi-wasm/matrix.json');
const matrix = JSON.parse(readFileSync(matrixPath, 'utf-8'));
const supportedCdns = Object.keys(matrix.assetUrlSets);
const remoteCdns = supportedCdns.filter((cdn) => cdn !== LOCAL_CDN);

const options = parseArgs(process.argv.slice(2));

if (options.help) {
  printHelp();
  process.exit(0);
}

normalizeSelectionOptions(options);
validateSelection(options);

const env = {
  ...process.env,
  ...(options.restartLocalstack ? { MULTI_WASM_RESTART_LOCALSTACK: '1' } : {}),
  ...(options.fhevmCliProfile !== undefined ? { MULTI_WASM_FHEVM_CLI_PROFILE: options.fhevmCliProfile } : {}),
  ...(options.tfheVersion !== undefined ? { MULTI_WASM_TFHE_VERSION: options.tfheVersion } : {}),
  ...(options.kmsVersion !== undefined ? { MULTI_WASM_KMS_VERSION: options.kmsVersion } : {}),
  ...(options.mode !== undefined ? { MULTI_WASM_MODE: options.mode } : {}),
  ...(options.cdn !== undefined ? { MULTI_WASM_CDN: options.cdn } : {}),
};

const result = spawnSync(
  'npx',
  ['playwright', 'test', '--config', 'test/multi-wasm/playwright.config.ts', ...options.playwrightArgs],
  {
    env,
    stdio: 'inherit',
  },
);

process.exit(result.status ?? 1);

function parseArgs(args) {
  let restartLocalstack = false;
  let fhevmCliProfile;
  let tfheVersion;
  let kmsVersion;
  let mode;
  let cdn;
  let help = false;
  const playwrightArgs = [];

  for (let i = 0; i < args.length; i++) {
    const arg = args[i];

    if (arg === '--') {
      playwrightArgs.push(...args.slice(i + 1));
      break;
    }

    if (arg === '--restart-localstack') {
      restartLocalstack = true;
      continue;
    }

    if (arg === '--fhevm-cli-profile') {
      fhevmCliProfile = readOptionValue(args, i, '--fhevm-cli-profile');
      i++;
      continue;
    }

    if (arg.startsWith('--fhevm-cli-profile=')) {
      fhevmCliProfile = readInlineOptionValue(arg, '--fhevm-cli-profile');
      continue;
    }

    if (arg === '--tfhe') {
      tfheVersion = readOptionValue(args, i, '--tfhe');
      i++;
      continue;
    }

    if (arg.startsWith('--tfhe=')) {
      tfheVersion = readInlineOptionValue(arg, '--tfhe');
      continue;
    }

    if (arg === '--kms') {
      kmsVersion = readOptionValue(args, i, '--kms');
      i++;
      continue;
    }

    if (arg.startsWith('--kms=')) {
      kmsVersion = readInlineOptionValue(arg, '--kms');
      continue;
    }

    if (arg === '--mode') {
      mode = readOptionValue(args, i, '--mode');
      i++;
      continue;
    }

    if (arg.startsWith('--mode=')) {
      mode = readInlineOptionValue(arg, '--mode');
      continue;
    }

    if (arg === '--cdn') {
      cdn = readOptionValue(args, i, '--cdn');
      i++;
      continue;
    }

    if (arg.startsWith('--cdn=')) {
      cdn = readInlineOptionValue(arg, '--cdn');
      continue;
    }

    if (arg === '--help' || arg === '-h') {
      help = true;
      continue;
    }

    throwUsage(`Unknown option '${arg}'. Use -- to pass arguments through to Playwright.`);
  }

  return { cdn, fhevmCliProfile, help, kmsVersion, mode, playwrightArgs, restartLocalstack, tfheVersion };
}

function readOptionValue(args, index, optionName) {
  const value = args[index + 1];
  if (value === undefined || value.startsWith('--')) {
    throwUsage(`${optionName} requires a value.`);
  }
  return value;
}

function readInlineOptionValue(arg, optionName) {
  const value = arg.slice(`${optionName}=`.length);
  if (value === '') {
    throwUsage(`${optionName} requires a value.`);
  }
  return value;
}

function normalizeModuleVersion(version) {
  return version.trim().replace(/^v/i, '');
}

function normalizeSelectionOptions(options) {
  if (options.tfheVersion !== undefined) {
    options.tfheVersion = normalizeModuleVersion(options.tfheVersion);
  }
  if (options.kmsVersion !== undefined) {
    options.kmsVersion = normalizeModuleVersion(options.kmsVersion);
  }
  if (options.mode !== undefined) {
    options.mode = options.mode.trim();
  }
  if (options.cdn !== undefined) {
    options.cdn = options.cdn.trim().toLowerCase();
  }
  if (options.fhevmCliProfile !== undefined) {
    options.fhevmCliProfile = options.fhevmCliProfile.trim();
  }
}

function validateSelection(options) {
  if (options.fhevmCliProfile === '') {
    throwUsage('--fhevm-cli-profile requires a non-empty value.');
  }

  if (options.fhevmCliProfile !== undefined && !options.restartLocalstack) {
    throwUsage('--fhevm-cli-profile requires --restart-localstack.');
  }

  if (options.mode !== undefined && !WASM_ASSET_LOAD_MODES.includes(options.mode)) {
    throwUsage(`Unknown --mode '${options.mode}'. Supported modes: ${WASM_ASSET_LOAD_MODES.join(', ')}.`);
  }

  if (options.cdn !== undefined && !supportedCdns.includes(options.cdn)) {
    throwUsage(`Unknown --cdn '${options.cdn}'. Supported CDNs: ${supportedCdns.join(', ')}.`);
  }

  if (options.mode === 'embedded-base64' && options.cdn !== undefined && options.cdn !== LOCAL_CDN) {
    throwUsage(`--mode embedded-base64 is incompatible with --cdn ${options.cdn}.`);
  }
}

function throwUsage(message) {
  console.error(message);
  console.error('');
  printHelp();
  process.exit(1);
}

function printHelp() {
  console.log(`Usage: npm run test:multi-wasm -- [options] [-- playwright args]

All flags are independent filters over the auto-generated matrix
(supportedVersionPairs x wasmAssetLoadMode x cdn, minus embedded-base64 x non-local).
Omitting a flag runs every value on that axis.

Options:
  --restart-localstack       Restart localstack before the browser suite.
  --fhevm-cli-profile <name> Profile filename forwarded to localstack-restart.sh.
  --tfhe <version>           Filter by TFHE version. Leading "v" is accepted.
  --kms <version>            Filter by TKMS version. Leading "v" is accepted.
  --mode <mode>              Filter by wasmAssetLoadMode.
  --cdn <cdn>                Filter by CDN. Defaults to all CDNs.
  -h, --help                 Show this help.

Supported modes:
  ${WASM_ASSET_LOAD_MODES.join('\n  ')}

Supported CDNs:
  ${supportedCdns.join('\n  ')}

Supported version pairs:
  ${matrix.supportedVersionPairs.map((pair) => `tfhe ${pair.tfhe}, kms ${pair.kms}`).join('\n  ')}

Examples:
  npm run test:multi-wasm                                                             # all entries
  npm run test:multi-wasm -- --tfhe 1.5.3 --kms 0.13.10                               # all entries for that pair
  npm run test:multi-wasm -- --tfhe 1.5.3 --kms 0.13.10 --cdn local                   # all modes for that pair, local only
  npm run test:multi-wasm -- --mode embedded-base64                                   # one entry per pair
  npm run test:multi-wasm -- --cdn local                                              # all local entries
  npm run test:multi-wasm -- --tfhe 1.5.3 --kms 0.13.10 --mode verified-blob --cdn ${remoteCdns[0] ?? 'jsdelivr'}  # exactly one
  npm run test:multi-wasm -- --restart-localstack
  npm run test:multi-wasm -- --restart-localstack --fhevm-cli-profile v0.11.0-mainnet.json
  npm run test:multi-wasm -- --mode auto --cdn local -- --headed
`);
}
