import type { Fhevm } from '../../core/types/coreFhevmClient.js';
import type { FhevmRuntime } from '../../core/types/coreFhevmRuntime.js';
import type { Address, FetchAccountConfig } from '@solana/kit';
import { getAddressDecoder } from '@solana/kit';
import type { FhevmSolanaChain } from '../../core/types/fhevmSolanaChain.js';
import type { FheEncryptionKeyBytes } from '../../core/types/fheEncryptionKey.js';
import { hexToBytes32 } from '../../core/base/bytes.js';
import { PRIVATE_SOLANA_TOKEN } from '../internal/solana-p.js';
import { createCoreFhevm } from '../../core/runtime/CoreFhevm-p.js';
import { getSolanaRuntime } from '../internal/runtime.js';
import { assertValidSolanaChainId } from '../../core/chains/utilsSolana.js';
import { fetchSolanaEncryptedStore, type SolanaRpc } from '../encryptedStore.js';
import {
  fetchSolanaUserDecryptionDelegation,
  type SolanaUserDecryptionDelegationTuple,
} from '../actions/userDecryptionDelegation.js';
import { fetchSolanaPermitInvalidation } from '../actions/revokePermits.js';

export type SolanaClientParameters<C extends FhevmSolanaChain = FhevmSolanaChain> = {
  readonly chain: C;
  readonly rpc: SolanaRpc;
};

export type SolanaEncryptOptions = {
  readonly fheEncryptionKey?: FheEncryptionKeyBytes | undefined;
};

// Core initialization and WASM ownership remain shared; EVM-only members stay private.
export function createSolanaCore<C extends FhevmSolanaChain>(
  parameters: SolanaClientParameters<C> & { readonly options?: SolanaEncryptOptions | undefined },
): Fhevm<undefined, FhevmRuntime, undefined> {
  assertValidSolanaChainId(parameters.chain.id);
  if (parameters.options !== undefined && Object.keys(parameters.options).some((key) => key !== 'fheEncryptionKey')) {
    throw new Error('Unsupported Solana client option');
  }
  const key = parameters.options?.fheEncryptionKey;
  if (key !== undefined && key.metadata.relayerUrl !== parameters.chain.fhevm.relayerUrl) {
    throw new Error('Encryption key relayer URL does not match the Solana chain');
  }
  return createCoreFhevm(PRIVATE_SOLANA_TOKEN, {
    runtime: getSolanaRuntime(),
    options: parameters.options,
  });
}

/** The chain's zama-host program id as a base58 address, for PDA derivation and account reads. */
export function solanaHostProgram(chain: FhevmSolanaChain): Address {
  return getAddressDecoder().decode(hexToBytes32(chain.fhevm.programs.host.address));
}

export function solanaClientSurface<C extends FhevmSolanaChain>(
  core: ReturnType<typeof createSolanaCore>,
  parameters: SolanaClientParameters<C>,
): FhevmSolanaBaseClient<C> {
  const { chain, rpc } = parameters;
  const programAddress = solanaHostProgram(chain);
  return {
    uid: core.uid,
    chain,
    rpc,
    init: () => core.init(),
    get ready() {
      return core.ready;
    },
    fetchEncryptedStore: (address: Address, config?: FetchAccountConfig) =>
      fetchSolanaEncryptedStore(rpc, address, config, programAddress),
    fetchUserDecryptionDelegation: (tuple: SolanaUserDecryptionDelegationTuple, config?: FetchAccountConfig) =>
      fetchSolanaUserDecryptionDelegation(rpc, tuple, { ...config, programAddress }),
    fetchPermitInvalidation: (user: Address, config?: FetchAccountConfig) =>
      fetchSolanaPermitInvalidation(rpc, user, { ...config, programAddress }),
  };
}

export type FhevmSolanaBaseClient<C extends FhevmSolanaChain = FhevmSolanaChain> = {
  readonly uid: string;
  readonly chain: C;
  readonly rpc: SolanaRpc;
  readonly init: () => Promise<void>;
  readonly ready: Promise<void>;
  readonly fetchEncryptedStore: (
    address: Address,
    config?: FetchAccountConfig,
  ) => ReturnType<typeof fetchSolanaEncryptedStore>;
  readonly fetchUserDecryptionDelegation: (
    tuple: SolanaUserDecryptionDelegationTuple,
    config?: FetchAccountConfig,
  ) => ReturnType<typeof fetchSolanaUserDecryptionDelegation>;
  readonly fetchPermitInvalidation: (user: Address, config?: FetchAccountConfig) => Promise<bigint>;
};

/** Creates a Solana client whose chain reads share the supplied native Kit RPC. */
export function createFhevmBaseClient<C extends FhevmSolanaChain>(
  parameters: SolanaClientParameters<C>,
): FhevmSolanaBaseClient<C> {
  return solanaClientSurface(createSolanaCore(parameters), parameters);
}
