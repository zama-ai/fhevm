import type { ContractRunner, Interface, Provider } from 'ethers';
import { types } from 'hardhat/config';
import type { ConfigurableTaskDefinition, HardhatRuntimeEnvironment } from 'hardhat/types';

import type { ProtocolConfig, ProtocolConfigReplica } from '../../types';
import { getRequiredEnvVar, loadHostAddresses } from './loadVariables';

// Shared plumbing for the ProtocolConfig and ProtocolConfigReplica tasks (KMS and coprocessor
// contexts, canonical and mirror): ABI loading, target address resolution, broadcasting and the
// common CLI parameters.

export const PROTOCOL_CONFIG_ADDRESS_ENV_VAR = 'PROTOCOL_CONFIG_CONTRACT_ADDRESS';

export interface EncodedCall {
  functionSignature: string;
  calldata: string;
  decodedArgs: unknown[];
}

// Builds a calldata-only ABI for `contractName` without a deployer key, so the `build*` tasks never
// need a signer.
async function getContractInterface(hre: HardhatRuntimeEnvironment, contractName: string): Promise<Interface> {
  await hre.run('compile:specific', { contract: 'contracts' });
  const artifact = await hre.artifacts.readArtifact(contractName);
  return new hre.ethers.Interface(artifact.abi);
}

export async function getProtocolConfigInterface(hre: HardhatRuntimeEnvironment): Promise<Interface> {
  return getContractInterface(hre, 'ProtocolConfig');
}

export async function getProtocolConfigReplicaInterface(hre: HardhatRuntimeEnvironment): Promise<Interface> {
  return getContractInterface(hre, 'ProtocolConfigReplica');
}

// Without a response cache, so the block-hash recheck that closes every canonical read reaches the RPC.
export function getCanonicalProvider(hre: HardhatRuntimeEnvironment, canonicalRpcUrl: string): Provider {
  return new hre.ethers.JsonRpcProvider(canonicalRpcUrl, undefined, { cacheTimeout: -1 });
}

// Connected to `runner` when given (e.g. the canonical provider), otherwise to the local network.
export async function getProtocolConfig(
  hre: HardhatRuntimeEnvironment,
  address: string,
  runner?: ContractRunner,
): Promise<ProtocolConfig> {
  const protocolConfig = await hre.ethers.getContractAt('ProtocolConfig', address);
  return (runner ? protocolConfig.connect(runner) : protocolConfig) as unknown as ProtocolConfig;
}

export async function getProtocolConfigReplica(
  hre: HardhatRuntimeEnvironment,
  address: string,
): Promise<ProtocolConfigReplica> {
  return (await hre.ethers.getContractAt('ProtocolConfigReplica', address)) as unknown as ProtocolConfigReplica;
}

// Resolves the ProtocolConfig proxy address from env (or the addresses directory when
// `useInternalProxyAddress` is set), returning undefined when none is configured.
export function resolveProtocolConfigAddress(useInternalProxyAddress: boolean): string | undefined {
  if (useInternalProxyAddress) {
    loadHostAddresses();
  }
  const address = process.env[PROTOCOL_CONFIG_ADDRESS_ENV_VAR];
  return address && address.trim() !== '' ? address : undefined;
}

export function requireProtocolConfigAddress(useInternalProxyAddress: boolean): string {
  const address = resolveProtocolConfigAddress(useInternalProxyAddress);
  if (!address) {
    throw new Error(
      `No ProtocolConfig address configured. Set ${PROTOCOL_CONFIG_ADDRESS_ENV_VAR} or pass --use-internal-proxy-address.`,
    );
  }
  return address;
}

// Broadcasts the byte-identical payload the DAO would sign, using the deployer key. On devnet / the
// test-suite the deployer is the ACL owner, so the call is authorized; this is the no-DAO path.
export async function broadcast(hre: HardhatRuntimeEnvironment, target: string, calldata: string): Promise<string> {
  const deployer = new hre.ethers.Wallet(getRequiredEnvVar('DEPLOYER_PRIVATE_KEY')).connect(hre.ethers.provider);
  const tx = await deployer.sendTransaction({ to: target, data: calldata });
  await tx.wait();
  return tx.hash;
}

export function addUseInternalProxyAddressParam(definition: ConfigurableTaskDefinition): ConfigurableTaskDefinition {
  return definition.addOptionalParam(
    'useInternalProxyAddress',
    'Resolve the ProtocolConfig address from the /addresses directory instead of the environment',
    false,
    types.boolean,
  );
}

// All mirror tasks (KMS and coprocessor) share the same CLI surface.
export function addMirrorTaskParams(
  definition: ConfigurableTaskDefinition,
  readTarget: string,
): ConfigurableTaskDefinition {
  return addUseInternalProxyAddressParam(
    definition
      .addParam(
        'canonicalRpcUrl',
        'RPC URL of the canonical host chain (Ethereum) to read ProtocolConfig from.',
        undefined,
        types.string,
      )
      .addParam(
        'canonicalProtocolConfigAddress',
        'Address of the ProtocolConfig contract on the canonical host chain.',
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
