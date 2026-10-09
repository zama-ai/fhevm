import { RelayerAbortError } from '../../core/errors/RelayerAbortError.js';
import { describe, expect, it } from 'vitest';
import { hashTypedData } from 'viem';
import { privateKeyToAccount } from 'viem/accounts';
import { createKmsPublicDecryptEip712 } from '../../core/kms/createKmsPublicDecryptEip712.js';
import { bytesToHex, hexToBytes } from '../../core/base/bytes.js';
import { toFhevmHandle } from '../../core/handle/FhevmHandle.js';
import { publicDecryptDigest, verifyPublicDecryptSignatures } from './decryptPublicValue.js';

const alice = privateKeyToAccount(`0x${'11'.repeat(32)}`);
const bob = privateKeyToAccount(`0x${'22'.repeat(32)}`);
const outsider = privateKeyToAccount(`0x${'33'.repeat(32)}`);
const handle = toFhevmHandle(`0x${'ab'.repeat(22)}01000000000030390500`);
const message = createKmsPublicDecryptEip712({
  chainId: 31337n,
  verifyingContractAddressDecryption: '0x0000000000000000000000000000000000000042',
  handles: [handle],
  decryptedResult: `0x${'00'.repeat(31)}2a`,
  extraData: `0x02${'44'.repeat(32)}${'45'.repeat(32)}`,
});
/** The SDK's EIP-712 message in the shape viem signs. */
const typedData = (eip712: ReturnType<typeof createKmsPublicDecryptEip712>) => ({
  ...eip712,
  message: {
    ...eip712.message,
    ctHandles: eip712.message.ctHandles as readonly `0x${string}`[],
    decryptedResult: eip712.message.decryptedResult as `0x${string}`,
    extraData: eip712.message.extraData as `0x${string}`,
  },
});
const signingMessage = typedData(message);
const { domain: signingDomain } = message;
const registered = [alice, bob].map(({ address }) => hexToBytes(address));

describe('Solana public decryption authentication', () => {
  it('hashes the canonical EVM schema identically to viem', () => {
    expect(bytesToHex(publicDecryptDigest(message))).toBe(hashTypedData(signingMessage));
  });
  it('accepts a distinct signer threshold while ignoring outsider signatures', async () => {
    const signatures = await Promise.all([alice, outsider, bob].map((signer) => signer.signTypedData(signingMessage)));
    expect(() => verifyPublicDecryptSignatures(publicDecryptDigest(message), signatures, registered, 2)).not.toThrow();
  });
  it('cannot reach the threshold by duplicating a valid signer', async () => {
    const signature = await alice.signTypedData(signingMessage);
    expect(() =>
      verifyPublicDecryptSignatures(publicDecryptDigest(message), [signature, signature], registered, 2),
    ).toThrow('threshold');
  });
  it.each(['cleartext', 'handle', 'context', 'chain', 'contract'])(
    'rejects a signature after changing %s',
    async (field) => {
      const signature = await alice.signTypedData(signingMessage);
      const changed = createKmsPublicDecryptEip712({
        chainId: field === 'chain' ? 31338n : 31337n,
        verifyingContractAddressDecryption:
          field === 'contract' ? '0x0000000000000000000000000000000000000043' : signingDomain.verifyingContract,
        handles: field === 'handle' ? [toFhevmHandle(`0x${'cd'.repeat(22)}01000000000030390500`)] : [handle],
        decryptedResult: field === 'cleartext' ? `0x${'00'.repeat(31)}2b` : message.message.decryptedResult,
        extraData: field === 'context' ? `0x02${'55'.repeat(32)}${'45'.repeat(32)}` : message.message.extraData,
      });
      expect(() => verifyPublicDecryptSignatures(publicDecryptDigest(changed), [signature], registered, 1)).toThrow(
        'threshold',
      );
    },
  );
  it('rejects raw parity and high-s encodings that the host rejects', async () => {
    const signature = await alice.signTypedData(signingMessage);
    const bytes = hexToBytes(signature);
    bytes[64] = bytes[64]! - 27;
    expect(() =>
      verifyPublicDecryptSignatures(publicDecryptDigest(message), [bytesToHex(bytes)], registered, 1),
    ).toThrow('threshold');
    const order = 0xfffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141n;
    const s = BigInt(`0x${signature.slice(66, 130)}`);
    const highS = `${signature.slice(0, 66)}${(order - s).toString(16).padStart(64, '0')}${signature.endsWith('1b') ? '1c' : '1b'}`;
    expect(() => verifyPublicDecryptSignatures(publicDecryptDigest(message), [highS], registered, 1)).toThrow(
      'threshold',
    );
  });
});

import { vi, afterEach } from 'vitest';
import type { SolanaRpc } from '../encryptedStore.js';
import {
  findHostConfigPda,
  findKmsContextPda,
  getHostConfigEncoder,
  getKmsContextEncoder,
  ZAMA_HOST_PROGRAM_ADDRESS,
  type HostConfigArgs,
  type KmsContextArgs,
} from '@fhevm/solana-zama-host';
import { getAddressEncoder } from '@solana/kit';
import { createFhevmPublicDecryptClient } from '../clients/createFhevmPublicDecryptClient.js';
import { setFhevmRuntimeConfig } from '../internal/config.js';
import * as certificateModule from './publicDecryptCertificate.js';
import { clearSolanaHostKmsReads, createSolanaHostKmsReads, userDecryptVerification } from './hostKms.js';
import { getSolanaRuntime } from '../internal/runtime.js';
import { asBytes32Hex } from '../../core/base/bytes.js';

const contextId = new Uint8Array(32).fill(0x44);
const epochId = new Uint8Array(32).fill(0x45);
const store = new Uint8Array(32).fill(0x44);
const chain = {
  id: 72057594037940281n,
  fhevm: {
    relayerUrl: 'https://relayer.example.test',
    programs: {
      host: {
        address: asBytes32Hex(bytesToHex(new Uint8Array(getAddressEncoder().encode(ZAMA_HOST_PROGRAM_ADDRESS)))),
      },
    },
  },
};

async function accountFixture() {
  const [configAddress, configBump] = await findHostConfigPda();
  const [contextAddress, contextBump] = await findKmsContextPda({ contextId });
  const config: HostConfigArgs = {
    admin: ZAMA_HOST_PROGRAM_ADDRESS,
    chainId: chain.id,
    gatewayChainId: 31337n,
    inputVerificationContract: new Uint8Array(20),
    coprocessorSigners: Array.from({ length: 8 }, () => new Uint8Array(20)),
    coprocessorSignerCount: 1,
    coprocessorThreshold: 1,
    decryptionContract: hexToBytes(signingDomain.verifyingContract),
    currentKmsContextId: contextId,
    currentKmsEpochId: epochId,
    paused: { execution: false, verifiedInputs: false, aclWrites: false },
    grantDenyListEnabled: false,
    maxHcuPerTx: 1n,
    maxHcuDepthPerTx: 1n,
    hcuBlockCapPerApp: 1n,
    bump: configBump,
  };
  const kms: KmsContextArgs = {
    contextId,
    signers: registered,
    thresholds: { publicDecryption: 1, userDecryption: 1, kmsGen: 1, mpc: 1 },
    destroyed: false,
    bump: contextBump,
  };
  const account = (data: Uint8Array) => ({
    data: [Buffer.from(data).toString('base64'), 'base64'],
    owner: ZAMA_HOST_PROGRAM_ADDRESS,
    executable: false,
    lamports: 1n,
    space: BigInt(data.length),
  });
  const configAccount = () => account(new Uint8Array(getHostConfigEncoder().encode(config)));
  const contextAccount = () => account(new Uint8Array(getKmsContextEncoder().encode(kms)));
  const rpc = {
    getAccountInfo: vi.fn(() => ({ send: async () => ({ value: configAccount() }) })),
    getMultipleAccounts: vi.fn(() => ({ send: async () => ({ value: [configAccount(), contextAccount()] }) })),
  } as unknown as SolanaRpc;
  const signature = await alice.signTypedData(signingMessage);
  const claim = {
    handles: [handle.bytes32Hex],
    abiEncodedCleartext: message.message.decryptedResult.slice(2),
    signatures: [signature.slice(2)],
    extraData: message.message.extraData,
  };
  const request = vi.spyOn(certificateModule, 'publicDecryptCertificate').mockResolvedValue(claim);
  setFhevmRuntimeConfig({});
  return {
    client: createFhevmPublicDecryptClient({ chain, rpc }),
    request,
    config,
    kms,
    configAddress,
    contextAddress,
    rpc,
    claim,
    configAccount,
    contextAccount,
  };
}

afterEach(() => {
  vi.useRealTimers();
  vi.unstubAllGlobals();
  vi.restoreAllMocks();
  clearSolanaHostKmsReads();
});

describe('public decrypt client account-to-plaintext flow', () => {
  it('does no I/O when already cancelled', async () => {
    const f = await accountFixture();
    const controller = new AbortController();
    controller.abort();
    await expect(
      f.client.decryptPublicValue({ handle, encryptedStore: store, options: { signal: controller.signal } }),
    ).rejects.toBeInstanceOf(RelayerAbortError);
    expect(f.rpc.getAccountInfo).not.toHaveBeenCalled();
    expect(f.request).not.toHaveBeenCalled();
  });
  it('fails a cancelled certificate request as the relayer does, before any read', async () => {
    const f = await accountFixture();
    const controller = new AbortController();
    controller.abort();
    await expect(
      f.client.publicDecryptCertificate({ handle, encryptedStore: store, options: { signal: controller.signal } }),
    ).rejects.toBeInstanceOf(RelayerAbortError);
    expect(f.rpc.getAccountInfo).not.toHaveBeenCalled();
    expect(f.request).not.toHaveBeenCalled();
  });
  it('forwards cancellation to the verification read and never returns cancelled plaintext', async () => {
    const f = await accountFixture();
    const controller = new AbortController();
    const send = vi.fn(async () => {
      controller.abort();
      return { value: [f.configAccount(), f.contextAccount()] };
    });
    vi.spyOn(f.rpc, 'getMultipleAccounts').mockReturnValueOnce({ send } as never);
    await expect(
      f.client.decryptPublicValue({ handle, encryptedStore: store, options: { signal: controller.signal } }),
    ).rejects.toBeInstanceOf(RelayerAbortError);
    expect(send).toHaveBeenCalledWith(expect.objectContaining({ abortSignal: controller.signal }));
  });
  it('reads the canonical accounts through its one RPC and returns a typed value', async () => {
    const f = await accountFixture();
    const value = await f.client.decryptPublicValue({ handle, encryptedStore: store });
    expect(value.type).toBe('uint64');
    expect(value.value).toBe(42n);
    expect(f.rpc.getMultipleAccounts).toHaveBeenCalledWith(
      [f.configAddress, f.contextAddress],
      expect.objectContaining({ commitment: 'finalized' }),
    );
    expect(f.request).toHaveBeenCalledWith(expect.anything(), expect.objectContaining({ contextId, epochId }));
  });
  it("reads accounts under the chain's host program, not the bundled one", async () => {
    const f = await accountFixture();
    // Same RPC and fixture accounts (owned by the generated id); only the chain names another host.
    const other = {
      ...chain,
      fhevm: { ...chain.fhevm, programs: { host: { address: asBytes32Hex(`0x${'22'.repeat(32)}`) } } },
    };
    const client = createFhevmPublicDecryptClient({ chain: other, rpc: f.rpc });
    await expect(client.decryptPublicValue({ handle, encryptedStore: store })).rejects.toThrow('Invalid host account');
  });
  it.each(['destroyed', 'context', 'bump', 'chain', 'domain', 'zero-domain'])(
    'rejects %s changed while waiting for the certificate',
    async (field) => {
      const f = await accountFixture();
      f.request.mockImplementationOnce(async () => {
        if (field === 'destroyed') f.kms.destroyed = true;
        if (field === 'context') f.kms.contextId = new Uint8Array(32).fill(0x55);
        if (field === 'bump') f.kms.bump = (f.kms.bump + 1) % 256;
        if (field === 'chain') f.config.chainId = chain.id + 1n;
        if (field === 'domain') f.config.gatewayChainId = 31338n;
        if (field === 'zero-domain') f.config.decryptionContract = new Uint8Array(20);
        return f.claim;
      });
      await expect(f.client.decryptPublicValue({ handle, encryptedStore: store })).rejects.toThrow();
    },
  );
  it.each([
    ['a version 1 routing of the context', `0x01${'44'.repeat(32)}`],
    ['another epoch', `0x02${'44'.repeat(32)}${'46'.repeat(32)}`],
  ])('rejects a certificate signed over %s', async (_case, extraData) => {
    const f = await accountFixture();
    const other = createKmsPublicDecryptEip712({
      chainId: 31337n,
      verifyingContractAddressDecryption: signingDomain.verifyingContract,
      handles: [handle],
      decryptedResult: message.message.decryptedResult,
      extraData,
    });
    const signature = await alice.signTypedData(typedData(other));
    f.request.mockResolvedValueOnce({ ...f.claim, signatures: [signature.slice(2)], extraData });
    await expect(f.client.decryptPublicValue({ handle, encryptedStore: store })).rejects.toThrow(
      'does not name the requested KMS context and epoch',
    );
  });
  it('routes a certificate request to the context and epoch HostConfig holds', async () => {
    const f = await accountFixture();
    f.request.mockRestore();
    vi.useFakeTimers();
    const fetchMock = vi
      .fn()
      .mockResolvedValueOnce(
        new Response(JSON.stringify({ status: 'queued', requestId: 'r1', result: { jobId: 'j1' } }), {
          status: 202,
          headers: { 'Retry-After': '1' },
        }),
      )
      .mockResolvedValueOnce(
        new Response(
          JSON.stringify({
            status: 'succeeded',
            requestId: 'r1',
            result: {
              decryptedValue: f.claim.abiEncodedCleartext,
              signatures: f.claim.signatures,
              extraData: f.claim.extraData,
            },
          }),
          { status: 200 },
        ),
      );
    vi.stubGlobal('fetch', fetchMock);
    const pending = f.client.publicDecryptCertificate({ handle, encryptedStore: store });
    // The request starts after the HostConfig read; then run out the relayer's Retry-After.
    await vi.waitFor(() => expect(fetchMock).toHaveBeenCalledTimes(1));
    await vi.runAllTimersAsync();
    const claim = await pending;
    const body = JSON.parse(String((fetchMock.mock.calls[0]?.[1] as RequestInit).body)) as { extraData: string };
    expect(body.extraData).toBe(`0x02${'44'.repeat(32)}${'45'.repeat(32)}`);
    expect(claim.extraData).toBe(body.extraData);
  });
  it('certifies one handle as a batch of one under the active context and epoch', async () => {
    const f = await accountFixture();
    const claim = await f.client.publicDecryptCertificate({ handle, encryptedStore: store });
    expect(f.request).toHaveBeenCalledWith(expect.anything(), {
      contextId,
      epochId,
      options: undefined,
      entries: [{ handle, encryptedStore: store }],
    });
    const { handles: _handles, ...rest } = f.claim;
    expect(claim).toEqual({ ...rest, handle: handle.bytes32Hex });
  });
  it.each([
    ['two handles', [handle.bytes32Hex, handle.bytes32Hex]],
    ['another handle', [`0x${'07'.repeat(22)}01${'00'.repeat(9)}`]],
  ])('refuses a single certificate covering %s', async (_case, handles) => {
    const f = await accountFixture();
    f.request.mockResolvedValueOnce({ ...f.claim, handles });
    await expect(f.client.publicDecryptCertificate({ handle, encryptedStore: store })).rejects.toThrow(
      'exactly the requested handle',
    );
  });
  it('keeps the originally requested context when a rotation leaves it live', async () => {
    const f = await accountFixture();
    f.request.mockImplementationOnce(async () => {
      f.config.currentKmsContextId = new Uint8Array(32).fill(0x55);
      return f.claim;
    });
    await expect(f.client.decryptPublicValue({ handle, encryptedStore: store })).resolves.toMatchObject({ value: 42n });
  });
  it.each(['owner', 'discriminator', 'missing'])('rejects a %s account response', async (field) => {
    const f = await accountFixture();
    const row = f.contextAccount();
    if (field === 'owner') row.owner = '11111111111111111111111111111111' as typeof row.owner;
    if (field === 'discriminator') {
      const bytes = Buffer.from(row.data[0]!, 'base64');
      bytes[0] = bytes[0]! ^ 1;
      row.data[0] = bytes.toString('base64');
    }
    vi.spyOn(f.rpc, 'getMultipleAccounts').mockReturnValueOnce({
      send: async () => ({ value: [f.configAccount(), field === 'missing' ? null : row] }),
    } as never);
    await expect(f.client.decryptPublicValue({ handle, encryptedStore: store })).rejects.toThrow(
      'Invalid host account',
    );
  });
  it('reads a KMS context account with trailing bytes', async () => {
    const f = await accountFixture();
    const row = f.contextAccount();
    row.data[0] = Buffer.concat([Buffer.from(row.data[0]!, 'base64'), Buffer.alloc(8)]).toString('base64');
    vi.spyOn(f.rpc, 'getMultipleAccounts').mockReturnValueOnce({
      send: async () => ({ value: [f.configAccount(), row] }),
    } as never);
    await expect(f.client.decryptPublicValue({ handle, encryptedStore: store })).resolves.toMatchObject({ value: 42n });
  });
  it.each([31, 33])('rejects a %s-byte ABI result', async (size) => {
    const f = await accountFixture();
    f.claim.abiEncodedCleartext = '00'.repeat(size);
    await expect(f.client.decryptPublicValue({ handle, encryptedStore: store })).rejects.toThrow('32 bytes per handle');
  });
  it('rejects an empty or oversized batch before I/O', async () => {
    const f = await accountFixture();
    await expect(f.client.decryptPublicValues({ entries: [] })).rejects.toThrow('at least one');
    const oversized = Array.from({ length: 33 }, () => ({ handle, encryptedStore: store }));
    await expect(f.client.decryptPublicValues({ entries: oversized })).rejects.toThrow('at most 32');
    expect(f.rpc.getAccountInfo).not.toHaveBeenCalled();
    expect(f.request).not.toHaveBeenCalled();
  });
  it('decodes a batch in order, each word by its own type, from one certificate and one host read', async () => {
    const f = await accountFixture();
    // An ebool beside the euint64: a word decoded under the wrong handle's type would come back 1n.
    const other = toFhevmHandle(`0x${'cd'.repeat(22)}01000000000030390000`);
    const otherStore = new Uint8Array(32).fill(0x66);
    const batch = createKmsPublicDecryptEip712({
      chainId: 31337n,
      verifyingContractAddressDecryption: signingDomain.verifyingContract,
      handles: [handle, other],
      decryptedResult: `0x${'00'.repeat(31)}2a${'00'.repeat(31)}01`,
      extraData: message.message.extraData,
    });
    const signature = await alice.signTypedData(typedData(batch));
    f.request.mockResolvedValueOnce({
      handles: [handle.bytes32Hex, other.bytes32Hex],
      abiEncodedCleartext: batch.message.decryptedResult.slice(2),
      signatures: [signature.slice(2)],
      extraData: batch.message.extraData,
    });
    const entries = [
      { handle, encryptedStore: store },
      { handle: other, encryptedStore: otherStore },
    ];
    const values = await f.client.decryptPublicValues({ entries });
    expect(values.map((value) => value.value)).toEqual([42n, true]);
    expect(f.request).toHaveBeenCalledTimes(1);
    expect(f.request).toHaveBeenCalledWith(expect.anything(), { entries, contextId, epochId, options: undefined });
    expect(f.rpc.getAccountInfo).toHaveBeenCalledTimes(1);
    expect(f.rpc.getAccountInfo).toHaveBeenCalledWith(
      f.configAddress,
      expect.objectContaining({ commitment: 'finalized' }),
    );
    expect(f.rpc.getMultipleAccounts).toHaveBeenCalledTimes(1);
  });
  it('rejects a batch certificate whose cleartext is short by one handle', async () => {
    const f = await accountFixture();
    await expect(
      f.client.decryptPublicValues({
        entries: [
          { handle, encryptedStore: store },
          { handle, encryptedStore: store },
        ],
      }),
    ).rejects.toThrow('32 bytes per handle');
  });
});

describe('the host KMS reads', () => {
  const contextServed = (f: Awaited<ReturnType<typeof accountFixture>>) =>
    vi
      .spyOn(f.rpc, 'getMultipleAccounts')
      .mockImplementation(() => ({ send: async () => ({ value: [f.contextAccount()] }) }) as never);

  it('routes repeated decryptions through one HostConfig read, and verifies each against a fresh one', async () => {
    const f = await accountFixture();
    await f.client.decryptPublicValue({ handle, encryptedStore: store });
    await f.client.decryptPublicValue({ handle, encryptedStore: store });
    expect(f.rpc.getAccountInfo).toHaveBeenCalledTimes(1);
    expect(f.rpc.getMultipleAccounts).toHaveBeenCalledTimes(2);
  });
  // Callers that build a client per operation still share the read: the cache is keyed by
  // runtime, host program and chain, not by client.
  it('shares one HostConfig read between concurrent and later callers, across clients', async () => {
    const f = await accountFixture();
    const hostReads = () => createSolanaHostKmsReads({ chain, rpc: f.rpc }, getSolanaRuntime());
    await Promise.all([hostReads().config(), hostReads().config()]);
    await hostReads().config();
    expect(f.rpc.getAccountInfo).toHaveBeenCalledTimes(1);
  });
  it('refuses a HostConfig of another chain, and caches nothing', async () => {
    const f = await accountFixture();
    const host = createSolanaHostKmsReads({ chain, rpc: f.rpc }, getSolanaRuntime());
    f.config.chainId = chain.id + 1n;
    await expect(host.config()).rejects.toThrow('Host configuration does not match the client');
    f.config.chainId = chain.id;
    await expect(host.config()).resolves.toMatchObject({ chainId: chain.id });
    expect(f.rpc.getAccountInfo).toHaveBeenCalledTimes(2);
  });
  it('refuses a destroyed context, and reads it again on the next call', async () => {
    const f = await accountFixture();
    const read = contextServed(f);
    const host = createSolanaHostKmsReads({ chain, rpc: f.rpc }, getSolanaRuntime());
    f.kms.destroyed = true;
    await expect(host.kmsContext(contextId)).rejects.toThrow('Invalid or destroyed KMS context');
    f.kms.destroyed = false;
    await expect(host.kmsContext(contextId)).resolves.toMatchObject({ destroyed: false });
    await host.kmsContext(contextId);
    expect(read).toHaveBeenCalledTimes(2);
  });
  it('refuses a context the host never defined', async () => {
    const f = await accountFixture();
    vi.spyOn(f.rpc, 'getMultipleAccounts').mockReturnValue({ send: async () => ({ value: [null] }) } as never);
    const host = createSolanaHostKmsReads({ chain, rpc: f.rpc }, getSolanaRuntime());
    await expect(host.kmsContext(contextId)).rejects.toThrow('Invalid host account');
  });
  it.each(['context', 'epoch'])('refuses to route while HostConfig holds no %s', async (field) => {
    const f = await accountFixture();
    if (field === 'context') f.config.currentKmsContextId = new Uint8Array(32);
    else f.config.currentKmsEpochId = new Uint8Array(32);
    await expect(f.client.decryptPublicValue({ handle, encryptedStore: store })).rejects.toThrow(
      'KMS context is not configured',
    );
    expect(f.request).not.toHaveBeenCalled();
  });
  // A permit stays answerable after a context switch: its response is checked against the context
  // it names, whose signers are parties 1..n in registered order.
  it("trusts the signers of the permit's context as parties 1..n, under the host's gateway domain", async () => {
    const f = await accountFixture();
    const permitContextId = new Uint8Array(32).fill(0x55);
    const [permitContext, bump] = await findKmsContextPda({ contextId: permitContextId });
    Object.assign(f.kms, {
      contextId: permitContextId,
      bump,
      signers: [bob, alice].map(({ address }) => hexToBytes(address)),
    });
    const read = contextServed(f);
    const host = createSolanaHostKmsReads({ chain, rpc: f.rpc }, getSolanaRuntime());

    await expect(userDecryptVerification(host, permitContextId, 'test')).resolves.toEqual({
      signers: [
        { partyId: 1, address: bob.address.toLowerCase() },
        { partyId: 2, address: alice.address.toLowerCase() },
      ],
      fheParameter: 'test',
      gatewayEip712Domain: {
        name: 'Decryption',
        version: '1',
        chainId: 31337n,
        verifyingContract: signingDomain.verifyingContract,
      },
    });
    expect(read).toHaveBeenCalledWith([permitContext], expect.objectContaining({ commitment: 'finalized' }));
  });
});
