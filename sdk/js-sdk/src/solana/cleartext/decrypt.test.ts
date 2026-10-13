import * as authorization from './authorization.js';
import * as storeValues from './storeValues.js';
import * as zamaHost from '@fhevm/solana-zama-host';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { address, getAddressEncoder, type Address } from '@solana/kit';
import type { FhevmSolanaChain } from '../../core/types/fhevmSolanaChain.js';
import type { SolanaRpc } from '../encryptedStore.js';
import type { SolanaLeafReader } from './leafRecord.js';
import { bytesToHex, bytesToHexNo0x, concatBytes, hexToBytes } from '../../core/base/bytes.js';
import { buildHandle, toFhevmHandle } from '../../core/handle/FhevmHandle.js';
import { createKmsPublicDecryptEip712 } from '../../core/kms/createKmsPublicDecryptEip712.js';
import { publicDecryptDigest, verifyPublicDecryptSignatures } from '../actions/decryptPublicValue.js';
import { RelayerAbortError } from '../../core/errors/RelayerAbortError.js';
import { RelayerTimeoutError } from '../../core/errors/RelayerTimeoutError.js';
import type { RelayerUserDecryptOptions } from '../../core/types/relayer.js';
import {
  PERMIT_IDENTITY_LEN,
  PERMIT_KMS_ROUTING_LEN,
  PERMIT_KMS_ROUTING_VERSION,
  PERMIT_SIGNATURE_LEN,
  PERMIT_TRANSPORT_KEY_LEN,
  decodeSolanaPermitFields,
} from '../permit/index.js';
import { SolanaDecryptionRunError } from '../decryption/retry.js';
import {
  MAX_REQUEST_HANDLES,
  cleartextPublicDecryptCertifier,
  cleartextConnectorRejection,
  cleartextUserDecryptExecution,
} from './decrypt.js';
import { SOLANA_CLEARTEXT_SIGNER_ADDRESSES } from './parties.js';

const host = address('11111111111111111111111111111112');
const signer = address('SysvarRent111111111111111111111111111111111');
const bytes = (value: Address): Uint8Array => new Uint8Array(getAddressEncoder().encode(value));
const now = BigInt(Math.floor(Date.now() / 1000));
const rpc = {} as SolanaRpc;
// The Connector's judgment is stubbed in these tests, so the leaf record is never read.
const readLeaves: SolanaLeafReader = () => Promise.reject(new Error('the leaf record is not read here'));
const chain = { fhevm: { programs: { host: { address: bytesToHex(bytes(host)) } } } } as unknown as FhevmSolanaChain;

const refusal = (failure: authorization.ConnectorFailure): authorization.ConnectorVerdict => ({
  authorized: false,
  failure,
  entry: 0,
  message: `entry 0: ${failure}`,
});

afterEach(() => vi.restoreAllMocks());

// The code per failure is held to the Connector in `authorization.test.ts`; this pins the message.
describe('cleartextConnectorRejection', () => {
  it('answers a request the Connector authorizes', () => {
    expect(cleartextConnectorRejection({ authorized: true })).toBeUndefined();
  });

  it('answers a refusal with the code the Connector sends for it, naming the failure', () => {
    expect(cleartextConnectorRejection(refusal('Delegation::NoLiveDelegation'))).toEqual({
      code: 'acl_denied',
      message: 'Delegation::NoLiveDelegation: entry 0: Delegation::NoLiveDelegation',
    });
  });
});

// A public decryption goes through the production retry loop: a code it retries is judged again,
// one it does not ends the request.
describe('cleartextPublicDecryptCertifier', () => {
  const certify = cleartextPublicDecryptCertifier(rpc, chain, readLeaves);
  const parameters = {
    entries: [
      {
        handle: buildHandle({ chainId: 5n, hash21: `0x${'b1'.repeat(21)}`, fheTypeId: 0 }).bytes32,
        encryptedStore: new Uint8Array(32).fill(0xea),
      },
    ],
    contextId: new Uint8Array(32),
    epochId: new Uint8Array(32),
  };

  beforeEach(() => vi.useFakeTimers());
  afterEach(() => vi.useRealTimers());

  it('certifies a batch in request order, one word per handle, as the SDK verifies it', async () => {
    const second = {
      handle: buildHandle({ chainId: 5n, hash21: `0x${'b2'.repeat(21)}`, fheTypeId: 5 }).bytes32,
      encryptedStore: new Uint8Array(32).fill(0xeb),
    };
    const batch = {
      entries: [...parameters.entries, second],
      contextId: new Uint8Array(32).fill(0x44),
      epochId: new Uint8Array(32).fill(0x45),
    };
    const word = (value: number) => Uint8Array.from({ length: 32 }, (_, index) => (index === 31 ? value : 0));
    const valueOf = new Map([
      [0xea, word(1)],
      [0xeb, word(7)],
    ]);
    const kmsSigners = SOLANA_CLEARTEXT_SIGNER_ADDRESSES.kms.map((signer) => hexToBytes(signer));
    const decryptionContract = new Uint8Array(20).fill(0xdc);
    vi.spyOn(authorization, 'judgeSolanaPublicDecryption').mockResolvedValue({ authorized: true });
    vi.spyOn(storeValues, 'fetchCleartextStoreValue').mockImplementation(async (_rpc, _program, store) => {
      const value = valueOf.get(store[0] ?? -1);
      if (value === undefined) throw new Error('unknown store');
      return value;
    });
    vi.spyOn(zamaHost, 'fetchHostConfig').mockResolvedValue({
      data: { gatewayChainId: 7n, decryptionContract },
    } as unknown as Awaited<ReturnType<typeof zamaHost.fetchHostConfig>>);
    vi.spyOn(zamaHost, 'fetchKmsContext').mockResolvedValue({
      data: { signers: kmsSigners, thresholds: { publicDecryption: 1 } },
    } as unknown as Awaited<ReturnType<typeof zamaHost.fetchKmsContext>>);

    const claim = await certify(batch);

    const handles = batch.entries.map(({ handle }) => toFhevmHandle(handle));
    expect(claim.handles).toEqual(handles.map((handle) => handle.bytes32Hex));
    expect(claim.abiEncodedCleartext).toBe(bytesToHexNo0x(concatBytes(word(1), word(7))));
    expect(claim.extraData).toBe(`0x02${'44'.repeat(32)}${'45'.repeat(32)}`);
    expect(authorization.judgeSolanaPublicDecryption).toHaveBeenCalledTimes(1);
    const digest = publicDecryptDigest(
      createKmsPublicDecryptEip712({
        verifyingContractAddressDecryption: bytesToHex(decryptionContract),
        chainId: 7n,
        handles,
        decryptedResult: `0x${claim.abiEncodedCleartext}`,
        extraData: claim.extraData,
      }),
    );
    expect(() =>
      verifyPublicDecryptSignatures(
        digest,
        claim.signatures.map((signature) => `0x${signature}`),
        kmsSigners,
        1,
      ),
    ).not.toThrow();
  });

  it("refuses a request over the handle cap as the relayer's admission does, before the Connector", async () => {
    vi.spyOn(authorization, 'judgeSolanaPublicDecryption');
    const overCap = {
      ...parameters,
      entries: Array.from({ length: MAX_REQUEST_HANDLES + 1 }, () => parameters.entries[0]!),
    };
    const error = await certify(overCap).catch((caught: unknown) => caught);
    expect(error).toMatchObject({ rejection: { code: 'malformed' }, attempts: 1 });
    expect(authorization.judgeSolanaPublicDecryption).not.toHaveBeenCalled();
  });

  it('fails at once on a code the retry loop gives up on', async () => {
    vi.spyOn(authorization, 'judgeSolanaPublicDecryption').mockResolvedValue(
      refusal('EncryptedStore::InvalidHostRecord'),
    );
    const error = await certify(parameters).catch((caught: unknown) => caught);
    expect(error).toBeInstanceOf(SolanaDecryptionRunError);
    expect(error).toMatchObject({ rejection: { code: 'unprocessable' }, attempts: 1 });
    expect(authorization.judgeSolanaPublicDecryption).toHaveBeenCalledTimes(1);
  });

  it('retries a code the retry loop retries, within its budget', async () => {
    vi.spyOn(authorization, 'judgeSolanaPublicDecryption').mockResolvedValue(
      refusal('HandleBinding::ProofRecordBehind'),
    );
    const outcome = certify(parameters).catch((error: unknown) => error);
    await vi.runAllTimersAsync();
    expect(await outcome).toMatchObject({ rejection: { code: 'acl_denied' }, attempts: 4 });
    expect(authorization.judgeSolanaPublicDecryption).toHaveBeenCalledTimes(4);
  });

  it('stops retrying when the signal aborts, as a public decryption', async () => {
    vi.spyOn(authorization, 'judgeSolanaPublicDecryption').mockResolvedValue(
      refusal('HandleBinding::ProofRecordBehind'),
    );
    const controller = new AbortController();
    const outcome = certify({ ...parameters, options: { signal: controller.signal } }).catch((error: unknown) => error);
    await vi.advanceTimersByTimeAsync(250);
    controller.abort();
    await vi.runAllTimersAsync();
    const error = await outcome;
    expect(error).toBeInstanceOf(RelayerAbortError);
    expect((error as RelayerAbortError).operation).toBe('PUBLIC_DECRYPT');
    expect(authorization.judgeSolanaPublicDecryption).toHaveBeenCalledTimes(1);
  });
});

// The execution through the production retry loop: the order of the real stack's checks, what
// changes between attempts, and what bounds them.
describe('cleartextUserDecryptExecution', () => {
  const hostChainId = 5n;
  const gatewayChainId = 7n;
  const decryptionContract = new Uint8Array(20).fill(0xdc);
  const kmsSigner = new Uint8Array(20).fill(0x5e);
  const identity = (fill: number): Uint8Array => new Uint8Array(PERMIT_IDENTITY_LEN).fill(fill);
  const routing = new Uint8Array(PERMIT_KMS_ROUTING_LEN);
  routing[0] = PERMIT_KMS_ROUTING_VERSION;
  const permitHandle = buildHandle({ chainId: hostChainId, hash21: `0x${'a1'.repeat(21)}`, fheTypeId: 0 }).bytes32;
  const sessionStarting = (startTimestamp: bigint) =>
    ({
      signedPermit: {
        fields: decodeSolanaPermitFields({
          userAddress: bytes(signer),
          transportKey: new Uint8Array(PERMIT_TRANSPORT_KEY_LEN),
          allowedScopes: [],
          startTimestamp,
          durationSeconds: 3_600n,
          verifyingProgramId: bytes(host),
          chainId: hostChainId,
          extraData: routing,
        }),
        signature: new Uint8Array(PERMIT_SIGNATURE_LEN),
      },
    }) as unknown as Parameters<ReturnType<typeof cleartextUserDecryptExecution>['execute']>[0]['session'];
  const entries = [{ handle: permitHandle, ownerAddress: bytes(signer), encryptedStore: identity(0xea) }];
  const kmsContext = (destroyed: boolean) =>
    ({ data: { destroyed, signers: [kmsSigner] } }) as unknown as Awaited<ReturnType<typeof zamaHost.fetchKmsContext>>;
  const execute = (
    options?: RelayerUserDecryptOptions,
    { attempts, start = now - 10n }: { attempts?: number; start?: bigint } = {},
  ) =>
    cleartextUserDecryptExecution(rpc, chain, readLeaves).execute({
      session: sessionStarting(start),
      entries,
      attempts,
      options,
    });
  /** The rejection a run of one attempt ends on. */
  const firstRejection = async (start?: bigint) => {
    const error = await execute(undefined, { attempts: 1, ...(start === undefined ? {} : { start }) }).catch(
      (caught: unknown) => caught,
    );
    expect(error).toBeInstanceOf(SolanaDecryptionRunError);
    return (error as SolanaDecryptionRunError).rejection;
  };
  const hostOnChain = (chainId: bigint) =>
    vi.mocked(zamaHost.fetchHostConfig).mockResolvedValue({
      data: { chainId, gatewayChainId, decryptionContract },
    } as unknown as Awaited<ReturnType<typeof zamaHost.fetchHostConfig>>);

  beforeEach(() => {
    vi.useFakeTimers();
    vi.spyOn(zamaHost, 'fetchHostConfig');
    hostOnChain(hostChainId);
    vi.spyOn(zamaHost, 'fetchKmsContext').mockResolvedValue(kmsContext(false));
    vi.spyOn(storeValues, 'fetchCleartextStoreValue').mockResolvedValue(new Uint8Array([1]));
    vi.spyOn(authorization, 'judgeSolanaUserDecryption').mockResolvedValue({ authorized: true });
  });

  afterEach(() => vi.useRealTimers());

  // Captured before the fake clock is installed. Each attempt derives PDAs with WebCrypto, which
  // settles on the real clock, so the wait below yields real time as well as fake.
  const realSetTimeout = globalThis.setTimeout;

  /** Runs, without advancing the fake clock, until `reached` holds. */
  const until = async (reached: () => boolean) => {
    for (let step = 0; !reached(); step += 1) {
      if (step === 5_000) throw new Error('the run never got there');
      await new Promise((resolve) => realSetTimeout(resolve, 1));
    }
  };

  /** Runs until the first attempt has its verdict and the retry loop waits on its backoff. */
  const untilBackoff = async () => {
    await until(() => vi.mocked(authorization.judgeSolanaUserDecryption).mock.calls.length > 0);
    await new Promise((resolve) => realSetTimeout(resolve, 1));
  };

  /** A read that never answers, keeping the signal it was given. */
  const stalled = () => {
    const signals: (AbortSignal | undefined)[] = [];
    const read = (_rpc: unknown, _address: unknown, config?: { abortSignal?: AbortSignal }) => {
      signals.push(config?.abortSignal);
      return new Promise<never>(() => undefined);
    };
    return { signals, read };
  };

  /** The first attempt finds the leaf record one leaf behind the store; later ones find it caught up. */
  const lagOnce = () =>
    vi
      .mocked(authorization.judgeSolanaUserDecryption)
      .mockResolvedValueOnce(refusal('HandleBinding::ProofRecordBehind'));

  it("refuses handles on another host chain as the relayer's admission does, before the Connector", async () => {
    hostOnChain(hostChainId + 1n);
    expect(await firstRejection()).toMatchObject({ code: 'malformed' });
    expect(authorization.judgeSolanaUserDecryption).not.toHaveBeenCalled();
  });

  it("refuses a request over the handle cap as the relayer's admission does, before the Connector", async () => {
    const error = await cleartextUserDecryptExecution(rpc, chain, readLeaves)
      .execute({
        session: sessionStarting(now - 10n),
        entries: Array.from({ length: MAX_REQUEST_HANDLES + 1 }, () => entries[0]!),
        attempts: 1,
        options: undefined,
      })
      .catch((caught: unknown) => caught);
    expect(error).toMatchObject({ rejection: { code: 'malformed' }, attempts: 1 });
    expect(authorization.judgeSolanaUserDecryption).not.toHaveBeenCalled();
  });

  it("refuses a window the relayer's wall clock does not hold, before the Connector's checks", async () => {
    vi.mocked(zamaHost.fetchKmsContext).mockResolvedValue(kmsContext(true));
    for (const start of [now + 600n, now - 3_700n]) {
      expect(await firstRejection(start)).toMatchObject({ code: 'malformed' });
    }
    expect(authorization.judgeSolanaUserDecryption).not.toHaveBeenCalled();
  });

  it('gives up at once on a destroyed KMS context, before the Connector authorizes', async () => {
    vi.mocked(zamaHost.fetchKmsContext).mockResolvedValue(kmsContext(true));
    const error = await execute().catch((caught: unknown) => caught);
    expect(error).toBeInstanceOf(SolanaDecryptionRunError);
    expect((error as SolanaDecryptionRunError).attempts).toBe(1);
    expect((error as SolanaDecryptionRunError).rejection.code).toBe('kms_context_destroyed');
    expect(authorization.judgeSolanaUserDecryption).not.toHaveBeenCalled();
  });

  it('gives up at once on a signature the Connector rejects', async () => {
    vi.mocked(authorization.judgeSolanaUserDecryption).mockResolvedValue(refusal('Signature'));
    const error = await execute().catch((caught: unknown) => caught);
    expect((error as SolanaDecryptionRunError).attempts).toBe(1);
    expect((error as SolanaDecryptionRunError).rejection.code).toBe('user_signature_rejected');
  });

  it('judges every attempt against the KMS context as it is then', async () => {
    lagOnce();
    vi.mocked(zamaHost.fetchKmsContext).mockResolvedValueOnce(kmsContext(false)).mockResolvedValue(kmsContext(true));
    const outcome = execute().catch((error: unknown) => error);
    await untilBackoff();
    await vi.advanceTimersByTimeAsync(2_000);
    expect(String(await outcome)).toMatch(/kms_context_destroyed: KMS context .* is destroyed/);
    expect(zamaHost.fetchKmsContext).toHaveBeenCalledTimes(2);
  });

  it('answers once the leaf record catches up', async () => {
    lagOnce();
    const outcome = execute();
    await untilBackoff();
    await vi.advanceTimersByTimeAsync(2_000);
    await expect(outcome).resolves.toEqual([{ bytes: new Uint8Array([1]), fheTypeId: 0 }]);
  });

  it('reads nothing under a signal already aborted', async () => {
    const controller = new AbortController();
    controller.abort();
    await expect(execute({ signal: controller.signal })).rejects.toBeInstanceOf(RelayerAbortError);
    expect(zamaHost.fetchKmsContext).not.toHaveBeenCalled();
    expect(zamaHost.fetchHostConfig).not.toHaveBeenCalled();
  });

  it('stops waiting between attempts when the signal aborts', async () => {
    lagOnce();
    const controller = new AbortController();
    const outcome = execute({ signal: controller.signal }).catch((error: unknown) => error);
    await untilBackoff();
    controller.abort();
    expect(await outcome).toBeInstanceOf(RelayerAbortError);
    expect(zamaHost.fetchKmsContext).toHaveBeenCalledTimes(1);
  });

  it('ends an attempt stalled on a host read at the caller timeout, and aborts the read', async () => {
    const { signals, read } = stalled();
    vi.mocked(zamaHost.fetchKmsContext).mockImplementation(read);
    const outcome = execute({ timeout: 1_000 }).catch((error: unknown) => error);
    await until(() => signals.length > 0);
    await vi.advanceTimersByTimeAsync(1_000);
    expect(await outcome).toBeInstanceOf(RelayerTimeoutError);
    expect(signals[0]?.aborted).toBe(true);
  });

  it('ends an attempt waiting on the shared leaf record when the signal aborts, reporting it once', async () => {
    // The leaf record is shared by every caller, so the wait on it ends for this one only.
    vi.mocked(authorization.judgeSolanaUserDecryption).mockReturnValue(new Promise(() => undefined));
    const controller = new AbortController();
    const onProgress = vi.fn();
    const outcome = execute({ signal: controller.signal, onProgress }).catch((error: unknown) => error);
    await until(() => vi.mocked(authorization.judgeSolanaUserDecryption).mock.calls.length > 0);
    controller.abort();
    expect(await outcome).toBeInstanceOf(RelayerAbortError);
    await vi.runAllTimersAsync();
    expect(onProgress.mock.calls.filter(([progress]) => progress.type === 'abort')).toHaveLength(1);
  });

  it('gives up at the caller timeout, backoff included', async () => {
    vi.mocked(authorization.judgeSolanaUserDecryption).mockResolvedValue(refusal('HandleBinding::ProofRecordBehind'));
    const outcome = execute({ timeout: 1_000 }).catch((error: unknown) => error);
    await untilBackoff();
    await vi.advanceTimersByTimeAsync(1_000);
    expect(await outcome).toBeInstanceOf(RelayerTimeoutError);
  });
});
