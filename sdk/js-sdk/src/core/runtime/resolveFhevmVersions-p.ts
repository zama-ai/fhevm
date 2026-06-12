import type { FhevmProtocolContext, ProtocolVersionResolution } from '../types/coreFhevmClient.js';
import type { TfheVersion, TkmsVersion } from '../types/moduleVersions.js';
import {
  asCoreClientFhevm,
  assertIsFhevmBaseClient,
  getResolvedProtocolVersion,
  getResolvedTfheVersion,
  getResolvedTkmsVersion,
  setResolvedProtocolVersion,
} from './CoreFhevm-p.js';
import { hyperWasmResolveTfheModuleVersion, hyperWasmResolveTkmsModuleVersion } from './HyperWasmSolver-p.js';
import { pubKeyCrsVersionFromProtocolVersion, resolveProtocolContext } from './ProtocolVersionResolver-p.js';

////////////////////////////////////////////////////////////////////////////////

/**
 * Resolves this client's FHEVM protocol version once and memoizes it on the
 * client instance.
 *
 * Idempotent: the first call resolves the protocol version from the on-chain ACL
 * contract version and stores it on the client; subsequent calls reuse the stored
 * value, it is safe to call from multiple init paths and from hot paths.
 *
 * This is the canonical way to ensure {@link getResolvedProtocolVersion} is
 * populated.
 *
 * @param fhevm - A core FHEVM base client.
 * @returns The resolved protocol version (exact or SDK-bounded).
 */
export async function ensureResolvedProtocolVersion(fhevm: unknown): Promise<ProtocolVersionResolution> {
  const f = asCoreClientFhevm(fhevm);

  const protocolVersion = await resolveFhevmProtocolVersion(f);
  setResolvedProtocolVersion(f, protocolVersion);

  return protocolVersion;
}

////////////////////////////////////////////////////////////////////////////////

export async function resolveFhevmProtocolVersion(fhevm: unknown): Promise<ProtocolVersionResolution> {
  return (await _resolveFhevmProtocolContext(fhevm)).protocolVersion;
}

////////////////////////////////////////////////////////////////////////////////

async function _resolveFhevmProtocolContext(fhevm: unknown): Promise<FhevmProtocolContext> {
  assertIsFhevmBaseClient(fhevm);

  const protocolVersion = getResolvedProtocolVersion(fhevm);
  if (protocolVersion !== undefined) {
    return Object.freeze({
      protocolVersion,
      pubKeyCrsVersion: pubKeyCrsVersionFromProtocolVersion(fhevm.chain, protocolVersion),
    });
  }

  return resolveProtocolContext(fhevm);
}

////////////////////////////////////////////////////////////////////////////////

export async function resolveFhevmTfheVersion(fhevm: unknown): Promise<TfheVersion> {
  const protocolContext = await _resolveFhevmProtocolContext(fhevm);
  const tfheVersion = getResolvedTfheVersion(fhevm);
  if (tfheVersion !== undefined) {
    return tfheVersion;
  }
  assertIsFhevmBaseClient(fhevm);
  return hyperWasmResolveTfheModuleVersion(fhevm, protocolContext);
}

////////////////////////////////////////////////////////////////////////////////

export async function resolveFhevmTkmsVersion(fhevm: unknown): Promise<TkmsVersion> {
  const protocolContext = await _resolveFhevmProtocolContext(fhevm);
  const tkmsVersion = getResolvedTkmsVersion(fhevm);
  if (tkmsVersion !== undefined) {
    return tkmsVersion;
  }
  assertIsFhevmBaseClient(fhevm);
  return hyperWasmResolveTkmsModuleVersion(fhevm, protocolContext);
}

////////////////////////////////////////////////////////////////////////////////
