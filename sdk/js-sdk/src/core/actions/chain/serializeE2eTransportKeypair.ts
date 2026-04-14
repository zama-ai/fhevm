import type { Fhevm, OptionalNativeClient } from '../../types/coreFhevmClient.js';
import type { FhevmRuntime } from '../../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { BytesHex } from '../../types/primitives.js';
import {
  serializeE2eTransportKeypair as serializeE2eTransportKeypair_,
  type E2eTransportKeypair,
} from '../../kms/E2eTransportKeypair-p.js';

////////////////////////////////////////////////////////////////////////////////

export type SerializeE2eTransportKeypairParameters = {
  readonly e2eTransportKeypair: E2eTransportKeypair;
};

export type SerializeE2eTransportKeypairReturnType = {
  publicKey: BytesHex;
  privateKey: BytesHex;
};

export function serializeE2eTransportKeypair(
  _fhevm: Fhevm<FhevmChain, FhevmRuntime, OptionalNativeClient>,
  parameters: SerializeE2eTransportKeypairParameters,
): SerializeE2eTransportKeypairReturnType {
  return serializeE2eTransportKeypair_(parameters.e2eTransportKeypair);
}

////////////////////////////////////////////////////////////////////////////////
