import { expect } from 'chai';
import { ethers, network } from 'hardhat';

import { createInstances } from '../instance';
import { getSigners, initSigners } from '../signers';
import { createTransaction, mineNBlocks, produceDummyTransactions, waitForBlock } from '../utils';
import { deployCompFixture } from './Comp.fixture';
import { deployGovernorZamaFixture, deployTimelockFixture } from './GovernorZama.fixture';

describe.skip('GovernorZama', function () {
  before(async function () {
    await initSigners(3);
    this.signers = await getSigners();
    this.comp = await deployCompFixture();

    const instances = await createInstances(await this.comp.getAddress(), ethers, this.signers);
    const encryptedAmountToTransfer = instances.alice.encrypt64(100000);
    const transfer1 = await this.comp['transfer(address,bytes)'](this.signers.bob.address, encryptedAmountToTransfer);
    const transfer2 = await this.comp['transfer(address,bytes)'](this.signers.carol.address, encryptedAmountToTransfer);
    await Promise.all([transfer1.wait(), transfer2.wait()]);

    const delegate1 = await this.comp.delegate(this.signers.alice);
    const delegate2 = await this.comp.connect(this.signers.bob).delegate(this.signers.bob);
    const delegate3 = await this.comp.connect(this.signers.carol).delegate(this.signers.carol);
    await Promise.all([delegate1, delegate2, delegate3]);
    if (network.name == 'localNetwork1' || network.name == 'hardhat') {
      // inside network1 or hardhat blocks are not
      // produced unless there are transactions and
      // we rely on block production for voting time
      produceDummyTransactions(100);
    }
  });

  beforeEach(async function () {
    const timelock = await deployTimelockFixture();

    const governor = await deployGovernorZamaFixture(this.comp, timelock);
    this.contractAddress = await governor.getAddress();
    this.governor = governor;
    this.instances = await createInstances(this.contractAddress, ethers, this.signers);

    const tx1 = await timelock.setPendingAdmin(governor.getAddress());
    await tx1.wait();

    const transaction = await this.comp.setAllowedContract(this.contractAddress);
    const transaction2 = await this.governor.__acceptAdmin();

    await Promise.all([transaction.wait(), transaction2.wait()]);
  });

  it('should propose a vote', async function () {
    const callDatas = [ethers.AbiCoder.defaultAbiCoder().encode(['address'], [this.signers.alice.address])];
    const tx = await createTransaction(
      this.governor.propose,
      [this.signers.alice],
      ['0'],
      ['getBalanceOf(address)'],
      callDatas,
      0,
      'do nothing',
    );
    const proposal = await tx.wait();
    expect(proposal?.status).to.equal(1);
    const proposalId = await this.governor.latestProposalIds(this.signers.alice.address);
    const proposals = await this.governor.proposals(proposalId);
    expect(proposals.id).to.equal(proposalId);
    expect(proposals.proposer).to.equal(this.signers.alice.address);
  });

  it('should vote and return a Succeed', async function () {
    const callDatas = [ethers.AbiCoder.defaultAbiCoder().encode(['address'], [this.signers.alice.address])];
    const tx = await createTransaction(
      this.governor.propose,
      [this.signers.alice],
      ['0'],
      ['getBalanceOf(address)'],
      callDatas,
      5,
      'do nothing',
    );
    const proposal = await tx.wait();
    expect(proposal?.status).to.equal(1);

    const proposalId = await this.governor.latestProposalIds(this.signers.alice.address);
    const proposals = await this.governor.proposals(proposalId);
    if (network.name == 'hardhat') {
      await mineNBlocks(2);
    }
    await waitForBlock(proposals.startBlock + 1n);
    // Cast some votes
    const encryptedSupportBob = this.instances.bob.encryptBool(true);
    const txVoteBob = await createTransaction(
      this.governor.connect(this.signers.bob)['castVote(uint256,bytes)'],
      proposalId,
      encryptedSupportBob,
    );

    const encryptedSupportCarol = this.instances.carol.encryptBool(true);
    const txVoteCarol = await createTransaction(
      this.governor.connect(this.signers.carol)['castVote(uint256,bytes)'],
      proposalId,
      encryptedSupportCarol,
    );

    const [bobResults, carolResults] = await Promise.all([txVoteBob.wait(), txVoteCarol.wait()]);
    expect(bobResults?.status).to.equal(1);
    expect(carolResults?.status).to.equal(1);
    if (network.name == 'hardhat') {
      await mineNBlocks(5);
    }
    await waitForBlock(proposals.endBlock + 1n);

    const state = await this.governor.state(proposalId);
    expect(state).to.equal(4n);
  });

  it('should vote and return a Defeated ', async function () {
    const callDatas = [ethers.AbiCoder.defaultAbiCoder().encode(['address'], [this.signers.alice.address])];
    const tx = await createTransaction(
      this.governor.propose,
      [this.signers.alice],
      ['0'],
      ['getBalanceOf(address)'],
      callDatas,
      4,
      'do nothing',
    );
    const proposal = await tx.wait();
    expect(proposal?.status).to.equal(1);
    const proposalId = await this.governor.latestProposalIds(this.signers.alice.address);
    const proposals = await this.governor.proposals(proposalId);
    if (network.name == 'hardhat') {
      await mineNBlocks(2);
    }
    await waitForBlock(proposals.startBlock + 1n);

    // Cast some votes
    const encryptedSupportBob = this.instances.bob.encryptBool(false);
    const txVoteBob = await createTransaction(
      this.governor.connect(this.signers.bob)['castVote(uint256,bytes)'],
      proposalId,
      encryptedSupportBob,
    );

    const encryptedSupportCarol = this.instances.carol.encryptBool(true);
    const txVoteCarol = await createTransaction(
      this.governor.connect(this.signers.carol)['castVote(uint256,bytes)'],
      proposalId,
      encryptedSupportCarol,
    );

    const [bobResults, aliceResults] = await Promise.all([txVoteBob.wait(), txVoteCarol.wait()]);
    expect(bobResults?.status).to.equal(1);
    expect(aliceResults?.status).to.equal(1);
    if (network.name == 'hardhat') {
      await mineNBlocks(5);
    }
    await waitForBlock(proposals.endBlock + 1n);

    const state = await this.governor.state(proposalId);
    expect(state).to.equal(3n);
  });

  it('should cancel', async function () {
    await this.comp.delegate(this.signers.alice.address);
    const callDatas = [ethers.AbiCoder.defaultAbiCoder().encode(['address'], [this.signers.alice.address])];
    const tx = await createTransaction(
      this.governor.propose,
      [this.signers.alice],
      ['0'],
      ['getBalanceOf(address)'],
      callDatas,
      0,
      'do nothing',
    );
    const proposal = await tx.wait();
    expect(proposal?.status).to.equal(1);
    const proposalId = await this.governor.latestProposalIds(this.signers.alice.address);
    const proposals = await this.governor.proposals(proposalId);
    if (network.name == 'hardhat') {
      await mineNBlocks(2);
    }
    await waitForBlock(proposals.startBlock + 1n);

    const state = await this.governor.state(proposalId);
    expect(state).to.equal(1n);

    const txCancel = await this.governor.cancel(proposalId);
    await txCancel.wait();
    const newState = await this.governor.state(proposalId);
    expect(newState).to.equal(2n);
  });
});
