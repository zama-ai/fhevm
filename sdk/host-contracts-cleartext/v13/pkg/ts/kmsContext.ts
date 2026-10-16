import { abi as aclOwnerAbi } from './artifacts/ACLOwner.js';
import { abi as protocolConfigAbi } from './artifacts/ProtocolConfig.js';
import { generateFromExistingDefaultKmsNodes, nextDefaultKmsSignerWindow } from './constants.js';
import type {
  AbstractEthereumProvider,
  AbstractEthereumSigner,
  AbstractEthereumUtils,
  KmsThresholds,
} from './types/public.js';

////////////////////////////////////////////////////////////////////////////////

/**
 * Rotate the live `ProtocolConfig` KMS context to the next window of default signers.
 *
 * Reads the current KMS signer set (which must be a consecutive window of the default 20-signer pool),
 * computes the next window of the same length via {@link nextDefaultKmsSignerWindow}, rebuilds the full
 * per-node details from the defaults, and calls `ProtocolConfig.defineNewKmsContext(...)`. The four
 * thresholds are preserved from the current context unless `thresholds` is provided.
 *
 * @dev `defineNewKmsContext` is `onlyACLOwner`, so the call is routed through the standing `ACLOwner`
 *      (the ACL owner): `admin` — the `ACLOwner`'s own owner — sends `ACLOwner.execute(protocolConfig,
 *      calldata)`, which forwards it with `msg.sender == ACL.owner()`. This is the default post-`deploy`
 *      topology (ACL owned by `ACLOwner`, `ACLOwner` owned by `admin`).
 */
export async function defineNewKmsContext(parameters: {
  readonly ethProvider: AbstractEthereumProvider;
  readonly ethUtils: AbstractEthereumUtils;
  readonly admin: AbstractEthereumSigner;
  readonly aclOwnerAddress: string;
  readonly protocolConfigAddress: string;
  readonly thresholds?: KmsThresholds | undefined;
}): Promise<{ readonly signers: readonly string[] }> {
  const read = (functionName: string): Promise<unknown> =>
    parameters.ethProvider.readContract({
      address: parameters.protocolConfigAddress,
      abi: protocolConfigAbi,
      functionName,
    });

  const currentSigners = (await read('getKmsSigners')) as readonly string[];
  const newSigners = nextDefaultKmsSignerWindow(currentSigners);
  const kmsNodes = generateFromExistingDefaultKmsNodes(newSigners);

  const thresholds: KmsThresholds = parameters.thresholds ?? {
    publicDecryption: (await read('getPublicDecryptionThreshold')) as bigint,
    userDecryption: (await read('getUserDecryptionThreshold')) as bigint,
    kmsGen: (await read('getKmsGenThreshold')) as bigint,
    mpc: (await read('getMpcThreshold')) as bigint,
  };

  // Encode ProtocolConfig.defineNewKmsContext(...) and forward it through ACLOwner.execute, so the
  // on-chain msg.sender is the ACL owner (this contract) and the onlyACLOwner gate passes.
  const callData = await parameters.ethUtils.encodeCall({
    abi: protocolConfigAbi,
    functionName: 'defineNewKmsContext',
    args: [kmsNodes, thresholds],
  });

  await parameters.admin.writeContract({
    address: parameters.aclOwnerAddress,
    abi: aclOwnerAbi,
    functionName: 'execute',
    args: [parameters.protocolConfigAddress, callData],
  });

  return { signers: newSigners };
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Destroy a past KMS context.
 *
 * Marks `kmsContextId` as destroyed via `ProtocolConfig.destroyKmsContext(...)`, routed through the
 * standing `ACLOwner` (same authorization model as {@link defineNewKmsContext}: `admin` owns the
 * `ACLOwner`, which is the ACL owner). The CURRENT context cannot be destroyed, and an unknown or
 * already-destroyed id is rejected — both revert on-chain and the revert bubbles up through
 * `ACLOwner.execute`.
 */
export async function destroyKmsContext(parameters: {
  readonly ethUtils: AbstractEthereumUtils;
  readonly admin: AbstractEthereumSigner;
  readonly aclOwnerAddress: string;
  readonly protocolConfigAddress: string;
  readonly kmsContextId: bigint;
}): Promise<void> {
  const callData = await parameters.ethUtils.encodeCall({
    abi: protocolConfigAbi,
    functionName: 'destroyKmsContext',
    args: [parameters.kmsContextId],
  });

  await parameters.admin.writeContract({
    address: parameters.aclOwnerAddress,
    abi: aclOwnerAbi,
    functionName: 'execute',
    args: [parameters.protocolConfigAddress, callData],
  });
}
