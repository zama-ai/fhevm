import {
  OPERATOR_REWARDER_CONTRACT_NAME,
  OPERATOR_STAKING_CONTRACT_NAME,
  PROTOCOL_STAKING_CONTRACT_NAME,
} from '../tasks/deployment';
import {
  getProtocolStakingCoproProxyAddress,
  getProtocolStakingKMSProxyAddress,
  getAllOperatorRewarderCoproAddresses,
  getAllOperatorRewarderKMSAddresses,
  getAllOperatorStakingCoproAddresses,
  getAllOperatorStakingKMSAddresses,
} from '../tasks/utils/getAddresses';
import { expect } from 'chai';
import hre from 'hardhat';

// Helper function to verify that a contract is deployed at the given address.
// Checks both that the address is valid and that bytecode exists at that address.
export async function expectContractDeployed(address: string) {
  expect(address).to.be.properAddress;
  const code = await hre.ethers.provider.getCode(address);
  expect(code).to.not.equal('0x');
}

export async function getProtocolStakingContractsFixture() {
  // Get the protocol staking addresses
  const coproProtocolStakingAddress = await getProtocolStakingCoproProxyAddress(hre);
  const kmsProtocolStakingAddress = await getProtocolStakingKMSProxyAddress(hre);

  // Load the protocol staking contracts
  const coproProtocolStaking = await hre.ethers.getContractAt(
    PROTOCOL_STAKING_CONTRACT_NAME,
    coproProtocolStakingAddress,
  );
  const kmsProtocolStaking = await hre.ethers.getContractAt(PROTOCOL_STAKING_CONTRACT_NAME, kmsProtocolStakingAddress);

  return { coproProtocolStaking, kmsProtocolStaking };
}

export async function getOperatorStakingContractsFixture() {
  // Get operator staking addresses and load contracts
  const coproOperatorStakingAddresses = await getAllOperatorStakingCoproAddresses(hre);
  const kmsOperatorStakingAddresses = await getAllOperatorStakingKMSAddresses(hre);

  // Load all coprocessor operator staking contracts
  const coproOperatorStakings = await Promise.all(
    coproOperatorStakingAddresses.map((address: string) =>
      hre.ethers.getContractAt(OPERATOR_STAKING_CONTRACT_NAME, address),
    ),
  );

  // Load all KMS operator staking contracts
  const kmsOperatorStakings = await Promise.all(
    kmsOperatorStakingAddresses.map((address: string) =>
      hre.ethers.getContractAt(OPERATOR_STAKING_CONTRACT_NAME, address),
    ),
  );

  return { coproOperatorStakings, kmsOperatorStakings };
}

export async function getOperatorRewarderContractsFixture() {
  // Get operator rewarder addresses and load contracts
  const coproOperatorRewarderAddresses = await getAllOperatorRewarderCoproAddresses(hre);
  const kmsOperatorRewarderAddresses = await getAllOperatorRewarderKMSAddresses(hre);

  // Load all coprocessor operator rewarder contracts
  const coproOperatorRewarders = await Promise.all(
    coproOperatorRewarderAddresses.map((address: string) =>
      hre.ethers.getContractAt(OPERATOR_REWARDER_CONTRACT_NAME, address),
    ),
  );

  // Load all KMS operator rewarder contracts
  const kmsOperatorRewarders = await Promise.all(
    kmsOperatorRewarderAddresses.map((address: string) =>
      hre.ethers.getContractAt(OPERATOR_REWARDER_CONTRACT_NAME, address),
    ),
  );

  return { coproOperatorRewarders, kmsOperatorRewarders };
}
