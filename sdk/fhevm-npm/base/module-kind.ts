import type { PackageJson } from './npm.ts';

export type ModuleKind = 'cjs' | 'esm';

export function consumerModuleKinds(packageJson: PackageJson): readonly ModuleKind[] {
  const conditions = exportConditions(packageJson.exports);
  const hasEsmEntry = conditions.has('import') || packageJson.module !== undefined;
  const hasCjsEntry =
    conditions.has('require') ||
    (packageJson.main !== undefined && (hasEsmEntry || packageJson.type !== 'module'));

  const kinds: ModuleKind[] = [];
  if (hasCjsEntry) kinds.push('cjs');
  if (hasEsmEntry || (!hasCjsEntry && packageJson.type === 'module')) kinds.push('esm');
  if (kinds.length === 0) kinds.push('cjs');
  return kinds;
}

function exportConditions(value: unknown, result = new Set<string>()): ReadonlySet<string> {
  if (Array.isArray(value)) {
    for (const element of value) exportConditions(element, result);
    return result;
  }
  if (typeof value !== 'object' || value === null) return result;
  for (const [key, nested] of Object.entries(value)) {
    if (key === 'import' || key === 'require') result.add(key);
    exportConditions(nested, result);
  }
  return result;
}
