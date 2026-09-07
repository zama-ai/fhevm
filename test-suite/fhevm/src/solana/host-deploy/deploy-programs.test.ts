import { afterEach, expect, test } from 'bun:test';
import { mkdtemp, readFile, rm, writeFile } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import path from 'node:path';

import { deployProgramArtifacts } from './deploy-programs';
import { programIdsFor } from './program-profile';

const originalPath = process.env.PATH;
let directory: string | undefined;
afterEach(async () => {
  process.env.PATH = originalPath;
  if (directory) await rm(directory, { recursive: true, force: true });
});

const fixture = async (failure = false) => {
  directory = await mkdtemp(path.join(tmpdir(), 'solana-deploy-test-'));
  const host = programIdsFor('preview-env').zamaHost;
  await writeFile(
    path.join(directory, 'solana'),
    `#!/bin/sh
if [ "$1" = address ]; then echo '${host}'; exit 0; fi
printf '%s\\n' "$1 $2" >> '${directory}/calls'
${failure ? 'echo "https://rpc.example/?api-key=private-test-value" >&2; exit 1' : 'exit 0'}
`,
    { mode: 0o700 },
  );
  await writeFile(path.join(directory, 'zama_host.so'), 'fixture');
  await writeFile(path.join(directory, 'host.json'), 'fixture');
  process.env.PATH = `${directory}:${originalPath}`;
  return {
    rpcUrl: 'https://rpc.example/?api-key=private-test-value',
    deployerKeypairPath: path.join(directory, 'payer.json'),
    artifactsDir: directory,
    programKeypairPaths: { zama_host: path.join(directory, 'host.json') },
    programs: ['zama_host'] as const,
    profile: 'preview-env' as const,
  };
};

test('host deployment needs no demo keys or artifacts', async () => {
  const parameters = await fixture();
  expect(await deployProgramArtifacts(parameters)).toEqual({ zama_host: programIdsFor('preview-env').zamaHost });
  expect(await readFile(path.join(directory!, 'calls'), 'utf8')).toBe('program deploy\n');
});

test('a subprocess failure cannot disclose the RPC credential', async () => {
  const parameters = await fixture(true);
  try {
    await deployProgramArtifacts(parameters);
    throw new Error('expected deployment failure');
  } catch (error) {
    expect(String(error)).toContain('exit 1');
    expect(String(error)).not.toContain('private-test-value');
  }
});

test('upgrade refuses an absent program before sending deployment', async () => {
  const parameters = await fixture(true);
  await expect(deployProgramArtifacts({ ...parameters, upgrade: true })).rejects.toThrow('exit 1');
  expect(await readFile(path.join(directory!, 'calls'), 'utf8')).toBe('program show\n');
});
