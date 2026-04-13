import { expect } from 'chai';
import { Wallet } from 'ethers';
import fs from 'fs';
import { ethers, run, upgrades } from 'hardhat';
import path from 'path';

import { getRequiredEnvVar } from '../../tasks/utils/loadVariables';
import type { KMSGeneration, ProtocolConfig } from '../../types';

const HOST_ENV_FILE = path.join(__dirname, '../../addresses/.env.host');

// Solidity counter bases — must stay in sync with the contracts.
const KMS_CONTEXT_COUNTER_BASE = BigInt(0x07) << BigInt(248);
const PREP_KEYGEN_COUNTER_BASE = BigInt(3) << BigInt(248);
const KEY_COUNTER_BASE = BigInt(4) << BigInt(248);
const CRS_COUNTER_BASE = BigInt(5) << BigInt(248);

/**
 * Deploy a fresh EmptyUUPSProxy instance (version 1, ready for an upgrade-and-init call).
 */
async function deployFreshEmptyProxy(deployer: Wallet): Promise<string> {
  const factory = await ethers.getContractFactory('EmptyUUPSProxy', deployer);
  const proxy = await upgrades.deployProxy(factory, { initializer: 'initialize', kind: 'uups' });
  await proxy.waitForDeployment();
  return proxy.getAddress();
}

/**
 * Replace a single key in the .env.host file.
 */
function patchHostEnv(key: string, value: string): void {
  const content = fs.readFileSync(HOST_ENV_FILE, 'utf-8');
  const updated = content.replace(new RegExp(`${key}=.*`), `${key}=${value}`);
  fs.writeFileSync(HOST_ENV_FILE, updated);
}

/**
 * Non-zero bytes32 helpers for consensus digests (content is irrelevant — only ≠ 0 matters).
 */
function nonZeroBytes32(seed: number): string {
  return '0x' + seed.toString(16).padStart(64, '0');
}

describe('Migration deploy tasks', function () {
  const deployerPrivateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
  const deployer = new ethers.Wallet(deployerPrivateKey).connect(ethers.provider);
  let originalEnvHost: string;

  before(function () {
    originalEnvHost = fs.readFileSync(HOST_ENV_FILE, 'utf-8');
  });

  afterEach(function () {
    // Restore .env.host so other tests are unaffected.
    fs.writeFileSync(HOST_ENV_FILE, originalEnvHost);

    // Clean up migration-specific env vars.
    for (const key of Object.keys(process.env)) {
      if (key.startsWith('MIGRATION_') || key === 'EXISTING_CONTEXT_ID') {
        delete process.env[key];
      }
    }
  });

  // ---------------------------------------------------------------------------
  // ProtocolConfig
  // ---------------------------------------------------------------------------

  describe('ProtocolConfig migration', function () {
    it('should deploy via initializeFromMigration and preserve the context id', async function () {
      const proxyAddress = await deployFreshEmptyProxy(deployer);
      patchHostEnv('PROTOCOL_CONFIG_CONTRACT_ADDRESS', proxyAddress);

      const migratedContextId = KMS_CONTEXT_COUNTER_BASE + BigInt(3);
      process.env.EXISTING_CONTEXT_ID = migratedContextId.toString();

      await run('task:deployProtocolConfigFromMigration');

      const protocolConfig = (await ethers.getContractAt('ProtocolConfig', proxyAddress)) as unknown as ProtocolConfig;

      // Context ID must match exactly.
      expect(await protocolConfig.getCurrentKmsContextId()).to.equal(migratedContextId);

      // Thresholds read from .env.example should be set.
      expect(await protocolConfig.getPublicDecryptionThreshold()).to.equal(
        +getRequiredEnvVar('PUBLIC_DECRYPTION_THRESHOLD'),
      );
      expect(await protocolConfig.getUserDecryptionThreshold()).to.equal(
        +getRequiredEnvVar('USER_DECRYPTION_THRESHOLD'),
      );
      expect(await protocolConfig.getKmsGenThreshold()).to.equal(+getRequiredEnvVar('KMS_GEN_THRESHOLD'));
      expect(await protocolConfig.getMpcThreshold()).to.equal(+getRequiredEnvVar('MPC_THRESHOLD'));

      // KMS nodes should be registered for the migrated context.
      const nodes = await protocolConfig.getKmsNodesForContext(migratedContextId);
      expect(nodes.length).to.equal(+getRequiredEnvVar('NUM_KMS_NODES'));
      expect(nodes[0].txSenderAddress).to.equal(getRequiredEnvVar('KMS_TX_SENDER_ADDRESS_0'));
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

    it('should deploy via initializeFromMigration and restore active state', async function () {
      const proxyAddress = await deployFreshEmptyProxy(deployer);
      patchHostEnv('KMS_GENERATION_CONTRACT_ADDRESS', proxyAddress);

      const txSender = getRequiredEnvVar('KMS_TX_SENDER_ADDRESS_0');
      const activeKeyId = KEY_COUNTER_BASE + BigInt(1);
      const activeCrsId = CRS_COUNTER_BASE + BigInt(1);
      const activePrepKeygenId = PREP_KEYGEN_COUNTER_BASE + BigInt(1);

      // Set all migration env vars.
      process.env.MIGRATION_PREP_KEYGEN_COUNTER = activePrepKeygenId.toString();
      process.env.MIGRATION_KEY_COUNTER = activeKeyId.toString();
      process.env.MIGRATION_CRS_COUNTER = activeCrsId.toString();
      process.env.MIGRATION_ACTIVE_KEY_ID = activeKeyId.toString();
      process.env.MIGRATION_ACTIVE_CRS_ID = activeCrsId.toString();
      process.env.MIGRATION_ACTIVE_PREP_KEYGEN_ID = activePrepKeygenId.toString();
      process.env.MIGRATION_ACTIVE_KEY_DIGESTS = JSON.stringify([
        { keyType: 0, digest: '0xabcdef0123456789' },
        { keyType: 1, digest: '0x9876543210fedcba' },
      ]);
      process.env.MIGRATION_ACTIVE_CRS_DIGEST = '0xdeadbeefcafe0123';
      process.env.MIGRATION_KEY_CONSENSUS_TX_SENDERS = txSender;
      process.env.MIGRATION_KEY_CONSENSUS_DIGEST = nonZeroBytes32(1);
      process.env.MIGRATION_CRS_CONSENSUS_TX_SENDERS = txSender;
      process.env.MIGRATION_CRS_CONSENSUS_DIGEST = nonZeroBytes32(2);
      process.env.MIGRATION_PREP_KEYGEN_CONSENSUS_TX_SENDERS = txSender;
      process.env.MIGRATION_PREP_KEYGEN_CONSENSUS_DIGEST = nonZeroBytes32(3);
      process.env.MIGRATION_CRS_MAX_BIT_LENGTH = '4096';
      process.env.MIGRATION_PREP_KEYGEN_PARAMS_TYPE = '0'; // Default
      process.env.MIGRATION_CRS_PARAMS_TYPE = '0'; // Default
      process.env.MIGRATION_CONTEXT_ID = contextId.toString();

      await run('task:deployKMSGenerationFromMigration');

      const kmsGeneration = (await ethers.getContractAt('KMSGeneration', proxyAddress)) as unknown as KMSGeneration;

      // Active IDs should match.
      expect(await kmsGeneration.getActiveKeyId()).to.equal(activeKeyId);
      expect(await kmsGeneration.getActiveCrsId()).to.equal(activeCrsId);

      // No pending requests after migration (all migrated items are marked done).
      expect(await kmsGeneration.hasPendingKeyManagementRequest()).to.be.false;

      // Consensus tx senders should be registered for each migrated request.
      const keyTxSenders = await kmsGeneration.getConsensusTxSenders(activeKeyId);
      expect(keyTxSenders.length).to.equal(1);
      expect(keyTxSenders[0]).to.equal(txSender);

      const crsTxSenders = await kmsGeneration.getConsensusTxSenders(activeCrsId);
      expect(crsTxSenders.length).to.equal(1);
      expect(crsTxSenders[0]).to.equal(txSender);

      const prepTxSenders = await kmsGeneration.getConsensusTxSenders(activePrepKeygenId);
      expect(prepTxSenders.length).to.equal(1);
      expect(prepTxSenders[0]).to.equal(txSender);
    });
  });
});
