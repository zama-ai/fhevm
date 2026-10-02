// The parties a Solana cleartext host is bootstrapped with. The cleartext clients sign with these keys
// in place of the coprocessors and the KMS: forge-fhevm's deploy-default keys, the ones an EVM
// cleartext deployment registers.
import type { ReadonlyUint8Array } from '@solana/kit';
import type { Bytes65Hex, BytesHex, ChecksummedAddress } from '../../core/types/primitives.js';
import { bytesToHex } from '../../core/base/bytes.js';
import { sign } from '../../core/base/sign.js';
import { localTestnet } from '../../core/chains/definitions/localTestnet.js';
import {
  FORGE_FHEVM_V1_COPROCESSOR_SIGNER,
  FORGE_FHEVM_V1_KMS_SIGNER,
} from '../../core/modules/relayer/cleartext/signers.js';

type Signer = { readonly address: ChecksummedAddress; readonly privateKey: BytesHex };

const SIGNERS = {
  coprocessor: [FORGE_FHEVM_V1_COPROCESSOR_SIGNER],
  kms: [FORGE_FHEVM_V1_KMS_SIGNER],
} as const satisfies Record<string, readonly Signer[]>;

/** The EVM addresses a Solana cleartext host registers as its coprocessor and KMS signer sets. */
export const SOLANA_CLEARTEXT_SIGNER_ADDRESSES = {
  coprocessor: SIGNERS.coprocessor.map((signer) => signer.address),
  kms: SIGNERS.kms.map((signer) => signer.address),
} as const;

/** The gateway a Solana cleartext host names in its EIP-712 domains: the local testnet's. */
export const SOLANA_CLEARTEXT_GATEWAY = localTestnet.fhevm.gateway;

/**
 * Signs `digest` with `threshold` of the `registered` signers, as the host verifies it. Every
 * registered signer must be a cleartext key: a host bootstrapped with other signers is not a
 * cleartext host, and signing for it would only fail later on chain.
 */
export function signAsCleartextParty(
  party: keyof typeof SIGNERS,
  registered: readonly ReadonlyUint8Array[],
  threshold: number,
  digest: BytesHex,
): Bytes65Hex[] {
  const keys = registered.map((signer) => {
    const address = bytesToHex(new Uint8Array(signer)).toLowerCase();
    const known = SIGNERS[party].find((candidate) => candidate.address.toLowerCase() === address);
    if (known === undefined) throw new Error(`${party} signer ${address} is not a cleartext key`);
    return known.privateKey;
  });
  if (threshold < 1 || threshold > keys.length) throw new Error(`invalid ${party} threshold ${threshold}`);
  return keys.slice(0, threshold).map((privateKey) => sign({ hash: digest, privateKey }));
}
