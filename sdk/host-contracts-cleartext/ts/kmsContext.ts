import { abi as aclOwnerAbi } from './artifacts/ACLOwner.js';
import { abi as protocolConfigAbi } from './artifacts/ProtocolConfig.js';
import type { AbstractEthereumSigner, AbstractEthereumUtils } from './types/public.js';

////////////////////////////////////////////////////////////////////////////////
//
// NOTE (v14 protocol delta): v13's one-call `defineNewKmsContext` helper has no v14 equivalent yet.
// v14 turned context rotation into a multi-party ceremony: `defineNewKmsContextAndEpoch` (ACL owner)
// leaves the context Pending, then `confirmKmsContextCreation` needs a split quorum of previous- and
// new-committee KMS tx senders, then `confirmEpochActivation` needs ALL new-context tx senders to
// attest the epoch. A helper for that needs a multi-signer API (the tx-sender keys exist in
// `./signers/defaultKmsTxSenderSigners.js`) and is deliberately left as a follow-up.
//
////////////////////////////////////////////////////////////////////////////////

/**
 * Destroy a past KMS context.
 *
 * Marks `kmsContextId` as destroyed via `ProtocolConfig.destroyKmsContext(...)`, routed through the
 * standing `ACLOwner` (authorization model: `admin` owns the `ACLOwner`, which is the ACL owner). The
 * CURRENT context cannot be destroyed, and an unknown or already-destroyed id is rejected — both
 * revert on-chain and the revert bubbles up through `ACLOwner.execute`.
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
