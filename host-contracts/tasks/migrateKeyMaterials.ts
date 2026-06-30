import { task, types } from 'hardhat/config';

import { getRequiredEnvVar, loadHostAddresses } from './utils/loadVariables';

// ⚠️ TEST/SPIKE GOVERNANCE IMPERSONATION ⚠️
// These tasks call KMSGeneration's `onlyACLOwner` entry points (keygen,
// scheduleKeyMaterialMigration) directly with the deployer key. In production
// those entry points are DAO-governed (testnet + mainnet are ruled by the DAO),
// so the real cutover is driven by governance proposals, NOT by these tasks.
// They exist only to drive the cutover deterministically in the rollout/spike.
//
// RFC-029 (fhevm-internal#1568) governance tasks driving the key-material
// version cutover. The flow (publish-not-activate, "governance publishes"
// variant): trigger a migration keygen-from-existing -> publish the re-derived
// material under the EXISTING (active) key as version 1, without moving
// activeKeyId -> schedule the per-chain / gateway cutover blocks. The
// coprocessor host-listener ingests the emitted KeyMaterialAdded /
// KeyMaterialMigrationScheduled events.

// RFC-029 cutover material version (0 = legacy, 1 = migrated CompressedXofKeySet).
const MIGRATED_MATERIAL_VERSION = 1;

async function getKmsGeneration(hre: any) {
  const deployerPrivateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
  const deployer = new hre.ethers.Wallet(deployerPrivateKey).connect(hre.ethers.provider);
  const proxyAddress = getRequiredEnvVar('KMS_GENERATION_CONTRACT_ADDRESS');
  const kmsGeneration = await hre.ethers.getContractAt('KMSGeneration', proxyAddress, deployer);
  return { kmsGeneration, deployer };
}

task('task:triggerMigrationKeygen')
  .addParam('paramsType', 'The FHE params type to use for the migration keygen.')
  .addOptionalParam(
    'copyToOriginal',
    'Whether kms-core copies the migrated CompressedXofKeySet to the original key id.',
    true,
    types.boolean,
  )
  .setAction(async function ({ paramsType, copyToOriginal }, hre) {
    await hre.run('compile:specific', { contract: 'contracts' });
    loadHostAddresses(); // KMS_GENERATION_CONTRACT_ADDRESS + PROTOCOL_CONFIG_CONTRACT_ADDRESS live in the host addresses file

    const { kmsGeneration } = await getKmsGeneration(hre);
    // The existing key whose private shares are re-used (keygen-from-existing). The contract derives
    // the v2 (context+epoch) extraData internally and flags the request as a migration; the connector
    // reads MigrationKeygenRequested to drive UseExisting + copy-to-original on the KMS.
    const existingKeyId: bigint = await kmsGeneration.getActiveKeyId();

    console.log(`RFC-029 migration keygen: existingKeyId=${existingKeyId} copyToOriginal=${copyToOriginal}`);
    const tx = await kmsGeneration.migrationKeygen(paramsType, existingKeyId, copyToOriginal);
    await tx.wait();
    console.log('Migration keygen triggered.');
  });

// RFC-029 GOVERNANCE PUBLISH (path that needs no KMS on-chain signature).
// The migration keygen produced the migrated CompressedXofKeySet (kms-core copied it under the
// existing key id via copy_compressed_key_to_original). The KMS-attested digests for it are carried
// in the KeygenResponse event the connector forwards (emitted on every call, independent of whether
// on-chain consensus forms). Governance reads those digests, reuses the existing key's per-node
// storage URLs, and publishes under the EXISTING key via addKeyMaterials -- emitting KeyMaterialAdded
// with NO KMS signature verification, so it is unaffected by the v3-extraData signing mismatch.
task('task:publishMigratedKeyMaterials')
  .addOptionalParam(
    'keyId',
    'Existing key to publish migrated material under; defaults to the active key.',
    '',
    types.string,
  )
  .addOptionalParam('lookbackBlocks', 'Recent blocks to scan for the migration KeygenResponse.', 50000, types.int)
  .addOptionalParam('timeoutMs', 'How long to wait for the migration keygen result event.', 20 * 60 * 1000, types.int)
  .setAction(async function ({ keyId, lookbackBlocks, timeoutMs }, hre) {
    await hre.run('compile:specific', { contract: 'contracts' });
    loadHostAddresses();

    const { kmsGeneration } = await getKmsGeneration(hre);
    const existingKeyId: bigint = keyId ? BigInt(keyId) : await kmsGeneration.getActiveKeyId();

    // Wait for the migration keygen to produce a result on-chain. The migration keygen is the only
    // keygen in flight this phase; take the latest KeygenResponse carrying a Server-type (the
    // migrated keyset) digest.
    const deadline = Date.now() + timeoutMs;
    let migratedDigests: unknown[] | null = null;
    while (Date.now() < deadline) {
      const tip = await hre.ethers.provider.getBlockNumber();
      const fromBlock = Math.max(0, tip - lookbackBlocks);
      const events = await kmsGeneration.queryFilter(kmsGeneration.filters.KeygenResponse(), fromBlock, tip);
      const withServerDigest = events.filter((e: any) =>
        (e.args?.keyDigests ?? []).some((d: any) => Number(d.keyType) === 0),
      );
      if (withServerDigest.length > 0) {
        const latest = withServerDigest[withServerDigest.length - 1] as any;
        migratedDigests = latest.args.keyDigests;
        console.log(`RFC-029 publish: using migrated digests from KeygenResponse key_id=${latest.args.keyId}`);
        break;
      }
      console.log('[publish] waiting for the migration KeygenResponse event...');
      await new Promise((resolve) => setTimeout(resolve, 15_000));
    }
    if (!migratedDigests) {
      throw new Error('no migration KeygenResponse event observed within timeout; did the keygen produce a result?');
    }

    // Reuse the existing key's per-KMS-node storage URLs: the migrated keyset lives under the same
    // nodes (copied to the existing key id), and the host-listener builds the object path from keyId.
    const [urls] = await kmsGeneration.getKeyMaterials(existingKeyId);
    if (urls.length === 0) {
      throw new Error(`no storage URLs found for existing key ${existingKeyId}`);
    }

    console.log(
      `RFC-029 governance publish: keyId=${existingKeyId} version=${MIGRATED_MATERIAL_VERSION} digests=${migratedDigests.length} urls=${urls.length}`,
    );
    const tx = await kmsGeneration.addKeyMaterials(existingKeyId, migratedDigests, urls, MIGRATED_MATERIAL_VERSION);
    await tx.wait();
    console.log('Migrated key materials published via governance addKeyMaterials (KeyMaterialAdded emitted).');
  });

task('task:scheduleKeyMaterialMigration')
  .addParam('hostChainIds', 'Comma-separated host chain ids, parallel to hostMigrationBlocks.')
  .addParam('hostMigrationBlocks', 'Comma-separated per-chain migration blocks (H_C).')
  .addParam('gatewayMigrationBlock', 'Gateway migration block (G).')
  .addOptionalParam('materialVersion', 'Target material version (RFC-029 cutover = 1).', 1, types.int)
  .addOptionalParam('keyId', 'Key id being migrated; defaults to the active key id.', '', types.string)
  .setAction(async function (
    { hostChainIds, hostMigrationBlocks, gatewayMigrationBlock, materialVersion, keyId },
    hre,
  ) {
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
      `RFC-029 schedule: keyId=${targetKeyId} version=${materialVersion} chains=${chainIds} blocks=${blocks} G=${gatewayMigrationBlock}`,
    );
    const tx = await kmsGeneration.scheduleKeyMaterialMigration(
      targetKeyId,
      chainIds,
      blocks,
      BigInt(gatewayMigrationBlock),
      materialVersion,
    );
    await tx.wait();
    console.log('Key material migration scheduled (KeyMaterialMigrationScheduled emitted).');
  });
