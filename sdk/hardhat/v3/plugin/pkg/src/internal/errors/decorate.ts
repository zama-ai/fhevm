// Rewrites, in place, the error hardhat 3 throws for a transaction that reverted inside an FHEVM
// contract. Two shapes reach the request hook: EDR's SolidityError (`data` hex, `transactionHash`,
// a `stackTrace` whose last entry names the reverting contract) and a remote node's ProviderError
// (`data` hex, or `{ data, transactionHash, message }`). Both end up with an FHEVM message and, on
// EDR, a stack that names our contracts instead of `<UnrecognizedContract>`.

import { type Address, type Hex, getAddress, isAddress, isHex, toHex } from 'viem';

import type { FhevmContractsRepository } from '../contracts.js';
import { type DecodedFhevmError, decodeFhevmError, decodeFhevmErrorAt } from './decode.js';
import { type FhevmErrorMessages, type TransactionParties, formatFhevmErrorMessages } from './messages.js';

export type RevertData = {
  readonly data: Hex;
  readonly transactionHash?: Hex | undefined;
  /** The contract the EDR stack trace blames, when there is one. */
  readonly revertedAt?: Address | undefined;
};

/** The revert return data carried by a thrown provider error (or the payload ethers nests it in), in either hardhat 3 shape. */
export function extractRevertData(e: unknown): RevertData | undefined {
  if (typeof e !== 'object' || e === null || !('data' in e)) return undefined;
  const payload: unknown = e.data;
  if (typeof payload === 'string') {
    return isHex(payload) && payload !== '0x'
      ? { data: payload, transactionHash: hashOf(e), revertedAt: revertedAt(e) }
      : undefined;
  }
  if (typeof payload !== 'object' || payload === null || !('data' in payload)) return undefined;
  const data: unknown = payload.data;
  if (typeof data !== 'string' || !isHex(data) || data === '0x') return undefined;
  return { data, transactionHash: hashOf(payload) };
}

function hashOf(holder: object): Hex | undefined {
  const hash: unknown =
    'transactionHash' in holder ? holder.transactionHash : 'txHash' in holder ? holder.txHash : undefined;
  return typeof hash === 'string' && isHex(hash) ? hash : undefined;
}

// EDR's stack trace is untyped: the last entry's `address` (bytes or hex) is the reverting contract.
function revertedAt(e: object): Address | undefined {
  if (!('stackTrace' in e) || !Array.isArray(e.stackTrace) || e.stackTrace.length === 0) return undefined;
  const last: unknown = e.stackTrace[e.stackTrace.length - 1];
  if (typeof last !== 'object' || last === null || !('address' in last)) return undefined;
  const raw: unknown = last.address;
  const hex = raw instanceof Uint8Array ? toHex(raw) : typeof raw === 'string' && isHex(raw) ? raw : undefined;
  return hex !== undefined && isAddress(hex) ? getAddress(hex) : undefined;
}

// EDR blames the IMPLEMENTATION behind a proxy, an address the repository does not know; then every ABI is tried.
export function decodeRevert(repository: FhevmContractsRepository, revert: RevertData): DecodedFhevmError | undefined {
  const at =
    revert.revertedAt === undefined ? undefined : decodeFhevmErrorAt(repository, revert.revertedAt, revert.data);
  return at ?? decodeFhevmError(repository, revert.data);
}

/** Decorates `e` when it is an FHEVM revert; returns the messages applied, undefined when it was not ours. */
export function decorateRevertError(
  repository: FhevmContractsRepository,
  e: unknown,
  tx: TransactionParties,
): FhevmErrorMessages | undefined {
  const revert = extractRevertData(e);
  if (revert === undefined || !(e instanceof Error)) return undefined;
  const decoded = decodeRevert(repository, revert);
  if (decoded === undefined) return undefined;

  const messages = formatFhevmErrorMessages(decoded, tx);
  const previous = e.message;
  e.message = messages.message;
  if (typeof e.stack === 'string') e.stack = patchStack(e.stack, previous, messages.message, repository);
  return messages;
}

// The first stack line repeats the message; the rest names our contracts by address only.
function patchStack(stack: string, previous: string, message: string, repository: FhevmContractsRepository): string {
  let patched = stack.startsWith(`Error: ${previous}`)
    ? `Error: ${message}${stack.slice(`Error: ${previous}`.length)}`
    : stack;
  for (const [address, wrapper] of repository.addressToContractMap()) {
    patched = patched.replaceAll(
      new RegExp(`<UnrecognizedContract>\\.<unknown> \\(${address}\\)`, 'gi'),
      `${wrapper.name}.<unknown> (${wrapper.address}, ${wrapper.package}/contracts/${wrapper.name}.sol:0:0)`,
    );
  }
  return patched;
}
