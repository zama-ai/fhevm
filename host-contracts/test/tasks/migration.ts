import { expect } from 'chai';
import { Wallet } from 'ethers';
import fs from 'fs';
import hre, { ethers, run, upgrades } from 'hardhat';
import os from 'os';
import path from 'path';

import { buildSnapshotArtifact, readCanonicalSnapshot } from '../../tasks/protocolConfigMirror';
import { buildKmsNodes, buildKmsThresholds } from '../../tasks/taskDeploy';
import { makeEnvHelpers } from '../../tasks/utils/envSnapshot';
import { KMS_CONTEXT_COUNTER_BASE } from '../../tasks/utils/kmsGenerationConstants';
import { getRequiredEnvVar } from '../../tasks/utils/loadVariables';
import {
  type ProtocolConfigMigrationEnv,
  type ProtocolConfigMigrationEnvSnapshot,
  applyProtocolConfigMigrationEnv,
  restoreProtocolConfigMigrationEnv,
  snapshotProtocolConfigMigrationEnv,
} from '../../tasks/utils/protocolConfigMigrationEnv';
import { UPGRADE_TO_AND_CALL_INTERFACE } from '../../tasks/utils/upgradeProposal';
import { deployEmptyProxy } from '../utils/deploymentHelpers';
import {
  HOST_ENV_FILE,
  buildProtocolConfigNodes,
  buildProtocolConfigThresholds,
  deployFreshProtocolConfigProxy,
  readHostAddress,
} from './taskHelpers';

async function deployEmptyUUPSProxy(deployer: Wallet): Promise<string> {
  const factory = await ethers.getContractFactory('EmptyUUPSProxy', deployer);
  return deployEmptyProxy(factory);
}

/**
 * Replace a single key in the .env.host file.
 */
function patchHostEnv(key: string, value: string): void {
  const content = fs.readFileSync(HOST_ENV_FILE, 'utf-8');
  const updated = content.replace(new RegExp(`${key}=.*`), `${key}=${value}`);
  fs.writeFileSync(HOST_ENV_FILE, updated);
}

const HOST_ADDRESS_ENV_KEYS = [
  'PROTOCOL_CONFIG_CONTRACT_ADDRESS',
  'KMS_GENERATION_CONTRACT_ADDRESS',
  'KMS_VERIFIER_CONTRACT_ADDRESS',
] as const;

const { snapshot: snapshotHostAddressEnv, restore: restoreHostAddressEnv } = makeEnvHelpers(HOST_ADDRESS_ENV_KEYS);
type HostAddressEnvSnapshot = ReturnType<typeof snapshotHostAddressEnv>;

function getKmsTxSenderAddresses(count: number): string[] {
  return Array.from({ length: count }, (_, idx) => getRequiredEnvVar(`KMS_TX_SENDER_ADDRESS_${idx}`));
}

async function readImplementationSlot(proxyAddress: string): Promise<string> {
  return upgrades.erc1967.getImplementationAddress(proxyAddress);
}

describe('Migration prepare tasks', function () {
  const deployerPrivateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
  const deployer = new ethers.Wallet(deployerPrivateKey).connect(ethers.provider);
  let originalEnvHost: string;
  let originalHostAddressEnv: HostAddressEnvSnapshot;
  let originalProtocolConfigMigrationEnv: ProtocolConfigMigrationEnvSnapshot;

  before(function () {
    originalEnvHost = fs.readFileSync(HOST_ENV_FILE, 'utf-8');
  });

  beforeEach(function () {
    originalHostAddressEnv = snapshotHostAddressEnv();
    originalProtocolConfigMigrationEnv = snapshotProtocolConfigMigrationEnv();
  });

  afterEach(async function () {
    // Restore .env.host so other tests are unaffected.
    fs.writeFileSync(HOST_ENV_FILE, originalEnvHost);
    restoreHostAddressEnv(originalHostAddressEnv);
    restoreProtocolConfigMigrationEnv(originalProtocolConfigMigrationEnv);
  });

  // ---------------------------------------------------------------------------
  // ProtocolConfig
  // ---------------------------------------------------------------------------

  describe('ProtocolConfig migration', function () {
    it('prepares DAO calldata without mutating the empty proxy implementation', async function () {
      const proxyAddress = await deployEmptyUUPSProxy(deployer);
      patchHostEnv('PROTOCOL_CONFIG_CONTRACT_ADDRESS', proxyAddress);

      const migratedContextId = KMS_CONTEXT_COUNTER_BASE + BigInt(3);
      const protocolConfigMigrationEnv: ProtocolConfigMigrationEnv = {
        MIGRATION_CONTEXT_ID: migratedContextId.toString(),
        MIGRATION_KMS_NODES: JSON.stringify(buildKmsNodes()),
        MIGRATION_KMS_THRESHOLDS: JSON.stringify(buildKmsThresholds()),
      };
      applyProtocolConfigMigrationEnv(protocolConfigMigrationEnv);

      const implementationSlotBefore = await readImplementationSlot(proxyAddress);
      const preparedUpgrade = await run('task:prepareDeployProtocolConfigFromMigration', {
        verifyContract: false,
      });
      const implementationSlotAfter = await readImplementationSlot(proxyAddress);

      expect(implementationSlotAfter).to.equal(implementationSlotBefore);

      const { newImplementationAddress, innerFunctionSignature, innerCalldata, outerCalldata } = preparedUpgrade;
      expect(preparedUpgrade.proxyAddress).to.equal(proxyAddress);
      const iface = new ethers.Interface([`function ${innerFunctionSignature}`]);
      const decoded = iface.decodeFunctionData('initializeFromMigration', innerCalldata);

      expect(decoded[0]).to.equal(migratedContextId);
      expect(decoded[1].map((node: string[]) => node[0])).to.deep.equal(
        getKmsTxSenderAddresses(+getRequiredEnvVar('NUM_KMS_NODES')),
      );
      expect(decoded[2][0]).to.equal(BigInt(getRequiredEnvVar('PUBLIC_DECRYPTION_THRESHOLD')));
      expect(outerCalldata).to.equal(
        UPGRADE_TO_AND_CALL_INTERFACE.encodeFunctionData('upgradeToAndCall', [newImplementationAddress, innerCalldata]),
      );
    });

    it('executes the devnet direct upgrade and initializes ProtocolConfig from migration state', async function () {
      const proxyAddress = await deployEmptyUUPSProxy(deployer);
      patchHostEnv('PROTOCOL_CONFIG_CONTRACT_ADDRESS', proxyAddress);

      const migratedContextId = KMS_CONTEXT_COUNTER_BASE + BigInt(4);
      const protocolConfigMigrationEnv: ProtocolConfigMigrationEnv = {
        MIGRATION_CONTEXT_ID: migratedContextId.toString(),
        MIGRATION_KMS_NODES: JSON.stringify(buildKmsNodes()),
        MIGRATION_KMS_THRESHOLDS: JSON.stringify(buildKmsThresholds()),
      };
      applyProtocolConfigMigrationEnv(protocolConfigMigrationEnv);

      const implementationSlotBefore = await readImplementationSlot(proxyAddress);
      await run('task:deployProtocolConfigFromMigration');
      const protocolConfig = await ethers.getContractAt('ProtocolConfig', proxyAddress);

      expect(await readImplementationSlot(proxyAddress)).to.not.equal(implementationSlotBefore);
      expect(await protocolConfig.getVersion()).to.equal('ProtocolConfig v0.1.0');
      expect(await protocolConfig.getCurrentKmsContextId()).to.equal(migratedContextId);
      expect(await protocolConfig.getPublicDecryptionThreshold()).to.equal(
        BigInt(getRequiredEnvVar('PUBLIC_DECRYPTION_THRESHOLD')),
      );

      const kmsNodes = await protocolConfig.getKmsNodesForContext(migratedContextId);
      expect(kmsNodes.map((node) => node.txSenderAddress)).to.deep.equal(
        getKmsTxSenderAddresses(+getRequiredEnvVar('NUM_KMS_NODES')),
      );
    });
  });

  describe('ProtocolConfig from canonical artifact', function () {
    async function writeCanonicalArtifact(): Promise<{ file: string; canonicalAddress: string }> {
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
      const file = path.join(fs.mkdtempSync(path.join(os.tmpdir(), 'canonical-snapshot-')), 'snapshot.json');
      fs.writeFileSync(file, JSON.stringify(artifact, null, 2));
      return { file, canonicalAddress };
    }

    it('prepares DAO calldata from a reviewed artifact without mutating the proxy', async function () {
      const proxyAddress = await deployEmptyUUPSProxy(deployer);
      patchHostEnv('PROTOCOL_CONFIG_CONTRACT_ADDRESS', proxyAddress);
      const { file, canonicalAddress } = await writeCanonicalArtifact();
      const canonical = await ethers.getContractAt('ProtocolConfig', canonicalAddress, deployer);
      const canonicalContextId = await canonical.getCurrentKmsContextId();

      const implementationSlotBefore = await readImplementationSlot(proxyAddress);
      const preparedUpgrade = await run('task:prepareDeployProtocolConfigFromCanonical', {
        snapshot: file,
        verifyContract: false,
      });
      expect(await readImplementationSlot(proxyAddress)).to.equal(implementationSlotBefore);

      const { newImplementationAddress, innerFunctionSignature, innerCalldata, outerCalldata } = preparedUpgrade;
      expect(preparedUpgrade.proxyAddress).to.equal(proxyAddress);
      const iface = new ethers.Interface([`function ${innerFunctionSignature}`]);
      const decoded = iface.decodeFunctionData('initializeFromMigration', innerCalldata);
      expect(decoded[0]).to.equal(canonicalContextId);
      expect(decoded[1].map((node: string[]) => node[0])).to.deep.equal(
        buildProtocolConfigNodes().map((node) => node.txSenderAddress),
      );
      expect(decoded[2][0]).to.equal(BigInt(buildProtocolConfigThresholds().publicDecryption));
      expect(outerCalldata).to.equal(
        UPGRADE_TO_AND_CALL_INTERFACE.encodeFunctionData('upgradeToAndCall', [newImplementationAddress, innerCalldata]),
      );
    });

    it('applies a reviewed artifact with the devnet deploy task, without canonical RPC access', async function () {
      const proxyAddress = await deployEmptyUUPSProxy(deployer);
      patchHostEnv('PROTOCOL_CONFIG_CONTRACT_ADDRESS', proxyAddress);
      const { file, canonicalAddress } = await writeCanonicalArtifact();
      const canonical = await ethers.getContractAt('ProtocolConfig', canonicalAddress, deployer);
      const canonicalContextId = await canonical.getCurrentKmsContextId();

      await run('task:deployProtocolConfigFromCanonical', { snapshot: file });

      const secondary = await ethers.getContractAt('ProtocolConfig', proxyAddress, deployer);
      expect(await secondary.getCurrentKmsContextId()).to.equal(canonicalContextId);
      expect((await secondary.getKmsNodesForContext(canonicalContextId)).length).to.equal(
        buildProtocolConfigNodes().length,
      );
      expect(await secondary.getPublicDecryptionThresholdForContext(canonicalContextId)).to.equal(
        BigInt(buildProtocolConfigThresholds().publicDecryption),
      );
    });

    it('rejects when neither a snapshot nor canonical RPC parameters are given', async function () {
      await expect(run('task:deployProtocolConfigFromCanonical')).to.be.rejectedWith(/either --snapshot/);
    });
  });

  describe('KMSVerifier deployment', function () {
    it('passes the standalone readiness check when ProtocolConfig is initialized', async function () {
      const protocolConfigProxyAddress = await deployEmptyUUPSProxy(deployer);

      patchHostEnv('PROTOCOL_CONFIG_CONTRACT_ADDRESS', protocolConfigProxyAddress);

      const currentImplementation = await ethers.getContractFactory('EmptyUUPSProxy', deployer);
      const newImplementation = await ethers.getContractFactory('ProtocolConfig', deployer);
      const proxy = await upgrades.forceImport(protocolConfigProxyAddress, currentImplementation);
      const protocolConfig = await upgrades.upgradeProxy(proxy, newImplementation, {
        call: { fn: 'initializeFromEmptyProxy', args: [buildProtocolConfigNodes(), buildProtocolConfigThresholds()] },
      });
      await protocolConfig.waitForDeployment();

      await run('task:assertProtocolConfigReady');
    });

    it('rejects the standalone readiness check when ProtocolConfig is not initialized', async function () {
      const protocolConfigProxyAddress = await deployEmptyUUPSProxy(deployer);

      patchHostEnv('PROTOCOL_CONFIG_CONTRACT_ADDRESS', protocolConfigProxyAddress);

      await expect(run('task:assertProtocolConfigReady')).to.be.rejectedWith(
        `Cannot deploy KMSVerifier: Contract at ${protocolConfigProxyAddress} does not expose getVersion(); it is not a ProtocolConfig proxy.`,
      );
    });

    it('rejects the standalone readiness check when pointed at a KMSVerifier address', async function () {
      const kmsVerifierAddress = readHostAddress('KMS_VERIFIER_CONTRACT_ADDRESS');

      patchHostEnv('PROTOCOL_CONFIG_CONTRACT_ADDRESS', kmsVerifierAddress);

      await expect(run('task:assertProtocolConfigReady')).to.be.rejectedWith(
        new RegExp(`Cannot deploy KMSVerifier: Contract at ${kmsVerifierAddress} reports version "KMSVerifier`),
      );
    });

    it('rejects deployment when ProtocolConfig is not initialized', async function () {
      const protocolConfigProxyAddress = await deployEmptyUUPSProxy(deployer);
      const kmsVerifierProxyAddress = await deployEmptyUUPSProxy(deployer);

      patchHostEnv('PROTOCOL_CONFIG_CONTRACT_ADDRESS', protocolConfigProxyAddress);
      patchHostEnv('KMS_VERIFIER_CONTRACT_ADDRESS', kmsVerifierProxyAddress);

      const kmsVerifierImplBefore = await upgrades.erc1967.getImplementationAddress(kmsVerifierProxyAddress);

      await expect(run('task:deployKMSVerifier')).to.be.rejectedWith(
        `Cannot deploy KMSVerifier: Contract at ${protocolConfigProxyAddress} does not expose getVersion(); it is not a ProtocolConfig proxy.`,
      );

      expect(await upgrades.erc1967.getImplementationAddress(kmsVerifierProxyAddress)).to.equal(kmsVerifierImplBefore);
    });

    it('prepares KMSVerifier upgrade calldata without mutating the proxy implementation', async function () {
      const kmsVerifierProxyAddress = readHostAddress('KMS_VERIFIER_CONTRACT_ADDRESS');
      const kmsVerifierImplBefore = await upgrades.erc1967.getImplementationAddress(kmsVerifierProxyAddress);

      await run('task:prepareUpgradeKMSVerifier', {
        currentImplementation: 'contracts/KMSVerifier.sol:KMSVerifier',
        newImplementation: 'contracts/KMSVerifier.sol:KMSVerifier',
        useInternalProxyAddress: true,
        verifyContract: false,
      });

      expect(await upgrades.erc1967.getImplementationAddress(kmsVerifierProxyAddress)).to.equal(kmsVerifierImplBefore);
    });
  });
});
