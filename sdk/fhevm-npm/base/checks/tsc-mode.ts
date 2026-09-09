// Detecting `tsc` invocations that drive a solution-style tsconfig (empty `files` plus `references`)
// in project mode: `tsc -p` and bare `tsc` load such a config, check zero files, and exit 0, so the
// script passes without type-checking anything. Only build mode (`tsc -b`) walks the references.

import { existsSync, readFileSync, statSync } from 'node:fs';
import { posix, relative, resolve, sep } from 'node:path';
import { flattenDiagnosticMessageText, parseConfigFileTextToJson } from 'typescript';

import type { NpmManifest } from '../../manifest.ts';
import type { Violation } from '../diagnostics.ts';
import { loadPackages } from '../npm.ts';

const RULE = '2.1.13';

type TscInvocation = {
  readonly script: string;
  readonly text: string;
  readonly mode: 'build' | 'project' | 'bare';
  /** As written in the script; undefined in build mode, where any target shape is valid. */
  readonly configPath?: string;
};

type TsConfig = {
  readonly files?: unknown;
  readonly include?: unknown;
  readonly references?: unknown;
};

export type TscModeInspection = {
  readonly checkedInvocationKeys: readonly string[];
  readonly successfulInvocations: readonly string[];
  readonly violations: readonly Violation[];
};

export function inspectTscMode(workspaceRoot: string, manifest: NpmManifest): TscModeInspection {
  const checkedInvocationKeys: string[] = [];
  const successfulInvocations: string[] = [];
  const violations: Violation[] = [];

  for (const pkg of loadPackages(workspaceRoot, manifest)) {
    for (const [script, command] of Object.entries(pkg.packageJson.scripts ?? {})) {
      for (const invocation of tscInvocations(script, command)) {
        checkedInvocationKeys.push(`${pkg.key} [${script}] ${invocation.text}`);
        const violation = validateInvocation(pkg.key, pkg.directory, invocation);
        if (violation === undefined) {
          successfulInvocations.push(`${pkg.key} [${script}] ${invocation.text} (${invocation.mode} mode)`);
        } else {
          violations.push(violation);
        }
      }
    }
  }

  if (checkedInvocationKeys.length === 0) {
    violations.push({
      rule: RULE,
      packageKey: '.',
      message: 'found no tsc invocation to inspect; refusing to pass vacuously',
    });
  }

  return { checkedInvocationKeys, successfulInvocations, violations };
}

function validateInvocation(packageKey: string, directory: string, invocation: TscInvocation): Violation | undefined {
  if (invocation.mode === 'build' || invocation.configPath === undefined) return undefined;

  const config = resolveTsconfigFile(directory, invocation.configPath);
  // A missing config makes tsc itself fail loudly, and 2.1.5 owns path existence; nothing silent here.
  if (config === undefined) return undefined;

  const parsed = parseConfigFileTextToJson(config, readFileSync(config, 'utf8'));
  if (parsed.error !== undefined) {
    return {
      rule: RULE,
      packageKey,
      message: `'${invocation.script}' targets '${invocation.configPath}', which could not be parsed: ${flattenDiagnosticMessageText(parsed.error.messageText, '\n')}`,
    };
  }
  if (!isSolutionStyle(parsed.config as TsConfig)) return undefined;

  const how = invocation.mode === 'bare' ? `resolves to '${pathKey(directory, config)}'` : 'is in project mode';
  return {
    rule: RULE,
    packageKey,
    message:
      `'${invocation.script}' runs '${invocation.text}', which ${how}; that tsconfig is solution-style ` +
      `(empty 'files' plus 'references'), so project mode checks zero files and exits 0 — use 'tsc -b'`,
  };
}

/** Every `tsc` segment of a script command, split on the shell operators npm scripts actually use. */
function tscInvocations(script: string, command: string): readonly TscInvocation[] {
  const invocations: TscInvocation[] = [];
  for (const segment of command.split(/&&|\|\||;/)) {
    const tokens = segment.trim().split(/\s+/);
    const tscIndex = tokens.indexOf('tsc');
    if (tscIndex === -1) continue;
    const invocation = parseTscInvocation(script, tokens.slice(0, tscIndex + 1).join(' '), tokens.slice(tscIndex + 1));
    if (invocation !== undefined) invocations.push(invocation);
  }
  return invocations;
}

function parseTscInvocation(script: string, prefix: string, args: readonly string[]): TscInvocation | undefined {
  const text = [prefix, ...args].join(' ');
  if (args.includes('-b') || args.includes('--build')) return { script, text, mode: 'build' };

  const projectIndex = args.findIndex((arg) => arg === '-p' || arg === '--project');
  if (projectIndex !== -1) {
    const configPath = args[projectIndex + 1];
    // A missing or flag-shaped value is a malformed invocation tsc rejects loudly on its own.
    if (configPath === undefined || configPath.startsWith('-')) return undefined;
    return { script, text, mode: 'project', configPath };
  }

  // Bare `tsc` uses ./tsconfig.json only when given no input files. A positional token is either an
  // input file or a value of a flag such as `--outDir`; both mean the local tsconfig is not what runs,
  // so this stays conservative and skips.
  if (args.some((arg) => !arg.startsWith('-'))) return undefined;
  return { script, text, mode: 'bare', configPath: './tsconfig.json' };
}

/** The file a target names: the path itself, or the `tsconfig.json` inside a directory target. */
function resolveTsconfigFile(directory: string, configPath: string): string | undefined {
  const target = resolve(directory, configPath);
  if (!existsSync(target)) return undefined;
  if (statSync(target).isDirectory()) {
    const inside = resolve(target, 'tsconfig.json');
    return existsSync(inside) ? inside : undefined;
  }
  return target;
}

function isSolutionStyle(config: TsConfig): boolean {
  if (!Array.isArray(config.files) || config.files.length > 0) return false;
  if (!Array.isArray(config.references) || config.references.length === 0) return false;
  return config.include === undefined || (Array.isArray(config.include) && config.include.length === 0);
}

function pathKey(directory: string, file: string): string {
  return `./${relative(directory, file).split(sep).join(posix.sep)}`;
}
