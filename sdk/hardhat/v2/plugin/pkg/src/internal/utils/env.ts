import setupDebug from 'debug';
import * as dotenv from 'dotenv';
import * as fs from 'fs';
import * as path from 'path';
import * as picocolors from 'picocolors';

import { HardhatFhevmError } from '../../error';

const debug = setupDebug('@fhevm/hardhat:env');

// eslint-disable-next-line @typescript-eslint/naming-convention
function __logDefaultValue(name: string, defaultValue: unknown): void {
  debug(`Resolve ${picocolors.magentaBright(name)}=${defaultValue}, using default value.`);
}
// eslint-disable-next-line @typescript-eslint/naming-convention
function __logEnvValue(name: string, value: string): void {
  debug(`Resolve ${picocolors.yellowBright(name)}=${value}, using env variable ${name}.`);
}
// eslint-disable-next-line @typescript-eslint/naming-convention
function __logDotEnvValue(name: string, value: string, dotEnvFile: string): void {
  debug(`Resolve ${picocolors.greenBright(name)}=${value}, using .env variable stored at ${path.resolve(dotEnvFile)}`);
}

export function getEnvUint({
  name,
  defaultValue,
  dotEnvFile,
}: {
  name: string;
  defaultValue?: number;
  dotEnvFile?: string | undefined;
}): number {
  let int: number;

  try {
    const str = getEnvString({ name, dotEnvFile });
    int = parseInt(str);
  } catch {
    int = Number.NaN;
  }

  if (!Number.isNaN(int)) {
    return int;
  }

  if (defaultValue !== undefined) {
    __logDefaultValue(name, defaultValue);
    return defaultValue;
  }

  throw new HardhatFhevmError(`Unable to determine integer constant ${name}`);
}

export function getEnvString({
  name,
  defaultValue,
  dotEnvFile,
}: {
  name: string;
  defaultValue?: string;
  dotEnvFile?: string | undefined;
}): string {
  if (dotEnvFile !== undefined && fs.existsSync(dotEnvFile)) {
    const parsedEnv = dotenv.parse(fs.readFileSync(dotEnvFile));
    const addr = parsedEnv[name];
    // `FOO=` parses to '', which counts as unset so the lookup falls through to process.env.
    if (addr !== undefined && addr !== '') {
      __logDotEnvValue(name, addr, dotEnvFile);
      return addr;
    }
  }

  if (name in process.env && process.env[name] !== undefined) {
    const addr = process.env[name];
    __logEnvValue(name, addr);
    return addr;
  }

  if (defaultValue !== undefined) {
    __logDefaultValue(name, defaultValue);
    return defaultValue;
  }

  throw new HardhatFhevmError(`Environment variable ${name} is undefined`);
}

export function getOptionalEnvString(params: {
  name: string;
  defaultValue?: string;
  dotEnvFile?: string | undefined;
}): string | undefined {
  try {
    return getEnvString(params);
  } catch {
    return undefined;
  }
}

export function getOptionalEnvUint(params: {
  name: string;
  defaultValue?: number;
  dotEnvFile?: string | undefined;
}): number | undefined {
  try {
    return getEnvUint(params);
  } catch {
    return undefined;
  }
}
