// Copy of the host-contracts ProtocolConfig EIP-712 test helpers (host-contracts/test/utils/protocolConfigEip712.ts),
// reduced to what OnchainPublicDecrypt.ts uses to define and activate a KMS context.
import { AbiCoder, Signer, TypedDataDomain, TypedDataEncoder, TypedDataField, keccak256, solidityPacked } from 'ethers';

import type { ProtocolConfig } from '@fhevm/host-contracts/types';
import type {
  IProtocolConfig,
  KmsNodeParamsStruct,
  KmsThresholdsStruct,
  ProtocolConfigInterface,
} from '@fhevm/host-contracts/types/contracts/ProtocolConfig';

const CONFIRMATION_TYPES = {
  ContextCreationConfirmation: {
    ContextCreationConfirmation: [
      { name: 'previousContextId', type: 'uint256' },
      { name: 'newContextId', type: 'uint256' },
      { name: 'nodeConfigHash', type: 'bytes32' },
      { name: 'extraData', type: 'bytes' },
    ],
  },
  EpochActivationConfirmation: {
    EpochActivationConfirmation: [
      { name: 'contextId', type: 'uint256' },
      { name: 'previousEpochId', type: 'uint256' },
      { name: 'epochId', type: 'uint256' },
      { name: 'epochMaterialHash', type: 'bytes32' },
      { name: 'extraData', type: 'bytes' },
    ],
  },
} satisfies Record<string, Record<string, TypedDataField[]>>;

const KEY_DIGEST_TYPES: Record<string, TypedDataField[]> = {
  KeyDigest: [
    { name: 'keyType', type: 'uint8' },
    { name: 'digest', type: 'bytes' },
  ],
};

async function protocolConfigDomain(pc: ProtocolConfig): Promise<TypedDataDomain> {
  const { chainId } = await pc.runner!.provider!.getNetwork();
  return { name: 'ProtocolConfig', version: '1', chainId, verifyingContract: await pc.getAddress() };
}

const abi = AbiCoder.defaultAbiCoder();

// keccak256(abi.encode(KmsNode[] nodes, KmsThresholds thresholds)). The encode types come from the
// ProtocolConfig ABI, so the tuple layout always matches the contract. KmsNode holds the KmsNodeParams
// fields that ProtocolConfig stores. The encoder drops the other fields.
function hashNodeConfig(
  iface: ProtocolConfigInterface,
  params: KmsNodeParamsStruct[],
  thresholds: KmsThresholdsStruct,
): string {
  const [kmsNodesType] = iface.getFunction('getKmsNodesForContext').outputs;
  const [, thresholdsType] = iface.getFunction('defineNewKmsContextAndEpoch').inputs;
  return keccak256(abi.encode([kmsNodesType, thresholdsType], [params, thresholds]));
}

// keccak256(abi.encode(prepKeygenId, keyId, keyDigestsHash)), where keyDigestsHash is the EIP-712 hash of
// the KeyDigest[] array.
function hashKeyResult(key: IProtocolConfig.EpochKeyResultStruct): string {
  const keyDigestHashes = key.keyDigests.map((d) => TypedDataEncoder.hashStruct('KeyDigest', KEY_DIGEST_TYPES, d));
  const keyDigestsHash = keccak256(solidityPacked(['bytes32[]'], [keyDigestHashes]));
  return keccak256(abi.encode(['uint256', 'uint256', 'bytes32'], [key.prepKeygenId, key.keyId, keyDigestsHash]));
}

// keccak256(abi.encode(crsId, maxBitLength, crsDigest)). Unlike hashKeyResult, crsDigest is ABI-encoded as raw
// dynamic bytes, not hashed first.
function hashCrsResult(crs: IProtocolConfig.EpochCrsResultStruct): string {
  return keccak256(abi.encode(['uint256', 'uint256', 'bytes'], [crs.crsId, crs.maxBitLength, crs.crsDigest]));
}

function hashEpochMaterial(
  keys: IProtocolConfig.EpochKeyResultStruct[],
  crsList: IProtocolConfig.EpochCrsResultStruct[],
): string {
  return keccak256(abi.encode(['bytes32[]', 'bytes32[]'], [keys.map(hashKeyResult), crsList.map(hashCrsResult)]));
}

// A pending KMS context: its ID and the definition that defineNewKmsContext submitted.
interface DefinedKmsContext {
  contextId: bigint;
  nodes: KmsNodeParamsStruct[];
  thresholds: KmsThresholdsStruct;
}

// Signs each signer's ContextCreationConfirmation for the pending context `context` with empty extraData and
// submits it from the signer account. The previous context is the active one.
// Returns the epoch ID of the NewKmsEpoch emitted once the creation quorum is reached.
async function confirmContextCreation(
  pc: ProtocolConfig,
  signers: Signer[],
  { contextId, nodes, thresholds }: DefinedKmsContext,
): Promise<bigint | undefined> {
  const domain = await protocolConfigDomain(pc);
  const previousContextId = await pc.getCurrentKmsContextId();
  const configHash = hashNodeConfig(pc.interface, nodes, thresholds);
  let epochId: bigint | undefined;
  for (const signer of signers) {
    const signature = await signer.signTypedData(domain, CONFIRMATION_TYPES.ContextCreationConfirmation, {
      previousContextId,
      newContextId: contextId,
      nodeConfigHash: configHash,
      extraData: '0x',
    });
    const receipt = await (await pc.connect(signer).confirmKmsContextCreation(contextId, signature, '0x')).wait();
    epochId ??= findEventArg(pc, receipt!.logs, 'NewKmsEpoch', 'epochId');
  }
  return epochId;
}

// Signs each signer's EpochActivationConfirmation over DUMMY_EPOCH_MATERIAL with empty extraData and submits
// it from the signer account. `contextId` and `epochId` identify the epoch to activate. The previous epoch is
// the active one.
async function confirmEpochActivation(
  pc: ProtocolConfig,
  signers: Signer[],
  contextId: bigint,
  epochId: bigint,
): Promise<void> {
  const { keys, crsList } = DUMMY_EPOCH_MATERIAL;
  const domain = await protocolConfigDomain(pc);
  const [, previousEpochId] = await pc.getCurrentKmsContextAndEpoch();
  const materialHash = hashEpochMaterial(keys, crsList);
  for (const signer of signers) {
    const signature = await signer.signTypedData(domain, CONFIRMATION_TYPES.EpochActivationConfirmation, {
      contextId,
      previousEpochId,
      epochId,
      epochMaterialHash: materialHash,
      extraData: '0x',
    });
    await (await pc.connect(signer).confirmEpochActivation(epochId, keys, crsList, signature, '0x')).wait();
  }
}

// DUMMY_EPOCH_MATERIAL has one key result and one CRS result, because an empty `keys` or `crsList` reverts
// with EmptyEpochActivationResults.
// The ids need not exist in KMSGeneration, because confirmEpochActivation checks only the signer.
const DUMMY_EPOCH_MATERIAL: {
  keys: IProtocolConfig.EpochKeyResultStruct[];
  crsList: IProtocolConfig.EpochCrsResultStruct[];
} = {
  keys: [{ prepKeygenId: 1n, keyId: 1n, keyDigests: [{ keyType: 0, digest: '0x01020304' }] }],
  crsList: [{ crsId: 1n, maxBitLength: 4096n, crsDigest: '0x01020304' }],
};

// Defines a new KMS context from the `pc` runner, which must be the owner. Returns the new context ID
// with its definition.
export async function defineNewKmsContext(
  pc: ProtocolConfig,
  nodes: KmsNodeParamsStruct[],
  thresholds: KmsThresholdsStruct,
): Promise<DefinedKmsContext> {
  const receipt = await (await pc.defineNewKmsContextAndEpoch(nodes, thresholds, '', [])).wait();
  const contextId = findEventArg(pc, receipt!.logs, 'NewKmsContext', 'contextId');
  if (contextId === undefined) {
    throw new Error('NewKmsContext not emitted');
  }
  return { contextId, nodes, thresholds };
}

// Confirms the creation of the pending context `context` and activates its epoch over
// DUMMY_EPOCH_MATERIAL. `signers` must reach both quorums.
export async function activateDefinedKmsContext(
  pc: ProtocolConfig,
  signers: Signer[],
  context: DefinedKmsContext,
): Promise<void> {
  const epochId = await confirmContextCreation(pc, signers, context);
  if (epochId === undefined) {
    throw new Error('Context creation quorum did not emit NewKmsEpoch');
  }
  await confirmEpochActivation(pc, signers, context.contextId, epochId);
}

function findEventArg(
  contract: ProtocolConfig,
  logs: readonly { topics: readonly string[]; data: string }[],
  eventName: string,
  argName: string,
): bigint | undefined {
  for (const log of logs) {
    const parsed = contract.interface.parseLog(log);
    if (parsed?.name === eventName) {
      return parsed.args[argName] as bigint;
    }
  }
  return undefined;
}
