import * as authorization from './authorization.js';
import * as storeHistory from './storeHistory.js';
import * as storeValues from './storeValues.js';
import * as hostConfigAccount from '../internal/generated/zamaHost/accounts/hostConfig.js';
import * as kmsContextAccount from '../internal/generated/zamaHost/accounts/kmsContext.js';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { address, getAddressEncoder, type Address } from '@solana/kit';
import type { FhevmSolanaChain } from '../../core/types/fhevmSolanaChain.js';
import type { SolanaDecryptTrust } from '../clients/decorators/permitDecrypt.js';
import type { SolanaRpc } from '../encryptedStore.js';
import type { SolanaStoreHistoryReader } from './storeHistory.js';
import { bytesToHex } from '../../core/base/bytes.js';
import { buildHandle } from '../../core/handle/FhevmHandle.js';
import { RelayerAbortError } from '../../core/errors/RelayerAbortError.js';
import { RelayerTimeoutError } from '../../core/errors/RelayerTimeoutError.js';
import {
  PERMIT_IDENTITY_LEN,
  PERMIT_KMS_ROUTING_LEN,
  PERMIT_KMS_ROUTING_VERSION,
  PERMIT_SIGNATURE_LEN,
  PERMIT_TRANSPORT_KEY_LEN,
  decodeSolanaPermitFields,
} from '../permit/index.js';
import { SolanaUserDecryptRunError } from '../userDecrypt/index.js';
import { cleartextUserDecryptExecution, cleartextUserDecryptRejection } from './decrypt.js';

const host = address('11111111111111111111111111111112');
const signer = address('SysvarRent111111111111111111111111111111111');
const bytes = (value: Address): Uint8Array => new Uint8Array(getAddressEncoder().encode(value));
const now = BigInt(Math.floor(Date.now() / 1000));
const readHistory = vi.fn<SolanaStoreHistoryReader>();
const rpc = {} as SolanaRpc;

const refusal = (failure: authorization.ConnectorFailure): authorization.ConnectorVerdict => ({
  authorized: false,
  failure,
  entry: 0,
  message: `entry 0: ${failure}`,
});

beforeEach(() => {
  vi.spyOn(storeHistory, 'createSolanaStoreHistoryReader').mockReturnValue(readHistory);
});

afterEach(() => vi.restoreAllMocks());

describe('cleartextUserDecryptRejection', () => {
  it('answers a request the Connector authorizes', () => {
    expect(cleartextUserDecryptRejection({ authorized: true })).toBeUndefined();
  });

  it('refuses a bad signature at submission, as the relayer does', () => {
    expect(cleartextUserDecryptRejection(refusal('Signature'))).toEqual({
      kind: 'refused',
      label: 'validation_failed',
      message: 'entry 0: Signature',
    });
  });

  it("gives up on a delegated entry without a live row, as the relayer's pre-check does", () => {
    expect(cleartextUserDecryptRejection(refusal('Delegation::NoLiveDelegation'))).toEqual({
      kind: 'failed',
      label: 'not_allowed_on_host_acl',
      message: 'entry 0: Delegation::NoLiveDelegation',
    });
  });

  it('leaves a failure the Connector would retry unanswered', () => {
    for (const failure of [
      'HandleBinding::ProofRecordBehind',
      'EncryptedStore::Absent',
      'Window::NotYetValid',
    ] as const) {
      expect(cleartextUserDecryptRejection(refusal(failure))).toEqual({ kind: 'unanswered' });
    }
  });

  it('fails at once on a failure the Connector never clears, naming it', () => {
    expect(() => cleartextUserDecryptRejection(refusal('ScopeNotAllowed'))).toThrow(
      /ScopeNotAllowed: entry 0: ScopeNotAllowed.*response_timed_out/,
    );
  });
});

// The execution through the production retry loop: what changes between attempts, and what bounds them.
describe('cleartextUserDecryptExecution', () => {
  const hostChainId = 5n;
  const gatewayChainId = 7n;
  const decryptionContract = new Uint8Array(20).fill(0xdc);
  const kmsSigner = new Uint8Array(20).fill(0x5e);
  const chain = { fhevm: { programs: { host: { address: bytesToHex(bytes(host)) } } } } as unknown as FhevmSolanaChain;
  const trust = {
    gatewayEip712Domain: { chainId: gatewayChainId, verifyingContract: bytesToHex(decryptionContract) },
    kmsSigners: [{ address: bytesToHex(kmsSigner) }],
  } as unknown as SolanaDecryptTrust;
  const identity = (fill: number): Uint8Array => new Uint8Array(PERMIT_IDENTITY_LEN).fill(fill);
  const routing = new Uint8Array(PERMIT_KMS_ROUTING_LEN);
  routing[0] = PERMIT_KMS_ROUTING_VERSION;
  const permitHandle = buildHandle({ chainId: hostChainId, hash21: `0x${'a1'.repeat(21)}`, fheTypeId: 0 }).bytes32;
  const session = {
    signedPermit: {
      fields: decodeSolanaPermitFields({
        userAddress: bytes(signer),
        transportKey: new Uint8Array(PERMIT_TRANSPORT_KEY_LEN),
        allowedScopes: [],
        startTimestamp: now - 10n,
        durationSeconds: 3_600n,
        verifyingProgramId: bytes(host),
        chainId: hostChainId,
        extraData: routing,
      }),
      signature: new Uint8Array(PERMIT_SIGNATURE_LEN),
    },
  } as unknown as Parameters<ReturnType<typeof cleartextUserDecryptExecution>>[0]['session'];
  const entries = [{ handle: permitHandle, ownerAddress: bytes(signer), encryptedStore: identity(0xea) }];
  const kmsContext = (destroyed: boolean) =>
    ({ data: { destroyed, signers: [kmsSigner] } }) as unknown as Awaited<
      ReturnType<typeof kmsContextAccount.fetchKmsContext>
    >;
  const execute = (options?: { signal?: AbortSignal; timeout?: number }) =>
    cleartextUserDecryptExecution(rpc, chain, trust)({ session, entries, attempts: undefined, options });

  beforeEach(() => {
    vi.useFakeTimers();
    vi.spyOn(hostConfigAccount, 'fetchHostConfig').mockResolvedValue({
      data: { chainId: hostChainId, gatewayChainId, decryptionContract },
    } as unknown as Awaited<ReturnType<typeof hostConfigAccount.fetchHostConfig>>);
    vi.spyOn(kmsContextAccount, 'fetchKmsContext').mockResolvedValue(kmsContext(false));
    vi.spyOn(storeValues, 'fetchCleartextStoreValue').mockResolvedValue(new Uint8Array([1]));
    vi.spyOn(authorization, 'judgeSolanaUserDecryption').mockResolvedValue({ authorized: true });
  });

  afterEach(() => vi.useRealTimers());

  // Captured before the fake clock is installed. Each attempt derives PDAs with WebCrypto, which
  // settles on the real clock, so the wait below yields real time as well as fake.
  const realSetTimeout = globalThis.setTimeout;

  /** Runs until the retry loop waits: its backoff is then the one pending timer. */
  const untilBackoff = async () => {
    for (let step = 0; vi.getTimerCount() === 0; step += 1) {
      if (step === 5_000) throw new Error('the run never reached its backoff');
      await vi.advanceTimersByTimeAsync(0);
      await new Promise((resolve) => realSetTimeout(resolve, 1));
    }
  };

  /** The first attempt finds the history one leaf behind the store; later ones find it caught up. */
  const lagOnce = () =>
    vi
      .mocked(authorization.judgeSolanaUserDecryption)
      .mockResolvedValueOnce(refusal('HandleBinding::ProofRecordBehind'));

  it('judges every attempt against the KMS context as it is then', async () => {
    lagOnce();
    vi.mocked(kmsContextAccount.fetchKmsContext)
      .mockResolvedValueOnce(kmsContext(false))
      .mockResolvedValue(kmsContext(true));
    const outcome = execute().catch((error: unknown) => error);
    await untilBackoff();
    await vi.advanceTimersByTimeAsync(2_000);
    expect(String(await outcome)).toMatch(/KmsContextDestroyed: KMS context .* is destroyed/);
    expect(kmsContextAccount.fetchKmsContext).toHaveBeenCalledTimes(2);
  });

  it('refuses a permit for another host chain at submission, as the relayer does', async () => {
    vi.mocked(hostConfigAccount.fetchHostConfig).mockResolvedValue({
      data: { chainId: hostChainId + 1n, gatewayChainId, decryptionContract },
    } as unknown as Awaited<ReturnType<typeof hostConfigAccount.fetchHostConfig>>);
    const error = await execute().catch((caught: unknown) => caught);
    expect(error).toBeInstanceOf(SolanaUserDecryptRunError);
    expect(error).toMatchObject({ attempts: 1, rejection: { kind: 'refused', label: 'host_chain_id_not_supported' } });
    expect(authorization.judgeSolanaUserDecryption).not.toHaveBeenCalled();
  });

  it('answers once the history catches up', async () => {
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
    expect(kmsContextAccount.fetchKmsContext).not.toHaveBeenCalled();
    expect(hostConfigAccount.fetchHostConfig).not.toHaveBeenCalled();
  });

  it('stops waiting between attempts when the signal aborts', async () => {
    lagOnce();
    const controller = new AbortController();
    const outcome = execute({ signal: controller.signal }).catch((error: unknown) => error);
    await untilBackoff();
    controller.abort();
    expect(await outcome).toBeInstanceOf(RelayerAbortError);
    expect(kmsContextAccount.fetchKmsContext).toHaveBeenCalledTimes(1);
  });

  it('gives up at the caller timeout, backoff included', async () => {
    vi.mocked(authorization.judgeSolanaUserDecryption).mockResolvedValue(refusal('HandleBinding::ProofRecordBehind'));
    const outcome = execute({ timeout: 1_000 }).catch((error: unknown) => error);
    await untilBackoff();
    await vi.advanceTimersByTimeAsync(1_000);
    expect(await outcome).toBeInstanceOf(RelayerTimeoutError);
  });
});
