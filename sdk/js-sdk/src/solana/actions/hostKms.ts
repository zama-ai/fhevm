/** Public API surface: the Solana decrypt clients, and the SDK's tests through `clearSolanaHostKmsReads`. */
import { fetchEncodedAccount, fetchEncodedAccounts, type Address, type MaybeEncodedAccount } from '@solana/kit';
import {
  findHostConfigPda,
  findKmsContextPda,
  getHostConfigDecoder,
  getKmsContextDecoder,
  HOST_CONFIG_DISCRIMINATOR,
  KMS_CONTEXT_DISCRIMINATOR,
  type HostConfig,
  type KmsContext,
} from '@fhevm/solana-zama-host';
import { bytesToHex, unsafeBytesEquals } from '../../core/base/bytes.js';
import { CACHE_TTL_15MIN, createCachedFetch } from '../../core/base/cachedFetch.js';
import { createKmsEip712Domain } from '../../core/kms/createKmsEip712Domain.js';
import type { FhevmSolanaChain } from '../../core/types/fhevmSolanaChain.js';
import type { FhevmRuntime } from '../../core/types/coreFhevmRuntime.js';
import type { SolanaUserDecryptVerification } from '../userDecrypt/execute.js';
import { solanaHostProgram, type SolanaClientParameters } from '../clients/createFhevmBaseClient.js';
import { hostAccountData, publicDecryptAbortCheck } from './publicDecryptCertificate.js';

/**
 * The KMS trust a client reads from the host program: `HostConfig` and the `KmsContext` accounts,
 * at finalized.
 *
 * Both reads are cached for 15 minutes, as the EVM SDK caches its `ProtocolConfig` and
 * `KMSVerifier` reads; concurrent callers share one request. A failed read is not cached, so a
 * destroyed context stops verifying within that window, as a revoked EVM context does.
 */
export type SolanaHostKmsReads = {
  readonly config: () => Promise<HostConfig>;
  /** The live context `contextId` names; a missing or destroyed one throws. */
  readonly kmsContext: (contextId: Uint8Array) => Promise<KmsContext>;
};

type HostReadContext = { readonly client: SolanaClientParameters; readonly runtime: FhevmRuntime };

// Keyed like the EVM caches, by runtime and program, plus the chain id: a Solana program id can
// repeat across clusters, and a Solana chain id names the cluster (DD-052).
const hostKey = ({ client, runtime }: HostReadContext): string =>
  `${runtime.uid}:${solanaHostProgram(client.chain)}:${client.chain.id}`;

const cachedHostConfig = createCachedFetch<HostReadContext, Record<never, never>, HostConfig>({
  executeFn: async ({ client }) => {
    const programAddress = solanaHostProgram(client.chain);
    const [address, bump] = await findHostConfigPda({ programAddress });
    const account = await fetchEncodedAccount(client.rpc, address, { commitment: 'finalized' });
    return clientHostConfig(account, programAddress, bump, client.chain);
  },
  cacheKeyFn: hostKey,
  ttlMs: CACHE_TTL_15MIN,
});

const cachedKmsContext = createCachedFetch<HostReadContext, { readonly contextId: Uint8Array }, KmsContext>({
  executeFn: async ({ client }, { contextId }) => {
    const programAddress = solanaHostProgram(client.chain);
    const [address, bump] = await findKmsContextPda({ contextId }, { programAddress });
    const [account] = await fetchEncodedAccounts(client.rpc, [address], { commitment: 'finalized' });
    return liveKmsContext(account, programAddress, contextId, bump);
  },
  cacheKeyFn: (context, { contextId }) => `${hostKey(context)}:${bytesToHex(contextId)}`,
  ttlMs: CACHE_TTL_15MIN,
});

/** The host KMS reads of a client on `runtime`, through the shared cache. */
export function createSolanaHostKmsReads(client: SolanaClientParameters, runtime: FhevmRuntime): SolanaHostKmsReads {
  const context = { client, runtime };
  return {
    config: () => cachedHostConfig.execute(context, {}),
    kmsContext: (contextId) => cachedKmsContext.execute(context, { contextId }),
  };
}

/** Empties the host KMS read cache; a test boundary. */
export function clearSolanaHostKmsReads(): void {
  cachedHostConfig.clear({ includeInflight: true });
  cachedKmsContext.clear({ includeInflight: true });
}

/**
 * The `HostConfig` in `account`, if it is the one at `bump` and records the client's chain. A
 * client whose RPC reaches another cluster fails here instead of caching that cluster's config.
 */
export function clientHostConfig(
  account: MaybeEncodedAccount | undefined,
  programAddress: Address,
  bump: number,
  chain: FhevmSolanaChain,
): HostConfig {
  const config = getHostConfigDecoder().decode(hostAccountData(account, programAddress, HOST_CONFIG_DISCRIMINATOR));
  if (config.bump !== bump || config.chainId !== chain.id)
    throw new Error('Host configuration does not match the client');
  return config;
}

/** The `KmsContext` in `account`, if it is the live context `contextId` names at `bump`. */
export function liveKmsContext(
  account: MaybeEncodedAccount | undefined,
  programAddress: Address,
  contextId: Uint8Array,
  bump: number,
): KmsContext {
  const kms = getKmsContextDecoder().decode(hostAccountData(account, programAddress, KMS_CONTEXT_DISCRIMINATOR));
  if (kms.bump !== bump || kms.destroyed || !unsafeBytesEquals(new Uint8Array(kms.contextId), contextId))
    throw new Error('Invalid or destroyed KMS context');
  return kms;
}

/**
 * The host's active KMS context and epoch. A decryption routes to this pair, as an EVM one routes to
 * `ProtocolConfig.getCurrentKmsContextAndEpoch()`.
 */
export async function readActiveKmsRouting(
  chain: FhevmSolanaChain,
  host: SolanaHostKmsReads,
  abortSignal?: AbortSignal,
): Promise<{ readonly contextId: Uint8Array; readonly epochId: Uint8Array }> {
  // A shared read cannot carry one caller's signal, so cancellation is checked around it.
  const checkAbort = publicDecryptAbortCheck(chain, abortSignal);
  checkAbort();
  const config = await host.config();
  checkAbort();
  const contextId = new Uint8Array(config.currentKmsContextId);
  const epochId = new Uint8Array(config.currentKmsEpochId);
  if (contextId.every((byte) => byte === 0) || epochId.every((byte) => byte === 0))
    throw new Error('KMS context is not configured');
  return { contextId, epochId };
}

/**
 * What user-decrypt response verification trusts for a permit: the signers of the KMS context the
 * permit names, as parties `1..n` in their registered order (the EVM SDK numbers `KMSVerifier`'s
 * signers the same way), and the gateway domain the host verifies certificates under.
 *
 * The context is the permit's, not the host's current one: a permit stays answerable after a
 * context switch until its context is destroyed, as on EVM.
 */
export async function userDecryptVerification(
  host: SolanaHostKmsReads,
  kmsContextId: Uint8Array,
  fheParameter: string,
): Promise<SolanaUserDecryptVerification> {
  const [config, kms] = await Promise.all([host.config(), host.kmsContext(kmsContextId)]);
  return {
    signers: kms.signers.map((signer, index) => ({ partyId: index + 1, address: bytesToHex(new Uint8Array(signer)) })),
    fheParameter,
    gatewayEip712Domain: createKmsEip712Domain({
      chainId: config.gatewayChainId,
      verifyingContractAddressDecryption: bytesToHex(new Uint8Array(config.decryptionContract)),
    }),
  };
}
