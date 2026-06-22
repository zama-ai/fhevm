import { expect } from 'chai';
import { ethers } from 'hardhat';

/**
 * Tests for the cross-chain bridging helpers in `fhevm/solidity`. They run against mock
 * contracts (a fake ACL and a fake bridge), so no real FHEVM stack, coprocessor, or LayerZero
 * network is needed.
 *
 * Covered:
 *   - `FHE.bridge` / `FHE.quoteBridge`: look the bridge up from the ACL and forward every
 *     argument plus `msg.value` unchanged — every encrypted type, single handles and lists, and
 *     the low-level escape-hatch overload.
 *   - `ConfidentialOAppSender`: resolves the destination peer from the shared registry and
 *     bridges to it.
 *   - `ConfidentialOAppReceiver`: only accepts calls from the trusted bridge and a trusted peer
 *     (a check apps can override), and hands the app the raw handles.
 *   - A full send → receive round trip across two OApp instances.
 */
describe('Confidential bridge helpers', function () {
  const DST_EID = 30101;
  const SRC_EID = 30184;
  const h = (n: string) => ethers.zeroPadValue(n, 32);
  const handleA = h('0x01');
  const handleB = h('0x02');
  const handleC = h('0x03');
  const padded = (addr: string) => ethers.zeroPadValue(addr.toLowerCase(), 32);
  const randAddr = () => ethers.Wallet.createRandom().address;

  async function deploySendFixture() {
    const [deployer, app, other] = await ethers.getSigners();
    const acl = await (await ethers.getContractFactory('MockACL')).deploy();
    const bridge = await (await ethers.getContractFactory('MockConfidentialBridge')).deploy();
    await acl.setConfidentialBridgeAddress(await bridge.getAddress());
    const harness = await (await ethers.getContractFactory('BridgeLibHarness')).deploy(await acl.getAddress());
    return { deployer, app, other, acl, bridge, harness };
  }

  describe('send side — typed single-handle overloads (multi-type)', function () {
    // One entrypoint per encrypted type; all funnel through the same bytes32 pass-through.
    const cases: [string, string][] = [
      ['ebool', 'bridgeBool'],
      ['euint8', 'bridge8'],
      ['euint16', 'bridge16'],
      ['euint32', 'bridge32'],
      ['euint64', 'bridge64'],
      ['euint128', 'bridge128'],
      ['euint256', 'bridge256'],
      ['eaddress', 'bridgeAddr'],
    ];

    for (const [typeName, fn] of cases) {
      it(`${typeName}: resolves the bridge from the ACL and forwards every argument + msg.value`, async function () {
        const { bridge, harness } = await deploySendFixture();
        const dstApp = randAddr();
        const payload = ethers.toUtf8Bytes(`payload-${typeName}`);
        const gas = 123_456n;
        const fee = ethers.parseEther('0.01');

        await (harness as any)[fn](DST_EID, dstApp, payload, handleA, gas, { value: fee });

        expect(await bridge.sendCalled()).to.equal(true);
        const sent = await bridge.lastSend();
        expect(sent.dstEid).to.equal(DST_EID);
        expect(sent.dstApp).to.equal(padded(dstApp));
        expect(sent.payload).to.equal(ethers.hexlify(payload));
        expect(sent.handleList).to.deep.equal([handleA]);
        expect(sent.lzComposeGas).to.equal(gas);
        expect(sent.options).to.equal('0x'); // single-handle overloads derive defaults from gas
        expect(sent.value).to.equal(fee);
        expect(sent.caller).to.equal(await harness.getAddress());
      });
    }

    it('forwards a zero native fee unchanged', async function () {
      const { bridge, harness } = await deploySendFixture();
      await harness.bridge64(DST_EID, randAddr(), '0x', handleA, 0, { value: 0 });
      expect((await bridge.lastSend()).value).to.equal(0n);
    });
  });

  describe('send side — typed array overloads (multi-handle)', function () {
    it('euint64[]: sends one message carrying the whole list, empty options', async function () {
      const { bridge, harness } = await deploySendFixture();
      const dstApp = randAddr();
      const fee = ethers.parseEther('0.03');

      await harness.bridgeArray64(DST_EID, dstApp, '0x', [handleA, handleB, handleC], 50_000, { value: fee });

      const sent = await bridge.lastSend();
      expect(sent.dstApp).to.equal(padded(dstApp));
      expect(sent.handleList).to.deep.equal([handleA, handleB, handleC]);
      expect(sent.options).to.equal('0x');
      expect(sent.value).to.equal(fee);
    });

    it('euint256[]: array generation is uniform across types', async function () {
      const { bridge, harness } = await deploySendFixture();
      await harness.bridgeArray256(DST_EID, randAddr(), '0x', [handleA, handleB], 1, { value: 0 });
      expect((await bridge.lastSend()).handleList).to.deep.equal([handleA, handleB]);
    });

    it('a single-element typed array behaves like the single-handle overload', async function () {
      const { bridge, harness } = await deploySendFixture();
      await harness.bridgeArray64(DST_EID, randAddr(), '0x', [handleA], 1, { value: 0 });
      expect((await bridge.lastSend()).handleList).to.deep.equal([handleA]);
    });

    it('an empty typed array forwards an empty handle list', async function () {
      const { bridge, harness } = await deploySendFixture();
      await harness.bridgeArray64(DST_EID, randAddr(), '0x', [], 1, { value: 0 });
      expect((await bridge.lastSend()).handleList).to.deep.equal([]);
    });
  });

  describe('send side — low-level bytes32[] escape hatch', function () {
    it('forwards an explicit handle list, raw bytes32 dstApp, and custom options verbatim', async function () {
      const { bridge, harness } = await deploySendFixture();
      const dstApp = padded(randAddr());
      const payload = ethers.toUtf8Bytes('multi');
      const options = '0xdeadbeef';

      await harness.bridgeList(DST_EID, dstApp, payload, [handleA, handleB], 0, options, { value: 0 });

      const sent = await bridge.lastSend();
      expect(sent.dstApp).to.equal(dstApp);
      expect(sent.handleList).to.deep.equal([handleA, handleB]);
      expect(sent.options).to.equal(options);
    });
  });

  describe('send side — receipt + failure + quotes', function () {
    it('returns the bridge receipt to the caller', async function () {
      const { harness } = await deploySendFixture();
      const fee = ethers.parseEther('0.02');
      const receipt = await harness.bridge64.staticCall(DST_EID, randAddr(), '0x', handleA, 0, { value: fee });
      expect(receipt.fee.nativeFee).to.equal(fee);
      expect(receipt.nonce).to.equal(1n);
    });

    it('reverts BridgeNotConfigured when the ACL reports no bridge', async function () {
      const { acl, harness } = await deploySendFixture();
      await acl.setConfidentialBridgeAddress(ethers.ZeroAddress);
      await expect(harness.bridge64(DST_EID, randAddr(), '0x', handleA, 0)).to.be.revertedWithCustomError(
        harness,
        'BridgeNotConfigured',
      );
    });

    it('re-resolves the bridge from the ACL on every call (no caching)', async function () {
      const { acl, harness } = await deploySendFixture();
      const bridge2 = await (await ethers.getContractFactory('MockConfidentialBridge')).deploy();
      await acl.setConfidentialBridgeAddress(await bridge2.getAddress());
      await harness.bridge64(DST_EID, randAddr(), '0x', handleA, 0);
      expect(await bridge2.sendCalled()).to.equal(true);
    });

    it('quotes the native fee for the single-euint64 overload', async function () {
      const { bridge, harness } = await deploySendFixture();
      await bridge.setQuotedNativeFee(777n);
      const fee = await harness.quote64(DST_EID, randAddr(), '0x', handleA, 0);
      expect(fee.nativeFee).to.equal(777n);
      expect(fee.lzTokenFee).to.equal(0n);
    });

    it('quotes a multi-type (euint32) and a typed-array (euint64[]) send', async function () {
      const { bridge, harness } = await deploySendFixture();
      await bridge.setQuotedNativeFee(42n);
      expect((await harness.quote32(DST_EID, randAddr(), '0x', handleA, 0)).nativeFee).to.equal(42n);
      expect((await harness.quoteArray64(DST_EID, randAddr(), '0x', [handleA, handleB], 0)).nativeFee).to.equal(42n);
    });

    it('quotes through the low-level overload', async function () {
      const { bridge, harness } = await deploySendFixture();
      await bridge.setQuotedNativeFee(99n);
      const fee = await harness.quoteList(DST_EID, padded(randAddr()), '0x', [handleA], 0, '0xbeef');
      expect(fee.nativeFee).to.equal(99n);
    });
  });

  describe('receive side — ConfidentialBridgeReceiver', function () {
    const peer = h('0xabcd');
    const payload = ethers.toUtf8Bytes('mint');
    const GUID = h('0x77');

    async function deployReceiveFixture() {
      const [, app] = await ethers.getSigners();
      const acl = await (await ethers.getContractFactory('MockACL')).deploy();
      const bridge = await (await ethers.getContractFactory('MockConfidentialBridge')).deploy();
      await acl.setConfidentialBridgeAddress(await bridge.getAddress());
      const receiver = await (await ethers.getContractFactory('OAppHarness')).deploy(await acl.getAddress());
      return { app, acl, bridge, receiver };
    }

    it('resolves the trusted bridge from the ACL (same source as the send path)', async function () {
      const { bridge, receiver } = await deployReceiveFixture();
      expect(await receiver.confidentialBridge()).to.equal(await bridge.getAddress());
    });

    it('tracks an ACL bridge-address change for inbound auth', async function () {
      const { acl, receiver } = await deployReceiveFixture();
      const bridge2 = await (await ethers.getContractFactory('MockConfidentialBridge')).deploy();
      await acl.setConfidentialBridgeAddress(await bridge2.getAddress());
      expect(await receiver.confidentialBridge()).to.equal(await bridge2.getAddress());
    });

    it('rejects onConfidentialBridgeReceived from a caller that is not the bridge', async function () {
      const { app, receiver } = await deployReceiveFixture();
      const asApp = receiver.connect(app) as typeof receiver;
      await expect(
        asApp.onConfidentialBridgeReceived(SRC_EID, peer, payload, [], [handleA], GUID),
      ).to.be.revertedWithCustomError(receiver, 'OnlyConfidentialBridge');
    });

    it('rejects an unregistered peer even when the bridge calls', async function () {
      const { bridge, receiver } = await deployReceiveFixture();
      await expect(
        bridge.deliver(await receiver.getAddress(), SRC_EID, peer, payload, [], [handleA], GUID),
      ).to.be.revertedWithCustomError(receiver, 'UntrustedPeer');
    });

    it('rejects a peer mismatch (registered, but different srcApp)', async function () {
      const { bridge, receiver } = await deployReceiveFixture();
      await receiver.setPeer(SRC_EID, peer);
      await expect(
        bridge.deliver(await receiver.getAddress(), SRC_EID, h('0x9999'), payload, [], [handleA], GUID),
      ).to.be.revertedWithCustomError(receiver, 'UntrustedPeer');
    });

    it('registers a peer, emits PeerSet, and exposes it via peers', async function () {
      const { receiver } = await deployReceiveFixture();
      await expect(receiver.setPeer(SRC_EID, peer)).to.emit(receiver, 'PeerSet').withArgs(SRC_EID, peer);
      expect(await receiver.peers(SRC_EID)).to.equal(peer);
    });

    it('dispatches typed handles to the app hook once caller + peer check out', async function () {
      const { bridge, receiver } = await deployReceiveFixture();
      await receiver.setPeer(SRC_EID, peer);

      await bridge.deliver(await receiver.getAddress(), SRC_EID, peer, payload, [handleC], [handleA, handleB], GUID);

      expect(await receiver.receiveCount()).to.equal(1n);
      expect(await receiver.lastSrcEid()).to.equal(SRC_EID);
      expect(await receiver.lastSrcApp()).to.equal(peer);
      expect(await receiver.lastPayload()).to.equal(ethers.hexlify(payload));
      expect(await receiver.lastGuid()).to.equal(GUID);
      expect(await receiver.lastHandlesLength()).to.equal(2n);
      expect(await receiver.lastHandles(0)).to.equal(handleA);
      expect(await receiver.lastHandles(1)).to.equal(handleB);
    });

    it('dispatches a single handle', async function () {
      const { bridge, receiver } = await deployReceiveFixture();
      await receiver.setPeer(SRC_EID, peer);
      await bridge.deliver(await receiver.getAddress(), SRC_EID, peer, payload, [], [handleA], GUID);
      expect(await receiver.lastHandlesLength()).to.equal(1n);
      expect(await receiver.lastHandles(0)).to.equal(handleA);
    });

    it('ignores the source-chain handle list (only dst handles are dispatched)', async function () {
      const { bridge, receiver } = await deployReceiveFixture();
      await receiver.setPeer(SRC_EID, peer);
      // srcHandleList differs from dstHandleList; only the dst list must reach the hook.
      await bridge.deliver(await receiver.getAddress(), SRC_EID, peer, payload, [handleC, handleC], [handleB], GUID);
      expect(await receiver.lastHandlesLength()).to.equal(1n);
      expect(await receiver.lastHandles(0)).to.equal(handleB);
    });

    it('counts repeated deliveries and reflects the latest one', async function () {
      const { bridge, receiver } = await deployReceiveFixture();
      await receiver.setPeer(SRC_EID, peer);
      const addr = await receiver.getAddress();
      await bridge.deliver(addr, SRC_EID, peer, payload, [], [handleA], GUID);
      await bridge.deliver(addr, SRC_EID, peer, payload, [], [handleB], GUID);
      expect(await receiver.receiveCount()).to.equal(2n);
      expect(await receiver.lastHandles(0)).to.equal(handleB);
    });

    it('lets a peer be re-registered and rejects the old one afterwards', async function () {
      const { bridge, receiver } = await deployReceiveFixture();
      const addr = await receiver.getAddress();
      await receiver.setPeer(SRC_EID, peer);
      const newPeer = h('0xfeed');
      await receiver.setPeer(SRC_EID, newPeer);

      await bridge.deliver(addr, SRC_EID, newPeer, payload, [], [handleA], GUID); // new peer ok
      expect(await receiver.receiveCount()).to.equal(1n);
      await expect(
        bridge.deliver(addr, SRC_EID, peer, payload, [], [handleA], GUID), // old peer rejected
      ).to.be.revertedWithCustomError(receiver, 'UntrustedPeer');
    });

    it('honors a custom trust policy via the isPeer override', async function () {
      const { bridge, receiver } = await deployReceiveFixture();
      const addr = await receiver.getAddress();
      const stranger = h('0xdead'); // no peer registered for SRC_EID

      // Default policy rejects an unregistered/mismatched peer.
      await expect(bridge.deliver(addr, SRC_EID, stranger, payload, [], [handleA], GUID)).to.be.revertedWithCustomError(
        receiver,
        'UntrustedPeer',
      );

      // A permissive override accepts the same delivery.
      await receiver.setTrustAllPeers(true);
      await bridge.deliver(addr, SRC_EID, stranger, payload, [], [handleA], GUID);
      expect(await receiver.receiveCount()).to.equal(1n);
      expect(await receiver.lastHandles(0)).to.equal(handleA);
    });
  });

  describe('send side — ConfidentialOAppSender (peer-resolved)', function () {
    async function deployOAppFixture() {
      const acl = await (await ethers.getContractFactory('MockACL')).deploy();
      const bridge = await (await ethers.getContractFactory('MockConfidentialBridge')).deploy();
      await acl.setConfidentialBridgeAddress(await bridge.getAddress());
      const oapp = await (await ethers.getContractFactory('OAppHarness')).deploy(await acl.getAddress());
      return { acl, bridge, oapp };
    }

    it('resolves the destination peer from the registry and bridges to it', async function () {
      const { bridge, oapp } = await deployOAppFixture();
      const peer = padded(randAddr());
      await oapp.setPeer(DST_EID, peer);
      const fee = ethers.parseEther('0.01');

      await oapp.bridgeToPeer(DST_EID, '0x', handleA, 200_000, { value: fee });

      const sent = await bridge.lastSend();
      expect(sent.dstApp).to.equal(peer); // the registered peer, not an arg
      expect(sent.handleList).to.deep.equal([handleA]);
      expect(sent.lzComposeGas).to.equal(200_000n);
      expect(sent.value).to.equal(fee);
    });

    it('reverts NoPeer when no peer is configured for the destination eid', async function () {
      const { oapp } = await deployOAppFixture();
      await expect(oapp.bridgeToPeer(DST_EID, '0x', handleA, 0)).to.be.revertedWithCustomError(oapp, 'NoPeer');
    });
  });

  describe('round trip — two OApps send and receive together', function () {
    it('a value sent by the source OApp is delivered to the destination OApp', async function () {
      // One mock ACL + bridge ties the two sides together. In production the two OApp instances
      // live on different chains; here a single mock bridge plays both roles so we can follow one
      // message end to end.
      const acl = await (await ethers.getContractFactory('MockACL')).deploy();
      const bridge = await (await ethers.getContractFactory('MockConfidentialBridge')).deploy();
      await acl.setConfidentialBridgeAddress(await bridge.getAddress());

      // Two OApps — each is a ConfidentialOAppSender + ConfidentialOAppReceiver.
      const srcApp = await (await ethers.getContractFactory('OAppHarness')).deploy(await acl.getAddress());
      const dstApp = await (await ethers.getContractFactory('OAppHarness')).deploy(await acl.getAddress());
      const srcAppAddr = await srcApp.getAddress();
      const dstAppAddr = await dstApp.getAddress();

      // Wire peers once (one entry per direction, by endpoint id).
      await srcApp.setPeer(DST_EID, padded(dstAppAddr)); // source knows where to send
      await dstApp.setPeer(SRC_EID, padded(srcAppAddr)); // destination trusts the source app

      const payload = ethers.toUtf8Bytes('hello from the source chain');
      const srcHandle = handleA;
      const fee = ethers.parseEther('0.01');
      const guid = h('0xa11ce5');

      // 1) SEND — the source OApp bridges to its peer on DST_EID via the sender helper.
      await srcApp.bridgeToPeer(DST_EID, payload, srcHandle, 100_000, { value: fee });

      // The bridge captured exactly what was forwarded — to the destination app, with the fee.
      const sent = await bridge.lastSend();
      expect(sent.dstApp).to.equal(padded(dstAppAddr));
      expect(sent.payload).to.equal(ethers.hexlify(payload));
      expect(sent.handleList).to.deep.equal([srcHandle]);
      expect(sent.value).to.equal(fee);

      // 2) DELIVER — the bridge derives a *new* destination handle (handles are chain-specific)
      //    and calls the destination OApp. We model the derived handle as `dstHandle`.
      const dstHandle = handleB;
      await bridge.deliver(
        dstAppAddr,
        SRC_EID,
        padded(srcAppAddr),
        sent.payload,
        [...sent.handleList],
        [dstHandle],
        guid,
      );

      // 3) The destination OApp authenticated the bridge + peer and received the destination
      //    handle, the original payload, and the message guid.
      expect(await dstApp.receiveCount()).to.equal(1n);
      expect(await dstApp.lastSrcEid()).to.equal(SRC_EID);
      expect(await dstApp.lastSrcApp()).to.equal(padded(srcAppAddr));
      expect(await dstApp.lastPayload()).to.equal(ethers.hexlify(payload));
      expect(await dstApp.lastGuid()).to.equal(guid);
      expect(await dstApp.lastHandlesLength()).to.equal(1n);
      expect(await dstApp.lastHandles(0)).to.equal(dstHandle);
    });
  });
});
