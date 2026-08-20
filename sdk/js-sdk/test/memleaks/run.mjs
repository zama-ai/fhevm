#!/usr/bin/env node

// Thin dispatcher: sets up the flags main.ts needs (a working directory
// relative to the sdk/js-sdk package root, and --expose-gc so the sampler can
// force a GC pass before every measurement) and hands off to main.ts via tsx,
// the same way test/multi-wasm/run.mjs hands off to Playwright. All the real
// logic lives in main.ts.
//
// Runs can take tens of minutes (see each scenario's defaultIterationsDuration),
// so this also wraps the child process with a platform-appropriate
// sleep-inhibitor — `caffeinate` on macOS, `systemd-inhibit` on Linux — rather
// than relying on the caller to remember to do that themselves. Falls back to
// running unwrapped (with a warning) when the inhibitor binary isn't installed,
// and skips it entirely on platforms with no known equivalent.

import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';
import { dirname, resolve } from 'node:path';

const sdkRoot = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
const mainTsPath = resolve(dirname(fileURLToPath(import.meta.url)), 'main.ts');

const existingNodeOptions = process.env.NODE_OPTIONS ?? '';
const nodeOptions = existingNodeOptions.includes('--expose-gc')
  ? existingNodeOptions
  : `${existingNodeOptions} --expose-gc`.trim();

const spawnOptions = {
  cwd: sdkRoot,
  env: { ...process.env, NODE_OPTIONS: nodeOptions },
  stdio: 'inherit',
};

const tsxArgs = ['tsx', mainTsPath, ...process.argv.slice(2)];

const sleepInhibitor =
  process.platform === 'darwin'
    ? { command: 'caffeinate', args: ['-i'] }
    : process.platform === 'linux'
      ? { command: 'systemd-inhibit', args: ['--what=idle:sleep', '--why=fhevm memleak tests'] }
      : undefined;

let result = sleepInhibitor
  ? spawnSync(sleepInhibitor.command, [...sleepInhibitor.args, 'npx', ...tsxArgs], spawnOptions)
  : spawnSync('npx', tsxArgs, spawnOptions);

if (sleepInhibitor && result.error?.code === 'ENOENT') {
  console.warn(`[memleaks] "${sleepInhibitor.command}" not found — running without sleep prevention.`);
  result = spawnSync('npx', tsxArgs, spawnOptions);
}

if (result.error) {
  console.error(result.error);
  process.exit(1);
}

process.exit(result.status ?? 1);
