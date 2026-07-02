import { task, types } from 'hardhat/config';
import type { HardhatRuntimeEnvironment } from 'hardhat/types';

import { getRequiredEnvVar, loadHostAddresses } from './utils/loadVariables';

// Operational tasks for the RFC-029 one-time compressed-key migration.
// They drive the KMSGeneration migration surface and assert its two
// invariants: the active key never changes, and the cutover schedule is
// single-assignment.

const KMS_GENERATION_ADDRESS_ENV_VAR = 'KMS_GENERATION_CONTRACT_ADDRESS';

async function kmsGeneration(hre: HardhatRuntimeEnvironment, useInternalProxyAddress: boolean) {
  if (useInternalProxyAddress) {
    loadHostAddresses();
  }
  const address = getRequiredEnvVar(KMS_GENERATION_ADDRESS_ENV_VAR);
  return hre.ethers.getContractAt('KMSGeneration', address);
}

const sleep = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));

task(
  'task:compressedKeyMigrationKeygen',
  'Trigger the RFC-029 migration keygen for the active key and wait for the compressed materials to be published',
)
  .addOptionalParam('keyId', 'The existing key to migrate (defaults to the active key)', '', types.string)
  .addOptionalParam('timeoutMinutes', 'How long to wait for KMS consensus', 60, types.int)
  .addOptionalParam('useInternalProxyAddress', 'Load addresses from the internal env file', false, types.boolean)
  .setAction(async ({ keyId, timeoutMinutes, useInternalProxyAddress }, hre) => {
    const contract = await kmsGeneration(hre, useInternalProxyAddress);

    const migratedKeyId: bigint = keyId !== '' ? BigInt(keyId) : await contract.getActiveKeyId();
    const activeKeyIdBefore: bigint = await contract.getActiveKeyId();
    const completedBefore: bigint[] = [...(await contract.getCompletedKeyIds())];
    console.info(`Triggering compressed-key migration keygen for key ${migratedKeyId}`);

    const tx = await contract.compressedKeyMigrationKeygen(migratedKeyId);
    await tx.wait();

    // The KMS runs a real (potentially long) MPC keygen-from-existing;
    // poll until the compressed materials reach on-chain consensus.
    const deadline = Date.now() + timeoutMinutes * 60_000;
    for (;;) {
      try {
        const [urls, digests] = await contract.getCompressedKeyMaterials(migratedKeyId);
        console.info(`Compressed key materials published: ${digests.length} digest(s), ${urls.length} storage URL(s)`);
        break;
      } catch {
        if (Date.now() > deadline) {
          throw new Error(`Timed out waiting for compressed key materials of key ${migratedKeyId}`);
        }
        await sleep(10_000);
      }
    }

    // Publication is not activation.
    const activeKeyIdAfter: bigint = await contract.getActiveKeyId();
    if (activeKeyIdAfter !== activeKeyIdBefore) {
      throw new Error(`activeKeyId changed during the migration: ${activeKeyIdBefore} -> ${activeKeyIdAfter}`);
    }
    const completedAfter: bigint[] = [...(await contract.getCompletedKeyIds())];
    if (completedAfter.length !== completedBefore.length) {
      throw new Error('completedKeyIds changed during the migration: publication must not look like a new key');
    }
    console.info('OK: compressed materials published; active key unchanged');
  });

task('task:scheduleCompressedKeyCutover', 'Schedule the RFC-029 cutover for a key with published compressed materials')
  .addOptionalParam('keyId', 'The migrated key (defaults to the active key)', '', types.string)
  .addParam('hostCutovers', 'JSON [{"chainId": .., "cutoverBlock": ..}, ..]', undefined, types.string)
  .addParam('gatewayCutoverBlock', 'The Gateway cutover block', undefined, types.string)
  .addOptionalParam('useInternalProxyAddress', 'Load addresses from the internal env file', false, types.boolean)
  .setAction(async ({ keyId, hostCutovers, gatewayCutoverBlock, useInternalProxyAddress }, hre) => {
    const contract = await kmsGeneration(hre, useInternalProxyAddress);
    const migratedKeyId: bigint = keyId !== '' ? BigInt(keyId) : await contract.getActiveKeyId();

    const cutovers = (JSON.parse(hostCutovers) as { chainId: string | number; cutoverBlock: string | number }[]).map(
      (c) => ({ chainId: BigInt(c.chainId), cutoverBlock: BigInt(c.cutoverBlock) }),
    );
    console.info(
      `Scheduling compressed-key cutover for key ${migratedKeyId}: gateway block ${gatewayCutoverBlock}, hosts ${hostCutovers}`,
    );
    const tx = await contract.scheduleCompressedKeyCutover(migratedKeyId, cutovers, BigInt(gatewayCutoverBlock));
    await tx.wait();

    const [exists, storedCutovers, storedGatewayBlock] = await contract.getCompressedKeyCutoverSchedule(migratedKeyId);
    if (!exists || storedGatewayBlock !== BigInt(gatewayCutoverBlock) || storedCutovers.length !== cutovers.length) {
      throw new Error('Stored cutover schedule does not match the scheduled values');
    }
    console.info('OK: cutover schedule stored');
  });

task('task:assertCompressedKeyCutoverScheduled', 'Assert the RFC-029 cutover schedule is stored for a key')
  .addOptionalParam('keyId', 'The migrated key (defaults to the active key)', '', types.string)
  .addOptionalParam('useInternalProxyAddress', 'Load addresses from the internal env file', false, types.boolean)
  .setAction(async ({ keyId, useInternalProxyAddress }, hre) => {
    const contract = await kmsGeneration(hre, useInternalProxyAddress);
    const migratedKeyId: bigint = keyId !== '' ? BigInt(keyId) : await contract.getActiveKeyId();
    const [exists, cutovers, gatewayBlock] = await contract.getCompressedKeyCutoverSchedule(migratedKeyId);
    if (!exists) {
      throw new Error(`No compressed-key cutover schedule stored for key ${migratedKeyId}`);
    }
    console.info(
      `Cutover schedule for key ${migratedKeyId}: gateway block ${gatewayBlock}, ${cutovers.length} host chain(s)`,
    );
  });
