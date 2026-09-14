import { execFileSync } from 'node:child_process';

export function gitRepositoryRoot(path: string): string {
  return execFileSync('git', ['-C', path, 'rev-parse', '--show-toplevel'], { encoding: 'utf8' }).trim();
}

export function gitVisibleFiles(path: string): readonly string[] {
  return execFileSync('git', ['-C', path, 'ls-files', '--cached', '--others', '--exclude-standard', '--', '.'], {
    encoding: 'utf8',
  })
    .split('\n')
    .filter(Boolean);
}
