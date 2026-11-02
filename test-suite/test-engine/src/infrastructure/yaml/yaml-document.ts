import { Ajv, type ErrorObject, type ValidateFunction } from 'ajv';
import { readFileSync } from 'node:fs';
import { YAMLParseError, parse } from 'yaml';

/**
 * Shared YAML + JSON Schema validation for the engine's declarative files (scenario manifests
 * and environment profiles). Returns human-readable problems instead of raw Ajv errors.
 */

/**
 * On a schema failure, `document` still holds the parsed YAML (when it parsed), so callers can run
 * their own semantic checks and report every problem at once.
 */
export type ValidationResult<T> = { ok: true; document: T } | { ok: false; problems: string[]; document?: unknown };

function describe(error: ErrorObject): string {
  const location = error.instancePath === '' ? '(root)' : error.instancePath.slice(1).replaceAll('/', '.');
  switch (error.keyword) {
    case 'additionalProperties':
      return `${location}: unknown field '${String(error.params.additionalProperty)}'`;
    case 'required':
      return `${location}: missing required field '${String(error.params.missingProperty)}'`;
    case 'const':
      return `${location}: must be '${String(error.params.allowedValue)}'`;
    case 'propertyNames':
      return `${location}: invalid key '${String(error.params.propertyName)}'`;
    default:
      return `${location}: ${error.message ?? 'is invalid'}`;
  }
}

/** Compiles the schema lazily (once) and returns a validator for YAML sources. */
export function createYamlValidator<T>(schemaPath: string): (source: string) => ValidationResult<T> {
  let validate: ValidateFunction | undefined;
  return (source) => {
    let document: unknown;
    try {
      document = parse(source);
    } catch (error) {
      // YAMLParseError messages embed a multi-line source excerpt; the first line is enough.
      const message = error instanceof YAMLParseError ? (error.message.split('\n')[0] ?? error.message) : String(error);
      return { ok: false, problems: [`invalid YAML: ${message}`] };
    }
    validate ??= new Ajv({ allErrors: true, useDefaults: true, strict: true, allowUnionTypes: true }).compile(
      JSON.parse(readFileSync(schemaPath, 'utf8')) as object,
    );
    if (!validate(document)) {
      // Errors inside `anyOf` branches are noise once the `anyOf` itself is reported.
      const errors = (validate.errors ?? []).filter(
        (error, _, all) =>
          error.keyword === 'anyOf' ||
          !all.some(
            (other) =>
              other.keyword === 'anyOf' && error.schemaPath.startsWith(other.schemaPath.replace(/\/anyOf$/, '/anyOf/')),
          ),
      );
      return { ok: false, problems: [...new Set(errors.map(describe))], document };
    }
    return { ok: true, document: document as T };
  };
}
