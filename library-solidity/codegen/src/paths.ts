import { existsSync, readdirSync, statSync } from 'fs';
import * as path from 'path';

export function isDirectory(p: string): boolean {
  if (!existsSync(p)) {
    return false;
  }
  const stats = statSync(p);
  return stats.isDirectory();
}

export function assertDirectoryExists(p: string): void {
  if (!existsSync(p)) {
    throw new Error(`Path '${p}' directory does not exist`);
  }
  const stats = statSync(p);
  if (!stats.isDirectory()) {
    throw new Error(`Path '${p}' is not a directory`);
  }
}

// "<name> path" when a name is given, "path" otherwise: used in the error messages below.
function pathLabel(name: string | undefined): string {
  return name !== undefined && name !== '' ? `${name} path` : 'path';
}

export function assertRelative(p: string, name?: string): void {
  if (path.isAbsolute(p)) {
    throw new Error(`Invalid ${pathLabel(name)}: ${p}. Expecting a relative path, got an absolute path instead.`);
  }
}

export function assertAbsolute(p: string, name?: string): void {
  if (!path.isAbsolute(p)) {
    throw new Error(`Invalid ${pathLabel(name)}: ${p}. Expecting an absolute path, got a relative path instead.`);
  }
}

export function fromFileToFile(fromFile: string, toFile: string): string {
  return fromDirToFile(path.dirname(fromFile), toFile);
}

export function fromDirToFile(fromDir: string, toFile: string): string {
  let rel = path.relative(fromDir, path.dirname(toFile));
  if (rel.trim().length === 0) {
    rel = '.';
  }
  let relFile = path.join(rel, path.basename(toFile));
  if (!relFile.startsWith('.')) {
    relFile = './' + relFile;
  }
  return relFile;
}

export function fromDirToDir(fromDir: string, toDir: string): string {
  let rel = path.relative(fromDir, toDir);
  if (rel.trim().length === 0) {
    rel = '.';
  }
  return rel;
}

export function isDirectoryEmpty(directoryPath: string): boolean {
  const contents = readdirSync(directoryPath);
  return contents.length === 0;
}

export function resolveTemplatePath(templateFilename: string): string {
  // The compiled module's directory should be /path/to/fhevm/library-solidity/codegen/dist/_esm
  // Throw an error in case dist naming is changed or paths.ts is moved somewhere elst
  // We need to perform the resolve manually
  const moduleDir = import.meta.dirname;
  if (!moduleDir.endsWith('library-solidity/codegen/dist/_esm')) {
    throw new Error(`Unexpected module directory ${moduleDir}. Should end with "library-solidity/codegen/dist/_esm"`);
  }

  return path.normalize(path.join(moduleDir, '../../src/templates', templateFilename));
}
