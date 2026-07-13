import {
  createClientContext,
  type ClientContext,
} from "@cli-fhevm-sdk/toolkit/config";
import { getUserDecryptionDelegationExpirationDate } from "@cli-fhevm-sdk/toolkit/acl/delegation";
import { parseClearValue } from "@cli-fhevm-sdk/toolkit/values";
import type {
  RelayerDelegatedUserDecryptProgressArgs,
  RelayerUserDecryptProgressArgs,
} from "@fhevm/sdk/actions/decrypt";
import { createFhevmDecryptClient } from "@fhevm/sdk/viem";
import { randomUUID } from "node:crypto";
import type { Hex } from "viem";
import type { Account } from "viem/accounts";

import { laneAccount, poolDir, type LoadTestEnv } from "../env";
import { HANDLE_POOLS, type HandlePoolFlow } from "../pool/handles";
import { PoolStore } from "../pool/store";
import type { FheHandlePoolItem, PoolMeta } from "../pool/types";
import type { RelayerClient } from "../relayer/client";
import { epochNowMs, monotonicNowMs } from "../shared/time";
import { interruptedLeg } from "./interruption";
import {
  captureInitialPostIdentity,
  sdkTerminalIdentityError,
} from "./progress-identity";
import type { FlowExecutor, RelayerLegRecord, RequestRecord } from "./types";

const PERMIT_CLOCK_SKEW_SECONDS = 120;
const permitDurationSeconds = (requestTimeoutMs: number): number =>
  Math.ceil(
    (Math.ceil(requestTimeoutMs / 1000) + PERMIT_CLOCK_SKEW_SECONDS) / 86_400,
  ) * 86_400;

type UserDecryptProgress =
  | RelayerUserDecryptProgressArgs
  | RelayerDelegatedUserDecryptProgressArgs;

export type UserDecryptSdkClient = Readonly<{
  ready: Promise<void>;
  generateTransportKeyPair: () => Promise<unknown>;
  signDecryptionPermit: (parameters: Record<string, unknown>) => Promise<unknown>;
  decryptValues: (parameters: Record<string, unknown>) => Promise<
    readonly Readonly<{ type: string; value: unknown }>[]
  >;
}>;

export type UserDecryptExecutorDependencies = Readonly<{
  createSdkClient: (context: ClientContext) => UserDecryptSdkClient;
}>;

const defaultDependencies: UserDecryptExecutorDependencies = {
  createSdkClient: (context) =>
    createFhevmDecryptClient({
      chain: context.chain,
      publicClient: context.publicClient,
    }) as unknown as UserDecryptSdkClient,
};

const normalizeValue = (type: string, value: unknown): string => {
  const serialized = typeof value === "bigint" ? value.toString() : String(value);
  if (type === "address") return serialized.toLowerCase();
  if (type === "bool") {
    return serialized === "true" || serialized === "1" ? "true" : "false";
  }
  return serialized;
};

const expectedValue = (item: FheHandlePoolItem): unknown =>
  parseClearValue(item.type, item.value);

const valuesMatch = (
  actual: readonly Readonly<{ type: string; value: unknown }>[],
  item: FheHandlePoolItem,
): boolean =>
  actual.length === 1 &&
  actual[0]?.type === item.type &&
  normalizeValue(item.type, actual[0].value) ===
    normalizeValue(item.type, expectedValue(item));

const lower = (value: string): string => value.toLowerCase();

/** Structural and environment checks that must pass before any load is emitted. */
export const validateUserDecryptPool = (options: {
  env: LoadTestEnv;
  flow: "user-decrypt" | "delegated-user-decrypt";
  meta: PoolMeta;
  items: readonly FheHandlePoolItem[];
  resolvedContractAddress: string;
  nowSeconds?: bigint;
  requiredValidUntilSeconds?: bigint;
}): void => {
  const { env, flow, meta, items, resolvedContractAddress } = options;
  if (meta.kind !== "fhe-handles") {
    throw new Error(`Expected fhe-handles pool, found ${meta.kind}.`);
  }
  if (meta.flow !== flow) {
    throw new Error(`Pool flow ${meta.flow} cannot serve ${flow}.`);
  }
  if (meta.network !== env.network) {
    throw new Error(
      `Pool network ${meta.network} does not match run network ${env.network}.`,
    );
  }
  if (meta.contractChainId !== env.contractChainId) {
    throw new Error(
      `Pool chain ${meta.contractChainId.toString()} does not match run chain ${env.contractChainId.toString()}.`,
    );
  }
  if (lower(meta.contractAddress) !== lower(resolvedContractAddress)) {
    throw new Error(
      `Pool contract ${meta.contractAddress} does not match run contract ${resolvedContractAddress}.`,
    );
  }
  if (meta.count !== items.length) {
    throw new Error(
      `Pool metadata count ${meta.count.toString()} does not match ${items.length.toString()} loaded item(s).`,
    );
  }
  if (items.length === 0) throw new Error("User-decrypt handle pool is empty.");

  const ownerIndices = new Set(meta.ownerIndices ?? []);
  for (const item of items) {
    if (item.isPublic) {
      throw new Error(
        `Pool item ${item.index.toString()} is public and cannot serve private user-decrypt load.`,
      );
    }
    try {
      expectedValue(item);
    } catch (error) {
      throw new Error(
        `Pool item ${item.index.toString()} has an invalid ${item.type} expected value.`,
        { cause: error },
      );
    }
    if (ownerIndices.size > 0 && !ownerIndices.has(item.ownerIndex)) {
      throw new Error(
        `Pool item ${item.index.toString()} references unrecorded owner lane ${item.ownerIndex.toString()}.`,
      );
    }
  }

  if (flow === "delegated-user-decrypt") {
    if (meta.delegateIndex === undefined || !meta.delegateAddress) {
      throw new Error("Delegated pool has no delegate account recorded.");
    }
    if (!meta.delegationExpirations) {
      throw new Error(
        "Delegated pool has no per-owner delegation expirations. Refresh the pool delegation.",
      );
    }
    const nowSeconds = options.nowSeconds ?? BigInt(Math.floor(Date.now() / 1000));
    const requiredValidUntil = options.requiredValidUntilSeconds ?? nowSeconds;
    for (const ownerIndex of new Set(items.map((item) => item.ownerIndex))) {
      const expiration = meta.delegationExpirations[ownerIndex.toString()];
      if (!expiration || !/^\d+$/.test(expiration)) {
        throw new Error(`Delegated pool has no valid expiration for owner lane ${ownerIndex.toString()}.`);
      }
      if (BigInt(expiration) < requiredValidUntil) {
        throw new Error(
          `Delegated pool ACL delegation for owner lane ${ownerIndex.toString()} expires at ${expiration}, before required ${requiredValidUntil.toString()}. Refresh the pool delegation.`,
        );
      }
    }
  }
};

const candidateFields = (candidate: RelayerLegRecord) => ({
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
});

/** SDK-native executor for self and delegated user-decrypt workloads. */
export class UserDecryptExecutor implements FlowExecutor {
  readonly flow: "user-decrypt" | "delegated-user-decrypt";
  private items: FheHandlePoolItem[] = [];
  private meta: PoolMeta | undefined;
  private nextItem = 0;
  private readonly accounts = new Map<number, Account>();
  private targetA: UserDecryptSdkClient | undefined;
  private targetB: UserDecryptSdkClient | undefined;

  constructor(
    private readonly env: LoadTestEnv,
    private readonly client: RelayerClient,
    private readonly clientB: RelayerClient | undefined,
    private readonly requestTimeoutMs: number,
    private readonly delegated: boolean,
    private readonly dependencies: UserDecryptExecutorDependencies = defaultDependencies,
  ) {
    this.flow = delegated ? "delegated-user-decrypt" : "user-decrypt";
    if (clientB && !env.relayerBUrl) {
      throw new Error("Candidate relayer client requires LOAD_TEST_RELAYER_B_URL.");
    }
    if (clientB && new URL(client.baseUrl).origin === new URL(clientB.baseUrl).origin) {
      throw new Error("Primary and candidate relayer targets must be different origins.");
    }
    for (const [label, target] of [["A", client], ["B", clientB]] as const) {
      if (target && target.apiPrefix !== "/v2") {
        throw new Error(
          `SDK-native user-decrypt target ${label} requires API prefix /v2; configured ${target.apiPrefix || "<root>"}.`,
        );
      }
    }
  }

  private account(index: number): Account {
    let account = this.accounts.get(index);
    if (!account) {
      account = laneAccount(index);
      this.accounts.set(index, account);
    }
    return account;
  }

  async prepare(_planned: number, signal?: AbortSignal): Promise<void> {
    signal?.throwIfAborted();
    const dir = poolDir(this.env, HANDLE_POOLS[this.flow as HandlePoolFlow]);
    const store = await PoolStore.openIfExists<FheHandlePoolItem>(dir);
    if (!store) {
      throw new Error(
        `No ${this.flow} handle pool at ${dir}. Create one: load-test pool add --flow ${this.flow} --count <n>.`,
      );
    }
    this.items = await store.loadItems();
    this.meta = store.meta;

    const contextOptions = {
      network: this.env.network,
      rpcUrl: this.env.rpcUrl,
      contractAddress: this.env.contractAddress,
    };
    const contextA = createClientContext({
      ...contextOptions,
      relayerUrl: this.client.baseUrl,
    });
    const requiredDelegationValidUntil = BigInt(Math.floor(Date.now() / 1000)) +
      BigInt(Math.ceil(this.requestTimeoutMs / 1000) + PERMIT_CLOCK_SKEW_SECONDS);
    validateUserDecryptPool({
      env: this.env,
      flow: this.flow,
      meta: this.meta,
      items: this.items,
      resolvedContractAddress: contextA.contractAddress,
      requiredValidUntilSeconds: requiredDelegationValidUntil,
    });

    for (const item of this.items) {
      const owner = this.account(item.ownerIndex);
      if (lower(owner.address) !== lower(item.ownerAddress)) {
        throw new Error(
          `Pool item ${item.index.toString()} owner ${item.ownerAddress} does not match lane ${item.ownerIndex.toString()} account ${owner.address}.`,
        );
      }
    }
    if (this.delegated) {
      const delegate = this.account(this.meta.delegateIndex!);
      if (lower(delegate.address) !== lower(this.meta.delegateAddress!)) {
        throw new Error(
          `Pool delegate ${this.meta.delegateAddress!} does not match lane ${this.meta.delegateIndex!.toString()} account ${delegate.address}.`,
        );
      }
      if (
        this.items.some(
          (item) => lower(item.ownerAddress) === lower(delegate.address),
        )
      ) {
        throw new Error("Delegator and delegate accounts must differ.");
      }
      const ownerAddresses = new Map<number, string>();
      for (const item of this.items) ownerAddresses.set(item.ownerIndex, item.ownerAddress);
      for (const [ownerIndex, ownerAddress] of ownerAddresses) {
        const expiration = await getUserDecryptionDelegationExpirationDate(contextA, {
          delegatorAddress: ownerAddress as Hex,
          delegateAddress: delegate.address,
        });
        if (expiration < requiredDelegationValidUntil) {
          throw new Error(
            `On-chain ACL delegation for owner lane ${ownerIndex.toString()} expires at ${expiration.toString()}, before required ${requiredDelegationValidUntil.toString()}.`,
          );
        }
      }
    }

    this.targetA = this.dependencies.createSdkClient(contextA);
    if (this.clientB) {
      const contextB = createClientContext({
        ...contextOptions,
        relayerUrl: this.clientB.baseUrl,
      });
      this.targetB = this.dependencies.createSdkClient(contextB);
    }
    await Promise.all([this.targetA.ready, this.targetB?.ready]);
    signal?.throwIfAborted();
  }

  private async executeLeg(
    target: UserDecryptSdkClient,
    signer: Account,
    item: FheHandlePoolItem,
    requestId: string,
    signal: AbortSignal,
  ): Promise<RelayerLegRecord> {
    const startedMono = monotonicNowMs();
    let queued:
      | Extract<UserDecryptProgress, { type: "queued" }>
      | undefined;
    let initialIdentity: Readonly<{ requestId: string; jobId: string }> | undefined;
    let lastProgress: UserDecryptProgress | undefined;

    try {
      signal.throwIfAborted();
      const transportKeyPair = await target.generateTransportKeyPair();
      signal.throwIfAborted();
      const signedPermit = await target.signDecryptionPermit({
        transportKeyPair,
        contractAddresses: [this.meta!.contractAddress],
        // Protocol v13 permits require whole days; round the request-derived
        // lifetime up to the smallest duration accepted by every alpha.8 path.
        durationSeconds: permitDurationSeconds(this.requestTimeoutMs),
        startTimestamp: Math.floor(Date.now() / 1000),
        signerAddress: signer.address,
        signer,
        ...(this.delegated ? { delegatorAddress: item.ownerAddress } : {}),
      });
      signal.throwIfAborted();
      const clearValues = await target.decryptValues({
        encryptedValues: [item.handle],
        contractAddress: this.meta!.contractAddress,
        signedPermit,
        transportKeyPair,
        options: {
          timeout: this.requestTimeoutMs,
          signal,
          headers: { "x-request-id": requestId },
          onProgress: (progress: UserDecryptProgress) => {
            lastProgress = progress;
            initialIdentity = captureInitialPostIdentity(initialIdentity, progress);
            if (
              progress.type === "queued" &&
              progress.method === "POST" &&
              queued === undefined
            ) {
              queued = progress;
            }
          },
        },
      });
      // Alpha.8 increments retryCount when scheduling each GET after the
      // initial POST, so it is the number of polls issued, not a zero-based index.
      const pollCount = lastProgress?.retryCount ?? 0;
      const common = {
        submitHttpStatus: queued?.status,
        submitLatencyMs: queued?.elapsed,
        firstRetryAfterMs: queued?.retryAfterMs,
        echoedRequestId: queued?.requestId,
        jobId: queued?.jobId,
        pollCount,
        e2eLatencyMs: monotonicNowMs() - startedMono,
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
      if (!valuesMatch(clearValues, item)) {
        return {
          ...common,
          outcome: "verify_failed",
          errorLabel: "values_mismatch",
          errorMessage: "SDK-verified clear values did not match the known pool value.",
          verified: false,
        };
      }
      return { ...common, outcome: "succeeded", verified: true };
    } catch (error) {
      const interruption = interruptedLeg(signal, lastProgress?.type);
      const relayerFailure =
        lastProgress?.type === "failed" || lastProgress?.type === "throttled"
          ? lastProgress.relayerApiError
          : undefined;
      const verificationFailed = lastProgress?.type === "succeeded";
      return {
        submitHttpStatus: queued?.status,
        submitLatencyMs: queued?.elapsed,
        firstRetryAfterMs: queued?.retryAfterMs,
        echoedRequestId: queued?.requestId,
        jobId: queued?.jobId,
        pollCount: lastProgress?.retryCount ?? 0,
        e2eLatencyMs: monotonicNowMs() - startedMono,
        outcome:
          interruption
            ? interruption.outcome
            : verificationFailed
              ? "verify_failed"
            : queued
              ? "failed"
              : "submit_failed",
        errorLabel: interruption
            ? interruption.errorLabel
              : verificationFailed
                ? "kms_verification_or_reconstruction_failed"
              : relayerFailure?.label ?? "sdk_user_decrypt_error",
        errorMessage:
          relayerFailure?.message ??
          (error instanceof Error ? error.message : String(error)),
        verified: false,
      };
    }
  }

  async execute(index: number, signal: AbortSignal): Promise<RequestRecord> {
    if (!this.targetA || !this.meta) throw new Error("Executor not prepared.");
    const item = this.items[this.nextItem % this.items.length];
    this.nextItem += 1;
    if (!item) throw new Error("Handle pool is empty.");

    const signer = this.delegated
      ? this.account(this.meta.delegateIndex!)
      : this.account(item.ownerIndex);
    const sentRequestId = randomUUID();
    const base = {
      flow: this.flow,
      index,
      startedAtMs: epochNowMs(),
      sentRequestId,
    };

    const [primary, candidate] = await Promise.all([
      this.executeLeg(this.targetA, signer, item, sentRequestId, signal),
      this.targetB
        ? this.executeLeg(this.targetB, signer, item, sentRequestId, signal)
        : Promise.resolve(undefined),
    ]);

    return {
      ...base,
      ...primary,
      ...(candidate ? candidateFields(candidate) : {}),
    };
  }

  async close(): Promise<void> {
    // SDK clients own no explicit close lifecycle.
  }
}
