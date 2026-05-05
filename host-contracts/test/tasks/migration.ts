import { expect } from 'chai';
import { Wallet } from 'ethers';
import fs from 'fs';
import { ethers, run, upgrades } from 'hardhat';
import os from 'os';
import path from 'path';

import { UPGRADE_TO_AND_CALL_INTERFACE, stringifyForProposal } from '../../tasks/taskMigrate';
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

async function writeProposalJson(proposal: unknown): Promise<string> {
  const proposalPath = path.join(os.tmpdir(), `dao-kms-migration-${Date.now()}-${Math.random()}.json`);
  fs.writeFileSync(proposalPath, stringifyForProposal(proposal));
  return proposalPath;
}

async function writeProtocolConfigMigrationState(
  contextId: bigint,
  overrides: Partial<{
    kmsNodes: Array<{ txSenderAddress: string; signerAddress: string; ipAddress: string; storageUrl: string }>;
    thresholds: Record<string, string>;
  }> = {},
): Promise<string> {
  const numNodes = +getRequiredEnvVar('NUM_KMS_NODES');
  const kmsNodes =
    overrides.kmsNodes ??
    Array.from({ length: numNodes }, (_, idx) => ({
      txSenderAddress: getRequiredEnvVar(`KMS_TX_SENDER_ADDRESS_${idx}`),
      signerAddress: getRequiredEnvVar(`KMS_SIGNER_ADDRESS_${idx}`),
      ipAddress: process.env[`KMS_NODE_IP_${idx}`] || '',
      storageUrl: getRequiredEnvVar(`KMS_NODE_STORAGE_URL_${idx}`),
    }));
  const thresholds = overrides.thresholds ?? {
    publicDecryption: getRequiredEnvVar('PUBLIC_DECRYPTION_THRESHOLD'),
    userDecryption: getRequiredEnvVar('USER_DECRYPTION_THRESHOLD'),
    kmsGen: getRequiredEnvVar('KMS_GEN_THRESHOLD'),
    mpc: getRequiredEnvVar('MPC_THRESHOLD'),
  };

  return writeProposalJson({
    contextId: contextId.toString(),
    kmsNodes,
    thresholds,
  });
}

describe('Migration prepare tasks', function () {
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
    it('prepares DAO calldata without mutating the empty proxy implementation', async function () {
      const proxyAddress = await deployEmptyUUPSProxy(deployer);
      patchHostEnv('PROTOCOL_CONFIG_CONTRACT_ADDRESS', proxyAddress);

      const migratedContextId = KMS_CONTEXT_COUNTER_BASE + BigInt(3);
      process.env.MIGRATION_CONTEXT_ID = migratedContextId.toString();
      const migrationStatePath = await writeProtocolConfigMigrationState(migratedContextId);

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

      const proposalPath = await writeProposalJson({
        newImplementationAddress,
        innerFunctionSignature,
        decodedArgs: [decoded[0], decoded[1], decoded[2]],
        innerCalldata,
        outerCalldata,
      });

      await run('task:verifyDaoKmsMigrationProposal', {
        proposal: proposalPath,
        migrationState: migrationStatePath,
      });

      const mismatchedProposalPath = await writeProposalJson({
        newImplementationAddress,
        innerFunctionSignature,
        decodedArgs: [decoded[0], decoded[1], decoded[2]],
        innerCalldata,
        outerCalldata: outerCalldata.replace(/.$/, outerCalldata.endsWith('0') ? '1' : '0'),
      });

      await expect(
        run('task:verifyDaoKmsMigrationProposal', {
          proposal: mismatchedProposalPath,
          migrationState: migrationStatePath,
        }),
      ).to.be.rejectedWith('upgradeToAndCall calldata mismatch');
    });

    it('rejects stale ProtocolConfig node metadata at proposal verification', async function () {
      const proxyAddress = await deployEmptyUUPSProxy(deployer);
      patchHostEnv('PROTOCOL_CONFIG_CONTRACT_ADDRESS', proxyAddress);

      const migratedContextId = KMS_CONTEXT_COUNTER_BASE + BigInt(3);
      process.env.MIGRATION_CONTEXT_ID = migratedContextId.toString();

      const preparedUpgrade = await run('task:prepareDeployProtocolConfigFromMigration');
      const proposalPath = await writeProposalJson(preparedUpgrade);

      const numNodes = +getRequiredEnvVar('NUM_KMS_NODES');
      const staleNodes = Array.from({ length: numNodes }, (_, idx) => ({
        txSenderAddress: getRequiredEnvVar(`KMS_TX_SENDER_ADDRESS_${idx}`),
        signerAddress: getRequiredEnvVar(`KMS_SIGNER_ADDRESS_${idx}`),
        ipAddress: process.env[`KMS_NODE_IP_${idx}`] || '',
        storageUrl: getRequiredEnvVar(`KMS_NODE_STORAGE_URL_${idx}`),
      }));
      staleNodes[0] = { ...staleNodes[0], storageUrl: `${staleNodes[0].storageUrl}-stale` };
      const migrationStatePath = await writeProtocolConfigMigrationState(migratedContextId, { kmsNodes: staleNodes });

      await expect(
        run('task:verifyDaoKmsMigrationProposal', { proposal: proposalPath, migrationState: migrationStatePath }),
      ).to.be.rejectedWith('ProtocolConfig proposal KMS nodes mismatch');
    });

    it('rejects stale ProtocolConfig thresholds at proposal verification', async function () {
      const proxyAddress = await deployEmptyUUPSProxy(deployer);
      patchHostEnv('PROTOCOL_CONFIG_CONTRACT_ADDRESS', proxyAddress);

      const migratedContextId = KMS_CONTEXT_COUNTER_BASE + BigInt(3);
      process.env.MIGRATION_CONTEXT_ID = migratedContextId.toString();

      const preparedUpgrade = await run('task:prepareDeployProtocolConfigFromMigration');
      const proposalPath = await writeProposalJson(preparedUpgrade);

      const migrationStatePath = await writeProtocolConfigMigrationState(migratedContextId, {
        thresholds: {
          publicDecryption: getRequiredEnvVar('PUBLIC_DECRYPTION_THRESHOLD'),
          userDecryption: (BigInt(getRequiredEnvVar('USER_DECRYPTION_THRESHOLD')) + 1n).toString(),
          kmsGen: getRequiredEnvVar('KMS_GEN_THRESHOLD'),
          mpc: getRequiredEnvVar('MPC_THRESHOLD'),
        },
      });

      await expect(
        run('task:verifyDaoKmsMigrationProposal', { proposal: proposalPath, migrationState: migrationStatePath }),
      ).to.be.rejectedWith('ProtocolConfig proposal thresholds mismatch');
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

    function buildKmsGenerationMigrationStateJson(migrationEnv: KmsGenerationMigrationEnv) {
      return {
        prepKeygenCounter: migrationEnv.MIGRATION_PREP_KEYGEN_COUNTER,
        keyCounter: migrationEnv.MIGRATION_KEY_COUNTER,
        crsCounter: migrationEnv.MIGRATION_CRS_COUNTER,
        activeKeyId: migrationEnv.MIGRATION_ACTIVE_KEY_ID,
        activeCrsId: migrationEnv.MIGRATION_ACTIVE_CRS_ID,
        activePrepKeygenId: migrationEnv.MIGRATION_ACTIVE_PREP_KEYGEN_ID,
        activeKeyDigests: JSON.parse(migrationEnv.MIGRATION_ACTIVE_KEY_DIGESTS),
        activeCrsDigest: migrationEnv.MIGRATION_ACTIVE_CRS_DIGEST,
        keyConsensusTxSenders: migrationEnv.MIGRATION_KEY_CONSENSUS_TX_SENDERS.split(','),
        keyConsensusDigest: migrationEnv.MIGRATION_KEY_CONSENSUS_DIGEST,
        crsConsensusTxSenders: migrationEnv.MIGRATION_CRS_CONSENSUS_TX_SENDERS.split(','),
        crsConsensusDigest: migrationEnv.MIGRATION_CRS_CONSENSUS_DIGEST,
        prepKeygenConsensusTxSenders: migrationEnv.MIGRATION_PREP_KEYGEN_CONSENSUS_TX_SENDERS.split(','),
        prepKeygenConsensusDigest: migrationEnv.MIGRATION_PREP_KEYGEN_CONSENSUS_DIGEST,
        crsMaxBitLength: migrationEnv.MIGRATION_CRS_MAX_BIT_LENGTH,
        prepKeygenParamsType: migrationEnv.MIGRATION_PREP_KEYGEN_PARAMS_TYPE,
        crsParamsType: migrationEnv.MIGRATION_CRS_PARAMS_TYPE,
        contextId: migrationEnv.MIGRATION_CONTEXT_ID,
      };
    }

    async function writeKmsGenerationMigrationState(
      migrationEnv: KmsGenerationMigrationEnv,
      contextIdOverride = migrationEnv.MIGRATION_CONTEXT_ID,
    ): Promise<string> {
      return writeProposalJson({
        contextId: contextIdOverride,
        hostKmsGenerationMigrationState: buildKmsGenerationMigrationStateJson(migrationEnv),
      });
    }

    it('prepares DAO calldata from MIGRATION_* env and verifies it against migration-state.json', async function () {
      const proxyAddress = await deployEmptyUUPSProxy(deployer);
      patchHostEnv('KMS_GENERATION_CONTRACT_ADDRESS', proxyAddress);

      const { activeKeyId, activeCrsId, consensusTxSenders, migrationEnv } = buildKmsGenerationMigrationFixture();
      applyKmsGenerationMigrationEnv(migrationEnv);
      const migrationStatePath = await writeKmsGenerationMigrationState(migrationEnv);

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

      await run('task:verifyDaoKmsMigrationProposal', {
        proposal: await writeProposalJson(preparedUpgrade),
        migrationState: migrationStatePath,
      });
    });

    it('rejects stale KMSGeneration decodedArgs at proposal verification', async function () {
      const proxyAddress = await deployEmptyUUPSProxy(deployer);
      patchHostEnv('KMS_GENERATION_CONTRACT_ADDRESS', proxyAddress);

      const { migrationEnv } = buildKmsGenerationMigrationFixture();
      applyKmsGenerationMigrationEnv(migrationEnv);
      const preparedUpgrade = await run('task:prepareDeployKMSGenerationFromMigration');
      const staleMigrationStatePath = await writeKmsGenerationMigrationState({
        ...migrationEnv,
        MIGRATION_ACTIVE_CRS_DIGEST: '0xfeedface',
      });

      await expect(
        run('task:verifyDaoKmsMigrationProposal', {
          proposal: await writeProposalJson(preparedUpgrade),
          migrationState: staleMigrationStatePath,
        }),
      ).to.be.rejectedWith('KMSGeneration proposal migration state mismatch');
    });

    it('rejects internally inconsistent migration-state context IDs', async function () {
      const proxyAddress = await deployEmptyUUPSProxy(deployer);
      patchHostEnv('KMS_GENERATION_CONTRACT_ADDRESS', proxyAddress);

      const { migrationEnv } = buildKmsGenerationMigrationFixture();
      applyKmsGenerationMigrationEnv(migrationEnv);
      const preparedUpgrade = await run('task:prepareDeployKMSGenerationFromMigration');
      const inconsistentMigrationStatePath = await writeKmsGenerationMigrationState(
        migrationEnv,
        (BigInt(migrationEnv.MIGRATION_CONTEXT_ID) + 1n).toString(),
      );

      await expect(
        run('task:verifyDaoKmsMigrationProposal', {
          proposal: await writeProposalJson(preparedUpgrade),
          migrationState: inconsistentMigrationStatePath,
        }),
      ).to.be.rejectedWith('migration-state.hostKmsGenerationMigrationState.contextId mismatch');
    });
  });

  describe('addresses.json generator', function () {
    function randomAddress(): string {
      return Wallet.createRandom().address;
    }

    async function writeMigrationStateMetadata(metadata: Record<string, unknown>): Promise<string> {
      return writeProposalJson({ metadata });
    }

    it('generates addresses.json from migration metadata and proposal artifacts', async function () {
      const gatewayKmsGenerationProxy = randomAddress();
      const gatewayConfigProxy = randomAddress();
      const hostKmsVerifierProxy = randomAddress();
      const hostProtocolConfigProxy = randomAddress();
      const hostKmsGenerationProxy = randomAddress();

      const migrationStatePath = await writeMigrationStateMetadata({
        gatewayKmsGenerationProxy,
        gatewayConfigProxy,
        legacyHostKmsVerifierProxy: hostKmsVerifierProxy,
      });
      const gatewayProposalPath = await writeProposalJson({
        proxyAddress: gatewayKmsGenerationProxy,
        newImplementationAddress: randomAddress(),
      });
      const protocolConfigProposalPath = await writeProposalJson({
        proxyAddress: hostProtocolConfigProxy,
        newImplementationAddress: randomAddress(),
      });
      const kmsGenerationProposalPath = await writeProposalJson({
        proxyAddress: hostKmsGenerationProxy,
        newImplementationAddress: randomAddress(),
      });
      const kmsVerifierProposalPath = await writeProposalJson({
        proxyAddress: hostKmsVerifierProxy,
        newImplementationAddress: randomAddress(),
      });
      const outputPath = path.join(os.tmpdir(), `dao-kms-migration-addresses-${Date.now()}-${Math.random()}.json`);

      const generated = await run('task:generateDaoKmsMigrationAddresses', {
        migrationState: migrationStatePath,
        gatewayKmsGenerationProposal: gatewayProposalPath,
        hostProtocolConfigProposal: protocolConfigProposalPath,
        hostKmsGenerationProposal: kmsGenerationProposalPath,
        hostKmsVerifierProposal: kmsVerifierProposalPath,
        output: outputPath,
      });

      const written = JSON.parse(fs.readFileSync(outputPath, 'utf-8'));
      expect(written).to.deep.equal(generated);
      expect(written.host.proxies.protocolConfig).to.equal(hostProtocolConfigProxy);
      expect(written.host.proxies.kmsGeneration).to.equal(hostKmsGenerationProxy);
      expect(written.host.proxies.kmsVerifier).to.equal(hostKmsVerifierProxy);
      expect(written.gateway.proxies.kmsGeneration).to.equal(gatewayKmsGenerationProxy);
      expect(written.gateway.proxies.gatewayConfig).to.equal(gatewayConfigProxy);
    });

    it('rejects gateway proxy mismatches between migration metadata and proposal artifacts', async function () {
      const migrationStatePath = await writeMigrationStateMetadata({
        gatewayKmsGenerationProxy: randomAddress(),
        gatewayConfigProxy: randomAddress(),
        legacyHostKmsVerifierProxy: randomAddress(),
      });
      const gatewayProposalPath = await writeProposalJson({
        proxyAddress: randomAddress(),
        newImplementationAddress: randomAddress(),
      });
      const protocolConfigProposalPath = await writeProposalJson({
        proxyAddress: randomAddress(),
        newImplementationAddress: randomAddress(),
      });
      const kmsGenerationProposalPath = await writeProposalJson({
        proxyAddress: randomAddress(),
        newImplementationAddress: randomAddress(),
      });
      const kmsVerifierProposalPath = await writeProposalJson({
        proxyAddress: JSON.parse(fs.readFileSync(migrationStatePath, 'utf-8')).metadata.legacyHostKmsVerifierProxy,
        newImplementationAddress: randomAddress(),
      });

      await expect(
        run('task:generateDaoKmsMigrationAddresses', {
          migrationState: migrationStatePath,
          gatewayKmsGenerationProposal: gatewayProposalPath,
          hostProtocolConfigProposal: protocolConfigProposalPath,
          hostKmsGenerationProposal: kmsGenerationProposalPath,
          hostKmsVerifierProposal: kmsVerifierProposalPath,
          output: path.join(os.tmpdir(), `dao-kms-migration-addresses-${Date.now()}-${Math.random()}.json`),
        }),
      ).to.be.rejectedWith('gateway KMSGeneration proxy mismatch');
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
