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

// v3 extra_data the connector decodes into a migration KeyGenRequest
// (UseExisting + CompressedAll + copy_compressed_key_to_original).
// Layout: [0x03][contextId(32)][existingKeysetId(32)][copyToOriginal(1)] == 66 bytes.
const EXTRA_DATA_V3 = 3;

async function getKmsGeneration(hre: any) {
  const deployerPrivateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
  const deployer = new hre.ethers.Wallet(deployerPrivateKey).connect(hre.ethers.provider);
  const proxyAddress = getRequiredEnvVar('KMS_GENERATION_CONTRACT_ADDRESS');
  const kmsGeneration = await hre.ethers.getContractAt('KMSGeneration', proxyAddress, deployer);
  return { kmsGeneration, deployer };
}

async function getCurrentKmsContextId(hre: any): Promise<bigint> {
  const protocolConfigAddress = getRequiredEnvVar('PROTOCOL_CONFIG_CONTRACT_ADDRESS');
  const protocolConfig = new hre.ethers.Contract(
    protocolConfigAddress,
    ['function getCurrentKmsContextId() view returns (uint256)'],
    hre.ethers.provider,
  );
  return protocolConfig.getCurrentKmsContextId();
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
    // The existing key whose private shares are re-used (keygen-from-existing).
    const existingKeysetId: bigint = await kmsGeneration.getActiveKeyId();
    const contextId = await getCurrentKmsContextId(hre);

    const extraData = hre.ethers.solidityPacked(
      ['uint8', 'uint256', 'uint256', 'uint8'],
      [EXTRA_DATA_V3, contextId, existingKeysetId, copyToOriginal ? 1 : 0],
    );

    console.log(
      `RFC-029 migration keygen: existingKeysetId=${existingKeysetId} contextId=${contextId} copyToOriginal=${copyToOriginal}`,
    );
    const tx = await kmsGeneration['keygen(uint8,bytes)'](paramsType, extraData);
    await tx.wait();
    console.log('Migration keygen triggered.');
  });

// NOTE: there is intentionally no governance "publish" task. The migration keygen's
// keygenResponse (v3 extraData) publishes the re-derived material under the existing key directly
// (publish-not-activate). A governance addKeyMaterials path would have to source the *migrated*
// digests, which are not retrievable on-chain (getKeyMaterials returns the original keygen digests),
// so it is omitted rather than shipped wrong.

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
