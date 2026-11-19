// Signs as the KMS connectors against the PRF endpoint of each coprocessor, until the pad connector lands.
import { expect } from 'chai';
import { Wallet, hexlify, keccak256, randomBytes } from 'ethers';
import { Pool } from 'pg';

import { type Reply, envList, metric, request } from '../coprocessorProxy/client';
import { type PrfOutputAttestation, type PrfOutputRef, prfPath, verifyPrfOutput } from '../coprocessorProxy/prfOutput';
import { fhevmSigHeader } from '../sdk/connector/connectorHttp';

// The e2e limits in the compose file.
const MAX_PENDING_BITS = 2048;
const MAX_IN_FLIGHT = 8;
const MAX_REQUESTS_PER_SIGNER = 20;
const MAX_PAD_LENGTH = 2048;
// A complete output answers in less time. A tfhe-rs evaluation takes more.
const AT_ONCE_MS = 2_000;

const sleep = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));
const newRef = (padLength: number, prfId = 0): PrfOutputRef => ({ prfId, label: hexlify(randomBytes(32)), padLength });
const attestationOf = (reply: Reply) => JSON.parse(String(reply.headers['x-prf-attestation'])) as PrfOutputAttestation;
const errorOf = (reply: Reply) => JSON.parse(reply.body.toString()) as { code: string; retryable: boolean };

/** The S3 object at `url` and its attestation, once the pad-worker uploaded it. */
async function s3Object(url: string): Promise<{ url: string; body: Buffer; attestation: PrfOutputAttestation }> {
  const deadline = Date.now() + 120_000;
  while (true) {
    const response = await fetch(url);
    if (response.ok) {
      const attestation = JSON.parse(response.headers.get('x-amz-meta-ct-attestation') ?? 'null');
      return { url, body: Buffer.from(await response.arrayBuffer()), attestation };
    }
    if (Date.now() > deadline) throw new Error(`${url}: ${response.status} after 120 s`);
    await sleep(1_000);
  }
}

/** The `outcome` and `local_result` of the ended pad_consensus check of `ref` in `databaseUrl`. */
async function padConsensus(databaseUrl: string, ref: PrfOutputRef): Promise<[string, string]> {
  const pool = new Pool({ connectionString: databaseUrl });
  try {
    const deadline = Date.now() + 120_000;
    while (true) {
      const { rows } = await pool.query(
        `SELECT outcome, local_result FROM pad_consensus
          WHERE prf_instance = $1 AND label = $2 AND pad_length = $3 AND done_at IS NOT NULL`,
        [ref.prfId, Buffer.from(ref.label.slice(2), 'hex'), ref.padLength],
      );
      if (rows.length) return [rows[0].outcome, rows[0].local_result];
      if (Date.now() > deadline) throw new Error(`no ended pad_consensus check of ${prfPath(ref)} after 120 s`);
      await sleep(2_000);
    }
  } finally {
    await pool.end();
  }
}

describe('PRF endpoint', function () {
  // A 2048-bit pad takes tens of seconds with tfhe-rs on CPU.
  this.timeout(600_000);
  const urls = envList('COPROCESSOR_PROXY_URLS');
  const signers = envList('COPROCESSOR_SIGNER_ADDRESSES');
  const prfMetricsUrls = envList('PRF_SERVICE_METRICS_URLS');
  const padMetricsUrls = envList('PAD_WORKER_METRICS_URLS');
  const bucketUrls = envList('COPROCESSOR_BUCKET_URLS');
  const databaseUrls = envList('COPROCESSOR_DATABASE_URLS');
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

    it('uploads both forms to S3, and the consensus-detectors find consensus', async function () {
      if (bucketUrls.length !== urls.length || databaseUrls.length !== urls.length) {
        console.log('[prf-endpoint] COPROCESSOR_BUCKET_URLS or COPROCESSOR_DATABASE_URLS is not set; skipping');
        this.skip();
      }
      const ref = newRef(64);
      const replies = await Promise.all(urls.map((_, i) => get(i, ref)));
      const key = `${ref.prfId}/${ref.label.slice(2)}/${ref.padLength}`;
      for (const i of urls.keys()) {
        expectOutput(replies[i], i, ref);
        const attestation = attestationOf(replies[i]);
        const ct128 = await s3Object(`${bucketUrls[i]}/pads/ct128/${key}`);
        const ct64 = await s3Object(`${bucketUrls[i]}/pads/ct64/${key}`);
        for (const object of [ct128, ct64]) expect(object.attestation, object.url).to.deep.equal(attestation);
        expect(ct128.body.equals(replies[i].body), ct128.url).to.equal(true);
        expect(keccak256(ct64.body), ct64.url).to.equal(attestation.ciphertext_digest);
      }
      // With one coprocessor, no other output can confirm ours.
      if (urls.length < 2) return;
      for (const databaseUrl of databaseUrls) {
        expect(await padConsensus(databaseUrl, ref), prfPath(ref)).to.deep.equal(['consensus', 'matches_quorum']);
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
        expect(Date.now() - started, again.where).to.be.below(AT_ONCE_MS);
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
          expect(Date.now() - started, joined.where).to.be.above(AT_ONCE_MS);
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
      // The whole budget, in rows still pending when the other connector arrives.
      const busy = Array.from({ length: MAX_IN_FLIGHT }, () =>
        get(0, newRef(MAX_PENDING_BITS / MAX_IN_FLIGHT)).then((reply) => {
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
