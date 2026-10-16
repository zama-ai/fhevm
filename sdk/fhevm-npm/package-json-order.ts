// Canonical order for top-level package.json entries.
// Edit this list to change the repository policy. Unlisted entries sort alphabetically after listed entries.
export const packageJsonFieldOrder = [
  'name',
  'version',
  'description',
  'private',
  'license',
  'type',
  'author',
  'contributors',
  'funding',
  'homepage',
  'repository',
  'bugs',
  'keywords',
  'engines',
  'packageManager',
  'os',
  'cpu',
  'main',
  'module',
  'browser',
  'types',
  'typings',
  'typesVersions',
  'sideEffects',
  'files',
  'exports',
  'imports',
  'bin',
  'man',
  'workspaces',
  'scripts',
  'config',
  'dependencies',
  'devDependencies',
  'peerDependencies',
  'peerDependenciesMeta',
  'optionalDependencies',
  'bundledDependencies',
  'overrides',
  'publishConfig',
  'fhevm',
] as const;

const fieldPositions = new Map<string, number>(packageJsonFieldOrder.map((field, index) => [field, index]));

export function sortPackageJsonFields(fields: readonly string[]): readonly string[] {
  return [...fields].sort((left, right) => {
    const leftPosition = fieldPositions.get(left);
    const rightPosition = fieldPositions.get(right);
    if (leftPosition !== undefined && rightPosition !== undefined) return leftPosition - rightPosition;
    if (leftPosition !== undefined) return -1;
    if (rightPosition !== undefined) return 1;
    return left.localeCompare(right);
  });
}
