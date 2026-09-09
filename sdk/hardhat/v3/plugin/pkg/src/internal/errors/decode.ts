// Finds the FHEVM contract a revert came from: the return data is tried against every ABI the
// repository knows, and exactly one contract must own the selector — zero means "not ours", several
// (OpenZeppelin errors shared by every proxy) means ambiguous, and both are left alone.

import { type AbiParameter, type Address, type Hex, decodeErrorResult } from 'viem';

import type { FhevmContractWrapper, FhevmContractsRepository } from '../contracts.js';

export type DecodedFhevmError = {
  readonly wrapper: FhevmContractWrapper;
  readonly errorName: string;
  readonly inputs: readonly AbiParameter[];
  readonly args: readonly unknown[];
};

export function decodeFhevmError(repository: FhevmContractsRepository, data: Hex): DecodedFhevmError | undefined {
  const owners: DecodedFhevmError[] = [];
  for (const wrapper of repository.addressToContractMap().values()) {
    const decoded = tryDecode(wrapper, data);
    if (decoded !== undefined) owners.push(decoded);
  }
  return owners.length === 1 ? owners[0] : undefined;
}

/** Decodes against the contract at `address` only — the one a stack trace names as the reverting callee. */
export function decodeFhevmErrorAt(
  repository: FhevmContractsRepository,
  address: Address,
  data: Hex,
): DecodedFhevmError | undefined {
  const wrapper = repository.getContractFromAddress(address);
  return wrapper === undefined ? undefined : tryDecode(wrapper, data);
}

function tryDecode(wrapper: FhevmContractWrapper, data: Hex): DecodedFhevmError | undefined {
  try {
    const { abiItem, errorName, args } = decodeErrorResult({ abi: wrapper.abi, data });
    const inputs = abiItem.type === 'error' ? abiItem.inputs : [];
    return { wrapper, errorName, inputs, args: args ?? [] };
  } catch {
    return undefined;
  }
}
