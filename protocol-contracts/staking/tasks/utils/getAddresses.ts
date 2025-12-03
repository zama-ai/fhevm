import { getOperatorStakingName, getOperatorRewarderName, getProtocolStakingProxyName } from '../deployment';
import { getRequiredEnvVar } from './loadVariables';
import { HardhatRuntimeEnvironment } from 'hardhat/types';

// Get the coprocessor protocol staking proxy address
export async function getProtocolStakingCoproProxyAddress(hre: HardhatRuntimeEnvironment): Promise<string> {
  const tokenName = getRequiredEnvVar('PROTOCOL_STAKING_COPRO_TOKEN_NAME');
  return await getProtocolStakingProxyAddress(tokenName, hre);
}

// Get the KMS protocol staking proxy address
export async function getProtocolStakingKMSProxyAddress(hre: HardhatRuntimeEnvironment): Promise<string> {
  const tokenName = getRequiredEnvVar('PROTOCOL_STAKING_KMS_TOKEN_NAME');
  return await getProtocolStakingProxyAddress(tokenName, hre);
}

// Get all the coprocessor operator staking addresses
export async function getAllOperatorStakingCoproAddresses(hre: HardhatRuntimeEnvironment): Promise<string[]> {
  const operatorStakingAddresses: string[] = [];

  // Get the number of operator staking contracts for coprocessors
  const numOperatorStakingCopro = parseInt(getRequiredEnvVar('NUM_OPERATOR_STAKING_COPRO'));

  // Register the operator staking contracts for coprocessors
  for (let i = 0; i < numOperatorStakingCopro; i++) {
    const coproTokenName = getRequiredEnvVar(`OPERATOR_STAKING_COPRO_TOKEN_NAME_${i}`);
    const operatorStakingCoproAddress = await getOperatorStakingAddress(coproTokenName, hre);
    operatorStakingAddresses.push(operatorStakingCoproAddress);
  }

  return operatorStakingAddresses;
}

export async function getAllOperatorStakingKMSAddresses(hre: HardhatRuntimeEnvironment): Promise<string[]> {
  const operatorStakingAddresses: string[] = [];

  // Get the number of operator staking contracts for KMS
  const numOperatorStakingKms = parseInt(getRequiredEnvVar('NUM_OPERATOR_STAKING_KMS'));

  // Register the operator staking contracts for KMS
  for (let i = 0; i < numOperatorStakingKms; i++) {
    const kmsTokenName = getRequiredEnvVar(`OPERATOR_STAKING_KMS_TOKEN_NAME_${i}`);
    const operatorStakingKmsAddress = await getOperatorStakingAddress(kmsTokenName, hre);
    operatorStakingAddresses.push(operatorStakingKmsAddress);
  }

  return operatorStakingAddresses;
}

export async function getAllOperatorStakingAddresses(hre: HardhatRuntimeEnvironment): Promise<string[]> {
  const operatorStakingCoproAddresses = await getAllOperatorStakingCoproAddresses(hre);
  const operatorStakingKmsAddresses = await getAllOperatorStakingKMSAddresses(hre);
  return [...operatorStakingCoproAddresses, ...operatorStakingKmsAddresses];
}

export async function getAllOperatorRewarderCoproAddresses(hre: HardhatRuntimeEnvironment): Promise<string[]> {
  const operatorRewarderAddresses: string[] = [];

  // Get the number of operator staking contracts for coprocessors
  const numOperatorStakingCopro = parseInt(getRequiredEnvVar('NUM_OPERATOR_STAKING_COPRO'));

  // Register the operator staking contracts for coprocessors
  for (let i = 0; i < numOperatorStakingCopro; i++) {
    const coproTokenName = getRequiredEnvVar(`OPERATOR_STAKING_COPRO_TOKEN_NAME_${i}`);
    const operatorStakingCoproAddress = await getOperatorRewarderAddress(coproTokenName, hre);
    operatorRewarderAddresses.push(operatorStakingCoproAddress);
  }

  return operatorRewarderAddresses;
}
export async function getAllOperatorRewarderKMSAddresses(hre: HardhatRuntimeEnvironment): Promise<string[]> {
  const operatorRewarderAddresses: string[] = [];

  // Get the number of operator staking contracts for KMS
  const numOperatorStakingKms = parseInt(getRequiredEnvVar('NUM_OPERATOR_STAKING_KMS'));

  // Register the operator staking contracts for KMS
  for (let i = 0; i < numOperatorStakingKms; i++) {
    const kmsTokenName = getRequiredEnvVar(`OPERATOR_STAKING_KMS_TOKEN_NAME_${i}`);
    const operatorStakingKmsAddress = await getOperatorRewarderAddress(kmsTokenName, hre);
    operatorRewarderAddresses.push(operatorStakingKmsAddress);
  }

  return operatorRewarderAddresses;
}

export async function getAllOperatorRewarderAddresses(hre: HardhatRuntimeEnvironment): Promise<string[]> {
  const operatorRewarderCoproAddresses = await getAllOperatorRewarderCoproAddresses(hre);
  const operatorRewarderKmsAddresses = await getAllOperatorRewarderKMSAddresses(hre);
  return [...operatorRewarderCoproAddresses, ...operatorRewarderKmsAddresses];
}

// Get the protocol staking proxy address
async function getProtocolStakingProxyAddress(tokenName: string, hre: HardhatRuntimeEnvironment): Promise<string> {
  const { get } = hre.deployments;
  return (await get(getProtocolStakingProxyName(tokenName))).address;
}

// Get the operator staking address
async function getOperatorStakingAddress(tokenName: string, hre: HardhatRuntimeEnvironment): Promise<string> {
  const { get } = hre.deployments;
  return (await get(getOperatorStakingName(tokenName))).address;
}

// Get the operator rewarder address
async function getOperatorRewarderAddress(tokenName: string, hre: HardhatRuntimeEnvironment): Promise<string> {
  const { get } = hre.deployments;
  return (await get(getOperatorRewarderName(tokenName))).address;
}
