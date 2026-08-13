import type { Fhevm, OptionalNativeClient } from '../../types/coreFhevmClient.js';
import type { FhevmRuntime } from '../../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { BytesHex } from '../../types/primitives.js';
import {
  serializeTransportKeyPair as serializeTransportKeyPair_,
  type TransportKeyPair,
} from '../../kms/TransportKeyPair-p.js';
import { initPublicAction } from '../../runtime/CoreFhevm-p.js';

////////////////////////////////////////////////////////////////////////////////

export type SerializeTransportKeyPairParameters = {
  readonly transportKeyPair: TransportKeyPair;
};

export type SerializeTransportKeyPairReturnType = {
  publicKey: BytesHex;
  privateKey: BytesHex;
  tkmsVersion?: string;
};

export async function serializeTransportKeyPair(
  fhevm: Fhevm<FhevmChain, FhevmRuntime, OptionalNativeClient>,
  parameters: SerializeTransportKeyPairParameters,
): Promise<SerializeTransportKeyPairReturnType> {
  const fhevmContext = await initPublicAction(fhevm);
  return serializeTransportKeyPair_({ transportKeyPair: parameters.transportKeyPair, fhevmContext });
}
