import { expect } from 'chai';
import dotenv from 'dotenv';
import 'dotenv/config';
import fs from 'fs';
import hre from 'hardhat';

describe('Deployment Task', function () {
  describe('Deploying Oracle Proxy and Implementation contracts', function () {
    it('Should deploy oracle', async function () {
      const privateKey = process.env.DEPLOYER_PRIVATE_KEY;
      if (!privateKey) {
        throw Error('DEPLOYER_PRIVATE_KEY not defined in .env');
      }
      const deployerAddress = hre.ethers.computeAddress(privateKey);
      const currentNonce = await hre.ethers.provider.getTransactionCount(deployerAddress);
      const predictedProxyAddress = hre.ethers.getCreateAddress({
        from: deployerAddress,
        nonce: currentNonce + 1, // +1 because first contract deployed is implementation, and second is actually the proxy
      });
      console.log('predictedProxyAddress', predictedProxyAddress);
      await hre.run('task:deployDecryptionOracle');
      const actualProxyaddress = dotenv.parse(
        fs.readFileSync('addresses/.env.decryptionoracle'),
      ).DECRYPTION_ORACLE_ADDRESS;
      expect(actualProxyaddress).to.equal(predictedProxyAddress);
    });
  });
});
