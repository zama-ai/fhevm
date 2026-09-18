import { expect, test } from 'bun:test';
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { parse } from 'yaml';

const repo = resolve(import.meta.dir, '../../..');
const readYaml = (file: string) => parse(readFileSync(resolve(repo, file), 'utf8'));

test('GPU baseline resolution authenticates package reads and refuses unverified locks', () => {
  const workflow = readYaml('.github/workflows/test-suite-orchestrate-e2e-gpu-tests.yml');
  const job = workflow.jobs['resolve-baseline'];
  const step = job.steps.find((step: { id?: string }) => step.id === 'resolve-baseline');
  expect(job.permissions.packages).toBe('read');
  expect(step.env.GH_TOKEN).toBe('${{ secrets.GHCR_READ_TOKEN }}');
  expect(step.env.REQUIRE_PUBLISHED_IMAGE_CHECK).toBe('true');
});

test('GPU bundle output is emitted only after the complete image publication loop', () => {
  const workflow = readYaml('.github/workflows/coprocessor-gpu-docker-build.yml');
  const step = workflow.jobs.build.steps.find((step: { id?: string }) => step.id === 'images');
  const script: string = step.run;
  const output = script.indexOf('echo "worker-version=');
  const loopEnd = script.lastIndexOf('done');
  expect(script).toContain('set -euo pipefail');
  expect(script).toContain('docker push "$image"');
  expect(output).toBeGreaterThan(loopEnd);
  expect(script.match(/echo "worker-version=/g)).toHaveLength(1);
});

test('container preparation installs ACL utilities before granting Docker access', () => {
  const action = readYaml('.github/actions/gpu_container_runtime/action.yml');
  const commands = action.runs.steps.map((step: { run: string }) => step.run).join('\n');
  const installation = commands.indexOf('install -y acl');
  expect(installation).toBeGreaterThanOrEqual(0);
  expect(installation).toBeLessThan(commands.indexOf('sudo setfacl'));
});
