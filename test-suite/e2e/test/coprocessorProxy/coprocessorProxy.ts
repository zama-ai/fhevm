// Plays the KMS connector against every coprocessor's proxy until the RFC-038 pad connector and its
// e2e land. Remove this suite then, with the test-suite env only it reads: COPROCESSOR_PROXY_URLS,
// COPROCESSOR_SIGNER_ADDRESSES, PRF_SERVICE_FAKE_COUNT_URLS, COPROCESSOR_PROXY_CA_CERT and the
// mapping of KMS party 1's KMS_CONNECTOR_PRIVATE_KEY.
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
    https
      .get(url, { ca, headers: { authorization }, timeout: 30_000 }, (res) => {
        let body = '';
        res.setEncoding('utf8');
        res.on('data', (chunk) => (body += chunk));
        res.on('end', () => resolve({ status: res.statusCode ?? 0, headers: res.headers, body }));
      })
      .on('error', reject);
  });
}

async function prfRequests(countUrl: string): Promise<number> {
  const resp = await fetch(countUrl, { signal: AbortSignal.timeout(10_000) });
  return ((await resp.json()) as { prfRequests: number }).prfRequests;
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

  before(function () {
    if (!urls.length) {
      console.log('[coprocessor-proxy] COPROCESSOR_PROXY_URLS is empty; skipping suite');
      this.skip();
    }
    expect(signers).to.have.lengthOf(urls.length);
    expect(countUrls).to.have.lengthOf(urls.length);
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
});
