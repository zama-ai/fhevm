import type { EthereumModule } from '../modules/ethereum/types.js';
import type { RelayerModule } from '../modules/relayer/types.js';
import type { BytesHex, ChecksummedAddress, Uint256BigInt, UintNumber } from '../types/primitives.js';
import type { HostContractVersion } from '../types/hostContract.js';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import { PRIVATE_ETHERS_TOKEN } from '../../ethers/internal/ethers-p.js';
import { sepolia } from '../chains/definitions/sepolia.js';
import { createCoreFhevm } from '../runtime/CoreFhevm-p.js';
import { createFhevmRuntime } from '../runtime/CoreFhevmRuntime-p.js';
import { invalidateVersionCache } from './HostContractVersion-p.js';
import { readKmsSignersContextFromPermitExtraData } from './readKmsSignersContext-p.js';
import { createKmsExtraDataV0, createKmsExtraDataV1, createKmsExtraDataV2 } from '../kms/kmsExtraData-p.js';
import { createFhevmClientFrozenContext } from '../frozenContext/fhevmClientFrozenContext-p.js';

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

const kmsVerifierVersion = (major: number, minor: number, patch: number): HostContractVersion<'KMSVerifier'> => ({
  version: `KMSVerifier v${major}.${minor}.${patch}`,
  contractName: 'KMSVerifier',
  major: major as UintNumber,
  minor: minor as UintNumber,
  patch: patch as UintNumber,
});

// The version basis each test resolves against. The KMSVerifier version is read
// from the frozen context (NOT the on-chain `getVersion`), which is why none of the
// tests below need a `getVersion` handler — reaching one would throw "No mocked
// handler". Only the actual context reads (getContextSignersAndThresholdFromExtraData,
// or getThreshold+getKmsSigners on the v11 path) are mocked per-test.
//
//   v11 → KMSVerifier < 0.2.0 (no context concept, extraData v0 only)
//   v13 → KMSVerifier 0.2.0–0.3.x (extraData up to v1)
//   v14 → KMSVerifier >= 0.4.0 (extraData up to v2, contextId + epochId)
const fhevmContextV11 = createFhevmClientFrozenContext({
  hostContractVersions: { KMSVerifier: kmsVerifierVersion(0, 1, 0) },
});
const fhevmContextV13 = createFhevmClientFrozenContext({
  hostContractVersions: { KMSVerifier: kmsVerifierVersion(0, 3, 0) },
});
const fhevmContextV14 = createFhevmClientFrozenContext({
  hostContractVersions: { KMSVerifier: kmsVerifierVersion(0, 4, 0) },
});

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

beforeEach(() => {
  invalidateVersionCache({ includeInflight: true });
});

////////////////////////////////////////////////////////////////////////////////
// readKmsSignersContextFromPermitExtraData
//
// Resolves the signer set for the EXACT extraData a user signed into a permit,
// reading it against the on-chain KMSVerifier *as given* — never re-encoding,
// upgrading, or substituting it. The read is deliberately faithful to the permit
// so the returned signer set is the one that permit committed to. The result is
// self-describing: its `id` (+ `epochId` on v2) is the extraData it was indexed by.
////////////////////////////////////////////////////////////////////////////////

describe('readKmsSignersContextFromPermitExtraData', () => {
  // --- v0 sentinel: passed through verbatim, NOT resolved to a concrete context ---
  it('passes a v0 sentinel through verbatim: id=0, signers from the on-chain v0 lookup', async () => {
    const { client, readContract } = makeClient({
      // The current/default signer set the contract returns for the `0x00` sentinel.
      getContextSignersAndThresholdFromExtraData: (parameters) => {
        expect(parameters.args[0]).toBe(v0Bytes); // looked up by the sentinel bytes themselves
        return [[SIGNER_A], 1n];
      },
    });

    const context = await readKmsSignersContextFromPermitExtraData(client, {
      kmsVerifierAddress: KMS_VERIFIER_ADDRESS,
      protocolConfigAddress: undefined,
      extraData: createKmsExtraDataV0(),
      fhevmContext: fhevmContextV13,
    });

    // Indexed on the extraData AS GIVEN (v0 → id 0), NOT re-derived to a concrete id.
    expect(context.id).toBe(0n);
    expect(context.epochId).toBe(0n);
    expect(context.threshold).toBe(1);
    expect(context.has(SIGNER_A)).toBe(true);
    // The current-context is never consulted for a permit read.
    expect(calledFunctionNames(readContract)).not.toContain('getCurrentKmsContextId');
  });

  // --- v1: concrete context id, keyed on itself ---
  it('honors an explicit v1 context id, keyed by the extraData’s own id', async () => {
    const { client, readContract } = makeClient({
      getContextSignersAndThresholdFromExtraData: (parameters) => {
        expect(parameters.args[0]).toBe(v1Bytes(5n));
        return [[SIGNER_B], 1n];
      },
    });

    const context = await readKmsSignersContextFromPermitExtraData(client, {
      kmsVerifierAddress: KMS_VERIFIER_ADDRESS,
      protocolConfigAddress: undefined,
      extraData: createKmsExtraDataV1({ kmsContextId: 5n as Uint256BigInt }),
      fhevmContext: fhevmContextV13,
    });

    expect(context.id).toBe(5n);
    expect(context.epochId).toBe(0n);
    expect(context.has(SIGNER_B)).toBe(true);
    expect(context.has(SIGNER_A)).toBe(false);
    // Exactly one on-chain call, and never the current-context read.
    expect(calledFunctionNames(readContract)).toEqual(['getContextSignersAndThresholdFromExtraData']);
  });

  // --- v2: contextId AND epochId both preserved ---
  it('preserves both contextId and epochId for a v2 extraData on a v14 chain', async () => {
    const { client } = makeClient({
      getContextSignersAndThresholdFromExtraData: (parameters) => {
        expect(parameters.args[0]).toBe(v2Bytes(9n, 3n));
        return [[SIGNER_A, SIGNER_B], 2n];
      },
    });

    const context = await readKmsSignersContextFromPermitExtraData(client, {
      kmsVerifierAddress: KMS_VERIFIER_ADDRESS,
      protocolConfigAddress: undefined,
      extraData: createKmsExtraDataV2({ kmsContextId: 9n as Uint256BigInt, kmsEpochId: 3n as Uint256BigInt }),
      fhevmContext: fhevmContextV14,
    });

    expect(context.id).toBe(9n);
    expect(context.epochId).toBe(3n);
    expect(context.threshold).toBe(2);
    expect(context.has(SIGNER_A)).toBe(true);
    expect(context.has(SIGNER_B)).toBe(true);
  });

  // --- backward compat: a v13-capped SDK reads a v14 chain through the same reader ---
  it('reads a v1 permit on a v14 chain via the shared reader (backward compat)', async () => {
    const { client, readContract } = makeClient({
      getContextSignersAndThresholdFromExtraData: (parameters) => {
        expect(parameters.args[0]).toBe(v1Bytes(5n));
        return [[SIGNER_A], 1n];
      },
    });

    const context = await readKmsSignersContextFromPermitExtraData(client, {
      kmsVerifierAddress: KMS_VERIFIER_ADDRESS,
      protocolConfigAddress: undefined,
      extraData: createKmsExtraDataV1({ kmsContextId: 5n as Uint256BigInt }),
      fhevmContext: fhevmContextV14,
    });

    expect(context.id).toBe(5n);
    expect(context.epochId).toBe(0n);
    expect(calledFunctionNames(readContract)).toEqual(['getContextSignersAndThresholdFromExtraData']);
  });

  // --- v11 chain: no context concept, the single global signer set ---
  it('uses the v11 reader (getThreshold + getKmsSigners) on a KMSVerifier < 0.2.0', async () => {
    const { client, readContract } = makeClient({
      getThreshold: () => 1n,
      getKmsSigners: () => [SIGNER_A],
    });

    const context = await readKmsSignersContextFromPermitExtraData(client, {
      kmsVerifierAddress: KMS_VERIFIER_ADDRESS,
      protocolConfigAddress: undefined,
      extraData: createKmsExtraDataV0(),
      fhevmContext: fhevmContextV11,
    });

    expect(context.id).toBe(0n);
    expect(context.epochId).toBe(0n);
    expect(context.threshold).toBe(1);
    expect(context.has(SIGNER_A)).toBe(true);
    // The context-aware reader must NOT be used on a context-less KMSVerifier.
    const called = calledFunctionNames(readContract);
    expect(called).toContain('getThreshold');
    expect(called).toContain('getKmsSigners');
    expect(called).not.toContain('getContextSignersAndThresholdFromExtraData');
  });

  // --- compat rejections: incompatible extraData never reaches the chain ---
  it('rejects extraData newer than the KMSVerifier supports (v2 on a v13 chain)', async () => {
    const { client, readContract } = makeClient({});

    await expect(
      readKmsSignersContextFromPermitExtraData(client, {
        kmsVerifierAddress: KMS_VERIFIER_ADDRESS,
        protocolConfigAddress: undefined,
        extraData: createKmsExtraDataV2({ kmsContextId: 5n as Uint256BigInt, kmsEpochId: 2n as Uint256BigInt }),
        fhevmContext: fhevmContextV13,
      }),
    ).rejects.toThrow('not compatible');
    // Rejected before any on-chain read.
    expect(readContract).not.toHaveBeenCalled();
  });

  it('rejects a concrete v1 extraData on a context-less KMSVerifier < 0.2.0', async () => {
    const { client, readContract } = makeClient({});

    await expect(
      readKmsSignersContextFromPermitExtraData(client, {
        kmsVerifierAddress: KMS_VERIFIER_ADDRESS,
        protocolConfigAddress: undefined,
        extraData: createKmsExtraDataV1({ kmsContextId: 5n as Uint256BigInt }),
        fhevmContext: fhevmContextV11,
      }),
    ).rejects.toThrow('not compatible');
    expect(readContract).not.toHaveBeenCalled();
  });

  // --- input validation: garbage is rejected before touching the chain ---
  it('rejects a value that is not a KmsExtraData, without any on-chain read', async () => {
    const { client, readContract } = makeClient({});

    await expect(
      readKmsSignersContextFromPermitExtraData(client, {
        kmsVerifierAddress: KMS_VERIFIER_ADDRESS,
        protocolConfigAddress: undefined,
        extraData: {} as unknown as ReturnType<typeof createKmsExtraDataV0>,
        fhevmContext: fhevmContextV13,
      }),
    ).rejects.toThrow();
    expect(readContract).not.toHaveBeenCalled();
  });
});
