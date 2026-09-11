import { afterEach, expect, test } from 'bun:test';
import { mkdtemp, readFile, rm, writeFile } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import path from 'node:path';

import { deployProgramArtifacts } from '../../../../../solana/deploy/src/deploy-programs';
import { programIdsFor } from '../../../../../solana/deploy/src/program-profile';

const originalPath = process.env.PATH;
let directory: string | undefined;
let server: ReturnType<typeof Bun.serve> | undefined;
afterEach(async () => {
  process.env.PATH = originalPath;
  server?.stop(true);
  if (directory) await rm(directory, { recursive: true, force: true });
});

const fixture = async (
  options: { exists?: boolean; changed?: boolean; wrongAuthority?: boolean; failure?: boolean } = {},
) => {
  directory = await mkdtemp(path.join(tmpdir(), 'solana-deploy-test-'));
  const host = programIdsFor('preview-env').zamaHost;
  server = Bun.serve({
    port: 0,
    async fetch(request) {
      const { id, method } = (await request.json()) as { id: string; method: string };
      if (method === 'getSlot') return Response.json({ jsonrpc: '2.0', id, result: 2 });
      return Response.json({
        jsonrpc: '2.0',
        id,
        result: {
          context: { slot: 1 },
          value: options.exists
            ? {
                executable: true,
                owner: 'BPFLoaderUpgradeab1e11111111111111111111111',
                lamports: 1,
                rentEpoch: 0,
                data: ['', 'base64'],
                space: 36,
              }
            : null,
        },
      });
    },
  });
  await writeFile(
    path.join(directory, 'solana'),
    `#!/bin/sh
if [ "$1" = address ]; then echo '${host}'; exit 0; fi
printf '%s\\n' "$1 $2" >> '${directory}/calls'
${options.failure ? 'echo "https://rpc.example/?api-key=private-test-value" >&2; exit 1' : ''}
if [ "$2" = show ]; then echo '{"lastDeploySlot":1,"authority":"${options.wrongAuthority ? 'wrong' : host}"}'; fi
if [ "$2" = dump ]; then printf '${options.changed ? 'different' : 'fixture\\000\\000'}' > "$6"; fi
`,
    { mode: 0o700 },
  );
  await writeFile(path.join(directory, 'zama_host.so'), 'fixture');
  await writeFile(path.join(directory, 'host.json'), 'fixture');
  await writeFile(path.join(directory, 'calls'), '');
  process.env.PATH = `${directory}:${originalPath}`;
  return {
    rpcUrl: `${server.url}?api-key=private-test-value`,
    deployerKeypairPath: path.join(directory, 'payer.json'),
    artifactsDir: directory,
    programKeypairPaths: { zama_host: path.join(directory, 'host.json') },
    programs: ['zama_host'] as const,
    profile: 'preview-env' as const,
  };
};
const calls = () => readFile(path.join(directory!, 'calls'), 'utf8');

test('host deployment needs no demo keys or artifacts', async () => {
  expect(await deployProgramArtifacts(await fixture())).toEqual({ zama_host: programIdsFor('preview-env').zamaHost });
  expect(await calls()).toBe('program deploy\nprogram show\n');
});
test('a subprocess failure cannot disclose the RPC credential', async () => {
  try {
    await deployProgramArtifacts(await fixture({ failure: true }));
    throw new Error('expected deployment failure');
  } catch (error) {
    expect(String(error)).toContain('exit 1');
    expect(String(error)).not.toContain('private-test-value');
  }
});
test('upgrade refuses an absent program before sending deployment', async () => {
  const parameters = await fixture();
  await expect(deployProgramArtifacts({ ...parameters, upgrade: true })).rejects.toThrow('does not exist');
  expect(await calls()).toBe('');
});
test('matching bytecode, including zero allocation padding, sends no deployment', async () => {
  await deployProgramArtifacts(await fixture({ exists: true }));
  expect(await calls()).toBe('program show\nprogram dump\n');
});
test('different bytecode requires an explicit upgrade', async () => {
  await expect(deployProgramArtifacts(await fixture({ exists: true, changed: true }))).rejects.toThrow(
    'explicit upgrade',
  );
  expect(await calls()).not.toContain('deploy');
});
test('explicit upgrade deploys different bytecode', async () => {
  const parameters = await fixture({ exists: true, changed: true });
  await deployProgramArtifacts({ ...parameters, upgrade: true });
  expect(await calls()).toBe('program show\nprogram dump\nprogram deploy\nprogram show\n');
});
test('wrong authority fails before uploading bytecode', async () => {
  const parameters = await fixture({ exists: true, wrongAuthority: true, changed: true });
  await expect(deployProgramArtifacts({ ...parameters, upgrade: true })).rejects.toThrow('authority');
  expect(await calls()).toBe('program show\nprogram dump\n');
});

test('matching bytecode does not require upgrade authority', async () => {
  await deployProgramArtifacts(await fixture({ exists: true, wrongAuthority: true }));
  expect(await calls()).not.toContain('deploy');
});
