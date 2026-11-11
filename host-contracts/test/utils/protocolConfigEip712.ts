// ProtocolConfig EIP-712 types, digest-input hashes and confirmation loops. The
// host-contracts tests use them to sign and submit lifecycle confirmations. library-solidity keeps a reduced copy.
import { AbiCoder, Signer, TypedDataDomain, TypedDataEncoder, TypedDataField, keccak256, solidityPacked } from 'ethers';

import type { ProtocolConfig } from '../../types';
import type {
  IProtocolConfig,
  KmsNodeParamsStruct,
  KmsThresholdsStruct,
  ProtocolConfigInterface,
} from '../../types/contracts/ProtocolConfig';

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
  ContextDestructionConfirmation: {
    ContextDestructionConfirmation: [
      { name: 'destroyedContextId', type: 'uint256' },
      { name: 'extraData', type: 'bytes' },
    ],
  },
  EpochDestructionConfirmation: {
    EpochDestructionConfirmation: [
      { name: 'destroyedEpochId', type: 'uint256' },
      { name: 'extraData', type: 'bytes' },
    ],
  },
} satisfies Record<string, Record<string, TypedDataField[]>>;

const KEYGEN_TYPES: Record<string, TypedDataField[]> = {
  KeygenVerification: [
    { name: 'prepKeygenId', type: 'uint256' },
    { name: 'keyId', type: 'uint256' },
    { name: 'keyDigests', type: 'KeyDigest[]' },
    { name: 'extraData', type: 'bytes' },
  ],
  KeyDigest: [
    { name: 'keyType', type: 'uint8' },
    { name: 'digest', type: 'bytes' },
  ],
};

const CRSGEN_TYPES: Record<string, TypedDataField[]> = {
  CrsgenVerification: [
    { name: 'crsId', type: 'uint256' },
    { name: 'maxBitLength', type: 'uint256' },
    { name: 'crsDigest', type: 'bytes' },
    { name: 'extraData', type: 'bytes' },
  ],
};

async function protocolConfigDomain(pc: ProtocolConfig): Promise<TypedDataDomain> {
  const { chainId } = await pc.runner!.provider!.getNetwork();
  return { name: 'ProtocolConfig', version: '1', chainId, verifyingContract: await pc.getAddress() };
}

const abi = AbiCoder.defaultAbiCoder();
const EXTRA_DATA_V2 = 2;

// keccak256(abi.encode(KmsNode[] nodes, KmsThresholds thresholds)). The encode types come from the
// ProtocolConfig ABI, so the tuple layout cannot drift from the contract. KmsNode is the projection of
// KmsNodeParams that ProtocolConfig stores, and the encoder drops the fields KmsNode does not have.
function hashNodeConfig(
  iface: ProtocolConfigInterface,
  params: KmsNodeParamsStruct[],
  thresholds: KmsThresholdsStruct,
): string {
  const [kmsNodesType] = iface.getFunction('getKmsNodesForContext').outputs;
  const [, thresholdsType] = iface.getFunction('defineNewKmsContextAndEpoch').inputs;
  return keccak256(abi.encode([kmsNodesType, thresholdsType], [params, thresholds]));
}

// keccak256(abi.encode(keyHashes, crsHashes)), where each entry is the EIP-712 hashStruct of its
// KeygenVerification or CrsgenVerification, with `materialExtraData` as the entry's extraData.
function hashEpochMaterial(
  keys: IProtocolConfig.EpochKeyResultStruct[],
  crsList: IProtocolConfig.EpochCrsResultStruct[],
  materialExtraData: string,
): string {
  const keyHashes = keys.map((key) =>
    TypedDataEncoder.hashStruct('KeygenVerification', KEYGEN_TYPES, { ...key, extraData: materialExtraData }),
  );
  const crsHashes = crsList.map((crs) =>
    TypedDataEncoder.hashStruct('CrsgenVerification', CRSGEN_TYPES, { ...crs, extraData: materialExtraData }),
  );
  return keccak256(abi.encode(['bytes32[]', 'bytes32[]'], [keyHashes, crsHashes]));
}

// A pending KMS context: its ID and the definition that defineNewKmsContext submitted.
export interface DefinedKmsContext {
  contextId: bigint;
  nodes: KmsNodeParamsStruct[];
  thresholds: KmsThresholdsStruct;
}

// Signs each signer's ContextCreationConfirmation for the pending context `context` and submits it from
// the signer account. The previous context is the active one.
// Returns the epoch ID of the NewKmsEpoch emitted once the creation quorum is reached.
export async function confirmContextCreation(
  pc: ProtocolConfig,
  signers: Signer[],
  { contextId, nodes, thresholds }: DefinedKmsContext,
  extraData = '0x',
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
      extraData,
    });
    const receipt = await (await pc.connect(signer).confirmKmsContextCreation(contextId, signature, extraData)).wait();
    const newEpoch = receipt!.logs
      .map((log) => pc.interface.parseLog(log))
      .find((event) => event?.name === 'NewKmsEpoch');
    epochId ??= newEpoch?.args.epochId;
  }
  return epochId;
}

// Signs each signer's EpochActivationConfirmation over `keys` and `crsList` and submits it from the signer
// account. `contextId` and `epochId` identify the epoch to activate. The previous epoch is the active one.
export async function confirmEpochActivation(
  pc: ProtocolConfig,
  signers: Signer[],
  contextId: bigint,
  epochId: bigint,
  { keys, crsList } = DUMMY_EPOCH_MATERIAL,
  extraData = '0x',
): Promise<void> {
  const domain = await protocolConfigDomain(pc);
  const [, previousEpochId] = await pc.getCurrentKmsContextAndEpoch();
  const materialExtraData = solidityPacked(['uint8', 'uint256', 'uint256'], [EXTRA_DATA_V2, contextId, epochId]);
  const materialHash = hashEpochMaterial(keys, crsList, materialExtraData);
  for (const signer of signers) {
    const signature = await signer.signTypedData(domain, CONFIRMATION_TYPES.EpochActivationConfirmation, {
      contextId,
      previousEpochId,
      epochId,
      epochMaterialHash: materialHash,
      extraData,
    });
    await (
      await pc.connect(signer).confirmEpochActivation(epochId, keys, crsList, materialExtraData, signature, extraData)
    ).wait();
  }
}

// One key result and one CRS result. An empty `keys` or `crsList` reverts with EmptyEpochActivationResults.
// The ids need not exist in KMSGeneration, because confirmEpochActivation checks only the signer.
export const DUMMY_EPOCH_MATERIAL: {
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

// Signs each signer's ContextDestructionConfirmation and submits it from the signer account.
export async function confirmContextDestruction(
  pc: ProtocolConfig,
  signers: Signer[],
  destroyedContextId: bigint,
): Promise<void> {
  const domain = await protocolConfigDomain(pc);
  for (const signer of signers) {
    const signature = await signer.signTypedData(domain, CONFIRMATION_TYPES.ContextDestructionConfirmation, {
      destroyedContextId,
      extraData: '0x',
    });
    await (await pc.connect(signer).confirmKmsContextDestruction(destroyedContextId, signature, '0x')).wait();
  }
}

// Signs each signer's EpochDestructionConfirmation and submits it from the signer account.
export async function confirmEpochDestruction(
  pc: ProtocolConfig,
  signers: Signer[],
  destroyedEpochId: bigint,
): Promise<void> {
  const domain = await protocolConfigDomain(pc);
  for (const signer of signers) {
    const signature = await signer.signTypedData(domain, CONFIRMATION_TYPES.EpochDestructionConfirmation, {
      destroyedEpochId,
      extraData: '0x',
    });
    await (await pc.connect(signer).confirmKmsEpochDestruction(destroyedEpochId, signature, '0x')).wait();
  }
}

export function findEventArg(
  contract: ProtocolConfig,
  logs: readonly { topics: readonly string[]; data: string }[],
  eventName: string,
  argName: string,
): bigint {
  for (const log of logs) {
    const parsed = contract.interface.parseLog(log);
    if (parsed?.name === eventName) {
      return parsed.args[argName] as bigint;
    }
  }
  throw new Error(`${eventName} not emitted`);
}
