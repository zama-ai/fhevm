import { getBytes, hexlify, isAddress } from 'ethers';

import type { ResolvedContractsSection, ResolvedRelayerSection } from '../../../config/profiles/types.js';
import type { Evidence, RecordOptions } from '../../world/evidence.js';
import type { Chain } from '../chain/chain.js';
import {
  type CreateOptions,
  FheClient,
  type PermitSigner,
  RelayerCallError,
  type RelayerTrace,
  SDK_VERSION,
  type SdkBindings,
  type SignedPermit,
  type TransportKeyPair,
  type TypedClear,
} from './client.js';

/** FHE types the SDK encrypts and decrypts. */
export type FheType = 'bool' | 'uint8' | 'uint16' | 'uint32' | 'uint64' | 'uint128' | 'uint256' | 'address';

/** A decrypted value: numbers are always `bigint`, so comparisons never mix `number` and `bigint`. */
export type ClearValue = bigint | boolean | string;

/** A handle as a contract getter returns it (hex string) or as `encrypt` returns it (bytes). */
export type HandleLike = string | Uint8Array;

/** A contract address, or anything with `getAddress()` (an ethers `Contract`). */
export type ContractLike = string | { getAddress(): Promise<string> };

export interface FheCallOptions {
  /** Label of the evidence entry (a default label describes the operation). */
  label?: string;
  /** Relayer timeout of the call, in milliseconds (SDK default otherwise). */
  timeoutMs?: number;
}

export interface EncryptParameters {
  contract: ContractLike;
  /** Wallet role of the user the input is for (the transaction sender). */
  user: string;
  /** Values in the order of the handles returned. */
  values: { type: FheType; value: boolean | bigint | number | string }[];
}

/** `encrypt` result, ready for an ethers contract call. */
export interface EncryptedInput {
  handles: Uint8Array[];
  inputProof: Uint8Array;
}

/** Options shared by user and delegated decryption. */
export interface DecryptOptions extends FheCallOptions {
  /** `legacy` (default, works on every relayer) or `unified` (the v3 route, checked first). */
  permit?: 'legacy' | 'unified';
  /** New keypair and permit for this call only, without the scenario cache. */
  fresh?: boolean;
  /** Permit start, in seconds; bypasses the cache. */
  startTimestamp?: number;
  /** Permit duration, in seconds (default 10 days, as in the e2e suite); bypasses the cache. */
  durationSeconds?: number;
  /** A given transport keypair (serialised); bypasses the cache. */
  transportKeyPair?: { publicKey: string; privateKey: string };
}

type OneOrMany = { handle: HandleLike; handles?: never } | { handles: HandleLike[]; handle?: never };

export type UserDecryptParameters = OneOrMany & DecryptOptions & { contract: ContractLike; user: string };

export type DelegatedDecryptParameters = OneOrMany &
  DecryptOptions & {
    contract: ContractLike;
    /** Wallet role that signs the permit and receives the values. */
    delegate: string;
    /** Wallet role or address (for example a smart wallet) that delegated its rights. */
    delegator: string;
  };

export interface PublicDecryption {
  /** Values in the order of the handles. */
  values: ClearValue[];
  /** What `FHE.checkSignatures` takes on chain. */
  abiEncodedClearValues: string;
  decryptionProof: string;
}

/** A failed FHE operation, recorded as evidence before it is thrown. */
export class FheError extends Error {
  constructor(
    message: string,
    /** Evidence kind of the operation, e.g. `fhe.user-decrypt`. */
    readonly operation: string,
    /** Label of the relayer error (e.g. `not_allowed_on_host_acl`), when the relayer rejected the call. */
    readonly relayerLabel: string | undefined,
    /** Relayer job ID, when the relayer created a job. */
    readonly jobId: string | undefined,
    options?: { cause?: unknown },
  ) {
    super(message, options);
    this.name = 'FheError';
  }
}

export interface FheOptions {
  profileName: string;
  relayer: ResolvedRelayerSection;
  contracts: ResolvedContractsSection;
  /** The scenario's Chain module: provider, chain ID and wallets. */
  chain: Pick<Chain, 'chainId' | 'provider' | 'wallet'>;
  evidence: Pick<Evidence, 'record'>;
  /** Receives the SDK's warnings and errors. */
  log?: (message: string) => void;
  /** See `CreateOptions.singleThread`; the scenario worker keeps the thread pool (default). */
  singleThread?: boolean;
  /** Replaces the SDK (unit tests). */
  sdk?: SdkBindings;
  /** Current time in seconds (unit tests). */
  nowSeconds?: () => number;
}

/** Default permit duration, the e2e suite's. */
const DEFAULT_PERMIT_SECONDS = 10 * 24 * 3600;
/** A cached permit is renewed this long before it expires. */
const PERMIT_RENEWAL_MARGIN_SECONDS = 60;

interface CachedPermit {
  transportKeyPair: TransportKeyPair;
  signedPermit: SignedPermit;
  expiresAt: number;
}

const errorMessage = (error: unknown): string => (error instanceof Error ? error.message : String(error));

/** Numbers become `bigint`; booleans and strings (addresses) are kept. */
const toClearValue = (value: TypedClear['value']): ClearValue => (typeof value === 'number' ? BigInt(value) : value);

/** Clear values as JSON (`bigint` has no JSON form). */
const evidenceValues = (types: string[], values: ClearValue[]) =>
  values.map((value, index) => ({ type: types[index], value: typeof value === 'bigint' ? value.toString() : value }));

const toHex = (handle: HandleLike): string => (typeof handle === 'string' ? handle : hexlify(handle));

/** The relayer fields of a trace, without the empty ones. */
function relayerEvidence(trace: RelayerTrace): Record<string, unknown> {
  const { operation, jobId, requestId, status, httpStatus, error } = trace;
  return Object.fromEntries(
    Object.entries({ operation, jobId, requestId, status, httpStatus, error }).filter(([, v]) => v !== undefined),
  );
}

/**
 * FHE operations for scenario steps, built on the profile's `relayer` and `contracts` sections and
 * the scenario's Chain module: encryption with input proof, user, delegated and public decryption.
 * Wallets are named by role; every operation is recorded as `fhe.*` evidence with its relayer job
 * ID. Transport keypairs and permits are cached for the scenario. One instance per World; the SDK
 * client is created on first use.
 */
export class Fhe {
  private clientPromise: Promise<FheClient> | undefined;
  private readonly keyPairs = new Map<string, Promise<TransportKeyPair>>();
  private readonly permits = new Map<string, CachedPermit>();
  private readonly nowSeconds: () => number;

  constructor(private readonly options: FheOptions) {
    this.nowSeconds = options.nowSeconds ?? (() => Math.floor(Date.now() / 1000));
  }

  /** Encrypts `values` for `contract` and `user` into one input proof; records `fhe.encrypt`. */
  async encrypt(parameters: EncryptParameters, options: FheCallOptions = {}): Promise<EncryptedInput> {
    const contract = await addressOf(parameters.contract);
    const user = this.account(parameters.user);
    const types = parameters.values.map((input) => input.type);
    const data = { contract, user, types };
    const label = options.label ?? `encrypt ${types.join(', ')} for ${parameters.user}`;
    const { result, trace, durationMs } = await this.run('fhe.encrypt', data, label, (client) =>
      client.encrypt({ contractAddress: contract, userAddress: user.address, values: parameters.values }, options),
    );
    const encrypted = {
      handles: result.handles.map((handle) => getBytes(handle)),
      inputProof: getBytes(result.inputProof),
    };
    this.record(
      'fhe.encrypt',
      {
        ...data,
        handles: result.handles,
        inputProofBytes: encrypted.inputProof.length,
        relayer: relayerEvidence(trace),
        durationMs,
        status: 'success',
      },
      label,
    );
    return encrypted;
  }

  /** User decryption of one handle (`handle`) or several in one request (`handles`); records `fhe.user-decrypt`. */
  userDecrypt(parameters: UserDecryptParameters & { handle: HandleLike }): Promise<ClearValue>;
  userDecrypt(parameters: UserDecryptParameters & { handles: HandleLike[] }): Promise<ClearValue[]>;
  async userDecrypt(parameters: UserDecryptParameters): Promise<ClearValue | ClearValue[]> {
    const values = await this.decrypt('fhe.user-decrypt', parameters, parameters.user, undefined);
    return parameters.handles ? values : values[0]!;
  }

  /**
   * Decryption by `delegate` of handles `delegator` delegated (on the host ACL) for `contract`;
   * records `fhe.delegated-decrypt`.
   */
  delegatedUserDecrypt(parameters: DelegatedDecryptParameters & { handle: HandleLike }): Promise<ClearValue>;
  delegatedUserDecrypt(parameters: DelegatedDecryptParameters & { handles: HandleLike[] }): Promise<ClearValue[]>;
  async delegatedUserDecrypt(parameters: DelegatedDecryptParameters): Promise<ClearValue | ClearValue[]> {
    const delegator = isAddress(parameters.delegator)
      ? { address: parameters.delegator }
      : this.account(parameters.delegator);
    const values = await this.decrypt('fhe.delegated-decrypt', parameters, parameters.delegate, delegator);
    return parameters.handles ? values : values[0]!;
  }

  /** Public decryption of handles made publicly decryptable; records `fhe.public-decrypt`. */
  async publicDecrypt(handles: HandleLike[], options: FheCallOptions = {}): Promise<PublicDecryption> {
    if (handles.length === 0) throw new Error('a public decryption needs at least one handle');
    const hexHandles = handles.map(toHex);
    const data = { handles: hexHandles };
    const label = options.label ?? `public decryption of ${hexHandles.length} handle(s)`;
    const { result, trace, durationMs } = await this.run('fhe.public-decrypt', data, label, (client) =>
      client.publicDecrypt(hexHandles, options),
    );
    const values = result.values.map((value) => toClearValue(value.value));
    this.record(
      'fhe.public-decrypt',
      {
        ...data,
        values: evidenceValues(
          result.values.map((value) => value.type),
          values,
        ),
        decryptionProofBytes: getBytes(result.decryptionProof).length,
        relayer: relayerEvidence(trace),
        durationMs,
        status: 'success',
      },
      label,
    );
    return { values, abiEncodedClearValues: result.abiEncodedClearValues, decryptionProof: result.decryptionProof };
  }

  /** Whether the relayer offers the unified (v3) user decryption route. */
  async canUseUnifiedPermit(): Promise<boolean> {
    return (await this.client()).canUseUnifiedPermit();
  }

  /** Drops the client and the cache. Called by the engine after each scenario (the SDK has nothing to close). */
  destroy(): void {
    this.clientPromise = undefined;
    this.keyPairs.clear();
    this.permits.clear();
  }

  private async decrypt(
    kind: 'fhe.user-decrypt' | 'fhe.delegated-decrypt',
    parameters: OneOrMany & DecryptOptions & { contract: ContractLike },
    signerRole: string,
    delegator: { role?: string; address: string } | undefined,
  ): Promise<ClearValue[]> {
    const contract = await addressOf(parameters.contract);
    const signer = this.account(signerRole);
    const given = parameters.handles ?? (parameters.handle === undefined ? [] : [parameters.handle]);
    if (given.length === 0) throw new Error('a decryption needs a handle or a non-empty handles list');
    const handles = given.map(toHex);
    const permitKind = parameters.permit ?? 'legacy';
    const data: Record<string, unknown> = {
      contract,
      ...(delegator ? { delegate: signer, delegator } : { user: signer }),
      permit: permitKind,
      handles,
    };
    const label =
      parameters.label ??
      (delegator
        ? `delegated decryption by ${signerRole} of ${handles.length} handle(s)`
        : `user decryption by ${signerRole} of ${handles.length} handle(s)`);
    const { result, trace, durationMs } = await this.run(kind, data, label, async (client) => {
      if (permitKind === 'unified' && !(await client.canUseUnifiedPermit())) {
        throw new Error(`the relayer of profile '${this.options.profileName}' does not offer the unified (v3) route`);
      }
      const permit = await this.permitFor(client, permitKind, signerRole, contract, delegator?.address, parameters);
      // Set on the shared data, so that a failed decryption also says where its permit came from.
      data.cache = permit.cache;
      return client.decrypt({ handles, contractAddress: contract, ...permit.entry }, parameters);
    });
    const values = result.map((value) => toClearValue(value.value));
    this.record(
      kind,
      {
        ...data,
        values: evidenceValues(
          result.map((value) => value.type),
          values,
        ),
        relayer: relayerEvidence(trace),
        durationMs,
        status: 'success',
      },
      label,
    );
    return values;
  }

  /** A permit for the call: from the scenario cache, or new when the options ask for one. */
  private async permitFor(
    client: FheClient,
    kind: 'legacy' | 'unified',
    signerRole: string,
    contract: string,
    delegatorAddress: string | undefined,
    options: DecryptOptions,
  ): Promise<{ entry: CachedPermit; cache: 'hit' | 'miss' | 'bypassed' }> {
    const bypass =
      options.fresh === true ||
      options.startTimestamp !== undefined ||
      options.durationSeconds !== undefined ||
      options.transportKeyPair !== undefined;
    const key = [kind, signerRole, contract.toLowerCase(), delegatorAddress?.toLowerCase() ?? ''].join('|');
    if (!bypass) {
      const cached = this.permits.get(key);
      if (cached && cached.expiresAt - PERMIT_RENEWAL_MARGIN_SECONDS > this.nowSeconds()) {
        return { entry: cached, cache: 'hit' };
      }
    }
    const transportKeyPair = options.transportKeyPair
      ? await client.parseTransportKeyPair(options.transportKeyPair)
      : bypass
        ? await client.generateTransportKeyPair()
        : await this.keyPairOf(client, signerRole);
    const startTimestamp = options.startTimestamp ?? this.nowSeconds();
    const durationSeconds = options.durationSeconds ?? DEFAULT_PERMIT_SECONDS;
    const wallet = this.options.chain.wallet(signerRole);
    const signedPermit = await client.signPermit(kind, {
      contractAddresses: [contract],
      startTimestamp,
      durationSeconds,
      signer: wallet as unknown as PermitSigner,
      signerAddress: wallet.address,
      ...(delegatorAddress ? { delegatorAddress } : {}),
      transportKeyPair,
    });
    const entry = { transportKeyPair, signedPermit, expiresAt: startTimestamp + durationSeconds };
    if (!bypass) this.permits.set(key, entry);
    return { entry, cache: bypass ? 'bypassed' : 'miss' };
  }

  /** The transport keypair of a wallet role, generated once per scenario. */
  private keyPairOf(client: FheClient, role: string): Promise<TransportKeyPair> {
    let keyPair = this.keyPairs.get(role);
    if (!keyPair) {
      const generation = client.generateTransportKeyPair();
      // Only this generation's entry is dropped on failure, never one that replaced it.
      generation.catch(() => {
        if (this.keyPairs.get(role) === generation) this.keyPairs.delete(role);
      });
      this.keyPairs.set(role, generation);
      keyPair = generation;
    }
    return keyPair;
  }

  /** The SDK client, created on first use; records `fhe.client`. A failed creation is retried next time. */
  private client(): Promise<FheClient> {
    if (!this.clientPromise) {
      const creation: Promise<FheClient> = this.createClient().catch((error: unknown) => {
        // A creation that fails after `destroy()` must not clear the client created since.
        if (this.clientPromise === creation) this.clientPromise = undefined;
        throw error;
      });
      this.clientPromise = creation;
    }
    return this.clientPromise;
  }

  private async createClient(): Promise<FheClient> {
    const { relayer, contracts, chain } = this.options;
    const singleThread = this.options.singleThread ?? false;
    const data = {
      relayerUrl: relayer.url.value,
      chainId: chain.chainId,
      gatewayChainId: relayer.gateway.chainId.value,
      sdkVersion: SDK_VERSION,
      authenticated: (relayer.apiKey?.value ?? '') !== '',
      threads: singleThread ? 'single' : 'pool',
    };
    const createOptions: CreateOptions = {
      singleThread,
      ...(this.options.log ? { log: this.options.log } : {}),
      ...(this.options.sdk ? { sdk: this.options.sdk } : {}),
    };
    try {
      const client = await FheClient.create(
        { relayer, contracts, chainId: chain.chainId },
        chain.provider,
        createOptions,
      );
      this.record('fhe.client', { ...data, readyMs: client.readyMs, status: 'success' }, 'FHE client ready');
      return client;
    } catch (error) {
      this.record('fhe.client', { ...data, status: 'failed', error: errorMessage(error) }, 'FHE client');
      throw new FheError(
        `profile '${this.options.profileName}': the FHE client could not be created: ${errorMessage(error)}`,
        'fhe.client',
        undefined,
        undefined,
        { cause: error },
      );
    }
  }

  /**
   * Runs one operation, timed. A failure is recorded (with what the relayer reported) before it is
   * rethrown as `FheError`, so the evidence of a failure is never lost.
   */
  private async run<T>(
    kind: string,
    data: Record<string, unknown>,
    label: string,
    operation: (client: FheClient) => Promise<{ result: T; trace: RelayerTrace }>,
  ): Promise<{ result: T; trace: RelayerTrace; durationMs: number }> {
    const started = Date.now();
    const client = await this.client();
    try {
      const { result, trace } = await operation(client);
      return { result, trace, durationMs: Date.now() - started };
    } catch (error) {
      const trace = error instanceof RelayerCallError ? error.trace : undefined;
      const cause = error instanceof RelayerCallError ? error.cause : error;
      const status = trace?.status === 'timeout' ? 'timeout' : 'failed';
      this.record(
        kind,
        {
          ...data,
          // An SDK check that fails before any relayer call leaves an empty trace.
          ...(trace && trace.events > 0 ? { relayer: relayerEvidence(trace) } : {}),
          durationMs: Date.now() - started,
          status,
          error: errorMessage(cause),
        },
        label,
      );
      const relayerLabel = trace?.error?.label;
      throw new FheError(
        `${label} failed${relayerLabel ? ` (${relayerLabel})` : ''}: ${errorMessage(cause)}`,
        kind,
        relayerLabel,
        trace?.jobId,
        { cause },
      );
    }
  }

  private account(role: string): { role: string; address: string } {
    return { role, address: this.options.chain.wallet(role).address };
  }

  private record(kind: string, data: Record<string, unknown>, label: string): void {
    const options: RecordOptions = { label };
    this.options.evidence.record(kind, data, options);
  }
}

async function addressOf(contract: ContractLike): Promise<string> {
  return typeof contract === 'string' ? contract : contract.getAddress();
}
