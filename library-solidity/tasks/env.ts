// The library-solidity .env file (see .env.example): host deployment parameters, also read by the tests.
import dotenv from 'dotenv';
import fs from 'node:fs';
import path from 'node:path';

const ENV_FILE = path.resolve(import.meta.dirname, '../.env');

/** Loads library-solidity/.env into process.env (variables already set in the environment take precedence). */
export function loadEnv(): void {
  if (!fs.existsSync(ENV_FILE)) {
    throw new Error(`${ENV_FILE} does not exist: copy .env.example to .env`);
  }
  dotenv.config({ path: ENV_FILE, quiet: true });
}

/** Returns the `name` env variable, throwing if it is not set or empty. */
export function getRequiredEnvVar(name: string): string {
  const value = process.env[name];
  if (value === undefined || value.trim() === '') {
    throw new Error(`"${name}" env variable is not set or empty (see .env.example)`);
  }
  return value;
}

/** Returns the `name` env variable as an integer, or `defaultValue` (if given) when it is not set or empty. */
export function getIntegerEnvVar(name: string, defaultValue?: number): number {
  const value = process.env[name];
  if ((value === undefined || value.trim() === '') && defaultValue !== undefined) {
    return defaultValue;
  }
  const integer = Number(getRequiredEnvVar(name));
  if (!Number.isInteger(integer)) {
    throw new Error(`"${name}" env variable must be an integer, got "${value ?? ''}"`);
  }
  return integer;
}

/**
 * Returns the `name` env variable as a strictly positive integer. Throws if it is missing, empty, non-integer or not
 * strictly positive, so a malformed count fails loudly instead of silently deploying or registering nothing.
 */
export function getRequiredCountEnvVar(name: string): number {
  const value = getRequiredEnvVar(name);
  const count = Number(value);
  if (!Number.isInteger(count) || count <= 0) {
    throw new Error(`"${name}" env variable must be a positive integer, got "${value}"`);
  }
  return count;
}
