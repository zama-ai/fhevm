import type { Hex } from "viem";

import { confidentialFungibleTokenAbi } from "./abi";
import type { WalletContext } from "../config";
import type { ContractWriteRequest } from "../shared/transactions";

type WriteContext = Pick<
  WalletContext,
  "account" | "contractAddress" | "publicClient"
>;

/** Simulated token write: the viem request plus the returned bytes32 handle. */
export type TokenWriteSimulation = Readonly<{
  request: ContractWriteRequest;
  transferredHandle: Hex;
}>;

const simulateWriteRequest = async (
  context: WriteContext,
  options: {
    functionName: string;
    args: readonly unknown[];
  },
): Promise<TokenWriteSimulation> => {
  const { request, result } = await context.publicClient.simulateContract({
    account: context.account,
    address: context.contractAddress,
    abi: confidentialFungibleTokenAbi,
    functionName: options.functionName,
    args: options.args,
  } as never);

  return {
    request: request as ContractWriteRequest,
    transferredHandle: result as Hex,
  };
};

/** Simulates an ERC-7984 confidentialTransfer and captures the result handle. */
export const simulateConfidentialTransfer = async (
  context: WriteContext,
  options: {
    to: Hex;
    encryptedAmount: Hex;
    inputProof: Hex;
  },
): Promise<TokenWriteSimulation> => {
  return simulateWriteRequest(context, {
    functionName: "confidentialTransfer",
    args: [options.to, options.encryptedAmount, options.inputProof],
  });
};

/** Simulates an ERC-7984 confidentialTransferFrom and captures the result handle. */
export const simulateConfidentialTransferFrom = async (
  context: WriteContext,
  options: {
    from: Hex;
    to: Hex;
    encryptedAmount: Hex;
    inputProof: Hex;
  },
): Promise<TokenWriteSimulation> => {
  return simulateWriteRequest(context, {
    functionName: "confidentialTransferFrom",
    args: [options.from, options.to, options.encryptedAmount, options.inputProof],
  });
};
