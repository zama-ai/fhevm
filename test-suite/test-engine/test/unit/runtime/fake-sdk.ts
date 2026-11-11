/** A fake `@fhevm/sdk` and the profile sections the FHE unit tests share. */
import { JsonRpcProvider } from 'ethers';

import type { ResolvedContractsSection, ResolvedRelayerSection } from '../../../src/config/profiles/types.js';
import type { FheClientConfig, SdkBindings, SdkClientLike } from '../../../src/runtime/modules/fhe/client.js';

export const API_KEY = 'relayer-api-key-0123456789';
const literal = <T extends string | number>(value: T) => ({ value, secret: false, source: 'literal' });
export const address = (n: number) => `0x${String(n).padStart(40, '0')}`;

export const relayer = (apiKey?: string): ResolvedRelayerSection => ({
  url: literal('http://fhevm-relayer:3000/v2'),
  ...(apiKey !== undefined ? { apiKey: { value: apiKey, secret: true, source: 'env:KEY' } } : {}),
  gateway: { chainId: literal(54321), decryption: literal(address(1)), inputVerification: literal(address(2)) },
});
export const contracts: ResolvedContractsSection = {
  acl: literal(address(3)),
  inputVerifier: literal(address(4)),
  kmsVerifier: literal(address(5)),
  protocolConfig: literal(address(6)),
  executor: literal(address(7)),
};
export const config = (apiKey?: string): FheClientConfig => ({ relayer: relayer(apiKey), contracts, chainId: 12345 });

export const provider = new JsonRpcProvider('http://127.0.0.1:1', 12345, { staticNetwork: true });

export type RelayerOptions = { auth?: unknown; timeout?: number; onProgress?: (event: object) => void };
export type Call = { method: string; parameters: Record<string, unknown> };

export const SUCCESS_EVENTS = [
  { type: 'queued', operation: 'INPUT_PROOF', jobId: 'job-1', requestId: 'req-1', status: 202 },
  { type: 'succeeded', operation: 'INPUT_PROOF', jobId: 'job-1', requestId: 'req-1', status: 200 },
];

export interface FakeBehaviour {
  /** Progress events replayed on every relayer call (default: queued then succeeded). */
  events?: object[];
  /** Error thrown by every relayer call, after the events. */
  fail?: Error;
  /** Whether the relayer offers the unified route (default true). */
  unified?: boolean;
  /** Type and value of every decrypted value (default uint64 49n). */
  clear?: { type: string; value: boolean | bigint | number | string };
  /** Error thrown when the client gets ready. */
  readyError?: Error;
}

/**
 * A fake SDK: records every call and replays the events through `onProgress` before answering.
 * Keypairs and permits are numbered objects (`{ id }`), so tests can tell them apart.
 */
export function fakeSdk(behaviour: FakeBehaviour = {}) {
  const calls: Call[] = [];
  const runtimeConfigs: Record<string, unknown>[] = [];
  let chain: Record<string, unknown> | undefined;
  let nextId = 0;
  const relayerCall =
    (method: string, answer: (parameters: Record<string, unknown>) => unknown) =>
    async (parameters: Record<string, unknown>) => {
      calls.push({ method, parameters });
      const options = parameters.options as RelayerOptions | undefined;
      const events = behaviour.events ?? SUCCESS_EVENTS;
      // Like the SDK, progress is reported in a microtask queued before the call settles.
      queueMicrotask(() => {
        for (const event of events) options?.onProgress?.(event);
      });
      if (behaviour.fail) throw behaviour.fail;
      return answer(parameters);
    };
  const plainCall = (method: string, answer: () => object) => async (parameters?: Record<string, unknown>) => {
    calls.push({ method, parameters: parameters ?? {} });
    return answer();
  };
  const clear = behaviour.clear ?? { type: 'uint64', value: 49n };
  const client = {
    ready: behaviour.readyError ? Promise.reject(behaviour.readyError) : Promise.resolve(),
    encryptValues: relayerCall('encryptValues', (parameters) => ({
      encryptedValues: (parameters.values as unknown[]).map((_, index) => `0x${'0a'.repeat(31)}0${index}`),
      inputProof: '0x0102030405',
    })),
    decryptValues: relayerCall('decryptValues', (parameters) =>
      (parameters.encryptedValues as unknown[]).map(() => clear),
    ),
    decryptPublicValuesWithSignatures: relayerCall('decryptPublicValuesWithSignatures', (parameters) => ({
      clearValues: (parameters.encryptedValues as unknown[]).map(() => clear),
      checkSignaturesArgs: {
        handlesList: parameters.encryptedValues,
        abiEncodedCleartexts: '0xabi',
        decryptionProof: '0x0d0e',
      },
    })),
    generateTransportKeyPair: plainCall('generateTransportKeyPair', () => ({ id: ++nextId })),
    parseTransportKeyPair: plainCall('parseTransportKeyPair', () => ({ id: ++nextId, parsed: true })),
    signLegacyDecryptionPermit: plainCall('signLegacyDecryptionPermit', () => ({ id: ++nextId, version: 1 })),
    signUnifiedDecryptionPermit: plainCall('signUnifiedDecryptionPermit', () => ({ id: ++nextId, version: 2 })),
  } as unknown as SdkClientLike;
  // An unhandled rejection would fail the test run before the client awaits it.
  (client.ready as Promise<void>).catch(() => undefined);
  const sdk: SdkBindings = {
    hasRuntimeConfig: () => runtimeConfigs.length > 0,
    setRuntimeConfig: (runtimeConfig) => void runtimeConfigs.push(runtimeConfig as Record<string, unknown>),
    createClient: (parameters) => {
      chain = parameters.chain as unknown as Record<string, unknown>;
      return client;
    },
    canUseUnifiedPermit: async (_client, options) => {
      calls.push({ method: 'canUseUnifiedPermit', parameters: options });
      return behaviour.unified ?? true;
    },
  };
  return { sdk, calls, runtimeConfigs, chain: () => chain! };
}
