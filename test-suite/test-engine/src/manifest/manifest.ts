import { MANIFEST_SCHEMA_PATH } from '../paths.js';
import { createYamlValidator } from '../schema/yaml-document.js';
import { ManifestError, type ScenarioManifest } from './types.js';

/** Schema validation of a manifest source; keeps the parsed document on failure. */
export const validateManifest = createYamlValidator<ScenarioManifest>(MANIFEST_SCHEMA_PATH);

/**
 * Parses and validates one manifest. Pure: no filesystem access besides loading the schema, so
 * cross-file checks (referenced files, known profiles, duplicate ids) belong to
 * discovery.
 *
 * @throws ManifestError listing every schema violation found.
 */
export function parseManifest(manifestPath: string, source: string): ScenarioManifest {
  const result = validateManifest(source);
  if (!result.ok) throw new ManifestError(manifestPath, result.problems);
  return result.document;
}
