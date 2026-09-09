// Real validator + PostgreSQL, with a fixed gateway committee fixture. The full e2e suite
// separately exercises the real gateway/KMS/coprocessor and confidential decryption.
import { createSolanaRpc } from '@solana/kit';
import { afterAll, beforeAll, expect, test } from 'bun:test';
import { cp, mkdir, mkdtemp, readFile, rm } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import path from 'node:path';

import { REPO_ROOT } from '../../src/layout';
import { withDeploymentLock } from '../../../../solana/deploy/src/lock';
import { programIdsFor } from '../../../../solana/deploy/src/program-profile';
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

const deploy = (action = 'deploy', overrides: Record<string, string> = {}, target = 'host') => {
  const variables = { ...env, ...overrides };
  const image = process.env.SOLANA_DEPLOY_TEST_IMAGE;
  if (image && process.platform === 'darwin') {
    for (const name of ['SOLANA_RPC_URL', 'GATEWAY_RPC_URL', 'SOLANA_DEPLOY_DATABASE_URL', 'DATABASE_URL']) {
      if (variables[name]) variables[name] = variables[name]!.replace(/127\.0\.0\.1|localhost/g, 'host.docker.internal');
    }
  }
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
          target,
          action,
        ]
      : ['bun', 'run', '../../solana/deploy/src/cli.ts', target, action],
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
    env: { CARGO_PROFILE_RELEASE_OPT_LEVEL: '2', SBF_OUT_PATH: path.join(directory, 'B') },
  });
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
      '--gossip-port',
      '19901',
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
        throw new Error(
          `validator failed: ${await Bun.file(path.join(directory, 'ledger/validator.log'))
            .text().catch(() => 'no validator log')}`,
        );
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
    for (const name of ['INPUT_VERIFICATION_ADDRESS', 'DECRYPTION_ADDRESS']) {
      const invalid = await deploy('deploy', { [name]: `0x${'0'.repeat(40)}` });
      expect(invalid.code).not.toBe(0);
      expect(invalid.stderr).toContain('nonzero 20-byte addresses');
      expect((await rpc.getAccountInfo(programIdsFor('localnet').zamaHost).send()).value).toBeNull();
    }
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

test('coprocessor register connects to PostgreSQL and preserves registration on retry', async () => {
  const sql = (input: string) => run(
    ['docker', 'exec', '-i', container, 'psql', '-U', 'postgres', '-At', '-v', 'ON_ERROR_STOP=1'],
    { input },
  );
  await sql(`
    CREATE TABLE host_chains (chain_id bigint PRIMARY KEY, name text, acl_contract_address text NOT NULL);
    CREATE TABLE keys (
      sequence_number bigserial PRIMARY KEY, key_id_gw bytea, key_id bytea, pks_key bytea,
      sks_key bytea, cks_key bytea, sns_pk bytea, compressed_xof_keyset bytea,
      chain_id bigint, block_hash bytea, UNIQUE (chain_id, key_id)
    );
  `);
  const variables = { DATABASE_URL: databaseUrl, SOLANA_KEY_SOURCE_CHAIN_ID: '12345' };
  const missingKeys = await deploy('register', variables, 'coprocessor');
  expect(missingKeys.code).not.toBe(0);
  expect((await sql('SELECT count(*) FROM host_chains')).stdout.trim()).toBe('0');
  await sql("INSERT INTO keys (chain_id,key_id,key_id_gw,pks_key) VALUES (12345,'\\x01','\\x02','\\x03')");
  for (let attempt = 0; attempt < 2; attempt++) {
    const result = await deploy('register', variables, 'coprocessor');
    expect(result.code, result.stderr).toBe(0);
  }
  expect((await sql('SELECT acl_contract_address FROM host_chains')).stdout.trim()).toBe(programIdsFor('localnet').zamaHost);
  expect((await sql('SELECT count(*) FROM keys WHERE chain_id <> 12345')).stdout.trim()).toBe('1');
  expect((await sql("SELECT encode(pks_key,'hex') FROM keys WHERE chain_id <> 12345")).stdout.trim()).toBe('03');
  await sql("UPDATE host_chains SET acl_contract_address = 'different-program'");
  expect((await deploy('register', variables, 'coprocessor')).code).not.toBe(0);
  expect((await sql('SELECT acl_contract_address FROM host_chains')).stdout.trim()).toBe('different-program');
}, 60_000);
