// The publication side of RELEASE_PLAN.md, read-only half. A published payload carries `file:` links to
// other payloads (rule 3.1.1); npmjs.com cannot resolve a path, so `publish render` shows the manifest a
// consumer will see — each link replaced by the generation range of the target's central version — and
// `publish order` derives, from those same links, the order payloads must reach the registry in. Nothing
// here writes; `publish pack` calls renderPackageJson so the tarball cannot differ from what render showed.

import { resolve } from 'node:path';

import type { NpmManifest } from '../manifest.ts';
import { npmPackedFiles } from './checks/published-files.ts';
import { type DependencyDeclaration, type LoadedPackage, dependencyDeclarations, loadPackages } from './npm.ts';
import { distributionChannels } from './package-versions.ts';
import { generationRange, parseVersion } from './semver.ts';
import { type VersionsFile, loadVersions } from './versions.ts';

export type PayloadGraph = {
  readonly nodes: readonly string[];
  /** source key → keys of the payloads it links. */
  readonly edges: ReadonlyMap<string, readonly string[]>;
};

export type Replacement = { readonly field: string; readonly name: string; readonly from: string; readonly to: string };

export type RenderedPackageJson = {
  readonly packageJson: Record<string, unknown>;
  readonly replacements: readonly Replacement[];
};

export function isNpmDistributedPayload(pkg: LoadedPackage): boolean {
  return pkg.inventory.kind === 'published' && distributionChannels(pkg.inventory).includes('npm');
}

/** Nodes are the npm-distributed payloads; an edge is a `file:` dependency that resolves to another payload. */
export function payloadGraph(packages: readonly LoadedPackage[]): PayloadGraph {
  const payloads = packages.filter(isNpmDistributedPayload);
  const edges = new Map(
    payloads.map((pkg) => [
      pkg.key,
      fileDeclarations(pkg)
        .map((declaration) => linkedPayload(pkg, declaration, packages))
        .filter((target): target is LoadedPackage => target !== undefined)
        .map((target) => target.key),
    ]),
  );
  return { nodes: payloads.map((pkg) => pkg.key).sort(), edges };
}

/** Dependencies first, ties broken by key; a cycle is an error because no order can publish it. */
export function topologicalOrder(graph: PayloadGraph): readonly string[] {
  const order: string[] = [];
  const remaining = new Set(graph.nodes);
  while (remaining.size > 0) {
    const ready = [...remaining].filter((node) => (graph.edges.get(node) ?? []).every((dep) => !remaining.has(dep)));
    if (ready.length === 0) throw new Error(`publish order: dependency cycle among ${[...remaining].join(', ')}`);
    for (const node of ready.sort()) {
      order.push(node);
      remaining.delete(node);
    }
  }
  return order;
}

/** The payload's package.json as npmjs.com must see it: every `file:` link becomes the target's generation range. */
export function renderPackageJson(
  pkg: LoadedPackage,
  packages: readonly LoadedPackage[],
  versions: VersionsFile,
): RenderedPackageJson {
  if (!isNpmDistributedPayload(pkg)) throw new Error(`publish render: ${pkg.key} is not an npm-distributed payload`);
  const rendered = structuredClone(pkg.packageJson) as Record<string, Record<string, string>>;
  const replacements = fileDeclarations(pkg).map((declaration) => {
    const to = renderedSpec(pkg, declaration, packages, versions);
    rendered[declaration.field] = { ...rendered[declaration.field], [declaration.name]: to };
    return { field: declaration.field, name: declaration.name, from: declaration.spec, to };
  });
  return { packageJson: rendered, replacements };
}

/** The lines a reader needs: a unified-style diff of the replaced specs, or the fact that there were none. */
export function formatRenderedDiff(pkg: LoadedPackage, rendered: RenderedPackageJson): string {
  if (rendered.replacements.length === 0) return 'nothing to render: package.json ships as-is';
  return [
    `--- ${pkg.key}/package.json`,
    '+++ rendered',
    ...rendered.replacements.flatMap((r) => [`-    "${r.name}": "${r.from}",`, `+    "${r.name}": "${r.to}",`]),
  ].join('\n');
}

export function formatPackedFiles(files: readonly string[]): string {
  return [`files (npm pack --dry-run): ${String(files.length)}`, ...files.map((file) => `  ${file}`)].join('\n');
}

/** A payload by its manifest key, its owner's key, or either without the leading './'. */
export function resolvePayload(packages: readonly LoadedPackage[], selector: string): LoadedPackage {
  const key = selector.startsWith('./') ? selector : `./${selector.replace(/^\/+/, '')}`;
  const byPayload = packages.find((pkg) => pkg.key === key);
  const byOwner = packages.find((pkg) => pkg.key === key && pkg.inventory.kind === 'dev')?.inventory.publishedRelPath;
  const pkg = byPayload?.inventory.kind === 'published' ? byPayload : packages.find((p) => p.key === byOwner);
  if (pkg === undefined || !isNpmDistributedPayload(pkg)) {
    const candidates = packages
      .filter(isNpmDistributedPayload)
      .map((p) => p.key)
      .sort();
    throw new Error(`No npm-distributed payload matches '${selector}'. Payloads: ${candidates.join(', ')}`);
  }
  return pkg;
}

/** The two read-only commands over the real tree. */
export function publishOrder(workspaceRoot: string, manifest: NpmManifest): readonly string[] {
  return topologicalOrder(payloadGraph(loadPackages(workspaceRoot, manifest)));
}

export function publishRender(
  workspaceRoot: string,
  manifest: NpmManifest,
  selector: string,
  packedFiles: (directory: string) => readonly string[] = npmPackedFiles,
): { readonly pkg: LoadedPackage; readonly rendered: RenderedPackageJson; readonly files: readonly string[] } {
  const packages = loadPackages(workspaceRoot, manifest);
  const pkg = resolvePayload(packages, selector);
  return {
    pkg,
    rendered: renderPackageJson(pkg, packages, loadVersions(workspaceRoot)),
    files: packedFiles(pkg.directory),
  };
}

function fileDeclarations(pkg: LoadedPackage): readonly DependencyDeclaration[] {
  return dependencyDeclarations(pkg.packageJson).filter((declaration) => declaration.spec.startsWith('file:'));
}

// The manifest package a `file:` spec points at, by resolved directory; undefined when it names none.
function linkedPayload(
  pkg: LoadedPackage,
  declaration: DependencyDeclaration,
  packages: readonly LoadedPackage[],
): LoadedPackage | undefined {
  const target = resolve(pkg.directory, declaration.spec.slice('file:'.length));
  return packages.find((candidate) => resolve(candidate.directory) === target);
}

// Decision 1: `^0.<generation>.0` of the target's central version; a link to anything unpublishable cannot render.
function renderedSpec(
  pkg: LoadedPackage,
  declaration: DependencyDeclaration,
  packages: readonly LoadedPackage[],
  versions: VersionsFile,
): string {
  const target = linkedPayload(pkg, declaration, packages);
  if (target === undefined || !isNpmDistributedPayload(target)) {
    throw new Error(
      `publish render: ${pkg.key} links '${declaration.name}' (${declaration.spec}), which is not an npm-published payload`,
    );
  }
  const central = versions.packages[target.key];
  const parsed = central === undefined ? undefined : parseVersion(central);
  if (parsed === undefined)
    throw new Error(`publish render: ${target.key} has no canonical central version in versions.json`);
  return generationRange(parsed);
}
