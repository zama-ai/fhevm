// Verifies the pending git changes — staged, unstaged and untracked alike — touch only files under
// the sdk workspace. The sdk is developed inside a larger monorepo, and a commit from here must never
// smuggle a change into someone else's tree.

import { execFileSync } from 'node:child_process';
import { relative, resolve, sep } from 'node:path';

import type { Violation } from '../diagnostics.ts';

export type CommitScopeInspection = {
  readonly checkedFileKeys: readonly string[];
  readonly violations: readonly Violation[];
};

/** `git status --porcelain` from the repository root; injectable so tests need no real repository. */
function gitStatusPorcelain(repoRoot: string): string {
  return execFileSync('git', ['status', '--porcelain'], { cwd: repoRoot, encoding: 'utf8' });
}

function gitRepoRoot(workspaceRoot: string): string {
  return execFileSync('git', ['rev-parse', '--show-toplevel'], { cwd: workspaceRoot, encoding: 'utf8' }).trim();
}

/** Every repo-relative path a porcelain line touches — both sides of a rename. */
export function pathsOfPorcelainLine(line: string): readonly string[] {
  const path = line.slice(3);
  const unquote = (value: string): string =>
    value.startsWith('"') && value.endsWith('"') ? (JSON.parse(value) as string) : value;
  const arrow = path.indexOf(' -> ');
  if (arrow === -1) return [unquote(path)];
  return [unquote(path.slice(0, arrow)), unquote(path.slice(arrow + 4))];
}

export function inspectCommitScope(
  workspaceRoot: string,
  readStatus: (repoRoot: string) => string = gitStatusPorcelain,
  resolveRepoRoot: (workspaceRoot: string) => string = gitRepoRoot,
): CommitScopeInspection {
  const repoRoot = resolveRepoRoot(workspaceRoot);
  const workspacePrefix = relative(repoRoot, resolve(workspaceRoot));

  const checkedFileKeys: string[] = [];
  const violations: Violation[] = [];
  for (const line of readStatus(repoRoot).split('\n')) {
    if (line === '') continue;
    for (const path of pathsOfPorcelainLine(line)) {
      checkedFileKeys.push(path);
      if (workspacePrefix === '' || path === workspacePrefix || path.startsWith(`${workspacePrefix}${sep}`)) {
        continue;
      }
      violations.push({
        rule: 'commit-scope',
        packageKey: `./${path}`,
        message: `changed outside the sdk workspace ('${workspacePrefix}/'); commit or revert it separately`,
      });
    }
  }

  return { checkedFileKeys, violations };
}
