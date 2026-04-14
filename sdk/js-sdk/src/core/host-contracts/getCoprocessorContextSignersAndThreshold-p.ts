import type { ChecksummedAddress, Uint8Number } from '../types/primitives.js';
import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import { assertIsChecksummedAddressArray } from '../base/address.js';
import { asUint8Number, isUint8 } from '../base/uint.js';
import { executeWithBatching } from '../base/promise.js';
import { getTrustedClient } from '../runtime/CoreFhevm-p.js';
import { getCoprocessorSignersAbi, getThresholdAbi } from './abi-fragments/fragments.js';
import { CACHE_TTL_24H, createCachedFetch } from '../base/cachedFetch.js';

////////////////////////////////////////////////////////////////////////////////

type Context = {
  readonly runtime: FhevmRuntime;
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

const cachedGetCoprocessorContextSignersAndThreshold = createCachedFetch<Context, Parameters, ReturnType>({
  executeFn: _getCoprocessorContextSignersAndThreshold,
  cacheKeyFn: (context, params) => `${context.runtime.uid.toLowerCase()}:${params.address.toLowerCase()}`,
  // Signers are not Use long TTL
  ttlMs: CACHE_TTL_24H,
});

////////////////////////////////////////////////////////////////////////////////

/**
 * Reads the global Coprocessor signers and threshold.
 *
 * @param parameters.address - The checksummed address of the InputVerifier contract.
 * @param parameters.forceRefresh - If `true`, invalidates the cached entry and
 *   makes a fresh RPC call. The new result is stored back in the cache.
 */
export function getCoprocessorContextSignersAndThreshold(
  context: Context,
  parameters: Parameters & { readonly forceRefresh?: boolean },
): Promise<ReturnType> {
  return cachedGetCoprocessorContextSignersAndThreshold.execute(context, parameters);
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
