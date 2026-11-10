import type { EthereumModule } from '../modules/ethereum/types.js';
import type { RelayerModule } from '../modules/relayer/types.js';
import type { ChecksummedAddress, Uint256BigInt } from '../types/primitives.js';
import type { FhevmCacheOptions } from '../types/cache.js';
import type { Logger } from '../types/logger.js';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import { PRIVATE_ETHERS_TOKEN } from '../../ethers/internal/ethers-p.js';
import { sepolia } from '../chains/definitions/sepolia.js';
import { createCoreFhevm } from '../runtime/CoreFhevm-p.js';
import { createFhevmRuntime } from '../runtime/CoreFhevmRuntime-p.js';
import { invalidateVersionCache } from './HostContractVersion-p.js';
import {
  kmsContextCache,
  kmsContextVersion,
  permitRequestStaleContextError,
  readCurrentKmsSignersContext,
  readKmsSignersContextFromPermitExtraData,
} from './readKmsSignersContext-p.js';
import { createKmsExtraDataV0, createKmsExtraDataV1, createKmsExtraDataV2 } from '../kms/kmsExtraData-p.js';
import { InvalidSignedPermitError } from '../errors/InvalidSignedPermitError.js';

////////////////////////////////////////////////////////////////////////////////
// npx vitest run --config src/vitest.config.ts src/core/host-contracts/readKmsSignersContext-p.test.ts
////////////////////////////////////////////////////////////////////////////////

const KMS_VERIFIER_ADDRESS = sepolia.fhevm.contracts.kmsVerifier.address as ChecksummedAddress;
const SIGNER_A = '0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266' as ChecksummedAddress;
const SIGNER_B = '0x70997970C51812dc3A010C7d01b50e0d17dc79C8' as ChecksummedAddress;

/** Reads the current KMS context again (see `VersionedValueCache.refresh`); resolves to its version. */
async function refreshCurrentKmsContext(
  client: Parameters<typeof kmsContextCache.currentParameters>[0],
  invalidVersion: string | undefined,
): Promise<string> {
  const parameters = kmsContextCache.currentParameters(client);
  return (await kmsContextCache.refresh(client, parameters, invalidVersion)).version;
}

const PROTOCOL_CONFIG_ADDRESS = sepolia.fhevm.contracts.protocolConfig?.address as ChecksummedAddress;

// Names of every on-chain function `readContract` was asked for, in call order.
const calledFunctionNames = (readContract: { readonly mock: { readonly calls: readonly unknown[][] } }): string[] =>
  readContract.mock.calls.map((call) => (call[1] as { readonly functionName: string }).functionName);

type ReadContractHandlers = Record<string, (parameters: { readonly args: readonly unknown[] }) => unknown>;

/**
 * Builds a real core client whose `EthereumModule.readContract` dispatches on
 * the contract function name — same harness as HostContractVersion-p.test.ts.
 * A missing handler throws, so every test also asserts which on-chain calls
 * are (not) made.
 */
function makeClient(handlers: ReadContractHandlers, cache?: FhevmCacheOptions, logger?: Logger) {
  const readContract = vi.fn(
    async (
      _trustedClient: unknown,
      parameters: { readonly functionName: string; readonly args: readonly unknown[] },
    ) => {
      const handler = handlers[parameters.functionName];
      if (handler === undefined) {
        throw new Error(`No mocked handler for ${parameters.functionName}`);
      }
      return handler(parameters);
    },
  );

  const ethereum = {
    readContract,
  } as unknown as EthereumModule;

  const runtime = createFhevmRuntime(PRIVATE_ETHERS_TOKEN, {
    ethereum,
    relayer: {} as RelayerModule,
    config: { logger },
  });

  const client = createCoreFhevm(PRIVATE_ETHERS_TOKEN, {
    chain: sepolia,
    client: {},
    runtime,
    options: { cache },
  });

  return { client, readContract };
}

beforeEach(() => {
  invalidateVersionCache({ includeInflight: true });
});

////////////////////////////////////////////////////////////////////////////////
// readKmsSignersContextFromPermitExtraData
//
// Resolves the signer set for the extraData a user signed into a permit: only
// the current KMS context is accepted; the result is indexed on the extraData
// as given (its `id`, plus `epochId` on v2).
////////////////////////////////////////////////////////////////////////////////

describe('readKmsSignersContextFromPermitExtraData', () => {
  // --- v1: concrete context id, keyed on itself ---
  it('honors an explicit v1 context id, keyed by the extraData’s own id', async () => {
    const { client, readContract } = makeClient({
      getCurrentKmsContextAndEpoch: () => [5n, 1n],
      getContextSignersAndThresholdFromExtraData: () => [[SIGNER_B], 1n],
    });

    const context = await readKmsSignersContextFromPermitExtraData(client, {
      kmsVerifierAddress: KMS_VERIFIER_ADDRESS,
      protocolConfigAddress: PROTOCOL_CONFIG_ADDRESS,
      extraData: createKmsExtraDataV1({ kmsContextId: 5n as Uint256BigInt }),
    });

    expect(context.id).toBe(5n);
    expect(context.epochId).toBe(0n);
    expect(context.has(SIGNER_B)).toBe(true);
    expect(context.has(SIGNER_A)).toBe(false);
    // The permit names the current context: read once, into the KMS context cache.
    expect(calledFunctionNames(readContract)).toEqual([
      'getCurrentKmsContextAndEpoch',
      'getContextSignersAndThresholdFromExtraData',
    ]);
  });

  // --- v2: contextId AND epochId both preserved ---
  it('preserves both contextId and epochId for a v2 extraData', async () => {
    const { client } = makeClient({
      getCurrentKmsContextAndEpoch: () => [9n, 3n],
      getContextSignersAndThresholdFromExtraData: () => [[SIGNER_A, SIGNER_B], 2n],
    });

    const context = await readKmsSignersContextFromPermitExtraData(client, {
      kmsVerifierAddress: KMS_VERIFIER_ADDRESS,
      protocolConfigAddress: PROTOCOL_CONFIG_ADDRESS,
      extraData: createKmsExtraDataV2({ kmsContextId: 9n as Uint256BigInt, kmsEpochId: 3n as Uint256BigInt }),
    });

    expect(context.id).toBe(9n);
    expect(context.epochId).toBe(3n);
    expect(context.threshold).toBe(2);
    expect(context.has(SIGNER_A)).toBe(true);
    expect(context.has(SIGNER_B)).toBe(true);
  });

  // --- input validation: garbage is rejected before touching the chain ---
  it('rejects a value that is not a KmsExtraData, without any on-chain read', async () => {
    const { client, readContract } = makeClient({});

    await expect(
      readKmsSignersContextFromPermitExtraData(client, {
        kmsVerifierAddress: KMS_VERIFIER_ADDRESS,
        protocolConfigAddress: undefined,
        extraData: {} as unknown as ReturnType<typeof createKmsExtraDataV0>,
      }),
    ).rejects.toThrow();
    expect(readContract).not.toHaveBeenCalled();
  });
});

////////////////////////////////////////////////////////////////////////////////
// KMS context cache
////////////////////////////////////////////////////////////////////////////////

describe('KMS context cache', () => {
  const addresses = {
    kmsVerifierAddress: KMS_VERIFIER_ADDRESS,
    protocolConfigAddress: sepolia.fhevm.contracts.protocolConfig?.address as ChecksummedAddress,
  };
  const v14 = addresses;

  /** A host whose current KMS context (and epoch) can switch; context 5 is signed by A, others by B. */
  function makeSwitchingClient(cache?: FhevmCacheOptions, logger?: Logger) {
    const current = { contextId: 5n, epochId: 1n };
    const made = makeClient(
      {
        getCurrentKmsContextAndEpoch: () => [current.contextId, current.epochId],
        getContextSignersAndThresholdFromExtraData: (parameters) =>
          BigInt(`0x${(parameters.args[0] as string).slice(4, 68)}`) === 5n ? [[SIGNER_A], 1n] : [[SIGNER_B], 1n],
      },
      cache,
      logger,
    );
    return { ...made, current };
  }

  it('reads the current context once within the TTL', async () => {
    const { client, readContract } = makeSwitchingClient();
    await readCurrentKmsSignersContext(client, v14);
    await readCurrentKmsSignersContext(client, v14);
    expect(calledFunctionNames(readContract)).toEqual([
      'getCurrentKmsContextAndEpoch',
      'getContextSignersAndThresholdFromExtraData',
    ]);
  });

  it('a refresh reads the ids: unchanged, it keeps the signers; switched, the next request uses the new context', async () => {
    const { client, readContract, current } = makeSwitchingClient();
    const before = await readCurrentKmsSignersContext(client, v14);
    readContract.mockClear();

    await expect(refreshCurrentKmsContext(client, kmsContextVersion(5n, 1n))).resolves.toBe(kmsContextVersion(5n, 1n));
    expect(calledFunctionNames(readContract)).toEqual(['getCurrentKmsContextAndEpoch']);

    current.contextId = 6n;
    await expect(refreshCurrentKmsContext(client, kmsContextVersion(5n, 1n))).resolves.toBe(kmsContextVersion(6n, 1n));
    const after = await readCurrentKmsSignersContext(client, v14);
    expect(after.id).toBe(6n);
    expect(after.has(SIGNER_B)).toBe(true);
    // The context a request still holds is unchanged.
    expect(before.has(SIGNER_A)).toBe(true);
  });

  it('a new epoch in the same context is a new version', async () => {
    const { client, current } = makeSwitchingClient();
    expect((await readCurrentKmsSignersContext(client, v14)).epochId).toBe(1n);
    current.epochId = 2n;
    await expect(refreshCurrentKmsContext(client, kmsContextVersion(5n, 1n))).resolves.toBe(kmsContextVersion(5n, 2n));
    expect((await readCurrentKmsSignersContext(client, v14)).epochId).toBe(2n);
  });

  it('a permit for the current context reuses it; one for another context throws InvalidSignedPermitError', async () => {
    const { client, readContract, current } = makeSwitchingClient();
    const permitFor = (contextId: bigint) =>
      readKmsSignersContextFromPermitExtraData(client, {
        ...v14,
        extraData: createKmsExtraDataV1({ kmsContextId: contextId as Uint256BigInt }),
      });
    await readCurrentKmsSignersContext(client, v14);
    readContract.mockClear();

    expect((await permitFor(5n)).has(SIGNER_A)).toBe(true);
    expect(readContract).not.toHaveBeenCalled();

    // A newer context than the cached one: read again once, then accepted.
    current.contextId = 6n;
    expect((await permitFor(6n)).has(SIGNER_B)).toBe(true);

    // The previous context: rejected without reading the chain (a read could
    // only return an older context, e.g. from a lagging RPC node), with the
    // ids in the message.
    readContract.mockClear();
    const error = await permitFor(5n).catch((e: unknown) => e);
    expect(readContract).not.toHaveBeenCalled();
    expect(error).toBeInstanceOf(InvalidSignedPermitError);
    expect((error as Error).message).toContain('signed for KMS context 5, but the current KMS context is 6 (epoch 1)');
  });

  it('a permit for a newer context than the cached one: debug, then read again', async () => {
    const logger = { debug: vi.fn(), warn: vi.fn(), error: vi.fn() };
    const { client, current } = makeSwitchingClient(undefined, logger);
    await readCurrentKmsSignersContext(client, v14);
    current.contextId = 6n;

    await readKmsSignersContextFromPermitExtraData(client, {
      ...v14,
      extraData: createKmsExtraDataV1({ kmsContextId: 6n as Uint256BigInt }),
    });
    expect(logger.debug).toHaveBeenCalledWith(
      'The permit names KMS context 6, newer than the cached 5 (epoch 1): reading the current context again.',
    );
    // The read that follows switched the cached context.
    expect(logger.debug).toHaveBeenCalledWith(expect.stringContaining('kmsContext: version changed'));
  });

  it('a lagging RPC node: an old permit is rejected, and the cache keeps the newer context', async () => {
    const { client, current } = makeSwitchingClient();
    current.contextId = 6n;
    await readCurrentKmsSignersContext(client, v14);
    current.contextId = 5n; // the node serving the next reads lags behind

    await expect(
      readKmsSignersContextFromPermitExtraData(client, {
        ...v14,
        extraData: createKmsExtraDataV1({ kmsContextId: 5n as Uint256BigInt }),
      }),
    ).rejects.toBeInstanceOf(InvalidSignedPermitError);
    expect((await readCurrentKmsSignersContext(client, v14)).id).toBe(6n);
  });

  it('a lagging RPC node on a read after the TTL: the cache keeps the newer context', async () => {
    const { client, current } = makeSwitchingClient({ kmsContext: { ttl: 0 } });
    current.contextId = 6n;
    expect((await readCurrentKmsSignersContext(client, v14)).id).toBe(6n);
    current.contextId = 5n;

    expect((await readCurrentKmsSignersContext(client, v14)).id).toBe(6n);
  });

  it('a context that failed is replaced by the older one read (e.g. a dev chain restarted from scratch)', async () => {
    const { client, current } = makeSwitchingClient();
    current.contextId = 6n;
    await readCurrentKmsSignersContext(client, v14);
    current.contextId = 1n;

    await expect(refreshCurrentKmsContext(client, kmsContextVersion(6n, 1n))).resolves.toBe(kmsContextVersion(1n, 1n));
    expect((await readCurrentKmsSignersContext(client, v14)).id).toBe(1n);
  });

  it('a failed request after an epoch-only switch: a v1 permit keeps its error, a v2 one gets InvalidSignedPermitError', async () => {
    const { client, current } = makeSwitchingClient();
    await readCurrentKmsSignersContext(client, v14);
    current.epochId = 2n;
    await refreshCurrentKmsContext(client, kmsContextVersion(5n, 1n));
    const requestError = new Error('timed out');

    const v1 = createKmsExtraDataV1({ kmsContextId: 5n as Uint256BigInt });
    expect(permitRequestStaleContextError(client, v1, requestError)).toBe(requestError);

    const v2 = createKmsExtraDataV2({ kmsContextId: 5n as Uint256BigInt, kmsEpochId: 1n as Uint256BigInt });
    const error = permitRequestStaleContextError(client, v2, requestError);
    expect(error).toBeInstanceOf(InvalidSignedPermitError);
    expect((error as Error).cause).toBe(requestError);
  });
});
