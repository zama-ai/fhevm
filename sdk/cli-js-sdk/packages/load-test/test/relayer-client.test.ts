import { createServer, type Server } from "node:http";
import type { AddressInfo } from "node:net";
import { afterAll, beforeAll, describe, expect, it } from "vitest";

import { RelayerClient } from "../src/relayer/client";
import type { InputProofResultJson } from "../src/relayer/types";

/**
 * Local stub of the relayer v2 surface: POSTs return 202 envelopes, job GETs
 * walk a scripted sequence of responses per job id.
 */
type ScriptedResponse = Readonly<{
  status: number;
  body?: unknown;
  retryAfterSec?: number;
}>;

const scripts = new Map<string, ScriptedResponse[]>();
let server: Server;
let client: RelayerClient;
let clientV3: RelayerClient;

beforeAll(async () => {
  server = createServer((request, response) => {
    const url = request.url ?? "";
    if (
      request.method === "POST" &&
      (url === "/v2/input-proof" || url === "/v3/input-proof")
    ) {
      let raw = "";
      request.on("data", (chunk: Buffer) => (raw += chunk.toString()));
      request.on("end", () => {
        const body = JSON.parse(raw) as { userAddress?: string };
        if (body.userAddress === "reject") {
          response.writeHead(400, { "content-type": "application/json" });
          response.end(JSON.stringify({ error: { label: "validation_failed", message: "bad" } }));
          return;
        }
        if (body.userAddress === "malformed") {
          response.writeHead(202, { "content-type": "application/json" });
          response.end(JSON.stringify({ status: "queued", requestId: "rid" }));
          return;
        }
        response.writeHead(202, { "content-type": "application/json", "retry-after": "1" });
        response.end(
          JSON.stringify({
            status: "queued",
            requestId: request.headers["x-request-id"] ?? "generated",
            result: { jobId: "job-1" },
          }),
        );
      });
      return;
    }
    const match = /^\/v[23]\/input-proof\/(.+)$/.exec(url);
    if (request.method === "GET" && match) {
      const script = scripts.get(match[1] ?? "") ?? [];
      const next = script.shift() ?? { status: 404 };
      const headers: Record<string, string> = { "content-type": "application/json" };
      if (next.retryAfterSec !== undefined) headers["retry-after"] = next.retryAfterSec.toString();
      response.writeHead(next.status, headers);
      response.end(JSON.stringify(next.body ?? {}));
      return;
    }
    if (request.method === "GET" && url === "/health/readiness") {
      response.writeHead(200, { "content-type": "application/json" });
      response.end(JSON.stringify({ status: "ready" }));
      return;
    }
    response.writeHead(404);
    response.end();
  });
  await new Promise<void>((resolve) => server.listen(0, resolve));
  const { port } = server.address() as AddressInfo;
  client = new RelayerClient({ baseUrl: `http://127.0.0.1:${port.toString()}` });
  clientV3 = new RelayerClient({
    baseUrl: `http://127.0.0.1:${port.toString()}`,
    apiPrefix: "v3",
  });
});

afterAll(async () => {
  await client.close();
  await clientV3.close();
  await new Promise<void>((resolve, reject) =>
    server.close((error) => (error ? reject(error) : resolve())),
  );
});

const submitBody = {
  contractChainId: 11_155_111,
  contractAddress: "0x0000000000000000000000000000000000000001",
  userAddress: "0x0000000000000000000000000000000000000002",
  ciphertextWithInputVerification: "aabb",
  extraData: "0x00",
};

describe("RelayerClient", () => {
  it("parses 202 envelopes and Retry-After on submit", async () => {
    const outcome = await client.submitInputProof(submitBody, "rid-1");
    expect(outcome.httpStatus).toBe(202);
    expect(outcome.accepted?.result.jobId).toBe("job-1");
    expect(outcome.accepted?.requestId).toBe("rid-1");
    expect(outcome.retryAfterMs).toBe(1000);
  });

  it("uses the configured API prefix", async () => {
    const outcome = await clientV3.submitInputProof(submitBody, "rid-v3");
    expect(clientV3.apiPrefix).toBe("/v3");
    expect(outcome.httpStatus).toBe(202);
    expect(outcome.accepted?.requestId).toBe("rid-v3");
  });

  it("surfaces error labels on rejected submissions", async () => {
    const outcome = await client.submitInputProof({ ...submitBody, userAddress: "reject" });
    expect(outcome.httpStatus).toBe(400);
    expect(outcome.accepted).toBeUndefined();
    expect(outcome.errorLabel).toBe("validation_failed");
  });

  it("classifies malformed accepted envelopes as protocol errors", async () => {
    const outcome = await client.submitInputProof({ ...submitBody, userAddress: "malformed" });
    expect(outcome).toMatchObject({
      httpStatus: 202,
      protocolError: true,
      errorLabel: "client_protocol_error",
    });
    expect(outcome.accepted).toBeUndefined();
  });

  it("polls through 202s to a successful result", async () => {
    scripts.set("job-poll", [
      { status: 202, body: { status: "queued", requestId: "r" }, retryAfterSec: 0.05 },
      { status: 202, body: { status: "queued", requestId: "r" }, retryAfterSec: 0.05 },
      {
        status: 200,
        body: {
          status: "succeeded",
          requestId: "r",
          result: { accepted: true, extraData: "0x00", handles: ["0xab"], signatures: ["cd"] },
        },
      },
    ]);
    const outcome = await client.pollJob<InputProofResultJson>("input-proof", "job-poll", {
      deadlineMs: 5000,
      initialRetryAfterMs: 0,
      minIntervalMs: 10,
    });
    expect(outcome.pollCount).toBe(3);
    expect(outcome.result?.accepted).toBe(true);
    expect(outcome.deadlineExceeded).toBe(false);
  });

  it("accepts rotating per-exchange response request ids", async () => {
    scripts.set("job-rotating-ids", [
      {
        status: 202,
        body: { status: "queued", requestId: "poll-request-1" },
        retryAfterSec: 0.01,
      },
      {
        status: 200,
        body: {
          status: "succeeded",
          requestId: "poll-request-2",
          result: { accepted: true, extraData: "0x00", handles: [], signatures: [] },
        },
      },
    ]);
    const outcome = await client.pollJob<InputProofResultJson>(
      "input-proof",
      "job-rotating-ids",
      { deadlineMs: 5_000, initialRetryAfterMs: 0, minIntervalMs: 1 },
    );
    expect(outcome).toMatchObject({ pollCount: 2, deadlineExceeded: false });
    expect(outcome.result?.accepted).toBe(true);
  });

  it("classifies missing queued and success identities as protocol errors", async () => {
    scripts.set("job-missing-queued-id", [{
      status: 202, body: { status: "queued" },
    }]);
    const queued = await client.pollJob<InputProofResultJson>(
      "input-proof", "job-missing-queued-id",
      { deadlineMs: 5_000, initialRetryAfterMs: 0, minIntervalMs: 10 },
    );
    expect(queued).toMatchObject({ protocolError: true, errorLabel: "client_protocol_error" });

    scripts.set("job-missing-success-id", [{
      status: 200,
      body: {
        status: "succeeded",
        result: { accepted: true, extraData: "0x00", handles: [], signatures: [] },
      },
    }]);
    const succeeded = await client.pollJob<InputProofResultJson>(
      "input-proof", "job-missing-success-id",
      { deadlineMs: 5_000, initialRetryAfterMs: 0, minIntervalMs: 10 },
    );
    expect(succeeded).toMatchObject({ protocolError: true, errorLabel: "client_protocol_error" });
  });

  it("encodes an untrusted job id as one URL path segment", async () => {
    const jobId = "job/with space?query=yes";
    scripts.set(encodeURIComponent(jobId), [{
      status: 200,
      body: {
        status: "succeeded",
        requestId: "accepted-request",
        result: { accepted: true, extraData: "0x00", handles: [], signatures: [] },
      },
    }]);
    const outcome = await client.pollJob<InputProofResultJson>("input-proof", jobId, {
      deadlineMs: 5_000,
      initialRetryAfterMs: 0,
      minIntervalMs: 10,
    });
    expect(outcome.result?.accepted).toBe(true);
  });

  it("maps terminal failures to their error label", async () => {
    scripts.set("job-fail", [
      {
        status: 503,
        body: {
          status: "failed",
          requestId: "r",
          error: { label: "response_timed_out", message: "gateway never answered" },
        },
      },
    ]);
    const outcome = await client.pollJob<InputProofResultJson>("input-proof", "job-fail", {
      deadlineMs: 5000,
      initialRetryAfterMs: 0,
      minIntervalMs: 10,
    });
    expect(outcome.errorLabel).toBe("response_timed_out");
    expect(outcome.result).toBeUndefined();
  });

  it("accepts a detailed terminal failure without a proxy request id", async () => {
    scripts.set("job-fail-no-id", [{
      status: 400,
      body: {
        status: "failed",
        error: {
          label: "validation_failed",
          message: "bad field",
          details: [{ field: "handle", issue: "invalid" }],
        },
      },
    }]);
    const outcome = await client.pollJob<InputProofResultJson>(
      "input-proof",
      "job-fail-no-id",
      { deadlineMs: 5_000, initialRetryAfterMs: 0, minIntervalMs: 10 },
    );
    expect(outcome).toMatchObject({
      errorLabel: "validation_failed",
      deadlineExceeded: false,
    });
    expect(outcome.protocolError).toBeUndefined();
  });

  it("classifies malformed queued, success, and failure envelopes as protocol errors", async () => {
    scripts.set("job-bad-queued", [{
      status: 202,
      body: { status: "succeeded", requestId: "r" },
    }]);
    const queued = await client.pollJob<InputProofResultJson>("input-proof", "job-bad-queued", {
      deadlineMs: 5000,
      initialRetryAfterMs: 0,
      minIntervalMs: 10,
    });
    expect(queued).toMatchObject({ protocolError: true, errorLabel: "client_protocol_error" });

    scripts.set("job-bad-success", [{
      status: 200,
      body: { status: "succeeded", requestId: "r", result: { accepted: "yes" } },
    }]);
    const succeeded = await client.pollJob<InputProofResultJson>("input-proof", "job-bad-success", {
      deadlineMs: 5000,
      initialRetryAfterMs: 0,
      minIntervalMs: 10,
    });
    expect(succeeded).toMatchObject({ protocolError: true, errorLabel: "client_protocol_error" });

    scripts.set("job-bad-input-proof", [{
      status: 200,
      body: {
        status: "succeeded",
        requestId: "r",
        result: { accepted: true, extraData: "0x00" },
      },
    }]);
    const inputProof = await client.pollJob<InputProofResultJson>(
      "input-proof",
      "job-bad-input-proof",
      { deadlineMs: 5_000, initialRetryAfterMs: 0, minIntervalMs: 10 },
    );
    expect(inputProof).toMatchObject({
      protocolError: true,
      errorLabel: "client_protocol_error",
    });

    scripts.set("job-bad-failure", [{
      status: 500,
      body: { status: "failed", requestId: "r" },
    }]);
    const failed = await client.pollJob<InputProofResultJson>(
      "input-proof",
      "job-bad-failure",
      { deadlineMs: 5_000, initialRetryAfterMs: 0, minIntervalMs: 10 },
    );
    expect(failed).toMatchObject({ protocolError: true, errorLabel: "client_protocol_error" });
  });

  it("gives up at the deadline while the job stays queued", async () => {
    scripts.set(
      "job-slow",
      Array.from({ length: 50 }, () => ({
        status: 202,
        body: { status: "queued", requestId: "r" },
        retryAfterSec: 0.05,
      })),
    );
    const outcome = await client.pollJob<InputProofResultJson>("input-proof", "job-slow", {
      deadlineMs: 200,
      initialRetryAfterMs: 0,
      minIntervalMs: 10,
    });
    expect(outcome.deadlineExceeded).toBe(true);
    expect(outcome.errorLabel).toBe("client_poll_deadline_exceeded");
  });

  it("honors an initial Retry-After before the first GET", async () => {
    scripts.set("job-initial-delay", [{
      status: 200,
      body: {
        status: "succeeded",
        requestId: "terminal-request",
        result: { accepted: true, extraData: "0x00", handles: [], signatures: [] },
      },
    }]);
    const startedAt = performance.now();
    const outcome = await client.pollJob<InputProofResultJson>(
      "input-proof",
      "job-initial-delay",
      { deadlineMs: 5_000, initialRetryAfterMs: 50, minIntervalMs: 0 },
    );
    expect(performance.now() - startedAt).toBeGreaterThanOrEqual(40);
    expect(outcome).toMatchObject({ pollCount: 1, deadlineExceeded: false });
  });

  it("uses the fallback interval when the POST omits Retry-After", async () => {
    const outcome = await client.pollJob<InputProofResultJson>(
      "input-proof",
      "job-after-fallback-deadline",
      { deadlineMs: 30, defaultIntervalMs: 1_000 },
    );
    expect(outcome).toMatchObject({
      pollCount: 0,
      deadlineExceeded: true,
      errorLabel: "client_poll_deadline_exceeded",
    });
  });

  it("caps the initial Retry-After at the maximum interval", async () => {
    scripts.set("job-upper-clamped-delay", [{
      status: 200,
      body: {
        status: "succeeded",
        requestId: "terminal-request",
        result: { accepted: true, extraData: "0x00", handles: [], signatures: [] },
      },
    }]);
    const outcome = await client.pollJob<InputProofResultJson>(
      "input-proof",
      "job-upper-clamped-delay",
      { deadlineMs: 200, initialRetryAfterMs: 1_000, maxIntervalMs: 20 },
    );
    expect(outcome).toMatchObject({ pollCount: 1, deadlineExceeded: false });
  });

  it("floors the initial Retry-After at the minimum interval", async () => {
    const outcome = await client.pollJob<InputProofResultJson>(
      "input-proof",
      "job-after-minimum-delay-deadline",
      { deadlineMs: 30, initialRetryAfterMs: 1, minIntervalMs: 50 },
    );
    expect(outcome).toMatchObject({
      pollCount: 0,
      deadlineExceeded: true,
      errorLabel: "client_poll_deadline_exceeded",
    });
  });

  it("aborts during the initial delay without issuing a GET", async () => {
    const controller = new AbortController();
    setTimeout(() => controller.abort(), 20);
    const startedAt = performance.now();
    const outcome = await client.pollJob<InputProofResultJson>(
      "input-proof",
      "job-aborted-before-first-poll",
      { deadlineMs: 5_000, initialRetryAfterMs: 1_000, signal: controller.signal },
    );
    expect(performance.now() - startedAt).toBeLessThan(200);
    expect(outcome).toMatchObject({ pollCount: 0, deadlineExceeded: false, aborted: true });
  });

  it("reports readiness", async () => {
    expect(await client.isReady()).toBe(true);
  });
});
