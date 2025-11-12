import { expect } from 'chai';
import { ethers } from 'hardhat';

import { createInstances } from '../instance';
import { getSigners, initSigners } from '../signers';

describe('TestEthereumCoprocessorConfig', function () {
  before(async function () {
    await initSigners(2);
    this.signers = await getSigners();
  });

  beforeEach(async function () {
    const contractFactory = await ethers.getContractFactory('TestEthereumCoprocessorConfig');
    this.contract = await contractFactory.connect(this.signers.alice).deploy();
    await this.contract.waitForDeployment();
    this.contractAddress = await this.contract.getAddress();
    this.instances = await createInstances(this.signers);
  });

  it('test local confidentialProtocolId', async function () {
    const confidentialProtocolId = await this.contract.connect(this.signers.carol).confidentialProtocolId();
    expect(confidentialProtocolId).to.equal(100001); // Zama confidentialProtocolId == 0 on any chain other than mainnet/sepolia
  });

  it('test local CoprocessorConfig', async function () {
    const cfg = await this.contract.connect(this.signers.carol).getCoprocessorConfig();
    // ACL
    expect(cfg[0]).to.eq('0x50157CFfD6bBFA2DECe204a89ec419c23ef5755D');
    // FHEVMExecutor
    expect(cfg[1]).to.eq('0xe3a9105a3a932253A70F126eb1E3b589C643dD24');
    // KMSVerifier
    expect(cfg[2]).to.eq('0x901F8942346f7AB3a01F6D7613119Bca447Bb030');
  });
});
