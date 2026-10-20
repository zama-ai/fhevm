// Plays the KMS connector against every coprocessor's proxy until the RFC-038 pad connector and its
// e2e land. Remove this suite then, with the test-suite env only it reads: COPROCESSOR_PROXY_URLS,
// COPROCESSOR_SIGNER_ADDRESSES, PRF_SERVICE_FAKE_COUNT_URLS, COPROCESSOR_PROXY_METRICS_URLS,
// COPROCESSOR_PROXY_CA_CERT and the mapping of KMS party 1's KMS_CONNECTOR_PRIVATE_KEY, and with
// the steps of fhevm-cli's kms-context-switch profile that run its "across a KMS context switch" tests.
// ci/preview-env/scripts/e2e/check-coprocessor-proxy.sh loads the proxies through its "through a
// rolling restart" test, and needs another client then.
//
// The suite signs as party 1's connector, a tx-sender of the active KMS context, for each
// coprocessor's signer. Behind each proxy runs the fake prf-service of
// test-suite/e2e/scripts/fake-prf-service.ts, which counts the PRF requests it receives.
import { expect } from 'chai';
import { type BaseWallet, Signature, Wallet, concat, toBeHex } from 'ethers';
import { readFileSync } from 'node:fs';
import type { IncomingHttpHeaders } from 'node:http';
import https from 'node:https';

import { fhevmSigHeader } from '../sdk/connector/connectorHttp';

const PRF_PATH = '/v1/prf/0/labels/00/1';
// What the fake prf-service answers on the PRF route.
const FAKE_PRF_OUTPUT = 'fake-prf-output';
const FAKE_PRF_ATTESTATION = 'fake-prf-attestation';
// Order of the secp256k1 group: `s` and `n - s` both sign the same message.
const SECP256K1_N = 0xfffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141n;

const envList = (name: string): string[] =>
  (process.env[name] ?? '')
    .split(',')
    .map((item) => item.trim())
    .filter(Boolean);

interface Sent {
  status: number;
  headers: IncomingHttpHeaders;
  body: string;
  /** PRF requests that reached the fake prf-service while this one was answered. */
  forwarded: number;
  where: string;
}

/** GET over TLS, trusting the proxies' self-signed test certificate. */
function get(url: string, authorization: string): Promise<Omit<Sent, 'forwarded' | 'where'>> {
  const ca = readFileSync(process.env.COPROCESSOR_PROXY_CA_CERT!);
  return new Promise((resolve, reject) => {
    const req = https
      .get(url, { ca, headers: { authorization }, timeout: 30_000 }, (res) => {
        let body = '';
        res.setEncoding('utf8');
        res.on('data', (chunk) => (body += chunk));
        res.on('end', () => resolve({ status: res.statusCode ?? 0, headers: res.headers, body }));
      })
      .on('error', (err) => reject(new Error(`${err.message}${req.reusedSocket ? ' on a reused connection' : ''}`)));
  });
}

async function prfRequests(countUrl: string): Promise<number> {
  const resp = await fetch(countUrl, { signal: AbortSignal.timeout(10_000) });
  return ((await resp.json()) as { prfRequests: number }).prfRequests;
}

/** Unix second at which the proxy last confirmed its KMS tx-senders at the finalized block. */
async function signersReadAt(metricsUrl: string): Promise<number> {
  const resp = await fetch(metricsUrl, { signal: AbortSignal.timeout(10_000) });
  const gauge = (await resp.text())
    .split('\n')
    .find((line) => line.startsWith('coprocessor_kms_tx_senders_read_timestamp_seconds '));
  return Number(gauge?.split(' ')[1] ?? 0);
}

async function waitForSignersReadAfter(metricsUrl: string, after: number): Promise<number> {
  const deadline = Date.now() + 60_000;
  for (;;) {
    const readAt = await signersReadAt(metricsUrl);
    if (readAt > after) return readAt;
    if (Date.now() > deadline) throw new Error(`${metricsUrl}: no read of the KMS tx-senders after ${after}`);
    await new Promise((resolve) => setTimeout(resolve, 500));
  }
}

/**
 * Waits for a read of the KMS tx-senders that started after `after`. The gauge is set when a read
 * ends, so the first read seen after `after` may have started before it; the next one did not.
 */
async function waitForSignersReadStartedAfter(metricsUrl: string, after: number) {
  await waitForSignersReadAfter(metricsUrl, await waitForSignersReadAfter(metricsUrl, after));
}

/** The high-s twin of the header's signature: it recovers the same signer, and EIP-2 refuses it. */
function withHighS(header: string): string {
  const [params, sig] = header.split('sig=');
  const { r, s, v } = Signature.from(sig);
  return `${params}sig=${concat([r, toBeHex(SECP256K1_N - BigInt(s), 32), toBeHex(v === 27 ? 28 : 27, 1)])}`;
}

function expectRefused(sent: Sent, status: number, code: string, retryable: boolean) {
  expect(sent.status, sent.where).to.equal(status);
  const body = JSON.parse(sent.body) as { code: string; retryable: boolean };
  expect(body.code, sent.where).to.equal(code);
  expect(body.retryable, sent.where).to.equal(retryable);
  expect(sent.forwarded, sent.where).to.equal(0);
}

describe('Coprocessor proxy', function () {
  this.timeout(60_000);
  const urls = envList('COPROCESSOR_PROXY_URLS');
  const signers = envList('COPROCESSOR_SIGNER_ADDRESSES');
  const countUrls = envList('PRF_SERVICE_FAKE_COUNT_URLS');
  const metricsUrls = envList('COPROCESSOR_PROXY_METRICS_URLS');

  before(function () {
    if (!urls.length) {
      console.log('[coprocessor-proxy] COPROCESSOR_PROXY_URLS is empty; skipping suite');
      this.skip();
    }
    expect(signers).to.have.lengthOf(urls.length);
    expect(countUrls).to.have.lengthOf(urls.length);
    expect(metricsUrls).to.have.lengthOf(urls.length);
  });

  /** `path`, signed by party 1's connector for coprocessor `i` unless `opts` says otherwise. */
  const sign = (i: number, path: string, opts?: { signer?: BaseWallet; expires?: number }) =>
    fhevmSigHeader(path, '', signers[i], { signer: new Wallet(process.env.KMS_CONNECTOR_PRIVATE_KEY!), ...opts });

  async function send(i: number, path: string, authorization: string): Promise<Sent> {
    const before = await prfRequests(countUrls[i]);
    const reply = await get(`${urls[i]}${path}`, authorization);
    const forwarded = (await prfRequests(countUrls[i])) - before;
    return { ...reply, forwarded, where: `${urls[i]}${path} -> ${reply.status} ${reply.body}` };
  }

  it('forwards a request a KMS tx-sender signed and returns the answer unchanged', async function () {
    for (const i of urls.keys()) {
      const sent = await send(i, PRF_PATH, await sign(i, PRF_PATH));
      expect(sent.status, sent.where).to.equal(200);
      expect(sent.body, sent.where).to.equal(FAKE_PRF_OUTPUT);
      expect(sent.headers['x-prf-attestation'], sent.where).to.equal(FAKE_PRF_ATTESTATION);
      expect(sent.forwarded, sent.where).to.equal(1);
    }
  });

  it('refuses a bad signature with 401', async function () {
    for (const i of urls.keys()) {
      const sent = await send(i, PRF_PATH, withHighS(await sign(i, PRF_PATH)));
      expectRefused(sent, 401, 'auth_invalid_signature', false);
    }
  });

  it('refuses an expired signature with 401, retryable after signing again', async function () {
    const expires = Math.floor(Date.now() / 1000) - 600;
    for (const i of urls.keys()) {
      const sent = await send(i, PRF_PATH, await sign(i, PRF_PATH, { expires }));
      expectRefused(sent, 401, 'auth_expired', true);
    }
  });

  it('tells a signer still unknown after a fresh read to retry in a second', async function () {
    for (const i of urls.keys()) {
      const sent = await send(i, PRF_PATH, await sign(i, PRF_PATH, { signer: Wallet.createRandom() }));
      expectRefused(sent, 502, 'upstream_transient', true);
      expect(sent.headers['retry-after'], sent.where).to.equal('1');
    }
  });

  it('answers 404 to a route it does not forward', async function () {
    const path = '/v1/unknown';
    for (const i of urls.keys()) {
      const sent = await send(i, path, await sign(i, path));
      expectRefused(sent, 404, 'not_found', false);
    }
  });

  // Run by fhevm-cli's kms-context-switch profile around its node swap, which passes the tx-sender
  // key of a KMS node in COPROCESSOR_PROXY_SWITCH_SIGNER_KEY.
  describe('across a KMS context switch', function () {
    // The refusal waits for two reads of the KMS tx-senders per proxy.
    this.timeout(180_000);
    const switchSigner = () => new Wallet(process.env.COPROCESSOR_PROXY_SWITCH_SIGNER_KEY!);

    before(function () {
      if (!process.env.COPROCESSOR_PROXY_SWITCH_SIGNER_KEY) this.skip();
    });

    it('accepts a node that is only in the new Active context', async function () {
      for (const i of urls.keys()) {
        const sent = await send(i, PRF_PATH, await sign(i, PRF_PATH, { signer: switchSigner() }));
        expect(sent.status, sent.where).to.equal(200);
        expect(sent.body, sent.where).to.equal(FAKE_PRF_OUTPUT);
        expect(sent.forwarded, sent.where).to.equal(1);
      }
    });

    it('refuses a node that is only in the destroyed context', async function () {
      // The Unix second at which the profile saw the destroy finalized.
      const finalizedAt = Number(process.env.COPROCESSOR_PROXY_SWITCH_FINALIZED_AT);
      expect(Number.isInteger(finalizedAt), 'COPROCESSOR_PROXY_SWITCH_FINALIZED_AT').to.equal(true);
      for (const i of urls.keys()) {
        await waitForSignersReadStartedAfter(metricsUrls[i], finalizedAt);
        const sent = await send(i, PRF_PATH, await sign(i, PRF_PATH, { signer: switchSigner() }));
        expectRefused(sent, 502, 'upstream_transient', true);
        expect(sent.headers['retry-after'], sent.where).to.equal('1');
      }
    });
  });

  // Run by ci/preview-env/scripts/e2e/check-coprocessor-proxy.sh, which restarts every proxy
  // Deployment once this test logs that its load started.
  describe('through a rolling restart', function () {
    const loadSeconds = Number(process.env.COPROCESSOR_PROXY_LOAD_SECONDS ?? 0);

    before(function () {
      if (!loadSeconds) this.skip();
    });

    it('answers every signed request while its pods are replaced', async function () {
      this.timeout((loadSeconds + 120) * 1000);
      const countsBefore = await Promise.all(countUrls.map(prfRequests));
      const failures: string[] = [];
      let sent = 0;
      console.log('[coprocessor-proxy] load started');
      for (const end = Date.now() + loadSeconds * 1000; Date.now() < end; sent += 1) {
        const i = sent % urls.length;
        try {
          const reply = await get(`${urls[i]}${PRF_PATH}`, await sign(i, PRF_PATH));
          if (reply.status !== 200) failures.push(`${urls[i]} -> ${reply.status} ${reply.body}`);
        } catch (err) {
          failures.push(`${urls[i]} -> ${err}`);
        }
      }
      const countsAfter = await Promise.all(countUrls.map(prfRequests));
      const forwarded = countsAfter.reduce((sum, count, i) => sum + count - countsBefore[i], 0);
      console.log(`[coprocessor-proxy] load ended: ${sent} requests, ${failures.length} failed`);
      expect(failures, failures.slice(0, 10).join('\n')).to.deep.equal([]);
      expect(forwarded).to.equal(sent);
    });
  });
});
