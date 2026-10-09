import { findAssociatedTokenPda } from '@solana-program/token';
import { INSTRUCTIONS_SYSVAR_ADDRESS, prepareTransientStore } from '@fhevm/sdk/solana';
import { AccountRole, address, type Address, type Signature, type TransactionSigner } from '@solana/kit';
import { base58 } from '@scure/base';

import { hexToBytes } from '@fhevm/sdk/base';
import { bytes32HexToHandle, isSolanaHostChainId } from '@fhevm/sdk/solana';
import type { FhevmSolanaChain } from '@fhevm/sdk/solana';
import type { Bytes32Hex } from '@fhevm/sdk/types';
import type { SolanaInputProof } from '@fhevm/sdk/solana';
import { getConfidentialTransferInstructionAsync,
  findEventAuthorityPda, CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS, ZAMA_HOST_PROGRAM_ADDRESS } from '@fhevm/confidential-token';
import type { DemoClient } from '../../demoClient';

export type SolanaConfidentialTransferParameters = {
  readonly inputProof: SolanaInputProof;

  readonly inputIndex: number;
  readonly owner: TransactionSigner;
  readonly mint: Address;
  /** SPL mint wrapped by `mint`. Freeze checks the owners' ATAs on this mint. */
  readonly underlyingMint: Address;
  /** Token program that owns `underlyingMint` (`Tokenkeg` or Token-2022). */
  readonly tokenProgram: Address;
  readonly fromAccount: Address;
  readonly toAccount: Address;
  /** Recipient owner; used to derive the destination freeze ATA. */
  readonly toOwner: Address;
  readonly fromStore: Address;
  readonly toStore: Address;
  readonly hcuBlockMeter?: Address | undefined;
  readonly hcuTrustedAppRecord?: Address | undefined;
  readonly denyRecords?: readonly Address[] | undefined;
};

/** Builds, sends, and confirms one confidential-token transfer; `client.payer` pays the fee and transientStore rent. */
export async function confidentialTransfer(
  fhevm: { readonly solanaChain: FhevmSolanaChain; readonly aclProgramAddress: Bytes32Hex },
  client: DemoClient,
  parameters: SolanaConfidentialTransferParameters,
): Promise<Signature> {
  const { inputProof, inputIndex, owner, mint } = parameters;
  const zamaHostProgramAddress = address(base58.encode(hexToBytes(fhevm.aclProgramAddress)));
  if (zamaHostProgramAddress !== ZAMA_HOST_PROGRAM_ADDRESS) {
    throw new Error('configured ACL program does not match the host compiled into confidential-token');
  }
  const handles = inputProof.handles.map((handle) => bytes32HexToHandle(handle.bytes32Hex));
  if (!Number.isInteger(inputIndex) || inputIndex < 0 || inputIndex > 255 || inputIndex >= handles.length) {
    throw new Error(`inputIndex ${inputIndex} is outside the submitted proof`);
  }
  const inputHandle = handles[inputIndex];
  if (inputHandle === undefined) throw new Error(`inputIndex ${inputIndex} is outside the submitted proof`);
  if (inputHandle.fheType !== 'euint64') throw new Error('confidential transfer amount must be euint64');
  if (!isSolanaHostChainId(inputProof.chainId)) throw new Error('confidential transfer requires a Solana chain id');
  if (inputProof.chainId !== fhevm.solanaChain.id)
    throw new Error('input proof chain id does not match the client chain');
  if (base58.encode(hexToBytes(inputProof.aclContractAddress)) !== zamaHostProgramAddress) {
    throw new Error('input proof ACL does not match the configured Zama host program');
  }

  if (base58.encode(hexToBytes(inputProof.userAddress)) !== owner.address) {
    throw new Error('input proof user does not match the transfer owner');
  }
  // The token program re-checks this binding in-execution (`assert_amount_attestation_binding`).
  if (base58.encode(hexToBytes(inputProof.contractAddress)) !== CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS) {
    throw new Error('input proof contract does not match the confidential-token program');
  }
  const signatures = inputProof.signatures.map((signature, index) => {
    const bytes = hexToBytes(signature);
    if (bytes.length !== 65) throw new Error(`input proof signature[${index}] must be 65 bytes`);
    return bytes;
  });
  if (
    parameters.fromAccount === parameters.toAccount &&
    parameters.denyRecords !== undefined &&
    parameters.denyRecords.length > 0
  ) {
    throw new Error('self-transfers cannot include deny records');
  }

  const tokenEventAuthority = (await findEventAuthorityPda())[0];
  const transientStore = await prepareTransientStore({ payer: client.payer, host: zamaHostProgramAddress });
  const transferInstruction = await getConfidentialTransferInstructionAsync({
    transientStore: transientStore.address,
    instructions: INSTRUCTIONS_SYSVAR_ADDRESS,
    owner,
    payer: client.payer,
    mint,
    underlyingMint: parameters.underlyingMint,
    fromAta: (await findAssociatedTokenPda({
      owner: owner.address,
      tokenProgram: parameters.tokenProgram,
      mint: parameters.underlyingMint,
    }))[0],
    toAta: (await findAssociatedTokenPda({
      owner: parameters.toOwner,
      tokenProgram: parameters.tokenProgram,
      mint: parameters.underlyingMint,
    }))[0],
    fromAccount: parameters.fromAccount,
    toAccount: parameters.toAccount,
    fromStore: parameters.fromStore,
    toStore: parameters.toStore,
    zamaProgram: zamaHostProgramAddress,
    ...(parameters.hcuBlockMeter !== undefined ? { hcuBlockMeter: parameters.hcuBlockMeter } : {}),
    ...(parameters.hcuTrustedAppRecord !== undefined ? { hcuTrustedAppRecord: parameters.hcuTrustedAppRecord } : {}),
    eventAuthority: tokenEventAuthority,
    program: CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS,
    amountAttestation: {
      inputHandle: hexToBytes(inputHandle.bytes32Hex),
      ctHandles: handles.map((handle) => hexToBytes(handle.bytes32Hex)),
      handleIndex: inputIndex,
      userAddress: hexToBytes(inputProof.userAddress),
      contractAddress: hexToBytes(inputProof.contractAddress),
      contractChainId: inputProof.chainId,
      extraData: hexToBytes(inputProof.extraData),
      signatures,
    },
    // A plain transfer leaves no transferred-amount receipt for a recipient program.
  });
  const instruction =
    parameters.denyRecords !== undefined && parameters.denyRecords.length > 0
      ? {
          ...transferInstruction,
          accounts: [
            ...transferInstruction.accounts,
            ...parameters.denyRecords.map((denyAddress) => ({
              address: denyAddress,
              role: AccountRole.READONLY,
            })),
          ],
        }
      : transferInstruction;
  return (await client.sendFheTransaction(transientStore, [instruction])).context.signature;
}
