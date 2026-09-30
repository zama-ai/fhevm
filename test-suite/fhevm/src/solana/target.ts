// target — which protocol the e2e drives. `SOLANA_E2E_SOURCE=cleartext` targets the cleartext stack
// (`./cleartext-stack.ts`): the SDK's cleartext clients, which take the same parameters and return
// the same actions, stand in for the relayer, coprocessors and KMS, and the trust a decrypt binds to
// is the cleartext parties'. Everything else a scenario does is the same on both targets.
import type * as SolanaSdk from '@fhevm/sdk/solana';

import { BRINGUP_KMS_CONTEXT_ID } from '../../../../solana/deploy/src/constants';
import { readActiveKmsPair, readGatewayBootstrapInputs } from './addresses';

export const targetsCleartext = (): boolean => process.env.SOLANA_E2E_SOURCE === 'cleartext';

/** `@fhevm/sdk/solana`, with the cleartext client factories on a cleartext target. */
export const loadSolanaSdk = async (): Promise<typeof SolanaSdk> => {
  const solana = await import('@fhevm/sdk/solana');
  if (!targetsCleartext()) return solana;
  const cleartext = await import('@fhevm/sdk/solana/cleartext');
  return {
    ...solana,
    createFhevmEncryptClient: cleartext.createFhevmCleartextEncryptClient,
    createFhevmDecryptClient: cleartext.createFhevmCleartextDecryptClient,
    createFhevmPublicDecryptClient: cleartext.createFhevmCleartextPublicDecryptClient,
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
