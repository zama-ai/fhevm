// Temporary KMS-connector client against every proxy and the prf-service behind it. Replace with the
// real pad-connector e2e when it lands. kms-context-switch and preview rollout checks invoke this
// suite and must switch to that client too, removing the proxy-only env and party-1 key mapping.
import { expect } from 'chai';
import { type BaseWallet, Signature, Wallet, concat, toBeHex } from 'ethers';
import { existsSync, readFileSync } from 'node:fs';
import type { IncomingHttpHeaders } from 'node:http';
import https from 'node:https';

import { fhevmSigHeader } from '../sdk/connector/connectorHttp';
import {
  type PrfOutputAttestation,
  type PrfOutputRef,
  SEEDED_BODY,
  SEEDED_CIPHERTEXT_DIGEST,
  SEEDED_REF,
  attestationDigest,
  prfPath,
  signPrfOutput,
  verifyPrfOutput,
} from './prfOutput';

// scripts/seed-prf-output.ts writes this output into each coprocessor's database.
const PRF_PATH = prfPath(SEEDED_REF);
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
  body: Buffer;
  /** The PRF requests that prf-service answered during this request. */
  forwarded: number;
  where: string;
}

/** A request over TLS, trusting the proxies' self-signed test certificate. */
function request(
  method: 'GET' | 'HEAD',
  url: string,
  authorization: string,
): Promise<Omit<Sent, 'forwarded' | 'where'>> {
  const ca = readFileSync(process.env.COPROCESSOR_PROXY_CA_CERT!);
  return new Promise((resolve, reject) => {
    const req = https
      .request(url, { method, ca, headers: { authorization }, signal: AbortSignal.timeout(30_000) }, (res) => {
        const chunks: Buffer[] = [];
        res.on('data', (chunk: Buffer) => chunks.push(chunk));
        res.on('end', () =>
          resolve({ status: res.statusCode ?? 0, headers: res.headers, body: Buffer.concat(chunks) }),
        );
      })
      .on('error', (err) => reject(new Error(`${err.message}${req.reusedSocket ? ' on a reused connection' : ''}`)));
    req.end();
  });
}

/** The sum of the `coprocessor_prf_service_requests_total` series of prf-service. */
async function prfRequests(metricsUrl: string): Promise<number> {
  const resp = await fetch(metricsUrl, { signal: AbortSignal.timeout(10_000) });
  return (await resp.text())
    .split('\n')
    .filter((line) => line.startsWith('coprocessor_prf_service_requests_total{'))
    .reduce((sum, line) => sum + Number(line.split(' ').pop()), 0);
}

/** Timestamp of the finalized block applied to the proxy's signer set. */
async function signersBlockTimestamp(metricsUrl: string): Promise<number> {
  const resp = await fetch(metricsUrl, { signal: AbortSignal.timeout(10_000) });
  const gauge = (await resp.text())
    .split('\n')
    .find((line) => line.startsWith('coprocessor_proxy_kms_tx_senders_block_timestamp_seconds '));
  return Number(gauge?.split(' ')[1] ?? 0);
}

async function waitForSignersBlock(metricsUrl: string, timestamp: number) {
  const deadline = Date.now() + 60_000;
  for (;;) {
    if ((await signersBlockTimestamp(metricsUrl)) >= timestamp) return;
    if (Date.now() > deadline)
      throw new Error(`${metricsUrl}: signer set has not reached block timestamp ${timestamp}`);
    await new Promise((resolve) => setTimeout(resolve, 500));
  }
}

/** The high-s twin of the header's signature: it recovers the same signer, and EIP-2 refuses it. */
function withHighS(header: string): string {
  const [params, sig] = header.split('sig=');
  const { r, s, v } = Signature.from(sig);
  return `${params}sig=${concat([r, toBeHex(SECP256K1_N - BigInt(s), 32), toBeHex(v === 27 ? 28 : 27, 1)])}`;
}

/** The answer of prf-service, passed through: the seeded output, as coprocessor `signer` attested it. */
function expectForwarded(sent: Sent, signer: string) {
  expect(sent.status, sent.where).to.equal(200);
  const attestation = JSON.parse(String(sent.headers['x-prf-attestation'])) as PrfOutputAttestation;
  expect(verifyPrfOutput(attestation, SEEDED_REF, sent.body, signer), sent.where).to.deep.equal([]);
  expect(sent.forwarded, sent.where).to.equal(1);
}

function expectRefused(sent: Sent, status: number, code: string, retryable: boolean) {
  expect(sent.status, sent.where).to.equal(status);
  const body = JSON.parse(sent.body.toString()) as { code: string; retryable: boolean };
  expect(body.code, sent.where).to.equal(code);
  expect(body.retryable, sent.where).to.equal(retryable);
  expect(sent.forwarded, sent.where).to.equal(0);
}

describe('Coprocessor proxy', function () {
  this.timeout(60_000);
  const urls = envList('COPROCESSOR_PROXY_URLS');
  const signers = envList('COPROCESSOR_SIGNER_ADDRESSES');
  const prfMetricsUrls = envList('PRF_SERVICE_METRICS_URLS');
  const metricsUrls = envList('COPROCESSOR_PROXY_METRICS_URLS');

  before(function () {
    if (!urls.length) {
      console.log('[coprocessor-proxy] COPROCESSOR_PROXY_URLS is empty; skipping suite');
      this.skip();
    }
    expect(signers).to.have.lengthOf(urls.length);
    expect(prfMetricsUrls).to.have.lengthOf(urls.length);
    expect(metricsUrls).to.have.lengthOf(urls.length);
  });

  /** `path`, signed by party 1's connector for coprocessor `i` unless `opts` says otherwise. */
  const sign = (i: number, path: string, opts?: { signer?: BaseWallet; expires?: number }) =>
    fhevmSigHeader(path, '', signers[i], { signer: new Wallet(process.env.KMS_CONNECTOR_PRIVATE_KEY!), ...opts });

  async function send(i: number, path: string, authorization: string, method: 'GET' | 'HEAD' = 'GET'): Promise<Sent> {
    const before = await prfRequests(prfMetricsUrls[i]);
    const reply = await request(method, `${urls[i]}${path}`, authorization);
    const forwarded = (await prfRequests(prfMetricsUrls[i])) - before;
    return { ...reply, forwarded, where: `${method} ${urls[i]}${path} -> ${reply.status} ${reply.body}` };
  }

  it('forwards a request a KMS tx-sender signed and returns the answer unchanged', async function () {
    for (const i of urls.keys()) {
      expectForwarded(await send(i, PRF_PATH, await sign(i, PRF_PATH)), signers[i]);
    }
  });

  it('forwards a signed HEAD request and returns the answer headers without a body', async function () {
    for (const i of urls.keys()) {
      const get = await send(i, PRF_PATH, await sign(i, PRF_PATH));
      expect(get.status, get.where).to.equal(200);
      const sent = await send(i, PRF_PATH, await sign(i, PRF_PATH), 'HEAD');
      expect(sent.status, sent.where).to.equal(200);
      expect(sent.headers['x-prf-attestation'], sent.where).to.equal(get.headers['x-prf-attestation']);
      expect(sent.headers['content-length'], sent.where).to.equal(String(get.body.length));
      expect(sent.body, sent.where).to.have.lengthOf(0);
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

  it('refuses a signer still unknown after a fresh read with 401', async function () {
    for (const i of urls.keys()) {
      const sent = await send(i, PRF_PATH, await sign(i, PRF_PATH, { signer: Wallet.createRandom() }));
      expectRefused(sent, 401, 'auth_unknown_signer', false);
      expect(sent.headers['retry-after'], sent.where).to.equal(undefined);
    }
  });

  it('answers 404 to a route it does not forward', async function () {
    const path = '/v1/unknown';
    for (const i of urls.keys()) {
      const sent = await send(i, path, await sign(i, path));
      expectRefused(sent, 404, 'not_found', false);
    }
  });

  // Needs no stack, but skips with the suite: the coprocessor-proxy profile fails if nothing passed.
  describe('PRF output attestation', function () {
    it('signs the digest that shared/ciphertext-attestation pins', function () {
      // v1_canonical_bytes_and_digest_pinned in shared/ciphertext-attestation/src/prf/sign.rs.
      const ref = { prfId: 3, label: `0x${'aa'.repeat(32)}`, padLength: 0x0800 };
      expect(attestationDigest(ref, `0x${'11'.repeat(32)}`, `0x${'22'.repeat(32)}`)).to.equal(
        '0x218dc88ae0721faabc1e249ae25d670d8961b9944e31e49e3c71bbcfc37dcab3',
      );
    });

    it('refuses another output, another body or another coprocessor', function () {
      const coprocessor = Wallet.createRandom();
      const attestation = signPrfOutput(coprocessor.privateKey, SEEDED_REF, SEEDED_CIPHERTEXT_DIGEST, SEEDED_BODY);
      // The first word of each problem names the check that refused.
      const refusals = (ref: PrfOutputRef, body: Uint8Array, signer: string) =>
        verifyPrfOutput(attestation, ref, body, signer).map((problem) => problem.split(' ')[0]);
      expect(refusals(SEEDED_REF, SEEDED_BODY, coprocessor.address)).to.deep.equal([]);
      const others = [
        { ...SEEDED_REF, prfId: 1 },
        { ...SEEDED_REF, label: `0x${'02'.repeat(32)}` },
        { ...SEEDED_REF, padLength: 2 },
      ];
      for (const ref of others) {
        expect(refusals(ref, SEEDED_BODY, coprocessor.address), prfPath(ref)).to.deep.equal(['signature']);
      }
      expect(refusals(SEEDED_REF, SEEDED_BODY.subarray(1), coprocessor.address)).to.deep.equal(['body']);
      expect(refusals(SEEDED_REF, SEEDED_BODY, Wallet.createRandom().address)).to.deep.equal(['signer']);
    });
  });

  // Run by fhevm-cli's kms-context-switch profile around its node swap, which passes the tx-sender
  // key of a KMS node in COPROCESSOR_PROXY_SWITCH_SIGNER_KEY.
  describe('across a KMS context switch', function () {
    this.timeout(180_000);
    const switchSigner = () => new Wallet(process.env.COPROCESSOR_PROXY_SWITCH_SIGNER_KEY!);

    before(function () {
      if (!process.env.COPROCESSOR_PROXY_SWITCH_SIGNER_KEY) this.skip();
    });

    it('accepts a node of an Active context', async function () {
      for (const i of urls.keys()) {
        expectForwarded(await send(i, PRF_PATH, await sign(i, PRF_PATH, { signer: switchSigner() })), signers[i]);
      }
    });

    // Once the proxy has read ProtocolConfig at or after COPROCESSOR_PROXY_SWITCH_BLOCK_TIMESTAMP.
    it('refuses a node of no Active context', async function () {
      const blockTimestamp = Number(process.env.COPROCESSOR_PROXY_SWITCH_BLOCK_TIMESTAMP);
      expect(
        Number.isInteger(blockTimestamp) && blockTimestamp > 0,
        'COPROCESSOR_PROXY_SWITCH_BLOCK_TIMESTAMP',
      ).to.equal(true);
      for (const i of urls.keys()) {
        await waitForSignersBlock(metricsUrls[i], blockTimestamp);
        const sent = await send(i, PRF_PATH, await sign(i, PRF_PATH, { signer: switchSigner() }));
        expectRefused(sent, 401, 'auth_unknown_signer', false);
        expect(sent.headers['retry-after'], sent.where).to.equal(undefined);
      }
    });
  });

  // Run by ci/preview-env/scripts/e2e/check-coprocessor-proxy.sh, which restarts every proxy
  // Deployment once this test logs that its load started, and creates the stop file once every old
  // pod is gone. The load lasts at most COPROCESSOR_PROXY_LOAD_SECONDS.
  describe('through a rolling restart', function () {
    const loadSeconds = Number(process.env.COPROCESSOR_PROXY_LOAD_SECONDS ?? 0);
    const stopFile = process.env.COPROCESSOR_PROXY_LOAD_STOP_FILE;
    const stopped = () => stopFile !== undefined && existsSync(stopFile);

    before(function () {
      if (!loadSeconds) this.skip();
    });

    it('answers every signed request while its pods are replaced', async function () {
      this.timeout((loadSeconds + 120) * 1000);
      const countsBefore = await Promise.all(prfMetricsUrls.map(prfRequests));
      const failures: string[] = [];
      let sent = 0;
      console.log('[coprocessor-proxy] load started');
      for (const end = Date.now() + loadSeconds * 1000; Date.now() < end && !stopped(); sent += 1) {
        const i = sent % urls.length;
        const fail = (reason: string) => failures.push(`${new Date().toISOString()} ${urls[i]} -> ${reason}`);
        try {
          const reply = await request('GET', `${urls[i]}${PRF_PATH}`, await sign(i, PRF_PATH));
          if (reply.status !== 200) fail(`${reply.status} ${reply.body}`);
        } catch (err) {
          fail(`${err}`);
        }
      }
      const countsAfter = await Promise.all(prfMetricsUrls.map(prfRequests));
      const forwarded = countsAfter.reduce((sum, count, i) => sum + count - countsBefore[i], 0);
      console.log(`[coprocessor-proxy] load ended: ${sent} requests, ${failures.length} failed`);
      expect(failures, failures.slice(0, 10).join('\n')).to.deep.equal([]);
      expect(forwarded).to.equal(sent);
    });
  });
});
