import { expect } from 'chai';
import { ethers } from 'hardhat';

import { getSigners, initSigners } from '../signers';
import { DST_CHAIN_ID, DST_EID, SRC_EID, deployBridgeFixture } from './fixture';

describe('Bridge', function () {
  before(async function () {
    await initSigners(3);
    this.signers = await getSigners();
  });

  beforeEach(async function () {
    const fx = await deployBridgeFixture();
    this.fx = fx;
    // The unified ConfidentialBridge plays both roles; alias by direction for clarity.
    this.srcBridge = fx.srcBridge;
    this.dstBridge = fx.dstBridge;
    this.owner = fx.owner;
  });

  describe('ConfidentialBridge: source-side configuration', function () {
    it('exposes the configured destination chain id', async function () {
      expect(await this.srcBridge.getDstChainId(DST_EID)).to.equal(DST_CHAIN_ID);
    });

    it('rejects setDstChainId from a non-owner', async function () {
      const bob = this.signers.bob;
      await expect(this.srcBridge.connect(bob).setDstChainId(DST_EID, 99n)).to.be.reverted;
    });

    it('emits DstChainIdSet and updates the mapping on owner update', async function () {
      await expect(this.srcBridge.connect(this.owner).setDstChainId(DST_EID, 99n))
        .to.emit(this.srcBridge, 'DstChainIdSet')
        .withArgs(DST_EID, 99n);
      expect(await this.srcBridge.getDstChainId(DST_EID)).to.equal(99n);
    });

    it('reports MAX_HANDLES = 32', async function () {
      expect(await this.srcBridge.MAX_HANDLES()).to.equal(32n);
    });
  });

  describe('ConfidentialBridge.send: revert paths', function () {
    /**
     * Build a handle whose metadata bytes look valid. Used only for guard tests
     * the handle is never expected to pass ACL allowance here.
     */
    function makeHandle(seed: number) {
      const raw = ethers.keccak256(ethers.toUtf8Bytes(`test-handle-${seed}`));
      // Clear bytes 21-31 then set: byte 21 = 0xff, bytes 22-29 = chainid (0 in hardhat),
      // byte 30 = 0x05 (Uint64), byte 31 = 0 (HANDLE_VERSION). For this test we only need
      // a stable bytes32; the metadata bake-in matters for the receiver, not the sender.
      const top21 = raw.slice(0, 2 + 21 * 2); // "0x" + 21 bytes
      return top21 + 'ff' + '0000000000000000' + '05' + '00';
    }

    it('reverts UnknownDstEid for an unregistered endpoint id', async function () {
      const handleList = [makeHandle(0)];
      const unknownEid = 99;
      await expect(this.srcBridge.send(unknownEid, ethers.ZeroHash, '0x', handleList, 0, '0x'))
        .to.be.revertedWithCustomError(this.srcBridge, 'UnknownDstEid')
        .withArgs(unknownEid);
    });

    it('reverts TooManyHandles when length exceeds the cap', async function () {
      const max = Number(await this.srcBridge.MAX_HANDLES());
      const handleList = Array.from({ length: max + 1 }, (_, i) => makeHandle(i));
      await expect(this.srcBridge.send(DST_EID, ethers.ZeroHash, '0x', handleList, 0, '0x'))
        .to.be.revertedWithCustomError(this.srcBridge, 'TooManyHandles')
        .withArgs(max + 1, max);
    });

    it('reverts HandleNotAllowed when caller lacks ACL allowance on a handle', async function () {
      const handleList = [makeHandle(0)];
      // signers.bob has no ACL allowance on this fresh handle.
      const bob = this.signers.bob;
      await expect(
        this.srcBridge.connect(bob).send(DST_EID, ethers.ZeroHash, '0x', handleList, 0, '0x')
      ).to.be.revertedWithCustomError(this.srcBridge, 'HandleNotAllowed');
    });

    it('reverts ComposeGasMustBeZeroWithRawOptions when both options and compose gas are non-empty', async function () {
      const aclAddr = await getAclAddress();
      const acl = await ethers.getContractAt('ACL', aclAddr);
      const fhevmExecutor = await acl.getFHEVMExecutorAddress();
      const handle = makeHandle(0);

      // Grant persistent allowance to signers.alice via a single multicall from the
      // FHEVMExecutor. Two sub-calls inside the same tx (delegatecall preserves
      // msg.sender), so transient storage persists between them:
      //   1) allowTransient(h, fhevmExecutor)  — executor bypass triggers, grants transient
      //   2) allow(h, alice)                   — executor is now isAllowed, can grant alice
      await grantAllowanceToUser(acl, fhevmExecutor, handle, this.signers.alice.address);

      // _resolveOptions is checked BEFORE the endpoint send, so this revert fires
      // without us needing a registered send library.
      const rawOpts = '0x00030100110100000000000000000000000000000186a0';
      await expect(
        this.srcBridge
          .connect(this.signers.alice)
          .send(DST_EID, ethers.ZeroHash, '0x', [handle], 50_000n, rawOpts, { value: ethers.parseEther('1') })
      ).to.be.revertedWithCustomError(this.srcBridge, 'ComposeGasMustBeZeroWithRawOptions');
    });
  });

  describe('ConfidentialBridge: destination-side governance', function () {
    it('rejects grantFallbackPlaintext from non-owner', async function () {
      const dst = await makeDstHandle(0);
      await expect(this.dstBridge.connect(this.signers.bob).grantFallbackPlaintext(dst, 42n)).to.be.reverted;
    });

    it('reverts WrongChainIdInDstHandle when the handle encodes a different chain id', async function () {
      // Plain keccak256 has no chain-id metadata baked into bytes 22-29, so the
      // contract's chain-id check on the handle must reject it.
      const dst = ethers.keccak256(ethers.toUtf8Bytes('dst'));
      await expect(this.dstBridge.connect(this.owner).grantFallbackPlaintext(dst, 0n)).to.be.revertedWithCustomError(
        this.dstBridge,
        'WrongChainIdInDstHandle'
      );
    });

    it('emits FallbackGrantedPlaintext when called by the owner', async function () {
      const dst = await makeDstHandle(1);
      const plaintext = 42n;
      await expect(this.dstBridge.connect(this.owner).grantFallbackPlaintext(dst, plaintext))
        .to.emit(this.dstBridge, 'FallbackGrantedPlaintext')
        .withArgs(dst, plaintext);
    });
  });

  describe('ConfidentialBridge: lzCompose authentication', function () {
    it('rejects calls from a non-endpoint caller', async function () {
      const composeMsg = ethers.AbiCoder.defaultAbiCoder().encode(
        ['uint32', 'bytes32', 'bytes32', 'bytes', 'bytes32[]', 'bytes32[]'],
        [
          SRC_EID,
          ethers.zeroPadValue(this.signers.alice.address, 32),
          ethers.zeroPadValue(this.signers.bob.address, 32),
          '0x',
          [],
          [],
        ]
      );
      await expect(
        this.dstBridge
          .connect(this.signers.bob)
          .lzCompose(
            await this.dstBridge.getAddress(),
            ethers.keccak256(ethers.toUtf8Bytes('g')),
            composeMsg,
            ethers.ZeroAddress,
            '0x'
          )
      ).to.be.revertedWithCustomError(this.dstBridge, 'NotLzEndpoint');
    });

    it('rejects compose messages whose `from` is not the bridge itself', async function () {
      const endpointAddr = await this.fx.dstEndpoint.getAddress();
      const composeMsg = ethers.AbiCoder.defaultAbiCoder().encode(
        ['uint32', 'bytes32', 'bytes32', 'bytes', 'bytes32[]', 'bytes32[]'],
        [
          SRC_EID,
          ethers.zeroPadValue(this.signers.alice.address, 32),
          ethers.zeroPadValue(this.signers.bob.address, 32),
          '0x',
          [],
          [],
        ]
      );
      await impersonate(endpointAddr);
      await fundAddress(endpointAddr);
      const endpointSigner = await ethers.getSigner(endpointAddr);
      await expect(
        this.dstBridge
          .connect(endpointSigner)
          .lzCompose(
            this.signers.bob.address,
            ethers.keccak256(ethers.toUtf8Bytes('g')),
            composeMsg,
            ethers.ZeroAddress,
            '0x'
          )
      )
        .to.be.revertedWithCustomError(this.dstBridge, 'UnexpectedComposeOrigin')
        .withArgs(this.signers.bob.address);
      await stopImpersonating(endpointAddr);
    });
  });
});

/**
 * Build a bytes32 handle whose metadata bytes match this chain — required by the
 * destination-side checks (bytes 22-29 = uint64(block.chainid)).
 *   - byte 21:    0xff (computation/bridged marker)
 *   - bytes 22-29: uint64 chain id (this chain) in big-endian
 *   - byte 30:    0x05 (FheType.Uint64)
 *   - byte 31:    HANDLE_VERSION 0
 */
async function makeDstHandle(seed: number): Promise {
  const raw = ethers.keccak256(ethers.toUtf8Bytes(`dst-handle-${seed}`));
  const top21 = raw.slice(2, 2 + 21 * 2); // 21 random bytes (hex)
  const { chainId } = await ethers.provider.getNetwork();
  const chainIdHex = chainId.toString(16).padStart(16, '0'); // 8 bytes (uint64) big-endian
  return '0x' + top21 + 'ff' + chainIdHex + '05' + '00';
}

async function getAclAddress(): Promise {
  const dotenv = await import('dotenv');
  const fs = await import('fs');
  return dotenv.parse(fs.readFileSync('addresses/.env.host')).ACL_CONTRACT_ADDRESS;
}

async function impersonate(addr: string) {
  await ethers.provider.send('hardhat_impersonateAccount', [addr]);
}

async function stopImpersonating(addr: string) {
  await ethers.provider.send('hardhat_stopImpersonatingAccount', [addr]);
}

/**
 * Set the address's balance directly via the hardhat RPC so impersonated contract
 * accounts can pay gas. Avoids sending ETH through a normal transfer, which would
 * revert against contracts without a receive/fallback function.
 */
async function fundAddress(addr: string, weiHex = '0xde0b6b3a7640000' /* 1 ether */) {
  await ethers.provider.send('hardhat_setBalance', [addr, weiHex]);
}

/**
 * Grant persistent ACL allowance on `handle` to `user`. Performs both calls inside
 * a single multicall (which delegatecalls so msg.sender is preserved across calls),
 * making transient storage live across the two sub-calls:
 *   1) allowTransient(handle, fhevmExecutor)  — executor bypass grants itself transient
 *   2) allow(handle, user)                    — executor is now isAllowed, grants `user`
 *
 * This is the Hardhat-friendly equivalent of acl.t.sol's `_allowHandle`, which works
 * implicitly because forge tests run sequential cheatcoded calls in the same tx.
 */
export async function grantAllowanceToUser(acl: any, fhevmExecutor: string, handle: string, user: string) {
  await ethers.provider.send('hardhat_impersonateAccount', [fhevmExecutor]);
  await ethers.provider.send('hardhat_setBalance', [fhevmExecutor, '0xde0b6b3a7640000']);
  const execSigner = await ethers.getSigner(fhevmExecutor);
  const calls = [
    acl.interface.encodeFunctionData('allowTransient', [handle, fhevmExecutor]),
    acl.interface.encodeFunctionData('allow', [handle, user]),
  ];
  await (await acl.connect(execSigner).multicall(calls)).wait();
  await ethers.provider.send('hardhat_stopImpersonatingAccount', [fhevmExecutor]);
}
