import type { SolanaPublicDecryptCertificateClaim } from '@fhevm/sdk/solana';
import type { Bytes32Hex } from '@fhevm/sdk/types';

import { loadSolanaSdk } from './target';

/**
 * The KMS public-decrypt certificate the SDK action returns — the glossary term is "certificate"
 * (the SDK's type name keeps its historical "claim" suffix). Type-only import: the SDK workspace
 * need not be materialized to run the offline suites.
 */
export type PublicDecryptCertificate = SolanaPublicDecryptCertificateClaim;
/**
 * Interprets the certificate cleartext as a number. `abiEncodedCleartext` is UNPREFIXED ABI hex
 * (a 32-byte big-endian uint256), so it must be parsed as hex explicitly — `BigInt(...)` on the
 * raw string reads all-digit hex like "46" as decimal 46 instead of 0x46 = 70.
 */
export const certificateCleartext = (certificate: Pick<PublicDecryptCertificate, 'abiEncodedCleartext'>): bigint =>
  BigInt(`0x${certificate.abiEncodedCleartext.replace(/^0x/, '')}`);

/** The facts a public-decrypt client binds to. */
export type PublicDecryptClientInput = {
  rpcUrl: string;
  chainId: bigint;
  relayerUrl: string;
  verifyingProgramId: Bytes32Hex;
  apiKey: string;
};

// Keep the dynamic import seam narrow: clean CLI checkouts do not contain the SDK's generated
// `_types`, while the full vertical exercises this public package entry at runtime.
/** The target's public-decrypt client: the relayer's, or the cleartext stack's. */
export const createPublicDecryptClient = async (input: PublicDecryptClientInput) => {
  const solana = await loadSolanaSdk();
  const { createSolanaRpc } = await import('@solana/kit');
  const rpc = createSolanaRpc(input.rpcUrl);
  const chain = solana.defineFhevmSolanaChain({ id: input.chainId, fhevm: {
      relayerUrl: input.relayerUrl,
      programs: { host: { address: input.verifyingProgramId } },
    },
  });
  solana.setFhevmRuntimeConfig({ auth: { type: 'ApiKeyHeader', value: input.apiKey } });
  return solana.createFhevmPublicDecryptClient({ chain, rpc });
};
