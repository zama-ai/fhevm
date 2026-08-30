import type { HardhatEthersSigner } from '@nomicfoundation/hardhat-ethers/signers';
import { ethers, fhevm } from 'hardhat';
import * as hre from 'hardhat';

import { DevnetFHECounterPublicDecryptAddress, TestnetFHECounterPublicDecryptAddress } from './addresses';

// npx hardhat test --grep "Sepolia:DeployFHECounterPublicDecrypt" --network sepolia
// npx hardhat test --grep "Sepolia:DeployFHECounterPublicDecrypt" --network devnet

type Signers = {
  alice: HardhatEthersSigner;
};

describe('Sepolia:DeployFHECounterPublicDecrypt', function () {
  let signers: Signers;
  let fheCounterContractAddress: string;

  before(async function () {
    // Only Sepolia
    if (fhevm.isCleartext) {
      return;
    }

    if (hre.network.name === 'devnet') {
      fheCounterContractAddress = DevnetFHECounterPublicDecryptAddress;
    } else {
      fheCounterContractAddress = TestnetFHECounterPublicDecryptAddress;
    }

    const ethSigners: HardhatEthersSigner[] = await ethers.getSigners();
    signers = { alice: ethSigners[0] };
  });

  it('Seplia:Deploy', async function () {
    if (fhevm.isCleartext) {
      console.log(`Ignore. This test is not running in cleartext mode.`);
      return;
    }

    console.log(`Network: ${hre.network.name}`);

    // Set test timeout. Must be long enough since we are deploying a contract on Sepolia
    this.timeout(4 * 40000);

    if (fheCounterContractAddress !== '' && fheCounterContractAddress !== ethers.ZeroAddress) {
      const code = await hre.ethers.provider.getCode(fheCounterContractAddress);
      if (code.length > 3) {
        console.log(`FHECounterPublicDecrypt: ${fheCounterContractAddress} (already deployed)`);
        return;
      }
    }

    const contractFactory = await hre.ethers.getContractFactory('FHECounterPublicDecrypt');
    const contract = await contractFactory.connect(signers.alice).deploy();
    const tx = await contract.waitForDeployment();
    const deployTx = tx.deploymentTransaction();
    if (deployTx) {
      console.log(`Tx: ${deployTx.hash}`);
      console.log(`BlockHash: ${deployTx.blockHash}`);
      console.log(`chainId: ${deployTx.chainId}`);
    }

    console.log(`Deployer: ${signers.alice.address}`);
    console.log(`FHECounterPublicDecrypt: ${await contract.getAddress()}`);

    const coprocessorConfig = await fhevm.getCoprocessorConfig(await contract.getAddress());
    console.log(JSON.stringify(coprocessorConfig, null, 2));
  });
});
