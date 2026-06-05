import type { TfheVersion, TkmsVersion } from '../types/moduleVersions.js';
import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../types/fhevmChain.js';
import type { HostContractVersion } from '../types/hostContract.js';
import type { NativeClient, OptionalNativeClient, ResolvedFhevmOptions } from '../types/coreFhevmClient.js';
import { asAddress, addressToChecksummedAddress } from '../base/address.js';
import {
  assertIsHostContractVersionOf,
  getVersion,
  isVersionStrictlyBefore,
} from '../host-contracts/HostContractVersion-p.js';

////////////////////////////////////////////////////////////////////////////////

type ResolveParameters = {
  readonly runtime: FhevmRuntime;
  readonly chain: FhevmChain;
  readonly client?: OptionalNativeClient;
  readonly options: ResolvedFhevmOptions;
};

////////////////////////////////////////////////////////////////////////////////

export async function hyperWasmResolveTfheModuleVersion(parameters: ResolveParameters): Promise<TfheVersion> {
  const clientVersions = parameters.options.moduleVersions;
  if (clientVersions === 'auto') {
    return _autoResolveTfheModuleVersion(parameters);
  }

  if (clientVersions?.tfhe !== undefined) {
    return clientVersions.tfhe;
  }

  const runtimeVersions = parameters.runtime.config.moduleVersions;
  if (runtimeVersions !== 'auto' && runtimeVersions?.tfhe !== undefined) {
    return runtimeVersions.tfhe;
  }

  return _autoResolveTfheModuleVersion(parameters);
}

async function _autoResolveTfheModuleVersion(parameters: ResolveParameters): Promise<TfheVersion> {
  const aclVersion = await _resolveAclVersion(parameters, 'TFHE');
  return isVersionStrictlyBefore(aclVersion, { major: 0, minor: 4 }) ? '1.5.3' : '1.6.1';
}

////////////////////////////////////////////////////////////////////////////////

export async function hyperWasmResolveTkmsModuleVersion(parameters: ResolveParameters): Promise<TkmsVersion> {
  const clientVersions = parameters.options.moduleVersions;
  if (clientVersions === 'auto') {
    return _autoResolveTkmsModuleVersion(parameters);
  }

  if (clientVersions?.kms !== undefined) {
    return clientVersions.kms;
  }

  const runtimeVersions = parameters.runtime.config.moduleVersions;
  if (runtimeVersions !== 'auto' && runtimeVersions?.kms !== undefined) {
    return runtimeVersions.kms;
  }

  return _autoResolveTkmsModuleVersion(parameters);
}

async function _autoResolveTkmsModuleVersion(parameters: ResolveParameters): Promise<TkmsVersion> {
  const aclVersion = await _resolveAclVersion(parameters, 'TKMS');
  return isVersionStrictlyBefore(aclVersion, { major: 0, minor: 4 }) ? '0.13.10' : '0.13.20-0';
}

////////////////////////////////////////////////////////////////////////////////
// Shared ACL version probe. No caching layer here — `getVersion` already
// caches results for 24h (see HostContractVersion-p.ts), and a `forceRefresh`
// at that layer correctly invalidates downstream callers.

async function _resolveAclVersion(parameters: ResolveParameters, label: 'TFHE' | 'TKMS'): Promise<HostContractVersion> {
  if (parameters.client === undefined) {
    throw new Error(`Cannot auto-resolve ${label} WASM version without a native client.`);
  }

  // Identity preservation: `parameters` is a CoreFhevmImpl from the caller,
  // and downstream `getVersion` -> `getTrustedClient` does an `instanceof
  // CoreFhevmImpl` check. Don't build a new object — narrow the type instead.
  const context = parameters as ResolveParameters & { readonly client: NativeClient };

  const aclAddress = addressToChecksummedAddress(asAddress(context.chain.fhevm.contracts.acl.address));
  const aclVersion = await getVersion(context, { address: aclAddress });

  assertIsHostContractVersionOf(aclVersion, 'ACL');

  return aclVersion;
}
