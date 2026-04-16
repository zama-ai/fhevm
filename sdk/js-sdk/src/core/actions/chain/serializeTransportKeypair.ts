import type { Fhevm, OptionalNativeClient } from '../../types/coreFhevmClient.js';
import type { FhevmRuntime } from '../../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { BytesHex } from '../../types/primitives.js';
import {
  serializeTransportKeypair as serializeTransportKeypair_,
  type TransportKeypair,
} from '../../kms/TransportKeypair-p.js';

////////////////////////////////////////////////////////////////////////////////

export type SerializeTransportKeypairParameters = {
  readonly transportKeypair: TransportKeypair;
};

export type SerializeTransportKeypairReturnType = {
  publicKey: BytesHex;
  privateKey: BytesHex;
};

export function serializeTransportKeypair(
  _fhevm: Fhevm<FhevmChain, FhevmRuntime, OptionalNativeClient>,
  parameters: SerializeTransportKeypairParameters,
): SerializeTransportKeypairReturnType {
  return serializeTransportKeypair_(parameters.transportKeypair);
}

////////////////////////////////////////////////////////////////////////////////
