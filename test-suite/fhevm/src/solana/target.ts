// target — which protocol the e2e drives. `SOLANA_E2E_SOURCE=cleartext` targets the cleartext stack
// (`./cleartext-stack.ts`): the SDK's cleartext clients, which take the same parameters and return
// the same actions, stand in for the relayer, coprocessors and KMS, and the trust a decrypt binds to
// is the cleartext parties'. Everything else a scenario does is the same on both targets.
import type * as SolanaSdk from '@fhevm/sdk/solana';
import type { SolanaMerkleProofReader } from '@fhevm/sdk/solana/cleartext';

import { BRINGUP_KMS_CONTEXT_ID } from '../../../../solana/deploy/src/constants';
import { ZAMA_HOST_PROGRAM_ADDRESS } from '@fhevm/solana-zama-host';
import { readActiveKmsPair, readGatewayBootstrapInputs } from './addresses';

/** The protocol `SOLANA_E2E_SOURCE` names: the local stack, devnet, or the cleartext stack. */
export const solanaE2eSource = (env: NodeJS.ProcessEnv = process.env): 'local' | 'devnet' | 'cleartext' => {
  const value = env.SOLANA_E2E_SOURCE ?? 'local';
  if (value !== 'local' && value !== 'devnet' && value !== 'cleartext') {
    throw new Error(`SOLANA_E2E_SOURCE must be "local", "devnet" or "cleartext", got ${value}`);
  }
  return value;
};

export const targetsCleartext = (): boolean => solanaE2eSource() === 'cleartext';

// The cleartext stack's leaf record, kept for the test process as coprocessors keep theirs, so a
// decrypt reads only the store writes since the last one.
let cleartextLeafRecord: SolanaMerkleProofReader | undefined;

/** `@fhevm/sdk/solana`, with the cleartext client factories on a cleartext target. */
export const loadSolanaSdk = async (): Promise<typeof SolanaSdk> => {
  const solana = await import('@fhevm/sdk/solana');
  if (!targetsCleartext()) return solana;
  const cleartext = await import('@fhevm/sdk/solana/cleartext');
  const readMerkleProofs = ({ rpc }: { readonly rpc: SolanaSdk.SolanaRpc }) =>
    (cleartextLeafRecord ??= cleartext.createSolanaLeafRecord(rpc, ZAMA_HOST_PROGRAM_ADDRESS));
  return {
    ...solana,
    createFhevmEncryptClient: cleartext.createFhevmCleartextEncryptClient,
    createFhevmDecryptClient: (parameters) =>
      cleartext.createFhevmCleartextDecryptClient({ ...parameters, readMerkleProofs: readMerkleProofs(parameters) }),
    createFhevmPublicDecryptClient: (parameters) =>
      cleartext.createFhevmCleartextPublicDecryptClient({ ...parameters, readMerkleProofs: readMerkleProofs(parameters) }),
  };
};

/** Whom a decrypt believes: the KMS signer set, the gateway domain, and the KMS pair permits name. */
export type DecryptTrustInputs = {
  readonly kmsSigners: readonly `0x${string}`[];
  readonly gatewayChainId: bigint;
  readonly decryptionContract: `0x${string}`;
  readonly kmsContextId: bigint;
  readonly kmsEpochId: bigint;
};

/**
 * Reads the decrypt trust live: the signer set and Decryption contract from the gateway, the active
 * KMS pair from the primary host chain's ProtocolConfig, the pair the KMS Connector validates each
 * permit against. A cleartext target has no gateway or KMS; its trust is the cleartext parties and
 * the context the stack bootstraps.
 */
export const readDecryptTrustInputs = async (endpoints: {
  readonly gatewayRpcUrl: string;
  readonly hostRpcUrl: string;
}): Promise<DecryptTrustInputs> => {
  if (targetsCleartext()) {
    const { SOLANA_CLEARTEXT_GATEWAY, SOLANA_CLEARTEXT_SIGNER_ADDRESSES } = await import('@fhevm/sdk/solana/cleartext');
    return {
      kmsSigners: SOLANA_CLEARTEXT_SIGNER_ADDRESSES.kms,
      gatewayChainId: BigInt(SOLANA_CLEARTEXT_GATEWAY.id),
      decryptionContract: SOLANA_CLEARTEXT_GATEWAY.contracts.decryption.address,
      kmsContextId: BigInt(`0x${Buffer.from(BRINGUP_KMS_CONTEXT_ID).toString('hex')}`),
      // No KMS serves a cleartext stack, so nothing checks the epoch a permit names.
      kmsEpochId: 1n,
    };
  }
  const [gateway, kmsPair] = await Promise.all([
    readGatewayBootstrapInputs({ gatewayRpcUrl: endpoints.gatewayRpcUrl }),
    readActiveKmsPair({ hostRpcUrl: endpoints.hostRpcUrl }),
  ]);
  const hex20 = (bytes: Uint8Array): `0x${string}` => `0x${Buffer.from(bytes).toString('hex')}`;
  return {
    kmsSigners: gateway.kmsSigners.map(hex20),
    gatewayChainId: gateway.gatewayChainId,
    decryptionContract: hex20(gateway.decryptionContract),
    kmsContextId: kmsPair.kmsContextId,
    kmsEpochId: kmsPair.kmsEpochId,
  };
};
