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
import {
  extractBridgeGuid,
  forgeDelivery,
  relayBridgeMessage,
  relayCompose,
  sendWithNonceRetry,
  waitForBridgedHandles,
} from './relay';

// Mock endpoint has no executor, so the test relays itself; with a real endpoint (BRIDGE_REAL_LZ), LZ delivers and the test only waits for the HandleBridged event.
const USE_REAL_LZ = (process.env.BRIDGE_REAL_LZ || '').toLowerCase() === 'true';

const BRIDGE_SEND_ABI = [
  'function send(uint32 dstEid, bytes32 dstApp, bytes payload, bytes32[] handleList, uint64 lzComposeGas) payable',
];
const LZ_COMPOSE_GAS = 1_000_000n;
const DECRYPT_TIMEOUT_MS = 180_000;

// Per-chain bridge/endpoint addresses: primary chain uses unindexed vars, others HOST_CHAIN_<i>_*.
const bridgeAddrFor = (i: number) =>
  (i === 0 ? process.env.CONFIDENTIAL_BRIDGE_CONTRACT_ADDRESS : process.env[`HOST_CHAIN_${i}_CONFIDENTIAL_BRIDGE_CONTRACT_ADDRESS`]) || undefined;
const endpointAddrFor = (i: number) =>
  (i === 0 ? process.env.LZ_ENDPOINT_ADDRESS : process.env[`HOST_CHAIN_${i}_LZ_ENDPOINT_ADDRESS`]) || undefined;

/** Everything needed to act as a source or destination of a bridge transfer on one chain. */
interface BridgeEnd {
  cfg: ChainConfig;
  bridge: string;
  endpoint: string;
  app: ethers.Contract;
  appAddr: string;
  alice: ReturnType<typeof getSigners>['alice'];
  instance: Awaited<ReturnType<typeof createInstance>>;
}

/** Retries an async decrypt until the destination coprocessor has associated the bridged handle. */
async function pollDecrypt(fn: () => Promise<bigint | undefined>): Promise<bigint> {
  const deadline = Date.now() + DECRYPT_TIMEOUT_MS;
  let lastError: unknown;
  while (Date.now() < deadline) {
    try {
      const value = await fn();
      if (value !== undefined) return value;
    } catch (error) {
      lastError = error;
    }
    await new Promise((resolve) => setTimeout(resolve, 3_000));
  }
  throw new Error(`decrypt did not resolve within ${DECRYPT_TIMEOUT_MS}ms: ${lastError}`);
}

const publicDecrypt = (end: BridgeEnd, handle: string) =>
  pollDecrypt(async () => {
    const value = (await end.instance.publicDecrypt([handle])).clearValues[handle as `0x${string}`];
    return value === undefined ? undefined : BigInt(value as string | number | bigint);
  });

const userDecrypt = (end: BridgeEnd, handle: string) =>
  pollDecrypt(async () =>
    BigInt(
      await end.instance.userDecryptSingleHandle({ handle, contractAddress: end.appAddr, signer: end.alice }),
    ),
  );

/** True if publicDecrypt rejects the handle now (no public-decrypt ACL yet, or no ciphertext). */
async function notPubliclyDecryptable(end: BridgeEnd, handle: string): Promise<boolean> {
  try {
    await end.instance.publicDecrypt([handle]);
    return false;
  } catch {
    return true;
  }
}

describe('Confidential Bridge', function () {
  this.timeout(600_000);

  let host: BridgeEnd;
  let chainB: BridgeEnd;

  /** Reads the `HandleMinted` handle out of a `makeHandle`/`makeComputedHandle`/`addToHandle` receipt. */
  function mintedHandle(end: BridgeEnd, receipt: ethers.TransactionReceipt): string {
    const minted = receipt.logs
      .map((log: ethers.Log) => {
        try {
          return end.app.interface.parseLog({ topics: [...log.topics], data: log.data });
        } catch {
          return null;
        }
      })
      .find((parsed: ethers.LogDescription | null) => parsed?.name === 'HandleMinted');
    if (!minted) throw new Error('HandleMinted not emitted');
    return minted.args.handle as string;
  }

  /** Mints a handle ACL-allowed to the source `alice`; `addend` switches to a computed handle. */
  async function mint(end: BridgeEnd, value: number, addend?: number): Promise<string> {
    const enc = await end.instance.encryptUint64({
      value,
      contractAddress: end.appAddr,
      userAddress: end.alice.address,
    });
    const fn = addend === undefined ? 'makeHandle' : 'makeComputedHandle';
    const args = addend === undefined ? [enc.handles[0], enc.inputProof] : [enc.handles[0], enc.inputProof, addend];
    const receipt = await sendWithNonceRetry(end.alice, () =>
      end.app.connect(end.alice).getFunction(fn)(...args, { gasLimit: 5_000_000 }),
    );
    return mintedHandle(end, receipt);
  }

  /** Computes `existing + addend` on an already-registered handle (e.g. one received via the
   *  bridge), returning a new handle ACL-allowed to `alice` so it can be re-bridged. */
  async function computeOn(end: BridgeEnd, existing: string, addend: number): Promise<string> {
    const receipt = await sendWithNonceRetry(end.alice, () =>
      end.app.connect(end.alice).getFunction('addToHandle')(existing, addend, { gasLimit: 5_000_000 }),
    );
    return mintedHandle(end, receipt);
  }

  /** Bridges `handles` from `src` to `dst` (targeting dst's app) and delivers them. */
  async function bridge(src: BridgeEnd, dst: BridgeEnd, handles: string[], payload = '0x', skipCompose = false) {
    const ctx = { srcEndpoint: src.endpoint, dstEndpoint: dst.endpoint, dstBridge: dst.bridge, dstSigner: dst.alice };
    const fromBlock = await getProvider(dst.cfg).getBlockNumber();
    const dstApp = ethers.zeroPadValue(dst.appAddr, 32);
    const bridgeContract = new ethers.Contract(src.bridge, BRIDGE_SEND_ABI, src.alice);
    const sendReceipt = await sendWithNonceRetry(src.alice, () =>
      bridgeContract.send(dst.cfg.chainId, dstApp, payload, handles, LZ_COMPOSE_GAS, {
        value: 0,
        gasLimit: 5_000_000,
      }),
    );

    if (USE_REAL_LZ) {
      const dstHandles = await waitForBridgedHandles(
        getProvider(dst.cfg),
        dst.bridge,
        extractBridgeGuid(sendReceipt, src.endpoint),
        fromBlock,
        DECRYPT_TIMEOUT_MS,
      );
      return { dstHandles, ctx, compose: undefined };
    }
    const { dstHandles, compose } = await relayBridgeMessage(sendReceipt, ctx, { skipCompose });
    return { dstHandles, ctx, compose };
  }

  before(async function () {
    if (HOST_CHAINS.length < 2) {
      this.skip();
      return;
    }
    const addrs = [0, 1].map((i) => ({ bridge: bridgeAddrFor(i), endpoint: endpointAddrFor(i) }));
    if (addrs.some((a) => !a.bridge || !a.endpoint)) {
      // Bridge infra not deployed for this stack (e.g. bridge-deploy step skipped).
      this.skip();
      return;
    }
    const build = async (i: number): Promise<BridgeEnd> => {
      const cfg = HOST_CHAINS[i];
      const alice = getSigners(cfg).alice;
      const app = await deployContract('BridgeApp', alice);
      return {
        cfg,
        bridge: addrs[i].bridge!,
        endpoint: addrs[i].endpoint!,
        app,
        appAddr: await app.getAddress(),
        alice,
        instance: await createInstance(cfg),
      };
    };
    host = await build(0);
    chainB = await build(1);
  });

  it('bridges a handle host->chain-b and publicly decrypts it on the destination', async function () {
    const srcHandle = await mint(host, 7);
    const { dstHandles } = await bridge(host, chainB, [srcHandle]);
    expect(dstHandles.length, 'one destination handle').to.equal(1);
    expect(dstHandles[0]).to.not.equal(ethers.ZeroHash);
    // The derived handle carries the destination chain id in bytes 22-29.
    expect(dstHandles[0].slice(46, 62)).to.equal(chainB.cfg.chainId.toString(16).padStart(16, '0'));
    expect(await publicDecrypt(chainB, dstHandles[0])).to.equal(7n);
  });

  it('bridges a handle host->chain-b and lets a user decrypt it on the destination', async function () {
    const srcHandle = await mint(host, 11);
    const payload = ethers.AbiCoder.defaultAbiCoder().encode(['address'], [chainB.alice.address]);
    const { dstHandles } = await bridge(host, chainB, [srcHandle], payload);
    expect(await userDecrypt(chainB, dstHandles[0])).to.equal(11n);
  });

  it('bridges a computed (non-input) handle and decrypts it on the destination', async function () {
    const srcHandle = await mint(host, 7, 1); // 7 + 1
    const { dstHandles } = await bridge(host, chainB, [srcHandle]);
    expect(await publicDecrypt(chainB, dstHandles[0])).to.equal(8n);
  });

  it('bridges multiple handles in a single send', async function () {
    const handles = [await mint(host, 3), await mint(host, 5)];
    const { dstHandles } = await bridge(host, chainB, handles);
    expect(dstHandles.length, 'two destination handles').to.equal(2);
    // dstHandles follow the source handleList order.
    expect(await publicDecrypt(chainB, dstHandles[0])).to.equal(3n);
    expect(await publicDecrypt(chainB, dstHandles[1])).to.equal(5n);
  });

  it('bridges a handle in the reverse direction (chain-b->host)', async function () {
    const srcHandle = await mint(chainB, 9);
    const { dstHandles } = await bridge(chainB, host, [srcHandle]);
    expect(dstHandles[0].slice(46, 62)).to.equal(host.cfg.chainId.toString(16).padStart(16, '0'));
    expect(await publicDecrypt(host, dstHandles[0])).to.equal(9n);
  });

  it('round-trips a handle host->chain-b->host (re-bridges a bridge-derived handle)', async function () {
    const value = 17;
    const srcHandle = await mint(host, value);

    // Leg 1: host -> chain-b. Use a user payload (chain-b's alice) so the derived handle is
    // ACL-allowed to her — makePubliclyDecryptable alone would not let her re-bridge it.
    const toB = ethers.AbiCoder.defaultAbiCoder().encode(['address'], [chainB.alice.address]);
    const { dstHandles: onB } = await bridge(host, chainB, [srcHandle], toB);
    expect(onB[0].slice(46, 62), 'leg-1 handle carries chain-b id').to.equal(
      chainB.cfg.chainId.toString(16).padStart(16, '0'),
    );

    // Leg 2: chain-b -> host, re-bridging the *bridge-derived* handle (its srcHandle already
    // carries the 0xff computation marker + chain-b metadata). Empty payload => publicly decryptable.
    const { dstHandles: back } = await bridge(chainB, host, [onB[0]]);
    expect(back[0].slice(46, 62), 'leg-2 handle carries host id').to.equal(
      host.cfg.chainId.toString(16).padStart(16, '0'),
    );
    expect(back[0]).to.not.equal(onB[0]); // a fresh handle is derived each hop
    expect(await publicDecrypt(host, back[0]), 'value survives the round trip').to.equal(BigInt(value));
  });

  it('computes on a bridged handle on chain-b, then bridges the result back to host', async function () {
    const value = 20;
    const addend = 5;
    const srcHandle = await mint(host, value);

    // Leg 1: host -> chain-b with a user payload, so chain-b's app holds ACL allowance on the
    // received handle (FHE.allowThis) and can compute on it.
    const toB = ethers.AbiCoder.defaultAbiCoder().encode(['address'], [chainB.alice.address]);
    const { dstHandles: onB } = await bridge(host, chainB, [srcHandle], toB);

    // Compute on the received ciphertext on chain-b (value + addend) -> a new handle allowed to alice.
    // This proves the bridged ciphertext is functionally usable in an FHE op on the destination,
    // not merely decryptable.
    const computed = await computeOn(chainB, onB[0], addend);

    // Leg 2: bridge the computed result back to host; empty payload => publicly decryptable.
    const { dstHandles: back } = await bridge(chainB, host, [computed]);
    expect(await publicDecrypt(host, back[0]), 'computed value survives the round trip').to.equal(
      BigInt(value + addend),
    );
  });

  it('associates on lzReceive but defers the dapp callback until lzCompose runs', async function () {
    if (USE_REAL_LZ) {
      this.skip(); // real LZ always delivers the compose leg; the deferred path is mock-only.
      return;
    }
    const srcHandle = await mint(host, 13);
    // Deliver lzReceive only: the handle associates, but onConfidentialBridgeReceived (makePubliclyDecryptable) hasn't run.
    const { dstHandles, ctx, compose } = await bridge(host, chainB, [srcHandle], '0x', true);
    expect(await notPubliclyDecryptable(chainB, dstHandles[0]), 'not decryptable before the compose leg').to.equal(true);

    // Now run the compose leg -> onConfidentialBridgeReceived -> makePubliclyDecryptable -> decryptable.
    await relayCompose(ctx, compose!);
    expect(await publicDecrypt(chainB, dstHandles[0])).to.equal(13n);
  });

  it('materializes an unassociated bridged handle via grantFallbackPlaintext (governance)', async function () {
    if (USE_REAL_LZ) {
      this.skip(); // forging a delivery bypasses bridge.send; only meaningful with the local mock.
      return;
    }
    const value = 42n;
    const FAKE_EID = 424242; // throwaway inbound channel, isolated from the real host<->chain-b nonces
    const owner = new ethers.Wallet(process.env.DEPLOYER_PRIVATE_KEY!, getProvider(chainB.cfg));

    // Register a throwaway receive path on chain-b (a lib + peer for FAKE_EID) so the forged delivery
    // advances that channel's nonce instead of the real one — keeps the suite re-runnable on a
    // persistent stack. Both setters are owner-gated; the e2e env carries the deployer (ACL owner) key.
    const endpoint = new ethers.Contract(
      chainB.endpoint,
      ['function defaultReceiveLibrary(uint32) view returns (address)', 'function setDefaultReceiveLibrary(uint32,address,uint256)'],
      owner,
    );
    if ((await endpoint.defaultReceiveLibrary(FAKE_EID)) === ethers.ZeroAddress) {
      const lib = await endpoint.defaultReceiveLibrary(Number(host.cfg.chainId));
      await (await endpoint.setDefaultReceiveLibrary(FAKE_EID, lib, 0)).wait();
    }
    const fakeSrcBridge = host.bridge; // arbitrary; just must match the registered peer
    const bridgeOwner = new ethers.Contract(chainB.bridge, ['function setPeer(uint32,bytes32)'], owner);
    await (await bridgeOwner.setPeer(FAKE_EID, ethers.zeroPadValue(fakeSrcBridge, 32))).wait();

    // Mint a real euint64 handle on host but DO NOT bridge.send it, so there is no source BridgeHandle.
    // Forging the delivery leaves the handle unassociated (no ciphertext) while onConfidentialBridgeReceived still grants
    // on-chain public ACL — the exact state the fallback exists to rescue.
    const srcHandle = await mint(host, 0); // value irrelevant: it's never bridged/associated
    const ctx = { srcEndpoint: host.endpoint, dstEndpoint: chainB.endpoint, dstBridge: chainB.bridge, dstSigner: chainB.alice };
    const [dstHandle] = await forgeDelivery(ctx, {
      srcEid: FAKE_EID,
      dstEid: Number(chainB.cfg.chainId),
      srcBridge: fakeSrcBridge,
      srcHandle,
      dstApp: chainB.appAddr,
      payload: '0x',
    });

    // No matching source BridgeHandle -> never associated -> no ciphertext -> not decryptable yet.
    expect(await notPubliclyDecryptable(chainB, dstHandle), 'no ciphertext before the fallback grant').to.equal(true);

    // Governance (the ACL owner) grants the plaintext fallback; the coprocessor materializes it.
    const bridge = new ethers.Contract(chainB.bridge, ['function grantFallbackPlaintext(bytes32 dstHandle, uint256 plaintext)'], owner);
    await (await bridge.grantFallbackPlaintext(dstHandle, value)).wait();

    expect(await publicDecrypt(chainB, dstHandle)).to.equal(value);
  });
});
