import {
  AccountRole,
  address,
  appendTransactionMessageInstructions,
  assertIsFullySignedTransaction,
  assertIsTransactionWithBlockhashLifetime,
  assertIsTransactionWithinSizeLimit,
  createTransactionMessage,
  getBase64EncodedWireTransaction,
  getProgramDerivedAddress,
  getSignatureFromTransaction,
  pipe,
  sendAndConfirmTransactionFactory,
  setTransactionMessageComputeUnitLimit,
  setTransactionMessageFeePayerSigner,
  setTransactionMessageLifetimeUsingBlockhash,
  signTransactionMessageWithSigners,
  type Address,
  type Rpc,
  type RpcSubscriptions,
  type Signature,
  type SolanaRpcApi,
  type SolanaRpcSubscriptionsApi,
  type TransactionSigner,
} from '@solana/kit';
import { base58 } from '@scure/base';

import { hexToBytes } from '../../core/base/bytes.js';
import { assertHandleArrayEquals } from '../../core/handle/FhevmHandle.js';
import type { SolanaZkProof } from '../../core/types/zkProof-p.js';
import type { FhevmSolanaChain } from '../../core/types/fhevmSolanaChain.js';
import type { Bytes32Hex } from '../../core/types/primitives.js';
import type { SolanaSubmitInputProofResult } from './submitInputProof.js';
import { deriveValueKey } from '../proof.js';
import { getConfidentialTransferInstructionAsync } from '../internal/generated/confidentialToken/instructions/confidentialTransfer.js';
import { findComputeSignerPda } from '../internal/generated/confidentialToken/pdas/computeSigner.js';
import {
  CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS,
  ZAMA_HOST_PROGRAM_ADDRESS,
} from '../internal/generated/confidentialToken/programAddress.js';

const EVENT_AUTHORITY_SEED = new TextEncoder().encode('__event_authority');
const HCU_AUTHORITY_SEED = new TextEncoder().encode('hcu-authority');
const ENCRYPTED_VALUE_SEED = new TextEncoder().encode('encrypted-value');
const TRANSFERRED_AMOUNT_LABEL = new TextEncoder().encode('transferred_amount______________');

export type SolanaConfidentialTransferParameters = {
  readonly rpc: Rpc<SolanaRpcApi>;
  readonly rpcSubscriptions: RpcSubscriptions<SolanaRpcSubscriptionsApi>;
  readonly inputProof: SolanaZkProof;
  readonly inputProofResult: SolanaSubmitInputProofResult;
  readonly inputIndex: number;
  readonly owner: TransactionSigner;
  readonly feePayer: TransactionSigner;
  readonly mint: Address;
  readonly fromAccount: Address;
  readonly toAccount: Address;
  readonly fromBalanceValue: Address;
  readonly toBalanceValue: Address;
  readonly hostConfig: Address;
  readonly hcuBlockMeter?: Address | undefined;
  readonly hcuTrustedAppRecord?: Address | undefined;
  readonly denyRecords?: readonly Address[] | undefined;
};

async function pda(programAddress: Address, seeds: Uint8Array[]): Promise<Address> {
  return (await getProgramDerivedAddress({ programAddress, seeds }))[0];
}

async function transferredAmountValue(
  zamaHostProgramAddress: Address,
  mint: Address,
  fromAccount: Address,
): Promise<Address> {
  const valueKey = deriveValueKey(base58.decode(mint), base58.decode(fromAccount), TRANSFERRED_AMOUNT_LABEL);
  return pda(zamaHostProgramAddress, [ENCRYPTED_VALUE_SEED, valueKey]);
}

/** Builds, simulates, sends, and confirms one confidential-token transfer. */
export async function confidentialTransfer(
  fhevm: { readonly solanaChain: FhevmSolanaChain; readonly aclProgramAddress: Bytes32Hex },
  parameters: SolanaConfidentialTransferParameters,
): Promise<Signature> {
  const { inputProof, inputProofResult, inputIndex, owner, feePayer, mint } = parameters;
  const zamaHostProgramAddress = address(base58.encode(hexToBytes(fhevm.aclProgramAddress)));
  if (zamaHostProgramAddress !== ZAMA_HOST_PROGRAM_ADDRESS) {
    throw new Error('configured ACL program does not match the host compiled into confidential-token');
  }
  const handles = inputProof.getInputHandles();
  assertHandleArrayEquals(inputProofResult.handles, handles, {
    actualName: 'input proof submission',
    expectedName: 'input proof',
  });
  if (!Number.isInteger(inputIndex) || inputIndex < 0 || inputIndex > 255 || inputIndex >= handles.length) {
    throw new Error(`inputIndex ${inputIndex} is outside the submitted proof`);
  }
  const inputHandle = handles[inputIndex];
  if (inputHandle === undefined) throw new Error(`inputIndex ${inputIndex} is outside the submitted proof`);
  if (inputProof.encryptionBits[inputIndex] !== 64) throw new Error('confidential transfer amount must be euint64');
  if ((inputProof.chainId & (1n << 63n)) === 0n) throw new Error('confidential transfer requires a Solana chain id');
  if (inputProof.chainId !== fhevm.solanaChain.id)
    throw new Error('input proof chain id does not match the client chain');
  if (base58.encode(hexToBytes(inputProof.aclContractAddress)) !== zamaHostProgramAddress) {
    throw new Error('input proof ACL does not match the configured Zama host program');
  }

  const [computeSigner] = await findComputeSignerPda({ mint });
  if (base58.encode(hexToBytes(inputProof.userAddress)) !== owner.address) {
    throw new Error('input proof user does not match the transfer owner');
  }
  if (base58.encode(hexToBytes(inputProof.contractAddress)) !== computeSigner) {
    throw new Error('input proof contract does not match the mint compute signer');
  }
  const signatures = inputProofResult.signatures.map((signature, index) => {
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

  const tokenEventAuthority = await pda(CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS, [EVENT_AUTHORITY_SEED]);
  const zamaEventAuthority = await pda(zamaHostProgramAddress, [EVENT_AUTHORITY_SEED]);
  const hcuAuthority = await pda(CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS, [HCU_AUTHORITY_SEED, base58.decode(mint)]);
  const transferInstruction = await getConfidentialTransferInstructionAsync({
    owner,
    payer: feePayer,
    mint,
    fromAccount: parameters.fromAccount,
    toAccount: parameters.toAccount,
    fromBalanceValue: parameters.fromBalanceValue,
    toBalanceValue: parameters.toBalanceValue,
    transferredAmountValue: await transferredAmountValue(zamaHostProgramAddress, mint, parameters.fromAccount),
    zamaEventAuthority,
    zamaProgram: zamaHostProgramAddress,
    hostConfig: parameters.hostConfig,
    ...(parameters.hcuBlockMeter !== undefined ? { hcuBlockMeter: parameters.hcuBlockMeter } : {}),
    ...(parameters.hcuTrustedAppRecord !== undefined ? { hcuTrustedAppRecord: parameters.hcuTrustedAppRecord } : {}),
    hcuAuthority,
    eventAuthority: tokenEventAuthority,
    program: CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS,
    amountAttestation: {
      inputHandle: hexToBytes(inputHandle.bytes32Hex),
      ctHandles: handles.map((handle) => hexToBytes(handle.bytes32Hex)),
      handleIndex: inputIndex,
      userAddress: hexToBytes(inputProof.userAddress),
      contractAddress: hexToBytes(inputProof.contractAddress),
      contractChainId: inputProof.chainId,
      extraData: hexToBytes(inputProofResult.extraData),
      signatures,
    },
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
  const { value: latestBlockhash } = await parameters.rpc.getLatestBlockhash({ commitment: 'confirmed' }).send();
  const message = pipe(
    createTransactionMessage({ version: 0 }),
    (m) => setTransactionMessageFeePayerSigner(feePayer, m),
    (m) => setTransactionMessageLifetimeUsingBlockhash(latestBlockhash, m),
    (m) => setTransactionMessageComputeUnitLimit(400_000, m),
    (m) => appendTransactionMessageInstructions([instruction], m),
  );
  const transaction = await signTransactionMessageWithSigners(message);
  assertIsFullySignedTransaction(transaction);
  assertIsTransactionWithBlockhashLifetime(transaction);
  assertIsTransactionWithinSizeLimit(transaction);
  const wireTransaction = getBase64EncodedWireTransaction(transaction);
  const simulation = await parameters.rpc
    .simulateTransaction(wireTransaction, {
      commitment: 'confirmed',
      encoding: 'base64',
      sigVerify: true,
    })
    .send();
  if (simulation.value.err !== null)
    throw new Error(`confidential transfer simulation failed: ${JSON.stringify(simulation.value.err)}`);
  await sendAndConfirmTransactionFactory({ rpc: parameters.rpc, rpcSubscriptions: parameters.rpcSubscriptions })(
    transaction,
    {
      commitment: 'confirmed',
      skipPreflight: true,
    },
  );
  return getSignatureFromTransaction(transaction);
}
