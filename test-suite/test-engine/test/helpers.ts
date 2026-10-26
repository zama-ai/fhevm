import { spawnSync } from 'node:child_process';
import { mkdtempSync, readFileSync } from 'node:fs';
import os from 'node:os';
import path from 'node:path';

import type { DiscoveredScenario } from '../src/discovery/discovery.js';
import { DEFAULT_PROFILES_DIR, DEFAULT_SCENARIOS_DIR, DIST_DIR, PACKAGE_ROOT } from '../src/paths.js';
import type { PlanningProfile } from '../src/planner/planner.js';
import { listProfileNames } from '../src/profiles/profile.js';
import type { EvidenceFile } from '../src/report/evidence.js';
import type { RunReport } from '../src/report/report.js';

export const FIXTURES_DIR = path.join(PACKAGE_ROOT, 'test', 'fixtures');
export const FIXTURE_SCENARIOS = path.join(FIXTURES_DIR, 'scenarios');
export const FIXTURE_PROFILES = path.join(FIXTURES_DIR, 'profiles');
export const INVALID_SCENARIOS = path.join(FIXTURES_DIR, 'invalid-scenarios');
export const INVALID_PROFILES = path.join(FIXTURES_DIR, 'invalid-profiles');
export const RESERVED_PROFILES = path.join(FIXTURES_DIR, 'reserved-profiles');
export const CLI = path.join(DIST_DIR, 'src', 'cli', 'main.js');

/** Fake secret used by the secret-handling fixtures. */
export const FIXTURE_SECRET = 'fixture-secret-s3cr3t-value';

export const discoveryOptions = (scenariosDir: string, profilesDir = FIXTURE_PROFILES) => ({
  scenariosDir,
  sourceRoot: PACKAGE_ROOT,
  distDir: DIST_DIR,
  knownProfiles: listProfileNames(profilesDir),
});

export function tempDir(prefix = 'test-engine-'): string {
  return mkdtempSync(path.join(os.tmpdir(), prefix));
}

export interface CliResult {
  code: number | null;
  stdout: string;
  stderr: string;
  outputDir: string;
}

export interface CliOptions {
  scenariosDir?: string;
  profilesDir?: string;
  env?: NodeJS.ProcessEnv;
}

/**
 * Runs the compiled CLI, by default against the fixture scenarios and fixture profiles, writing
 * the run output to a temporary directory.
 */
export function runCli(args: string[], options: CliOptions = {}): CliResult {
  const outputDir = tempDir();
  const scenariosDir = options.scenariosDir ?? FIXTURE_SCENARIOS;
  const profilesDir =
    options.profilesDir ?? (scenariosDir === DEFAULT_SCENARIOS_DIR ? DEFAULT_PROFILES_DIR : FIXTURE_PROFILES);
  const result = spawnSync(
    process.execPath,
    [CLI, ...args, '--scenarios-dir', scenariosDir, '--profiles-dir', profilesDir, '--output-dir', outputDir],
    {
      cwd: PACKAGE_ROOT,
      encoding: 'utf8',
      env: { ...process.env, NO_COLOR: '1', ...options.env },
    },
  );
  return { code: result.status, stdout: result.stdout, stderr: result.stderr, outputDir };
}

export function readReport(outputDir: string): RunReport {
  return JSON.parse(readFileSync(path.join(outputDir, 'report.json'), 'utf8')) as RunReport;
}

export function readEvidence(outputDir: string): EvidenceFile {
  return JSON.parse(readFileSync(path.join(outputDir, 'evidence.json'), 'utf8')) as EvidenceFile;
}

export function scenarioOf(report: RunReport, id: string) {
  const scenario = report.scenarios.find((candidate) => candidate.id === id);
  if (!scenario) throw new Error(`scenario ${id} not in report`);
  return scenario;
}

export const planningProfile = (name: string, requiresConfirmation = false): PlanningProfile => ({
  name,
  requiresConfirmation,
});

/** Minimal in-memory DiscoveredScenario for planner tests. */
export function fakeScenario(
  id: string,
  tags: string[] = [],
  enabled = true,
  extra: { targets?: string[] } = {},
): DiscoveredScenario {
  return {
    manifest: {
      apiVersion: 'fhevm.zama.ai/scenario/v1',
      kind: 'Scenario',
      metadata: { id, name: id, owner: 'qa', description: id, tags },
      spec: {
        runtime: 'cucumber',
        entrypoint: './x.feature',
        supportCode: [],
        timeoutSeconds: 30,
        enabled,
        ...(extra.targets ? { targets: extra.targets } : {}),
      },
    },
    manifestPath: `/scenarios/${id}/scenario.yaml`,
    directory: `/scenarios/${id}`,
    featurePath: `/scenarios/${id}/x.feature`,
    supportModules: [],
  };
}
