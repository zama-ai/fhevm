import { expect } from 'chai';
import type { Signer } from 'ethers';
import hre, { ethers } from 'hardhat';

import { type CanonicalLifecycleEvents, buildReplicaReplay, planReplicaReplay } from '../../tasks/replayKmsLifecycle';
import { getRequiredEnvVar } from '../../tasks/utils/loadVariables';
import type { ProtocolConfig, ProtocolConfigReplica } from '../../types';
import {
  activateDefinedKmsContext,
  confirmContextCreation,
  confirmContextDestruction,
  confirmEpochDestruction,
  defineNewKmsContext,
  reshareKmsEpoch,
} from '../utils/protocolConfigEip712';
import { buildKmsNode, deployFreshProtocolConfigProxy, deployFreshProtocolConfigReplicaProxy } from './taskHelpers';

const thresholds = (mpc: bigint) => ({ publicDecryption: 1n, userDecryption: 1n, kmsGen: 1n, mpc });

// A committee of the given signer accounts. No test sends from a tx-sender, so each tx-sender address is random.
async function buildCommittee(signers: Signer[]) {
  return Promise.all(
    signers.map(async (signer, index) =>
      buildKmsNode(ethers.Wallet.createRandom().address, await signer.getAddress(), index),
    ),
  );
}

describe('KMS lifecycle replay task', function () {
  const deployer = new ethers.Wallet(getRequiredEnvVar('DEPLOYER_PRIVATE_KEY')).connect(ethers.provider);

  // Deploys a canonical ProtocolConfig and a replica. The replica starts on the first canonical
  // context and epoch. It verifies signatures under the canonical domain.
  async function deployCanonicalAndReplica(initialSigners: Signer[]) {
    const nodes = await buildCommittee(initialSigners);
    const canonicalAddress = await deployFreshProtocolConfigProxy(deployer, nodes, thresholds(1n));
    const canonical = (await ethers.getContractAt(
      'ProtocolConfig',
      canonicalAddress,
      deployer,
    )) as unknown as ProtocolConfig;
    const [contextId, epochId] = await canonical.getCurrentKmsContextAndEpoch();
    const { chainId } = await ethers.provider.getNetwork();
    const replicaAddress = await deployFreshProtocolConfigReplicaProxy(
      deployer,
      contextId,
      epochId,
      nodes,
      thresholds(1n),
      canonicalAddress,
      chainId,
    );
    const replica = (await ethers.getContractAt(
      'ProtocolConfigReplica',
      replicaAddress,
    )) as unknown as ProtocolConfigReplica;
    const replay = (fromBlock = 0) =>
      buildReplicaReplay(hre, {
        canonicalProvider: ethers.provider,
        canonicalProtocolConfigAddress: canonicalAddress,
        replicaAddress,
        fromBlock,
      });
    // Anyone can submit. This sender is not the owner.
    const submit = async (calls: readonly { calldata: string }[]) => {
      const submitter = (await ethers.getSigners())[19];
      for (const { calldata } of calls) {
        await (await submitter.sendTransaction({ to: replicaAddress, data: calldata })).wait();
      }
    };
    return { canonical, replica, contextId, epochId, replay, submit };
  }

  it('brings a replica three rotations behind up to date from canonical events alone', async function () {
    const [, a0, a1, a2, a3, b3, c0, c1] = await ethers.getSigners();
    // The first committee has four signers and the quorum n - t = 3.
    const {
      canonical,
      replica,
      contextId: context1,
      replay,
      submit,
    } = await deployCanonicalAndReplica([a0, a1, a2, a3]);

    // An aborted context switch. It leaves a gap in the context IDs, confirmations with no activation,
    // and a destruction quorum for a context that the replica never stored.
    const aborted = await defineNewKmsContext(canonical, await buildCommittee([a0, a1, a2, b3]), thresholds(1n));
    await confirmContextCreation(canonical, [a0], aborted);
    await (await canonical.destroyKmsContext(aborted.contextId)).wait();
    await confirmContextDestruction(canonical, [a0, a1, a2], aborted.contextId);

    // Rotation 1: a context switch to a committee with the new signer b3 and the quorum n - t = 2. The
    // previous signer a3 confirms another extraData first, so the quorum forms on the extraData of
    // the other signers.
    const context3 = await defineNewKmsContext(canonical, await buildCommittee([a0, a1, a2, b3]), thresholds(2n));
    await confirmContextCreation(canonical, [a3], context3, '0x01');
    await activateDefinedKmsContext(canonical, [b3, a0, a1, a2], context3);
    const [, epoch2] = await canonical.getCurrentKmsContextAndEpoch();

    // Rotation 2: a same-set resharing.
    const epoch3 = await reshareKmsEpoch(canonical, [a0, a1, a2, b3]);

    // Two signers of the active committee, which is its quorum, confirm the destruction of a
    // superseded epoch and of the first context.
    await (await canonical.destroyKmsEpoch(epoch2)).wait();
    await confirmEpochDestruction(canonical, [a0, b3], epoch2);
    await (await canonical.destroyKmsContext(context1)).wait();
    await confirmContextDestruction(canonical, [a1, b3], context1);

    // Rotation 3: a context switch to a committee that shares no signer with the previous one.
    const context4 = await defineNewKmsContext(canonical, await buildCommittee([c0, c1]), thresholds(1n));
    await confirmContextCreation(canonical, [a0, b3], context4);
    await activateDefinedKmsContext(canonical, [c0, c1], context4);

    const { calls, skipped } = await replay();
    expect(calls.map((call) => call.functionName)).to.deep.equal([
      'submitKmsContextAndEpoch',
      'submitKmsEpoch',
      'submitKmsEpochDestruction',
      'submitKmsContextDestruction',
      'submitKmsContextAndEpoch',
    ]);
    expect([calls[2].args[0], calls[3].args[0]]).to.deep.equal([epoch2, context1]);
    // Of the five creation confirmations of rotation 1, a0, a1 and a2 are previous signers on the quorum extraData.
    const [rotation1] = calls;
    if (rotation1.functionName !== 'submitKmsContextAndEpoch') {
      throw new Error(`Unexpected first call ${rotation1.functionName}`);
    }
    expect(rotation1.args[0].extraData).to.equal('0x');
    expect(rotation1.args[0].signatures).to.have.lengthOf(3);
    expect(skipped).to.deep.equal([{ target: 'context', destroyedId: aborted.contextId, reason: 'notLive' }]);

    await submit(calls);

    expect(await replica.getCurrentKmsContextAndEpoch()).to.deep.equal(await canonical.getCurrentKmsContextAndEpoch());
    expect(await replica.getCurrentKmsContextId()).to.equal(context4.contextId);
    for (const contextId of [context3.contextId, context4.contextId]) {
      expect(await replica.getKmsNodesForContext(contextId)).to.deep.equal(
        await canonical.getKmsNodesForContext(contextId),
      );
      expect(await replica.getMpcThresholdForContext(contextId)).to.equal(
        await canonical.getMpcThresholdForContext(contextId),
      );
      expect(await replica.getPublicDecryptionThresholdForContext(contextId)).to.equal(
        await canonical.getPublicDecryptionThresholdForContext(contextId),
      );
    }
    expect(await replica.isLiveKmsContext(context1)).to.equal(false);
    expect(await replica.isValidEpochForContext(context3.contextId, epoch2)).to.equal(false);
    expect(await replica.isValidEpochForContext(context3.contextId, epoch3)).to.equal(true);

    // The replica is up to date, so a second replay has nothing to send.
    expect((await replay()).calls).to.deep.equal([]);
  });

  it('replays the destruction of an epoch that is already Active on the replica', async function () {
    const [, a0, a1] = await ethers.getSigners();
    const { canonical, replica, contextId, replay, submit } = await deployCanonicalAndReplica([a0, a1]);
    const epoch2 = await reshareKmsEpoch(canonical, [a0, a1]);
    await reshareKmsEpoch(canonical, [a0, a1]);
    await submit((await replay()).calls);
    expect(await replica.isValidEpochForContext(contextId, epoch2)).to.equal(true);

    await (await canonical.destroyKmsEpoch(epoch2)).wait();
    await confirmEpochDestruction(canonical, [a0], epoch2);

    const { calls, skipped } = await replay();
    expect(calls.map((call) => [call.functionName, call.args[0]])).to.deep.equal([
      ['submitKmsEpochDestruction', epoch2],
    ]);
    expect(skipped).to.deep.equal([]);
    await submit(calls);
    expect(await replica.isValidEpochForContext(contextId, epoch2)).to.equal(false);
  });

  it('rejects a scan that does not reach the state of the replica', async function () {
    const [, a0, a1] = await ethers.getSigners();
    const { canonical, replay } = await deployCanonicalAndReplica([a0, a1]);
    await reshareKmsEpoch(canonical, [a0, a1]);
    const afterFirstResharing = (await ethers.provider.getBlockNumber()) + 1;
    const epoch3 = await reshareKmsEpoch(canonical, [a0, a1]);

    await expect(replay(afterFirstResharing)).to.be.rejectedWith(
      new RegExp(`Cannot replay epoch ${epoch3}: .* Pass a lower --from-block`),
    );
    expect((await replay()).calls.map((call) => call.functionName)).to.deep.equal(['submitKmsEpoch', 'submitKmsEpoch']);
  });

  it('rejects a scan that starts after the last canonical activation', async function () {
    const [, a0, a1] = await ethers.getSigners();
    const { canonical, epochId, replay } = await deployCanonicalAndReplica([a0, a1]);
    const epoch2 = await reshareKmsEpoch(canonical, [a0, a1]);

    await expect(replay((await ethers.provider.getBlockNumber()) + 1)).to.be.rejectedWith(
      new RegExp(
        `The replay ends at .* epoch ${epochId}, but canonical is at .* epoch ${epoch2} .* Pass a lower --from-block`,
      ),
    );
  });

  it('rejects a scan that holds only the destruction of the active context of the replica', async function () {
    const [, a0, a1, b1] = await ethers.getSigners();
    const { canonical, contextId: context1, replay } = await deployCanonicalAndReplica([a0, a1]);
    const context2 = await defineNewKmsContext(canonical, await buildCommittee([a0, b1]), thresholds(1n));
    await activateDefinedKmsContext(canonical, [a0, b1], context2);
    const afterContextSwitch = (await ethers.provider.getBlockNumber()) + 1;
    await (await canonical.destroyKmsContext(context1)).wait();
    await confirmContextDestruction(canonical, [a0], context1);

    await expect(replay(afterContextSwitch)).to.be.rejectedWith(
      new RegExp(`The replay ends at context ${context1} .* but canonical is at context ${context2.contextId} `),
    );
  });

  describe('planReplicaReplay', function () {
    const NO_EVENTS: CanonicalLifecycleEvents = {
      newContexts: [],
      newEpochs: [],
      epochActivations: [],
      contextCreationConfirmations: [],
      epochActivationConfirmations: [],
      contextDestructionConfirmations: [],
      contextDestructionQuorums: [],
      epochDestructionConfirmations: [],
      epochDestructionQuorums: [],
    };
    // A replica on context 2, with three signers and the quorum n - t = 2. Context 1 is still live.
    const REPLICA = {
      contextId: 2n,
      epochId: 5n,
      signers: ['0xA', '0xB', '0xC'],
      mpcThreshold: 1n,
      liveContextIds: new Set([1n]),
      activeEpochIds: new Set<bigint>(),
    };
    const confirmation = (signer: string, extraData = '0x') => ({
      destroyedId: 1n,
      signer,
      signature: `sig-${signer}`,
      extraData,
    });
    const quorum = { blockNumber: 10, logIndex: 0, destroyedId: 1n, extraData: '0x' };

    it('keeps only the destruction signatures of the active committee on the quorum extraData', function () {
      const plan = planReplicaReplay(
        {
          ...NO_EVENTS,
          contextDestructionConfirmations: [
            confirmation('0xA'),
            confirmation('0xB', '0x01'),
            confirmation('0xC'),
            confirmation('0xOld'),
          ],
          contextDestructionQuorums: [quorum],
        },
        REPLICA,
      );

      expect(plan).to.deep.equal({
        _tag: 'ok',
        calls: [{ functionName: 'submitKmsContextDestruction', args: [1n, ['sig-0xA', 'sig-0xC'], '0x'] }],
        skipped: [],
        contextId: 2n,
        epochId: 5n,
      });
    });

    it('skips a destruction that the active committee signed below its quorum', function () {
      const plan = planReplicaReplay(
        {
          ...NO_EVENTS,
          contextDestructionConfirmations: [confirmation('0xA'), confirmation('0xOld')],
          contextDestructionQuorums: [quorum],
        },
        REPLICA,
      );

      expect(plan).to.deep.include({
        calls: [],
        skipped: [{ target: 'context', destroyedId: 1n, reason: 'belowQuorum', signatureCount: 1, quorum: 2 }],
      });
    });

    it('skips an epoch destruction when the replica does not report the epoch as valid', function () {
      const plan = planReplicaReplay(
        {
          ...NO_EVENTS,
          epochDestructionConfirmations: [confirmation('0xA'), confirmation('0xB')],
          epochDestructionQuorums: [quorum],
        },
        REPLICA,
      );

      expect(plan).to.deep.include({
        calls: [],
        skipped: [{ target: 'epoch', destroyedId: 1n, reason: 'unknownState' }],
      });
    });
  });
});
