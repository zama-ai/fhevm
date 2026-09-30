import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import { z } from 'zod';

import type { NpmManifest, NpmManifestEntry } from '../manifest.ts';
import { packageDirectory } from './paths.ts';

export const dependencyFields = [
  'dependencies',
  'devDependencies',
  'peerDependencies',
  'optionalDependencies',
] as const;

export type DependencyField = (typeof dependencyFields)[number];
export type DependencyMap = Readonly<Record<string, string>>;

const dependencyMapSchema = z.record(z.string(), z.string());
const packageJsonSchema = z
  .object({
    name: z.string().min(1).optional(),
    version: z.string().min(1).optional(),
    private: z.boolean().optional(),
    type: z.enum(['commonjs', 'module']).optional(),
    main: z.string().min(1).optional(),
    module: z.string().min(1).optional(),
    types: z.string().min(1).optional(),
    typings: z.string().min(1).optional(),
    exports: z.unknown().optional(),
    imports: z.unknown().optional(),
    engines: z.record(z.string(), z.string()).optional(),
    workspaces: z.array(z.string()).optional(),
    scripts: z.record(z.string(), z.string()).optional(),
    dependencies: dependencyMapSchema.optional(),
    devDependencies: dependencyMapSchema.optional(),
    peerDependencies: dependencyMapSchema.optional(),
    optionalDependencies: dependencyMapSchema.optional(),
  })
  .passthrough();

export type PackageJson = z.infer<typeof packageJsonSchema>;

export type LoadedPackage = {
  readonly key: string;
  readonly directory: string;
  readonly inventory: NpmManifestEntry;
  readonly packageJson: PackageJson;
};

export type DependencyDeclaration = {
  readonly field: DependencyField;
  readonly name: string;
  readonly spec: string;
};

export function readPackageJson(file: string): PackageJson {
  let value: unknown;
  try {
    value = JSON.parse(readFileSync(file, 'utf8')) as unknown;
  } catch (error) {
    throw new Error(`Unable to parse ${file}: ${errorMessage(error)}`);
  }

  const result = packageJsonSchema.safeParse(value);
  if (!result.success) {
    throw new Error(`Invalid package.json ${file}: ${z.prettifyError(result.error)}`);
  }
  const originalKeys = Object.keys(value as Record<string, unknown>);
  return Object.fromEntries(originalKeys.map((key) => [key, result.data[key]])) as PackageJson;
}

export function loadPackages(workspaceRoot: string, manifest: NpmManifest): readonly LoadedPackage[] {
  return Object.entries(manifest.packages)
    .map(([key, inventory]) => {
      const directory = packageDirectory(workspaceRoot, key);
      return {
        key,
        directory,
        inventory,
        packageJson: readPackageJson(join(directory, 'package.json')),
      } satisfies LoadedPackage;
    })
    .sort((left, right) => left.key.localeCompare(right.key));
}

export function dependencyDeclarations(packageJson: PackageJson): readonly DependencyDeclaration[] {
  return dependencyFields.flatMap((field) =>
    Object.entries(packageJson[field] ?? {}).map(([name, spec]) => ({ field, name, spec })),
  );
}

export function declarationsByName(packageJson: PackageJson): ReadonlyMap<string, readonly DependencyDeclaration[]> {
  const result = new Map<string, DependencyDeclaration[]>();
  for (const declaration of dependencyDeclarations(packageJson)) {
    const declarations = result.get(declaration.name) ?? [];
    declarations.push(declaration);
    result.set(declaration.name, declarations);
  }
  return result;
}

export function isExactVersion(value: string): boolean {
  return /^\d+\.\d+\.\d+(?:-[0-9A-Za-z.-]+)?(?:\+[0-9A-Za-z.-]+)?$/.test(value);
}

function errorMessage(error: unknown): string {
  return error instanceof Error ? error.message : String(error);
}
