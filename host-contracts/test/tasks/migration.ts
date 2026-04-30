import { expect } from 'chai';
import { Wallet } from 'ethers';
import fs from 'fs';
import { ethers, run, upgrades } from 'hardhat';

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
import type { KMSGeneration, ProtocolConfig } from '../../types';
import { deployEmptyProxy } from '../utils/deploymentHelpers';
import {
  HOST_ENV_FILE,
  buildProtocolConfigNodes,
  buildProtocolConfigThresholds,
  readHostAddress,
  withPatchedMethods,
} from './taskHelpers';

async function deployEmptyUUPSProxy(deployer: Wallet): Promise<string> {
  const factory = await ethers.getContractFactory('EmptyUUPSProxy', deployer);
  return deployEmptyProxy(factory);
}

/**
 * Deploy a legacy-style KMSVerifier proxy mock and advance it to the requested context id.
 * The migrated tasks reconcile against this contract before upgrading ProtocolConfig / KMSGeneration.
 */
async function deployLegacyKMSVerifier(deployer: Wallet, targetContextId: bigint): Promise<string> {
  const initialContextId = KMS_CONTEXT_COUNTER_BASE + BigInt(1);
  if (targetContextId < initialContextId) {
    throw new Error(
      `Legacy KMSVerifier target context ${targetContextId.toString()} is below ${initialContextId.toString()}`,
    );
  }

  const proxyAddress = await deployEmptyUUPSProxy(deployer);
  const legacyImplementation = await ethers.getContractFactory(
    'test/migration-only-previous-contracts/KMSVerifier.sol:KMSVerifier',
    deployer,
  );

  const numNodes = +getRequiredEnvVar('NUM_KMS_NODES');
  const signerAddresses = Array.from({ length: numNodes }, (_, idx) => getRequiredEnvVar(`KMS_SIGNER_ADDRESS_${idx}`));
  const threshold = +getRequiredEnvVar('PUBLIC_DECRYPTION_THRESHOLD');
  const verifyingContractSource = getRequiredEnvVar('DECRYPTION_ADDRESS');
  const chainIDSource = +getRequiredEnvVar('CHAIN_ID_GATEWAY');

  const legacyVerifier = await upgrades.upgradeProxy(proxyAddress, legacyImplementation, {
    call: {
      fn: 'initializeFromEmptyProxy',
      args: [verifyingContractSource, chainIDSource, signerAddresses, threshold],
    },
  });
  await legacyVerifier.waitForDeployment();

  for (let contextId = initialContextId + BigInt(1); contextId <= targetContextId; contextId++) {
    await legacyVerifier.defineNewContext(signerAddresses, threshold);
  }

  return proxyAddress;
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

describe('Migration deploy tasks', function () {
  const deployerPrivateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
  const deployer = new ethers.Wallet(deployerPrivateKey).connect(ethers.provider);
  let originalEnvHost: string;
  let originalMigrationEnv: KmsGenerationMigrationEnvSnapshot;

  before(function () {
    originalEnvHost = fs.readFileSync(HOST_ENV_FILE, 'utf-8');
  });

  beforeEach(function () {
    originalMigrationEnv = snapshotKmsGenerationMigrationEnv();
  });

  afterEach(async function () {
    // Restore .env.host so other tests are unaffected.
    fs.writeFileSync(HOST_ENV_FILE, originalEnvHost);
    restoreKmsGenerationMigrationEnv(originalMigrationEnv);
  });

  // ---------------------------------------------------------------------------
  // ProtocolConfig
  // ---------------------------------------------------------------------------

  describe('ProtocolConfig migration', function () {
    it('should deploy via initializeFromMigration and preserve the context id', async function () {
      const proxyAddress = await deployEmptyUUPSProxy(deployer);
      patchHostEnv('PROTOCOL_CONFIG_CONTRACT_ADDRESS', proxyAddress);

      const migratedContextId = KMS_CONTEXT_COUNTER_BASE + BigInt(3);
      const legacyVerifierAddress = await deployLegacyKMSVerifier(deployer, migratedContextId);
      patchHostEnv('KMS_VERIFIER_CONTRACT_ADDRESS', legacyVerifierAddress);
      process.env.MIGRATION_CONTEXT_ID = migratedContextId.toString();

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

      // All migrated nodes must match the env-derived tx senders and signers, not just nodes[0].
      const numNodes = +getRequiredEnvVar('NUM_KMS_NODES');
      const expectedTxSenders = getKmsTxSenderAddresses(numNodes);
      const expectedSigners = Array.from({ length: numNodes }, (_, idx) =>
        getRequiredEnvVar(`KMS_SIGNER_ADDRESS_${idx}`),
      );
      const nodes = await protocolConfig.getKmsNodesForContext(migratedContextId);
      const signers = await protocolConfig.getKmsSignersForContext(migratedContextId);
      expect(nodes.map((node) => node.txSenderAddress)).to.deep.equal(expectedTxSenders);
      expect(nodes.map((node) => node.signerAddress)).to.deep.equal(expectedSigners);
      expect(signers).to.deep.equal(expectedSigners);
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
      const proxyAddress = await deployEmptyUUPSProxy(deployer);
      patchHostEnv('KMS_GENERATION_CONTRACT_ADDRESS', proxyAddress);

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
      const legacyVerifierAddress = await deployLegacyKMSVerifier(deployer, contextId);
      patchHostEnv('KMS_VERIFIER_CONTRACT_ADDRESS', legacyVerifierAddress);
      applyKmsGenerationMigrationEnv(migrationEnv);

      await run('task:deployKMSGenerationFromMigration');

      const kmsGeneration = (await ethers.getContractAt('KMSGeneration', proxyAddress)) as unknown as KMSGeneration;

      // Active IDs should match.
      expect(await kmsGeneration.getActiveKeyId()).to.equal(activeKeyId);
      expect(await kmsGeneration.getActiveCrsId()).to.equal(activeCrsId);

      // No pending requests after migration (all migrated items are marked done).
      expect(await kmsGeneration.isRequestDone(activePrepKeygenId)).to.equal(true);
      expect(await kmsGeneration.isRequestDone(activeKeyId)).to.equal(true);
      expect(await kmsGeneration.isRequestDone(activeCrsId)).to.equal(true);

      // Consensus tx senders should be registered for each migrated request.
      const keyTxSenders = await kmsGeneration.getConsensusTxSenders(activeKeyId);
      expect(keyTxSenders).to.deep.equal(consensusTxSenders);

      const crsTxSenders = await kmsGeneration.getConsensusTxSenders(activeCrsId);
      expect(crsTxSenders).to.deep.equal(consensusTxSenders);

      const prepTxSenders = await kmsGeneration.getConsensusTxSenders(activePrepKeygenId);
      expect(prepTxSenders).to.deep.equal(consensusTxSenders);
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

      const hardhatEthers = ethers as typeof ethers & {
        getContractFactory: (...args: unknown[]) => Promise<unknown>;
        getContractAt: (...args: unknown[]) => Promise<unknown>;
      };

      await withPatchedMethods(
        hardhatEthers,
        {
          getContractFactory: async () => {
            throw new Error('ethers.getContractFactory should not be called by task:assertProtocolConfigReady');
          },
          getContractAt: async () => {
            throw new Error('ethers.getContractAt should not be called by task:assertProtocolConfigReady');
          },
        },
        async () => {
          await run('task:assertProtocolConfigReady');
        },
      );
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
