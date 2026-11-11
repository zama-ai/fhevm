import type { Interface, Provider } from 'ethers';
import { types } from 'hardhat/config';
import type { ConfigurableTaskDefinition, HardhatRuntimeEnvironment } from 'hardhat/types';

import type { CoprocessorConfigReplica } from '../../types';
import { getContractInterface } from './contractInterface';
import { loadHostAddresses } from './loadVariables';

// Shared plumbing for the CoprocessorConfig and CoprocessorConfigReplica tasks: ABI loading, target
// address resolution and the common CLI parameters.

export const COPROCESSOR_CONFIG_ADDRESS_ENV_VAR = 'COPROCESSOR_CONFIG_CONTRACT_ADDRESS';

export interface EncodedCall {
  functionSignature: string;
  calldata: string;
  decodedArgs: unknown[];
}

export async function getCoprocessorConfigInterface(hre: HardhatRuntimeEnvironment): Promise<Interface> {
  return getContractInterface(hre, 'CoprocessorConfig');
}

export async function getCoprocessorConfigReplicaInterface(hre: HardhatRuntimeEnvironment): Promise<Interface> {
  return getContractInterface(hre, 'CoprocessorConfigReplica');
}

export async function getCoprocessorConfigReplica(
  hre: HardhatRuntimeEnvironment,
  address: string,
): Promise<CoprocessorConfigReplica> {
  return (await hre.ethers.getContractAt('CoprocessorConfigReplica', address)) as unknown as CoprocessorConfigReplica;
}

// Without a response cache, so the block-hash recheck that closes every canonical read reaches the RPC.
export function getCanonicalProvider(hre: HardhatRuntimeEnvironment, canonicalRpcUrl: string): Provider {
  return new hre.ethers.JsonRpcProvider(canonicalRpcUrl, undefined, { cacheTimeout: -1 });
}

// Resolves the CoprocessorConfig proxy address from env (or the addresses directory when
// `useInternalProxyAddress` is set), returning undefined when none is configured.
export function resolveCoprocessorConfigAddress(useInternalProxyAddress: boolean): string | undefined {
  if (useInternalProxyAddress) {
    loadHostAddresses();
  }
  const address = process.env[COPROCESSOR_CONFIG_ADDRESS_ENV_VAR];
  return address && address.trim() !== '' ? address : undefined;
}

export function requireCoprocessorConfigAddress(useInternalProxyAddress: boolean): string {
  const address = resolveCoprocessorConfigAddress(useInternalProxyAddress);
  if (!address) {
    throw new Error(
      `No CoprocessorConfig address configured. Set ${COPROCESSOR_CONFIG_ADDRESS_ENV_VAR} or pass --use-internal-proxy-address.`,
    );
  }
  return address;
}

export function addUseInternalProxyAddressParam(definition: ConfigurableTaskDefinition): ConfigurableTaskDefinition {
  return definition.addOptionalParam(
    'useInternalProxyAddress',
    'Resolve the CoprocessorConfig address from the /addresses directory instead of the environment',
    false,
    types.boolean,
  );
}

// All coprocessor mirror tasks share the same CLI surface.
export function addMirrorTaskParams(
  definition: ConfigurableTaskDefinition,
  readTarget: string,
): ConfigurableTaskDefinition {
  return addUseInternalProxyAddressParam(
    definition
      .addParam(
        'canonicalRpcUrl',
        'RPC URL of the canonical host chain (Ethereum) to read CoprocessorConfig from.',
        undefined,
        types.string,
      )
      .addParam(
        'canonicalCoprocessorConfigAddress',
        'Address of the CoprocessorConfig contract on the canonical host chain.',
        undefined,
        types.string,
      )
      .addOptionalParam(
        'blockNumber',
        `Canonical block height to read ${readTarget} from; must already be finalized. Defaults to the latest finalized block.`,
        undefined,
        types.int,
      ),
  );
}
