import { ethers } from 'hardhat';

import type { ConfidentialBridge, ConfidentialOFT } from '../../types';
import { getSigners } from '../signers';

export const SRC_EID = 1;
export const DST_EID = 2;
export const DST_CHAIN_ID = 4242n;

/**
 * Deploys two LayerZero V2 endpoint mocks (one per eid) and one ConfidentialBridge per
 * endpoint. The bridge merges the source-side (`send`) and destination-side
 * (`_lzReceive` + lzCompose) roles into a single deployed contract; in this two-endpoint
 * topology each instance plays one role. Peers are configured bidirectionally and the
 * source-side dstEid → dstChainId map is seeded.
 *
 * No send/receive library is registered — these Hardhat tests never trigger an actual
 * LayerZero send through the endpoint (forge tests via TestHelperOz5 cover end-to-end
 * delivery). For inbound paths, tests impersonate the endpoint and call
 * `lzReceive`/`lzCompose` directly.
 */
export async function deployBridgeFixture() {
  const signers = await getSigners();
  const owner = signers.alice;

  const endpointFactory = await ethers.getContractFactory('EndpointV2Mock');
  const srcEndpoint = await endpointFactory.connect(owner).deploy(SRC_EID, owner.address);
  await srcEndpoint.waitForDeployment();
  const dstEndpoint = await endpointFactory.connect(owner).deploy(DST_EID, owner.address);
  await dstEndpoint.waitForDeployment();

  const bridgeFactory = await ethers.getContractFactory('ConfidentialBridge');
  const srcBridge = (await bridgeFactory
    .connect(owner)
    .deploy(await srcEndpoint.getAddress(), owner.address)) as unknown as ConfidentialBridge;
  await srcBridge.waitForDeployment();

  const dstBridge = (await bridgeFactory
    .connect(owner)
    .deploy(await dstEndpoint.getAddress(), owner.address)) as unknown as ConfidentialBridge;
  await dstBridge.waitForDeployment();

  // Configure peers (bytes32-padded addresses).
  const dstBridgeAsBytes32 = ethers.zeroPadValue(await dstBridge.getAddress(), 32);
  const srcBridgeAsBytes32 = ethers.zeroPadValue(await srcBridge.getAddress(), 32);
  await (await srcBridge.connect(owner).setPeer(DST_EID, dstBridgeAsBytes32)).wait();
  await (await dstBridge.connect(owner).setPeer(SRC_EID, srcBridgeAsBytes32)).wait();

  // Seed dstEid → dstChainId on the source-side bridge.
  await (await srcBridge.connect(owner).setDstChainId(DST_EID, DST_CHAIN_ID)).wait();

  return {
    owner,
    signers,
    srcEndpoint,
    dstEndpoint,
    srcBridge,
    dstBridge,
  };
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
