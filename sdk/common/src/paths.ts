// Locating the sdk workspace, the fhevm repo around it, and the files that live at either root.

import { execFileSync } from 'node:child_process';
import { existsSync, readFileSync } from 'node:fs';
import { dirname, join, relative } from 'node:path';
import { fileURLToPath } from 'node:url';

/** This module's own directory — the anchor for anything found relative to the workspace. */
const THIS_DIR = dirname(fileURLToPath(import.meta.url));

/** Path of ZamaConfig.sol relative to the fhevm repo root. */
const ZAMA_CONFIG_REPO_PATH = join('library-solidity', 'config', 'ZamaConfig.sol');

/**
 * Nearest ancestor of `startDir` whose package.json declares `workspaces`.
 *
 * @param startDir Absolute path to start from — the calling package's own root.
 * @throws if no ancestor declares `workspaces`.
 * @example
 * findWorkspaceRootAbsPath('/repo/sdk/host-contracts-cleartext/v13'); // '/repo/sdk'
 */
export function findWorkspaceRootAbsPath(startDir: string): string {
  let current = startDir;
  for (;;) {
    const manifest = join(current, 'package.json');
    if (existsSync(manifest)) {
      const parsed: unknown = JSON.parse(readFileSync(manifest, 'utf8'));
      if (typeof parsed === 'object' && parsed !== null && 'workspaces' in parsed) {
        return current;
      }
    }
    const parent = dirname(current);
    if (parent === current) {
      throw new Error(`no package.json declaring "workspaces" above ${startDir}`);
    }
    current = parent;
  }
}

/**
 * The shared `tarballs` directory at the workspace root, where every member collects its tarball.
 * Members only add to it; only the workspace root clears it.
 *
 * @param startDir Absolute path inside the workspace — the calling package's own root.
 * @example
 * workspaceTarballsDirAbsPath('/repo/sdk/js-sdk'); // '/repo/sdk/tarballs'
 */
export function workspaceTarballsDirAbsPath(startDir: string): string {
  return join(findWorkspaceRootAbsPath(startDir), 'tarballs');
}

/** The fhevm repo root according to git, or undefined outside a checkout. */
function _gitRepoRoot(): string | undefined {
  try {
    return execFileSync('git', ['rev-parse', '--show-toplevel'], {
      cwd: THIS_DIR,
      encoding: 'utf8',
      stdio: ['ignore', 'pipe', 'ignore'],
    }).trim();
  } catch {
    return undefined;
  }
}

/**
 * Absolute path of `library-solidity/config/ZamaConfig.sol`, tried as the sdk workspace's sibling then
 * from the git repo root — so moving a package within the repo cannot silently lose the file.
 *
 * @throws if neither candidate exists, which means it moved rather than that it may be skipped.
 * @example
 * zamaConfigAbsPath(); // '/repo/library-solidity/config/ZamaConfig.sol'
 */
export function zamaConfigAbsPath(): string {
  const layoutRelative = join(dirname(findWorkspaceRootAbsPath(THIS_DIR)), ZAMA_CONFIG_REPO_PATH);
  const repoRoot = _gitRepoRoot();
  const candidates =
    repoRoot === undefined ? [layoutRelative] : [...new Set([layoutRelative, join(repoRoot, ZAMA_CONFIG_REPO_PATH)])];

  const found = candidates.find((candidate) => existsSync(candidate));
  if (found === undefined) {
    throw new Error(
      `ZamaConfig.sol not found. Tried:\n${candidates.map((candidate) => `     ${candidate}`).join('\n')}\n` +
        `   It is the source of truth for the localhost address set, so this check cannot be skipped: fix ` +
        `the path in @fhevm/sdk-common (src/paths.ts) if the file moved.`,
    );
  }

  return found;
}

/**
 * How to name a path in output: relative to the fhevm repo root when it sits inside one, absolute
 * otherwise.
 *
 * @example
 * sourceLabel('/repo/library-solidity/config/ZamaConfig.sol'); // 'library-solidity/config/ZamaConfig.sol'
 */
export function sourceLabel(sourcePath: string): string {
  const repoRoot = _gitRepoRoot();
  if (repoRoot === undefined) {
    return sourcePath;
  }

  const fromRoot = relative(repoRoot, sourcePath);
  return fromRoot.startsWith('..') ? sourcePath : fromRoot;
}
