import { expect } from 'chai';
import { ethers } from 'hardhat';

import { initDecryptionOracle } from '../asyncDecrypt';
import { createInstances } from '../instance';
import { getSigners, initSigners } from '../signers';

describe('TestSepoliaCoprocessorConfig', function () {
  before(async function () {
    await initSigners(2);
    this.signers = await getSigners();
    this.relayerAddress = '0x97F272ccfef4026A1F3f0e0E879d514627B84E69';
    this.instances = await createInstances(this.signers);
    await initDecryptionOracle();
  });

  beforeEach(async function () {
    const contractFactory = await ethers.getContractFactory('TestSepoliaCoprocessorConfig');
    this.contract = await contractFactory.connect(this.signers.alice).deploy();
    await this.contract.waitForDeployment();
    this.contractAddress = await this.contract.getAddress();
    this.instances = await createInstances(this.signers);
  });

  it('test protocolId', async function () {
    const protocolId = await this.contract.connect(this.signers.carol).protocolId();
    expect(protocolId).to.equal(1);
  });
});
