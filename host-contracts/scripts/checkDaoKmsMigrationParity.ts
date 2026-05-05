import { calculateERC7201StorageLocation } from '@openzeppelin/upgrades-core/dist/utils/erc7201';
import { AbiCoder, Contract, JsonRpcProvider, keccak256, toBeHex, zeroPadValue } from 'ethers';

import {
  assertSame,
  loadKmsGenerationMigrationState,
  loadProtocolConfigMigrationState,
  normalizeAddress,
  normalizeAddresses,
  normalizeKmsGenerationMigrationState,
  normalizeNodes,
} from '../tasks/utils/daoKmsMigrationState';
import { getRequiredEnvVar } from '../tasks/utils/loadVariables';
import { KMSGeneration__factory, KMSVerifier__factory, ProtocolConfig__factory } from '../types';

// GatewayConfig lives in the gateway-contracts package; typechain factories from
// that package are not available here, so the ABI fragment stays inline.
const GATEWAY_CONFIG_ABI = [
  'function getCurrentKmsContextId() view returns (uint256)',
  'function getKmsSignersForContext(uint256) view returns (address[])',
  'function getKmsTxSendersForContext(uint256) view returns (address[])',
  'function getKmsNodeForContext(uint256,address) view returns ((address txSenderAddress,address signerAddress,string ipAddress,string storageUrl))',
  'function getPublicDecryptionThresholdForContext(uint256) view returns (uint256)',
  'function getUserDecryptionThresholdForContext(uint256) view returns (uint256)',
  'function getKmsGenThreshold() view returns (uint256)',
  'function getMpcThreshold() view returns (uint256)',
];

const HOST_KMS_GENERATION_STORAGE_LOCATION = BigInt(
  calculateERC7201StorageLocation('fhevm.storage.KMSGeneration'),
);

function storageSlot(offset: bigint): string {
  return toBeHex(HOST_KMS_GENERATION_STORAGE_LOCATION + offset, 32);
}

const HOST_KMS_GENERATION_SLOT = {
  isRequestDone: storageSlot(1n),
  consensusDigest: storageSlot(3n),
  prepKeygenCounter: storageSlot(4n),
  keygenIdPairs: storageSlot(6n),
  crsMaxBitLength: storageSlot(10n),
  requestParamsType: storageSlot(13n),
  requestExtraData: storageSlot(14n),
} as const;

const coder = AbiCoder.defaultAbiCoder();

function parseArgs(): string {
  const migrationStateIndex = process.argv.indexOf('--migration-state');
  const migrationStatePath = process.argv[migrationStateIndex + 1];
  if (migrationStateIndex === -1 || !migrationStatePath) {
    throw new Error('Usage: ts-node scripts/checkDaoKmsMigrationParity.ts --migration-state <path>');
  }
  return migrationStatePath;
}

function requiredAddressEnv(name: string): string {
  return normalizeAddress(getRequiredEnvVar(name), name);
}

function mappingSlot(key: bigint, slot: string): string {
  return keccak256(coder.encode(['uint256', 'uint256'], [key, BigInt(slot)]));
}

async function readStorageUint(provider: JsonRpcProvider, address: string, slot: string): Promise<bigint> {
  return BigInt(await provider.getStorage(address, slot));
}

async function readStorageWord(provider: JsonRpcProvider, address: string, slot: string): Promise<string> {
  return zeroPadValue(await provider.getStorage(address, slot), 32).toLowerCase();
}

async function readStorageBytes(provider: JsonRpcProvider, address: string, slot: string): Promise<string> {
  const rawHead = await readStorageWord(provider, address, slot);
  const encodedLength = BigInt(rawHead);
  if ((encodedLength & 1n) === 0n) {
    const length = Number((encodedLength & 0xffn) / 2n);
    return `0x${rawHead.slice(2, 2 + length * 2)}`;
  }

  const length = Number((encodedLength - 1n) / 2n);
  const baseSlot = BigInt(keccak256(slot));
  const wordCount = Math.ceil(length / 32);
  const words = await Promise.all(
    Array.from({ length: wordCount }, (_, index) =>
      readStorageWord(provider, address, toBeHex(baseSlot + BigInt(index), 32)),
    ),
  );
  return `0x${words
    .map((word) => word.slice(2))
    .join('')
    .slice(0, length * 2)}`;
}

function contextExtraData(contextId: bigint): string {
  return `0x01${toBeHex(contextId, 32).slice(2)}`;
}

function expectedStorageUrls(
  txSenders: string[],
  nodes: Array<{ txSenderAddress: string; storageUrl: string }>,
  label: string,
): string[] {
  const urlsByTxSender = new Map(nodes.map((node) => [node.txSenderAddress, node.storageUrl]));
  return txSenders.map((txSender, index) => {
    const storageUrl = urlsByTxSender.get(normalizeAddress(txSender, `${label}[${index}]`));
    if (storageUrl === undefined) {
      throw new Error(`${label}[${index}] has no matching KMS node in migration-state.json: ${txSender}`);
    }
    return storageUrl;
  });
}

async function main(): Promise<void> {
  const migrationStatePath = parseArgs();
  const expected = loadProtocolConfigMigrationState(migrationStatePath);
  const expectedKmsGeneration = loadKmsGenerationMigrationState(migrationStatePath);
  const contextId = BigInt(expected.contextId);
  const activeKeyId = BigInt(expectedKmsGeneration.activeKeyId);
  const activeCrsId = BigInt(expectedKmsGeneration.activeCrsId);
  const activePrepKeygenId = BigInt(expectedKmsGeneration.activePrepKeygenId);

  const hostProvider = new JsonRpcProvider(getRequiredEnvVar('HOST_RPC'));
  const gatewayProvider = new JsonRpcProvider(getRequiredEnvVar('GATEWAY_RPC'));
  const hostProtocolConfig = ProtocolConfig__factory.connect(
    requiredAddressEnv('HOST_PROTOCOL_CONFIG_PROXY'),
    hostProvider,
  );
  const hostKmsGenerationAddress = requiredAddressEnv('HOST_KMS_GENERATION_PROXY');
  const hostKmsGeneration = KMSGeneration__factory.connect(hostKmsGenerationAddress, hostProvider);
  const hostKmsVerifier = KMSVerifier__factory.connect(requiredAddressEnv('HOST_KMS_VERIFIER_PROXY'), hostProvider);
  const gatewayConfig = new Contract(requiredAddressEnv('GATEWAY_CONFIG_PROXY'), GATEWAY_CONFIG_ABI, gatewayProvider);
  const readUint = (slot: string) => readStorageUint(hostProvider, hostKmsGenerationAddress, slot);
  const readWord = (slot: string) => readStorageWord(hostProvider, hostKmsGenerationAddress, slot);
  const readBytes = (slot: string) => readStorageBytes(hostProvider, hostKmsGenerationAddress, slot);

  const [
    hostContextId,
    hostSigners,
    hostNodes,
    hostPublicThreshold,
    hostUserThreshold,
    hostKmsGenThreshold,
    hostMpcThreshold,
    gatewayContextId,
    gatewaySigners,
    gatewayTxSenders,
    gatewayPublicThreshold,
    gatewayUserThreshold,
    gatewayKmsGenThreshold,
    gatewayMpcThreshold,
    kmsGenerationActiveKeyId,
    kmsGenerationActiveCrsId,
    kmsGenerationKeyCounter,
    kmsGenerationCrsCounter,
    kmsGenerationPrepKeygenCounter,
    kmsGenerationActivePrepKeygenId,
    kmsGenerationPairedKeyId,
    kmsGenerationCrsMaxBitLength,
    kmsGenerationPrepKeygenParamsType,
    kmsGenerationKeyParamsType,
    kmsGenerationCrsParamsType,
    kmsGenerationKeyConsensusDigest,
    kmsGenerationCrsConsensusDigest,
    kmsGenerationPrepKeygenConsensusDigest,
    kmsGenerationKeyConsensusTxSenders,
    kmsGenerationCrsConsensusTxSenders,
    kmsGenerationPrepKeygenConsensusTxSenders,
    kmsGenerationKeyMaterials,
    kmsGenerationCrsMaterials,
    kmsGenerationPrepExtraData,
    kmsGenerationKeyExtraData,
    kmsGenerationCrsExtraData,
    kmsGenerationPrepDone,
    kmsGenerationKeyDone,
    kmsGenerationCrsDone,
    kmsVerifierContextId,
    kmsVerifierSigners,
    kmsVerifierThreshold,
    kmsVerifierExtraDataResult,
  ] = await Promise.all([
    hostProtocolConfig.getCurrentKmsContextId(),
    hostProtocolConfig.getKmsSignersForContext(contextId),
    hostProtocolConfig.getKmsNodesForContext(contextId),
    hostProtocolConfig.getPublicDecryptionThreshold(),
    hostProtocolConfig.getUserDecryptionThreshold(),
    hostProtocolConfig.getKmsGenThreshold(),
    hostProtocolConfig.getMpcThreshold(),
    gatewayConfig.getCurrentKmsContextId(),
    gatewayConfig.getKmsSignersForContext(contextId),
    gatewayConfig.getKmsTxSendersForContext(contextId),
    gatewayConfig.getPublicDecryptionThresholdForContext(contextId),
    gatewayConfig.getUserDecryptionThresholdForContext(contextId),
    gatewayConfig.getKmsGenThreshold(),
    gatewayConfig.getMpcThreshold(),
    hostKmsGeneration.getActiveKeyId(),
    hostKmsGeneration.getActiveCrsId(),
    hostKmsGeneration.getKeyCounter(),
    hostKmsGeneration.getCrsCounter(),
    readUint(HOST_KMS_GENERATION_SLOT.prepKeygenCounter),
    readUint(mappingSlot(activeKeyId, HOST_KMS_GENERATION_SLOT.keygenIdPairs)),
    readUint(mappingSlot(activePrepKeygenId, HOST_KMS_GENERATION_SLOT.keygenIdPairs)),
    readUint(mappingSlot(activeCrsId, HOST_KMS_GENERATION_SLOT.crsMaxBitLength)),
    readUint(mappingSlot(activePrepKeygenId, HOST_KMS_GENERATION_SLOT.requestParamsType)),
    hostKmsGeneration.getKeyParamsType(activeKeyId),
    hostKmsGeneration.getCrsParamsType(activeCrsId),
    readWord(mappingSlot(activeKeyId, HOST_KMS_GENERATION_SLOT.consensusDigest)),
    readWord(mappingSlot(activeCrsId, HOST_KMS_GENERATION_SLOT.consensusDigest)),
    readWord(mappingSlot(activePrepKeygenId, HOST_KMS_GENERATION_SLOT.consensusDigest)),
    hostKmsGeneration.getConsensusTxSenders(activeKeyId),
    hostKmsGeneration.getConsensusTxSenders(activeCrsId),
    hostKmsGeneration.getConsensusTxSenders(activePrepKeygenId),
    hostKmsGeneration.getKeyMaterials(activeKeyId),
    hostKmsGeneration.getCrsMaterials(activeCrsId),
    readBytes(mappingSlot(activePrepKeygenId, HOST_KMS_GENERATION_SLOT.requestExtraData)),
    readBytes(mappingSlot(activeKeyId, HOST_KMS_GENERATION_SLOT.requestExtraData)),
    readBytes(mappingSlot(activeCrsId, HOST_KMS_GENERATION_SLOT.requestExtraData)),
    readUint(mappingSlot(activePrepKeygenId, HOST_KMS_GENERATION_SLOT.isRequestDone)),
    readUint(mappingSlot(activeKeyId, HOST_KMS_GENERATION_SLOT.isRequestDone)),
    readUint(mappingSlot(activeCrsId, HOST_KMS_GENERATION_SLOT.isRequestDone)),
    hostKmsVerifier.getCurrentKmsContextId(),
    hostKmsVerifier.getKmsSigners(),
    hostKmsVerifier.getThreshold(),
    hostKmsVerifier.getContextSignersAndThresholdFromExtraData(contextExtraData(contextId)),
  ]);

  const gatewayNodes = await Promise.all(
    normalizeAddresses(gatewayTxSenders, 'gateway GatewayConfig tx sender set').map((txSender) =>
      gatewayConfig.getKmsNodeForContext(contextId, txSender),
    ),
  );

  for (const [name, actual] of [
    ['host ProtocolConfig contextId', hostContextId],
    ['gateway GatewayConfig contextId', gatewayContextId],
  ]) {
    assertSame(name, actual.toString(), expected.contextId);
  }

  for (const [name, actual] of [
    ['host ProtocolConfig signer set', hostSigners],
    ['gateway GatewayConfig signer set', gatewaySigners],
  ]) {
    assertSame(name, normalizeAddresses(actual, name), expected.signers);
  }

  assertSame('host ProtocolConfig nodes', normalizeNodes(hostNodes, 'host ProtocolConfig nodes'), expected.nodes);
  assertSame(
    'gateway GatewayConfig nodes',
    normalizeNodes(gatewayNodes, 'gateway GatewayConfig nodes'),
    expected.nodes,
  );

  for (const [name, actual, expectedValue] of [
    ['host public threshold', hostPublicThreshold, expected.thresholds.publicDecryption],
    ['host user threshold', hostUserThreshold, expected.thresholds.userDecryption],
    ['host kmsGen threshold', hostKmsGenThreshold, expected.thresholds.kmsGen],
    ['host mpc threshold', hostMpcThreshold, expected.thresholds.mpc],
    ['gateway public threshold', gatewayPublicThreshold, expected.thresholds.publicDecryption],
    ['gateway user threshold', gatewayUserThreshold, expected.thresholds.userDecryption],
    ['gateway kmsGen threshold', gatewayKmsGenThreshold, expected.thresholds.kmsGen],
    ['gateway mpc threshold', gatewayMpcThreshold, expected.thresholds.mpc],
  ]) {
    assertSame(name, actual.toString(), expectedValue);
  }

  const [kmsGenerationKeyUrls, kmsGenerationActiveKeyDigests] = kmsGenerationKeyMaterials;
  const [kmsGenerationCrsUrls, kmsGenerationActiveCrsDigest] = kmsGenerationCrsMaterials;
  const actualKmsGeneration = normalizeKmsGenerationMigrationState(
    {
      prepKeygenCounter: kmsGenerationPrepKeygenCounter,
      keyCounter: kmsGenerationKeyCounter,
      crsCounter: kmsGenerationCrsCounter,
      activeKeyId: kmsGenerationActiveKeyId,
      activeCrsId: kmsGenerationActiveCrsId,
      activePrepKeygenId: kmsGenerationActivePrepKeygenId,
      activeKeyDigests: kmsGenerationActiveKeyDigests,
      activeCrsDigest: kmsGenerationActiveCrsDigest,
      keyConsensusTxSenders: kmsGenerationKeyConsensusTxSenders,
      keyConsensusDigest: kmsGenerationKeyConsensusDigest,
      crsConsensusTxSenders: kmsGenerationCrsConsensusTxSenders,
      crsConsensusDigest: kmsGenerationCrsConsensusDigest,
      prepKeygenConsensusTxSenders: kmsGenerationPrepKeygenConsensusTxSenders,
      prepKeygenConsensusDigest: kmsGenerationPrepKeygenConsensusDigest,
      crsMaxBitLength: kmsGenerationCrsMaxBitLength,
      prepKeygenParamsType: kmsGenerationPrepKeygenParamsType,
      crsParamsType: kmsGenerationCrsParamsType,
      contextId: expected.contextId,
    },
    'host KMSGeneration migration state',
  );

  assertSame('host KMSGeneration migration state', actualKmsGeneration, expectedKmsGeneration);
  assertSame(
    'host KMSGeneration prep-keygen paired key ID',
    kmsGenerationPairedKeyId.toString(),
    activeKeyId.toString(),
  );
  assertSame('host KMSGeneration prep-keygen done', kmsGenerationPrepDone.toString(), '1');
  assertSame('host KMSGeneration key done', kmsGenerationKeyDone.toString(), '1');
  assertSame('host KMSGeneration CRS done', kmsGenerationCrsDone.toString(), '1');
  assertSame(
    'host KMSGeneration key params type',
    kmsGenerationKeyParamsType.toString(),
    expectedKmsGeneration.prepKeygenParamsType,
  );
  assertSame('host KMSGeneration prep-keygen extraData', kmsGenerationPrepExtraData, contextExtraData(contextId));
  assertSame('host KMSGeneration key extraData', kmsGenerationKeyExtraData, contextExtraData(contextId));
  assertSame('host KMSGeneration CRS extraData', kmsGenerationCrsExtraData, contextExtraData(contextId));
  assertSame(
    'host KMSGeneration key material storage URLs',
    Array.from(kmsGenerationKeyUrls),
    expectedStorageUrls(expectedKmsGeneration.keyConsensusTxSenders, expected.nodes, 'key consensus tx senders'),
  );
  assertSame(
    'host KMSGeneration CRS material storage URLs',
    Array.from(kmsGenerationCrsUrls),
    expectedStorageUrls(expectedKmsGeneration.crsConsensusTxSenders, expected.nodes, 'CRS consensus tx senders'),
  );

  const [kmsVerifierExtraDataSigners, kmsVerifierExtraDataThreshold] = kmsVerifierExtraDataResult;

  assertSame('host KMSVerifier contextId', kmsVerifierContextId.toString(), expected.contextId);
  assertSame(
    'host KMSVerifier signer set',
    normalizeAddresses(kmsVerifierSigners, 'host KMSVerifier signers'),
    expected.signers,
  );
  assertSame('host KMSVerifier threshold', kmsVerifierThreshold.toString(), expected.thresholds.publicDecryption);
  assertSame(
    'host KMSVerifier extraData signer set',
    normalizeAddresses(kmsVerifierExtraDataSigners, 'host KMSVerifier extraData signers'),
    expected.signers,
  );
  assertSame(
    'host KMSVerifier extraData threshold',
    kmsVerifierExtraDataThreshold.toString(),
    expected.thresholds.publicDecryption,
  );

  console.log(`DAO KMS migration parity check passed for context ${expected.contextId}`);
}

main().catch((error: unknown) => {
  console.error(error);
  process.exitCode = 1;
});
