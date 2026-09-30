import * as envelope from '../permit/envelope.js';
import * as encryptedStore from '../encryptedStore.js';
import * as revokePermits from '../actions/revokePermits.js';
import * as delegation from '../actions/userDecryptionDelegation.js';
import * as storeHistory from './storeHistory.js';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { address, getAddressEncoder, type Address } from '@solana/kit';
import type { SolanaRpc } from '../encryptedStore.js';
import type { SolanaPermitFields } from '../permit/types.js';
import type { SolanaUserDecryptHandleEntry } from '../userDecrypt/index.js';
import { userDecryptRejection } from './decrypt.js';

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
  const data = new Uint8Array(40);
  new DataView(data.buffer).setBigInt64(32, unixTimestamp, true);
  const value = {
    data: [Buffer.from(data).toString('base64'), 'base64'],
    executable: false,
    lamports: 1n,
    owner: 'Sysvar1111111111111111111111111111111111111',
    space: 40n,
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
