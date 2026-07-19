import type { Address, BytesHex, ChecksummedAddress } from '../../../types/primitives.js';
import type { Handle } from '../../../types/encryptedTypes-p.js';
import type { CleartextEthereumModule } from '../../ethereum/types-ct.js';
import type { TrustedClient } from '../../ethereum/types.js';
import type { RelayerClientWithRuntime } from '../types.js';
import type { KmsSignersContext } from '../../../types/kmsSignersContext.js';
import { remove0x } from '../../../base/string.js';
import { addressToChecksummedAddress } from '../../../base/address.js';
import { createCachedFetch } from '../../../base/cachedFetch.js';
import { getFHEVMExecutorAddressAbi } from '../../../host-contracts/abi-fragments/fragments.js';
import { readCurrentKmsSignersContext } from '../../../host-contracts/readKmsSignersContext-p.js';
import { isForgeFhevmV1KmsSigner } from './signers.js';

////////////////////////////////////////////////////////////////////////////////
//
// Option B — cleartext "disabled KMSVerifier" plaintext source.
//
// The cleartext mock relayer used to read masked cleartexts + the KMS payload
// from the `Cleartext*KMSVerifier` view. That view required the full `Cleartext*`
// host set, which partners deploying only forge-fhevm do not have. Instead we
// read the raw plaintext straight from `CleartextFHEVMExecutor.plaintexts` (the
// mapping forge-fhevm populates) and rebuild the KMS wire format off-chain,
// reusing the already-local KMS signing. Only the *plaintext source* moves;
// everything downstream (masking, signing, reconstruct) is unchanged.
//
////////////////////////////////////////////////////////////////////////////////

// CleartextFHEVMExecutor.plaintexts(bytes32) — the handle→cleartext mirror.
const plaintextsAbi = [
  {
    type: 'function',
    name: 'plaintexts',
    inputs: [{ name: 'handle', type: 'bytes32', internalType: 'bytes32' }],
    outputs: [{ name: '', type: 'uint256', internalType: 'uint256' }],
    stateMutability: 'view',
  },
] as const;

////////////////////////////////////////////////////////////////////////////////

export async function isForgeFhevmV1(relayerClient: RelayerClientWithRuntime): Promise<boolean> {
  const currentKmsSignersContext: KmsSignersContext = await readCurrentKmsSignersContext(relayerClient, {
    kmsVerifierAddress: relayerClient.chain.fhevm.contracts.kmsVerifier.address as ChecksummedAddress,
    protocolConfigAddress: relayerClient.chain.fhevm.contracts.protocolConfig?.address as
      | ChecksummedAddress
      | undefined,
  });
  return currentKmsSignersContext.signers.length === 1 && isForgeFhevmV1KmsSigner(currentKmsSignersContext.signers[0]);
}

////////////////////////////////////////////////////////////////////////////////

type ReadCleartextExecutorAddressContext = {
  readonly relayerClient: RelayerClientWithRuntime;
  readonly trustedClient: TrustedClient;
};

// ACL → FHEVMExecutor is immutable per deployment, so the lookup is cached
// permanently (no TTL). Keyed by ACL address (lowercase) + chainId so distinct
// deployments/chains never collide, and concurrent callers share one in-flight read.
const cachedReadCleartextExecutorAddress = createCachedFetch<
  ReadCleartextExecutorAddressContext,
  Record<string, never>,
  ChecksummedAddress
>({
  executeFn: async ({ relayerClient, trustedClient }) => {
    const res = await relayerClient.runtime.ethereum.readContract(trustedClient, {
      abi: getFHEVMExecutorAddressAbi,
      address: relayerClient.chain.fhevm.contracts.acl.address as ChecksummedAddress,
      args: [],
      functionName: getFHEVMExecutorAddressAbi[0].name,
    });
    return addressToChecksummedAddress(res as Address);
  },
  cacheKeyFn: ({ relayerClient }) =>
    `${(relayerClient.chain.fhevm.contracts.acl.address as string).toLowerCase()}:${relayerClient.chain.id}`,
});

async function readCleartextExecutorAddress(
  relayerClient: RelayerClientWithRuntime,
  trustedClient: TrustedClient,
): Promise<ChecksummedAddress> {
  return cachedReadCleartextExecutorAddress.execute({ relayerClient, trustedClient }, {});
}

/**
 * Reads the cleartext uint256 recorded for each handle from
 * `CleartextFHEVMExecutor.plaintexts`. Authorization (ACL) is enforced by the
 * decrypt caller before the relayer is invoked, matching the on-chain view which
 * gated reads through `CleartextACL`.
 */
export async function readPlaintexts(
  relayerClient: RelayerClientWithRuntime,
  trustedClient: TrustedClient,
  handles: readonly Handle[],
): Promise<bigint[]> {
  const executorAddress = await readCleartextExecutorAddress(relayerClient, trustedClient);

  const cleartexts: bigint[] = [];
  for (const handle of handles) {
    const res = await relayerClient.runtime.ethereum.readContract(trustedClient, {
      abi: plaintextsAbi,
      address: executorAddress,
      args: [handle.bytes32Hex],
      functionName: plaintextsAbi[0].name,
    });
    cleartexts.push(res as bigint);
  }
  return cleartexts;
}

/**
 * Reversible mask that replicates `CleartextKMSVerifier._xorMaskWithPublicKey`:
 * XORs each cleartext with the first 32 bytes of the user's public key. The
 * client reverses it in `decrypt/mock.ts:_xorUnmaskWithPublicKey` with the same
 * 32 bytes. NOT real encryption — only the cleartext wire format.
 */
export function xorMaskWithPublicKey(publicKey: BytesHex, cleartexts: readonly bigint[]): bigint[] {
  const hex = remove0x(publicKey);
  if (hex.length < 64) {
    throw new Error(`PublicKeyTooShort: publicKey has ${hex.length / 2} bytes, need >= 32`);
  }
  const mask = BigInt('0x' + hex.slice(0, 64));
  return cleartexts.map((c) => c ^ mask);
}

/**
 * Replicates `CleartextKMSVerifier._encodeTypedCleartexts`: each cleartext is cast
 * to its FHE-natural type (bool → 0/1, address → uint160, else uint256) and packed
 * into a 32-byte word. For these static value types `abi.encode(v0, …, vn)` is
 * byte-identical to the packed layout, and it round-trips through the proof's
 * `decode(handle.solidityPrimitiveTypeName[], …)`.
 */
export function encodeTypedCleartexts(
  ethereumModule: CleartextEthereumModule,
  handles: readonly Handle[],
  cleartexts: readonly bigint[],
): BytesHex {
  const words = handles.map((handle, i) => {
    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
    const value = cleartexts[i]!;
    switch (handle.solidityPrimitiveTypeName) {
      case 'bool':
        return value !== 0n ? 1n : 0n;
      case 'address':
        return value & ((1n << 160n) - 1n);
      case 'uint256':
        return value;
    }
  });

  return ethereumModule.encode({
    types: words.map(() => 'uint256'),
    values: words,
  });
}
