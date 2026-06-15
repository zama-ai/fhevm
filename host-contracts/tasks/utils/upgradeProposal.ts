import { Interface } from 'ethers';
import type { HardhatRuntimeEnvironment } from 'hardhat/types';

import { getRequiredEnvVar } from './loadVariables';

// Generic prepare/execute machinery for UUPS upgrades governed by upgradeToAndCall payloads.
//
// The DAO path stops after `buildUpgradeProposal` + `printUpgradeProposal`: the implementation is
// deployed, the payload is printed, and the DAO executes it. The direct (devnet) path is the same
// prepare step followed by `executeUpgradeProposal`, which simply sends the prepared payload
// with the deployer key — so what runs on devnet is byte-identical to what the DAO signs.

export function toJsonString(value: unknown): string {
  return JSON.stringify(
    value,
    (_, nestedValue: unknown) => (typeof nestedValue === 'bigint' ? nestedValue.toString() : nestedValue),
    2,
  );
}

export const UPGRADE_TO_AND_CALL_INTERFACE = new Interface([
  'function upgradeToAndCall(address newImplementation, bytes data) payable',
]);

export type UpgradeProposal = {
  proxyAddress: string;
  newImplementationAddress: string;
  innerFunctionSignature: string;
  decodedArgs: unknown[];
  innerCalldata: string;
  outerCalldata: string;
};

export async function buildUpgradeProposal(
  hre: HardhatRuntimeEnvironment,
  params: {
    proxyAddress: string;
    contractName: string;
    innerFunctionName: string;
    decodedArgs: unknown[];
  },
): Promise<UpgradeProposal> {
  const { ethers, upgrades } = hre;
  const privateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
  const deployer = new ethers.Wallet(privateKey).connect(ethers.provider);
  const currentImplementation = await ethers.getContractFactory('EmptyUUPSProxy', deployer);
  const newImplementation = await ethers.getContractFactory(params.contractName, deployer);
  await upgrades.forceImport(params.proxyAddress, currentImplementation);
  const newImplementationAddress = String(
    await upgrades.prepareUpgrade(params.proxyAddress, newImplementation, {
      kind: 'uups',
    }),
  );
  // The factory's interface already knows the new implementation's ABI, so encode by function name
  // directly — no need to re-read the artifact or precompute a sighash at the call site.
  const innerCalldata = newImplementation.interface.encodeFunctionData(params.innerFunctionName, params.decodedArgs);
  const innerFunctionSignature = newImplementation.interface.getFunction(params.innerFunctionName)!.format('sighash');
  const outerCalldata = UPGRADE_TO_AND_CALL_INTERFACE.encodeFunctionData('upgradeToAndCall', [
    newImplementationAddress,
    innerCalldata,
  ]);

  return {
    proxyAddress: params.proxyAddress,
    newImplementationAddress,
    innerFunctionSignature,
    decodedArgs: params.decodedArgs,
    innerCalldata,
    outerCalldata,
  };
}

// The entire direct (non-DAO) execution path: send the exact payload the DAO would sign, using the
// deployer key.
export async function executeUpgradeProposal(hre: HardhatRuntimeEnvironment, prepared: UpgradeProposal): Promise<void> {
  const { ethers } = hre;
  const deployer = new ethers.Wallet(getRequiredEnvVar('DEPLOYER_PRIVATE_KEY')).connect(ethers.provider);
  console.log(
    `Executing prepared upgrade on ${prepared.proxyAddress} (implementation ${prepared.newImplementationAddress})...`,
  );
  const tx = await deployer.sendTransaction({ to: prepared.proxyAddress, data: prepared.outerCalldata });
  await tx.wait();
}

export async function verifyProposalImplementation(
  hre: HardhatRuntimeEnvironment,
  data: UpgradeProposal,
  contract: string,
): Promise<void> {
  console.log('Waiting 2 minutes before contract verification... Please wait...');
  await new Promise((resolve) => setTimeout(resolve, 2 * 60 * 1000));
  await hre.run('verify:verify', {
    address: data.newImplementationAddress,
    contract,
    constructorArguments: [],
  });
}

export function printUpgradeProposal(data: UpgradeProposal): void {
  console.log('proxyAddress:', data.proxyAddress);
  console.log('newImplementationAddress:', data.newImplementationAddress);
  console.log('innerFunctionSignature:', data.innerFunctionSignature);
  console.log('decodedArgs:', toJsonString(data.decodedArgs));
  console.log(`${data.innerFunctionSignature} calldata:`, data.innerCalldata);
  console.log('upgradeToAndCall(address,bytes) calldata:', data.outerCalldata);
  console.log(
    `Cast command: cast calldata 'upgradeToAndCall(address,bytes)' ${data.newImplementationAddress} ${data.innerCalldata}`,
  );
}
