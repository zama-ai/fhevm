import * as envelope from '../permit/envelope.js';
import * as encryptedStore from '../encryptedStore.js';
import * as revokePermits from '../actions/revokePermits.js';
import * as delegation from '../actions/userDecryptionDelegation.js';
import * as storeHistory from './storeHistory.js';
import * as storeValues from './storeValues.js';
import * as hostConfigAccount from '../internal/generated/zamaHost/accounts/hostConfig.js';
import * as kmsContextAccount from '../internal/generated/zamaHost/accounts/kmsContext.js';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { address, getAddressEncoder, type Address } from '@solana/kit';
import { getSysvarClockEncoder } from '@solana/sysvars';
import type { FhevmSolanaChain } from '../../core/types/fhevmSolanaChain.js';
import type { SolanaDecryptTrust } from '../clients/decorators/permitDecrypt.js';
import type { SolanaRpc } from '../encryptedStore.js';
import type { SolanaPermitFields } from '../permit/types.js';
import type { SolanaUserDecryptHandleEntry } from '../userDecrypt/index.js';
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
import { cleartextUserDecryptExecution, userDecryptRejection } from './decrypt.js';

const host = address('11111111111111111111111111111112');
const signer = address('SysvarRent111111111111111111111111111111111');
const delegator = address('SysvarS1otHashes111111111111111111111111111');
const store = address('SysvarStakeHistory1111111111111111111111111');
const app = address('SysvarRecentB1ockHashes11111111111111111111');
const bytes = (value: Address): Uint8Array => new Uint8Array(getAddressEncoder().encode(value));
const handle = new Uint8Array(32).fill(7);
const now = BigInt(Math.floor(Date.now() / 1000));

/** An RPC that serves only the Clock sysvar, at `unixTimestamp`. */
function rpcAtClock(unixTimestamp: bigint): SolanaRpc {
  const data = getSysvarClockEncoder().encode({
    slot: 0n,
    epochStartTimestamp: unixTimestamp,
    epoch: 0n,
    leaderScheduleEpoch: 0n,
    unixTimestamp,
  } as Parameters<ReturnType<typeof getSysvarClockEncoder>['encode']>[0]);
  const value = {
    data: [Buffer.from(data).toString('base64'), 'base64'],
    executable: false,
    lamports: 1n,
    owner: 'Sysvar1111111111111111111111111111111111111',
    space: BigInt(data.length),
  };
  return { getAccountInfo: () => ({ send: () => Promise.resolve({ value }) }) } as unknown as SolanaRpc;
}

let fields: Record<string, unknown>;
let entry: SolanaUserDecryptHandleEntry;
let hostState: Parameters<typeof userDecryptRejection>[2];
let rpc: SolanaRpc;

const judge = () =>
  userDecryptRejection(rpc, host, hostState, fields as unknown as SolanaPermitFields, new Uint8Array(64), [entry]);
const refusedWith = (message: RegExp) => ({
  kind: 'refused',
  label: 'not_allowed_on_host_acl',
  message: expect.stringMatching(message),
});

beforeEach(() => {
  fields = {
    chainId: 5n,
    kmsRouting: { kmsContextId: new Uint8Array(32).fill(1) },
    startTimestamp: now - 10n,
    durationSeconds: 100n,
    verifyingProgramId: bytes(host),
    userAddress: bytes(signer),
    allowedScopes: [],
  };
  entry = { handle, ownerAddress: bytes(signer), encryptedStore: bytes(store) } as SolanaUserDecryptHandleEntry;
  hostState = { config: { chainId: 5n }, context: { destroyed: false } } as unknown as typeof hostState;
  rpc = rpcAtClock(now);
  vi.spyOn(envelope, 'verifySolanaPermitSignature').mockReturnValue(undefined as never);
  vi.spyOn(revokePermits, 'fetchSolanaPermitInvalidation').mockResolvedValue(0n);
  vi.spyOn(encryptedStore, 'fetchSolanaEncryptedStore').mockResolvedValue({
    program: app,
    scope: host,
    leafCount: 1n,
  } as unknown as encryptedStore.SolanaEncryptedStore);
  vi.spyOn(storeHistory, 'fetchSolanaStoreHistory').mockResolvedValue([
    { kind: 'allowed', handle, key: bytes(signer) },
  ]);
});

afterEach(() => vi.restoreAllMocks());

describe('userDecryptRejection', () => {
  it('answers a request the Connector would authorize', async () => {
    await expect(judge()).resolves.toBeUndefined();
  });

  it('refuses a permit for another host chain', async () => {
    fields.chainId = 6n;
    await expect(judge()).resolves.toEqual(refusedWith(/host chain 6/));
  });

  it('refuses under a destroyed KMS context', async () => {
    hostState = { ...hostState, context: { ...hostState.context, destroyed: true } };
    await expect(judge()).resolves.toEqual(refusedWith(/is destroyed/));
  });

  it('refuses a permit whose signature does not verify', async () => {
    vi.mocked(envelope.verifySolanaPermitSignature).mockImplementation(() => {
      throw new Error('bad signature');
    });
    await expect(judge()).resolves.toEqual(refusedWith(/bad signature/));
  });

  it('refuses outside the validity window', async () => {
    fields.startTimestamp = now + 10n;
    await expect(judge()).resolves.toEqual(refusedWith(/is valid from/));
  });

  it('refuses a permit signed for another host program', async () => {
    fields.verifyingProgramId = bytes(app);
    await expect(judge()).resolves.toEqual(refusedWith(/signed for host program/));
  });

  it('refuses a permit that starts before the revocation watermark', async () => {
    vi.mocked(revokePermits.fetchSolanaPermitInvalidation).mockResolvedValue(now);
    await expect(judge()).resolves.toEqual(refusedWith(/revoked permits up to/));
  });

  it('refuses a store it cannot read, as the Connector does', async () => {
    vi.mocked(encryptedStore.fetchSolanaEncryptedStore).mockRejectedValue(new Error('does not exist'));
    await expect(judge()).resolves.toEqual(refusedWith(/entry 0: Error: does not exist/));
  });

  it("refuses a store outside the permit's scopes, and admits every store under no scope", async () => {
    fields.allowedScopes = [new Uint8Array([...bytes(app), ...bytes(app)])];
    await expect(judge()).resolves.toEqual(refusedWith(/scopes do not include/));
    fields.allowedScopes = [new Uint8Array([...bytes(app), ...bytes(host)])];
    await expect(judge()).resolves.toBeUndefined();
  });

  it('refuses a delegated entry at the second its delegation expires, and answers it before', async () => {
    entry = { ...entry, ownerAddress: bytes(delegator) };
    vi.mocked(storeHistory.fetchSolanaStoreHistory).mockResolvedValue([
      { kind: 'allowed', handle, key: bytes(delegator) },
    ]);
    const live = { expiresAt: now } as delegation.SolanaUserDecryptionDelegationRecord;
    vi.spyOn(delegation, 'fetchSolanaUserDecryptionDelegation').mockResolvedValue({ exact: null, wildcard: live });
    await expect(judge()).resolves.toEqual(refusedWith(/no live delegation/));
    rpc = rpcAtClock(now - 1n);
    await expect(judge()).resolves.toBeUndefined();
  });

  it("refuses an entry without the owner's allow leaf, even when the signer has one", async () => {
    entry = { ...entry, ownerAddress: bytes(delegator) };
    vi.spyOn(delegation, 'fetchSolanaUserDecryptionDelegation').mockResolvedValue({
      exact: { expiresAt: now + 60n } as delegation.SolanaUserDecryptionDelegationRecord,
      wildcard: null,
    });
    await expect(judge()).resolves.toEqual(refusedWith(/is not allowed on handle/));
  });

  it('leaves the request unanswered while the history does not reach the leaf count', async () => {
    vi.mocked(encryptedStore.fetchSolanaEncryptedStore).mockResolvedValue({
      program: app,
      scope: host,
      leafCount: 2n,
    } as unknown as encryptedStore.SolanaEncryptedStore);
    await expect(judge()).resolves.toEqual({ kind: 'unanswered' });
  });
});

describe('userDecryptRejection reads', () => {
  it('reads each store and its history once, however many entries name it', async () => {
    const other = new Uint8Array(32).fill(8);
    vi.mocked(storeHistory.fetchSolanaStoreHistory).mockResolvedValue([
      { kind: 'allowed', handle, key: bytes(signer) },
      { kind: 'allowed', handle: other, key: bytes(signer) },
    ]);
    vi.mocked(encryptedStore.fetchSolanaEncryptedStore).mockResolvedValue({
      program: app,
      scope: host,
      leafCount: 2n,
    } as unknown as encryptedStore.SolanaEncryptedStore);
    const entries = [entry, { ...entry, handle: other }, entry];
    const rejection = await userDecryptRejection(
      rpc,
      host,
      hostState,
      fields as unknown as SolanaPermitFields,
      new Uint8Array(64),
      entries,
    );
    expect(rejection).toBeUndefined();
    expect(encryptedStore.fetchSolanaEncryptedStore).toHaveBeenCalledTimes(1);
    expect(storeHistory.fetchSolanaStoreHistory).toHaveBeenCalledTimes(1);
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
    vi.mocked(storeHistory.fetchSolanaStoreHistory).mockResolvedValue([
      { kind: 'allowed', handle: permitHandle, key: bytes(signer) },
    ]);
  });

  afterEach(() => vi.useRealTimers());

  /** Runs until the retry loop waits: its backoff is then the one pending timer. */
  const untilBackoff = async () => {
    for (let step = 0; vi.getTimerCount() === 0; step += 1) {
      if (step === 100) throw new Error('the run never reached its backoff');
      await vi.advanceTimersByTimeAsync(0);
    }
  };

  /** The first attempt finds the history one leaf behind the store; later ones find it caught up. */
  const lagOnce = () =>
    vi.mocked(encryptedStore.fetchSolanaEncryptedStore).mockResolvedValueOnce({
      program: app,
      scope: host,
      leafCount: 2n,
    } as unknown as encryptedStore.SolanaEncryptedStore);

  it('judges every attempt against the KMS context as it is then', async () => {
    lagOnce();
    vi.mocked(kmsContextAccount.fetchKmsContext)
      .mockResolvedValueOnce(kmsContext(false))
      .mockResolvedValue(kmsContext(true));
    const outcome = execute().catch((error: unknown) => error);
    await untilBackoff();
    await vi.advanceTimersByTimeAsync(2_000);
    const error = await outcome;
    expect(error).toBeInstanceOf(SolanaUserDecryptRunError);
    expect(error).toMatchObject({ attempts: 2, rejection: refusedWith(/is destroyed/) });
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
    vi.mocked(encryptedStore.fetchSolanaEncryptedStore).mockResolvedValue({
      program: app,
      scope: host,
      leafCount: 2n,
    } as unknown as encryptedStore.SolanaEncryptedStore);
    const outcome = execute({ timeout: 1_000 }).catch((error: unknown) => error);
    await untilBackoff();
    await vi.advanceTimersByTimeAsync(1_000);
    expect(await outcome).toBeInstanceOf(RelayerTimeoutError);
  });
});
