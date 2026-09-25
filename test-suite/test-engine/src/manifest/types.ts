/**
 * Contract of the Manifest component: the typed view of a `scenario.yaml` after it has been
 * validated against `schema/scenario.v1.schema.json` and defaults have been applied.
 */

export const MANIFEST_API_VERSION = 'fhevm.zama.ai/scenario/v1';
export const MANIFEST_FILE_NAME = 'scenario.yaml';

export interface ScenarioManifest {
  apiVersion: typeof MANIFEST_API_VERSION;
  kind: 'Scenario';
  metadata: {
    id: string;
    name: string;
    owner: string;
    description: string;
    tags: string[];
    origin?: string;
  };
  spec: {
    runtime: 'cucumber';
    /** Feature file, relative to the scenario directory. */
    entrypoint: string;
    /** TypeScript support code, relative to the scenario directory. */
    supportCode: string[];
    timeoutSeconds: number;
    enabled: boolean;
  };
}

export class ManifestError extends Error {
  constructor(
    readonly manifestPath: string,
    readonly problems: string[],
  ) {
    super(`${manifestPath}: ${problems.join('; ')}`);
    this.name = 'ManifestError';
  }
}
