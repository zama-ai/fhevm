import assert from 'node:assert/strict';
import { describe, it } from 'node:test';

import { relayerBaseUrl } from '../../../src/config/profiles/profile.js';
import { FheClient, RelayerCallError } from '../../../src/runtime/modules/fhe/client.js';
import { API_KEY, type RelayerOptions as Options, address, config, fakeSdk, provider } from './fake-sdk.js';

const permit = { transportKeyPair: {} as never, signedPermit: {} as never };

/** Runs every relayer operation of the client once. */
async function everyRelayerCall(client: FheClient): Promise<void> {
  await client.encrypt({
    contractAddress: address(9),
    userAddress: address(8),
    values: [{ type: 'uint64', value: 7n }],
  });
  await client.decrypt({ handles: ['0xaa'], contractAddress: address(9), ...permit });
  await client.publicDecrypt(['0xaa']);
}

describe('FHE client', () => {
  it('strips the API version from the relayer URL', () => {
    assert.equal(relayerBaseUrl('http://relayer:3000/v2'), 'http://relayer:3000');
    assert.equal(relayerBaseUrl('https://relayer.example/v1/'), 'https://relayer.example');
    assert.equal(relayerBaseUrl('https://relayer.example'), 'https://relayer.example');
  });

  it('builds the SDK chain from the sections and configures the runtime once, with the thread mode', async () => {
    const fake = fakeSdk();
    const client = await FheClient.create(config(), provider, { sdk: fake.sdk, singleThread: true });
    await FheClient.create(config(), provider, { sdk: fake.sdk });
    assert.equal(fake.runtimeConfigs.length, 1);
    assert.equal(fake.runtimeConfigs[0]!.singleThread, true);
    assert.ok(!('auth' in fake.runtimeConfigs[0]!), 'no auth without an API key');
    const chain = fake.chain() as { id: number; fhevm: Record<string, any> };
    assert.equal(chain.id, 12345);
    assert.equal(chain.fhevm.relayerUrl, 'http://fhevm-relayer:3000');
    assert.equal(chain.fhevm.gateway.id, 54321);
    assert.equal(chain.fhevm.gateway.contracts.inputVerification.address, address(2));
    assert.equal(chain.fhevm.contracts.protocolConfig.address, address(6));
    assert.equal(client.authenticated, false);
    assert.ok(client.readyMs >= 0);
  });

  it('sends the API key on every relayer call and in the runtime configuration', async () => {
    const fake = fakeSdk();
    const client = await FheClient.create(config(API_KEY), provider, { sdk: fake.sdk });
    const auth = { type: 'ApiKeyHeader', value: API_KEY };
    assert.deepEqual(fake.runtimeConfigs[0]!.auth, auth);
    await everyRelayerCall(client);
    await client.canUseUnifiedPermit();
    const relayerCalls = fake.calls.filter((call) => call.method !== 'canUseUnifiedPermit');
    assert.equal(relayerCalls.length, 3);
    for (const call of relayerCalls) assert.deepEqual((call.parameters.options as Options).auth, auth, call.method);
    assert.deepEqual(fake.calls.at(-1)!.parameters, { auth });
    assert.equal(client.authenticated, true);
  });

  it('sends no auth at all when the API key is empty or absent', async () => {
    for (const apiKey of ['', undefined]) {
      const fake = fakeSdk();
      const client = await FheClient.create(config(apiKey), provider, { sdk: fake.sdk });
      await everyRelayerCall(client);
      await client.canUseUnifiedPermit();
      // An explicit `auth: undefined` would override the SDK's runtime default.
      for (const call of fake.calls) {
        assert.ok(!('auth' in ((call.parameters.options as object | undefined) ?? call.parameters)), call.method);
      }
    }
  });

  it('returns what the relayer reported through onProgress with each result', async () => {
    const fake = fakeSdk();
    const client = await FheClient.create(config(), provider, { sdk: fake.sdk });
    const encrypted = await client.encrypt(
      { contractAddress: address(9), userAddress: address(8), values: [{ type: 'bool', value: true }] },
      { timeoutMs: 5_000 },
    );
    assert.deepEqual(encrypted.result, { handles: [`0x${'0a'.repeat(31)}00`], inputProof: '0x0102030405' });
    assert.deepEqual(encrypted.trace, {
      events: 2,
      operation: 'INPUT_PROOF',
      status: 'succeeded',
      jobId: 'job-1',
      requestId: 'req-1',
      httpStatus: 200,
    });
    assert.equal((fake.calls[0]!.parameters.options as Options).timeout, 5_000);
    const decrypted = await client.decrypt({ handles: ['0xaa'], contractAddress: address(9), ...permit });
    assert.deepEqual(decrypted.result, [{ type: 'uint64', value: 49n }]);
    const opened = await client.publicDecrypt(['0xaa']);
    assert.deepEqual(opened.result, {
      values: [{ type: 'uint64', value: 49n }],
      abiEncodedClearValues: '0xabi',
      decryptionProof: '0x0d0e',
    });
  });

  it('keeps the trace of a failed call, relayer error included', async () => {
    const failure = new Error('relayer said no');
    const events = [
      { type: 'queued', operation: 'USER_DECRYPT', jobId: 'job-9', requestId: 'req-9', status: 202 },
      {
        type: 'failed',
        operation: 'USER_DECRYPT',
        status: 400,
        relayerApiError: { label: 'not_allowed_on_host_acl', message: 'not allowed' },
      },
    ];
    const fake = fakeSdk({ events, fail: failure });
    const client = await FheClient.create(config(), provider, { sdk: fake.sdk });
    await assert.rejects(
      client.decrypt({ handles: ['0xaa'], contractAddress: address(9), ...permit }),
      (error: unknown) => {
        assert.ok(error instanceof RelayerCallError);
        assert.equal(error.message, 'user decryption failed: relayer said no');
        assert.equal(error.cause, failure);
        assert.deepEqual(error.trace, {
          events: 2,
          operation: 'USER_DECRYPT',
          status: 'failed',
          jobId: 'job-9',
          requestId: 'req-9',
          httpStatus: 400,
          error: { label: 'not_allowed_on_host_acl', message: 'not allowed' },
        });
        return true;
      },
    );
  });

  it('reports an empty trace when the SDK rejects before calling the relayer', async () => {
    const fake = fakeSdk({ events: [], fail: new Error('User 0x… is not authorized to decrypt handle 0x…') });
    const client = await FheClient.create(config(), provider, { sdk: fake.sdk });
    await assert.rejects(client.publicDecrypt(['0xaa']), (error: unknown) => {
      assert.ok(error instanceof RelayerCallError);
      assert.deepEqual(error.trace, { events: 0 });
      return true;
    });
  });

  it("routes the SDK's warnings and errors to the log of the client created last, without debug lines", async () => {
    const fake = fakeSdk();
    const first: string[] = [];
    const last: string[] = [];
    await FheClient.create(config(), provider, { sdk: fake.sdk, log: (line) => first.push(line) });
    await FheClient.create(config(), provider, { sdk: fake.sdk, log: (line) => last.push(line) });
    const logger = fake.runtimeConfigs[0]!.logger as Record<string, ((...args: unknown[]) => void) | undefined>;
    assert.equal(logger.debug, undefined);
    logger.warn!('slow relayer');
    logger.error!('key download failed', new Error('timeout'));
    assert.deepEqual(first, []);
    assert.deepEqual(last, ['[fhevm-sdk] warn: slow relayer', '[fhevm-sdk] error: key download failed (timeout)']);
  });

  it('signs legacy or unified permits, forwarding the delegator', async () => {
    const fake = fakeSdk();
    const client = await FheClient.create(config(), provider, { sdk: fake.sdk });
    const parameters = {
      contractAddresses: [address(9)],
      startTimestamp: 1,
      durationSeconds: 60,
      signer: {} as never,
      signerAddress: address(8),
      delegatorAddress: address(7),
      transportKeyPair: {} as never,
    };
    assert.deepEqual(await client.signPermit('legacy', parameters), { id: 1, version: 1 });
    assert.deepEqual(await client.signPermit('unified', parameters), { id: 2, version: 2 });
    assert.deepEqual(
      fake.calls.map((call) => [call.method, call.parameters.delegatorAddress]),
      [
        ['signLegacyDecryptionPermit', address(7)],
        ['signUnifiedDecryptionPermit', address(7)],
      ],
    );
  });
});
