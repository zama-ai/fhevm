import { expect } from 'chai';
import hre, { ethers, run, upgrades } from 'hardhat';

import { buildReplicaReplay } from '../../tasks/replayKmsLifecycle';
import { getRequiredEnvVar } from '../../tasks/utils/loadVariables';
import type { ProtocolConfig, ProtocolConfigReplica } from '../../types';
import { reshareKmsEpoch } from '../utils/protocolConfigEip712';
import {
  PLACEHOLDER_CANONICAL_CHAIN_ID,
  PLACEHOLDER_CANONICAL_PROTOCOL_CONFIG_ADDRESS,
  buildControllableKmsCommittee,
  captureConsoleLog,
  deployFreshEmptyUUPSProxy,
  deployFreshProtocolConfigProxy,
  upgradeEmptyProxy,
} from './taskHelpers';

const TASK_ARGS = {
  currentImplementation: 'examples/ProtocolConfigV014LayoutExample.sol:ProtocolConfigV014LayoutExample',
  newImplementation: 'contracts/ProtocolConfigReplica.sol:ProtocolConfigReplica',
  verifyContract: false,
};

describe('ProtocolConfigReplica upgrade tasks', function () {
  const deployer = new ethers.Wallet(getRequiredEnvVar('DEPLOYER_PRIVATE_KEY')).connect(ethers.provider);
  // The tasks read environment variables, so each test restores the environment.
  let envBackup: NodeJS.ProcessEnv;
  let proxyAddress: string;

  beforeEach(async function () {
    envBackup = { ...process.env };
    // A proxy with the v0.14 ProtocolConfig layout, initialized at version 3.
    proxyAddress = await deployFreshEmptyUUPSProxy(deployer);
    await upgradeEmptyProxy(proxyAddress, deployer, 'ProtocolConfigV014LayoutExample', {
      call: { fn: 'initializeFromEmptyProxy' },
    });
    process.env.PROTOCOL_CONFIG_CONTRACT_ADDRESS = proxyAddress;
  });

  afterEach(function () {
    for (const key of Object.keys(process.env)) {
      if (!(key in envBackup)) {
        delete process.env[key];
      }
    }
    Object.assign(process.env, envBackup);
  });

  it('upgrades a v0.14 proxy to a replica that accepts canonical signatures', async function () {
    const committee = await buildControllableKmsCommittee();
    const canonicalAddress = await deployFreshProtocolConfigProxy(deployer, committee.nodes, committee.thresholds);
    const canonical = (await ethers.getContractAt(
      'ProtocolConfig',
      canonicalAddress,
      deployer,
    )) as unknown as ProtocolConfig;
    const { chainId } = await ethers.provider.getNetwork();
    process.env.CANONICAL_PROTOCOL_CONFIG_ADDRESS = canonicalAddress;
    process.env.CANONICAL_CHAIN_ID = chainId.toString();

    await run('task:upgradeProtocolConfigReplica', TASK_ARGS);

    const replica = (await ethers.getContractAt(
      'ProtocolConfigReplica',
      proxyAddress,
      deployer,
    )) as unknown as ProtocolConfigReplica;
    expect(await replica.getVersion()).to.equal('ProtocolConfigReplica v0.3.0');
    const [, name, version, domainChainId, verifyingContract] = await replica.eip712Domain();
    expect([name, version, domainChainId, verifyingContract]).to.deep.equal([
      'ProtocolConfig',
      '1',
      chainId,
      canonicalAddress,
    ]);

    // The fixture proxy holds no KMS context, so the owner mirrors the first canonical one. The
    // canonical committee then signs a resharing. The replica accepts it through the submit path.
    // This proves that the reinitializer stored the canonical domain.
    const [contextId, epochId] = await canonical.getCurrentKmsContextAndEpoch();
    await (await replica.mirrorKmsContextAndEpoch(contextId, epochId, committee.nodes, committee.thresholds)).wait();
    const newEpochId = await reshareKmsEpoch(canonical, committee.signerSigners);

    const { calls } = await buildReplicaReplay(hre, {
      canonicalProvider: ethers.provider,
      canonicalProtocolConfigAddress: canonicalAddress,
      replicaAddress: proxyAddress,
      fromBlock: 0,
    });
    expect(calls).to.have.lengthOf(1);
    await (await deployer.sendTransaction({ to: proxyAddress, data: calls[0].calldata })).wait();
    expect(await replica.getCurrentKmsContextAndEpoch()).to.deep.equal([contextId, newEpochId]);
  });

  it('prepares the upgrade without changing the proxy implementation', async function () {
    process.env.CANONICAL_PROTOCOL_CONFIG_ADDRESS = PLACEHOLDER_CANONICAL_PROTOCOL_CONFIG_ADDRESS;
    process.env.CANONICAL_CHAIN_ID = PLACEHOLDER_CANONICAL_CHAIN_ID.toString();
    const implementationBefore = await upgrades.erc1967.getImplementationAddress(proxyAddress);

    const lines = await captureConsoleLog(() => run('task:prepareUpgradeProtocolConfigReplica', TASK_ARGS));

    const replicaFactory = await ethers.getContractFactory('ProtocolConfigReplica');
    const reinitializeCalldata = replicaFactory.interface.encodeFunctionData('reinitializeV3', [
      PLACEHOLDER_CANONICAL_PROTOCOL_CONFIG_ADDRESS,
      PLACEHOLDER_CANONICAL_CHAIN_ID,
    ]);
    expect(lines).to.include(`reinitializeV3 calldata: ${reinitializeCalldata}`);
    expect(await upgrades.erc1967.getImplementationAddress(proxyAddress)).to.equal(implementationBefore);
  });

  it('rejects an invalid canonical address before any deployment', async function () {
    process.env.CANONICAL_PROTOCOL_CONFIG_ADDRESS = '0x1234';
    process.env.CANONICAL_CHAIN_ID = '1';

    for (const taskName of ['task:upgradeProtocolConfigReplica', 'task:prepareUpgradeProtocolConfigReplica']) {
      await expect(run(taskName, TASK_ARGS)).to.be.rejectedWith(
        /"CANONICAL_PROTOCOL_CONFIG_ADDRESS" env variable must be a valid address/,
      );
    }
  });
});
