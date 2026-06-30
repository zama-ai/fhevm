import { task, types } from 'hardhat/config';

import { getRequiredEnvVar, loadHostAddresses } from './utils/loadVariables';

// ⚠️ TEST/SPIKE GOVERNANCE IMPERSONATION ⚠️
// These tasks call KMSGeneration's `onlyACLOwner` entry points (migrationKeygen,
// addKeyMaterials, scheduleKeyMaterialMigration) directly with the deployer key.
// In production those entry points are DAO-governed (testnet + mainnet are ruled
// by the DAO), so the real cutover is driven by governance proposals, NOT by
// these tasks. They exist only to drive the cutover deterministically in the
// rollout/spike.
//
// RFC-029 (fhevm-internal#1568) governance tasks driving the key-material
// version cutover. The flow (publish-not-activate): trigger a migration
// keygen-from-existing -> publish the re-derived material under the EXISTING
// (active) key as version 1, without moving activeKeyId -> schedule the
// per-chain / gateway cutover blocks. The coprocessor host-listener ingests the
// emitted KeyMaterialAdded / KeyMaterialMigrationScheduled events.

async function getKmsGeneration(hre: any) {
  const deployerPrivateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
  const deployer = new hre.ethers.Wallet(deployerPrivateKey).connect(hre.ethers.provider);
  const proxyAddress = getRequiredEnvVar('KMS_GENERATION_CONTRACT_ADDRESS');
  const kmsGeneration = await hre.ethers.getContractAt('KMSGeneration', proxyAddress, deployer);
  return { kmsGeneration, deployer };
}

task('task:triggerMigrationKeygen')
  .addParam('paramsType', 'The FHE params type to use for the migration keygen.')
  .setAction(async function ({ paramsType }, hre) {
    await hre.run('compile:specific', { contract: 'contracts' });
    loadHostAddresses(); // KMS_GENERATION_CONTRACT_ADDRESS + PROTOCOL_CONFIG_CONTRACT_ADDRESS live in the host addresses file

    const { kmsGeneration } = await getKmsGeneration(hre);
    // The existing key whose private shares are re-used (keygen-from-existing). The contract derives
    // the v2 (context+epoch) extraData internally and records the request as a migration; at
    // prep-keygen consensus it emits the typed MigrationKeygenRequest, which the connector branches on
    // to drive UseExisting + copy-to-original on the KMS.
    const existingKeyId: bigint = await kmsGeneration.getActiveKeyId();

    console.log(`RFC-029 migration keygen: existingKeyId=${existingKeyId}`);
    const tx = await kmsGeneration.migrationKeygen(paramsType, existingKeyId);
    await tx.wait();

    // The freshly allocated (throwaway) migration key id is the new key counter. addKeyMaterials must
    // be bound to exactly this id, so surface it for the publish step.
    const migrationKeyId: bigint = await kmsGeneration.getKeyCounter();
    console.log(`Migration keygen triggered. migrationKeyId=${migrationKeyId}`);
  });

// RFC-029 GOVERNANCE PUBLISH (path that needs no KMS on-chain signature).
// The migration keygen produced the migrated CompressedXofKeySet (kms-core copied it under the
// existing key id via copy_compressed_key_to_original). Its KMS-attested digests are stored on-chain
// under the migration key id once the keygen reached consensus, so we read them by EXACT id rather
// than scanning for the "latest" keygen result. addKeyMaterials binds to that migrationKeyId and
// publishes under the EXISTING key -- emitting KeyMaterialAdded with NO KMS signature verification.
task('task:publishMigratedKeyMaterials')
  .addParam('migrationKeyId', 'The migration keygen key id whose migrated material is published.')
  .addOptionalParam(
    'keyId',
    'Existing key to publish migrated material under; defaults to the active key.',
    '',
    types.string,
  )
  .setAction(async function ({ migrationKeyId, keyId }, hre) {
    await hre.run('compile:specific', { contract: 'contracts' });
    loadHostAddresses();

    const { kmsGeneration } = await getKmsGeneration(hre);
    const existingKeyId: bigint = keyId ? BigInt(keyId) : await kmsGeneration.getActiveKeyId();
    const migrationId: bigint = BigInt(migrationKeyId);

    // Migrated digests come from the exact migration keygen (stored on-chain at consensus). This binds
    // the published material to the migration keygen the contract validates against in addKeyMaterials.
    const [, migratedDigests] = await kmsGeneration.getKeyMaterials(migrationId);
    if (migratedDigests.length === 0) {
      throw new Error(`migration keygen ${migrationId} has no key digests (did it reach consensus?)`);
    }

    // Reuse the existing key's per-KMS-node storage URLs: the migrated keyset lives under the same
    // nodes (copied to the existing key id), and the host-listener builds the object path from keyId.
    const [urls] = await kmsGeneration.getKeyMaterials(existingKeyId);
    if (urls.length === 0) {
      throw new Error(`no storage URLs found for existing key ${existingKeyId}`);
    }

    console.log(
      `RFC-029 governance publish: existingKeyId=${existingKeyId} migrationKeyId=${migrationId} digests=${migratedDigests.length} urls=${urls.length}`,
    );
    const tx = await kmsGeneration.addKeyMaterials(existingKeyId, migrationId, migratedDigests, urls);
    await tx.wait();
    console.log('Migrated key materials published via governance addKeyMaterials (KeyMaterialAdded emitted).');
  });

task('task:scheduleKeyMaterialMigration')
  .addParam('hostChainIds', 'Comma-separated host chain ids, parallel to hostMigrationBlocks.')
  .addParam('hostMigrationBlocks', 'Comma-separated per-chain migration blocks (H_C).')
  .addParam('gatewayMigrationBlock', 'Gateway migration block (G).')
  .addOptionalParam('keyId', 'Key id being migrated; defaults to the active key id.', '', types.string)
  .setAction(async function ({ hostChainIds, hostMigrationBlocks, gatewayMigrationBlock, keyId }, hre) {
    await hre.run('compile:specific', { contract: 'contracts' });
    loadHostAddresses(); // KMS_GENERATION_CONTRACT_ADDRESS + PROTOCOL_CONFIG_CONTRACT_ADDRESS live in the host addresses file

    const { kmsGeneration } = await getKmsGeneration(hre);
    const targetKeyId: bigint = keyId ? BigInt(keyId) : await kmsGeneration.getActiveKeyId();

    const chainIds = String(hostChainIds)
      .split(',')
      .map((s) => BigInt(s.trim()));
    const blocks = String(hostMigrationBlocks)
      .split(',')
      .map((s) => BigInt(s.trim()));
    if (chainIds.length !== blocks.length) {
      throw new Error(`hostChainIds (${chainIds.length}) and hostMigrationBlocks (${blocks.length}) must be parallel`);
    }

    console.log(
      `RFC-029 schedule: keyId=${targetKeyId} chains=${chainIds} blocks=${blocks} G=${gatewayMigrationBlock}`,
    );
    const tx = await kmsGeneration.scheduleKeyMaterialMigration(
      targetKeyId,
      chainIds,
      blocks,
      BigInt(gatewayMigrationBlock),
    );
    await tx.wait();
    console.log('Key material migration scheduled (KeyMaterialMigrationScheduled emitted).');
  });
