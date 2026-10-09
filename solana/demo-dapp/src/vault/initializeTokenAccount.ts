import { tokenStoreAddress } from './internal/encryptedStores.js';
import { tokenApp, withDenyRecords, type HostPolicyParameters } from './internal/hostPolicy.js';
import { INSTRUCTIONS_SYSVAR_ADDRESS, type TransientStore } from '@fhevm/sdk/solana';
import type { Address, Instruction, TransactionSigner } from '@solana/kit';
import {
  getInitializeTokenAccountInstructionAsync,
  findTokenAccountPda,
  CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS,
} from '@fhevm/confidential-token';

export type SolanaVaultInitializeTokenAccountParameters = HostPolicyParameters & {
  readonly transientStore: TransientStore;
  /** Signer funding the new confidential account and encrypted balance. */
  readonly payer: TransactionSigner;
  /** Owner of the confidential account. This address does not need to sign. */
  readonly owner: Address;
  /** The confidential mint this account belongs to. */
  readonly mint: Address;
};

/**
 * Builds `confidential_token::initialize_token_account`: creates the owner's confidential token
 * account PDA for `mint` and its zero balance handle, or leaves an existing account unchanged, so a
 * caller can always include it. The account PDA, its balance encrypted store, and the two Anchor
 * event authorities are derived here from `(mint, owner)`.
 */
export async function buildInitializeTokenAccountInstruction(
  parameters: SolanaVaultInitializeTokenAccountParameters,
): Promise<Instruction> {
  const [tokenAccount] = await findTokenAccountPda({ mint: parameters.mint, owner: parameters.owner });
  const app = tokenApp(parameters.mint);
  const hcu = await parameters.host.hcuAccounts(app);
  const instruction = await getInitializeTokenAccountInstructionAsync({
    transientStore: parameters.transientStore.address,
    instructions: INSTRUCTIONS_SYSVAR_ADDRESS,
    payer: parameters.payer,
    owner: parameters.owner,
    mint: parameters.mint,
    tokenAccount,
    balanceEncryptedStore: await tokenStoreAddress(parameters.mint, tokenAccount),
    program: CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS,
    hcuBlockMeter: hcu.hcuBlockMeter,
    hcuTrustedAppRecord: hcu.hcuTrustedAppRecord,
  });
  return withDenyRecords(instruction, parameters.host.denyListEnabled, [app]);
}
