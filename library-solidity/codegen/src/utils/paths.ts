import { existsSync, readdirSync, statSync } from 'fs';
import * as path from 'path';

export function isDirectory(p: string): boolean {
  if (!existsSync(p)) {
    return false;
  }
  const stats = statSync(p);
  return stats.isDirectory();
}

export function assertDirectoryExists(p: string) {
  if (!existsSync(p)) {
    throw new Error(`Path '${p}' directory does not exist`);
  }
  const stats = statSync(p);
  if (!stats.isDirectory()) {
    throw new Error(`Path '${p}' is not a directory`);
  }
}

export function assertRelative(p: string, name?: string) {
  if (path.isAbsolute(p)) {
    if (name) {
      throw new Error(`Invalid ${name} path: ${p}. Expecting a relative path, got an absolute path instead.`);
    } else {
      throw new Error(`Invalid path: ${p}. Expecting a relative path, got an absolute path instead.`);
    }
  }
}

export function assertAbsolute(p: string, name?: string) {
  if (!path.isAbsolute(p)) {
    if (name) {
      throw new Error(`Invalid ${name} path: ${p}. Expecting an absolute path, got a relative path instead.`);
    } else {
      throw new Error(`Invalid path: ${p}. Expecting an absolute path, got a relative path instead.`);
    }
  }
}

export function fromFileToFile(fromFile: string, toFile: string) {
  return fromDirToFile(path.dirname(fromFile), toFile);
}

export function fromDirToFile(fromDir: string, toFile: string) {
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

export function fromDirToDir(fromDir: string, toDir: string) {
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
