import type {
  Fhevm,
  OptionalNativeClient,
} from '../../types/coreFhevmClient.js';
import type { FhevmRuntime } from '../../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import {
  toE2eTransportKeypair,
  type E2eTransportKeypair,
} from '../../kms/E2eTransportKeypair-p.js';

////////////////////////////////////////////////////////////////////////////////

export type ParseE2eTransportKeypairParameters = {
  /** The serialized keypair — output of `serializeE2eTransportKeypair` or a previously parsed object. */
  readonly serialized: string | Record<string, unknown>;
};

export type ParseE2eTransportKeypairReturnType = E2eTransportKeypair;

export async function parseE2eTransportKeypair(
  fhevm: Fhevm<FhevmChain, FhevmRuntime, OptionalNativeClient>,
  parameters: ParseE2eTransportKeypairParameters,
): Promise<ParseE2eTransportKeypairReturnType> {
  const parsed =
    typeof parameters.serialized === 'string'
      ? (JSON.parse(parameters.serialized) as unknown)
      : parameters.serialized;
  return toE2eTransportKeypair(fhevm, parsed);
}

////////////////////////////////////////////////////////////////////////////////
