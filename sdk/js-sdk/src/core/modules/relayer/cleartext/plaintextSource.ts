import type { Address, BytesHex, ChecksummedAddress } from '../../../types/primitives.js';
import type { Handle } from '../../../types/encryptedTypes-p.js';
import type { CleartextEthereumModule } from '../../ethereum/types-ct.js';
import type { TrustedClient } from '../../ethereum/types.js';
import type { RelayerClientWithRuntime } from '../types.js';
import { remove0x } from '../../../base/string.js';
import { asUint32BigInt } from '../../../base/uint.js';
import { addressToChecksummedAddress } from '../../../base/address.js';
import {
  getFHEVMExecutorAddressAbi,
  getKmsSignersAbi,
  getThresholdAbi,
} from '../../../host-contracts/abi-fragments/fragments.js';

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

/**
 * Resolves the (cleartext) FHEVMExecutor address from the host ACL, so the
 * `plaintexts` mapping can be read without hardcoding a host-address constant.
 * forge-fhevm wires its `CleartextFHEVMExecutor` in as the ACL's executor.
 */
export async function readCleartextExecutorAddress(
  relayerClient: RelayerClientWithRuntime,
  trustedClient: TrustedClient,
): Promise<ChecksummedAddress> {
  const res = await relayerClient.runtime.ethereum.readContract(trustedClient, {
    abi: getFHEVMExecutorAddressAbi,
    address: relayerClient.chain.fhevm.contracts.acl.address as ChecksummedAddress,
    args: [],
    functionName: getFHEVMExecutorAddressAbi[0].name,
  });
  return addressToChecksummedAddress(res as Address);
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
  executorAddress: ChecksummedAddress,
  handles: readonly Handle[],
): Promise<bigint[]> {
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
 * Reads the registered KMS signer set and threshold from the standard KMSVerifier
 * (which forge-fhevm implements). These replace the `signers` / `threshold` the
 * `Cleartext*KMSVerifier` view used to return.
 */
export async function readKmsSignersAndThreshold(
  relayerClient: RelayerClientWithRuntime,
  trustedClient: TrustedClient,
): Promise<{ signers: readonly ChecksummedAddress[]; threshold: bigint }> {
  const kmsVerifierAddress = relayerClient.chain.fhevm.contracts.kmsVerifier.address as ChecksummedAddress;

  const signersRes = await relayerClient.runtime.ethereum.readContract(trustedClient, {
    abi: getKmsSignersAbi,
    address: kmsVerifierAddress,
    args: [],
    functionName: getKmsSignersAbi[0].name,
  });

  const thresholdRes = await relayerClient.runtime.ethereum.readContract(trustedClient, {
    abi: getThresholdAbi,
    address: kmsVerifierAddress,
    args: [],
    functionName: getThresholdAbi[0].name,
  });

  const signers = (signersRes as readonly Address[]).map(addressToChecksummedAddress);
  return { signers, threshold: asUint32BigInt(thresholdRes) };
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
