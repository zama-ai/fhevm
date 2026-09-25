import { Ajv, type ErrorObject, type ValidateFunction } from 'ajv';
import { readFileSync } from 'node:fs';
import { YAMLParseError, parse } from 'yaml';

import { MANIFEST_SCHEMA_PATH } from '../paths.js';
import { ManifestError, type ScenarioManifest } from './types.js';

let validator: ValidateFunction | undefined;

function getValidator(): ValidateFunction {
  if (!validator) {
    const schema = JSON.parse(readFileSync(MANIFEST_SCHEMA_PATH, 'utf8')) as object;
    validator = new Ajv({ allErrors: true, useDefaults: true, strict: true }).compile(schema);
  }
  return validator;
}

function describe(error: ErrorObject): string {
  const location = error.instancePath === '' ? '(root)' : error.instancePath.slice(1).replaceAll('/', '.');
  switch (error.keyword) {
    case 'additionalProperties':
      return `${location}: unknown field '${String(error.params.additionalProperty)}'`;
    case 'required':
      return `${location}: missing required field '${String(error.params.missingProperty)}'`;
    case 'const':
      return `${location}: must be '${String(error.params.allowedValue)}'`;
    default:
      return `${location}: ${error.message ?? 'is invalid'}`;
  }
}

/**
 * Parses and validates one manifest. Pure: no filesystem access besides loading the schema, so
 * cross-file checks (referenced files, duplicate ids) belong to discovery.
 *
 * @throws ManifestError listing every schema violation found.
 */
export function parseManifest(manifestPath: string, source: string): ScenarioManifest {
  let document: unknown;
  try {
    document = parse(source);
  } catch (error) {
    // YAMLParseError messages embed a multi-line source excerpt; the first line is enough.
    const message = error instanceof YAMLParseError ? (error.message.split('\n')[0] ?? error.message) : String(error);
    throw new ManifestError(manifestPath, [`invalid YAML: ${message}`]);
  }

  const validate = getValidator();
  if (!validate(document)) {
    throw new ManifestError(manifestPath, (validate.errors ?? []).map(describe));
  }
  return document as ScenarioManifest;
}
