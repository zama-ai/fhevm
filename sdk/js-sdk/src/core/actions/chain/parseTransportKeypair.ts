import type { Fhevm, OptionalNativeClient } from '../../types/coreFhevmClient.js';
import type { FhevmRuntime } from '../../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import { toTransportKeypair, type TransportKeypair } from '../../kms/TransportKeypair-p.js';

////////////////////////////////////////////////////////////////////////////////

export type ParseTransportKeypairParameters = {
  /** The serialized keypair — output of `serializeE2eTransportKeypair` or a previously parsed object. */
  readonly serialized: string | Record<string, unknown>;
};

export type ParseTransportKeypairReturnType = TransportKeypair;

export async function parseTransportKeypair(
  fhevm: Fhevm<FhevmChain, FhevmRuntime, OptionalNativeClient>,
  parameters: ParseTransportKeypairParameters,
): Promise<ParseTransportKeypairReturnType> {
  const parsed =
    typeof parameters.serialized === 'string' ? (JSON.parse(parameters.serialized) as unknown) : parameters.serialized;
  return toTransportKeypair(fhevm, parsed);
}

////////////////////////////////////////////////////////////////////////////////
