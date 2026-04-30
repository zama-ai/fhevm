import type { Fhevm, OptionalNativeClient } from '../../types/coreFhevmClient.js';
import type { FhevmRuntime } from '../../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { BytesHex } from '../../types/primitives.js';
import {
  serializeTransportKeyPair as serializeTransportKeyPair_,
  type TransportKeyPair,
} from '../../kms/TransportKeyPair-p.js';

////////////////////////////////////////////////////////////////////////////////

export type SerializeTransportKeyPairParameters = {
  readonly transportKeyPair: TransportKeyPair;
};

export type SerializeTransportKeyPairReturnType = {
  publicKey: BytesHex;
  privateKey: BytesHex;
};

export function serializeTransportKeyPair(
  _fhevm: Fhevm<FhevmChain, FhevmRuntime, OptionalNativeClient>,
  parameters: SerializeTransportKeyPairParameters,
): SerializeTransportKeyPairReturnType {
  return serializeTransportKeyPair_(parameters.transportKeyPair);
}
