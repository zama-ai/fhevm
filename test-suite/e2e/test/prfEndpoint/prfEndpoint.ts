// Signs as the KMS connectors against the PRF endpoint of each coprocessor, until the pad connector lands.
import { expect } from 'chai';
import { Wallet, hexlify, randomBytes } from 'ethers';

import { type Reply, envList, metric, request } from '../coprocessorProxy/client';
import { type PrfOutputAttestation, type PrfOutputRef, prfPath, verifyPrfOutput } from '../coprocessorProxy/prfOutput';
import { fhevmSigHeader } from '../sdk/connector/connectorHttp';

// The e2e limits in the compose file.
const MAX_PENDING_BITS = 2048;
const MAX_IN_FLIGHT = 8;
const MAX_REQUESTS_PER_SIGNER = 20;
const MAX_PAD_LENGTH = 2048;
// Mock evaluator: 6 KiB of ct128 for each 4-bit block, and 2 s or more for each row.
const MOCK_CT128_BLOCK_BYTES = 6 * 1024;
const MOCK_LATENCY_MS = 2_000;

const sleep = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));
const newRef = (padLength: number, prfId = 0): PrfOutputRef => ({ prfId, label: hexlify(randomBytes(32)), padLength });
const attestationOf = (reply: Reply) => JSON.parse(String(reply.headers['x-prf-attestation'])) as PrfOutputAttestation;
const errorOf = (reply: Reply) => JSON.parse(reply.body.toString()) as { code: string; retryable: boolean };

describe('PRF endpoint', function () {
  this.timeout(120_000);
  const urls = envList('COPROCESSOR_PROXY_URLS');
  const signers = envList('COPROCESSOR_SIGNER_ADDRESSES');
  const prfMetricsUrls = envList('PRF_SERVICE_METRICS_URLS');
  const padMetricsUrls = envList('PAD_WORKER_METRICS_URLS');
  const connectors = envList('KMS_CONNECTOR_PRIVATE_KEYS').map((key) => new Wallet(key));

  before(function () {
    if (!urls.length) {
      console.log('[prf-endpoint] COPROCESSOR_PROXY_URLS is empty; skipping suite');
      this.skip();
    }
    expect(signers).to.have.lengthOf(urls.length);
    expect(prfMetricsUrls).to.have.lengthOf(urls.length);
    expect(padMetricsUrls).to.have.lengthOf(urls.length);
    expect(connectors.length, 'KMS_CONNECTOR_PRIVATE_KEYS').to.be.at.least(2);
  });

  /** Sends `path` to coprocessor `i`, signed by `connector`. */
  async function send(
    i: number,
    path: string,
    opts: { connector?: number; method?: string; signal?: AbortSignal } = {},
  ): Promise<Reply & { where: string }> {
    const signer = connectors[opts.connector ?? 0];
    const authorization = await fhevmSigHeader(path, '', signers[i], { signer });
    const reply = await request(opts.method ?? 'GET', `${urls[i]}${path}`, authorization, opts);
    const error = reply.status === 200 ? '' : reply.body.toString();
    return { ...reply, where: `${opts.method ?? 'GET'} ${urls[i]}${path} -> ${reply.status} ${error}` };
  }

  const get = (i: number, ref: PrfOutputRef, connector = 0) => send(i, prfPath(ref), { connector });
  const evaluations = (i: number) =>
    metric(padMetricsUrls[i], 'coprocessor_pad_worker_evaluations_total', 'prf_instance="0"');
  const forwarded = (i: number) => metric(prfMetricsUrls[i], 'coprocessor_prf_service_requests_total');
  const allEvaluations = () => Promise.all(urls.map((_, i) => evaluations(i)));

  /** A `200` with the output of `ref`, attested by coprocessor `i`. */
  function expectOutput(reply: Reply & { where: string }, i: number, ref: PrfOutputRef) {
    expect(reply.status, reply.where).to.equal(200);
    expect(verifyPrfOutput(attestationOf(reply), ref, reply.body, signers[i]), reply.where).to.deep.equal([]);
    expect(reply.body, reply.where).to.have.lengthOf(Math.ceil(ref.padLength / 4) * MOCK_CT128_BLOCK_BYTES);
  }

  function expectError(reply: Reply & { where: string }, status: number, code: string) {
    expect(reply.status, reply.where).to.equal(status);
    expect(errorOf(reply).code, reply.where).to.equal(code);
  }

  describe('normal flow', function () {
    it('serves each pad length, and every coprocessor attests the same digests', async function () {
      for (const padLength of [1, 64, 2048]) {
        const ref = newRef(padLength);
        const replies = await Promise.all(urls.map((_, i) => get(i, ref)));
        replies.forEach((reply, i) => expectOutput(reply, i, ref));
        const attestations = replies.map(attestationOf);
        for (const attestation of attestations) {
          expect(attestation.ciphertext_digest, prfPath(ref)).to.equal(attestations[0].ciphertext_digest);
          expect(attestation.sns_ciphertext_digest, prfPath(ref)).to.equal(attestations[0].sns_ciphertext_digest);
        }
        expect(new Set(attestations.map((a) => a.signer.toLowerCase())).size).to.equal(urls.length);
      }
    });

    it('answers a complete output at once, to GET and HEAD, with no new evaluation', async function () {
      const ref = newRef(64);
      const first = await Promise.all(urls.map((_, i) => get(i, ref)));
      const before = await allEvaluations();
      for (const i of urls.keys()) {
        const started = Date.now();
        const again = await get(i, ref);
        const head = await send(i, prfPath(ref), { method: 'HEAD' });
        expect(Date.now() - started, again.where).to.be.below(MOCK_LATENCY_MS);
        expect(again.status, again.where).to.equal(200);
        expect(again.body.equals(first[i].body), again.where).to.equal(true);
        expect(again.headers['x-prf-attestation'], again.where).to.equal(first[i].headers['x-prf-attestation']);
        expect(head.status, head.where).to.equal(200);
        expect(head.headers['x-prf-attestation'], head.where).to.equal(first[i].headers['x-prf-attestation']);
        expect(head.headers['content-length'], head.where).to.equal(String(first[i].body.length));
        expect(head.body, head.where).to.have.lengthOf(0);
      }
      expect(await allEvaluations()).to.deep.equal(before);
    });

    it('refuses an unknown prfId with 404 and a non-canonical path with 400', async function () {
      const label = hexlify(randomBytes(32)).slice(2);
      const nonCanonical = [
        `/v1/prf/00/labels/${label}/1`,
        `/v1/prf/0/labels/${label.toUpperCase()}/1`,
        `/v1/prf/0/labels/${label.slice(1)}/1`,
        `/v1/prf/0/labels/${label}/0`,
        `/v1/prf/0/labels/${label}/${MAX_PAD_LENGTH + 1}`,
      ];
      for (const i of urls.keys()) {
        expectError(await get(i, newRef(1, 1)), 404, 'unknown_prf');
        for (const path of nonCanonical) expectError(await send(i, path), 400, 'non_canonical_path');
      }
    });
  });

  describe('one evaluation for many requests', function () {
    it('evaluates a path once for all connectors, and a request during the evaluation joins it', async function () {
      const ref = newRef(2048);
      const before = await allEvaluations();
      await Promise.all(
        urls.map(async (_, i) => {
          const first = connectors.slice(1).map((_, c) => get(i, ref, c + 1));
          await sleep(500);
          const started = Date.now();
          const joined = await get(i, ref);
          // A complete output answers at once: this request waited for the evaluation.
          expect(Date.now() - started, joined.where).to.be.above(MOCK_LATENCY_MS / 2);
          for (const reply of [joined, ...(await Promise.all(first))]) {
            expectOutput(reply, i, ref);
            expect(reply.body.equals(joined.body), reply.where).to.equal(true);
            expect(reply.headers['x-prf-attestation'], reply.where).to.equal(joined.headers['x-prf-attestation']);
          }
        }),
      );
      expect(await allEvaluations()).to.deep.equal(before.map((count) => count + 1));
    });
  });

  // The limits apply to one prf-service, so these tests use only the first coprocessor.
  describe('budgets and fairness', function () {
    it('refuses a connector over its pending bits with 429, and admits it once its row completes', async function () {
      const full = get(0, newRef(MAX_PENDING_BITS));
      await sleep(500);
      const small = newRef(1);
      const refused = await get(0, small);
      expectError(refused, 429, 'concurrency_limited');
      expect(errorOf(refused).retryable, refused.where).to.equal(true);
      expect(refused.headers['retry-after'], refused.where).to.equal('1');
      expect((await full).status).to.equal(200);
      expectOutput(await get(0, small), 0, small);
    });

    it('refuses a connector over its requests in progress with 429', async function () {
      const replies = await Promise.all(Array.from({ length: MAX_IN_FLIGHT + 1 }, () => get(0, newRef(1))));
      const statuses = replies.map((reply) => reply.status).sort();
      expect(statuses, replies.map((r) => r.where).join('\n')).to.deep.equal([
        ...Array<number>(MAX_IN_FLIGHT).fill(200),
        429,
      ]);
      expectError(replies.find((reply) => reply.status === 429)!, 429, 'concurrency_limited');
    });

    it('refuses a second request of a connector for a label it already requests with 429', async function () {
      const ref = newRef(1);
      const replies = await Promise.all([get(0, ref), get(0, ref)]);
      const statuses = replies.map((reply) => reply.status).sort();
      expect(statuses, replies.map((r) => r.where).join('\n')).to.deep.equal([200, 429]);
      expectError(replies.find((reply) => reply.status === 429)!, 429, 'concurrency_limited');
    });

    it('lets a connector join the row of another connector without using its budget', async function () {
      const other = newRef(MAX_PENDING_BITS);
      const first = get(0, other, 1);
      await sleep(500);
      const own = newRef(MAX_PENDING_BITS);
      const [joined, added] = await Promise.all([get(0, other), get(0, own)]);
      expectOutput(joined, 0, other);
      expectOutput(added, 0, own);
      expectOutput(await first, 0, other);
    });

    it('computes the row of another connector before the remaining rows of a busy connector', async function () {
      let busyAnswered = 0;
      const busy = Array.from({ length: MAX_IN_FLIGHT }, () =>
        get(0, newRef(64)).then((reply) => {
          busyAnswered += 1;
          return reply;
        }),
      );
      await sleep(500);
      const other = await get(0, newRef(64), 1);
      const pendingAtAnswer = MAX_IN_FLIGHT - busyAnswered;
      expect(other.status, other.where).to.equal(200);
      for (const reply of await Promise.all(busy)) expect(reply.status, reply.where).to.equal(200);
      // The pad-worker computes 4 rows at once: in fair order the other row is in round 2, else in round 3.
      expect(pendingAtAnswer, 'rows of the busy connector still pending').to.be.at.least(1);
    });

    it('frees the place of a request whose connector disconnects', async function () {
      const ref = newRef(MAX_PENDING_BITS);
      const abort = new AbortController();
      const dropped = send(0, prfPath(ref), { signal: abort.signal });
      await sleep(500);
      abort.abort();
      await dropped.catch(() => undefined);
      await sleep(500);
      // If the place is still held, this request gets `429`.
      expectOutput(await get(0, ref), 0, ref);
    });
  });

  describe('spam at the proxy', function () {
    // The proxy integration tests check a request with a body: the proxy closes such a connection.
    it('refuses a query string, another method and an unknown route without forwarding', async function () {
      const path = prfPath(newRef(1));
      for (const i of urls.keys()) {
        const before = await forwarded(i);
        expectError(await send(i, `${path}?a=1`), 401, 'auth_query_string');
        expectError(await send(i, path, { method: 'DELETE' }), 405, 'method_not_allowed');
        expectError(await send(i, `${path}/more`), 404, 'not_found');
        expect(await forwarded(i), 'requests prf-service answered').to.equal(before);
      }
    });

    it('answers HEAD requests on a complete output with no new evaluation', async function () {
      const ref = newRef(64);
      await Promise.all(urls.map((_, i) => get(i, ref)));
      const before = await allEvaluations();
      await Promise.all(
        urls.flatMap((_, i) =>
          connectors.map(async (_, c) => {
            for (let n = 0; n < 5; n += 1) {
              const head = await send(i, prfPath(ref), { connector: c, method: 'HEAD' });
              expect(head.status, head.where).to.equal(200);
            }
          }),
        ),
      );
      expect(await allEvaluations()).to.deep.equal(before);
    });

    // Last test: it uses all the rate of connector 0 at the first proxy.
    it('limits the request rate of each signer with 429, and not the other signers', async function () {
      const path = prfPath(newRef(1, 1));
      const before = await forwarded(0);
      const replies = await Promise.all(Array.from({ length: 5 * MAX_REQUESTS_PER_SIGNER }, () => send(0, path)));
      const limited = replies.filter((reply) => reply.status === 429);
      expect(limited.length, 'requests over the signer rate').to.be.above(0);
      for (const reply of limited) {
        expectError(reply, 429, 'concurrency_limited');
        expect(reply.headers['retry-after'], reply.where).to.equal('1');
      }
      const passed = replies.filter((reply) => reply.status !== 429);
      for (const reply of passed) expectError(reply, 404, 'unknown_prf');
      expect(await forwarded(0), 'requests prf-service answered').to.equal(before + passed.length);
      expectError(await send(0, path, { connector: 1 }), 404, 'unknown_prf');
    });
  });
});
