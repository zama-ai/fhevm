import { expect } from 'chai';
import { ethers } from 'hardhat';

import { createInstances } from '../instance';
import { getSigners } from '../signers';
import { waitForBlock } from '../utils';
import { deployCompFixture } from './Comp.fixture';
import { deployGovernorZamaFixture } from './GovernorZama.fixture';

describe('GovernorZama', function () {
  before(async function () {
    this.signers = await getSigners();
  });

  before(async function () {
    this.comp = await deployCompFixture();
    const { governor, timelock } = await deployGovernorZamaFixture(this.comp);
    this.contractAddress = await governor.getAddress();
    this.governor = governor;
    this.instances = await createInstances(this.contractAddress, ethers, this.signers);
    const encryptedAmount = this.instances.alice.encrypt32(1000);

    const tx1 = await timelock.setPendingAdmin(governor.getAddress());
    await tx1.wait();

    const transaction = await this.comp.initSupply(encryptedAmount);
    const transaction2 = await this.comp.setAllowedContract(this.contractAddress);
    const transaction3 = await this.governor.__acceptAdmin();

    await Promise.all([transaction.wait(), transaction2.wait(), transaction3.wait()]);
    await this.comp.delegate(this.signers.alice.address);
  });

  it('should vote', async function () {
    const callDatas = [ethers.AbiCoder.defaultAbiCoder().encode(['address'], [this.signers.alice.address])];
    const tx = await this.governor.propose(
      [this.signers.alice],
      ['0'],
      ['getBalanceOf(address)'],
      callDatas,
      'do nothing',
      { gasLimit: 500000 },
    );
    await tx.wait();
    const proposalId = await this.governor.latestProposalIds(this.signers.alice.address);
    const proposals = await this.governor.proposals(proposalId);
    expect(proposals.id).to.equal(proposalId);
    expect(proposals.proposer).to.equal(this.signers.alice.address);
    await waitForBlock(proposals.startBlock + 1n);

    const encryptedSupport = this.instances.alice.encrypt32(1000);
    const txVote = await this.governor['castVote(uint256,bytes)'](1, encryptedSupport);
    const results = await txVote.wait();
    expect(results?.status).to.equal(1);
  }).timeout(120000);

  it('should cancel', async function () {
    await this.comp.delegate(this.signers.alice.address);
    const callDatas = [ethers.AbiCoder.defaultAbiCoder().encode(['address'], [this.signers.alice.address])];
    const tx = await this.governor.propose(
      [this.signers.alice],
      ['0'],
      ['getBalanceOf(address)'],
      callDatas,
      'do nothing',
      { gasLimit: 500000 },
    );
    await tx.wait();
    const proposalId = await this.governor.latestProposalIds(this.signers.alice.address);
    const proposals = await this.governor.proposals(proposalId);
    await waitForBlock(proposals.startBlock + 1n);

    const state = await this.governor.state(proposalId);
    expect(state).to.equal(1n);

    const txCancel = await this.governor.cancel(proposalId);
    await txCancel.wait();
    const newState = await this.governor.state(proposalId);
    expect(newState).to.equal(2n);
  }).timeout(120000);
});
