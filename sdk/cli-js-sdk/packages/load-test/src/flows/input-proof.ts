import { randomUUID } from "node:crypto";

import { poolDir, type LoadTestEnv } from "../env";
import { INPUT_PROOF_POOL } from "../pool/input-proof";
import { PoolStore, type Cursor } from "../pool/store";
import type { InputProofPoolItem } from "../pool/types";
import type { RelayerClient } from "../relayer/client";
import type { InputProofResultJson } from "../relayer/types";
import { epochNowMs, monotonicNowMs } from "../shared/time";
import { interruptedLeg } from "./interruption";
import {
  PoolExhaustedError,
  type FlowExecutor,
  type RelayerLegRecord,
  type RequestRecord,
} from "./types";

/**
 * Input-proof executor: submits pre-generated ZK-proof payloads over raw
 * HTTP and verifies that the relayer accepted the proof and returned exactly
 * the handles computed locally at generation time.
 */
export class InputProofExecutor implements FlowExecutor {
  readonly flow = "input-proof" as const;
  private items: InputProofPoolItem[] = [];
  private cursor: Cursor | undefined;

  constructor(
    private readonly env: LoadTestEnv,
    private readonly client: RelayerClient,
    private readonly clientB: RelayerClient | undefined,
    private readonly requestTimeoutMs: number,
  ) {}

  async prepare(planned: number, signal?: AbortSignal): Promise<void> {
    signal?.throwIfAborted();
    const dir = poolDir(this.env, INPUT_PROOF_POOL);
    const store = await PoolStore.openIfExists<InputProofPoolItem>(dir);
    if (!store) {
      throw new Error(
        `No input-proof pool at ${dir}. Generate one: load-test pool add input-proof --count <n>.`,
      );
    }
    this.items = await store.loadItems();
    signal?.throwIfAborted();
    this.cursor = store.cursor("submit");
    const remaining = BigInt(this.items.length) - this.cursor.position;
    if (remaining < BigInt(planned)) {
      throw new Error(
        `Input-proof pool has ${remaining.toString()} unused payload(s); the scenario needs ` +
          `${planned.toString()}. Payloads are single-use (relayer dedup is permanent); ` +
          `top up with: load-test pool add input-proof --count ${(planned - Number(remaining)).toString()}.`,
      );
    }
  }

  private claim(): InputProofPoolItem {
    if (!this.cursor) throw new Error("Executor not prepared.");
    const position = Number(this.cursor.claim());
    const item = this.items[position];
    if (!item) {
      throw new PoolExhaustedError(
        `Input-proof pool exhausted at position ${position.toString()}.`,
      );
    }
    return item;
  }

  private async executeLeg(
    client: RelayerClient,
    body: {
      contractChainId: number;
      contractAddress: string;
      userAddress: string;
      ciphertextWithInputVerification: string;
      extraData: string;
    },
    requestId: string,
    startedMono: number,
    item: InputProofPoolItem,
    signal: AbortSignal,
  ): Promise<RelayerLegRecord> {
    let submit;
    try {
      submit = await client.submitInputProof(body, requestId, signal);
    } catch (error) {
      const interrupted = interruptedLeg(signal);
      return {
        pollCount: 0,
        outcome: interrupted?.outcome ?? "submit_failed",
        errorLabel: interrupted?.errorLabel ?? "client_transport_error",
        errorMessage: (error as Error).message,
      };
    }

    if (!submit.accepted) {
      return {
        submitHttpStatus: submit.httpStatus,
        submitLatencyMs: submit.latencyMs,
        pollCount: 0,
        outcome: submit.protocolError ? "protocol_error" : "submit_failed",
        errorLabel: submit.errorLabel ?? `http_${submit.httpStatus.toString()}`,
        errorMessage: submit.errorMessage,
      };
    }

    const jobId = submit.accepted.result.jobId;
    const poll = await client.pollJob<InputProofResultJson>(
      this.flow,
      jobId,
      {
        deadlineMs: this.requestTimeoutMs,
        initialRetryAfterMs: submit.retryAfterMs,
        requestId,
        signal,
      },
    );
    const e2eLatencyMs = monotonicNowMs() - startedMono;
    const common = {
      submitHttpStatus: submit.httpStatus,
      submitLatencyMs: submit.latencyMs,
      firstRetryAfterMs: submit.retryAfterMs,
      echoedRequestId: submit.accepted.requestId,
      jobId,
      pollCount: poll.pollCount,
      e2eLatencyMs,
    };

    if (poll.deadlineExceeded) {
      return {
        ...common,
        ...(interruptedLeg(signal) ?? {
          outcome: "timed_out" as const,
          errorLabel: poll.errorLabel,
        }),
      };
    }
    if (poll.aborted) {
      return { ...common, outcome: "aborted", errorLabel: "client_aborted" };
    }
    if (!poll.result) {
      return {
        ...common,
        outcome: poll.protocolError ? "protocol_error" : "failed",
        errorLabel: poll.errorLabel,
        errorMessage: poll.errorMessage,
      };
    }

    if (!poll.result.accepted) {
      return {
        ...common,
        outcome: "failed",
        errorLabel: "proof_rejected",
        errorMessage: "Relayer completed the job but the gateway rejected the proof.",
        verified: false,
      };
    }

    const returned = (poll.result.handles ?? []).map((handle) => handle.toLowerCase());
    const expected = item.expectedHandles.map((handle) => handle.toLowerCase());
    const handlesMatch =
      returned.length === expected.length &&
      returned.every((handle, position) => handle === expected[position]);
    if (!handlesMatch) {
      return {
        ...common,
        outcome: "verify_failed",
        errorLabel: "handles_mismatch",
        errorMessage: `Expected [${expected.join(", ")}], got [${returned.join(", ")}].`,
        verified: false,
      };
    }

    return { ...common, outcome: "succeeded", verified: true };
  }

  async execute(index: number, signal: AbortSignal): Promise<RequestRecord> {
    const item = this.claim();
    const sentRequestId = randomUUID();
    const startedAtMs = epochNowMs();
    const startedMono = monotonicNowMs();
    const base = { flow: this.flow, index, startedAtMs, sentRequestId };
    const body = {
      contractChainId: item.contractChainId,
      contractAddress: item.contractAddress,
      userAddress: item.userAddress,
      ciphertextWithInputVerification: item.ciphertextWithInputVerification,
      extraData: item.extraData,
    };

    const [primary, candidate] = await Promise.all([
      this.executeLeg(this.client, body, sentRequestId, startedMono, item, signal),
      this.clientB
        ? this.executeLeg(this.clientB, body, sentRequestId, startedMono, item, signal)
        : Promise.resolve(undefined),
    ]);

    return {
      ...base,
      ...primary,
      ...(candidate
        ? {
            echoedRequestIdB: candidate.echoedRequestId,
            jobIdB: candidate.jobId,
            submitHttpStatusB: candidate.submitHttpStatus,
            submitLatencyMsB: candidate.submitLatencyMs,
            firstRetryAfterMsB: candidate.firstRetryAfterMs,
            pollCountB: candidate.pollCount,
            outcomeB: candidate.outcome,
            errorLabelB: candidate.errorLabel,
            errorMessageB: candidate.errorMessage,
            e2eLatencyMsB: candidate.e2eLatencyMs,
            verifiedB: candidate.verified,
          }
        : {}),
    };
  }

  async close(): Promise<void> {
    // The shared RelayerClient is owned by the runner.
  }
}
