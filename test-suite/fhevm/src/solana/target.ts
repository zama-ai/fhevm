// target — which protocol the e2e drives. `SOLANA_E2E_SOURCE=cleartext` targets the cleartext stack
// (`./cleartext-stack.ts`): the SDK's cleartext clients, which take the same parameters and return
// the same actions, stand in for the relayer, coprocessors and KMS, and the trust a decrypt binds to
// is the cleartext parties'. Everything else a scenario does is the same on both targets.
import type * as SolanaSdk from '@fhevm/sdk/solana';
import type { SolanaMerkleProofReader } from '@fhevm/sdk/solana/cleartext';

import { ZAMA_HOST_PROGRAM_ADDRESS } from '@fhevm/solana-zama-host';

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

