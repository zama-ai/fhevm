import { calculateERC7201StorageLocation } from "@openzeppelin/upgrades-core/dist/utils/erc7201";
import { AbiCoder, ZeroHash, getAddress, isAddress, keccak256, toBeHex, zeroPadValue } from "ethers";
import fs from "fs";
import { task, types } from "hardhat/config";
import path from "path";

const KMS_GENERATION_STORAGE_LOCATION = BigInt(
  calculateERC7201StorageLocation("fhevm_gateway.storage.KMSGeneration"),
);

function storageSlot(offset: bigint): string {
  return toBeHex(KMS_GENERATION_STORAGE_LOCATION + offset, 32);
}

const KMS_GENERATION_SLOT = {
  consensusDigest: storageSlot(3n),
  prepKeygenCounter: storageSlot(4n),
  keyCounter: storageSlot(5n),
  keygenIdPairs: storageSlot(6n),
  activeKeyId: storageSlot(8n),
  crsCounter: storageSlot(9n),
  crsMaxBitLength: storageSlot(10n),
  activeCrsId: storageSlot(12n),
} as const;

const coder = AbiCoder.defaultAbiCoder();

const KMS_GENERATION_MIGRATION_ENV_FIELDS = {
  MIGRATION_PREP_KEYGEN_COUNTER: "prepKeygenCounter",
  MIGRATION_KEY_COUNTER: "keyCounter",
  MIGRATION_CRS_COUNTER: "crsCounter",
  MIGRATION_ACTIVE_KEY_ID: "activeKeyId",
  MIGRATION_ACTIVE_CRS_ID: "activeCrsId",
  MIGRATION_ACTIVE_PREP_KEYGEN_ID: "activePrepKeygenId",
  MIGRATION_ACTIVE_KEY_DIGESTS: "activeKeyDigests",
  MIGRATION_ACTIVE_CRS_DIGEST: "activeCrsDigest",
  MIGRATION_KEY_CONSENSUS_TX_SENDERS: "keyConsensusTxSenders",
  MIGRATION_KEY_CONSENSUS_DIGEST: "keyConsensusDigest",
  MIGRATION_CRS_CONSENSUS_TX_SENDERS: "crsConsensusTxSenders",
  MIGRATION_CRS_CONSENSUS_DIGEST: "crsConsensusDigest",
  MIGRATION_PREP_KEYGEN_CONSENSUS_TX_SENDERS: "prepKeygenConsensusTxSenders",
  MIGRATION_PREP_KEYGEN_CONSENSUS_DIGEST: "prepKeygenConsensusDigest",
  MIGRATION_CRS_MAX_BIT_LENGTH: "crsMaxBitLength",
  MIGRATION_PREP_KEYGEN_PARAMS_TYPE: "prepKeygenParamsType",
  MIGRATION_CRS_PARAMS_TYPE: "crsParamsType",
  MIGRATION_CONTEXT_ID: "contextId",
};

function checksumAddress(value: string, label: string): string {
  if (!isAddress(value)) {
    throw new Error(`${label} address is invalid: ${value}`);
  }
  return getAddress(value);
}

function mappingSlot(key: bigint, slot: string): string {
  return keccak256(coder.encode(["uint256", "uint256"], [key, BigInt(slot)]));
}

function bytes32(value: string): string {
  return zeroPadValue(value, 32);
}

async function readKmsStorageState(
  ethers: typeof import("hardhat").ethers,
  kmsGenerationAddress: string,
  blockTag: number,
) {
  const readUint = async (slot: string) =>
    BigInt(await ethers.provider.getStorage(kmsGenerationAddress, slot, blockTag));
  const readWord = async (slot: string) =>
    bytes32(await ethers.provider.getStorage(kmsGenerationAddress, slot, blockTag));

  const [prepKeygenCounter, keyCounter, crsCounter, activeKeyId, activeCrsId] = await Promise.all([
    readUint(KMS_GENERATION_SLOT.prepKeygenCounter),
    readUint(KMS_GENERATION_SLOT.keyCounter),
    readUint(KMS_GENERATION_SLOT.crsCounter),
    readUint(KMS_GENERATION_SLOT.activeKeyId),
    readUint(KMS_GENERATION_SLOT.activeCrsId),
  ]);

  const [activePrepKeygenId, crsMaxBitLength, keyConsensusDigest, crsConsensusDigest] = await Promise.all([
    readUint(mappingSlot(activeKeyId, KMS_GENERATION_SLOT.keygenIdPairs)),
    readUint(mappingSlot(activeCrsId, KMS_GENERATION_SLOT.crsMaxBitLength)),
    readWord(mappingSlot(activeKeyId, KMS_GENERATION_SLOT.consensusDigest)),
    readWord(mappingSlot(activeCrsId, KMS_GENERATION_SLOT.consensusDigest)),
  ]);

  const prepKeygenConsensusDigest = await readWord(mappingSlot(activePrepKeygenId, KMS_GENERATION_SLOT.consensusDigest));

  return {
    prepKeygenCounter,
    keyCounter,
    crsCounter,
    activeKeyId,
    activeCrsId,
    activePrepKeygenId,
    crsMaxBitLength,
    keyConsensusDigest,
    crsConsensusDigest,
    prepKeygenConsensusDigest,
  };
}

function assertMigrationPrecondition(condition: boolean) {
  if (!condition) {
    throw new Error("Migration precondition failed");
  }
}

function buildMigrationEnv(state: Record<string, unknown>): Record<string, unknown> {
  const env: Record<string, unknown> = {};
  for (const [envKey, stateField] of Object.entries(KMS_GENERATION_MIGRATION_ENV_FIELDS)) {
    const value = state[stateField];
    if (Array.isArray(value)) {
      env[envKey] = value.every((item) => typeof item === "string") ? value.join(",") : JSON.stringify(value);
    } else {
      env[envKey] = value;
    }
  }
  return env;
}

task("task:exportKmsMigrationState", "Exports Gateway KMSGeneration migration state for the DAO runbook")
  .addParam("kmsGenerationProxy", "Gateway KMSGeneration proxy address")
  .addParam("gatewayConfigProxy", "GatewayConfig proxy address")
  .addOptionalParam("legacyHostKmsVerifier", "Legacy host KMSVerifier proxy address to record as metadata")
  .addOptionalParam("output", "Output JSON path", "migration-state.json", types.string)
  .addOptionalParam(
    "blockTag",
    "Block number to export from. Defaults to the latest provider block",
    undefined,
    types.int,
  )
  .setAction(async function (taskArguments, { ethers }) {
    const kmsGenerationAddress = checksumAddress(taskArguments.kmsGenerationProxy, "Gateway KMSGeneration proxy");
    const gatewayConfigAddress = checksumAddress(taskArguments.gatewayConfigProxy, "GatewayConfig proxy");
    const legacyHostKmsVerifierAddress = taskArguments.legacyHostKmsVerifier
      ? checksumAddress(taskArguments.legacyHostKmsVerifier, "Legacy host KMSVerifier proxy")
      : null;

    const exportBlockNumber =
      taskArguments.blockTag === undefined ? await ethers.provider.getBlockNumber() : Number(taskArguments.blockTag);
    const exportBlock = await ethers.provider.getBlock(exportBlockNumber);
    if (exportBlock === null) {
      throw new Error(`Block ${exportBlockNumber} not found`);
    }

    const gatewayConfig = await ethers.getContractAt("GatewayConfig", gatewayConfigAddress);
    const kmsGeneration = await ethers.getContractAt("KMSGeneration", kmsGenerationAddress);

    const blockOpts = { blockTag: exportBlockNumber };
    const contextId = await gatewayConfig.getCurrentKmsContextId(blockOpts);

    const [kmsTxSenders, publicDecryptionThreshold, userDecryptionThreshold, mpcThreshold, kmsGenThreshold] =
      await Promise.all([
        gatewayConfig.getKmsTxSendersForContext(contextId, blockOpts),
        gatewayConfig.getPublicDecryptionThresholdForContext(contextId, blockOpts),
        gatewayConfig.getUserDecryptionThresholdForContext(contextId, blockOpts),
        gatewayConfig.getMpcThreshold(blockOpts),
        gatewayConfig.getKmsGenThreshold(blockOpts),
      ]);

    const [rawKmsNodes, storageState] = await Promise.all([
      Promise.all(kmsTxSenders.map((txSender) => gatewayConfig.getKmsNodeForContext(contextId, txSender, blockOpts))),
      readKmsStorageState(ethers, kmsGenerationAddress, exportBlockNumber),
    ]);
    const kmsNodes = rawKmsNodes.map((node) => ({
      txSenderAddress: getAddress(node.txSenderAddress),
      signerAddress: getAddress(node.signerAddress),
      ipAddress: node.ipAddress,
      storageUrl: node.storageUrl,
    }));

    const [
      [, activeKeyDigestsRaw],
      [, activeCrsDigest],
      keyConsensusTxSenders,
      crsConsensusTxSenders,
      prepKeygenConsensusTxSenders,
      prepKeygenParamsType,
      crsParamsType,
    ] = await Promise.all([
      kmsGeneration.getKeyMaterials(storageState.activeKeyId, blockOpts),
      kmsGeneration.getCrsMaterials(storageState.activeCrsId, blockOpts),
      kmsGeneration.getConsensusTxSenders(storageState.activeKeyId, blockOpts),
      kmsGeneration.getConsensusTxSenders(storageState.activeCrsId, blockOpts),
      kmsGeneration.getConsensusTxSenders(storageState.activePrepKeygenId, blockOpts),
      kmsGeneration.getKeyParamsType(storageState.activeKeyId, blockOpts),
      kmsGeneration.getCrsParamsType(storageState.activeCrsId, blockOpts),
    ]);

    const activeKeyDigests = activeKeyDigestsRaw.map((digest) => ({
      keyType: Number(digest.keyType),
      digest: digest.digest,
    }));

    const hostKmsGenerationMigrationState = {
      prepKeygenCounter: toBeHex(storageState.prepKeygenCounter),
      keyCounter: toBeHex(storageState.keyCounter),
      crsCounter: toBeHex(storageState.crsCounter),
      activeKeyId: toBeHex(storageState.activeKeyId),
      activeCrsId: toBeHex(storageState.activeCrsId),
      activePrepKeygenId: toBeHex(storageState.activePrepKeygenId),
      activeKeyDigests,
      activeCrsDigest,
      keyConsensusTxSenders: keyConsensusTxSenders.map(getAddress),
      keyConsensusDigest: storageState.keyConsensusDigest,
      crsConsensusTxSenders: crsConsensusTxSenders.map(getAddress),
      crsConsensusDigest: storageState.crsConsensusDigest,
      prepKeygenConsensusTxSenders: prepKeygenConsensusTxSenders.map(getAddress),
      prepKeygenConsensusDigest: storageState.prepKeygenConsensusDigest,
      crsMaxBitLength: storageState.crsMaxBitLength.toString(),
      prepKeygenParamsType: Number(prepKeygenParamsType),
      crsParamsType: Number(crsParamsType),
      contextId: toBeHex(contextId),
    };

    const thresholds = {
      publicDecryption: publicDecryptionThreshold.toString(),
      userDecryption: userDecryptionThreshold.toString(),
      kmsGen: kmsGenThreshold.toString(),
      mpc: mpcThreshold.toString(),
    };
    const migrationEnv = {
      ...buildMigrationEnv(hostKmsGenerationMigrationState),
      MIGRATION_KMS_NODES: JSON.stringify(kmsNodes),
      MIGRATION_KMS_THRESHOLDS: JSON.stringify(thresholds),
    };

    const migrationStateJson = {
      contextId: contextId.toString(),
      kmsNodes,
      thresholds,
      hostKmsGenerationMigrationState,
      metadata: {
        exportBlockNumber,
        exportTimestamp: new Date(Number(exportBlock.timestamp) * 1000).toISOString(),
        gatewayKmsGenerationProxy: kmsGenerationAddress,
        gatewayConfigProxy: gatewayConfigAddress,
        legacyHostKmsVerifierProxy: legacyHostKmsVerifierAddress,
      },
      export: migrationEnv,
    };

    // These assertions check that the gateway snapshot can be consumed by host KMSGeneration.initializeFromMigration(...).
    assertMigrationPrecondition(storageState.prepKeygenCounter === storageState.activePrepKeygenId);
    assertMigrationPrecondition(storageState.keyCounter === storageState.activeKeyId);
    assertMigrationPrecondition(storageState.crsCounter === storageState.activeCrsId);
    assertMigrationPrecondition(activeKeyDigests.length > 0);
    assertMigrationPrecondition(activeCrsDigest !== "0x");
    const assertConsensus = async (digest: string, senders: string[]) => {
      assertMigrationPrecondition(digest !== ZeroHash);
      assertMigrationPrecondition(BigInt(senders.length) >= kmsGenThreshold);
      const registrations = await Promise.all(
        senders.map((sender) => gatewayConfig.isKmsTxSenderForContext(contextId, sender, blockOpts)),
      );
      assertMigrationPrecondition(registrations.every((registered) => registered));
    };
    await Promise.all([
      assertConsensus(storageState.keyConsensusDigest, keyConsensusTxSenders),
      assertConsensus(storageState.crsConsensusDigest, crsConsensusTxSenders),
      assertConsensus(storageState.prepKeygenConsensusDigest, prepKeygenConsensusTxSenders),
    ]);

    const outputPath = path.resolve(process.cwd(), taskArguments.output);
    fs.mkdirSync(path.dirname(outputPath), { recursive: true });
    fs.writeFileSync(outputPath, `${JSON.stringify(migrationStateJson, null, 2)}\n`);

    console.log(`Exported gateway KMS migration state to ${outputPath}`);
  });
