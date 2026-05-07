import { expect } from 'chai';
import { Wallet } from 'ethers';
import fs from 'fs';
import { ethers, run, upgrades } from 'hardhat';

import { UPGRADE_TO_AND_CALL_INTERFACE } from '../../tasks/taskMigrate';
import { buildKmsNodes, buildKmsThresholds } from '../../tasks/taskDeploy';
import {
  CRS_COUNTER_BASE,
  KEY_COUNTER_BASE,
  KMS_CONTEXT_COUNTER_BASE,
  PREP_KEYGEN_COUNTER_BASE,
} from '../../tasks/utils/kmsGenerationConstants';
import {
  type KmsGenerationMigrationEnv,
  type KmsGenerationMigrationEnvSnapshot,
  applyKmsGenerationMigrationEnv,
  restoreKmsGenerationMigrationEnv,
  snapshotKmsGenerationMigrationEnv,
} from '../../tasks/utils/kmsGenerationMigrationEnv';
import { getRequiredEnvVar } from '../../tasks/utils/loadVariables';
import {
  type ProtocolConfigMigrationEnv,
  type ProtocolConfigMigrationEnvSnapshot,
  applyProtocolConfigMigrationEnv,
  restoreProtocolConfigMigrationEnv,
  snapshotProtocolConfigMigrationEnv,
} from '../../tasks/utils/protocolConfigMigrationEnv';
import { deployEmptyProxy } from '../utils/deploymentHelpers';
import { HOST_ENV_FILE, buildProtocolConfigNodes, buildProtocolConfigThresholds, readHostAddress } from './taskHelpers';

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

function getKmsTxSenderAddresses(count: number): string[] {
  return Array.from({ length: count }, (_, idx) => getRequiredEnvVar(`KMS_TX_SENDER_ADDRESS_${idx}`));
}

/**
 * Non-zero bytes32 helpers for consensus digests (content is irrelevant — only ≠ 0 matters).
 */
function nonZeroBytes32(seed: number): string {
  return '0x' + seed.toString(16).padStart(64, '0');
}

async function readImplementationSlot(proxyAddress: string): Promise<string> {
  return upgrades.erc1967.getImplementationAddress(proxyAddress);
}

describe('Migration prepare tasks', function () {
  const deployerPrivateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
  const deployer = new ethers.Wallet(deployerPrivateKey).connect(ethers.provider);
  let originalEnvHost: string;
  let originalMigrationEnv: KmsGenerationMigrationEnvSnapshot;
  let originalProtocolConfigMigrationEnv: ProtocolConfigMigrationEnvSnapshot;

  before(function () {
    originalEnvHost = fs.readFileSync(HOST_ENV_FILE, 'utf-8');
  });

  beforeEach(function () {
    originalMigrationEnv = snapshotKmsGenerationMigrationEnv();
    originalProtocolConfigMigrationEnv = snapshotProtocolConfigMigrationEnv();
  });

  afterEach(async function () {
    // Restore .env.host so other tests are unaffected.
    fs.writeFileSync(HOST_ENV_FILE, originalEnvHost);
    restoreKmsGenerationMigrationEnv(originalMigrationEnv);
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
      const preparedUpgrade = await run('task:prepareDeployProtocolConfigFromMigration');
      const implementationSlotAfter = await readImplementationSlot(proxyAddress);

      expect(implementationSlotAfter).to.equal(implementationSlotBefore);

      const { newImplementationAddress, innerFunctionSignature, innerCalldata, outerCalldata } = preparedUpgrade;
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
  });

  // ---------------------------------------------------------------------------
  // KMSGeneration
  // ---------------------------------------------------------------------------

  describe('KMSGeneration migration', function () {
    // The KMSGeneration migration validates consensus tx senders against the
    // canonical ProtocolConfig (deployed by the test hook). That config has
    // context KMS_CONTEXT_COUNTER_BASE + 1 with the nodes from .env.example.
    const contextId = KMS_CONTEXT_COUNTER_BASE + BigInt(1);

    function buildKmsGenerationMigrationFixture() {
      const activeKeyId = KEY_COUNTER_BASE + BigInt(1);
      const activeCrsId = CRS_COUNTER_BASE + BigInt(1);
      const activePrepKeygenId = PREP_KEYGEN_COUNTER_BASE + BigInt(1);
      const consensusTxSenders = getKmsTxSenderAddresses(+getRequiredEnvVar('KMS_GEN_THRESHOLD'));
      const consensusTxSendersEnv = consensusTxSenders.join(',');
      const migrationEnv: KmsGenerationMigrationEnv = {
        MIGRATION_PREP_KEYGEN_COUNTER: activePrepKeygenId.toString(),
        MIGRATION_KEY_COUNTER: activeKeyId.toString(),
        MIGRATION_CRS_COUNTER: activeCrsId.toString(),
        MIGRATION_ACTIVE_KEY_ID: activeKeyId.toString(),
        MIGRATION_ACTIVE_CRS_ID: activeCrsId.toString(),
        MIGRATION_ACTIVE_PREP_KEYGEN_ID: activePrepKeygenId.toString(),
        MIGRATION_ACTIVE_KEY_DIGESTS: JSON.stringify([
          { keyType: 0, digest: '0xabcdef0123456789' },
          { keyType: 1, digest: '0x9876543210fedcba' },
        ]),
        MIGRATION_ACTIVE_CRS_DIGEST: '0xdeadbeefcafe0123',
        MIGRATION_KEY_CONSENSUS_TX_SENDERS: consensusTxSendersEnv,
        MIGRATION_KEY_CONSENSUS_DIGEST: nonZeroBytes32(1),
        MIGRATION_CRS_CONSENSUS_TX_SENDERS: consensusTxSendersEnv,
        MIGRATION_CRS_CONSENSUS_DIGEST: nonZeroBytes32(2),
        MIGRATION_PREP_KEYGEN_CONSENSUS_TX_SENDERS: consensusTxSendersEnv,
        MIGRATION_PREP_KEYGEN_CONSENSUS_DIGEST: nonZeroBytes32(3),
        MIGRATION_CRS_MAX_BIT_LENGTH: '4096',
        MIGRATION_PREP_KEYGEN_PARAMS_TYPE: '0',
        MIGRATION_CRS_PARAMS_TYPE: '0',
        MIGRATION_CONTEXT_ID: contextId.toString(),
      };
      return { activeKeyId, activeCrsId, consensusTxSenders, migrationEnv };
    }

    it('prepares DAO calldata from MIGRATION_* env without mutating the empty proxy implementation', async function () {
      const proxyAddress = await deployEmptyUUPSProxy(deployer);
      patchHostEnv('KMS_GENERATION_CONTRACT_ADDRESS', proxyAddress);

      const { activeKeyId, activeCrsId, consensusTxSenders, migrationEnv } = buildKmsGenerationMigrationFixture();
      applyKmsGenerationMigrationEnv(migrationEnv);

      const implementationSlotBefore = await readImplementationSlot(proxyAddress);
      const preparedUpgrade = await run('task:prepareDeployKMSGenerationFromMigration');
      const implementationSlotAfter = await readImplementationSlot(proxyAddress);

      expect(implementationSlotAfter).to.equal(implementationSlotBefore);

      const { newImplementationAddress, innerFunctionSignature, innerCalldata, outerCalldata } = preparedUpgrade;
      const iface = new ethers.Interface([`function ${innerFunctionSignature}`]);
      const decoded = iface.decodeFunctionData('initializeFromMigration', innerCalldata);

      expect(decoded[0][3]).to.equal(activeKeyId);
      expect(decoded[0][4]).to.equal(activeCrsId);
      expect(decoded[0][8]).to.deep.equal(consensusTxSenders);
      expect(decoded[0][17]).to.equal(contextId);
      expect(outerCalldata).to.equal(
        UPGRADE_TO_AND_CALL_INTERFACE.encodeFunctionData('upgradeToAndCall', [newImplementationAddress, innerCalldata]),
      );
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
  });
});
