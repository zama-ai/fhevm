import { spawnSync } from 'node:child_process';
import { mkdtempSync, readFileSync } from 'node:fs';
import os from 'node:os';
import path from 'node:path';

import type { DiscoveredScenario } from '../src/discovery/discovery.js';
import { DIST_DIR, PACKAGE_ROOT } from '../src/paths.js';
import type { RunReport } from '../src/report/report.js';

export const FIXTURES_DIR = path.join(PACKAGE_ROOT, 'test', 'fixtures');
export const FIXTURE_SCENARIOS = path.join(FIXTURES_DIR, 'scenarios');
export const INVALID_SCENARIOS = path.join(FIXTURES_DIR, 'invalid-scenarios');
const CLI = path.join(DIST_DIR, 'src', 'cli', 'main.js');

export const discoveryOptions = (scenariosDir: string) => ({
  scenariosDir,
  sourceRoot: PACKAGE_ROOT,
  distDir: DIST_DIR,
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

/** Runs the compiled CLI against the fixture scenarios, writing the run output to a temp dir. */
export function runCli(args: string[], scenariosDir = FIXTURE_SCENARIOS): CliResult {
  const outputDir = tempDir();
  const result = spawnSync(
    process.execPath,
    [CLI, ...args, '--scenarios-dir', scenariosDir, '--output-dir', outputDir],
    {
      cwd: PACKAGE_ROOT,
      encoding: 'utf8',
      env: { ...process.env, NO_COLOR: '1' },
    },
  );
  return { code: result.status, stdout: result.stdout, stderr: result.stderr, outputDir };
}

export function readReport(outputDir: string): RunReport {
  return JSON.parse(readFileSync(path.join(outputDir, 'report.json'), 'utf8')) as RunReport;
}

export function scenarioOf(report: RunReport, id: string) {
  const scenario = report.scenarios.find((candidate) => candidate.id === id);
  if (!scenario) throw new Error(`scenario ${id} not in report`);
  return scenario;
}

/** Minimal in-memory DiscoveredScenario for planner tests. */
export function fakeScenario(id: string, tags: string[] = [], enabled = true): DiscoveredScenario {
  return {
    manifest: {
      apiVersion: 'fhevm.zama.ai/scenario/v1',
      kind: 'Scenario',
      metadata: { id, name: id, owner: 'qa', description: id, tags },
      spec: { runtime: 'cucumber', entrypoint: './x.feature', supportCode: [], timeoutSeconds: 30, enabled },
    },
    manifestPath: `/scenarios/${id}/scenario.yaml`,
    directory: `/scenarios/${id}`,
    featurePath: `/scenarios/${id}/x.feature`,
    supportModules: [],
  };
}
