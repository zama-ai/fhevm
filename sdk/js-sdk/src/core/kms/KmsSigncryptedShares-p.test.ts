import type { EthereumModule } from '../modules/ethereum/types.js';
import type { RelayerModule } from '../modules/relayer/types.js';
import type { TkmsVersion } from '../../wasm/tkms/KmsLibApi.js';
import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import type { KmsSigncryptedShare, KmsSigncryptedSharesMetadata } from '../types/kms-p.js';
import type {
  Bytes65Hex,
  Bytes65HexNo0x,
  BytesHexNo0x,
  ChecksummedAddress,
  Uint256BigInt,
  Uint8Number,
} from '../types/primitives.js';
import { describe, expect, it, vi } from 'vitest';
import { PRIVATE_ETHERS_TOKEN } from '../../ethers/internal/ethers-p.js';
import { sepolia } from '../chains/definitions/sepolia.js';
import { createCoreFhevm } from '../runtime/CoreFhevm-p.js';
import { createFhevmRuntime } from '../runtime/CoreFhevmRuntime-p.js';
import { createKmsSignersContext } from '../host-contracts/KmsSignersContext-p.js';
import { createKmsEip712Domain } from './createKmsEip712Domain.js';
import { createKmsSigncryptedShares } from './KmsSigncryptedShares-p.js';

////////////////////////////////////////////////////////////////////////////////
// npx vitest run --config src/vitest.config.ts src/core/kms/KmsSigncryptedShares-p.test.ts
////////////////////////////////////////////////////////////////////////////////

const KMS_VERIFIER_ADDRESS = sepolia.fhevm.contracts.kmsVerifier.address as ChecksummedAddress;
const SIGNER_A = '0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266' as ChecksummedAddress;

const word = (value: bigint): string => value.toString(16).padStart(64, '0');

function makeClient() {
  const readContract = vi.fn(async (_trustedClient: unknown, parameters: { readonly functionName: string }) => {
    throw new Error(`No mocked handler for ${parameters.functionName}`);
  });

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

function makeMetadata(kmsContextId: bigint): KmsSigncryptedSharesMetadata {
  return {
    kmsSignersContext: createKmsSignersContext(new WeakRef({} as FhevmRuntime), {
      kmsVerifierAddress: KMS_VERIFIER_ADDRESS,
      kmsContextId: kmsContextId as Uint256BigInt,
      kmsEpochId: 0n as Uint256BigInt,
      kmsSigners: [SIGNER_A],
      kmsSignerThreshold: 1 as Uint8Number,
    }),
    eip712Domain: createKmsEip712Domain({
      chainId: 1n,
      verifyingContractAddressDecryption: KMS_VERIFIER_ADDRESS,
    }),
    eip712Signature: `0x${'11'.repeat(65)}` as Bytes65Hex,
    eip712SignerAddress: SIGNER_A,
    handles: [],
    tkmsVersion: '0.13.10' as TkmsVersion,
  };
}

function makeShare(extraData: string): KmsSigncryptedShare {
  return {
    payload: 'aabb' as BytesHexNo0x,
    signature: '22'.repeat(65) as Bytes65HexNo0x,
    extraData: extraData as BytesHexNo0x,
  };
}

describe('createKmsSigncryptedShares', () => {
  it('accepts shares with uniform extraData naming the permit context, without on-chain access', async () => {
    const { client, readContract } = makeClient();

    const shares = await createKmsSigncryptedShares(client, {
      metadata: makeMetadata(7n),
      shares: [makeShare(`01${word(7n)}`), makeShare(`01${word(7n)}`), makeShare(`01${word(7n)}`)],
    });

    expect(shares).toBeDefined();
    expect(readContract).not.toHaveBeenCalled();
  });

  // Mixed-version KMS response window. The production rollout upgrades KMS
  // nodes in stages (a subset on the new version below threshold, then above
  // threshold, then all — see the protocol migration runbook), so a response
  // batch may be assembled while old- and new-version nodes coexist. The SDK
  // requires every share in a batch to carry byte-identical extraData: a batch
  // mixing encodings is rejected wholesale, even if enough individually valid
  // shares exist. This pins that behavior — old and new KMS nodes MUST echo
  // the request's extraData verbatim during mixed-version phases, or user
  // decryption breaks exactly during the upgrade window.
  it('rejects a batch mixing extraData encodings across shares (mixed-version response)', async () => {
    const { client } = makeClient();

    await expect(
      createKmsSigncryptedShares(client, {
        metadata: makeMetadata(7n),
        shares: [makeShare(`01${word(7n)}`), makeShare('00'), makeShare(`01${word(7n)}`)],
      }),
    ).rejects.toThrow('Mismatched extraData across shares');
  });

  it('rejects an empty share batch', async () => {
    const { client } = makeClient();

    await expect(
      createKmsSigncryptedShares(client, {
        metadata: makeMetadata(7n),
        shares: [],
      }),
    ).rejects.toThrow('Expected at least one signcrypted share');
  });
});
