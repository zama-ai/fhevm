import { abi as aclOwnerAbi, template as aclOwnerTemplate } from './artifacts/ACLOwner.js';
import { abi as aclAbi } from './artifacts/ACL.js';
import { abi as pauserSetAbi } from './artifacts/PauserSet.js';
import type { AbstractEthereumSigner, AbstractEthereumUtils, DeployReturnType } from './types/public.js';
import type { HexString, DeployedImplementation } from './types/private.js';

/**
 * Deploys the standing `ACLOwner` admin. One-time; afterwards ACL ownership is handed to it via
 * `setupACLOwner` (or a manual transfer + `acceptACLOwnership`).
 */
export async function deployACLOwner(parameters: {
  readonly deployer: AbstractEthereumSigner;
  readonly initialOwner: string;
  readonly aclAddress: string;
}): Promise<DeployReturnType> {
  return await parameters.deployer.deploy({
    abi: aclOwnerAbi,
    bytecode: aclOwnerTemplate.bytecode,
    args: [parameters.initialOwner, parameters.aclAddress],
  });
}

/**
 * One-time setup: deploy `ACLOwner` and hand it ACL ownership.
 *
 * `ACLOwner`'s owner is set to `admin`'s address. `currentAclOwner` (the current ACL owner) transfers
 * ownership to the new `ACLOwner`, then `admin` completes the two-step transfer via
 * `acceptACLOwnership`. The three signers are often the same account in a single-operator setup.
 *
 * @dev Sends the transfer and accept as sequential transactions; correct on ordered/auto-mining
 *      networks. If `currentAclOwner` and `admin` are different accounts on a live network, ensure the
 *      transfer is confirmed before the accept.
 */
export async function setupACLOwner(parameters: {
  readonly deployer: AbstractEthereumSigner;
  readonly currentAclOwner: AbstractEthereumSigner;
  readonly admin: AbstractEthereumSigner;
  readonly aclAddress: string;
  readonly pauserSetAddress: string;
}): Promise<{ readonly aclOwnerAddress: string }> {
  const initialOwner = await parameters.admin.getAddress();

  const { contractAddress: aclOwnerAddress } = await deployACLOwner({
    deployer: parameters.deployer,
    initialOwner,
    aclAddress: parameters.aclAddress,
  });

  // Register ACLOwner as a pauser so its `pause()` (which calls ACL.pause() as msg.sender) is
  // authorized. `PauserSet.addPauser` is `onlyACLOwner`, so this must happen BEFORE ownership is
  // transferred — while `currentAclOwner` still holds ACL ownership.
  await parameters.currentAclOwner.writeContract({
    address: parameters.pauserSetAddress,
    abi: pauserSetAbi,
    functionName: 'addPauser',
    args: [aclOwnerAddress],
  });

  // Hand ACL ownership to ACLOwner (two-step): current owner transfers, admin accepts via ACLOwner.
  await parameters.currentAclOwner.writeContract({
    address: parameters.aclAddress,
    abi: aclAbi,
    functionName: 'transferOwnership',
    args: [aclOwnerAddress],
  });
  await parameters.admin.writeContract({
    address: aclOwnerAddress,
    abi: aclOwnerAbi,
    functionName: 'acceptACLOwnership',
    args: [],
  });

  return { aclOwnerAddress };
}

/**
 * Emergency-stop the ACL via the standing `ACLOwner`. Sent by `admin` (the `ACLOwner`'s owner);
 * `ACLOwner.pause()` forwards to `ACL.pause()`, which is authorized because `ACLOwner` is a
 * registered pauser (see `setupACLOwner`).
 */
export async function pauseACL(parameters: {
  readonly admin: AbstractEthereumSigner;
  readonly aclOwnerAddress: string;
}): Promise<void> {
  await parameters.admin.writeContract({
    address: parameters.aclOwnerAddress,
    abi: aclOwnerAbi,
    functionName: 'pause',
    args: [],
  });
}

/**
 * Lift a pause on the ACL via the standing `ACLOwner`. Sent by `admin`; `ACLOwner.unpause()` forwards
 * to `ACL.unpause()`, which ACL gates on its owner — the `ACLOwner` itself.
 */
export async function unpauseACL(parameters: {
  readonly admin: AbstractEthereumSigner;
  readonly aclOwnerAddress: string;
}): Promise<void> {
  await parameters.admin.writeContract({
    address: parameters.aclOwnerAddress,
    abi: aclOwnerAbi,
    functionName: 'unpause',
    args: [],
  });
}

/** Maps a Phase 1 plan to the `ACLOwner.Op[]` argument for `upgrade(ops)`. */
export function toACLOwnerOps(
  implementations: readonly DeployedImplementation[],
): ReadonlyArray<{ readonly proxy: string; readonly implementation: string; readonly initData: HexString }> {
  return implementations.map((proxy) => ({
    proxy: proxy.proxyAddress,
    implementation: proxy.implementationAddress,
    initData: proxy.initData,
  }));
}

/**
 * Encodes `ACLOwner.upgrade(ops)` calldata from a Phase 1 plan — for the owner to send directly, or
 * to hand to a multisig/timelock owner to execute.
 */
export async function encodeACLOwnerUpgrade(parameters: {
  readonly ethUtils: AbstractEthereumUtils;
  readonly implementations: readonly DeployedImplementation[];
}): Promise<HexString> {
  return await parameters.ethUtils.encodeCall({
    abi: aclOwnerAbi,
    functionName: 'upgrade',
    args: [toACLOwnerOps(parameters.implementations)],
  });
}
