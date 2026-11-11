import assert from 'node:assert/strict';
import { after, describe, it } from 'node:test';

import { redactedPreflight, runPreflight } from '../../../src/boot/preflight.js';
import { loadProfile, secretsOf } from '../../../src/config/profiles/profile.js';
import type { ResolvedProfile } from '../../../src/config/profiles/types.js';
import { createRedactor } from '../../../src/infrastructure/secrets/redactor.js';
import { FHE_PROFILES } from '../../helpers.js';
import { type FakeEnvironment, type FakeEnvironmentOptions, startFakeEnvironment } from './fake-environment.js';

const API_KEY = 'relayer-api-key-0123456789';
const environments: FakeEnvironment[] = [];
after(() => Promise.all(environments.map((environment) => environment.close())));

async function environment(options: FakeEnvironmentOptions = {}): Promise<FakeEnvironment> {
  const started = await startFakeEnvironment(options);
  environments.push(started);
  return started;
}

/** The fx-fhe fixture profile, with its RPC and relayer pointing at `rpc` and `relayer`. */
function profileFor(rpc: string, relayer: string, apiKey = ''): ResolvedProfile {
  const profile = loadProfile('fx-fhe', { profilesDir: FHE_PROFILES, env: {} });
  profile.chain!.rpcUrl = { ...profile.chain!.rpcUrl, value: rpc };
  profile.relayer!.url = { ...profile.relayer!.url, value: `${relayer}/v2` };
  profile.relayer!.apiKey = { value: apiKey, secret: true, source: 'env:KEY' };
  return profile;
}

const byName = (result: Awaited<ReturnType<typeof runPreflight>>) =>
  Object.fromEntries(result.checks.map((check) => [check.name, check]));

describe('preflight', () => {
  it('passes when the RPC, the relayer and the FHE key answer', async () => {
    const env = await environment();
    const result = await runPreflight(profileFor(env.url, env.url));
    assert.equal(result.status, 'passed');
    assert.deepEqual(
      result.checks.map((check) => [check.name, check.status]),
      [
        ['chain.rpc', 'passed'],
        ['contracts.code', 'passed'],
        ['relayer.keyurl', 'passed'],
        ['relayer.fhe-key', 'passed'],
      ],
    );
    const checks = byName(result);
    assert.equal(checks['chain.rpc']!.detail, 'chain ID 12345');
    assert.equal(checks['contracts.code']!.detail, '5 protocol contracts deployed');
    assert.equal(checks['relayer.keyurl']!.target, `${env.url}/v2/keyurl`);
    assert.equal(checks['relayer.fhe-key']!.target, `${env.url}/key`);
    assert.ok(env.requests.some((request) => request.method === 'HEAD' && request.path === '/key'));
  });

  it('fails chain.rpc on another chain ID or an unreachable RPC', async () => {
    const env = await environment({ chainId: 1 });
    const wrong = byName(await runPreflight(profileFor(env.url, env.url)))['chain.rpc']!;
    assert.equal(wrong.status, 'failed');
    assert.match(wrong.detail!, /expects chain ID 12345, but the RPC .* reports 1/);
    const down = byName(await runPreflight(profileFor('http://127.0.0.1:9', env.url), { timeoutMs: 3_000 }))[
      'chain.rpc'
    ]!;
    assert.equal(down.status, 'failed');
    assert.match(down.detail!, /cannot reach the chain RPC|no answer within/);
  });

  it('explains a relayer refusal with and without an API key, and skips the key check', async () => {
    const env = await environment({ keyUrlStatus: 401 });
    const keyless = byName(await runPreflight(profileFor(env.url, env.url)));
    assert.equal(keyless['relayer.keyurl']!.status, 'failed');
    assert.match(keyless['relayer.keyurl']!.detail!, /HTTP 401: .*it requires relayer\.apiKey/);
    assert.deepEqual(
      [keyless['relayer.fhe-key']!.status, keyless['relayer.fhe-key']!.detail],
      ['skipped', 'relayer.keyurl failed'],
    );
    const forbidden = await environment({ keyUrlStatus: 403 });
    const withKey = byName(await runPreflight(profileFor(forbidden.url, forbidden.url, API_KEY)));
    assert.match(withKey['relayer.keyurl']!.detail!, /HTTP 403: .*relayer\.apiKey rejected/);
  });

  it('sends the API key header only when the profile has a non-empty key', async () => {
    const env = await environment();
    await runPreflight(profileFor(env.url, env.url, API_KEY));
    await runPreflight(profileFor(env.url, env.url, ''));
    const keyUrlCalls = env.requests.filter((request) => request.path === '/v2/keyurl');
    assert.equal(keyUrlCalls[0]!.headers['x-api-key'], API_KEY);
    assert.equal(keyUrlCalls[1]!.headers['x-api-key'], undefined);
    const keyCalls = env.requests.filter((request) => request.path === '/key');
    assert.ok(
      keyCalls.every((request) => request.headers['x-api-key'] === undefined),
      'the key object gets no key',
    );
  });

  it('fails relayer.keyurl when the answer holds no FHE public key URL', async () => {
    const env = await environment({ keyUrlBody: () => ({ status: 'succeeded', response: { fheKeyInfo: [] } }) });
    const check = byName(await runPreflight(profileFor(env.url, env.url)))['relayer.keyurl']!;
    assert.deepEqual([check.status, check.detail], ['failed', 'the answer holds no FHE public key URL']);
  });

  it('fails relayer.fhe-key when the key host only exists on the stack network', async () => {
    const env = await environment({
      keyUrlBody: () => ({
        response: { fheKeyInfo: [{ fhePublicKey: { urls: ['http://minio.invalid:9000/kms-public/key'] } }] },
      }),
    });
    const check = byName(await runPreflight(profileFor(env.url, env.url)))['relayer.fhe-key']!;
    assert.equal(check.status, 'failed');
    assert.match(check.detail!, /FHE key unreachable: ENOTFOUND.*the engine must run on it/);
  });

  it('fails relayer.fhe-key on a non-2xx status and relayer.keyurl on a timeout', async () => {
    const missingKey = await environment({ keyStatus: 404 });
    assert.equal(
      byName(await runPreflight(profileFor(missingKey.url, missingKey.url)))['relayer.fhe-key']!.detail,
      'HTTP 404',
    );
    const slow = await environment({ keyUrlDelayMs: 1_000 });
    const check = byName(await runPreflight(profileFor(slow.url, slow.url), { timeoutMs: 200 }))['relayer.keyurl']!;
    assert.match(check.detail!, /relayer unreachable: no answer \(timeout\)/);
  });

  it('is skipped on a profile without chain or relayer section', async () => {
    const result = await runPreflight(loadProfile('none', { profilesDir: FHE_PROFILES }));
    assert.deepEqual(result, {
      status: 'skipped',
      reason: "profile 'none' has no chain or relayer section",
      checks: [],
    });
  });

  it('redacts secret URLs and the API key from the result', async () => {
    const env = await environment({ chainId: 1, keyUrlStatus: 403 });
    const profile = profileFor(env.url, env.url, API_KEY);
    profile.chain!.rpcUrl = { value: `${env.url}/rpc-secret-path`, secret: true, source: 'env:RPC' };
    const result = redactedPreflight(await runPreflight(profile), createRedactor(secretsOf(profile)));
    const printed = JSON.stringify(result);
    assert.ok(!printed.includes('rpc-secret-path'));
    assert.ok(!printed.includes(API_KEY));
    assert.equal(byName(result)['chain.rpc']!.target, '[REDACTED:chain.rpcUrl]');
  });

  it('fails contracts.code when a protocol contract of the profile has no code on the chain', async () => {
    const env = await environment({
      emptyAddresses: ['0x0000000000000000000000000000000000000004', '0x0000000000000000000000000000000000000007'],
    });
    const result = await runPreflight(profileFor(env.url, env.url));
    assert.equal(result.status, 'failed');
    const check = byName(result)['contracts.code']!;
    assert.equal(check.status, 'failed');
    assert.match(
      check.detail!,
      /^no contract at contracts\.executor \(0x0+7\), contracts\.inputVerifier \(0x0+4\): the profile does not match this chain$/,
    );
  });
});
