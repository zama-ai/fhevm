import { HardhatEthersSigner } from '@nomicfoundation/hardhat-ethers/signers';
import { ethers, upgrades } from 'hardhat';

import type { ConfidentialBridge, ConfidentialOFT } from '../../types';
import { getSigners } from '../signers';

export const SRC_EID = 1;
export const SRC_CHAIN_ID = 3535n;
export const DST_EID = 2;
export const DST_CHAIN_ID = 4242n;

/**
 * Deploys two LayerZero V2 endpoint mocks (one per eid) and one ConfidentialBridge per
 * endpoint, each behind a UUPS proxy. The bridge merges the source-side (`send`) and
 * destination-side (`_lzReceive` + lzCompose) roles into a single deployed contract; in
 * this two-endpoint topology each instance plays one role. Peers are configured
 * bidirectionally and the source-side dstEid → dstChainId map is seeded.
 *
 * No send/receive library is registered — these Hardhat tests never trigger an actual
 * LayerZero send through the endpoint (forge tests via TestHelperOz5 cover end-to-end
 * delivery). For inbound paths, tests impersonate the endpoint and call
 * `lzReceive`/`lzCompose` directly.
 */
export async function deployBridgeFixture() {
  const signers = await getSigners();
  const owner = signers.fred; // because ACL's owner is fred, i.e signers[5] in the default `.env.example`

  const endpointFactory = await ethers.getContractFactory('EndpointV2Mock');
  const srcEndpoint = await endpointFactory.connect(owner).deploy(SRC_EID, owner.address);
  await srcEndpoint.waitForDeployment();
  const dstEndpoint = await endpointFactory.connect(owner).deploy(DST_EID, owner.address);
  await dstEndpoint.waitForDeployment();

  // Seed dstEid → dstChainId on the source-side bridge in `initializeFromEmptyProxy`.
  // The destination-side bridge doesn't send, so its map stays empty.
  const srcBridge = await _deployBridgeProxy(await srcEndpoint.getAddress(), [DST_EID], [DST_CHAIN_ID]);
  const dstBridge = await _deployBridgeProxy(await dstEndpoint.getAddress(), [SRC_EID], [SRC_CHAIN_ID]);

  // Configure peers (bytes32-padded addresses).
  const dstBridgeAsBytes32 = ethers.zeroPadValue(await dstBridge.getAddress(), 32);
  const srcBridgeAsBytes32 = ethers.zeroPadValue(await srcBridge.getAddress(), 32);
  await (await srcBridge.connect(owner).setPeer(DST_EID, dstBridgeAsBytes32)).wait();
  await (await dstBridge.connect(owner).setPeer(SRC_EID, srcBridgeAsBytes32)).wait();

  return {
    owner,
    signers,
    srcEndpoint,
    dstEndpoint,
    srcBridge,
    dstBridge,
  };
}

/**
 * Mirrors the production two-phase UUPS pattern used by the other host contracts:
 *  1. Deploy an `EmptyUUPSProxy` (whose upgrade hook is gated by `onlyACLOwner`).
 *  2. Upgrade it to a fresh `ConfidentialBridge` implementation (the LayerZero
 *     endpoint is baked into the implementation as an immutable, so each bridge needs
 *     its own implementation) and call `initializeFromEmptyProxy` in the same tx,
 *     setting `bridgeOwner` as the bridge's operational owner.
 *
 * The upgrade itself must be authorized by the ACL owner, which in this test env
 * is the wallet keyed by `DEPLOYER_PRIVATE_KEY` — the same one that deployed the ACL
 * empty proxy during `npm run compile` (see `task:deployEmptyUUPSProxies`). The
 * bridge's operational owner (`bridgeOwner`) can be a different account.
 */
async function _deployBridgeProxy(
  lzEndpoint: string,
  dstEids: number[],
  dstChainIds: bigint[],
): Promise<ConfidentialBridge> {
  const aclOwner = new ethers.Wallet(process.env.DEPLOYER_PRIVATE_KEY!).connect(ethers.provider);

  const emptyFactory = await ethers.getContractFactory('EmptyUUPSProxy', aclOwner);
  const emptyProxy = await upgrades.deployProxy(emptyFactory, [], {
    initializer: 'initialize',
    kind: 'uups',
  });
  await emptyProxy.waitForDeployment();

  const bridgeFactory = await ethers.getContractFactory('ConfidentialBridge', aclOwner);
  const bridge = (await upgrades.upgradeProxy(await emptyProxy.getAddress(), bridgeFactory, {
    constructorArgs: [lzEndpoint],
    // - constructor / state-variable-immutable: LayerZero's `OAppCoreUpgradeable`
    //   stores the endpoint as an immutable in the implementation's constructor.
    // - missing-initializer-call: `__OApp(Sender|Receiver)_init_unchained()` are no-ops
    //   and we call them explicitly; OZ's static validator doesn't recognize the
    //   `_unchained` variants as satisfying the `_init` requirement.
    unsafeAllow: ['constructor', 'state-variable-immutable', 'missing-initializer-call'],
    call: { fn: 'initializeFromEmptyProxy', args: [dstEids, dstChainIds] },
  })) as unknown as ConfidentialBridge;
  await bridge.waitForDeployment();
  return bridge;
}

export async function deployConfidentialOFTFixture() {
  const base = await deployBridgeFixture();
  const oftFactory = await ethers.getContractFactory('ConfidentialOFT');
  // Deploy the OFT against the destination-side bridge — that is the contract whose
  // address is checked in `onReceive` and which dispatches outbound sends from here.
  const oft = (await oftFactory
    .connect(base.owner)
    .deploy(await base.dstBridge.getAddress(), base.owner.address)) as unknown as ConfidentialOFT;
  await oft.waitForDeployment();
  return { ...base, oft };
}
