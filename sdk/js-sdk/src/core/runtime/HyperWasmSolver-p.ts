import type { TfheVersion } from '../../wasm/tfhe/TfheApi.js';
import type { TkmsVersion } from '../../wasm/tkms/KmsLibApi.js';
import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../types/fhevmChain.js';
import type { HostContractVersion } from '../types/hostContract.js';
import type { NativeClient, OptionalNativeClient } from '../types/coreFhevmClient.js';
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
};

////////////////////////////////////////////////////////////////////////////////

export async function hyperWasmResolveTfheModuleVersion(parameters: ResolveParameters): Promise<TfheVersion> {
  const moduleVersions = parameters.runtime.config.moduleVersions;
  const tfheChoice = moduleVersions === 'auto' ? 'auto' : moduleVersions?.tfhe;

  if (tfheChoice !== undefined && tfheChoice !== 'auto') {
    return tfheChoice;
  }

  const aclVersion = await _resolveAclVersion(parameters, 'TFHE');
  return isVersionStrictlyBefore(aclVersion, { major: 0, minor: 4 }) ? '1.5.3' : '1.6.1';
}

////////////////////////////////////////////////////////////////////////////////

export async function hyperWasmResolveTkmsModuleVersion(parameters: ResolveParameters): Promise<TkmsVersion> {
  const moduleVersions = parameters.runtime.config.moduleVersions;
  const kmsChoice = moduleVersions === 'auto' ? 'auto' : moduleVersions?.kms;

  if (kmsChoice !== undefined && kmsChoice !== 'auto') {
    return kmsChoice;
  }

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
