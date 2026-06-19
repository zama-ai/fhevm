import { expect } from 'chai';
import { ethers } from 'ethers';

import {
  ChainConfig,
  HOST_CHAINS,
  createInstance,
  deployContract,
  getProvider,
  getSigners,
} from '../multiChain/multiChainHelper';
import { extractBridgeGuid, relayBridgeMessage, waitForBridgedHandles } from './relay';

// Mock endpoint has no executor, so the test relays itself. With a real endpoint (BRIDGE_REAL_LZ),
// LZ delivers and the test only waits for the HandleBridged event.
const USE_REAL_LZ = (process.env.BRIDGE_REAL_LZ || '').toLowerCase() === 'true';

const BRIDGE_SEND_ABI = [
  'function send(uint32 dstEid, bytes32 dstApp, bytes payload, bytes32[] handleList, uint128 lzComposeGas, bytes options) payable',
];

// Per-chain bridge/endpoint addresses: primary chain uses unindexed vars, others HOST_CHAIN_<i>_*.
function bridgeAddrFor(index: number): string | undefined {
  return index === 0
    ? process.env.CONFIDENTIAL_BRIDGE_CONTRACT_ADDRESS || undefined
    : process.env[`HOST_CHAIN_${index}_CONFIDENTIAL_BRIDGE_CONTRACT_ADDRESS`] || undefined;
}
function endpointAddrFor(index: number): string | undefined {
  return index === 0
    ? process.env.LZ_ENDPOINT_ADDRESS || undefined
    : process.env[`HOST_CHAIN_${index}_LZ_ENDPOINT_ADDRESS`] || undefined;
}

/** Retries publicDecrypt until the destination coprocessor has associated the bridged handle. */
async function pollPublicDecrypt(
  instance: Awaited<ReturnType<typeof createInstance>>,
  handle: string,
  timeoutMs: number,
): Promise<bigint> {
  const deadline = Date.now() + timeoutMs;
  let lastError: unknown;
  while (Date.now() < deadline) {
    try {
      const res = await instance.publicDecrypt([handle]);
      const value = res.clearValues[handle];
      if (value !== undefined) return BigInt(value as string | number | bigint);
    } catch (error) {
      lastError = error;
    }
    await new Promise((resolve) => setTimeout(resolve, 3_000));
  }
  throw new Error(`publicDecrypt did not resolve handle ${handle} within ${timeoutMs}ms: ${lastError}`);
}

describe('Confidential Bridge', function () {
  this.timeout(600_000);

  let host: ChainConfig;
  let chainB: ChainConfig;
  let srcBridge: string;
  let dstBridge: string;
  let srcEndpoint: string;
  let dstEndpoint: string;
  let probeSrc: ethers.Contract;
  let probeDst: ethers.Contract;
  let probeSrcAddress: string;
  let probeDstAddress: string;
  let aliceHost: ReturnType<typeof getSigners>['alice'];
  let aliceB: ReturnType<typeof getSigners>['alice'];
  let instanceHost: Awaited<ReturnType<typeof createInstance>>;
  let instanceB: Awaited<ReturnType<typeof createInstance>>;

  before(async function () {
    if (HOST_CHAINS.length < 2) {
      this.skip();
      return;
    }
    host = HOST_CHAINS[0];
    chainB = HOST_CHAINS[1];

    const sb = bridgeAddrFor(0);
    const db = bridgeAddrFor(1);
    const se = endpointAddrFor(0);
    const de = endpointAddrFor(1);
    if (!sb || !db || !se || !de) {
      // Bridge infra not deployed for this stack (e.g. bridge-deploy step skipped).
      this.skip();
      return;
    }
    srcBridge = sb;
    dstBridge = db;
    srcEndpoint = se;
    dstEndpoint = de;

    aliceHost = getSigners(host).alice;
    aliceB = getSigners(chainB).alice;

    probeSrc = await deployContract('BridgeProbe', aliceHost);
    probeDst = await deployContract('BridgeProbe', aliceB);
    probeSrcAddress = await probeSrc.getAddress();
    probeDstAddress = await probeDst.getAddress();

    instanceHost = await createInstance(host);
    instanceB = await createInstance(chainB);
  });

  it('bridges a handle host->chain-b and publicly decrypts it on the destination', async function () {
    const value = 7n;

    // 1. On host: encrypt an input and mint a handle ACL-allowed to the sender.
    const enc = await instanceHost.encryptUint64({
      value: Number(value),
      contractAddress: probeSrcAddress,
      userAddress: aliceHost.address,
    });
    const mintReceipt = await (
      await probeSrc.connect(aliceHost).getFunction('makeHandle')(enc.handles[0], enc.inputProof, {
        gasLimit: 5_000_000,
      })
    ).wait();
    const minted = mintReceipt!.logs
      .map((log: ethers.Log) => {
        try {
          return probeSrc.interface.parseLog({ topics: [...log.topics], data: log.data });
        } catch {
          return null;
        }
      })
      .find((parsed: ethers.LogDescription | null) => parsed?.name === 'HandleMinted');
    expect(minted, 'HandleMinted emitted').to.not.equal(undefined);
    const srcHandle = minted!.args.handle as string;

    // 2. On host: bridge the handle to chain-b, targeting the destination probe.
    const dstFromBlock = await getProvider(chainB).getBlockNumber();
    const bridge = new ethers.Contract(srcBridge, BRIDGE_SEND_ABI, aliceHost);
    const dstApp = ethers.zeroPadValue(probeDstAddress, 32);
    const sendReceipt = await (
      await bridge.send(chainB.chainId, dstApp, '0x', [srcHandle], 1_000_000n, '0x', {
        value: 0,
        gasLimit: 5_000_000,
      })
    ).wait();

    // 3. Deliver on chain-b. Local mock: relay manually (verify -> lzReceive -> lzCompose).
    //    Real LayerZero: the executor delivers; just wait for the HandleBridged event.
    const dstHandles = USE_REAL_LZ
      ? await waitForBridgedHandles(
          getProvider(chainB),
          dstBridge,
          extractBridgeGuid(sendReceipt, srcEndpoint),
          dstFromBlock,
          180_000,
        )
      : (await relayBridgeMessage(sendReceipt, { srcEndpoint, dstEndpoint, dstBridge, dstSigner: aliceB })).dstHandles;
    expect(dstHandles.length, 'one destination handle').to.equal(1);
    const dstHandle = dstHandles[0];
    expect(dstHandle).to.not.equal(ethers.ZeroHash);
    // The derived handle carries the destination chain id in bytes 22-29.
    expect(dstHandle.slice(46, 62)).to.equal(chainB.chainId.toString(16).padStart(16, '0'));

    // 4. Wait for the chain-b coprocessor to associate the bridged ciphertext, then decrypt.
    const clear = await pollPublicDecrypt(instanceB, dstHandle, 180_000);
    expect(clear).to.equal(value);
  });
});
