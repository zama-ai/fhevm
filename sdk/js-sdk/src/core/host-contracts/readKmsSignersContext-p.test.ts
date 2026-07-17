import type { EthereumModule } from '../modules/ethereum/types.js';
import type { RelayerModule } from '../modules/relayer/types.js';
import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import type { KmsSignersContext } from '../types/kmsSignersContext.js';
import type { BytesHex, ChecksummedAddress, Uint256BigInt, Uint8Number } from '../types/primitives.js';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import { PRIVATE_ETHERS_TOKEN } from '../../ethers/internal/ethers-p.js';
import { sepolia } from '../chains/definitions/sepolia.js';
import { createCoreFhevm } from '../runtime/CoreFhevm-p.js';
import { createFhevmRuntime } from '../runtime/CoreFhevmRuntime-p.js';
import { invalidateVersionCache } from './HostContractVersion-p.js';
import { createKmsSignersContext } from './KmsSignersContext-p.js';
import { readKmsSignersContextFromExtraData, reconcileKmsSignersContext } from './readKmsSignersContext-p.js';
import { createKmsExtraDataV0, createKmsExtraDataV1, createKmsExtraDataV2 } from '../kms/kmsExtraData-p.js';

////////////////////////////////////////////////////////////////////////////////
// npx vitest run --config src/vitest.config.ts src/core/host-contracts/readKmsSignersContext-p.test.ts
////////////////////////////////////////////////////////////////////////////////

const KMS_VERIFIER_ADDRESS = sepolia.fhevm.contracts.kmsVerifier.address as ChecksummedAddress;
const SIGNER_A = '0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266' as ChecksummedAddress;
const SIGNER_B = '0x70997970C51812dc3A010C7d01b50e0d17dc79C8' as ChecksummedAddress;

const word = (value: bigint): string => value.toString(16).padStart(64, '0');
const v0Bytes = '0x00' as BytesHex;
const v1Bytes = (contextId: bigint): BytesHex => `0x01${word(contextId)}` as BytesHex;
const v2Bytes = (contextId: bigint, epochId: bigint): BytesHex => `0x02${word(contextId)}${word(epochId)}` as BytesHex;

type ReadContractHandlers = Record<string, (parameters: { readonly args: readonly unknown[] }) => unknown>;

/**
 * Builds a real core client whose `EthereumModule.readContract` dispatches on
 * the contract function name — same harness as HostContractVersion-p.test.ts.
 * A missing handler throws, so every test also asserts which on-chain calls
 * are (not) made.
 */
function makeClient(handlers: ReadContractHandlers) {
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
    config: {},
  });

  const client = createCoreFhevm(PRIVATE_ETHERS_TOKEN, {
    chain: sepolia,
    client: {},
    runtime,
  });

  return { client, readContract };
}

function makeRequestedContext(kmsContextId: bigint, kmsEpochId: bigint): KmsSignersContext {
  return createKmsSignersContext(new WeakRef({} as FhevmRuntime), {
    kmsVerifierAddress: KMS_VERIFIER_ADDRESS,
    kmsContextId: kmsContextId as Uint256BigInt,
    kmsEpochId: kmsEpochId as Uint256BigInt,
    kmsSigners: [SIGNER_A],
    kmsSignerThreshold: 1 as Uint8Number,
  });
}

beforeEach(() => {
  invalidateVersionCache({ includeInflight: true });
});

////////////////////////////////////////////////////////////////////////////////
// reconcileKmsSignersContext
//
// The security core of KMS response verification: the shares returned by the
// (untrusted) relayer must come from the context the user committed to in the
// signed permit — compared on contextId, with on-chain state as the source of
// truth. Epoch and encoding version are deliberately NOT compared (the gateway
// ignores them too, and they legitimately drift across protocol migrations).
////////////////////////////////////////////////////////////////////////////////

describe('reconcileKmsSignersContext', () => {
  it('accepts without any on-chain access when the relayer extraData names the permit context id', async () => {
    const { client, readContract } = makeClient({});
    const requested = makeRequestedContext(7n, 0n);

    await expect(
      reconcileKmsSignersContext(client, {
        kmsVerifierAddress: KMS_VERIFIER_ADDRESS,
        protocolConfigAddress: undefined,
        requestedKmsSignersContext: requested,
        relayerKmsExtraDataBytesHex: v1Bytes(7n),
      }),
    ).resolves.toBe(requested);
    expect(readContract).not.toHaveBeenCalled();
  });

  it('accepts a newer-encoded response (v2) for the same context id — epoch is not compared', async () => {
    // A v13-era request (extraData v1, epoch 0) answered by a v14-era KMS
    // (extraData v2 with a live epoch): same context id, different encoding
    // and epoch. Must be accepted on the fast path, without chain access.
    const { client, readContract } = makeClient({});
    const requested = makeRequestedContext(7n, 0n);

    await expect(
      reconcileKmsSignersContext(client, {
        kmsVerifierAddress: KMS_VERIFIER_ADDRESS,
        protocolConfigAddress: undefined,
        requestedKmsSignersContext: requested,
        relayerKmsExtraDataBytesHex: v2Bytes(7n, 3n),
      }),
    ).resolves.toBe(requested);
    expect(readContract).not.toHaveBeenCalled();
  });

  it('resolves a v0 relayer response to the current context and accepts when it matches the permit', async () => {
    // A lagging KMS echoes extraData v0 ("current context" sentinel) while the
    // permit is anchored on a concrete context id. The v0 response resolves
    // on-chain to the current context — accepted when it is the permit's.
    const { client } = makeClient({
      getVersion: () => 'KMSVerifier v0.3.0',
      getCurrentKmsContextId: () => 7n,
      getContextSignersAndThresholdFromExtraData: () => [[SIGNER_A], 1n],
    });
    const requested = makeRequestedContext(7n, 0n);

    await expect(
      reconcileKmsSignersContext(client, {
        kmsVerifierAddress: KMS_VERIFIER_ADDRESS,
        protocolConfigAddress: undefined,
        requestedKmsSignersContext: requested,
        relayerKmsExtraDataBytesHex: v0Bytes,
      }),
    ).resolves.toBe(requested);
  });

  it('rejects a v0 relayer response when the current context differs from the permit context', async () => {
    // Same lagging-KMS shape, but the chain rotated to context 9 while the
    // permit commits to context 7 — substitution, must be rejected.
    const { client } = makeClient({
      getVersion: () => 'KMSVerifier v0.3.0',
      getCurrentKmsContextId: () => 9n,
      getContextSignersAndThresholdFromExtraData: () => [[SIGNER_B], 1n],
    });
    const requested = makeRequestedContext(7n, 0n);

    await expect(
      reconcileKmsSignersContext(client, {
        kmsVerifierAddress: KMS_VERIFIER_ADDRESS,
        protocolConfigAddress: undefined,
        requestedKmsSignersContext: requested,
        relayerKmsExtraDataBytesHex: v0Bytes,
      }),
    ).rejects.toThrow('KMS context mismatch');
  });

  it('rejects a relayer response naming a different, on-chain-valid context (anti-substitution)', async () => {
    // Context 9 exists and is valid on-chain — but the permit commits to
    // context 7. A valid-but-different context is still a substitution.
    const { client } = makeClient({
      getVersion: () => 'KMSVerifier v0.3.0',
      getContextSignersAndThresholdFromExtraData: () => [[SIGNER_B], 1n],
    });
    const requested = makeRequestedContext(7n, 0n);

    await expect(
      reconcileKmsSignersContext(client, {
        kmsVerifierAddress: KMS_VERIFIER_ADDRESS,
        protocolConfigAddress: undefined,
        requestedKmsSignersContext: requested,
        relayerKmsExtraDataBytesHex: v1Bytes(9n),
      }),
    ).rejects.toThrow('KMS context mismatch');
  });

  it('propagates the on-chain revert for a relayer context that does not exist', async () => {
    const { client } = makeClient({
      getVersion: () => 'KMSVerifier v0.3.0',
      getContextSignersAndThresholdFromExtraData: () => {
        throw new Error('execution reverted: InvalidKmsContext');
      },
    });
    const requested = makeRequestedContext(7n, 0n);

    await expect(
      reconcileKmsSignersContext(client, {
        kmsVerifierAddress: KMS_VERIFIER_ADDRESS,
        protocolConfigAddress: undefined,
        requestedKmsSignersContext: requested,
        relayerKmsExtraDataBytesHex: v1Bytes(0xdeadbeefn),
      }),
    ).rejects.toThrow('InvalidKmsContext');
  });
});

////////////////////////////////////////////////////////////////////////////////
// readKmsSignersContextFromExtraData
//
// The migration-critical resolver: extraData v0 (a stale v11-era permit) is
// the "current context" sentinel — mirroring KMSVerifier._extractKmsContextId
// and the gateway Decryption._extractContextId — while concrete versions are
// looked up on-chain and version-checked against the KMSVerifier.
////////////////////////////////////////////////////////////////////////////////

describe('readKmsSignersContextFromExtraData', () => {
  it('resolves extraData v0 to the CURRENT context (sentinel), not context id 0', async () => {
    const { client } = makeClient({
      getVersion: () => 'KMSVerifier v0.3.0',
      getCurrentKmsContextId: () => 7n,
      getContextSignersAndThresholdFromExtraData: () => [[SIGNER_A], 1n],
    });

    const context = await readKmsSignersContextFromExtraData(client, {
      kmsVerifierAddress: KMS_VERIFIER_ADDRESS,
      protocolConfigAddress: undefined,
      extraData: createKmsExtraDataV0(),
    });

    expect(context.id).toBe(7n);
    expect(context.threshold).toBe(1);
    expect(context.has(SIGNER_A)).toBe(true);
  });

  it('honors an explicit v1 context id without reading the current context', async () => {
    const { client, readContract } = makeClient({
      getVersion: () => 'KMSVerifier v0.3.0',
      getContextSignersAndThresholdFromExtraData: (parameters) => {
        // The lookup must be keyed by the extraData's OWN context id.
        expect(parameters.args[0]).toBe(v1Bytes(5n));
        return [[SIGNER_B], 1n];
      },
    });

    const context = await readKmsSignersContextFromExtraData(client, {
      kmsVerifierAddress: KMS_VERIFIER_ADDRESS,
      protocolConfigAddress: undefined,
      extraData: createKmsExtraDataV1({ kmsContextId: 5n as Uint256BigInt }),
    });

    expect(context.id).toBe(5n);
    expect(context.has(SIGNER_B)).toBe(true);
    // getCurrentKmsContextId has no handler — reaching it would have thrown.
    expect(readContract).toHaveBeenCalled();
  });

  it('rejects extraData newer than the KMSVerifier supports', async () => {
    const { client } = makeClient({
      getVersion: () => 'KMSVerifier v0.3.0',
    });

    await expect(
      readKmsSignersContextFromExtraData(client, {
        kmsVerifierAddress: KMS_VERIFIER_ADDRESS,
        protocolConfigAddress: undefined,
        extraData: createKmsExtraDataV2({ kmsContextId: 5n as Uint256BigInt, kmsEpochId: 2n as Uint256BigInt }),
      }),
    ).rejects.toThrow('not compatible');
  });
});
