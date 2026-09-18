import type { HardhatEthersSigner } from '@nomicfoundation/hardhat-ethers/signers';
import { ethers, fhevm } from 'hardhat';
import * as hre from 'hardhat';

import { DevnetFHETestAddress, TestnetFHETestAddress } from './addresses';

// npx hardhat test --grep "FHETest:deploy:" --network sepolia
// npx hardhat test --grep "FHETest:deploy:" --network devnet
// npx hardhat test --grep "FHETest:deploy:" --network hardhat

type Signers = {
  alice: HardhatEthersSigner;
};

describe('FHETest deploy', function () {
  let signers: Signers;
  let fheTestContractAddress: string;

  before(async function () {
    fheTestContractAddress = ethers.ZeroAddress;

    if (hre.network.name === 'devnet') {
      fheTestContractAddress = DevnetFHETestAddress;
    } else if (hre.network.name === 'sepolia') {
      fheTestContractAddress = TestnetFHETestAddress;
    } else {
      fheTestContractAddress = ethers.ZeroAddress;
    }

    const ethSigners: HardhatEthersSigner[] = await ethers.getSigners();
    signers = { alice: ethSigners[0] };
  });

  it('FHETest:deploy:', async function () {
    console.log(`Network: ${hre.network.name}`);

    // Set test timeout. Must be long enough since we are deploying a contract on Sepolia
    this.timeout(4 * 40000);

    if (fheTestContractAddress !== '' && fheTestContractAddress !== ethers.ZeroAddress) {
      const code = await hre.ethers.provider.getCode(fheTestContractAddress);
      if (code.length > 3) {
        console.log(`FHETest: ${fheTestContractAddress} (already deployed)`);
        return;
      }
    }

    const contractFactory = await hre.ethers.getContractFactory('FHETest');
    const contract = await contractFactory.connect(signers.alice).deploy();
    const tx = await contract.waitForDeployment();
    const deployTx = tx.deploymentTransaction();
    if (deployTx) {
      console.log(`Tx: ${deployTx.hash}`);
      console.log(`BlockHash: ${deployTx.blockHash}`);
      console.log(`chainId: ${deployTx.chainId}`);
    }

    console.log(`Deployer: ${signers.alice.address}`);
    console.log(`FHETest: ${await contract.getAddress()}`);

    const coprocessorConfig = await fhevm.getCoprocessorConfig(await contract.getAddress());
    console.log(JSON.stringify(coprocessorConfig, null, 2));

    const cfg = await contract.getCoprocessorConfig();
    console.log(JSON.stringify(cfg, null, 2));
  });
});
