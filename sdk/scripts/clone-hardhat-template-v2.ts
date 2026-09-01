#!/usr/bin/env node
//
// Rebuilds `hardhat/v2/fhevm-hardhat-template` from a fresh clone of the public template.
//
// Usage: npm run clone:hardhat-template-v2 [-- --force] [--ref <branch|tag>] [--install]
//
//   --force    replace the template directory. Required whenever it already exists.
//   --ref      clone this branch or tag instead of the default branch.
//   --install  install afterwards, by handing off to setup:hardhat-template-v2.
//
// ## What it produces
//
// The template is mirrored BYTE-FOR-BYTE to github.com/zama-ai/fhevm-hardhat-template, so this makes
// exactly three divergences and no others:
//
//   - `package.json` drops the v0.11 relayer generation (@fhevm/mock-utils, @zama-fhe/relayer-sdk) for
//     the v0.13 one, taking the two @fhevm packages from workspace candidate directories instead of the registry,
//     and renames the package so an installed tree is never mistaken for the published template.
//   - `.github/` is deleted. The public CI runs `npm ci` against registry ranges, which cannot resolve a
//     workspace-relative `file:` directories, so it would fail on every push here.
//   - `package-lock.json` is deleted. It locks the generation the manifest patch replaces, so the next
//     install has to resolve from scratch; `setup:hardhat-template-v2` regenerates it.
//
// The clone's `.git` is never copied. The workspace copy is ordinary source owned by this repository;
// `check:mirror` checks it against a fresh temporary clone.
//
// It does not install unless asked. `--install` hands off to `setup:hardhat-template-v2` rather than
// running `npm install` itself, so there is one install path with npm's packed-directory semantics.
import { findWorkspaceRootAbsPath } from '@fhevm/sdk-common-dev';
import { patchHardhatTemplateV2Manifest } from '../fhevm-npm/base/mirrors/hardhat-template-v2.ts';
import { execFileSync } from 'node:child_process';
import { cpSync, existsSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';

const USAGE = 'usage: npm run clone:hardhat-template-v2 [-- --force] [--ref <branch|tag>] [--install]';

/** The install step, handed the whole job so both entry points install identically. */
const SETUP_SCRIPT_REL = join('scripts', 'setup-hardhat-template-v2.ts');

const UPSTREAM = 'https://github.com/zama-ai/fhevm-hardhat-template.git';

/** The name upstream's manifest carries. A clone that does not have it is not the template. */
const UPSTREAM_NAME = 'fhevm-hardhat-template';

/** The dev owner, relative to the workspace root. It holds the mirror and is never itself replaced. */
const TEMPLATE_OWNER_REL = join('hardhat', 'v2', 'fhevm-hardhat-template');

/** The mirrored template itself. Only this is wiped and re-cloned; the owner's package.json survives. */
const TEMPLATE_REL = join(TEMPLATE_OWNER_REL, 'pkg');

type Options = { force: boolean; install: boolean; ref: string | undefined };

/** The leading comment block doubles as the help text, so there is only one copy of it. */
function usage(): void {
  const source = readFileSync(new URL(import.meta.url), 'utf8');
  for (const line of source.split('\n').slice(1)) {
    if (!line.startsWith('//')) break;
    console.log(line.replace(/^\/\/ ?/, ''));
  }
}

function parseArgs(argv: readonly string[]): Options {
  let force = false;
  let install = false;
  let ref: string | undefined;

  for (let index = 0; index < argv.length; index++) {
    const token = argv[index];
    if (token === undefined) continue;

    if (token === '-h' || token === '--help') {
      usage();
      process.exit(0);
    } else if (token === '--force') {
      force = true;
    } else if (token === '--install') {
      install = true;
    } else if (token === '--ref') {
      const value = argv[index + 1];
      // Rejecting a leading `-` so `--ref --force` fails instead of cloning a branch named "--force".
      if (value === undefined || value.startsWith('-')) {
        throw new Error(`--ref requires a value\n${USAGE}`);
      }
      ref = value;
      index++;
    } else if (token.startsWith('--ref=')) {
      const value = token.slice('--ref='.length);
      if (value === '') throw new Error(`--ref requires a value\n${USAGE}`);
      ref = value;
    } else {
      throw new Error(`unexpected argument ${JSON.stringify(token)}\n${USAGE}`);
    }
  }

  return { force, install, ref };
}

function readManifest(file: string): Record<string, unknown> {
  const parsed: unknown = JSON.parse(readFileSync(file, 'utf8'));
  if (typeof parsed !== 'object' || parsed === null || Array.isArray(parsed)) {
    throw new Error(`${file} is not a JSON object`);
  }
  return parsed as Record<string, unknown>;
}

/**
 * Swaps the relayer generation for the v0.13 one. Reassigning the two dependency fields in place keeps
 * every other key where upstream put it, so the diff stays the dependency change and nothing else.
 */
function patchManifest(templateDir: string): void {
  const file = join(templateDir, 'package.json');
  const manifest = patchHardhatTemplateV2Manifest(readManifest(file), (message) => console.log(`   ${message}`));

  // Trailing newline: the template's own `prettier:check` covers package.json and fails without one.
  writeFileSync(file, `${JSON.stringify(manifest, null, 2)}\n`);
}

function main(): void {
  const { force, install, ref } = parseArgs(process.argv.slice(2));
  const workspaceRoot = findWorkspaceRootAbsPath(import.meta.dirname);
  const templateDir = join(workspaceRoot, TEMPLATE_REL);

  if (existsSync(templateDir) && !force) {
    console.error(`   ❌ ${templateDir} already exists.`);
    console.error('      Rebuilding replaces it wholesale — its node_modules, its package-lock.json and any');
    console.error('      local edit to a mirrored file are all lost. Pass --force once you are sure.');
    process.exit(1);
  }

  const tmpRoot = mkdtempSync(join(tmpdir(), 'fhevm-hardhat-template-'));
  const clonedDir = join(tmpRoot, UPSTREAM_NAME);

  try {
    console.log(`📦 cloning ${UPSTREAM}${ref === undefined ? '' : ` @ ${ref}`}`);
    const cloneArgs = ['clone', '--depth', '1'];
    if (ref !== undefined) cloneArgs.push('--branch', ref);
    execFileSync('git', [...cloneArgs, UPSTREAM, clonedDir], { stdio: 'inherit' });

    const cloned = readManifest(join(clonedDir, 'package.json'));
    if (cloned['name'] !== UPSTREAM_NAME) {
      throw new Error(`cloned package.json is named ${JSON.stringify(cloned['name'])}, expected ${UPSTREAM_NAME}`);
    }

    if (existsSync(templateDir)) {
      rmSync(templateDir, { recursive: true, force: true });
      console.log(`   🧹 removed the previous ${TEMPLATE_REL}`);
    }
    const clonedGitDirectory = join(clonedDir, '.git');
    cpSync(clonedDir, templateDir, {
      recursive: true,
      filter: (source) => source !== clonedGitDirectory,
    });
    console.log(`   → ${templateDir}`);

    const workflows = join(templateDir, '.github');
    if (existsSync(workflows)) {
      rmSync(workflows, { recursive: true, force: true });
      console.log('   🧹 removed .github (its CI installs from the registry, not workspace directories)');
    }

    // Upstream's lock pins the generation the manifest patch is replacing, down to the integrity hash
    // of every @fhevm package. Reconciling it is what npm does badly and silently; deleting it makes the
    // next install resolve from scratch.
    const lockfile = join(templateDir, 'package-lock.json');
    if (existsSync(lockfile)) {
      rmSync(lockfile, { force: true });
      console.log('   🧹 removed package-lock.json (it pins the generation being replaced)');
    }

    console.log('   patching package.json:');
    patchManifest(templateDir);

    console.log(`   ✅ clone complete.${install ? '' : ' Next: npm run setup:hardhat-template-v2'}`);

    if (install) {
      console.log('');
      execFileSync(process.execPath, [join(workspaceRoot, SETUP_SCRIPT_REL)], { stdio: 'inherit' });
    }

    console.log('');
    console.log(`   Check the mirror: (cd ${TEMPLATE_OWNER_REL} && npm run check:mirror)`);
  } finally {
    rmSync(tmpRoot, { recursive: true, force: true });
  }
}

main();
