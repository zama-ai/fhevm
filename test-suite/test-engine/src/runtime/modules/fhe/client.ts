/**
 * The only file that imports `@fhevm/sdk`: it creates the SDK client from the profile's `relayer`
 * and `contracts` sections, sends the relayer API key on every relayer call, and reports what each
 * call told about its relayer job through `onProgress` (job and request IDs, status, error).
 *
 * The SDK is reached through `SdkBindings`, so that unit tests can replace it with a fake.
 */
import { canUseUnifiedDecryptionPermit } from '@fhevm/sdk/actions/base';
import { defineFhevmChain } from '@fhevm/sdk/chains';
import { createFhevmClient, hasFhevmRuntimeConfig, setFhevmRuntimeConfig } from '@fhevm/sdk/ethers';
import type { ContractRunner } from 'ethers';

import type { ResolvedContractsSection, ResolvedRelayerSection } from '../../../config/profiles/types.js';

type Address = `0x${string}`;
type SdkClient = ReturnType<typeof createFhevmClient>;
type SdkAuth = { type: 'ApiKeyHeader'; value: string };
type SdkRuntimeConfig = Parameters<typeof setFhevmRuntimeConfig>[0];

/** What the module uses of an SDK client. */
export type SdkClientLike = Pick<
  SdkClient,
  | 'ready'
  | 'encryptValues'
  | 'generateTransportKeyPair'
  | 'parseTransportKeyPair'
  | 'signLegacyDecryptionPermit'
  | 'signUnifiedDecryptionPermit'
  | 'decryptValues'
  | 'decryptPublicValuesWithSignatures'
>;

export type TransportKeyPair = Awaited<ReturnType<SdkClientLike['parseTransportKeyPair']>>;
export type SignedPermit = Awaited<ReturnType<SdkClientLike['signLegacyDecryptionPermit']>>;
export type PermitSigner = Parameters<SdkClientLike['signLegacyDecryptionPermit']>[0]['signer'];

/** The SDK entry points the module calls; replaced by a fake in the unit tests. */
export interface SdkBindings {
  hasRuntimeConfig(): boolean;
  setRuntimeConfig(config: SdkRuntimeConfig): void;
  createClient(parameters: { provider: ContractRunner; chain: ReturnType<typeof defineFhevmChain> }): SdkClientLike;
  canUseUnifiedPermit(client: SdkClientLike, options: { auth?: SdkAuth }): Promise<boolean>;
}

const sdkBindings: SdkBindings = {
  hasRuntimeConfig: hasFhevmRuntimeConfig,
  setRuntimeConfig: setFhevmRuntimeConfig,
  createClient: (parameters) => createFhevmClient({ provider: parameters.provider, chain: parameters.chain }),
  canUseUnifiedPermit: (client, options) =>
    canUseUnifiedDecryptionPermit(client as unknown as Parameters<typeof canUseUnifiedDecryptionPermit>[0], {
      options,
    }),
};

/** The relayer operations that create a job. */
export type RelayerOperation = 'INPUT_PROOF' | 'USER_DECRYPT' | 'DELEGATED_USER_DECRYPT' | 'PUBLIC_DECRYPT';

/** What one relayer call reported through `onProgress`, for the evidence. */
export interface RelayerTrace {
  /** Operation the relayer reported (a permit with a delegator gives `DELEGATED_USER_DECRYPT`). */
  operation?: RelayerOperation;
  jobId?: string;
  requestId?: string;
  /** Last progress event: `queued`, `throttled`, `succeeded`, `failed`, `timeout` or `abort`. */
  status?: string;
  /** HTTP status of the last relayer response. */
  httpStatus?: number;
  /** Relayer error of a failed call. */
  error?: { label: string; message: string };
  /** Number of progress events received (0 means the SDK reported nothing). */
  events: number;
}

/** Result of a relayer call, with what the relayer reported, even when the call failed. */
export interface Traced<T> {
  result: T;
  trace: RelayerTrace;
}

/** A relayer call that failed; `trace` holds what the relayer reported before the failure. */
export class RelayerCallError extends Error {
  constructor(
    readonly operation: string,
    readonly trace: RelayerTrace,
    override readonly cause: unknown,
  ) {
    super(`${operation} failed: ${causeMessage(cause)}`);
    this.name = 'RelayerCallError';
  }
}

const causeMessage = (error: unknown): string => (error instanceof Error ? error.message : String(error));

/** A value to encrypt, as the SDK takes it (`type` is an FHE type name such as `uint64`). */
export interface TypedInput {
  type: string;
  value: boolean | bigint | number | string;
}

/** A decrypted value as the SDK returns it. */
export interface TypedClear {
  type: string;
  value: boolean | bigint | number | string;
}

export interface CallOptions {
  /** Relayer timeout of the call, in milliseconds (SDK default otherwise). */
  timeoutMs?: number;
}

export interface FheClientConfig {
  relayer: ResolvedRelayerSection;
  contracts: ResolvedContractsSection;
  /** Host chain ID (the profile's `chain` section). */
  chainId: number;
}

export interface CreateOptions {
  /**
   * WASM on the main thread instead of the SDK thread pool. The pool cannot be stopped, so code
   * outside a scenario worker (which ends with `process.exit`) must use `true`. Default `false`.
   */
  singleThread?: boolean;
  /** Receives the SDK's warnings and errors. */
  log?: (message: string) => void;
  /** Replaces the SDK (unit tests). */
  sdk?: SdkBindings;
}

/** The SDK takes the relayer base URL; profiles give it with the API version, as the e2e suite does. */
export const relayerBaseUrl = (url: string): string => url.replace(/\/v[12]\/?$/, '');

/** Where the SDK logger writes: the client created last (one profile per process). */
let logSink: ((message: string) => void) | undefined;

/**
 * Configures the SDK runtime once per process. The API key also goes into the runtime
 * configuration, because the SDK uses it for calls that take no per-call options (the FHE key
 * download while the client gets ready, the relayer feature lookup).
 */
function configureRuntime(sdk: SdkBindings, auth: SdkAuth | undefined, singleThread: boolean): void {
  if (sdk.hasRuntimeConfig()) return;
  sdk.setRuntimeConfig({
    singleThread,
    ...(auth ? { auth } : {}),
    logger: {
      debug: undefined,
      warn: (message) => logSink?.(`[fhevm-sdk] warn: ${message}`),
      error: (message, cause) =>
        logSink?.(`[fhevm-sdk] error: ${message}${cause === undefined ? '' : ` (${causeMessage(cause)})`}`),
    },
  });
}

/**
 * SDK client for one scenario: every relayer call carries the API key (when the profile has one)
 * and returns what the relayer reported about its job.
 */
export class FheClient {
  private constructor(
    private readonly sdk: SdkBindings,
    private readonly client: SdkClientLike,
    private readonly auth: SdkAuth | undefined,
    /** Time the client took to become ready (FHE key download included), in milliseconds. */
    readonly readyMs: number,
  ) {}

  /** Creates the client on `provider` (the Chain module's) and waits until it is ready. */
  static async create(
    config: FheClientConfig,
    provider: ContractRunner,
    options: CreateOptions = {},
  ): Promise<FheClient> {
    const sdk = options.sdk ?? sdkBindings;
    const apiKey = config.relayer.apiKey?.value ?? '';
    // An empty key means no authentication: no `auth` at all, since an explicit `auth: undefined`
    // would override the runtime default in the SDK's option merge.
    const auth: SdkAuth | undefined = apiKey === '' ? undefined : { type: 'ApiKeyHeader', value: apiKey };
    if (options.log) logSink = options.log;
    configureRuntime(sdk, auth, options.singleThread ?? false);

    const { relayer, contracts } = config;
    const chain = defineFhevmChain({
      id: config.chainId,
      fhevm: {
        contracts: {
          acl: { address: contracts.acl.value as Address },
          inputVerifier: { address: contracts.inputVerifier.value as Address },
          kmsVerifier: { address: contracts.kmsVerifier.value as Address },
          protocolConfig: { address: contracts.protocolConfig.value as Address },
        },
        relayerUrl: relayerBaseUrl(relayer.url.value),
        gateway: {
          id: relayer.gateway.chainId.value,
          contracts: {
            decryption: { address: relayer.gateway.decryption.value as Address },
            inputVerification: { address: relayer.gateway.inputVerification.value as Address },
          },
        },
      },
    });
    const started = Date.now();
    const client = sdk.createClient({ provider, chain });
    await client.ready;
    return new FheClient(sdk, client, auth, Date.now() - started);
  }

  /** True when the profile has a (non-empty) relayer API key. */
  get authenticated(): boolean {
    return this.auth !== undefined;
  }

  /** Encrypts `values` for `contractAddress` and `userAddress` into one input proof. */
  encrypt(
    parameters: { contractAddress: string; userAddress: string; values: TypedInput[] },
    options: CallOptions = {},
  ): Promise<Traced<{ handles: string[]; inputProof: string }>> {
    return this.traced('encrypt', options, async (relayerOptions) => {
      const result = await this.client.encryptValues({ ...parameters, options: relayerOptions });
      return { handles: result.encryptedValues.map(String), inputProof: String(result.inputProof) };
    });
  }

  generateTransportKeyPair(): Promise<TransportKeyPair> {
    return this.client.generateTransportKeyPair();
  }

  /** Rebuilds a transport keypair from its serialised form (a test that needs a given keypair). */
  parseTransportKeyPair(serialized: { publicKey: string; privateKey: string }): Promise<TransportKeyPair> {
    return this.client.parseTransportKeyPair(serialized);
  }

  /** Signs a decryption permit (no relayer call). A `delegatorAddress` makes it a delegated permit. */
  signPermit(
    kind: 'legacy' | 'unified',
    parameters: {
      contractAddresses: string[];
      startTimestamp: number;
      durationSeconds: number;
      signer: PermitSigner;
      signerAddress: string;
      delegatorAddress?: string;
      transportKeyPair: TransportKeyPair;
    },
  ): Promise<SignedPermit> {
    return kind === 'legacy'
      ? this.client.signLegacyDecryptionPermit(parameters)
      : this.client.signUnifiedDecryptionPermit(parameters);
  }

  /** User (or delegated, depending on the permit) decryption of `handles`, in order, in one request. */
  decrypt(
    parameters: {
      handles: string[];
      contractAddress: string;
      transportKeyPair: TransportKeyPair;
      signedPermit: SignedPermit;
    },
    options: CallOptions = {},
  ): Promise<Traced<TypedClear[]>> {
    return this.traced('user decryption', options, async (relayerOptions) => {
      const values = await this.client.decryptValues({
        encryptedValues: parameters.handles,
        contractAddress: parameters.contractAddress,
        transportKeyPair: parameters.transportKeyPair,
        signedPermit: parameters.signedPermit,
        options: relayerOptions,
      });
      return values.map((value) => ({ type: value.type, value: value.value }));
    });
  }

  /** Public decryption of `handles`, with what `checkSignatures` needs on chain. */
  publicDecrypt(
    handles: string[],
    options: CallOptions = {},
  ): Promise<Traced<{ values: TypedClear[]; abiEncodedClearValues: string; decryptionProof: string }>> {
    return this.traced('public decryption', options, async (relayerOptions) => {
      const result = await this.client.decryptPublicValuesWithSignatures({
        encryptedValues: handles,
        options: relayerOptions,
      });
      return {
        values: result.clearValues.map((value) => ({ type: value.type, value: value.value })),
        abiEncodedClearValues: result.checkSignaturesArgs.abiEncodedCleartexts,
        decryptionProof: result.checkSignaturesArgs.decryptionProof,
      };
    });
  }

  /** Whether the relayer offers the unified (v3) user decryption route. */
  canUseUnifiedPermit(): Promise<boolean> {
    return this.sdk.canUseUnifiedPermit(this.client, this.auth ? { auth: this.auth } : {});
  }

  /**
   * Runs one relayer call with the API key and a progress listener, and returns its result with
   * what the relayer reported. A failure is rethrown as `RelayerCallError`, carrying the trace.
   */
  private async traced<T>(
    operation: string,
    options: CallOptions,
    call: (relayerOptions: RelayerOptions) => Promise<T>,
  ): Promise<Traced<T>> {
    const trace: RelayerTrace = { events: 0 };
    const relayerOptions: RelayerOptions = {
      ...(this.auth ? { auth: this.auth } : {}),
      ...(options.timeoutMs !== undefined ? { timeout: options.timeoutMs } : {}),
      onProgress: (event: ProgressEvent) => recordProgress(trace, event),
    };
    try {
      return { result: await call(relayerOptions), trace };
    } catch (error) {
      throw new RelayerCallError(operation, trace, error);
    }
  }
}

/** The options every relayer call of the module sends. */
interface RelayerOptions {
  auth?: SdkAuth;
  timeout?: number;
  onProgress: (event: ProgressEvent) => void;
}

/** The fields the module reads from the SDK's progress events (all operations share them). */
interface ProgressEvent {
  readonly type: string;
  readonly operation: string;
  readonly jobId?: string | undefined;
  readonly requestId?: string;
  readonly status?: number;
  readonly relayerApiError?: { readonly label: string; readonly message: string };
}

function recordProgress(trace: RelayerTrace, event: ProgressEvent): void {
  trace.events += 1;
  trace.operation = event.operation as RelayerOperation;
  trace.status = event.type;
  if (event.jobId) trace.jobId = event.jobId;
  if (event.requestId) trace.requestId = event.requestId;
  if (event.status !== undefined) trace.httpStatus = event.status;
  if (event.relayerApiError) {
    trace.error = { label: event.relayerApiError.label, message: event.relayerApiError.message };
  }
}
