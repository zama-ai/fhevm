import { expect } from 'chai';
import fs from 'fs';
import hre, { ethers, run } from 'hardhat';

import {
  buildCanonicalUpgradeProposal,
  buildSnapshotArtifact,
  parseSnapshotArtifact,
  readCanonicalSnapshot,
} from '../../tasks/protocolConfigMirror';
import {
  CRS_COUNTER_BASE,
  KEY_COUNTER_BASE,
  KMS_CONTEXT_COUNTER_BASE,
  PREP_KEYGEN_COUNTER_BASE,
} from '../../tasks/utils/kmsGenerationConstants';
import { getRequiredEnvVar } from '../../tasks/utils/loadVariables';
import { executeUpgradeProposal } from '../../tasks/utils/upgradeProposal';
import type { KMSGeneration, ProtocolConfig } from '../../types';
import {
  HOST_ADDRESSES_SOL_FILE,
  HOST_ENV_FILE,
  buildControllableKmsCommittee,
  buildProtocolConfigNodes,
  buildProtocolConfigThresholds,
  deployFreshEmptyUUPSProxy,
  deployFreshKMSGenerationProxy,
  deployFreshProtocolConfigProxy,
  deployFreshUninitializedProtocolConfigProxy,
  readHostAddress,
  rotateToNewKmsContext,
} from './taskHelpers';

describe('task:deployAllHostContracts', function () {
  const canonicalSnapshotEnv = {
    CANONICAL_KMS_CONTEXT_ID: (KMS_CONTEXT_COUNTER_BASE + 1n).toString(),
    CANONICAL_PROTOCOL_CONFIG_ADDRESS: '0x0000000000000000000000000000000000C0FFEE',
    KMS_PCR_VALUES: '[]',
    KMS_SOFTWARE_VERSION: 'kms-v1',
  };
  let previousEnv: Partial<Record<keyof typeof canonicalSnapshotEnv, string | undefined>>;
  let previousSolidityCoverage: string | undefined;
  let originalEnvHost: string;
  let originalAddressesSol: string;

  beforeEach(function () {
    previousEnv = {};
    previousSolidityCoverage = process.env.SOLIDITY_COVERAGE;
    // Snapshot .env.host: the fresh-deploy test rewrites PROTOCOL_CONFIG_CONTRACT_ADDRESS.
    originalEnvHost = fs.readFileSync(HOST_ENV_FILE, 'utf-8');
    // Snapshot FHEVMHostAddresses.sol: the withKmsGeneration=false path regenerates this shared
    // file without kmsGenerationAdd, which would break the subsequent `forge test` compile of
    // contracts that unconditionally import that constant.
    originalAddressesSol = fs.readFileSync(HOST_ADDRESSES_SOL_FILE, 'utf-8');
    for (const [key, value] of Object.entries(canonicalSnapshotEnv)) {
      const envKey = key as keyof typeof canonicalSnapshotEnv;
      previousEnv[envKey] = process.env[envKey];
      process.env[envKey] = value;
    }
  });

  afterEach(function () {
    for (const key of Object.keys(canonicalSnapshotEnv) as (keyof typeof canonicalSnapshotEnv)[]) {
      const previousValue = previousEnv[key];
      if (previousValue === undefined) {
        delete process.env[key];
      } else {
        process.env[key] = previousValue;
      }
    }
    if (previousSolidityCoverage === undefined) {
      delete process.env.SOLIDITY_COVERAGE;
    } else {
      process.env.SOLIDITY_COVERAGE = previousSolidityCoverage;
    }
    fs.writeFileSync(HOST_ENV_FILE, originalEnvHost);
    fs.writeFileSync(HOST_ADDRESSES_SOL_FILE, originalAddressesSol);
  });

  it('requires the KMSGeneration deployment role to be explicit', async function () {
    await expect(run('task:deployAllHostContracts')).to.be.rejectedWith(/withKmsGeneration/);
  });

  it('rejects an invalid --protocol-config-source value before mutating state', async function () {
    await expect(
      run('task:deployAllHostContracts', { withKmsGeneration: false, protocolConfigSource: 'bogus' }),
    ).to.be.rejectedWith(/Invalid --protocol-config-source "bogus"\. Allowed values: fresh, canonical\./);
  });

  it('rejects --protocol-config-source canonical on a canonical host', async function () {
    await expect(
      run('task:deployAllHostContracts', { withKmsGeneration: true, protocolConfigSource: 'canonical' }),
    ).to.be.rejectedWith(/cannot be combined with --with-kms-generation true/);
  });

  it('rejects --protocol-config-source canonical without the canonical chain parameters', async function () {
    await expect(
      run('task:deployAllHostContracts', { withKmsGeneration: false, protocolConfigSource: 'canonical' }),
    ).to.be.rejectedWith(/requires --canonical-rpc-url and --canonical-protocol-config-address/);
  });

  it('deploys a fresh non-canonical host without a KMSGeneration proxy', async function () {
    process.env.SOLIDITY_COVERAGE = 'true';
    await run('task:deployAllHostContracts', { withKmsGeneration: false, protocolConfigSource: 'fresh' });

    const protocolConfig = await ethers.getContractAt(
      'ProtocolConfig',
      readHostAddress('PROTOCOL_CONFIG_CONTRACT_ADDRESS'),
    );

    expect(await protocolConfig.getVersion()).to.equal('ProtocolConfig v0.2.0');
    expect(await protocolConfig.getCurrentKmsContextId()).to.equal(KMS_CONTEXT_COUNTER_BASE + 1n);
  });
});

describe('task:deployEmptyUUPSProxies', function () {
  it('requires the KMSGeneration deployment role to be explicit', async function () {
    await expect(run('task:deployEmptyUUPSProxies')).to.be.rejectedWith(/withKmsGeneration/);
  });
});

describe('task:assertNoPendingKeyManagementRequest', function () {
  const deployerPrivateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
  const deployer = new ethers.Wallet(deployerPrivateKey).connect(ethers.provider);
  let kmsGeneration: KMSGeneration;
  let kmsGenerationAddress: string;

  beforeEach(async function () {
    kmsGeneration = await deployFreshKMSGenerationProxy(deployer);
    kmsGenerationAddress = await kmsGeneration.getAddress();
  });

  it('passes for a freshly initialized proxy', async function () {
    await run('task:assertNoPendingKeyManagementRequest', { address: kmsGenerationAddress });
  });

  it('rejects a wrong code-bearing address via the getVersion identity check', async function () {
    const protocolConfigAddress = readHostAddress('PROTOCOL_CONFIG_CONTRACT_ADDRESS');

    await expect(
      run('task:assertNoPendingKeyManagementRequest', { address: protocolConfigAddress }),
    ).to.be.rejectedWith(
      `Contract at ${protocolConfigAddress} reports version "ProtocolConfig v0.2.0"; expected "KMSGeneration v…".`,
    );
  });

  it('rejects when keygen is pending', async function () {
    await kmsGeneration.keygen(0, 0, 0);

    await expect(run('task:assertNoPendingKeyManagementRequest', { address: kmsGenerationAddress })).to.be.rejectedWith(
      `Keygen pending on ${kmsGenerationAddress}: keyCounter=${KEY_COUNTER_BASE + 1n} has not completed (isRequestDone=false). Complete or abort before proposing a new key management request.`,
    );
  });

  it('rejects when CRS generation is pending', async function () {
    await kmsGeneration.crsgenRequest(4096, 0);

    await expect(run('task:assertNoPendingKeyManagementRequest', { address: kmsGenerationAddress })).to.be.rejectedWith(
      `CRS generation pending on ${kmsGenerationAddress}: crsCounter=${CRS_COUNTER_BASE + 1n} has not completed (isRequestDone=false). Complete or abort before proposing a new key management request.`,
    );
  });

  it('passes again after aborting the pending key request', async function () {
    await kmsGeneration.keygen(0, 0, 0);
    await kmsGeneration.abortKeygen(PREP_KEYGEN_COUNTER_BASE + 1n);

    await run('task:assertNoPendingKeyManagementRequest', { address: kmsGenerationAddress });
  });

  it('passes again after aborting the pending CRS request', async function () {
    await kmsGeneration.crsgenRequest(4096, 0);
    await kmsGeneration.abortCrsgen(CRS_COUNTER_BASE + 1n);

    await run('task:assertNoPendingKeyManagementRequest', { address: kmsGenerationAddress });
  });
});

describe('canonical snapshot apply (canonical → secondary deploy flow)', function () {
  const deployerPrivateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
  const deployer = new ethers.Wallet(deployerPrivateKey).connect(ethers.provider);

  // The composition task:deployProtocolConfigFromCanonical performs in live-read mode: the DAO
  // prepare step, then direct execution of the produced payload.
  async function readAndApply(canonicalProtocolConfigAddress: string, secondaryProxyAddress: string) {
    const snapshot = await readCanonicalSnapshot(hre, {
      canonicalProvider: ethers.provider,
      canonicalProtocolConfigAddress,
    });
    const prepared = await buildCanonicalUpgradeProposal(hre, { snapshot, proxyAddress: secondaryProxyAddress });
    await executeUpgradeProposal(hre, prepared);
    return snapshot;
  }

  it('mirrors the canonical ProtocolConfig snapshot onto a fresh secondary proxy', async function () {
    const canonicalNodes = buildProtocolConfigNodes();
    const canonicalThresholds = buildProtocolConfigThresholds();
    const canonicalAddress = await deployFreshProtocolConfigProxy(deployer, canonicalNodes, canonicalThresholds);
    const canonical = (await ethers.getContractAt(
      'ProtocolConfig',
      canonicalAddress,
      deployer,
    )) as unknown as ProtocolConfig;

    const secondaryProxyAddress = await deployFreshEmptyUUPSProxy(deployer);

    expect(canonicalAddress).to.not.equal(secondaryProxyAddress);

    const blockBeforeMirror = await ethers.provider.getBlockNumber();
    const snapshot = await readAndApply(canonicalAddress, secondaryProxyAddress);
    const blockAfterMirror = await ethers.provider.getBlockNumber();

    const secondary = (await ethers.getContractAt(
      'ProtocolConfig',
      secondaryProxyAddress,
      deployer,
    )) as unknown as ProtocolConfig;

    const canonicalContextId = await canonical.getCurrentKmsContextId();
    const secondaryContextId = await secondary.getCurrentKmsContextId();
    expect(snapshot.currentKmsContextId).to.equal(canonicalContextId);
    expect(secondaryContextId).to.equal(canonicalContextId);
    const secondaryState = await secondary.getCurrentKmsContextAndEpoch();
    expect(secondaryState[0]).to.equal(snapshot.currentKmsContextId);
    expect(secondaryState[1]).to.equal(snapshot.currentEpochId);

    expect(snapshot.canonicalChainId).to.equal((await ethers.provider.getNetwork()).chainId);
    expect(snapshot.blockNumber).to.be.gte(blockBeforeMirror);
    expect(snapshot.blockNumber).to.be.lte(blockAfterMirror);

    const canonicalNodesOnChain = await canonical.getKmsNodesForContext(canonicalContextId);
    const secondaryNodesOnChain = await secondary.getKmsNodesForContext(secondaryContextId);
    expect(secondaryNodesOnChain.length).to.equal(canonicalNodesOnChain.length);
    expect(secondaryNodesOnChain.length).to.equal(canonicalNodes.length);
    for (let i = 0; i < canonicalNodes.length; i += 1) {
      expect(secondaryNodesOnChain[i].txSenderAddress).to.equal(canonicalNodesOnChain[i].txSenderAddress);
      expect(secondaryNodesOnChain[i].signerAddress).to.equal(canonicalNodesOnChain[i].signerAddress);
      expect(secondaryNodesOnChain[i].ipAddress).to.equal(canonicalNodesOnChain[i].ipAddress);
      expect(secondaryNodesOnChain[i].storageUrl).to.equal(canonicalNodesOnChain[i].storageUrl);
    }

    expect(await secondary.getPublicDecryptionThresholdForContext(secondaryContextId)).to.equal(
      await canonical.getPublicDecryptionThresholdForContext(canonicalContextId),
    );
    expect(await secondary.getUserDecryptionThresholdForContext(secondaryContextId)).to.equal(
      await canonical.getUserDecryptionThresholdForContext(canonicalContextId),
    );
    expect(await secondary.getKmsGenThresholdForContext(secondaryContextId)).to.equal(
      await canonical.getKmsGenThresholdForContext(canonicalContextId),
    );
    expect(await secondary.getMpcThresholdForContext(secondaryContextId)).to.equal(
      await canonical.getMpcThresholdForContext(canonicalContextId),
    );

    expect(await secondary.isValidKmsContext(secondaryContextId)).to.equal(true);
  });

  it('pins canonical reads to a historical block under a rotation', async function () {
    // A controllable committee lets us drive the epoch lifecycle so the active context actually advances.
    const committee = await buildControllableKmsCommittee();
    const canonicalAddress = await deployFreshProtocolConfigProxy(deployer, committee.nodes, committee.thresholds);
    const canonical = (await ethers.getContractAt(
      'ProtocolConfig',
      canonicalAddress,
      deployer,
    )) as unknown as ProtocolConfig;
    const secondaryProxyAddress = await deployFreshEmptyUUPSProxy(deployer);

    const snapshot = await readAndApply(canonicalAddress, secondaryProxyAddress);
    const pinnedBlock = snapshot.blockNumber;
    const pinnedContextId = snapshot.currentKmsContextId;
    const pinnedEpochId = snapshot.currentEpochId;

    await rotateToNewKmsContext(canonicalAddress, deployer, committee);
    const latestContextId = await canonical.getCurrentKmsContextId();
    expect(latestContextId).to.not.equal(pinnedContextId);

    const historicalContextId = await canonical.getCurrentKmsContextId({ blockTag: pinnedBlock });
    expect(historicalContextId).to.equal(pinnedContextId);
    const historicalState = await canonical.getCurrentKmsContextAndEpoch({ blockTag: pinnedBlock });
    expect(historicalState[0]).to.equal(pinnedContextId);
    expect(historicalState[1]).to.equal(pinnedEpochId);
  });
});

describe('canonical snapshot export (readCanonicalSnapshot)', function () {
  const deployer = new ethers.Wallet(getRequiredEnvVar('DEPLOYER_PRIVATE_KEY')).connect(ethers.provider);

  it('reads the canonical context and reproduces it on re-read', async function () {
    const canonicalNodes = buildProtocolConfigNodes();
    const canonicalAddress = await deployFreshProtocolConfigProxy(
      deployer,
      canonicalNodes,
      buildProtocolConfigThresholds(),
    );

    const snapshot = await readCanonicalSnapshot(hre, {
      canonicalProvider: ethers.provider,
      canonicalProtocolConfigAddress: canonicalAddress,
    });
    expect(snapshot.kmsNodes.length).to.equal(canonicalNodes.length);
    expect(snapshot.currentKmsContextId).to.not.equal(0n);
    expect(snapshot.currentEpochId).to.not.equal(0n);
    expect(snapshot.canonicalChainId).to.equal((await ethers.provider.getNetwork()).chainId);

    // The DAO's review check: re-reading the artifact's pinned block reproduces the snapshot exactly.
    const reread = await readCanonicalSnapshot(hre, {
      canonicalProvider: ethers.provider,
      canonicalProtocolConfigAddress: canonicalAddress,
      blockNumber: snapshot.blockNumber,
    });
    expect(reread).to.deep.equal(snapshot);
  });

  it('rejects when the canonical address is not a ProtocolConfig', async function () {
    const notProtocolConfig = await deployFreshKMSGenerationProxy(deployer);

    await expect(
      readCanonicalSnapshot(hre, {
        canonicalProvider: ethers.provider,
        canonicalProtocolConfigAddress: await notProtocolConfig.getAddress(),
      }),
    ).to.be.rejectedWith(/reports version "KMSGeneration.*expected "ProtocolConfig/);
  });

  it('rejects when the canonical address is an uninitialized empty proxy', async function () {
    const uninitializedEmpty = await deployFreshEmptyUUPSProxy(deployer);

    await expect(
      readCanonicalSnapshot(hre, {
        canonicalProvider: ethers.provider,
        canonicalProtocolConfigAddress: uninitializedEmpty,
      }),
    ).to.be.rejectedWith(/does not expose getVersion\(\); it is not a ProtocolConfig proxy/);
  });

  it('rejects when the canonical ProtocolConfig has no active KMS context', async function () {
    const noContextCanonical = await deployFreshUninitializedProtocolConfigProxy(deployer);

    await expect(
      readCanonicalSnapshot(hre, {
        canonicalProvider: ethers.provider,
        canonicalProtocolConfigAddress: noContextCanonical,
      }),
    ).to.be.rejectedWith(/has no active KMS context \(currentKmsContextId=0\); cannot mirror/);
  });

  it('reproduces a pinned snapshot after a rotation, while a latest re-read drifts', async function () {
    const committee = await buildControllableKmsCommittee();
    const canonicalAddress = await deployFreshProtocolConfigProxy(deployer, committee.nodes, committee.thresholds);

    const exported = await readCanonicalSnapshot(hre, {
      canonicalProvider: ethers.provider,
      canonicalProtocolConfigAddress: canonicalAddress,
    });

    // Rotate the canonical committee so "latest" no longer matches the exported block.
    await rotateToNewKmsContext(canonicalAddress, deployer, committee);

    // Re-reading latest drifts: this is exactly what a signer would get with no block pin.
    const atLatest = await readCanonicalSnapshot(hre, {
      canonicalProvider: ethers.provider,
      canonicalProtocolConfigAddress: canonicalAddress,
    });
    expect(atLatest.currentKmsContextId).to.not.equal(exported.currentKmsContextId);

    // Re-reading at the artifact's blockNumber reproduces the original snapshot despite the rotation.
    const atPinned = await readCanonicalSnapshot(hre, {
      canonicalProvider: ethers.provider,
      canonicalProtocolConfigAddress: canonicalAddress,
      blockNumber: exported.blockNumber,
    });
    expect(atPinned).to.deep.equal(exported);
  });
});

describe('canonical snapshot artifact (buildSnapshotArtifact / parseSnapshotArtifact)', function () {
  const deployer = new ethers.Wallet(getRequiredEnvVar('DEPLOYER_PRIVATE_KEY')).connect(ethers.provider);

  async function exportArtifact(): Promise<string> {
    const canonicalAddress = await deployFreshProtocolConfigProxy(
      deployer,
      buildProtocolConfigNodes(),
      buildProtocolConfigThresholds(),
    );
    const snapshot = await readCanonicalSnapshot(hre, {
      canonicalProvider: ethers.provider,
      canonicalProtocolConfigAddress: canonicalAddress,
    });
    return JSON.stringify(buildSnapshotArtifact(snapshot, canonicalAddress));
  }

  it('round-trips a snapshot through the JSON artifact, preserving the block hash', async function () {
    const canonicalAddress = await deployFreshProtocolConfigProxy(
      deployer,
      buildProtocolConfigNodes(),
      buildProtocolConfigThresholds(),
    );
    const snapshot = await readCanonicalSnapshot(hre, {
      canonicalProvider: ethers.provider,
      canonicalProtocolConfigAddress: canonicalAddress,
    });

    const artifact = buildSnapshotArtifact(snapshot, canonicalAddress);
    expect(artifact.blockHash).to.equal(snapshot.blockHash);
    expect(parseSnapshotArtifact(JSON.stringify(artifact))).to.deep.equal(snapshot);
  });

  it('rejects an artifact whose blockHash is not a 32-byte hex string', async function () {
    const artifact = JSON.parse(await exportArtifact());
    artifact.blockHash = '0xdeadbeef';
    expect(() => parseSnapshotArtifact(JSON.stringify(artifact))).to.throw(/"blockHash" must be a 32-byte hex string/);
  });

  it('rejects an artifact whose node signer address is malformed', async function () {
    const artifact = JSON.parse(await exportArtifact());
    artifact.kmsNodes[0].signerAddress = 'not-an-address';
    expect(() => parseSnapshotArtifact(JSON.stringify(artifact))).to.throw(
      /"kmsNodes\[0\]\.signerAddress" must be a valid address/,
    );
  });
});
