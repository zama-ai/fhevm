import { HDNodeWallet } from 'ethers';
import assert from 'node:assert/strict';
import { describe, it } from 'node:test';

import { SDK_VERSION } from '../../../src/runtime/modules/fhe/client.js';
import { Fhe, FheError, type FheOptions } from '../../../src/runtime/modules/fhe/fhe.js';
import { ANVIL_MNEMONIC } from '../../helpers.js';
import { API_KEY, type FakeBehaviour, address, contracts, fakeSdk, provider, relayer } from './fake-sdk.js';

const ROLES: Record<string, number> = { alice: 0, bob: 1, carol: 2 };
const wallets = Object.fromEntries(
  Object.entries(ROLES).map(([role, index]) => [
    role,
    HDNodeWallet.fromPhrase(ANVIL_MNEMONIC, undefined, `m/44'/60'/0'/0/${index}`),
  ]),
);
const CONTRACT = address(9);
const OTHER_CONTRACT = address(10);
const HANDLE = `0x${'ab'.repeat(32)}`;

interface Entry {
  kind: string;
  data: Record<string, any>;
  label?: string;
}

/** An Fhe module on the fake SDK, with recorded evidence and a settable clock (seconds). */
function setup(behaviour: FakeBehaviour = {}, overrides: Partial<FheOptions> = {}) {
  const fake = fakeSdk(behaviour);
  const entries: Entry[] = [];
  const clock = { now: 1_000_000 };
  const fhe = new Fhe({
    profileName: 'fx-fhe',
    relayer: relayer(),
    contracts,
    chain: {
      chainId: 12345,
      provider,
      wallet: (role: string) => {
        const wallet = wallets[role];
        if (!wallet) throw new Error(`unknown wallet role '${role}'`);
        return wallet as never;
      },
    },
    evidence: { record: (kind, data, options) => void entries.push({ kind, data, label: options?.label }) },
    sdk: fake.sdk,
    nowSeconds: () => clock.now,
    ...overrides,
  });
  const ofKind = (kind: string) => entries.filter((entry) => entry.kind === kind);
  const count = (method: string) => fake.calls.filter((call) => call.method === method).length;
  return { fhe, fake, entries, clock, ofKind, count };
}

describe('FHE module', () => {
  it('encrypts for a wallet role and records fhe.encrypt with the relayer job ID', async () => {
    const { fhe, fake, ofKind } = setup();
    const contract = { getAddress: async () => CONTRACT };
    const input = await fhe.encrypt({
      contract,
      user: 'alice',
      values: [
        { type: 'uint64', value: 7n },
        { type: 'bool', value: true },
      ],
    });
    assert.equal(input.handles.length, 2);
    assert.ok(input.handles[0] instanceof Uint8Array);
    assert.deepEqual([...input.inputProof], [1, 2, 3, 4, 5]);
    assert.deepEqual(fake.calls[0]!.parameters.userAddress, wallets.alice!.address);
    assert.deepEqual(fake.calls[0]!.parameters.contractAddress, CONTRACT);
    const [entry] = ofKind('fhe.encrypt');
    assert.equal(entry!.label, 'encrypt uint64, bool for alice');
    assert.deepEqual(entry!.data.user, { role: 'alice', address: wallets.alice!.address });
    assert.deepEqual(entry!.data.types, ['uint64', 'bool']);
    assert.equal(entry!.data.handles.length, 2);
    assert.equal(entry!.data.inputProofBytes, 5);
    assert.deepEqual(entry!.data.relayer, {
      operation: 'INPUT_PROOF',
      jobId: 'job-1',
      requestId: 'req-1',
      status: 'succeeded',
      httpStatus: 200,
    });
    assert.equal(entry!.data.status, 'success');
    assert.equal(typeof entry!.data.durationMs, 'number');
  });

  it('creates the SDK client once, on first use, and records fhe.client', async () => {
    const { fhe, entries, ofKind } = setup();
    assert.equal(entries.length, 0, 'nothing happens before the first operation');
    await fhe.encrypt({ contract: CONTRACT, user: 'alice', values: [{ type: 'uint8', value: 1 }] });
    await fhe.publicDecrypt([HANDLE]);
    const clients = ofKind('fhe.client');
    assert.equal(clients.length, 1);
    assert.deepEqual(
      { ...clients[0]!.data, readyMs: 0 },
      {
        relayerUrl: 'http://fhevm-relayer:3000/v2',
        chainId: 12345,
        gatewayChainId: 54321,
        sdkVersion: SDK_VERSION,
        authenticated: false,
        threads: 'pool',
        readyMs: 0,
        status: 'success',
      },
    );
  });

  it('says an API key is used without ever recording it', async () => {
    const { fhe, entries } = setup({}, { relayer: relayer(API_KEY) });
    await fhe.encrypt({ contract: CONTRACT, user: 'alice', values: [{ type: 'uint8', value: 1 }] });
    await fhe.userDecrypt({ handle: HANDLE, contract: CONTRACT, user: 'alice' });
    assert.equal(entries[0]!.data.authenticated, true);
    assert.ok(!JSON.stringify(entries).includes(API_KEY));
  });

  it('user-decrypts one handle or a batch, returning bigint values in order', async () => {
    const { fhe, fake, ofKind } = setup({ clear: { type: 'uint32', value: 49 } });
    const one = await fhe.userDecrypt({ handle: HANDLE, contract: CONTRACT, user: 'alice' });
    assert.equal(one, 49n);
    const bytes = new Uint8Array(32).fill(0xcd);
    const many = await fhe.userDecrypt({ handles: [HANDLE, bytes], contract: CONTRACT, user: 'alice' });
    assert.deepEqual(many, [49n, 49n]);
    const decryptCall = fake.calls.filter((call) => call.method === 'decryptValues').at(-1)!;
    assert.deepEqual(decryptCall.parameters.encryptedValues, [HANDLE, `0x${'cd'.repeat(32)}`]);
    const entry = ofKind('fhe.user-decrypt').at(-1)!;
    assert.deepEqual(entry.data.values, [
      { type: 'uint32', value: '49' },
      { type: 'uint32', value: '49' },
    ]);
    assert.equal(entry.data.permit, 'legacy');
    assert.equal(entry.label, 'user decryption by alice of 2 handle(s)');
  });

  it('caches one keypair per wallet and one permit per wallet, permit kind and contract', async () => {
    const { fhe, ofKind, count } = setup();
    await fhe.userDecrypt({ handle: HANDLE, contract: CONTRACT, user: 'alice' });
    await fhe.userDecrypt({ handle: HANDLE, contract: CONTRACT, user: 'alice' });
    assert.equal(count('generateTransportKeyPair'), 1);
    assert.equal(count('signLegacyDecryptionPermit'), 1);
    await fhe.userDecrypt({ handle: HANDLE, contract: OTHER_CONTRACT, user: 'alice' });
    await fhe.userDecrypt({ handle: HANDLE, contract: CONTRACT, user: 'alice', permit: 'unified' });
    assert.equal(count('generateTransportKeyPair'), 1, 'the wallet keeps its keypair');
    assert.equal(count('signLegacyDecryptionPermit'), 2);
    assert.equal(count('signUnifiedDecryptionPermit'), 1);
    await fhe.userDecrypt({ handle: HANDLE, contract: CONTRACT, user: 'bob' });
    assert.equal(count('generateTransportKeyPair'), 2);
    assert.deepEqual(
      ofKind('fhe.user-decrypt').map((entry) => entry.data.cache),
      ['miss', 'hit', 'miss', 'miss', 'miss'],
    );
  });

  it('signs permits for 10 days from now and renews a cached one 60 s before it expires', async () => {
    const { fhe, fake, clock, count } = setup();
    await fhe.userDecrypt({ handle: HANDLE, contract: CONTRACT, user: 'alice' });
    const signed = fake.calls.find((call) => call.method === 'signLegacyDecryptionPermit')!.parameters;
    assert.equal(signed.startTimestamp, 1_000_000);
    assert.equal(signed.durationSeconds, 864_000);
    assert.deepEqual(signed.contractAddresses, [CONTRACT]);
    assert.equal(signed.signerAddress, wallets.alice!.address);
    clock.now += 864_000 - 61;
    await fhe.userDecrypt({ handle: HANDLE, contract: CONTRACT, user: 'alice' });
    assert.equal(count('signLegacyDecryptionPermit'), 1);
    clock.now += 1;
    await fhe.userDecrypt({ handle: HANDLE, contract: CONTRACT, user: 'alice' });
    assert.equal(count('signLegacyDecryptionPermit'), 2);
  });

  it('bypasses the cache for fresh or explicit permits, without replacing the cached one', async () => {
    const { fhe, fake, ofKind, count } = setup();
    await fhe.userDecrypt({ handle: HANDLE, contract: CONTRACT, user: 'alice' });
    await fhe.userDecrypt({ handle: HANDLE, contract: CONTRACT, user: 'alice', fresh: true });
    assert.equal(count('generateTransportKeyPair'), 2);
    await fhe.userDecrypt({
      handle: HANDLE,
      contract: CONTRACT,
      user: 'alice',
      startTimestamp: 5,
      durationSeconds: 60,
    });
    const signed = fake.calls.filter((call) => call.method === 'signLegacyDecryptionPermit').at(-1)!.parameters;
    assert.equal(signed.startTimestamp, 5);
    assert.equal(signed.durationSeconds, 60);
    await fhe.userDecrypt({
      handle: HANDLE,
      contract: CONTRACT,
      user: 'alice',
      transportKeyPair: { publicKey: 'pub', privateKey: 'priv' },
    });
    assert.deepEqual(fake.calls.find((call) => call.method === 'parseTransportKeyPair')!.parameters, {
      publicKey: 'pub',
      privateKey: 'priv',
    });
    await fhe.userDecrypt({ handle: HANDLE, contract: CONTRACT, user: 'alice' });
    assert.deepEqual(
      ofKind('fhe.user-decrypt').map((entry) => entry.data.cache),
      ['miss', 'bypassed', 'bypassed', 'bypassed', 'hit'],
    );
  });

  it('decrypts as a delegate, with the delegator in the permit and in the cache key', async () => {
    const { fhe, fake, ofKind, count } = setup();
    const value = await fhe.delegatedUserDecrypt({
      handle: HANDLE,
      contract: CONTRACT,
      delegate: 'bob',
      delegator: 'alice',
    });
    assert.equal(value, 49n);
    const signed = fake.calls.find((call) => call.method === 'signLegacyDecryptionPermit')!.parameters;
    assert.equal(signed.signerAddress, wallets.bob!.address);
    assert.equal(signed.delegatorAddress, wallets.alice!.address);
    const smartWallet = address(77);
    await fhe.delegatedUserDecrypt({ handle: HANDLE, contract: CONTRACT, delegate: 'bob', delegator: smartWallet });
    await fhe.userDecrypt({ handle: HANDLE, contract: CONTRACT, user: 'bob' });
    assert.equal(count('signLegacyDecryptionPermit'), 3, 'one permit per delegator, and one of its own');
    assert.equal(count('generateTransportKeyPair'), 1, 'bob keeps one keypair');
    const [first, second] = ofKind('fhe.delegated-decrypt');
    assert.deepEqual(first!.data.delegate, { role: 'bob', address: wallets.bob!.address });
    assert.deepEqual(first!.data.delegator, { role: 'alice', address: wallets.alice!.address });
    assert.deepEqual(second!.data.delegator, { address: smartWallet });
    assert.equal(first!.label, 'delegated decryption by bob of 1 handle(s)');
  });

  it('refuses a unified permit when the relayer has no v3 route, recording the failure', async () => {
    const { fhe, ofKind, count } = setup({ unified: false });
    await assert.rejects(
      fhe.userDecrypt({ handle: HANDLE, contract: CONTRACT, user: 'alice', permit: 'unified' }),
      (error: unknown) => error instanceof FheError && /does not offer the unified \(v3\) route/.test(error.message),
    );
    assert.equal(count('decryptValues'), 0);
    assert.equal(ofKind('fhe.user-decrypt')[0]!.data.status, 'failed');
  });

  it('records a relayer rejection before failing, and exposes its label and job ID', async () => {
    const events = [
      { type: 'queued', operation: 'USER_DECRYPT', jobId: 'job-9', requestId: 'req-9', status: 202 },
      {
        type: 'failed',
        operation: 'USER_DECRYPT',
        status: 400,
        relayerApiError: { label: 'not_allowed_on_host_acl', message: 'not allowed' },
      },
    ];
    const { fhe, ofKind } = setup({ events, fail: new Error('relayer rejected the request') });
    await assert.rejects(fhe.userDecrypt({ handle: HANDLE, contract: CONTRACT, user: 'carol' }), (error: unknown) => {
      assert.ok(error instanceof FheError);
      assert.equal(error.operation, 'fhe.user-decrypt');
      assert.equal(error.relayerLabel, 'not_allowed_on_host_acl');
      assert.equal(error.jobId, 'job-9');
      assert.equal(
        error.message,
        'user decryption by carol of 1 handle(s) failed (not_allowed_on_host_acl): relayer rejected the request',
      );
      return true;
    });
    const entry = ofKind('fhe.user-decrypt')[0]!;
    assert.equal(entry.data.status, 'failed');
    assert.equal(entry.data.cache, 'miss', 'a failure says where its permit came from');
    assert.equal(entry.data.error, 'relayer rejected the request');
    assert.deepEqual(entry.data.relayer.error, { label: 'not_allowed_on_host_acl', message: 'not allowed' });
  });

  it('records a failure the SDK detects before calling the relayer, without relayer fields', async () => {
    const { fhe, ofKind } = setup({ events: [], fail: new Error('User 0x… is not authorized to decrypt handle 0x…') });
    await assert.rejects(fhe.userDecrypt({ handle: HANDLE, contract: CONTRACT, user: 'carol' }), (error: unknown) => {
      assert.ok(error instanceof FheError);
      assert.equal(error.relayerLabel, undefined);
      assert.equal(error.jobId, undefined);
      assert.match(error.message, /is not authorized to decrypt handle/);
      return true;
    });
    const entry = ofKind('fhe.user-decrypt')[0]!;
    assert.equal(entry.data.status, 'failed');
    assert.ok(!('relayer' in entry.data));
  });

  it('records a relayer timeout as status timeout', async () => {
    const events = [{ type: 'timeout', operation: 'PUBLIC_DECRYPT', jobId: 'job-3', status: 202 }];
    const { fhe, ofKind } = setup({ events, fail: new Error('timed out') });
    await assert.rejects(fhe.publicDecrypt([HANDLE]), FheError);
    assert.equal(ofKind('fhe.public-decrypt')[0]!.data.status, 'timeout');
  });

  it('records a failed client creation and fails with the profile name', async () => {
    const { fhe, ofKind } = setup({ readyError: new Error('ENOTFOUND minio') });
    await assert.rejects(
      fhe.encrypt({ contract: CONTRACT, user: 'alice', values: [{ type: 'uint8', value: 1 }] }),
      /profile 'fx-fhe': the FHE client could not be created: ENOTFOUND minio/,
    );
    assert.equal(ofKind('fhe.client')[0]!.data.status, 'failed');
    assert.equal(ofKind('fhe.encrypt').length, 0);
  });

  it('public-decrypts with the values and what checkSignatures needs', async () => {
    const { fhe, ofKind } = setup({ clear: { type: 'bool', value: true } });
    const result = await fhe.publicDecrypt([HANDLE, HANDLE]);
    assert.deepEqual(result, { values: [true, true], abiEncodedClearValues: '0xabi', decryptionProof: '0x0d0e' });
    const entry = ofKind('fhe.public-decrypt')[0]!;
    assert.equal(entry.data.decryptionProofBytes, 2);
    assert.deepEqual(entry.data.values, [
      { type: 'bool', value: true },
      { type: 'bool', value: true },
    ]);
  });

  it('never records keypairs or permits', async () => {
    const { fhe, entries } = setup();
    await fhe.userDecrypt({ handle: HANDLE, contract: CONTRACT, user: 'alice' });
    await fhe.delegatedUserDecrypt({ handle: HANDLE, contract: CONTRACT, delegate: 'bob', delegator: 'alice' });
    const recorded = JSON.stringify(entries);
    for (const field of ['transportKeyPair', 'signedPermit', 'privateKey', 'signature']) {
      assert.ok(!recorded.includes(field), `evidence holds ${field}`);
    }
  });

  it('drops the cache when destroyed', async () => {
    const { fhe, count, ofKind } = setup();
    await fhe.userDecrypt({ handle: HANDLE, contract: CONTRACT, user: 'alice' });
    fhe.destroy();
    await fhe.userDecrypt({ handle: HANDLE, contract: CONTRACT, user: 'alice' });
    assert.equal(count('signLegacyDecryptionPermit'), 2);
    assert.equal(ofKind('fhe.client').length, 2);
  });
});
