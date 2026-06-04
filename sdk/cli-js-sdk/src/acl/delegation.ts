import type { Hex } from "viem";

import type { ClientContext, WalletContext } from "../config";
import type { ProgressReporter } from "../shared/progress";
import { sendAndWait } from "../shared/transactions";
import { aclUserDecryptionAbi } from "./abi";

type PublicContext = Pick<ClientContext, "chain" | "contractAddress" | "publicClient">;
type DelegatorContext = Pick<
  WalletContext,
  "account" | "chain" | "contractAddress" | "publicClient" | "walletClient"
>;

/** ACL delegation state for one delegator/delegate/contract tuple. */
export type DelegationStatus = Readonly<{
  aclAddress: Hex;
  delegatorAddress: Hex;
  delegateAddress: Hex;
  contractAddress: Hex;
  expirationDate: string;
  transactionHash?: Hex;
}>;

const aclAddressOf = (context: Pick<ClientContext, "chain">): Hex =>
  context.chain.fhevm.contracts.acl.address as Hex;

/** Reads the current ACL user-decryption delegation expiration timestamp. */
export const getUserDecryptionDelegationExpirationDate = async (
  context: PublicContext,
  options: {
    delegatorAddress: Hex;
    delegateAddress: Hex;
  },
): Promise<bigint> =>
  (await context.publicClient.readContract({
    address: aclAddressOf(context),
    abi: aclUserDecryptionAbi,
    functionName: "getUserDecryptionDelegationExpirationDate",
    args: [
      options.delegatorAddress,
      options.delegateAddress,
      context.contractAddress,
    ],
  })) as bigint;

/** Creates or extends ACL permission for a delegate to user-decrypt owner data. */
export const delegateForUserDecryption = async (
  context: DelegatorContext,
  options: {
    delegateAddress: Hex;
    expirationDate: bigint;
    onProgress?: ProgressReporter;
  },
): Promise<Hex> => {
  options.onProgress?.(
    `Delegating user decryption to ${options.delegateAddress}`,
  );
  const { request } = await context.publicClient.simulateContract({
    account: context.account,
    address: aclAddressOf(context),
    abi: aclUserDecryptionAbi,
    functionName: "delegateForUserDecryption",
    args: [
      options.delegateAddress,
      context.contractAddress,
      options.expirationDate,
    ],
  });

  return sendAndWait({
    walletClient: context.walletClient,
    publicClient: context.publicClient,
    request,
    onProgress: options.onProgress,
  });
};

/**
 * Reuses an active user-decryption delegation or creates one when possible.
 *
 * Cached delegated flows can call this without delegator credentials only when
 * the ACL already has an unexpired delegation.
 */
export const ensureUserDecryptionDelegation = async (
  context: PublicContext,
  options: {
    delegatorContext?: DelegatorContext;
    delegatorAddress: Hex;
    delegateAddress: Hex;
    durationDays: number;
    onProgress?: ProgressReporter;
  },
): Promise<DelegationStatus> => {
  const aclAddress = aclAddressOf(context);
  const currentExpiration = await getUserDecryptionDelegationExpirationDate(
    context,
    options,
  );
  const block = await context.publicClient.getBlock();

  if (currentExpiration > block.timestamp) {
    options.onProgress?.(
      `Delegation already active until ${currentExpiration.toString()}`,
    );
    return {
      aclAddress,
      delegatorAddress: options.delegatorAddress,
      delegateAddress: options.delegateAddress,
      contractAddress: context.contractAddress,
      expirationDate: currentExpiration.toString(),
    };
  }

  if (!options.delegatorContext) {
    throw new Error(
      `No active user decryption delegation from ${options.delegatorAddress} to ${options.delegateAddress}. Provide delegator credentials to create it.`,
    );
  }

  const durationSeconds = BigInt(options.durationDays) * 86_400n;
  const expirationDate = block.timestamp + durationSeconds;
  const transactionHash = await delegateForUserDecryption(
    options.delegatorContext,
    {
      delegateAddress: options.delegateAddress,
      expirationDate,
      onProgress: options.onProgress,
    },
  );

  return {
    aclAddress,
    delegatorAddress: options.delegatorAddress,
    delegateAddress: options.delegateAddress,
    contractAddress: context.contractAddress,
    expirationDate: expirationDate.toString(),
    transactionHash,
  };
};
