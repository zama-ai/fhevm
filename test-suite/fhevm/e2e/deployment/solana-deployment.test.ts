// Real validator + PostgreSQL, with a fixed gateway committee fixture. The full e2e suite
// separately exercises the real gateway/KMS/coprocessor and confidential decryption.
import { createSolanaRpc } from '@solana/kit';
import { afterAll, beforeAll, expect, test } from 'bun:test';
import { cp, mkdir, mkdtemp, readFile, rm } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import path from 'node:path';

import { REPO_ROOT } from '../../src/layout';
import { withDeploymentLock } from '../../src/solana/host-deploy/lock';
import { programIdsFor } from '../../src/solana/host-deploy/program-profile';
import { findHostConfigPda } from '../../src/solana/internal/generated/zamaHost/pdas/hostConfig';
import { validatorStartArgs } from '../../src/solana/validator';
import { run, runStreaming } from '../../src/utils/process';

const solana = path.join(REPO_ROOT, 'solana');
const rpcUrl = 'http://127.0.0.1:18999';
const rpc = createSolanaRpc(rpcUrl);
const container = `solana-deploy-test-${process.pid}`;
let directory = '';
let databaseUrl = '';
let validator: ReturnType<typeof Bun.spawn> | undefined;
let gateway: ReturnType<typeof Bun.serve> | undefined;
let env: Record<string, string>;

const deploy = (action = 'deploy', overrides: Record<string, string> = {}) => {
  const variables = { ...env, ...overrides };
  const image = process.env.SOLANA_DEPLOY_TEST_IMAGE;
  return run(
    image
      ? [
          'docker',
          'run',
          '--rm',
          '--network=host',
          '--user',
          `${process.getuid!()}:${process.getgid!()}`,
          '-v',
          `${directory}:${directory}`,
          '-v',
          `${solana}/scripts/e2e/test-keypairs:${solana}/scripts/e2e/test-keypairs:ro`,
          ...Object.keys(variables).flatMap((name) => ['-e', name]),
          image,
          'host',
          action,
        ]
      : ['bun', 'run', 'src/solana/host-deploy/cli.ts', 'host', action],
    {
      cwd: path.join(REPO_ROOT, 'test-suite/fhevm'),
      env: variables,
      allowFailure: true,
    },
  );
};
const hostData = async () => {
  const [config] = await findHostConfigPda();
  return (await rpc.getAccountInfo(config, { encoding: 'base64' }).send()).value?.data;
};

beforeAll(async () => {
  directory = await mkdtemp(path.join(tmpdir(), 'solana-deploy-'));
  await mkdir(path.join(directory, 'A'));
  await mkdir(path.join(directory, 'B'));
  if (process.env.SOLANA_DEPLOY_TEST_IMAGE) {
    const id = (await run(['docker', 'create', process.env.SOLANA_DEPLOY_TEST_IMAGE])).stdout.trim();
    try {
      await run(['docker', 'cp', `${id}:/app/programs/zama_host.so`, path.join(directory, 'A/zama_host.so')]);
    } finally {
      await run(['docker', 'rm', id]);
    }
  } else {
    await cp(path.join(solana, 'target/deploy/zama_host.so'), path.join(directory, 'A/zama_host.so'));
  }
  await runStreaming(['bash', 'scripts/build-programs.sh', 'localnet', 'zama_host'], {
    cwd: solana,
    env: { CARGO_PROFILE_RELEASE_OPT_LEVEL: '2' },
  });
  await cp(path.join(solana, 'target/deploy/zama_host.so'), path.join(directory, 'B/zama_host.so'));
  await cp(path.join(directory, 'A/zama_host.so'), path.join(solana, 'target/deploy/zama_host.so'));
  expect(await readFile(path.join(directory, 'A/zama_host.so'))).not.toEqual(
    await readFile(path.join(directory, 'B/zama_host.so')),
  );
  await run(['solana-keygen', 'new', '--no-bip39-passphrase', '--silent', '-o', path.join(directory, 'payer.json')]);
  validator = Bun.spawn(
    [
      ...validatorStartArgs({ ledgerDir: path.join(directory, 'ledger'), rpcPort: 18999 }),
      '--quiet',
      '--faucet-port',
      '19900',
    ],
    {
      stdout: Bun.file(path.join(directory, 'validator.log')),
      stderr: 'inherit',
    },
  );
  for (let i = 0; ; i++) {
    try {
      await rpc.getHealth().send();
      break;
    } catch {
      if (i === 90 || validator.exitCode !== null)
        throw new Error(`validator failed; inspect ${directory}/validator.log`);
      await Bun.sleep(1000);
    }
  }
  await run(['solana', 'airdrop', '100', '-u', rpcUrl, '-k', path.join(directory, 'payer.json')]);
  await run([
    'docker',
    'run',
    '-d',
    '--name',
    container,
    '-e',
    'POSTGRES_PASSWORD=test',
    '-p',
    '127.0.0.1::5432',
    'postgres:17-alpine',
  ]);
  const port = (await run(['docker', 'port', container, '5432'])).stdout.trim().split(':').at(-1);
  databaseUrl = `postgresql://postgres:test@127.0.0.1:${port}/postgres`;
  for (let i = 0; ; i++) {
    const result = await run(['docker', 'exec', container, 'pg_isready', '-U', 'postgres'], { allowFailure: true });
    if (result.code === 0) break;
    if (i === 30) throw new Error('PostgreSQL did not become ready');
    await Bun.sleep(1000);
  }
  gateway = Bun.serve({
    port: 0,
    async fetch(request) {
      const { id, method } = (await request.json()) as { id: number; method: string };
      const word = (value: string) => value.padStart(64, '0');
      const result = method === 'eth_chainId' ? '0xd431' : `0x${word('20')}${word('1')}${word('a'.repeat(40))}`;
      return Response.json({ jsonrpc: '2.0', id, result });
    },
  });
  env = {
    SOLANA_RPC_URL: rpcUrl,
    SOLANA_DEPLOY_DATABASE_URL: databaseUrl,
    SOLANA_DEPLOYER_KEYPAIR: path.join(directory, 'payer.json'),
    SOLANA_ZAMA_HOST_KEYPAIR: path.join(solana, 'scripts/e2e/test-keypairs/zama_host-keypair.json'),
    SOLANA_ARTIFACTS_DIR: path.join(directory, 'A'),
    SOLANA_PROGRAM_PROFILE: 'localnet',
    ADDRESSES_DIR: path.join(directory, 'addresses'),
    GATEWAY_RPC_URL: gateway.url.toString(),
    GATEWAY_CONFIG_ADDRESS: `0x${'1'.repeat(40)}`,
    INPUT_VERIFICATION_ADDRESS: `0x${'2'.repeat(40)}`,
    DECRYPTION_ADDRESS: `0x${'3'.repeat(40)}`,
    KMS_THRESHOLD: '0',
    COPROCESSOR_THRESHOLD: '1',
  };
}, 15 * 60_000);

afterAll(async () => {
  gateway?.stop(true);
  if (validator) {
    validator.kill();
    await validator.exited;
  }
  await run(['docker', 'rm', '-f', container], { allowFailure: true });
  if (directory) await rm(directory, { recursive: true, force: true });
});

test(
  'deploy, verified no-op, explicit compatible upgrade, and persistent HostConfig',
  async () => {
    const first = await deploy();
    expect(first.code, first.stderr).toBe(0);
    const before = await hostData();
    expect(before).toBeDefined();
    const again = await deploy();
    expect(again.code, again.stderr).toBe(0);
    expect(again.stdout).toContain('unchanged');
    expect(again.stdout).not.toContain('OK initialize');
    const changed = { SOLANA_ARTIFACTS_DIR: path.join(directory, 'B') };
    expect((await deploy('deploy', changed)).stderr).toContain('explicit upgrade');
    expect((await deploy('upgrade', { ...changed, DECRYPTION_ADDRESS: `0x${'4'.repeat(40)}` })).stderr).toContain(
      'does not match',
    );
    // A remains deployed after the rejected incompatible configuration.
    expect((await deploy()).stdout).toContain('unchanged');
    const upgraded = await deploy('upgrade', changed);
    expect(upgraded.code, upgraded.stderr).toBe(0);
    expect(await hostData()).toEqual(before);
    expect((await deploy('deploy', changed)).stdout).toContain('unchanged');
  },
  15 * 60_000,
);

test('a competing deployer fails before it can change on-chain state', async () => {
  await withDeploymentLock(databaseUrl, rpcUrl, programIdsFor('localnet').zamaHost, async () => {
    const blocked = await deploy();
    expect(blocked.code).not.toBe(0);
    expect(blocked.stderr).toContain('another deployment is running');
  });
  expect((await deploy('deploy', { SOLANA_ARTIFACTS_DIR: path.join(directory, 'B') })).code).toBe(0);
}, 60_000);
