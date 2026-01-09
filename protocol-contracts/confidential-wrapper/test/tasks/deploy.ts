import {
  getConfidentialWrapperName,
  getConfidentialWrapperProxyName,
  getConfidentialWrapperImplName,
  CONTRACT_NAME,
} from '../../tasks/deploy';
import { getRequiredEnvVar } from '../../tasks/utils/loadVariables';
import { expect } from 'chai';
import hre from 'hardhat';

// Helper function to verify that a contract is deployed at the given address.
// Checks both that the address is valid and that bytecode exists at that address.
async function expectContractDeployed(address: string) {
  expect(address).to.be.properAddress;
  const code = await hre.ethers.provider.getCode(address);
  expect(code).to.not.equal('0x');
}

describe('ConfidentialWrapper Deployment', function () {
  describe('All Confidential Wrappers', function () {
    it('Should deploy all confidential wrapper proxy contracts with correct configuration', async function () {
      // Deploy all confidential wrapper proxy contracts
      await hre.run('task:deployAllConfidentialWrappers');

      const numWrappers = parseInt(getRequiredEnvVar('NUM_CONFIDENTIAL_WRAPPERS'));

      for (let i = 0; i < numWrappers; i++) {
        // Get the expected configuration from environment
        const name = getRequiredEnvVar(`CONFIDENTIAL_WRAPPER_NAME_${i}`);
        const symbol = getRequiredEnvVar(`CONFIDENTIAL_WRAPPER_SYMBOL_${i}`);
        const contractUri = getRequiredEnvVar(`CONFIDENTIAL_WRAPPER_CONTRACT_URI_${i}`);
        const underlying = getRequiredEnvVar(`CONFIDENTIAL_WRAPPER_UNDERLYING_ADDRESS_${i}`);
        const owner = getRequiredEnvVar(`CONFIDENTIAL_WRAPPER_OWNER_ADDRESS_${i}`);

        // Get the deployed proxy contract
        const proxyDeployment = await hre.deployments.get(getConfidentialWrapperProxyName(name));
        const confidentialWrapper = await hre.ethers.getContractAt(CONTRACT_NAME, proxyDeployment.address);

        // Verify the contract was deployed
        await expectContractDeployed(proxyDeployment.address);

        // Verify the contract configuration
        expect(await confidentialWrapper.name()).to.equal(name);
        expect(await confidentialWrapper.symbol()).to.equal(symbol);
        expect(await confidentialWrapper.contractURI()).to.equal(contractUri);
        expect(await confidentialWrapper.underlying()).to.equal(underlying);
        expect(await confidentialWrapper.owner()).to.equal(owner);
      }
    });
  });

  describe('Helper Functions', function () {
    it('Should generate correct ConfidentialWrapper name', function () {
      expect(getConfidentialWrapperName('MyToken')).to.equal('ConfidentialWrapper_MyToken');
    });

    it('Should generate correct ConfidentialWrapper proxy name', function () {
      expect(getConfidentialWrapperProxyName('MyToken')).to.equal('ConfidentialWrapper_MyToken_Proxy');
    });

    it('Should generate correct ConfidentialWrapper implementation name', function () {
      expect(getConfidentialWrapperImplName('MyToken')).to.equal('ConfidentialWrapper_MyToken_Impl');
    });
  });
});
