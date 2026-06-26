import { task, types } from 'hardhat/config';
import { HardhatRuntimeEnvironment } from 'hardhat/types';

import { getRequiredEnvVar, loadGatewayAddresses } from './utils';

// This file defines tasks to perform the Gateway side of a KMS context switch: build the cross-chain
// `GatewayConfig.updateKmsContext` governance proposal triple for the protocol-apps
// governance-proposal-builder (DAO path, never broadcasts) or broadcast the same call directly with
// the deployer key (devnet / test-suite). Inputs come from the `KMS_*` env vars.

const GATEWAY_CONFIG_ADDRESS_ENV_VAR = 'GATEWAY_CONFIG_ADDRESS';

// Reads the post-switch KMS node set from the same indexed env vars the Gateway deployment task uses
// (tasks/deployment/contracts.ts), as field-ordered tuples matching the Solidity KmsNode layout
// (txSenderAddress, signerAddress, ipAddress, storageUrl). The Gateway KmsNode carries no MPC
// metadata, so only these four fields are read.
function buildKmsNodeTuplesFromEnv(): [string, string, string, string][] {
  const numNodes = parseInt(getRequiredEnvVar('NUM_KMS_NODES'));
  const nodes: [string, string, string, string][] = [];
  for (let idx = 0; idx < numNodes; idx++) {
    nodes.push([
      getRequiredEnvVar(`KMS_TX_SENDER_ADDRESS_${idx}`),
      getRequiredEnvVar(`KMS_SIGNER_ADDRESS_${idx}`),
      getRequiredEnvVar(`KMS_NODE_IP_ADDRESS_${idx}`),
      getRequiredEnvVar(`KMS_NODE_STORAGE_URL_${idx}`),
    ]);
  }
  return nodes;
}

// `updateKmsContext` takes the thresholds in (mpc, publicDecryption, userDecryption, kmsGen) order.
// The env var names mirror the Gateway deployment task — note KMS_GENERATION_THRESHOLD (the host
// uses KMS_GEN_THRESHOLD; the two repos differ here). The on-chain GatewayConfig checks validate the
// node set and thresholds, so this stays a thin encoder and relies on decode-before-approve.
function buildUpdateKmsContextArgs(newContextId: bigint): unknown[] {
  return [
    newContextId,
    buildKmsNodeTuplesFromEnv(),
    getRequiredEnvVar('MPC_THRESHOLD'),
    getRequiredEnvVar('PUBLIC_DECRYPTION_THRESHOLD'),
    getRequiredEnvVar('USER_DECRYPTION_THRESHOLD'),
    getRequiredEnvVar('KMS_GENERATION_THRESHOLD'),
  ];
}

function resolveGatewayConfigAddress(useInternalProxyAddress: boolean, override?: string): string {
  if (override && override.trim() !== '') {
    return override;
  }
  if (useInternalProxyAddress) {
    loadGatewayAddresses();
  }
  return getRequiredEnvVar(GATEWAY_CONFIG_ADDRESS_ENV_VAR);
}

interface EncodedUpdateKmsContext {
  newContextId: bigint;
  functionSignature: string;
  fullCalldata: string;
  argsData: string;
  decodedArgs: unknown[];
}

export async function encodeUpdateKmsContext(hre: HardhatRuntimeEnvironment): Promise<EncodedUpdateKmsContext> {
  const newContextId = BigInt(getRequiredEnvVar('KMS_CONTEXT_ID'));

  await hre.run('compile:specific', { contract: 'contracts' });
  const artifact = await hre.artifacts.readArtifact('GatewayConfig');
  const iface = new hre.ethers.Interface(artifact.abi);

  const args = buildUpdateKmsContextArgs(newContextId);
  const functionSignature = iface.getFunction('updateKmsContext')!.format('sighash');
  const fullCalldata = iface.encodeFunctionData('updateKmsContext', args);
  // Strip the 4-byte selector: governance-proposal-builder pairs the signature with selector-less
  // argument data.
  const argsData = '0x' + fullCalldata.slice(10);
  const decodedArgs = iface.decodeFunctionData('updateKmsContext', fullCalldata).toArray();

  return { newContextId, functionSignature, fullCalldata, argsData, decodedArgs };
}

// Asserts the configured newContextId is strictly greater than the live GatewayConfig context id.
async function assertContextIdAdvances(
  hre: HardhatRuntimeEnvironment,
  target: string,
  newContextId: bigint,
): Promise<void> {
  const gatewayConfig = await hre.ethers.getContractAt('GatewayConfig', target);
  const currentContextId: bigint = await gatewayConfig.getCurrentKmsContextId();
  if (newContextId <= currentContextId) {
    throw new Error(
      `New KMS context ID ${newContextId} must be strictly greater than the GatewayConfig current context id ` +
        `${currentContextId} at ${target}.`,
    );
  }
}

task(
  'task:buildUpdateKmsContextProposal',
  'Builds the cross-chain Gateway governance proposal triple for GatewayConfig.updateKmsContext from KMS_* env vars (DAO path, never broadcasts)',
)
  .addOptionalParam(
    'useInternalProxyAddress',
    'Resolve the GatewayConfig address from the /addresses directory instead of the environment',
    false,
    types.boolean,
  )
  .addOptionalParam(
    'gatewayConfigAddress',
    'GatewayConfig proxy address to target; overrides the environment / addresses directory',
    undefined,
    types.string,
  )
  .addOptionalParam(
    'verifyContextId',
    'Read the GatewayConfig current context id and assert the configured newContextId is strictly greater',
    true,
    types.boolean,
  )
  .setAction(async function (
    { useInternalProxyAddress, gatewayConfigAddress, verifyContextId },
    hre: HardhatRuntimeEnvironment,
  ): Promise<void> {
    const encoded = await encodeUpdateKmsContext(hre);
    const target = resolveGatewayConfigAddress(useInternalProxyAddress, gatewayConfigAddress);

    if (verifyContextId) {
      await assertContextIdAdvances(hre, target, encoded.newContextId);
    }

    console.log('GatewayConfig.updateKmsContext');
    console.log('  newContextId (must match the Ethereum proposal):', encoded.newContextId.toString());
    console.log('  target:', target);
    console.log('  signature:', encoded.functionSignature);
    console.log('  calldata:', encoded.fullCalldata);
  });

task(
  'task:updateKmsContext',
  'Broadcasts GatewayConfig.updateKmsContext from KMS_* env vars with the deployer key (no-DAO path for devnet / test-suite)',
)
  .addOptionalParam(
    'useInternalProxyAddress',
    'Resolve the GatewayConfig address from the /addresses directory instead of the environment',
    false,
    types.boolean,
  )
  .addOptionalParam(
    'gatewayConfigAddress',
    'GatewayConfig proxy address to target; overrides the environment / addresses directory',
    undefined,
    types.string,
  )
  .setAction(async function (
    { useInternalProxyAddress, gatewayConfigAddress },
    hre: HardhatRuntimeEnvironment,
  ): Promise<void> {
    const encoded = await encodeUpdateKmsContext(hre);
    const target = resolveGatewayConfigAddress(useInternalProxyAddress, gatewayConfigAddress);
    await assertContextIdAdvances(hre, target, encoded.newContextId);

    const deployer = new hre.ethers.Wallet(getRequiredEnvVar('DEPLOYER_PRIVATE_KEY')).connect(hre.ethers.provider);
    const tx = await deployer.sendTransaction({ to: target, data: encoded.fullCalldata });
    await tx.wait();
    console.log(
      `Broadcast updateKmsContext on ${target} (tx: ${tx.hash}). Current KMS context id is now ${encoded.newContextId}.`,
    );
  });
