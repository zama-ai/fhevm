// `tryParseFhevmError`: the structured view of a revert a test caught itself. Only InputVerifier's
// `InvalidSigner` has a structured shape so far (the one developers hit most: an input encrypted for
// another contract or user); everything else answers undefined, as in v2.

import { getAddress } from 'viem';

import type { FhevmContractError, FhevmInputVerifierError } from '../../types.js';
import type { FhevmContractsRepository } from '../contracts.js';
import { type RevertData, decodeRevert, extractRevertData } from './decorate.js';
import { type TransactionParties, formatFhevmErrorMessages } from './messages.js';

const GENERIC_SHORT_MESSAGE =
  "The transaction's contract address or signer account differs from the ones originally used to create the encrypted input. Please ensure they match to avoid encryption errors.";
const GENERIC_LONG_MESSAGE = `You created an encrypted input using createEncryptedInput() with a specific
contract address and user address.

However, you're now attempting to use this encrypted input in a contract
transaction involving a different contract address and/or signing account.

Encrypted inputs are bound to both the contract and the user they were
created for. To ensure proper encryption and execution, the same contract
address and user address must be used.

For example:
------------
  const input = fhevm.createEncryptedInput(fooContract.target, barAccount);
  await fooContract.connect(barAccount).someFunc(<input arguments>);
`;

export async function parseFhevmError(
  repository: FhevmContractsRepository,
  e: unknown,
): Promise<FhevmContractError | undefined> {
  const revert =
    extractRevertData(e) ??
    extractRevertData(nested(e, 'error')) ??
    extractRevertData(nested(nested(e, 'info'), 'error'));
  if (revert === undefined) return undefined;
  const decoded = decodeRevert(repository, revert);
  if (decoded === undefined) return undefined;
  if (decoded.wrapper.name !== 'InputVerifier' || decoded.errorName !== 'InvalidSigner') return undefined;
  return invalidSignerError(decoded, await transactionParties(repository, revert));
}

// ethers wraps the provider error it received: `error.error`, or `error.info.error` on a call exception.
function nested(holder: unknown, key: string): unknown {
  return typeof holder === 'object' && holder !== null && key in holder
    ? (holder as Record<string, unknown>)[key]
    : undefined;
}

async function transactionParties(
  repository: FhevmContractsRepository,
  revert: RevertData,
): Promise<TransactionParties> {
  if (revert.transactionHash === undefined) return {};
  try {
    const tx = await repository.client.getTransaction({ hash: revert.transactionHash });
    return { from: tx.from, to: tx.to ?? undefined };
  } catch {
    return {};
  }
}

function invalidSignerError(
  decoded: NonNullable<ReturnType<typeof decodeRevert>>,
  tx: TransactionParties,
): FhevmInputVerifierError {
  const parties =
    tx.from !== undefined && tx.to !== undefined ? { from: getAddress(tx.from), to: getAddress(tx.to) } : undefined;
  const messages = formatFhevmErrorMessages(decoded, parties ?? {});
  return {
    type: 'InputVerifier',
    name: 'InvalidSigner',
    ...(parties !== undefined ? { txContractAddress: parties.to, txUserAddress: parties.from } : {}),
    shortMessage: messages.shortMessage ?? GENERIC_SHORT_MESSAGE,
    longMessage: messages.longMessage ?? GENERIC_LONG_MESSAGE,
  };
}
