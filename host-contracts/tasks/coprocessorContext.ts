import type { Interface, Result } from 'ethers';
import { task, types } from 'hardhat/config';
import type { HardhatRuntimeEnvironment } from 'hardhat/types';

import { getCoprocessorConfig } from './coprocessorConfigMirror';
import { broadcast } from './kmsContext';
import { buildCoprocessorContextArgs } from './taskDeploy';
import {
  COPROCESSOR_CONFIG_ADDRESS_ENV_VAR,
  type EncodedCall,
  addUseInternalProxyAddressParam,
  getCoprocessorConfigInterface,
  requireCoprocessorConfigAddress,
  resolveCoprocessorConfigAddress,
} from './utils/coprocessorConfigTasks';
import { toJsonString } from './utils/upgradeProposal';

// This file defines tasks that change the coprocessor contexts of the canonical (Ethereum)
// CoprocessorConfig: build the `defineNewCoprocessorContext` / `destroyCoprocessorContext` governance
// proposal calldata (DAO path, never broadcasts) or broadcast it directly with the deployer key
// (devnet / test-suite). The new set comes from the `COPROCESSOR_*` env vars (reusing
// `buildCoprocessorContextArgs`); the destroy tasks take the target ID directly. Replicas follow with
// the tasks in mirrorCoprocessorContext.ts.

// The new context takes the next ID. The operator sets it as the Gateway proposal's coprocessor
// context ID so the host and Gateway proposals stay aligned.
export async function predictNewCoprocessorContextId(
  hre: HardhatRuntimeEnvironment,
  coprocessorConfigAddress: string,
): Promise<bigint> {
  const coprocessorConfig = await getCoprocessorConfig(hre, coprocessorConfigAddress);
  return (await coprocessorConfig.getCurrentCoprocessorContextId()) + 1n;
}

export function encodeDefineNewCoprocessorContext(iface: Interface): EncodedCall {
  const args = buildCoprocessorContextArgs();
  const functionSignature = iface.getFunction('defineNewCoprocessorContext')!.format('sighash');
  const calldata = iface.encodeFunctionData('defineNewCoprocessorContext', args);
  // Key each struct by field name, so a reviewer can tell a tx sender from a signer.
  const [coprocessors, majorityThreshold] = iface.decodeFunctionData('defineNewCoprocessorContext', calldata);
  const decodedArgs = [coprocessors.map((coprocessor: Result) => coprocessor.toObject()), majorityThreshold];
  return { functionSignature, calldata, decodedArgs };
}

export function encodeDestroyCoprocessorContext(iface: Interface, coprocessorContextId: bigint): EncodedCall {
  const functionSignature = iface.getFunction('destroyCoprocessorContext')!.format('sighash');
  const calldata = iface.encodeFunctionData('destroyCoprocessorContext', [coprocessorContextId]);
  return { functionSignature, calldata, decodedArgs: [coprocessorContextId] };
}

addUseInternalProxyAddressParam(
  task(
    'task:buildDefineNewCoprocessorContextCalldata',
    'Builds Aragon proposal calldata for CoprocessorConfig.defineNewCoprocessorContext from COPROCESSOR_* env vars (DAO path, never broadcasts)',
  ),
).setAction(async function ({ useInternalProxyAddress }, hre): Promise<void> {
  const iface = await getCoprocessorConfigInterface(hre);
  const encoded = encodeDefineNewCoprocessorContext(iface);
  const target = resolveCoprocessorConfigAddress(useInternalProxyAddress);
  const newContextId = target ? (await predictNewCoprocessorContextId(hre, target)).toString() : undefined;

  console.log('CoprocessorConfig.defineNewCoprocessorContext');
  if (newContextId) {
    console.log("  newContextId (set as the Gateway proposal's coprocessor context ID):", newContextId);
  }
  console.log(
    '  target:',
    target ?? `<unresolved — set ${COPROCESSOR_CONFIG_ADDRESS_ENV_VAR} or pass --use-internal-proxy-address>`,
  );
  console.log('  decodedArgs:', toJsonString(encoded.decodedArgs));
  console.log('  calldata:', encoded.calldata);
});

addUseInternalProxyAddressParam(
  task(
    'task:defineNewCoprocessorContext',
    'Broadcasts CoprocessorConfig.defineNewCoprocessorContext from COPROCESSOR_* env vars with the deployer key (no-DAO path for devnet / test-suite)',
  ),
).setAction(async function ({ useInternalProxyAddress }, hre): Promise<void> {
  const iface = await getCoprocessorConfigInterface(hre);
  const { calldata } = encodeDefineNewCoprocessorContext(iface);
  const target = requireCoprocessorConfigAddress(useInternalProxyAddress);
  const newContextId = await predictNewCoprocessorContextId(hre, target);
  const hash = await broadcast(hre, target, calldata);
  console.log(
    `Broadcast defineNewCoprocessorContext on ${target} (tx: ${hash}). Coprocessor context ${newContextId} is now current.`,
  );
});

task(
  'task:buildDestroyCoprocessorContextCalldata',
  'Builds Aragon proposal calldata for CoprocessorConfig.destroyCoprocessorContext (retires a past context; DAO path, never broadcasts)',
)
  .addParam('coprocessorContextId', 'The coprocessor context ID to destroy', undefined, types.string)
  .setAction(async function ({ coprocessorContextId }, hre): Promise<void> {
    const iface = await getCoprocessorConfigInterface(hre);
    const encoded = encodeDestroyCoprocessorContext(iface, BigInt(coprocessorContextId));

    console.log('CoprocessorConfig.destroyCoprocessorContext');
    console.log('  coprocessorContextId:', coprocessorContextId);
    console.log('  calldata:', encoded.calldata);
  });

addUseInternalProxyAddressParam(
  task(
    'task:destroyCoprocessorContext',
    'Broadcasts CoprocessorConfig.destroyCoprocessorContext with the deployer key (no-DAO path for devnet / test-suite)',
  ).addParam('coprocessorContextId', 'The coprocessor context ID to destroy', undefined, types.string),
).setAction(async function ({ coprocessorContextId, useInternalProxyAddress }, hre): Promise<void> {
  const iface = await getCoprocessorConfigInterface(hre);
  const { calldata } = encodeDestroyCoprocessorContext(iface, BigInt(coprocessorContextId));
  const target = requireCoprocessorConfigAddress(useInternalProxyAddress);
  const hash = await broadcast(hre, target, calldata);
  console.log(
    `Broadcast destroyCoprocessorContext(${coprocessorContextId}) on ${target} (tx: ${hash}). Context is now destroyed.`,
  );
});
