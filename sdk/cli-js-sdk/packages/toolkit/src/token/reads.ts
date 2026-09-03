import type { Hex } from "viem";

import { confidentialFungibleTokenAbi } from "./abi";
import type { ClientContext } from "../config";

type ReadContext = Pick<ClientContext, "publicClient" | "contractAddress">;

/** ERC-7984 token metadata; fields are undefined when the token omits them. */
export type TokenMetadata = Readonly<{
  name?: string;
  symbol?: string;
  decimals?: number;
}>;

/** Reads the caller-visible confidential balance handle for an account. */
export const readConfidentialBalance = async (
  context: ReadContext,
  account: Hex,
): Promise<Hex> =>
  (await context.publicClient.readContract({
    address: context.contractAddress,
    abi: confidentialFungibleTokenAbi,
    functionName: "confidentialBalanceOf",
    args: [account],
  } as never)) as Hex;

const readOptional = async <T>(read: () => Promise<T>): Promise<T | undefined> => {
  try {
    return await read();
  } catch {
    return undefined;
  }
};

/** Reads name/symbol/decimals, tolerating tokens that omit individual fields. */
export const readTokenMetadata = async (
  context: ReadContext,
): Promise<TokenMetadata> => {
  const [name, symbol, decimals] = await Promise.all([
    readOptional(
      async () =>
        (await context.publicClient.readContract({
          address: context.contractAddress,
          abi: confidentialFungibleTokenAbi,
          functionName: "name",
        } as never)) as string,
    ),
    readOptional(
      async () =>
        (await context.publicClient.readContract({
          address: context.contractAddress,
          abi: confidentialFungibleTokenAbi,
          functionName: "symbol",
        } as never)) as string,
    ),
    readOptional(
      async () =>
        Number(
          (await context.publicClient.readContract({
            address: context.contractAddress,
            abi: confidentialFungibleTokenAbi,
            functionName: "decimals",
          } as never)) as number,
        ),
    ),
  ]);

  return { name, symbol, decimals };
};
