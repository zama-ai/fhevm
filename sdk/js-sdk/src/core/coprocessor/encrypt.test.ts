import type { EthereumModule } from '../modules/ethereum/types.js';
import type { RelayerModule } from '../modules/relayer/types.js';
import type { Fhevm } from '../types/coreFhevmClient.js';
import type { WithEncrypt } from '../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../types/fhevmChain.js';
import type { ChecksummedAddress } from '../types/primitives.js';
import type { CoprocessorSignersContext } from '../types/coprocessorSignersContext.js';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import { PRIVATE_ETHERS_TOKEN } from '../../ethers/internal/ethers-p.js';
import { sepolia } from '../chains/definitions/sepolia.js';
import { createCoreFhevm } from '../runtime/CoreFhevm-p.js';
import { createFhevmRuntime } from '../runtime/CoreFhevmRuntime-p.js';
import { createFhevmClientFrozenContext } from '../frozenContext/fhevmClientFrozenContext-p.js';
import { createTypedValue } from '../base/typedValue.js';
import { ThresholdSignerError } from '../errors/SignersError.js';
import { RelayerResponseInputProofRejectedError } from '../errors/RelayerResponseInputProofRejectedError.js';
import { keyMaterialCache } from '../key/ensureFheEncryptionKeyDownload.js';
import { coprocessorSignersCache } from '../host-contracts/getCoprocessorContextSignersAndThreshold-p.js';
import { createZkProof } from './ZkProofBuilder-p.js';
import { verifyInputProof } from './verifyInputProof.js';
import { encrypt } from './encrypt.js';

////////////////////////////////////////////////////////////////////////////////
// npx vitest run --config src/vitest.config.ts src/core/coprocessor/encrypt.test.ts
//
// What runs again when a cached value went stale: a rotated key rebuilds the
// ZK proof and sends it again; a changed coprocessor signer set only checks
// the relayer's response again. The retry logic and the coprocessor signer
// cache are real (over a mocked chain); the key, the proof, the relayer and
// the signature check are stand-ins that fail the way the real ones do.
////////////////////////////////////////////////////////////////////////////////

const SIGNER_A = '0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266' as ChecksummedAddress;
const SIGNER_B = '0x70997970C51812dc3A010C7d01b50e0d17dc79C8' as ChecksummedAddress;

const { chain, HANDLE, makeProof } = vi.hoisted(() => {
  /** The on-chain state: the active key and the coprocessor signer. */
  const chain = { keyVersion: 'k1', coprocessorSigner: '' };
  const HANDLE = { bytes32Hex: `0x${'11'.repeat(32)}` };
  /** A ZK proof stand-in: it remembers the key it was built with. */
  const makeProof = () => ({
    keyVersion: chain.keyVersion,
    userAddress: '0x8ba1f109551bD432803012645Ac136ddd64DBA72',
    contractAddress: '0xa5e1defb98EFe38EBb2D958CEe052410247F4c80',
    getInputHandles: () => [HANDLE],
    getExtraData: () => '0x00',
  });
  return { chain, HANDLE, makeProof };
});

// The key in use: the one active on-chain when it is read.
vi.mock('../key/fetchFheEncryptionKey.js', () => ({
  fetchVersionedFheEncryptionKeyWasm: vi.fn(async () => Promise.resolve({ value: {}, version: chain.keyVersion })),
}));
vi.mock('./ZkProofBuilder-p.js', () => ({
  createZkProof: vi.fn(async () => Promise.resolve(makeProof())),
}));
vi.mock('./ZkProof-p.js', async (importOriginal) => ({
  ...(await importOriginal<object>()),
  assertIsZkProof: () => undefined,
}));
vi.mock('../handle/FhevmHandle.js', async (importOriginal) => ({
  ...(await importOriginal<object>()),
  assertHandleArrayEquals: () => undefined,
}));
vi.mock('./InputProof-p.js', async (importOriginal) => ({
  ...(await importOriginal<object>()),
  createInputProofFromInputHandles: (p: { readonly inputHandles: unknown }) => ({
    inputHandles: p.inputHandles,
    bytesHex: '0x',
  }),
}));
// The signatures are checked against the signer set the SDK read.
vi.mock('./verifyInputProof.js', () => ({
  verifyInputProof: vi.fn(
    async (
      _context: unknown,
      parameters: { readonly inputProof: unknown; coprocessorSigners: CoprocessorSignersContext },
    ) => {
      await Promise.resolve();
      if (!parameters.coprocessorSigners.signers.includes(chain.coprocessorSigner as ChecksummedAddress)) {
        throw new ThresholdSignerError({ type: 'coprocessor' });
      }
      return parameters.inputProof;
    },
  ),
}));

const fhevmContext = createFhevmClientFrozenContext({});

// The relayer rejects a proof built with another key than the active one.
const fetchCoprocessorSignatures = vi.fn(async (_client: unknown, p: { readonly payload: { zkProof: unknown } }) => {
  await Promise.resolve();
  if ((p.payload.zkProof as { keyVersion: string }).keyVersion !== chain.keyVersion) {
    throw new RelayerResponseInputProofRejectedError({
      url: 'https://relayer.test',
      fetchMethod: 'POST',
      operation: 'INPUT_PROOF',
      retryCount: 0,
      elapsed: 0,
      status: 400,
    } as unknown as ConstructorParameters<typeof RelayerResponseInputProofRejectedError>[0]);
  }
  return { handles: [HANDLE], coprocessorEip712Signatures: [] };
});

function makeClient() {
  const readContract = vi.fn(async (_client: unknown, p: { readonly functionName: string }) => {
    await Promise.resolve();
    switch (p.functionName) {
      case 'getThreshold':
        return 1n;
      case 'getCoprocessorSigners':
        return [chain.coprocessorSigner];
      default:
        throw new Error(`No mocked handler for ${p.functionName}`);
    }
  });
  const runtime = createFhevmRuntime(PRIVATE_ETHERS_TOKEN, {
    ethereum: { readContract } as unknown as EthereumModule,
    relayer: { fetchCoprocessorSignatures } as unknown as RelayerModule,
    config: {},
  });
  const client = createCoreFhevm(PRIVATE_ETHERS_TOKEN, {
    chain: sepolia,
    client: {},
    runtime,
    options: { cache: { refreshRetryDelay: 0 } },
  });
  return client as unknown as Fhevm<FhevmChain, WithEncrypt>;
}

const encryptOne = (client: Fhevm<FhevmChain, WithEncrypt>) =>
  encrypt(client, {
    contractAddress: '0xa5e1defb98EFe38EBb2D958CEe052410247F4c80' as ChecksummedAddress,
    userAddress: '0x8ba1f109551bD432803012645Ac136ddd64DBA72' as ChecksummedAddress,
    values: [createTypedValue({ type: 'uint64', value: 42n })],
    fhevmContext,
  });

function clearCalls() {
  vi.mocked(createZkProof).mockClear();
  vi.mocked(verifyInputProof).mockClear();
  fetchCoprocessorSignatures.mockClear();
}

beforeEach(() => {
  chain.keyVersion = 'k1';
  chain.coprocessorSigner = SIGNER_A;
  coprocessorSignersCache.clear();
  // A client that checks keys: a rejected proof may come from a rotated key,
  // read again as the active one.
  vi.spyOn(keyMaterialCache, 'mayBeStale').mockImplementation(
    (_context, error) => error instanceof RelayerResponseInputProofRejectedError,
  );
  vi.spyOn(keyMaterialCache, 'refreshCurrent').mockImplementation(async () =>
    Promise.resolve({ version: chain.keyVersion, value: {} as never, timestamp: 0 }),
  );
  clearCalls();
});

describe('encrypt on a stale cached value', () => {
  it('a changed coprocessor signer set checks the same response again: no new proof, no new relayer call', async () => {
    const client = makeClient();
    await encryptOne(client);
    chain.coprocessorSigner = SIGNER_B;
    clearCalls();

    await expect(encryptOne(client)).resolves.toBeDefined();
    expect(createZkProof).toHaveBeenCalledOnce();
    expect(fetchCoprocessorSignatures).toHaveBeenCalledOnce();
    expect(verifyInputProof).toHaveBeenCalledTimes(2);
  });

  it('a rotated key rebuilds the proof with the new key and sends it again', async () => {
    const client = makeClient();
    await encryptOne(client);
    clearCalls();
    // The key rotates between building the proof and sending it.
    vi.mocked(createZkProof).mockImplementationOnce(async () => {
      const proof = makeProof();
      chain.keyVersion = 'k2';
      return Promise.resolve(proof as never);
    });

    await expect(encryptOne(client)).resolves.toBeDefined();
    expect(createZkProof).toHaveBeenCalledTimes(2);
    expect(fetchCoprocessorSignatures).toHaveBeenCalledTimes(2);
    expect(verifyInputProof).toHaveBeenCalledOnce();
  });

  it('a rotated key and a changed signer set at once: each retried once', async () => {
    const client = makeClient();
    await encryptOne(client);
    clearCalls();
    vi.mocked(createZkProof).mockImplementationOnce(async () => {
      const proof = makeProof();
      chain.keyVersion = 'k2';
      chain.coprocessorSigner = SIGNER_B;
      return Promise.resolve(proof as never);
    });

    // 1st proof (k1): rejected. 2nd proof (k2): signers stale, then checked again.
    await expect(encryptOne(client)).resolves.toBeDefined();
    expect(createZkProof).toHaveBeenCalledTimes(2);
    expect(fetchCoprocessorSignatures).toHaveBeenCalledTimes(2);
    expect(verifyInputProof).toHaveBeenCalledTimes(2);
  });
});
