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
import { createZkProof } from './ZkProofBuilder-p.js';
import { fetchVerifiedInputProof } from './fetchVerifiedInputProof.js';
import { encrypt } from './encrypt.js';
import { keyMaterialCache } from '../key/ensureFheEncryptionKeyDownload.js';

////////////////////////////////////////////////////////////////////////////////
// npx vitest run --config src/vitest.config.ts src/core/coprocessor/encrypt.test.ts
//
// Which part of an encryption runs again when a cached value went stale: a
// rotated key rebuilds the ZK proof; a changed coprocessor signer set only
// sends the same proof again. The retry logic and the coprocessor signer
// cache are real (over a mocked chain); the key, the proof and the relayer
// check are stand-ins that fail the way the real ones do on a stale value.
////////////////////////////////////////////////////////////////////////////////

const SIGNER_A = '0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266' as ChecksummedAddress;
const SIGNER_B = '0x70997970C51812dc3A010C7d01b50e0d17dc79C8' as ChecksummedAddress;

/** The on-chain state: the active key and the coprocessor signer. */
const chain = vi.hoisted(() => ({ keyVersion: 'k1', coprocessorSigner: '' }));

// The key in use: the one active on-chain when it is read.
vi.mock('../key/fetchFheEncryptionKey.js', () => ({
  fetchVersionedFheEncryptionKeyWasm: vi.fn(async () => Promise.resolve({ key: {}, version: chain.keyVersion })),
}));
// A client that checks keys: a rejected proof may come from a rotated key.
vi.mock('../key/verifyFheEncryptionKeyBytes-p.js', async (importOriginal) => ({
  ...(await importOriginal<object>()),
  requiresKeyVerification: () => true,
}));
// The proof remembers the key it was built with.
vi.mock('./ZkProofBuilder-p.js', () => ({
  createZkProof: vi.fn(async () => Promise.resolve({ keyVersion: chain.keyVersion })),
}));
// The relayer rejects a proof built with another key than the active one; the
// signatures are checked against the signer set the SDK read.
vi.mock('./fetchVerifiedInputProof.js', () => ({
  fetchVerifiedInputProof: vi.fn(
    async (
      _context: unknown,
      parameters: {
        readonly zkProof: { readonly keyVersion: string };
        readonly coprocessorSigners: CoprocessorSignersContext;
      },
    ) => {
      await Promise.resolve();
      if (parameters.zkProof.keyVersion !== chain.keyVersion) {
        throw new RelayerResponseInputProofRejectedError({
          url: 'https://relayer.test',
          fetchMethod: 'POST',
          operation: 'INPUT_PROOF',
          retryCount: 0,
          elapsed: 0,
          status: 400,
        } as unknown as ConstructorParameters<typeof RelayerResponseInputProofRejectedError>[0]);
      }
      if (!parameters.coprocessorSigners.signers.includes(chain.coprocessorSigner as ChecksummedAddress)) {
        throw new ThresholdSignerError({ type: 'coprocessor' });
      }
      return { inputHandles: [], bytesHex: '0x' };
    },
  ),
}));

const fhevmContext = createFhevmClientFrozenContext({});

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
    relayer: {} as RelayerModule,
    config: {},
  });
  const client = createCoreFhevm(PRIVATE_ETHERS_TOKEN, { chain: sepolia, client: {}, runtime });
  return client as unknown as Fhevm<FhevmChain, WithEncrypt>;
}

const encryptOne = (client: Fhevm<FhevmChain, WithEncrypt>) =>
  encrypt(client, {
    contractAddress: '0xa5e1defb98EFe38EBb2D958CEe052410247F4c80' as ChecksummedAddress,
    userAddress: '0x8ba1f109551bD432803012645Ac136ddd64DBA72' as ChecksummedAddress,
    values: [createTypedValue({ type: 'uint64', value: 42n })],
    fhevmContext,
  });

// Reading the key again after a rejected proof: the active one.
vi.spyOn(keyMaterialCache, 'refresh').mockImplementation(async () =>
  Promise.resolve({ version: chain.keyVersion, value: {} as never, timestamp: 0 }),
);

beforeEach(() => {
  chain.keyVersion = 'k1';
  chain.coprocessorSigner = SIGNER_A;
  vi.mocked(createZkProof).mockClear();
  vi.mocked(fetchVerifiedInputProof).mockClear();
});

describe('encrypt on a stale cached value', () => {
  it('a changed coprocessor signer set sends the same proof again, without rebuilding it', async () => {
    const client = makeClient();
    await encryptOne(client);
    chain.coprocessorSigner = SIGNER_B;
    vi.mocked(createZkProof).mockClear();
    vi.mocked(fetchVerifiedInputProof).mockClear();

    await expect(encryptOne(client)).resolves.toBeDefined();
    expect(createZkProof).toHaveBeenCalledOnce();
    expect(fetchVerifiedInputProof).toHaveBeenCalledTimes(2);
  });

  it('a rotated key rebuilds the proof with the new key', async () => {
    const client = makeClient();
    await encryptOne(client);
    vi.mocked(createZkProof).mockClear();
    vi.mocked(fetchVerifiedInputProof).mockClear();
    // The key rotates between building the proof and sending it.
    vi.mocked(createZkProof).mockImplementationOnce(async () => {
      const proof = { keyVersion: chain.keyVersion };
      chain.keyVersion = 'k2';
      return Promise.resolve(proof as never);
    });

    await expect(encryptOne(client)).resolves.toBeDefined();
    expect(createZkProof).toHaveBeenCalledTimes(2);
    expect(fetchVerifiedInputProof).toHaveBeenCalledTimes(2);
  });

  it('a rotated key and a changed signer set at once: rebuilt once, and sent again once', async () => {
    const client = makeClient();
    await encryptOne(client);
    vi.mocked(createZkProof).mockClear();
    vi.mocked(fetchVerifiedInputProof).mockClear();
    vi.mocked(createZkProof).mockImplementationOnce(async () => {
      const proof = { keyVersion: chain.keyVersion };
      chain.keyVersion = 'k2';
      chain.coprocessorSigner = SIGNER_B;
      return Promise.resolve(proof as never);
    });

    // 1st proof (k1): rejected. 2nd proof (k2): signers stale, then accepted.
    await expect(encryptOne(client)).resolves.toBeDefined();
    expect(createZkProof).toHaveBeenCalledTimes(2);
    expect(fetchVerifiedInputProof).toHaveBeenCalledTimes(3);
  });
});
