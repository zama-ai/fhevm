import {
  createClientContext,
  type ClientContext,
} from "@cli-fhevm-sdk/toolkit/config";
import type { RelayerPublicDecryptProgressArgs } from "@fhevm/sdk/actions/base";
import { randomUUID } from "node:crypto";
import type { Hex } from "viem";

import { poolDir, type LoadTestEnv } from "../env";
import {
  binomial,
  minimumCombinationPoolSize,
  unrankCombination,
} from "../pool/combinations";
import { HANDLE_POOLS } from "../pool/handles";
import { PoolStore, type Cursor } from "../pool/store";
import type { FheHandlePoolItem } from "../pool/types";
import type { RelayerClient } from "../relayer/client";
import { epochNowMs, monotonicNowMs } from "../shared/time";
import { interruptedLeg } from "./interruption";
import {
  captureInitialPostIdentity,
  sdkTerminalIdentityError,
} from "./progress-identity";
import {
  PoolExhaustedError,
  type FlowExecutor,
  type RelayerLegRecord,
  type RequestRecord,
} from "./types";

const normalizedValue = (value: boolean | bigint | number | string): string =>
  String(value);

export const publicValuesMatch = (
  actual: readonly Readonly<{ type: string; value: boolean | bigint | number | string }>[],
  expected: readonly Readonly<{ type: string; value: string }>[],
): boolean =>
  actual.length === expected.length &&
  actual.every(
    (value, index) =>
      value.type === expected[index]?.type &&
      normalizedValue(value.value) === expected[index]?.value,
  );

/**
 * Public-decrypt executor driven by the SDK's public journey. The SDK owns
 * request construction, polling, KMS signer discovery and signature
 * verification; the load test only checks the returned typed clear values
 * against its known pool values and translates SDK progress into metrics.
 */
export class PublicDecryptExecutor implements FlowExecutor {
  readonly flow = "public-decrypt" as const;
  private items: FheHandlePoolItem[] = [];
  private cursor: Cursor | undefined;
  private context: ClientContext | undefined;
  private contextB: ClientContext | undefined;

  constructor(
    private readonly env: LoadTestEnv,
    _client: RelayerClient,
    clientB: RelayerClient | undefined,
    private readonly requestTimeoutMs: number,
    private readonly handlesPerRequest: number,
  ) {
    void _client;
    if (clientB && !env.relayerBUrl) {
      throw new Error("Candidate relayer client requires LOAD_TEST_RELAYER_B_URL.");
    }
  }

  async prepare(planned: number, signal?: AbortSignal): Promise<void> {
    signal?.throwIfAborted();
    const dir = poolDir(this.env, HANDLE_POOLS["public-decrypt"]);
    const store = await PoolStore.openIfExists<FheHandlePoolItem>(dir);
    if (!store) {
      throw new Error(
        `No public-decrypt handle pool at ${dir}. Create one: load-test pool add public-decrypt --count <n>.`,
      );
    }
    this.items = await store.loadItems();
    signal?.throwIfAborted();
    this.cursor = store.cursor(`combos-k${this.handlesPerRequest.toString()}`);

    const total = binomial(this.items.length, this.handlesPerRequest);
    const remaining = total - this.cursor.position;
    if (remaining < BigInt(planned)) {
      const neededTotalHandles = minimumCombinationPoolSize(
        this.cursor.position + BigInt(planned),
        this.handlesPerRequest,
      );
      const handlesToAdd = Math.max(0, neededTotalHandles - this.items.length);
      const targetCombinations = binomial(neededTotalHandles, this.handlesPerRequest);
      throw new Error(
        `Public-decrypt pool of ${this.items.length.toString()} handle(s) has ` +
          `${remaining.toString()} unused ${this.handlesPerRequest.toString()}-handle combination(s); ` +
          `the scenario needs ${planned.toString()}. Add at least ${handlesToAdd.toString()} handle(s) ` +
          `for ${neededTotalHandles.toString()} total handle(s), which provides ${targetCombinations.toString()} ` +
          `${this.handlesPerRequest.toString()}-handle combination(s): ` +
          `load-test pool add public-decrypt --count ${handlesToAdd.toString()}.`,
      );
    }

    const options = {
      network: this.env.network,
      rpcUrl: this.env.rpcUrl,
      contractAddress: this.env.contractAddress,
    };
    this.context = createClientContext({ ...options, relayerUrl: this.env.relayerUrl });
    this.contextB = this.env.relayerBUrl
      ? createClientContext({ ...options, relayerUrl: this.env.relayerBUrl })
      : undefined;
  }

  private claimCombination(): FheHandlePoolItem[] {
    if (!this.cursor) throw new Error("Executor not prepared.");
    const rank = this.cursor.claim();
    if (rank >= binomial(this.items.length, this.handlesPerRequest)) {
      throw new PoolExhaustedError(
        `Public-decrypt combinations exhausted at rank ${rank.toString()}.`,
      );
    }
    return unrankCombination(rank, this.items.length, this.handlesPerRequest).map(
      (position) => {
        const item = this.items[position];
        if (!item) throw new PoolExhaustedError(`Missing pool item ${position.toString()}.`);
        return item;
      },
    );
  }

  private async executeLeg(
    context: ClientContext,
    requestId: string,
    startedMono: number,
    handles: readonly Hex[],
    combination: readonly FheHandlePoolItem[],
    signal: AbortSignal,
  ): Promise<RelayerLegRecord> {
    let queued: Extract<RelayerPublicDecryptProgressArgs, { type: "queued" }> | undefined;
    let initialIdentity: Readonly<{ requestId: string; jobId: string }> | undefined;
    let lastProgress: RelayerPublicDecryptProgressArgs | undefined;

    try {
      const result = await context.fhevm.decryptPublicValuesWithSignatures({
        encryptedValues: handles,
        options: {
          timeout: this.requestTimeoutMs,
          signal,
          headers: { "x-request-id": requestId },
          onProgress: (progress) => {
            lastProgress = progress;
            initialIdentity = captureInitialPostIdentity(initialIdentity, progress);
            if (progress.type === "queued" && progress.method === "POST" && queued === undefined) {
              queued = progress;
            }
          },
        },
      });
      const e2eLatencyMs = monotonicNowMs() - startedMono;
      const valuesMatch = publicValuesMatch(
        result.clearValues,
        combination.map((item) => ({ type: item.type, value: item.value })),
      );
      const common = {
        submitHttpStatus: queued?.status,
        submitLatencyMs: queued?.elapsed,
        firstRetryAfterMs: queued?.retryAfterMs,
        echoedRequestId: queued?.requestId,
        jobId: queued?.jobId,
        pollCount: lastProgress?.retryCount ?? 0,
        e2eLatencyMs,
      };
      const identityError = sdkTerminalIdentityError(initialIdentity, lastProgress);
      if (identityError) {
        return {
          ...common,
          outcome: "protocol_error",
          errorLabel: "client_response_identity_mismatch",
          errorMessage: identityError,
          verified: false,
        };
      }
      return valuesMatch
        ? { ...common, outcome: "succeeded", verified: true }
        : {
            ...common,
            outcome: "verify_failed",
            errorLabel: "values_mismatch",
            errorMessage: "SDK-verified clear values did not match the known pool values.",
            verified: false,
          };
    } catch (error) {
      const interruption = interruptedLeg(signal, lastProgress?.type);
      return {
        submitHttpStatus: queued?.status,
        submitLatencyMs: queued?.elapsed,
        firstRetryAfterMs: queued?.retryAfterMs,
        echoedRequestId: queued?.requestId,
        jobId: queued?.jobId,
        pollCount: lastProgress?.retryCount ?? 0,
        e2eLatencyMs: monotonicNowMs() - startedMono,
        outcome: interruption?.outcome ?? (queued ? "failed" : "submit_failed"),
        errorLabel: interruption?.errorLabel ?? "sdk_public_decrypt_error",
        errorMessage: error instanceof Error ? error.message : String(error),
        verified: false,
      };
    }
  }

  async execute(index: number, signal: AbortSignal): Promise<RequestRecord> {
    if (!this.context) throw new Error("Executor not prepared.");
    const combination = this.claimCombination();
    const handles = combination.map((item) => item.handle as Hex);
    const sentRequestId = randomUUID();
    const startedAtMs = epochNowMs();
    const startedMono = monotonicNowMs();
    const base = { flow: this.flow, index, startedAtMs, sentRequestId };

    const [primary, candidate] = await Promise.all([
      this.executeLeg(
        this.context,
        sentRequestId,
        startedMono,
        handles,
        combination,
        signal,
      ),
      this.contextB
        ? this.executeLeg(
            this.contextB,
            sentRequestId,
            startedMono,
            handles,
            combination,
            signal,
          )
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
    // SDK clients are stateless and own no explicit close lifecycle.
  }
}
