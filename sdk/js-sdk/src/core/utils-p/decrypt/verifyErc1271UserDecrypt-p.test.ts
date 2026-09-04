import type { EthCallResult, EthereumModule } from '../../modules/ethereum/types.js';
import type { RelayerModule } from '../../modules/relayer/types.js';
import type { KmsUserDecryptEip712V2 } from '../../types/kms.js';
import type { Logger } from '../../types/logger.js';
import type { Address, BytesHex, ChecksummedAddress } from '../../types/primitives.js';
import { describe, expect, it, vi } from 'vitest';
import { PRIVATE_ETHERS_TOKEN } from '../../../ethers/internal/ethers-p.js';
import { addressToChecksummedAddress } from '../../base/address.js';
import { remove0x } from '../../base/string.js';
import { sepolia } from '../../chains/definitions/sepolia.js';
import {
  Erc1271EmptySigOnEoaError,
  Erc1271EoaMismatchNoCodeError,
  Erc1271RejectedError,
  Erc1271WrongMagicError,
} from '../../errors/Erc1271Error.js';
import { createCoreFhevm } from '../../runtime/CoreFhevm-p.js';
import { createFhevmRuntime } from '../../runtime/CoreFhevmRuntime-p.js';
import { ERC1271_MAGIC_VALUE, verifyErc1271UserDecrypt } from './verifyErc1271UserDecrypt-p.js';

////////////////////////////////////////////////////////////////////////////////

const WALLET = addressToChecksummedAddress(`0x${'de'.repeat(20)}` as Address);
const OWNER = addressToChecksummedAddress(`0x${'ab'.repeat(20)}` as Address);

const SIG_65 = ('0x' + '11'.repeat(65)) as BytesHex;
const SIG_130 = ('0x' + '22'.repeat(130)) as BytesHex; // concatenated 2-of-N multisig blob
const SIG_EMPTY = '0x' as BytesHex; // approveHash flow
const SIG_1 = '0xff' as BytesHex; // unparsable

const RETURN_MAGIC = (ERC1271_MAGIC_VALUE + '00'.repeat(28)) as BytesHex; // 32-byte left-aligned
const RETURN_WRONG_MAGIC = ('0x' + '00'.repeat(32)) as BytesHex;
const RETURN_EMPTY = '0x' as BytesHex;
const RETURN_SHORT = ERC1271_MAGIC_VALUE as BytesHex; // only 4 bytes

const DIGEST = '0x' + 'cd'.repeat(32);

////////////////////////////////////////////////////////////////////////////////

function makeEip712(): KmsUserDecryptEip712V2 {
  return {
    domain: { name: 'Decryption', version: '1', chainId: 11155111n, verifyingContract: WALLET },
    types: { UserDecryptRequestVerification: [{ name: 'userAddress', type: 'address' }] },
    primaryType: 'UserDecryptRequestVerification',
    message: {
      userAddress: WALLET,
      publicKey: '0x',
      allowedContracts: [],
      startTimestamp: '0',
      durationSeconds: '1',
      extraData: '0x',
    },
  } as unknown as KmsUserDecryptEip712V2;
}

type MockOptions = {
  readonly call?: EthereumModule['call'];
  readonly recover?: EthereumModule['recoverTypedDataAddress'];
  readonly logger?: Logger;
};

function makeEthereum(opts: MockOptions): EthereumModule {
  return {
    hashTypedData: vi.fn(() => DIGEST),
    // Default: recovers a *different* address, forcing the ERC-1271 fall-through.
    recoverTypedDataAddress: opts.recover ?? vi.fn(async () => OWNER),
    // Concatenate the raw values so the emitted calldata embeds the full,
    // untruncated signature blob — enough to assert against.
    encode: vi.fn(
      ({ values }: { values: readonly unknown[] }) =>
        ('0x' + values.map((v) => remove0x(String(v))).join('')) as BytesHex,
    ),
    call: opts.call ?? vi.fn(async (): Promise<EthCallResult> => ({ success: true, data: RETURN_MAGIC })),
  } as unknown as EthereumModule;
}

/** A context with a valid (sealed) trusted client — the ERC-1271 STATICCALL path runs. */
function makeContext(opts: MockOptions) {
  const ethereum = makeEthereum(opts);
  const runtime = createFhevmRuntime(PRIVATE_ETHERS_TOKEN, {
    ethereum,
    relayer: {} as RelayerModule,
    config: opts.logger !== undefined ? { logger: opts.logger } : {},
  });
  const context = createCoreFhevm(PRIVATE_ETHERS_TOKEN, { chain: sepolia, client: {}, runtime });
  return { context, ethereum };
}

function queuedCall(...results: EthCallResult[]): EthereumModule['call'] {
  const queue = [...results];
  return vi.fn(async (): Promise<EthCallResult> => {
    const next = queue.shift();
    if (next === undefined) {
      throw new Error('mock call: no queued response');
    }
    return next;
  });
}

////////////////////////////////////////////////////////////////////////////////

describe('verifyErc1271UserDecrypt', () => {
  it('accepts a valid EOA signature via the fast path without any RPC', async () => {
    const { context, ethereum } = makeContext({ recover: vi.fn(async () => WALLET) });

    await expect(
      verifyErc1271UserDecrypt(context, { userAddress: WALLET, eip712: makeEip712(), signature: SIG_65 }),
    ).resolves.toBeUndefined();

    // Reaching Ok without consuming a queued call proves the EOA fast path was used.
    expect(ethereum.call).toHaveBeenCalledTimes(0);
  });

  it('rejects an EOA-mismatch signature when userAddress has no contract code', async () => {
    const { context } = makeContext({
      recover: vi.fn(async () => OWNER),
      call: queuedCall({ success: true, data: RETURN_EMPTY }),
    });

    await expect(
      verifyErc1271UserDecrypt(context, { userAddress: WALLET, eip712: makeEip712(), signature: SIG_65 }),
    ).rejects.toBeInstanceOf(Erc1271EoaMismatchNoCodeError);
  });

  it('rejects an empty signature when userAddress has no contract code', async () => {
    const { context } = makeContext({ call: queuedCall({ success: true, data: RETURN_EMPTY }) });

    await expect(
      verifyErc1271UserDecrypt(context, { userAddress: WALLET, eip712: makeEip712(), signature: SIG_EMPTY }),
    ).rejects.toBeInstanceOf(Erc1271EmptySigOnEoaError);
  });

  it('accepts when isValidSignature returns the magic value', async () => {
    const { context } = makeContext({ call: queuedCall({ success: true, data: RETURN_MAGIC }) });

    await expect(
      verifyErc1271UserDecrypt(context, { userAddress: WALLET, eip712: makeEip712(), signature: SIG_65 }),
    ).resolves.toBeUndefined();
  });

  it('rejects when isValidSignature returns a non-magic value', async () => {
    const { context } = makeContext({ call: queuedCall({ success: true, data: RETURN_WRONG_MAGIC }) });

    await expect(
      verifyErc1271UserDecrypt(context, { userAddress: WALLET, eip712: makeEip712(), signature: SIG_65 }),
    ).rejects.toBeInstanceOf(Erc1271WrongMagicError);
  });

  it('rejects when isValidSignature reverts', async () => {
    const { context } = makeContext({ call: queuedCall({ success: false, reason: 'execution reverted' }) });

    await expect(
      verifyErc1271UserDecrypt(context, { userAddress: WALLET, eip712: makeEip712(), signature: SIG_65 }),
    ).rejects.toBeInstanceOf(Erc1271RejectedError);
  });

  it('rejects when isValidSignature returns malformed (short) returndata', async () => {
    const { context } = makeContext({ call: queuedCall({ success: true, data: RETURN_SHORT }) });

    await expect(
      verifyErc1271UserDecrypt(context, { userAddress: WALLET, eip712: makeEip712(), signature: SIG_65 }),
    ).rejects.toBeInstanceOf(Erc1271RejectedError);
  });

  it('forwards a 130-byte multisig blob to ERC-1271 without truncation', async () => {
    const call = queuedCall({ success: true, data: RETURN_MAGIC });
    const { context } = makeContext({ call });

    await expect(
      verifyErc1271UserDecrypt(context, { userAddress: WALLET, eip712: makeEip712(), signature: SIG_130 }),
    ).resolves.toBeUndefined();

    // Calldata assertion: the full 130-byte blob (260 hex chars) must appear in
    // the eth_call data — proving it was not truncated to a single 65-byte sig.
    expect(call).toHaveBeenCalledTimes(1);
    const mockCalls = (call as unknown as { mock: { calls: [unknown, { data: string; to: string }][] } }).mock.calls;
    const callArgs = mockCalls[0];
    if (callArgs === undefined) {
      throw new Error('expected the mocked eth_call to have been invoked');
    }
    const { data, to } = callArgs[1];
    expect(to).toBe(WALLET);
    expect(data.startsWith(ERC1271_MAGIC_VALUE)).toBe(true); // selector == magic
    expect(data).toContain(remove0x(SIG_130));
  });

  it('accepts the empty approveHash signature when the wallet returns magic', async () => {
    const { context } = makeContext({ call: queuedCall({ success: true, data: RETURN_MAGIC }) });

    await expect(
      verifyErc1271UserDecrypt(context, { userAddress: WALLET, eip712: makeEip712(), signature: SIG_EMPTY }),
    ).resolves.toBeUndefined();
  });

  it('falls through an unparsable signature to ERC-1271', async () => {
    const { context } = makeContext({ call: queuedCall({ success: true, data: RETURN_MAGIC }) });

    await expect(
      verifyErc1271UserDecrypt(context, { userAddress: WALLET, eip712: makeEip712(), signature: SIG_1 }),
    ).resolves.toBeUndefined();
  });

  it('falls through a 65-byte owner signature (recover throws) to ERC-1271', async () => {
    const { context, ethereum } = makeContext({
      recover: vi.fn(async () => {
        throw new Error('unparsable v byte');
      }),
      call: queuedCall({ success: true, data: RETURN_MAGIC }),
    });

    await expect(
      verifyErc1271UserDecrypt(context, { userAddress: WALLET, eip712: makeEip712(), signature: SIG_65 }),
    ).resolves.toBeUndefined();
    expect(ethereum.call).toHaveBeenCalledTimes(1);
  });

  it('degrades gracefully (forwards with a warning) on an RPC transport error', async () => {
    const warn = vi.fn();
    const { context } = makeContext({
      logger: { warn } as unknown as Logger,
      call: vi.fn(async () => {
        throw new Error('rate limit exceeded');
      }),
    });

    await expect(
      verifyErc1271UserDecrypt(context, { userAddress: WALLET, eip712: makeEip712(), signature: SIG_130 }),
    ).resolves.toBeUndefined();
    expect(warn).toHaveBeenCalledTimes(1);
  });

  it('degrades gracefully (forwards with a warning) when no read provider is available', async () => {
    const warn = vi.fn();
    const ethereum = makeEthereum({ call: queuedCall({ success: true, data: RETURN_MAGIC }) });
    const runtime = createFhevmRuntime(PRIVATE_ETHERS_TOKEN, {
      ethereum,
      relayer: {} as RelayerModule,
      config: { logger: { warn } as unknown as Logger },
    });
    // A plain context (not a sealed CoreClientFhevm) makes getTrustedClient throw.
    const context = { runtime, client: {} as NonNullable<object> } as {
      runtime: typeof runtime;
      client: NonNullable<object>;
    };

    await expect(
      verifyErc1271UserDecrypt(context, {
        userAddress: WALLET as ChecksummedAddress,
        eip712: makeEip712(),
        signature: SIG_130,
      }),
    ).resolves.toBeUndefined();
    expect(warn).toHaveBeenCalledTimes(1);
    expect(ethereum.call).toHaveBeenCalledTimes(0);
  });
});
