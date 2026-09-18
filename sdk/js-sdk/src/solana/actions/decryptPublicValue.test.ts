import { RelayerAbortError } from '../../core/errors/RelayerAbortError.js';
import { describe, expect, it } from 'vitest';
import { hashTypedData } from 'viem';
import { privateKeyToAccount } from 'viem/accounts';
import { createKmsPublicDecryptEip712, publicDecryptDigest } from '../../core/kms/createKmsPublicDecryptEip712.js';
import { bytesToHex, hexToBytes } from '../../core/base/bytes.js';
import { toFhevmHandle } from '../../core/handle/FhevmHandle.js';
import { verifyPublicDecryptSignatures } from './decryptPublicValue.js';

const alice = privateKeyToAccount(`0x${'11'.repeat(32)}`);
const bob = privateKeyToAccount(`0x${'22'.repeat(32)}`);
const outsider = privateKeyToAccount(`0x${'33'.repeat(32)}`);
const handle = toFhevmHandle(`0x${'ab'.repeat(22)}80000000000030390500`);
const message = createKmsPublicDecryptEip712({
  chainId: 31337n,
  verifyingContractAddressDecryption: '0x0000000000000000000000000000000000000042',
  handles: [handle],
  decryptedResult: `0x${'00'.repeat(31)}2a`,
  extraData: `0x04${'44'.repeat(64)}`,
});
const signingMessage = {
  ...message,
  message: {
    ...message.message,
    ctHandles: message.message.ctHandles as readonly `0x${string}`[],
    decryptedResult: message.message.decryptedResult as `0x${string}`,
    extraData: message.message.extraData as `0x${string}`,
  },
};
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
  it.each(['cleartext', 'handle', 'store', 'context', 'chain', 'contract'])(
    'rejects a signature after changing %s',
    async (field) => {
      const signature = await alice.signTypedData(signingMessage);
      const changed = createKmsPublicDecryptEip712({
        chainId: field === 'chain' ? 31338n : 31337n,
        verifyingContractAddressDecryption:
          field === 'contract' ? '0x0000000000000000000000000000000000000043' : signingDomain.verifyingContract,
        handles: field === 'handle' ? [toFhevmHandle(`0x${'cd'.repeat(22)}80000000000030390500`)] : [handle],
        decryptedResult: field === 'cleartext' ? `0x${'00'.repeat(31)}2b` : message.message.decryptedResult,
        extraData:
          field === 'store'
            ? `0x04${'44'.repeat(32)}${'55'.repeat(32)}`
            : field === 'context'
              ? `0x04${'55'.repeat(32)}${'44'.repeat(32)}`
              : message.message.extraData,
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
import { getHostConfigEncoder, type HostConfigArgs } from '../internal/generated/zamaHost/accounts/hostConfig.js';
import { getKmsContextEncoder, type KmsContextArgs } from '../internal/generated/zamaHost/accounts/kmsContext.js';
import { findHostConfigPda } from '../internal/generated/zamaHost/pdas/hostConfig.js';
import { findKmsContextPda } from '../internal/generated/zamaHost/pdas/kmsContext.js';
import { getAddressEncoder } from '@solana/kit';
import { ZAMA_HOST_PROGRAM_ADDRESS } from '../internal/generated/zamaHost/programAddress.js';
import { createFhevmPublicDecryptClient } from '../clients/createFhevmPublicDecryptClient.js';
import { setFhevmRuntimeConfig } from '../internal/config.js';
import * as certificateModule from './publicDecryptCertificate.js';
import { asBytes32Hex } from '../../core/base/bytes.js';

const contextId = new Uint8Array(32).fill(0x44);
const store = new Uint8Array(32).fill(0x44);
const chain = {
  id: 9223372036854788153n,
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
    paused: false,
    grantDenyListEnabled: false,
    maxHcuPerTx: 1n,
    maxHcuDepthPerTx: 1n,
    hcuBlockCapPerApp: 1n,
    updatedSlot: 1n,
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
    handle: handle.bytes32Hex,
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

afterEach(() => vi.restoreAllMocks());

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
    expect(f.rpc.getMultipleAccounts).toHaveBeenCalledWith([f.configAddress, f.contextAddress], expect.anything());
    expect(f.request).toHaveBeenCalledWith(expect.anything(), expect.objectContaining({ contextId }));
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
  it.each([0, 1, 2, 255])('validates raw destroyed byte %s with trailing account bytes', async (flag) => {
    const f = await accountFixture();
    const row = f.contextAccount();
    const encoded = Buffer.from(row.data[0]!, 'base64');
    encoded[encoded.length - 2] = flag;
    row.data[0] = Buffer.concat([encoded, Buffer.alloc(8)]).toString('base64');
    vi.spyOn(f.rpc, 'getMultipleAccounts').mockReturnValueOnce({
      send: async () => ({ value: [f.configAccount(), row] }),
    } as never);
    const result = f.client.decryptPublicValue({ handle, encryptedStore: store });
    if (flag === 0) await expect(result).resolves.toMatchObject({ value: 42n });
    else await expect(result).rejects.toThrow('Invalid or destroyed KMS context');
  });
  it.each([31, 33])('rejects a %s-byte ABI result', async (size) => {
    const f = await accountFixture();
    f.claim.abiEncodedCleartext = '00'.repeat(size);
    await expect(f.client.decryptPublicValue({ handle, encryptedStore: store })).rejects.toThrow('32 bytes');
  });
  it('returns ordered typed values and rejects an empty batch before I/O', async () => {
    const f = await accountFixture();
    await expect(f.client.decryptPublicValues({ entries: [] })).rejects.toThrow('at least one');
    expect(f.request).not.toHaveBeenCalled();
    const values = await f.client.decryptPublicValues({
      entries: [
        { handle, encryptedStore: store },
        { handle, encryptedStore: store },
      ],
    });
    expect(values.map((value) => value.value)).toEqual([42n, 42n]);
  });
});
