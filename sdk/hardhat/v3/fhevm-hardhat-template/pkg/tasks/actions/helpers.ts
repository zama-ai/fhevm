import type { NetworkConnection } from 'hardhat/types/network';
import { type Address, getAddress } from 'viem';

export type CounterTaskArgs = { address: string; value: number };

export function parseAddress(address: string): Address {
  return getAddress(address);
}

export function parseValue(value: number): number {
  if (!Number.isSafeInteger(value) || value < 0) {
    throw new Error(`Invalid value '${String(value)}': expected a non-negative safe integer.`);
  }
  return value;
}

export async function firstWallet(connection: NetworkConnection) {
  return (await connection.viem.getWalletClients())[0];
}
