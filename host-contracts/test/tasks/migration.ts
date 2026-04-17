import { expect } from 'chai';
import { Wallet } from 'ethers';
import fs from 'fs';
import { ethers, run, upgrades } from 'hardhat';
import path from 'path';

import { getRequiredEnvVar } from '../../tasks/utils/loadVariables';
import type { KMSGeneration, ProtocolConfig } from '../../types';

const HOST_ENV_FILE = path.join(__dirname, '../../addresses/.env.host');
const HOST_ADDRESSES_FILE = path.join(__dirname, '../../addresses/FHEVMHostAddresses.sol');

// Solidity counter bases — must stay in sync with the contracts.
const KMS_CONTEXT_COUNTER_BASE = BigInt(0x07) << BigInt(248);
const PREP_KEYGEN_COUNTER_BASE = BigInt(3) << BigInt(248);
const KEY_COUNTER_BASE = BigInt(4) << BigInt(248);
const CRS_COUNTER_BASE = BigInt(5) << BigInt(248);

const HOST_ADDRESS_CONSTANT_BY_ENV_KEY = {
  PROTOCOL_CONFIG_CONTRACT_ADDRESS: 'protocolConfigAdd',
  KMS_GENERATION_CONTRACT_ADDRESS: 'kmsGenerationAdd',
  KMS_VERIFIER_CONTRACT_ADDRESS: 'kmsVerifierAdd',
} as const;

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

  const proxyAddress = await deployFreshEmptyProxy(deployer);
  const currentImplementation = await ethers.getContractFactory('EmptyUUPSProxy', deployer);
  const legacyImplementation = await ethers.getContractFactory(
    'test/migration-only-previous-contracts/KMSVerifier.sol:KMSVerifier',
    deployer,
  );
  const proxy = await upgrades.forceImport(proxyAddress, currentImplementation);

  const numNodes = +getRequiredEnvVar('NUM_KMS_NODES');
  const signerAddresses = Array.from({ length: numNodes }, (_, idx) => getRequiredEnvVar(`KMS_SIGNER_ADDRESS_${idx}`));
  const threshold = +getRequiredEnvVar('PUBLIC_DECRYPTION_THRESHOLD');
  const verifyingContractSource = getRequiredEnvVar('DECRYPTION_ADDRESS');
  const chainIDSource = +getRequiredEnvVar('CHAIN_ID_GATEWAY');

  const legacyVerifier = await upgrades.upgradeProxy(proxy, legacyImplementation, {
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

function patchHostAddress(key: keyof typeof HOST_ADDRESS_CONSTANT_BY_ENV_KEY, value: string): void {
  patchHostEnv(key, value);

  const constantName = HOST_ADDRESS_CONSTANT_BY_ENV_KEY[key];
  const content = fs.readFileSync(HOST_ADDRESSES_FILE, 'utf-8');
  const updated = content.replace(
    new RegExp(`address constant ${constantName} = .*;`),
    `address constant ${constantName} = ${value};`,
  );
  fs.writeFileSync(HOST_ADDRESSES_FILE, updated);
}

async function recompileMigratedContracts(): Promise<void> {
  await run('compile', { force: true });
}

/**
 * Non-zero bytes32 helpers for consensus digests (content is irrelevant — only ≠ 0 matters).
 */
function nonZeroBytes32(seed: number): string {
  return '0x' + seed.toString(16).padStart(64, '0');
}

function setKmsGenerationMigrationEnv(contextId: bigint, txSender: string): void {
  const activeKeyId = KEY_COUNTER_BASE + BigInt(1);
  const activeCrsId = CRS_COUNTER_BASE + BigInt(1);
  const activePrepKeygenId = PREP_KEYGEN_COUNTER_BASE + BigInt(1);

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
  process.env.MIGRATION_PREP_KEYGEN_PARAMS_TYPE = '0';
  process.env.MIGRATION_CRS_PARAMS_TYPE = '0';
  process.env.MIGRATION_CONTEXT_ID = contextId.toString();
}

describe('Migration deploy tasks', function () {
  const deployerPrivateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
  const deployer = new ethers.Wallet(deployerPrivateKey).connect(ethers.provider);
  let originalEnvHost: string;
  let originalHostAddresses: string;
  let hostArtifactsMutated = false;

  before(function () {
    originalEnvHost = fs.readFileSync(HOST_ENV_FILE, 'utf-8');
    originalHostAddresses = fs.readFileSync(HOST_ADDRESSES_FILE, 'utf-8');
  });

  afterEach(async function () {
    // Restore .env.host so other tests are unaffected.
    fs.writeFileSync(HOST_ENV_FILE, originalEnvHost);

    if (hostArtifactsMutated) {
      fs.writeFileSync(HOST_ADDRESSES_FILE, originalHostAddresses);
      await recompileMigratedContracts();
      hostArtifactsMutated = false;
    }

    // Clean up migration-specific env vars.
    for (const key of Object.keys(process.env)) {
      if (key.startsWith('MIGRATION_')) {
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
      const nodes = await protocolConfig.getKmsNodesForContext(migratedContextId);
      const signers = await protocolConfig.getKmsSignersForContext(migratedContextId);
      expect(nodes.length).to.equal(numNodes);
      expect(signers.length).to.equal(numNodes);
      for (let i = 0; i < numNodes; i++) {
        expect(nodes[i].txSenderAddress).to.equal(getRequiredEnvVar(`KMS_TX_SENDER_ADDRESS_${i}`));
        expect(nodes[i].signerAddress).to.equal(getRequiredEnvVar(`KMS_SIGNER_ADDRESS_${i}`));
        expect(signers[i]).to.equal(getRequiredEnvVar(`KMS_SIGNER_ADDRESS_${i}`));
      }
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
      const legacyVerifierAddress = await deployLegacyKMSVerifier(deployer, contextId);
      patchHostEnv('KMS_VERIFIER_CONTRACT_ADDRESS', legacyVerifierAddress);
      setKmsGenerationMigrationEnv(contextId, txSender);

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

  describe('KMSVerifier deployment', function () {
    it('rejects the standalone readiness check when ProtocolConfig is not initialized', async function () {
      const protocolConfigProxyAddress = await deployFreshEmptyProxy(deployer);

      patchHostAddress('PROTOCOL_CONFIG_CONTRACT_ADDRESS', protocolConfigProxyAddress);
      hostArtifactsMutated = true;

      await expect(run('task:assertProtocolConfigReady')).to.be.rejectedWith(
        `Cannot deploy KMSVerifier: ProtocolConfig at ${protocolConfigProxyAddress} is not initialized`,
      );
    });

    it('rejects deployment when ProtocolConfig is not initialized', async function () {
      const protocolConfigProxyAddress = await deployFreshEmptyProxy(deployer);
      const kmsVerifierProxyAddress = await deployFreshEmptyProxy(deployer);

      patchHostAddress('PROTOCOL_CONFIG_CONTRACT_ADDRESS', protocolConfigProxyAddress);
      patchHostAddress('KMS_VERIFIER_CONTRACT_ADDRESS', kmsVerifierProxyAddress);
      hostArtifactsMutated = true;

      const kmsVerifierImplBefore = await upgrades.erc1967.getImplementationAddress(kmsVerifierProxyAddress);

      await expect(run('task:deployKMSVerifier')).to.be.rejectedWith(
        `Cannot deploy KMSVerifier: ProtocolConfig at ${protocolConfigProxyAddress} is not initialized`,
      );

      expect(await upgrades.erc1967.getImplementationAddress(kmsVerifierProxyAddress)).to.equal(kmsVerifierImplBefore);
    });
  });
});
