// `sdk/versions.json` is the one authority for every published payload's version (PACKAGE_VERSIONS_PLAN.md).
// This module reads it and proves it agrees with the manifest: one entry per `kind: published` payload,
// in manifest order, every value canonical SemVer. Package.json versions are derived from it, never read
// into it — the only exception was the bootstrap commit that created the file.

import { existsSync, readFileSync } from 'node:fs';
import { join } from 'node:path';

import { z } from 'zod';

import type { NpmManifest } from '../manifest.ts';
import type { Violation } from './diagnostics.ts';
import { isCanonicalVersion } from './semver.ts';

/** Beside npm-manifest.json at the workspace root; hardcoded like the manifest's own name. */
export const VERSIONS_FILE = 'versions.json';
export const VERSIONS_SCHEMA_REFERENCE = './fhevm-npm/schemas/versions.schema.json';

const versionsFileSchema = z.object({
  $schema: z.literal(VERSIONS_SCHEMA_REFERENCE),
  schemaVersion: z.literal(1),
  packages: z.record(z.string(), z.string()),
});

export type VersionsFile = z.infer<typeof versionsFileSchema>;

export function versionsPath(workspaceRoot: string): string {
  return join(workspaceRoot, VERSIONS_FILE);
}

/** The parsed file; a missing or malformed file is an error, not a violation, because nothing can run without it. */
export function loadVersions(workspaceRoot: string): VersionsFile {
  const file = versionsPath(workspaceRoot);
  if (!existsSync(file)) throw new Error(`${VERSIONS_FILE} is missing at the workspace root: ${file}`);
  const parsed = versionsFileSchema.safeParse(JSON.parse(readFileSync(file, 'utf8')));
  if (!parsed.success) throw new Error(`${VERSIONS_FILE}: ${parsed.error.issues.map((i) => i.message).join('; ')}`);
  return parsed.data;
}

/** The manifest keys that must have a central version, in the order the file must list them. */
export function publishedPayloadKeys(manifest: NpmManifest): readonly string[] {
  return Object.entries(manifest.packages)
    .filter(([, entry]) => entry.kind === 'published')
    .map(([key]) => key);
}

/** Coverage both ways, canonical values, manifest order — the graph is valid when this returns nothing. */
export function validateVersionGraph(manifest: NpmManifest, versions: VersionsFile): readonly Violation[] {
  const expected = publishedPayloadKeys(manifest);
  const actual = Object.keys(versions.packages);
  return [
    ...expected.filter((key) => !actual.includes(key)).map((key) => coverage(key, 'published payload is missing from')),
    ...actual
      .filter((key) => !expected.includes(key))
      .map((key) => coverage(key, 'is not a published payload, remove it from')),
    ...actual.flatMap((key) => canonical(key, versions.packages[key] ?? '')),
    ...orderViolations(expected, actual),
  ];
}

function coverage(key: string, message: string): Violation {
  return { rule: 'version-coverage', packageKey: key, message: `${message} ${VERSIONS_FILE}` };
}

function canonical(key: string, value: string): readonly Violation[] {
  if (isCanonicalVersion(value)) return [];
  return [
    { rule: 'version-semver', packageKey: key, message: `"${value}" is not canonical SemVer (x.y.z or x.y.z-pre)` },
  ];
}

// Only judged on the keys both sides share, so a coverage gap is reported once, not twice.
function orderViolations(expected: readonly string[], actual: readonly string[]): readonly Violation[] {
  const shared = actual.filter((key) => expected.includes(key));
  const inManifestOrder = expected.filter((key) => shared.includes(key));
  if (shared.join('\n') === inManifestOrder.join('\n')) return [];
  return [
    {
      rule: 'version-order',
      packageKey: `./${VERSIONS_FILE}`,
      message: `entries must follow npm-manifest.json order: ${inManifestOrder.join(', ')}`,
    },
  ];
}
