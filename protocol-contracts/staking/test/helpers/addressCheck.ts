import { expect } from 'chai';
import hre from 'hardhat';

// Helper function to verify that a contract is deployed at the given address.
// Checks both that the address is valid and that bytecode exists at that address.
export async function expectContractDeployed(address: string) {
  expect(address).to.be.properAddress;
  const code = await hre.ethers.provider.getCode(address);
  expect(code).to.not.equal('0x');
}
