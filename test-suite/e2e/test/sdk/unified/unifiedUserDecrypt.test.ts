import { expect } from 'chai';
import { afterEach, beforeEach, describe, it } from 'mocha';

import { pollJob, type UnifiedConfig } from './unifiedUserDecrypt';

describe('Unified relayer HTTP authentication', function () {
  const cfg: UnifiedConfig = {
    relayerUrl: 'https://relayer.example/tenant/v2/',
    decryptionContractAddress: '0x0000000000000000000000000000000000000001',
    apiKey: 'test-only-api-key',
  };
  let originalFetch: typeof fetch;

  beforeEach(function () {
    originalFetch = globalThis.fetch;
  });

  afterEach(function () {
    globalThis.fetch = originalFetch;
  });

  for (const status of [401, 403]) {
    it(`rejects HTTP ${status} instead of reporting a pending KMS job`, async function () {
      globalThis.fetch = async (url, init) => {
        expect(url).to.equal('https://relayer.example/tenant/v3/user-decrypt/job-1');
        expect(new Headers(init?.headers).get('x-api-key')).to.equal(cfg.apiKey);
        return new Response('Access denied', { status });
      };

      let error: unknown;
      try {
        await pollJob(cfg, 'job-1', { timeoutMs: 1000, intervalMs: 0 });
      } catch (caught) {
        error = caught;
      }
      expect(error).to.be.instanceOf(Error);
      expect((error as Error).message).to.equal(
        `Relayer authentication failed while polling user decryption (HTTP ${status})`,
      );
      expect((error as Error).message).not.to.include(cfg.apiKey);
    });
  }

  it('keeps polling authenticated pending responses until success', async function () {
    let calls = 0;
    globalThis.fetch = async (_url, init) => {
      expect(new Headers(init?.headers).get('x-api-key')).to.equal(cfg.apiKey);
      return Response.json(++calls === 1 ? { status: 'pending' } : { status: 'succeeded', result: { result: [] } });
    };

    const result = await pollJob(cfg, 'job-1', { timeoutMs: 1000, intervalMs: 0 });
    expect(result.status).to.equal('succeeded');
    expect(calls).to.equal(2);
  });

  it('preserves unauthenticated requests and terminal domain failures', async function () {
    globalThis.fetch = async (_url, init) => {
      expect(new Headers(init?.headers).has('x-api-key')).to.equal(false);
      return Response.json({ status: 'failed', error: { label: 'not_allowed_on_host_acl' } });
    };

    const result = await pollJob({ ...cfg, apiKey: undefined }, 'job-1', { timeoutMs: 1000 });
    expect(result.status).to.equal('failed');
    expect(result.errorLabel).to.equal('not_allowed_on_host_acl');
  });
});
