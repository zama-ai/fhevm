import { expect } from 'chai';
import { ethers } from 'hardhat';

import { createInstances } from '../instance';
import { getSigners } from '../signers';
import { deployCompFixture } from './Comp.fixture';
import { deployGovernorZamaFixture } from './GovernorZama.fixture';

describe('GovernorZama', function () {
  before(async function () {
    this.signers = await getSigners();
  });

  beforeEach(async function () {
    this.comp = await deployCompFixture();
    const contract = await deployGovernorZamaFixture(this.comp);
    this.contractAddress = await contract.getAddress();
    this.governor = contract;
    this.instances = await createInstances(this.contractAddress, ethers, this.signers);
    const encryptedAmount = this.instances.alice.encrypt32(1000);
    const transaction = await this.comp.initSupply(encryptedAmount);
    await transaction.wait();
  });

  it('should propose a vote', async function () {
    await this.comp.delegate(this.signers.alice.address);
    const callDatas = [ethers.AbiCoder.defaultAbiCoder().encode(['address'], [this.signers.alice.address])];
    const tx = await this.governor.propose(
      [this.signers.alice],
      ['0'],
      ['getBalanceOf(address)'],
      callDatas,
      'do nothing',
    );
    await tx.wait();
    const proposalId = await this.governor.latestProposalIds(this.signers.alice.address);
    const proposals = await this.governor.proposals(proposalId);
    console.log(proposalId);
    console.log(proposals);
    expect(proposalId).to.equal(1);
  });
});
