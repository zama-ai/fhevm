import { readFileSync } from 'node:fs';
import { z } from 'zod';

const PACKAGE_KEY = /^\.(?:\/(?!\.{1,2}(?:\/|$))[A-Za-z0-9._-]+)*$/;
const PREFIXED_PATH = /^\.(?:\/(?!\.{1,2}(?:\/|$))[A-Za-z0-9._-]+)+$/;
const UNPREFIXED_PATH = /^(?!\.{1,2}(?:\/|$))[A-Za-z0-9._-]+(?:\/(?!\.{1,2}(?:\/|$))[A-Za-z0-9._-]+)*$/;
const FILE_NAME = /^(?!\.{1,2}$)[A-Za-z0-9._-]+$/;
const HTTPS_URL = /^https:\/\/[^\s]+$/;

export const packageKinds = [
  'published',
  'dev',
  'shared-helper',
  'internal-consumer',
  'standalone',
  'non-package',
  'workspace-root',
] as const;

/**
 * How a payload reaches the outside world. Orthogonal to `kind`, because the two do not correlate: a
 * payload can be published to npm, mirrored to a public repository, or both — `host-contracts-cleartext`
 * is npm-published and will also be mirrored, while `fhevm-hardhat-template` is only ever mirrored.
 *
 * Omitted means `["npm"]`, so every pre-existing entry keeps its meaning. The `pack:tarball` requirement
 * reads this now; the remaining rules that should key off it (own lockfile, sibling deps by plain version,
 * publint/consumer/build) still test `kind` and are deliberately unwired until each one's question is settled.
 */
export const distributionChannels = ['npm', 'mirror'] as const;

const distributionSchema = z.array(z.enum(distributionChannels)).min(1).superRefine(uniqueStrings);
const consumerTestsSchema = z
  .object({
    cjs: z.string().regex(PREFIXED_PATH, 'must be a safe path with a leading ./').optional(),
    esm: z.string().regex(PREFIXED_PATH, 'must be a safe path with a leading ./').optional(),
  })
  .strict()
  .refine((value) => value.cjs !== undefined || value.esm !== undefined, 'must select at least one consumer');

const mirrorSchema = z
  .object({
    repository: z.string().regex(HTTPS_URL, 'must be a canonical HTTPS repository URL'),
  })
  .strict();

const inventorySchema = z
  .object({
    exclude: z
      .array(z.string().regex(PREFIXED_PATH, 'must be a safe path with a leading ./'))
      .superRefine((paths, context) => {
        if (new Set(paths).size !== paths.length) {
          context.addIssue({ code: 'custom', message: 'must not contain duplicate paths' });
        }
      }),
  })
  .strict();

const dependencyPolicySchema = z
  .object({
    forbidden: z.array(z.string().min(1)).min(1).superRefine(uniqueStrings),
  })
  .strict();

const foundryPolicySchema = z
  .object({
    version: z.string().regex(/^\d+\.\d+\.\d+(?:-[0-9A-Za-z.-]+)?$/, 'must be an exact Foundry version'),
  })
  .strict();

const tarballsPolicySchema = z
  .object({
    relPath: z.string().regex(/^\.\/[^.]/, 'must be an sdk-relative path such as ./tarballs'),
  })
  .strict();

const packageJsonFieldNamesSchema = z
  .array(z.string().regex(/^[A-Za-z][A-Za-z0-9._-]*$/, 'must be a top-level package.json field name'))
  .superRefine(uniqueStrings);

const packageJsonKindPolicySchema = z
  .object({
    required: packageJsonFieldNamesSchema,
    excluded: packageJsonFieldNamesSchema,
  })
  .strict()
  .superRefine((policy, context) => {
    for (const field of policy.required) {
      if (policy.excluded.includes(field))
        issue(context, ['excluded'], `'${field}' cannot be both required and excluded`);
    }
  });

const packageJsonPolicySchema = z
  .object({
    published: packageJsonKindPolicySchema.optional(),
    dev: packageJsonKindPolicySchema.optional(),
    'shared-helper': packageJsonKindPolicySchema.optional(),
    'internal-consumer': packageJsonKindPolicySchema.optional(),
    standalone: packageJsonKindPolicySchema.optional(),
    'non-package': packageJsonKindPolicySchema.optional(),
    'workspace-root': packageJsonKindPolicySchema.optional(),
  })
  .strict()
  .refine((policy) => Object.keys(policy).length > 0, 'must define at least one package kind policy');

const pinnedSourceSchema = z
  .object({
    repository: z.string().regex(HTTPS_URL, 'must be an HTTPS repository URL'),
    tag: z.string().min(1),
    commit: z.string().regex(/^[0-9a-f]{40}$/, 'must be a full lowercase commit SHA'),
    from: z.string().regex(UNPREFIXED_PATH, 'must be a safe repository-relative path'),
  })
  .strict();

const vendoredElementSchema = z
  .object({
    relPath: z.string().regex(PREFIXED_PATH, 'must be a safe path with a leading ./'),
    files: z.array(z.string().regex(FILE_NAME, 'must be one safe filename')).min(1).optional(),
    source: z.union([
      z.string().regex(PREFIXED_PATH, 'must be a safe repository-root-relative path'),
      pinnedSourceSchema,
    ]),
    reason: z.string().min(1),
  })
  .strict()
  .superRefine((value, context) => {
    if (value.files !== undefined && new Set(value.files).size !== value.files.length) {
      context.addIssue({ code: 'custom', path: ['files'], message: 'must not contain duplicate filenames' });
    }
  });

const packageEntrySchema = z
  .object({
    kind: z.enum(packageKinds),
    type: z.enum(['cjs', 'esm', 'dual']),
    browser: z.boolean(),
    name: z.string().min(1).optional(),
    member: z.boolean(),
    memberOf: z.string().regex(PACKAGE_KEY, 'must be a canonical sdk-relative package key').optional(),
    private: z.boolean().optional(),
    publishedRelPath: z.string().regex(PREFIXED_PATH, 'must be a safe path with a leading ./').optional(),
    dependencyGroup: z.string().regex(UNPREFIXED_PATH, 'must be a safe relative path').optional(),
    dependencyExceptions: z.array(z.string().min(1)).min(1).superRefine(uniqueStrings).optional(),
    consumerTests: consumerTestsSchema.optional(),
    distribution: distributionSchema.optional(),
    mirror: mirrorSchema.optional(),
    vendored: z.array(vendoredElementSchema).min(1).optional(),
    note: z.string().min(1).optional(),
  })
  .strict()
  .superRefine((entry, context) => {
    // `member` means "member of SOME installation root"; `memberOf` names which (default: '.', the sdk
    // root workspace). A cluster root (a non-'.' workspace-root entry) is itself never a member.
    if (entry.memberOf !== undefined && !entry.member) {
      issue(context, ['memberOf'], 'only a workspace member can declare which installation root it belongs to');
    }
    if (entry.memberOf === '.') {
      issue(context, ['memberOf'], "'.' is the default installation root — omit memberOf");
    }

    if (entry.vendored !== undefined) {
      const serialized = entry.vendored.map((element) => JSON.stringify(element));
      if (new Set(serialized).size !== serialized.length) {
        issue(context, ['vendored'], 'must not contain duplicate elements');
      }
    }

    if (entry.kind === 'published') {
      requireName(entry, context);
      if (entry.private !== undefined) issue(context, ['private'], "a published package must omit 'private'");
    } else if (entry.consumerTests !== undefined) {
      issue(context, ['consumerTests'], 'only a published package can select its consumer tests');
    }

    if (entry.consumerTests !== undefined && entry.distribution?.includes('npm') === false) {
      issue(context, ['consumerTests'], 'consumer test overrides apply only to npm-distributed packages');
    }
    if (entry.consumerTests?.cjs !== undefined && entry.type === 'esm') {
      issue(context, ['consumerTests', 'cjs'], 'an ESM-only package does not require a CJS consumer');
    }
    if (entry.consumerTests?.esm !== undefined && entry.type === 'cjs') {
      issue(context, ['consumerTests', 'esm'], 'a CJS-only package does not require an ESM consumer');
    }

    // `distribution` describes a payload, and the 'mirror' channel and the `mirror` block are two halves
    // of one fact, so neither may appear without the other.
    if (entry.distribution !== undefined && entry.kind !== 'published') {
      issue(context, ['distribution'], 'only a published payload has distribution channels');
    }
    if (entry.distribution?.includes('mirror') === true && entry.mirror === undefined) {
      issue(context, ['mirror'], "the 'mirror' channel requires a mirror repository");
    }
    if (entry.mirror !== undefined && entry.distribution?.includes('mirror') === false) {
      issue(context, ['distribution'], "a mirrored package must list the 'mirror' channel");
    }

    if (entry.kind === 'dev' || entry.kind === 'shared-helper' || entry.kind === 'internal-consumer') {
      requireName(entry, context);
      if (entry.private !== true) issue(context, ['private'], `${entry.kind} must set private: true`);
      if (entry.member !== true) issue(context, ['member'], `${entry.kind} must be a workspace member`);
      if (entry.name !== undefined && !entry.name.endsWith('-dev')) {
        issue(context, ['name'], `${entry.kind} names must end in -dev`);
      }
    }

    if (entry.kind === 'dev') {
      if (entry.publishedRelPath === undefined) {
        issue(context, ['publishedRelPath'], 'a dev package must identify its published payload');
      } else if (!entry.publishedRelPath.endsWith('/pkg')) {
        issue(context, ['publishedRelPath'], 'a dev package payload path must end in /pkg');
      }
    } else if (entry.publishedRelPath !== undefined) {
      issue(context, ['publishedRelPath'], 'only a dev package can own a published payload');
    }

    if (entry.kind === 'standalone') {
      requireName(entry, context);
      if (entry.member) issue(context, ['member'], 'a standalone project cannot be a workspace member');
    }

    if (entry.kind === 'non-package') {
      if (entry.member) issue(context, ['member'], 'a non-package cannot be a workspace member');
      forbid(entry.name, context, 'name', 'a non-package has no package name');
      forbid(entry.dependencyGroup, context, 'dependencyGroup', 'a non-package has no dependency group');
      forbid(entry.dependencyExceptions, context, 'dependencyExceptions', 'a non-package has no dependencies');
      forbid(entry.consumerTests, context, 'consumerTests', 'a non-package cannot select consumer tests');
      forbid(entry.mirror, context, 'mirror', 'a non-package cannot be mirrored');
      forbid(entry.vendored, context, 'vendored', 'a non-package cannot own vendored content');
    }

    if (entry.kind === 'workspace-root') {
      requireName(entry, context);
      if (entry.private !== true) issue(context, ['private'], 'a workspace root must set private: true');
      if (entry.member) issue(context, ['member'], 'a workspace root is an installation root, never a member');
      forbid(entry.dependencyGroup, context, 'dependencyGroup', 'a workspace root has no dependency group');
      forbid(entry.dependencyExceptions, context, 'dependencyExceptions', 'a workspace root cannot carry exceptions');
      forbid(entry.consumerTests, context, 'consumerTests', 'a workspace root cannot select consumer tests');
      forbid(entry.mirror, context, 'mirror', 'a workspace root cannot be mirrored');
      forbid(entry.vendored, context, 'vendored', 'a workspace root cannot own vendored content');
    }
  });

const npmManifestSchema = z
  .object({
    $schema: z.literal('./npm-manifest.schema.json').optional(),
    inventory: inventorySchema.optional(),
    dependencies: dependencyPolicySchema.optional(),
    foundry: foundryPolicySchema.optional(),
    tarballs: tarballsPolicySchema.optional(),
    packageJson: packageJsonPolicySchema,
    packages: z.record(
      z.string().regex(PACKAGE_KEY, 'must be a canonical sdk-relative package key'),
      packageEntrySchema,
    ),
  })
  .strict()
  .superRefine((manifest, context) => {
    const root = manifest.packages['.'];
    if (root === undefined) {
      issue(context, ['packages', '.'], 'the workspace-root entry is required');
    } else if (root.kind !== 'workspace-root') {
      issue(context, ['packages', '.', 'kind'], 'the . entry must have kind workspace-root');
    }

    for (const [key, entry] of Object.entries(manifest.packages)) {
      // A non-'.' workspace-root is an installation-root CLUSTER (its own npm workspace: own lock, own
      // hoisting — one hardhat major per root). Members point at it with memberOf.
      if (entry.memberOf !== undefined && entry.memberOf !== '.') {
        const root = manifest.packages[entry.memberOf];
        if (root === undefined || root.kind !== 'workspace-root') {
          issue(context, ['packages', key, 'memberOf'], `'${entry.memberOf}' is not a workspace-root entry`);
        } else if (!key.startsWith(`${entry.memberOf}/`)) {
          issue(context, ['packages', key, 'memberOf'], 'a member must live inside its installation root');
        }
      }
      for (const exception of entry.dependencyExceptions ?? []) {
        if (!(manifest.dependencies?.forbidden ?? []).includes(exception)) {
          issue(
            context,
            ['packages', key, 'dependencyExceptions'],
            `'${exception}' is not listed in dependencies.forbidden`,
          );
        }
      }
      for (const [moduleKind, consumerKey] of Object.entries(entry.consumerTests ?? {})) {
        const consumer = manifest.packages[consumerKey];
        if (consumer === undefined) {
          issue(context, ['packages', key, 'consumerTests', moduleKind], `'${consumerKey}' is not a manifest package`);
          continue;
        }
        if (consumerKey === key) {
          issue(context, ['packages', key, 'consumerTests', moduleKind], 'a package cannot consume itself');
        }
        if (moduleKind === 'cjs' && consumer.type === 'esm') {
          issue(context, ['packages', key, 'consumerTests', moduleKind], `'${consumerKey}' does not support CJS`);
        }
        if (moduleKind === 'esm' && consumer.type === 'cjs') {
          issue(context, ['packages', key, 'consumerTests', moduleKind], `'${consumerKey}' does not support ESM`);
        }
      }
    }
  });

export type PackageKind = (typeof packageKinds)[number];
export type NpmManifestEntry = z.infer<typeof packageEntrySchema>;
export type NpmManifest = z.infer<typeof npmManifestSchema>;

export class ManifestValidationError extends Error {
  readonly file: string;

  constructor(file: string, message: string) {
    super(`${file}: ${message}`);
    this.name = 'ManifestValidationError';
    this.file = file;
  }
}

export function parseNpmManifest(value: unknown): NpmManifest {
  const result = npmManifestSchema.safeParse(value);
  if (!result.success) throw new ManifestValidationError('npm-manifest.json', z.prettifyError(result.error));
  return result.data;
}

export function loadNpmManifest(file: string): NpmManifest {
  let value: unknown;
  try {
    value = JSON.parse(readFileSync(file, 'utf8')) as unknown;
  } catch (error) {
    throw new ManifestValidationError(file, `unable to parse JSON: ${errorMessage(error)}`);
  }

  try {
    return parseNpmManifest(value);
  } catch (error) {
    if (error instanceof ManifestValidationError) {
      throw new ManifestValidationError(file, error.message.replace(/^npm-manifest\.json:\s*/, ''));
    }
    throw error;
  }
}

function requireName(entry: { readonly name?: string }, context: z.RefinementCtx): void {
  if (entry.name === undefined) issue(context, ['name'], 'a package name is required for this kind');
}

function forbid(value: unknown, context: z.RefinementCtx, field: string, message: string): void {
  if (value !== undefined) issue(context, [field], message);
}

function issue(context: z.RefinementCtx, path: readonly PropertyKey[], message: string): void {
  context.addIssue({ code: 'custom', path: [...path], message });
}

function uniqueStrings(values: readonly string[], context: z.RefinementCtx): void {
  if (new Set(values).size !== values.length) {
    context.addIssue({ code: 'custom', message: 'must not contain duplicate values' });
  }
}

function errorMessage(error: unknown): string {
  return error instanceof Error ? error.message : String(error);
}
