// Rule 5.2.6: every file in an npm-distributed payload directory is published or deliberately excluded.
// Rule 5.2.7: every npm-distributed payload ships LICENSE and README.md, on disk and listed in `files`.
//
// `files` in package.json is a whitelist, so a file that lives next to it without being selected is dead
// weight nobody ships and nobody reviews — the failure mode is a stray note or fixture committed under
// `pkg/` and forgotten. npm itself is the judge of what ships (`npm pack --dry-run`), and the developer's
// `!` patterns in `files` are the one place a deliberately unpublished file is declared. LICENSE and README
// are the two files npmjs.com renders for every package; npm packs them whatever `files` says, so the
// listing is not for npm's benefit but the developer's: the whitelist reads as the complete manifest.

import { execFileSync } from 'node:child_process';
import { matchesGlob } from 'node:path';

import type { NpmManifest } from '../../manifest.ts';
import type { Violation } from '../diagnostics.ts';
import { type LoadedPackage, loadPackages } from '../npm.ts';
import { gitVisibleFiles } from '../repository.ts';

export const RULE = '5.2.6';
export const REQUIRED_FILES_RULE = '5.2.7';

/** What npmjs.com shows on a package page: the license text and the front-page README. */
export const REQUIRED_PAYLOAD_FILES: readonly string[] = ['LICENSE', 'README.md'];

export type PublishedFilesInspection = {
  readonly checkedPackageKeys: readonly string[];
  readonly violations: readonly Violation[];
};

/** Injection points so the unit tests need neither git nor npm. */
export type PublishedFilesProbes = {
  readonly visibleFiles: (directory: string) => readonly string[];
  readonly packedFiles: (directory: string) => readonly string[];
};

const defaultProbes: PublishedFilesProbes = { visibleFiles: gitVisibleFiles, packedFiles: npmPackedFiles };

export function inspectPublishedFiles(
  workspaceRoot: string,
  manifest: NpmManifest,
  probes: PublishedFilesProbes = defaultProbes,
): PublishedFilesInspection {
  const payloads = loadPackages(workspaceRoot, manifest).filter(isNpmDistributedPayload);
  return {
    checkedPackageKeys: payloads.map((pkg) => pkg.key),
    violations: payloads.flatMap((pkg) => [
      ...validatePublishedFiles(pkg, probes),
      ...validateRequiredPayloadFiles(pkg, probes),
    ]),
  };
}

/** A mirror-only payload's directory IS the mirror, so its unpublished files are the point, not a defect. */
function isNpmDistributedPayload(pkg: LoadedPackage): boolean {
  return pkg.inventory.kind === 'published' && (pkg.inventory.distribution ?? ['npm']).includes('npm');
}

export function validatePublishedFiles(pkg: LoadedPackage, probes: PublishedFilesProbes): readonly Violation[] {
  const files = filesField(pkg);
  if (files === undefined) {
    return [
      violation(pkg, 'published package.json must declare "files": without the whitelist, every stray file ships'),
    ];
  }
  const negations = files.filter((entry) => entry.startsWith('!')).map((entry) => entry.slice(1));
  const packed = new Set(probes.packedFiles(pkg.directory));
  return probes
    .visibleFiles(pkg.directory)
    .filter((file) => !packed.has(file) && !negations.some((pattern) => matchesNegation(file, pattern)))
    .sort()
    .map((file) =>
      violation(
        pkg,
        `'${file}' is neither published (not selected by "files") nor excluded by a "!" pattern in "files"; delete it or declare it`,
      ),
    );
}

/** Rule 5.2.7: LICENSE and README.md exist in the payload directory and are named in `files`. */
export function validateRequiredPayloadFiles(pkg: LoadedPackage, probes: PublishedFilesProbes): readonly Violation[] {
  const visible = new Set(probes.visibleFiles(pkg.directory));
  const files = filesField(pkg);
  return REQUIRED_PAYLOAD_FILES.flatMap((required) => [
    ...(visible.has(required)
      ? []
      : [requiredFileViolation(pkg, `'${required}' is missing from the payload directory`)]),
    // Without `files` there is no list to be in; 5.2.6 already reports the missing whitelist itself.
    ...(files === undefined || files.includes(required)
      ? []
      : [requiredFileViolation(pkg, `'${required}' is not listed in "files"; the whitelist must name what ships`)]),
  ]);
}

/** `files` if it is an array of strings, else undefined — the schema passes the field through untyped. */
function filesField(pkg: LoadedPackage): readonly string[] | undefined {
  const value = (pkg.packageJson as Record<string, unknown>)['files'];
  return Array.isArray(value) && value.every((entry) => typeof entry === 'string') ? value : undefined;
}

/** npm's `!` patterns match at any depth when they name no directory, like .gitignore lines do. */
function matchesNegation(file: string, pattern: string): boolean {
  return matchesGlob(file, pattern) || (!pattern.includes('/') && matchesGlob(file, `**/${pattern}`));
}

/** What npm would put in the tarball, from npm's own dry run; scripts are skipped so nothing is built. */
export function npmPackedFiles(directory: string): readonly string[] {
  const stdout = execFileSync('npm', ['pack', '--dry-run', '--json', '--ignore-scripts'], {
    cwd: directory,
    encoding: 'utf8',
    stdio: ['ignore', 'pipe', 'ignore'],
  });
  const parsed = JSON.parse(stdout) as unknown;
  const first = Array.isArray(parsed) ? (parsed[0] as { files?: { path?: unknown }[] } | undefined) : undefined;
  if (first?.files === undefined) throw new Error(`Unexpected npm pack output in ${directory}: ${stdout}`);
  return first.files.map((entry) => entry.path).filter((path): path is string => typeof path === 'string');
}

function violation(pkg: LoadedPackage, message: string): Violation {
  return { rule: RULE, packageKey: pkg.key, message };
}

function requiredFileViolation(pkg: LoadedPackage, message: string): Violation {
  return { rule: REQUIRED_FILES_RULE, packageKey: pkg.key, message };
}
