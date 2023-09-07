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
    this.comp = await deployCompFixture();
    const instances = await createInstances(await this.comp.getAddress(), ethers, this.signers);
    await this.comp.delegate(this.signers.alice.address);
    const encryptedAmount = instances.alice.encrypt32(1000);
    const transaction = await this.comp.initSupply(encryptedAmount);
    transaction.wait();
  });

  beforeEach(async function () {
    const { governor, timelock } = await deployGovernorZamaFixture(this.comp);
    this.contractAddress = await governor.getAddress();
    this.governor = governor;
    this.instances = await createInstances(this.contractAddress, ethers, this.signers);

    const tx1 = await timelock.setPendingAdmin(governor.getAddress());
    await tx1.wait();

    const transaction = await this.comp.setAllowedContract(this.contractAddress);
    const transaction2 = await this.governor.__acceptAdmin();

    await Promise.all([transaction.wait(), transaction2.wait()]);
  });

  // it('should propose a vote', async function () {
  //   const callDatas = [ethers.AbiCoder.defaultAbiCoder().encode(['address'], [this.signers.alice.address])];
  //   const tx = await this.governor.propose(
  //     [this.signers.alice],
  //     ['0'],
  //     ['getBalanceOf(address)'],
  //     callDatas,
  //     0,
  //     'do nothing',
  //     { gasLimit: 500000 },
  //   );
  //   await tx.wait();
  //   const proposalId = await this.governor.latestProposalIds(this.signers.alice.address);
  //   const proposals = await this.governor.proposals(proposalId);
  //   expect(proposals.id).to.equal(proposalId);
  //   expect(proposals.proposer).to.equal(this.signers.alice.address);
  // }).timeout(120000);

  it('should vote and return a Succeed', async function () {
    const callDatas = [ethers.AbiCoder.defaultAbiCoder().encode(['address'], [this.signers.alice.address])];
    const tx = await this.governor.propose(
      [this.signers.alice],
      ['0'],
      ['getBalanceOf(address)'],
      callDatas,
      5,
      'do nothing',
      { gasLimit: 500000 },
    );
    await tx.wait();
    const proposalId = await this.governor.latestProposalIds(this.signers.alice.address);
    const proposals = await this.governor.proposals(proposalId);
    console.log(proposals);
    await waitForBlock(proposals.startBlock + 1n);

    // Cast some votes
    const encryptedSupportAlice = this.instances.alice.encrypt32(1);
    const txVoteAlice = await this.governor['castVote(uint256,bytes)'](proposalId, encryptedSupportAlice);
    const encryptedSupportBob = this.instances.bob.encrypt32(1);
    const txVoteBob = await this.governor
      .connect(this.signers.bob)
      ['castVote(uint256,bytes)'](proposalId, encryptedSupportBob);
    const encryptedSupportCarol = this.instances.carol.encrypt32(0);
    const txVoteCarol = await this.governor
      .connect(this.signers.carol)
      ['castVote(uint256,bytes)'](proposalId, encryptedSupportCarol);
    await Promise.all([txVoteAlice.wait(), txVoteBob.wait(), txVoteCarol.wait()]);

    await waitForBlock(proposals.endBlock + 1n);

    const state = await this.governor.state(proposalId);
    expect(state).to.equal(4n);
  }).timeout(120000);

  it('should vote and return a Defeated ', async function () {
    const callDatas = [ethers.AbiCoder.defaultAbiCoder().encode(['address'], [this.signers.alice.address])];
    const tx = await this.governor.propose(
      [this.signers.alice],
      ['0'],
      ['getBalanceOf(address)'],
      callDatas,
      5,
      'do nothing',
      { gasLimit: 500000 },
    );
    await tx.wait();
    const proposalId = await this.governor.latestProposalIds(this.signers.alice.address);
    const proposals = await this.governor.proposals(proposalId);
    console.log(proposals);
    await waitForBlock(proposals.startBlock + 1n);

    // Cast some votes
    const encryptedSupportAlice = this.instances.alice.encrypt32(0);
    const txVoteAlice = await this.governor['castVote(uint256,bytes)'](proposalId, encryptedSupportAlice);
    const encryptedSupportBob = this.instances.bob.encrypt32(0);
    const txVoteBob = await this.governor
      .connect(this.signers.bob)
      ['castVote(uint256,bytes)'](proposalId, encryptedSupportBob);
    const encryptedSupportCarol = this.instances.bob.encrypt32(1);
    const txVoteCarol = await this.governor
      .connect(this.signers.bob)
      ['castVote(uint256,bytes)'](proposalId, encryptedSupportCarol);
    await Promise.all([txVoteAlice.wait(), txVoteBob.wait(), txVoteCarol.wait()]);

    await waitForBlock(proposals.endBlock + 1n);

    const state = await this.governor.state(proposalId);
    expect(state).to.equal(3n);
  }).timeout(120000);

  it('should cancel', async function () {
    await this.comp.delegate(this.signers.alice.address);
    const callDatas = [ethers.AbiCoder.defaultAbiCoder().encode(['address'], [this.signers.alice.address])];
    const tx = await this.governor.propose(
      [this.signers.alice],
      ['0'],
      ['getBalanceOf(address)'],
      callDatas,
      0,
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
