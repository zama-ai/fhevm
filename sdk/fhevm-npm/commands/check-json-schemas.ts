import { existsSync, readFileSync, readdirSync } from 'node:fs';
import { dirname, isAbsolute, join, relative, resolve, sep } from 'node:path';

import { Ajv2020, type ErrorObject, type ValidateFunction } from 'ajv/dist/2020.js';

import type { CheckCommand } from '../base/command.ts';
import type { Violation } from '../base/diagnostics.ts';

const SCHEMA_DIRECTORY = join('fhevm-npm', 'schemas');
const REQUIRED_JSON_FILES = [
  'npm-manifest.json',
  'cleartext-config.json',
  'fhevm-chains.config.json',
  'fhevm-network-groups.config.json',
] as const;

type JsonObject = Readonly<Record<string, unknown>>;

/** Validate the committed JSON configuration surface against its editor-visible JSON Schemas. */
export const checkJsonSchemas: CheckCommand = (context) => {
  const schemaDirectory = join(context.workspaceRoot, SCHEMA_DIRECTORY);
  const violations: Violation[] = [];
  const validators = loadSchemas(schemaDirectory, context.workspaceRoot, violations);
  const targets = jsonTargets(context.workspaceRoot, Object.keys(context.manifest.packages));
  const referencedSchemas = new Set<string>();

  for (const target of targets) {
    const targetKey = displayPath(context.workspaceRoot, target);
    const document = readJsonObject(target, targetKey, 'json-schema-document', violations);
    if (document === undefined) continue;

    const schemaReference = document.$schema;
    if (typeof schemaReference !== 'string' || schemaReference.length === 0) {
      violations.push({
        rule: 'json-schema-reference',
        packageKey: targetKey,
        message: "missing non-empty '$schema' reference",
      });
      continue;
    }

    const schemaPath = resolve(dirname(target), schemaReference);
    if (!isInside(schemaDirectory, schemaPath)) {
      violations.push({
        rule: 'json-schema-reference',
        packageKey: targetKey,
        message: `'$schema' must resolve inside ${displayPath(context.workspaceRoot, schemaDirectory)} (got '${schemaReference}')`,
      });
      continue;
    }
    referencedSchemas.add(schemaPath);

    const validate = validators.get(schemaPath);
    if (validate === undefined) {
      violations.push({
        rule: 'json-schema-reference',
        packageKey: targetKey,
        message: `'$schema' does not resolve to a valid registered schema: ${schemaReference}`,
      });
      continue;
    }
    if (validate(document)) continue;

    for (const error of validate.errors ?? []) {
      violations.push({
        rule: 'json-schema-validation',
        packageKey: targetKey,
        message: formatValidationError(error),
      });
    }
  }

  for (const schemaPath of validators.keys()) {
    if (referencedSchemas.has(schemaPath)) continue;
    violations.push({
      rule: 'json-schema-coverage',
      packageKey: displayPath(context.workspaceRoot, schemaPath),
      message: 'schema is not referenced by any checked JSON file',
    });
  }

  return {
    command: 'check-json-schemas',
    checkedPackageKeys: [
      ...validators.keys().map((path) => displayPath(context.workspaceRoot, path)),
      ...targets.map((path) => displayPath(context.workspaceRoot, path)),
    ],
    checkedItemLabel: 'schema or JSON file(s)',
    violations,
  };
};

function loadSchemas(
  schemaDirectory: string,
  workspaceRoot: string,
  violations: Violation[],
): Map<string, ValidateFunction<unknown>> {
  const validators = new Map<string, ValidateFunction<unknown>>();
  if (!existsSync(schemaDirectory)) {
    violations.push({
      rule: 'json-schema-document',
      packageKey: displayPath(workspaceRoot, schemaDirectory),
      message: 'schema directory is missing',
    });
    return validators;
  }

  const schemaFiles = readdirSync(schemaDirectory)
    .filter((name) => name.endsWith('.schema.json'))
    .sort()
    .map((name) => join(schemaDirectory, name));
  // `required` inside `if`/`not` deliberately refers to properties declared by the enclosing object.
  // That is valid JSON Schema, but Ajv's optional strictRequired lint treats it as suspicious.
  const ajv = new Ajv2020({ allErrors: true, strict: true, strictRequired: false, strictTypes: false });
  for (const schemaFile of schemaFiles) {
    const schemaKey = displayPath(workspaceRoot, schemaFile);
    const schema = readJsonObject(schemaFile, schemaKey, 'json-schema-document', violations);
    if (schema === undefined) continue;
    try {
      validators.set(schemaFile, ajv.compile(schema));
    } catch (error) {
      violations.push({
        rule: 'json-schema-document',
        packageKey: schemaKey,
        message: `invalid JSON Schema: ${error instanceof Error ? error.message : String(error)}`,
      });
    }
  }
  return validators;
}

function jsonTargets(workspaceRoot: string, packageKeys: readonly string[]): string[] {
  const targets = REQUIRED_JSON_FILES.map((file) => join(workspaceRoot, file));
  for (const packageKey of packageKeys) {
    const exportManifest = join(workspaceRoot, packageKey, 'export.manifest.json');
    if (existsSync(exportManifest)) targets.push(exportManifest);
  }
  return [...new Set(targets)].sort();
}

function readJsonObject(path: string, display: string, rule: string, violations: Violation[]): JsonObject | undefined {
  if (!existsSync(path)) {
    violations.push({ rule, packageKey: display, message: 'file is missing' });
    return undefined;
  }
  let parsed: unknown;
  try {
    parsed = JSON.parse(readFileSync(path, 'utf8'));
  } catch (error) {
    violations.push({
      rule,
      packageKey: display,
      message: `invalid JSON: ${error instanceof Error ? error.message : String(error)}`,
    });
    return undefined;
  }
  if (typeof parsed !== 'object' || parsed === null || Array.isArray(parsed)) {
    violations.push({ rule, packageKey: display, message: 'expected a JSON object at the document root' });
    return undefined;
  }
  return parsed as JsonObject;
}

function formatValidationError(error: ErrorObject): string {
  const path = error.instancePath === '' ? '/' : error.instancePath;
  if (error.keyword === 'required') {
    return `${path}: missing required property '${String(error.params['missingProperty'])}'`;
  }
  if (error.keyword === 'additionalProperties') {
    return `${path}: unknown property '${String(error.params['additionalProperty'])}'`;
  }
  return `${path}: ${error.message ?? `failed '${error.keyword}' validation`}`;
}

function isInside(directory: string, path: string): boolean {
  const rel = relative(directory, path);
  return rel !== '' && rel !== '..' && !rel.startsWith(`..${sep}`) && !isAbsolute(rel);
}

function displayPath(workspaceRoot: string, path: string): string {
  const rel = relative(workspaceRoot, path);
  return rel === '' ? '.' : `./${rel}`;
}
