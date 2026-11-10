import type { ChecksummedAddress, Uint8Number } from '../types/primitives.js';
import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../types/fhevmChain.js';
import type { Logger } from '../types/logger.js';
import { assertIsChecksummedAddressArray } from '../base/address.js';
import { asUint8Number, isUint8 } from '../base/uint.js';
import { executeWithBatching } from '../base/promise.js';
import { getTrustedClient } from '../runtime/CoreFhevm-p.js';
import { getCoprocessorSignersAbi, getThresholdAbi } from './abi-fragments/fragments.js';
import { keccak_256 } from '@noble/hashes/sha3.js';
import { VersionedValueCache } from '../caching/versionedValueCache.js';
import { bytesToHex } from '../base/bytes.js';
import { getCacheTtl } from '../caching/cacheOptions-p.js';
import { DuplicateSignerError, ThresholdSignerError, UnknownSignerError } from '../errors/SignersError.js';

////////////////////////////////////////////////////////////////////////////////

type Context = {
  readonly runtime: FhevmRuntime;
  readonly chain?: { readonly id: number } | undefined;
  readonly client: NonNullable<object>;
  readonly options: { readonly batchRpcCalls: boolean };
};

type Parameters = {
  readonly address: ChecksummedAddress;
};

type ReturnType = {
  readonly threshold: Uint8Number;
  readonly signers: readonly ChecksummedAddress[];
};

////////////////////////////////////////////////////////////////////////////////

// The current set only: the on-chain check expects it (until coprocessor
// contexts exist).
class CoprocessorSignersCache extends VersionedValueCache<Context, Parameters, ReturnType> {
  protected override readonly itemName = 'coprocessorSigners';

  protected override getLogger(context: Context): Logger | undefined {
    return context.runtime.config.logger;
  }

  // One slot per chain and contract: two chains can deploy at the same address.
  // A client with no chain reads by address only.
  protected override getSlotKey(context: Context, params: Parameters): string {
    return `${context.runtime.uid.toLowerCase()}:${context.chain?.id ?? ''}:${params.address.toLowerCase()}`;
  }

  protected override async fetch(
    context: Context,
    params: Parameters,
  ): Promise<{ readonly version: string; readonly value: ReturnType }> {
    const value = await _getCoprocessorContextSignersAndThreshold(context, params);
    return { version: coprocessorSignersVersion(value), value };
  }

  // A coprocessor signature check failure: the signer set may have changed.
  public override mayBeStale(_context: Context, error: unknown): boolean {
    return (
      error instanceof UnknownSignerError ||
      error instanceof ThresholdSignerError ||
      error instanceof DuplicateSignerError
    );
  }

  // The InputVerifier's signers: the ones input proofs are checked against.
  public override currentParameters(context: Context & { readonly chain: FhevmChain }): Parameters {
    return { address: context.chain.fhevm.contracts.inputVerifier.address as ChecksummedAddress };
  }

  // The whole set (its version is its hash), not cached.
  public override async readCurrentVersion(context: Context, params: Parameters): Promise<string> {
    return coprocessorSignersVersion(await _getCoprocessorContextSignersAndThreshold(context, params));
  }
}

/** @internal The coprocessor signers cache (shared by every client in the thread). */
export const coprocessorSignersCache = new CoprocessorSignersCache();

////////////////////////////////////////////////////////////////////////////////

/**
 * The version of a coprocessor signer set: a hash of the set (in any order)
 * and the threshold, so a changed set never has the version of the old one.
 */
export function coprocessorSignersVersion(value: {
  readonly threshold: number;
  readonly signers: readonly string[];
}): string {
  const signers = value.signers.map((signer) => signer.toLowerCase()).sort();
  return bytesToHex(keccak_256(new TextEncoder().encode(`${value.threshold}:${signers.join(',')}`)));
}

/**
 * Reads the global Coprocessor signers and threshold, from the coprocessor
 * signers cache (the client's `coprocessorSigners` TTL).
 *
 * @param parameters.address - The checksummed address of the InputVerifier contract.
 */
export async function getCoprocessorContextSignersAndThreshold(
  context: Context,
  parameters: Parameters,
): Promise<ReturnType> {
  const entry = await coprocessorSignersCache.get(context, parameters, getCacheTtl(context, 'coprocessorSigners'));
  return entry.value;
}

////////////////////////////////////////////////////////////////////////////////

async function _getCoprocessorContextSignersAndThreshold(
  context: Context,
  parameters: Parameters,
): Promise<ReturnType> {
  ////////////////////////////////////////////////////////////////////////////
  //
  // Important remark:
  // =================
  // Do NOTE USE `Promise.all` here!
  // You may get a server response 500 Internal Server Error
  // "Batch of more than 3 requests are not allowed on free tier, to use this
  // feature register paid account at drpc.org"
  //
  ////////////////////////////////////////////////////////////////////////////

  const rpcCalls = [() => _getThreshold(context, parameters), () => _getCoprocessorSigners(context, parameters)];

  const res = await executeWithBatching<unknown>(rpcCalls, context.options.batchRpcCalls);

  const threshold = res[0];
  const coprocessorSigners = res[1];

  if (!isUint8(threshold)) {
    throw new Error(`Invalid InputVerifier coprocessor signers threshold.`);
  }

  try {
    assertIsChecksummedAddressArray(coprocessorSigners, {});
  } catch (e) {
    throw new Error(`Invalid InputVerifier coprocessor signers addresses.`, {
      cause: e,
    });
  }

  return Object.freeze({
    threshold: asUint8Number(Number(threshold)),
    signers: coprocessorSigners,
  });
}

////////////////////////////////////////////////////////////////////////////////

async function _getThreshold(context: Context, parameters: Parameters): Promise<Uint8Number> {
  const trustedClient = getTrustedClient(context);
  const address = parameters.address;

  const res = await context.runtime.ethereum.readContract(trustedClient, {
    address: address,
    abi: getThresholdAbi,
    args: [],
    functionName: getThresholdAbi[0].name,
  });

  if (!isUint8(res)) {
    throw new Error(`Invalid threshold.`);
  }

  return Number(res) as Uint8Number;
}

////////////////////////////////////////////////////////////////////////////////

async function _getCoprocessorSigners(context: Context, parameters: Parameters): Promise<ChecksummedAddress[]> {
  const trustedClient = getTrustedClient(context);
  const address = parameters.address;

  const res = await context.runtime.ethereum.readContract(trustedClient, {
    address: address,
    abi: getCoprocessorSignersAbi,
    args: [],
    functionName: getCoprocessorSignersAbi[0].name,
  });

  try {
    assertIsChecksummedAddressArray(res, {});
  } catch (e) {
    throw new Error(`Invalid coprocessor signers addresses.`, {
      cause: e,
    });
  }

  return res;
}
