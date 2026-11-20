import type { Interface } from 'ethers';
import type { HardhatRuntimeEnvironment } from 'hardhat/types';

// Builds a calldata-only ABI for `contractName` without a deployer key, so the `build*` tasks never
// need a signer.
export async function getContractInterface(hre: HardhatRuntimeEnvironment, contractName: string): Promise<Interface> {
  await hre.run('compile:specific', { contract: 'contracts' });
  const artifact = await hre.artifacts.readArtifact(contractName);
  return new hre.ethers.Interface(artifact.abi);
}
