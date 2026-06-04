import { expect } from 'chai';
import { ethers } from 'hardhat';

import { createInstances } from '../instance';
import { getSigners, initSigners } from '../signers';
import { DST_EID, SRC_EID, deployConfidentialOFTFixture } from './fixture';

describe('ConfidentialOFT', function () {
  before(async function () {
    await initSigners(3);
    this.signers = await getSigners();
  });

  beforeEach(async function () {
    const fx = await deployConfidentialOFTFixture();
    this.fx = fx;
    this.oft = fx.oft;
    this.oftAddress = await fx.oft.getAddress();
    // The OFT is wired to the destination-side bridge — that is what authenticates
    // `onReceive` calls and dispatches outbound sends.
    this.bridge = fx.dstBridge;
    this.instances = await createInstances(this.signers);
  });

  describe('governance', function () {
    it('owner can toggle a trusted peer', async function () {
      // setTrustedPeer takes bytes32 (forward-compat with non-EVM peers). For EVM
      // peers, pad the address to 32 bytes.
      const peerAddr = this.signers.bob.address;
      const peer = ethers.zeroPadValue(peerAddr, 32);
      expect(await this.oft.isTrustedPeer(SRC_EID, peer)).to.equal(false);

      await expect(this.oft.connect(this.fx.owner).setTrustedPeer(SRC_EID, peer, true))
        .to.emit(this.oft, 'TrustedPeerSet')
        .withArgs(SRC_EID, peer, true);
      expect(await this.oft.isTrustedPeer(SRC_EID, peer)).to.equal(true);

      await this.oft.connect(this.fx.owner).setTrustedPeer(SRC_EID, peer, false);
      expect(await this.oft.isTrustedPeer(SRC_EID, peer)).to.equal(false);
    });

    it('non-owner cannot setTrustedPeer', async function () {
      await expect(
        this.oft
          .connect(this.signers.bob)
          .setTrustedPeer(SRC_EID, ethers.zeroPadValue(this.signers.bob.address, 32), true)
      ).to.be.reverted;
    });
  });

  describe('onReceive authentication', function () {
    it('reverts when caller is not the ConfidentialBridge', async function () {
      // Caller is signers.bob; the OFT only accepts the bridge.
      await expect(
        this.oft
          .connect(this.signers.bob)
          .onReceive(
            SRC_EID,
            ethers.zeroPadValue(this.signers.alice.address, 32),
            ethers.AbiCoder.defaultAbiCoder().encode(
              ['address', 'bytes32'],
              [this.signers.alice.address, ethers.ZeroHash]
            ),
            [],
            []
          )
      )
        .to.be.revertedWithCustomError(this.oft, 'OnlyConfidentialBridge')
        .withArgs(this.signers.bob.address);
    });

    it('reverts when peer is not trusted', async function () {
      const bridgeAddr = await this.bridge.getAddress();
      await impersonate(bridgeAddr);
      await fundAddress(bridgeAddr);
      const bridgeSigner = await ethers.getSigner(bridgeAddr);

      const untrustedPeer = ethers.zeroPadValue(this.signers.bob.address, 32);
      await expect(
        this.oft
          .connect(bridgeSigner)
          .onReceive(
            SRC_EID,
            untrustedPeer,
            ethers.AbiCoder.defaultAbiCoder().encode(
              ['address', 'bytes32'],
              [this.signers.alice.address, ethers.ZeroHash]
            ),
            [],
            []
          )
      )
        .to.be.revertedWithCustomError(this.oft, 'UntrustedPeer')
        .withArgs(SRC_EID, untrustedPeer);
      await stopImpersonating(bridgeAddr);
    });
  });

  describe('send: sender-side checks', function () {
    it('reverts when the caller does not hold ACL allowance on the amount', async function () {
      // signers.alice deposits and gets a balance, then a different signer tries to send her amount.
      // Faster: just pass a fresh handle that has no allowance for anyone.
      const fakeAmount = makeFakeAmountHandle();
      await expect(
        this.oft
          .connect(this.signers.alice)
          .send(
            DST_EID,
            ethers.zeroPadValue(this.signers.alice.address, 32),
            fakeAmount,
            this.signers.bob.address,
            200_000n,
            { value: ethers.parseEther('1') }
          )
      ).to.be.reverted;
    });
  });

  describe('view methods', function () {
    it('returns zero balance handle for accounts that never received tokens', async function () {
      const balance = await this.oft.balanceOf(this.signers.bob.address);
      expect(balance).to.equal(ethers.ZeroHash);
    });

    it('exposes the immutable confidentialBridge address', async function () {
      expect(await this.oft.confidentialBridge()).to.equal(await this.fx.dstBridge.getAddress());
    });
  });
});

function makeFakeAmountHandle(): string {
  // bytes 0..21 random-ish | byte 21 = 0xff | bytes 22..29 = 0 | byte 30 = 0x05 (Uint64) | byte 31 = 0
  const raw = ethers.keccak256(ethers.toUtf8Bytes(`oft-test-handle-${Date.now()}-${Math.random()}`));
  return raw.slice(0, 2 + 21 * 2) + 'ff' + '0000000000000000' + '05' + '00';
}

async function impersonate(addr: string) {
  await ethers.provider.send('hardhat_impersonateAccount', [addr]);
}

async function stopImpersonating(addr: string) {
  await ethers.provider.send('hardhat_stopImpersonatingAccount', [addr]);
}

async function fundAddress(addr: string, weiHex = '0xde0b6b3a7640000' /* 1 ether */) {
  await ethers.provider.send('hardhat_setBalance', [addr, weiHex]);
}
