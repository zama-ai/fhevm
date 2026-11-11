import { expect } from 'chai';
import hre, { ethers } from 'hardhat';

import {
  assertCanonicalCoprocessorContextDestroyed,
  assertReplicaNeedsCoprocessorContextDestructionMirror,
  assertReplicaNeedsCoprocessorContextMirror,
  encodeMirrorCoprocessorContext,
  encodeMirrorCoprocessorContextDestruction,
  readCanonicalCoprocessorContext,
} from '../../tasks/mirrorCoprocessorContext';
import { getCoprocessorConfigReplicaInterface } from '../../tasks/utils/coprocessorConfigTasks';
import { COPROCESSOR_CONTEXT_COUNTER_BASE } from '../../tasks/utils/kmsGenerationConstants';
import { getRequiredEnvVar } from '../../tasks/utils/loadVariables';
import type { CoprocessorConfig, CoprocessorConfigReplica } from '../../types';
import {
  COPROCESSOR_MAJORITY_THRESHOLD,
  buildCoprocessors,
  deployFreshCoprocessorConfigProxy,
  deployFreshCoprocessorConfigReplicaProxy,
} from './taskHelpers';

// These tests drive the mirroring helpers directly: `ethers.provider` stands in for the canonical
// RPC provider. The CLI layer itself (`--canonical-rpc-url` constructing a real JsonRpcProvider) is
// untested.
describe('Coprocessor context mirror tasks', function () {
  const deployer = new ethers.Wallet(getRequiredEnvVar('DEPLOYER_PRIVATE_KEY')).connect(ethers.provider);
  const FIRST_COPROCESSOR_CONTEXT_ID = COPROCESSOR_CONTEXT_COUNTER_BASE + 1n;
  const SECOND_COPROCESSOR_CONTEXT_ID = FIRST_COPROCESSOR_CONTEXT_ID + 1n;

  // A second set, distinct from the first context's, so a mirrored context is told apart by content.
  const newCoprocessors = buildCoprocessors()
    .slice(0, 2)
    .map((coprocessor, i) => ({
      ...coprocessor,
      txSenderAddress: ethers.getAddress(coprocessor.txSenderAddress),
      signerAddress: ethers.getAddress(coprocessor.signerAddress),
      storageUrl: `https://new-coprocessor${i}.example.com`,
    }));
  const newMajorityThreshold = 2n;

  async function deployCanonical(): Promise<CoprocessorConfig> {
    const address = await deployFreshCoprocessorConfigProxy(
      deployer,
      buildCoprocessors(),
      COPROCESSOR_MAJORITY_THRESHOLD,
    );
    return (await ethers.getContractAt('CoprocessorConfig', address, deployer)) as unknown as CoprocessorConfig;
  }

  async function deployReplica(): Promise<CoprocessorConfigReplica> {
    const address = await deployFreshCoprocessorConfigReplicaProxy(
      deployer,
      COPROCESSOR_CONTEXT_COUNTER_BASE + 1n,
      buildCoprocessors(),
      COPROCESSOR_MAJORITY_THRESHOLD,
    );
    return (await ethers.getContractAt(
      'CoprocessorConfigReplica',
      address,
      deployer,
    )) as unknown as CoprocessorConfigReplica;
  }

  async function defineNewContext(canonical: CoprocessorConfig): Promise<void> {
    await (await canonical.defineNewCoprocessorContext(newCoprocessors, newMajorityThreshold)).wait();
  }

  describe('readCanonicalCoprocessorContext and encodeMirrorCoprocessorContext', function () {
    it("reads canonical's current context and builds calldata that decodes back to it", async function () {
      const canonical = await deployCanonical();
      await defineNewContext(canonical);

      const args = await readCanonicalCoprocessorContext(hre, {
        canonicalProvider: ethers.provider,
        canonicalCoprocessorConfigAddress: await canonical.getAddress(),
      });
      expect(args.contextId).to.equal(SECOND_COPROCESSOR_CONTEXT_ID);
      expect(args.majorityThreshold).to.equal(newMajorityThreshold);
      expect(args.coprocessors).to.deep.equal(newCoprocessors);

      const iface = await getCoprocessorConfigReplicaInterface(hre);
      const decoded = iface.decodeFunctionData('mirrorCoprocessorContext', encodeMirrorCoprocessorContext(iface, args));
      expect(decoded[0]).to.equal(args.contextId);
      expect(decoded[1].length).to.equal(newCoprocessors.length);
      newCoprocessors.forEach((coprocessor, i) => {
        expect(decoded[1][i].txSenderAddress).to.equal(coprocessor.txSenderAddress);
        expect(decoded[1][i].signerAddress).to.equal(coprocessor.signerAddress);
        expect(decoded[1][i].storageUrl).to.equal(coprocessor.storageUrl);
        expect(decoded[1][i].prfEndpointUrl).to.equal(coprocessor.prfEndpointUrl);
      });
      expect(decoded[2]).to.equal(args.majorityThreshold);
    });

    it('encodes the destroyed context ID for mirrorCoprocessorContextDestruction', async function () {
      const iface = await getCoprocessorConfigReplicaInterface(hre);
      const calldata = encodeMirrorCoprocessorContextDestruction(iface, FIRST_COPROCESSOR_CONTEXT_ID);
      const decoded = iface.decodeFunctionData('mirrorCoprocessorContextDestruction', calldata);
      expect(decoded[0]).to.equal(FIRST_COPROCESSOR_CONTEXT_ID);
    });
  });

  describe('readiness guards', function () {
    it('assertReplicaNeedsCoprocessorContextMirror throws once the replica is at or past the canonical context', async function () {
      const replicaAddress = await (await deployReplica()).getAddress();

      await expect(
        assertReplicaNeedsCoprocessorContextMirror(hre, replicaAddress, FIRST_COPROCESSOR_CONTEXT_ID),
      ).to.be.rejectedWith(/is already at coprocessor context/);
      await expect(assertReplicaNeedsCoprocessorContextMirror(hre, replicaAddress, SECOND_COPROCESSOR_CONTEXT_ID)).to
        .not.be.rejected;
    });

    it('assertCanonicalCoprocessorContextDestroyed only accepts a destroyed past context', async function () {
      const canonical = await deployCanonical();
      await defineNewContext(canonical);
      const check = (coprocessorContextId: bigint) =>
        canonical.getAddress().then((canonicalCoprocessorConfigAddress) =>
          assertCanonicalCoprocessorContextDestroyed(hre, {
            canonicalProvider: ethers.provider,
            canonicalCoprocessorConfigAddress,
            coprocessorContextId,
          }),
        );

      await expect(check(COPROCESSOR_CONTEXT_COUNTER_BASE)).to.be.rejectedWith(/is not a past context on canonical/);
      await expect(check(SECOND_COPROCESSOR_CONTEXT_ID)).to.be.rejectedWith(/is not a past context on canonical/);
      await expect(check(FIRST_COPROCESSOR_CONTEXT_ID)).to.be.rejectedWith(/is not destroyed on canonical/);

      await (await canonical.destroyCoprocessorContext(FIRST_COPROCESSOR_CONTEXT_ID)).wait();
      await expect(check(FIRST_COPROCESSOR_CONTEXT_ID)).to.not.be.rejected;
    });

    it('assertReplicaNeedsCoprocessorContextDestructionMirror throws on the current, an unknown or a destroyed context', async function () {
      const replica = await deployReplica();
      const replicaAddress = await replica.getAddress();
      const check = (coprocessorContextId: bigint) =>
        assertReplicaNeedsCoprocessorContextDestructionMirror(hre, replicaAddress, coprocessorContextId);

      await expect(check(FIRST_COPROCESSOR_CONTEXT_ID)).to.be.rejectedWith(/Run task:mirrorCoprocessorContext first/);

      await (
        await replica.mirrorCoprocessorContext(SECOND_COPROCESSOR_CONTEXT_ID, newCoprocessors, newMajorityThreshold)
      ).wait();
      await expect(check(FIRST_COPROCESSOR_CONTEXT_ID)).to.not.be.rejected;
      await expect(check(SECOND_COPROCESSOR_CONTEXT_ID + 5n)).to.be.rejectedWith(/is unknown or already destroyed/);

      await (await replica.mirrorCoprocessorContextDestruction(FIRST_COPROCESSOR_CONTEXT_ID)).wait();
      await expect(check(FIRST_COPROCESSOR_CONTEXT_ID)).to.be.rejectedWith(/is unknown or already destroyed/);
    });
  });

  describe('end-to-end mirror onto a replica', function () {
    it('mirrors a new coprocessor context, then the destruction of the previous one', async function () {
      const canonical = await deployCanonical();
      const canonicalCoprocessorConfigAddress = await canonical.getAddress();
      const replica = await deployReplica();
      const replicaAddress = await replica.getAddress();
      const iface = await getCoprocessorConfigReplicaInterface(hre);

      await defineNewContext(canonical);
      const args = await readCanonicalCoprocessorContext(hre, {
        canonicalProvider: ethers.provider,
        canonicalCoprocessorConfigAddress,
      });
      await assertReplicaNeedsCoprocessorContextMirror(hre, replicaAddress, args.contextId);
      await (
        await deployer.sendTransaction({ to: replicaAddress, data: encodeMirrorCoprocessorContext(iface, args) })
      ).wait();

      expect(await replica.getCurrentCoprocessorContextId()).to.equal(SECOND_COPROCESSOR_CONTEXT_ID);
      expect(await replica.getCoprocessorMajorityThresholdForContext(SECOND_COPROCESSOR_CONTEXT_ID)).to.equal(
        newMajorityThreshold,
      );
      const mirrored = await replica.getCoprocessorsForContext(SECOND_COPROCESSOR_CONTEXT_ID);
      expect(mirrored.map((coprocessor) => coprocessor.storageUrl)).to.deep.equal(
        newCoprocessors.map((coprocessor) => coprocessor.storageUrl),
      );

      await (await canonical.destroyCoprocessorContext(FIRST_COPROCESSOR_CONTEXT_ID)).wait();
      await assertCanonicalCoprocessorContextDestroyed(hre, {
        canonicalProvider: ethers.provider,
        canonicalCoprocessorConfigAddress,
        coprocessorContextId: FIRST_COPROCESSOR_CONTEXT_ID,
      });
      await assertReplicaNeedsCoprocessorContextDestructionMirror(hre, replicaAddress, FIRST_COPROCESSOR_CONTEXT_ID);
      await (
        await deployer.sendTransaction({
          to: replicaAddress,
          data: encodeMirrorCoprocessorContextDestruction(iface, FIRST_COPROCESSOR_CONTEXT_ID),
        })
      ).wait();

      expect(await replica.isValidCoprocessorContext(FIRST_COPROCESSOR_CONTEXT_ID)).to.equal(false);
      expect(await replica.isValidCoprocessorContext(SECOND_COPROCESSOR_CONTEXT_ID)).to.equal(true);
    });
  });
});
