import { expect } from 'chai';
import { ethers } from 'hardhat';

import type { EncryptedERC20 } from '../../types/contracts';
import { createInstance as createHardhatInstance } from '../instance';
import { isLiveNetwork } from '../network';
import { getSigners as getHardhatSigners, initSigners } from '../signers';
import { deployChainFixture } from './multiChain.fixture';
import type { ChainConfig } from './multiChainHelper';
import {
  HOST_CHAINS,
  createInstance,
  evmRevert,
  evmSnapshot,
  getProvider,
  getSigners,
  getWallet,
} from './multiChainHelper';

async function getVersion(chain: ChainConfig, address: string): Promise<string | undefined> {
  const contract = new ethers.Contract(address, ['function getVersion() view returns (string)'], getProvider(chain));
  try {
    return await contract.getVersion();
  } catch {
    return undefined;
  }
}

describe('Multi-Chain State Isolation', function () {
  this.timeout(300_000);

  before(async function () {
    if (HOST_CHAINS.length < 2) {
      this.skip();
      return;
    }

    this.chains = HOST_CHAINS;
    await initSigners(2);
    this.hardhatSigners = await getHardhatSigners();
    this.signersA = getSigners(this.chains[0]);
    this.signersB = getSigners(this.chains[1]);

    this.deployerA = getWallet(this.chains[0], 50);
    this.deployerB = getWallet(this.chains[1], 50);

    this.chainA = await deployChainFixture(this.deployerA);
    this.chainB = await deployChainFixture(this.deployerB);
  });

  afterEach(async function () {
    try {
      await ethers.provider.send('evm_setAutomine', [true]);
    } catch {
      // automine may already be enabled
    }
  });

  describe('Canonical Host Contracts Topology', function () {
    it('KMSGeneration is deployed only on the canonical host', async function () {
      const kmsGenAddress = process.env.KMS_GENERATION_CONTRACT_ADDRESS;
      if (!kmsGenAddress) {
        this.skip();
        return;
      }
      expect(kmsGenAddress, 'KMS_GENERATION_CONTRACT_ADDRESS must be set in the e2e env').to.match(
        /^0x[0-9a-fA-F]{40}$/,
      );

      expect(await getVersion(this.chains[0], kmsGenAddress)).to.match(/^KMSGeneration v/);

      await Promise.all(
        this.chains.slice(1).map(async (chain: ChainConfig) => {
          expect(
            chain.kmsGenerationAddress,
            `non-canonical chain ${chain.rpcUrl} must not export KMS_GENERATION_CONTRACT_ADDRESS`,
          ).to.be.undefined;
          const version = await getVersion(chain, kmsGenAddress);
          expect(
            version ?? '',
            `canonical KMSGeneration address ${kmsGenAddress} must not identify as KMSGeneration on ${chain.rpcUrl}`,
          ).not.to.match(/^KMSGeneration v/);
        }),
      );
    });

    it('ProtocolConfig is deployed on every host chain', async function () {
      if (this.chains.some((chain: { protocolConfigAddress?: string }) => !chain.protocolConfigAddress)) {
        this.skip();
        return;
      }
      for (const chain of this.chains) {
        expect(chain.protocolConfigAddress, `chain.protocolConfigAddress must be set for ${chain.rpcUrl}`).to.match(
          /^0x[0-9a-fA-F]{40}$/,
        );
        const code = await getProvider(chain).getCode(chain.protocolConfigAddress!);
        expect(
          code,
          `ProtocolConfig should be deployed at ${chain.protocolConfigAddress} on ${chain.rpcUrl}`,
        ).to.not.eq('0x');
      }
    });
  });

  describe('User Input Across Chains', function () {
    it('encrypted input and FHE computation work independently on both chains', async function () {
      const erc20A = this.chainA.erc20 as unknown as EncryptedERC20;
      const erc20B = this.chainB.erc20 as unknown as EncryptedERC20;

      // Chain A: encrypted transfer to carol (unused by other tests)
      const instanceA = await createInstance(this.chains[0]);

      // const inputA = instanceA.createEncryptedInput(this.chainA.erc20Address, this.deployerA.address);
      // inputA.add64(200);
      // const encryptedA = await inputA.encrypt();

      const encryptedA = await instanceA.encryptUint64({
        value: 200,
        contractAddress: this.chainA.erc20Address,
        userAddress: this.deployerA.address,
      });

      const txA = await (erc20A.connect(this.deployerA) as EncryptedERC20)['transfer(address,bytes32,bytes)'](
        this.signersA.carol.address,
        encryptedA.handles[0],
        encryptedA.inputProof,
        { gasLimit: 10_000_000 },
      );
      const receiptA = await txA.wait();
      expect(receiptA?.status).to.eq(1);

      const balanceHandleCarolA = await erc20A.balanceOf(this.signersA.carol.address);
      expect(balanceHandleCarolA).to.not.eq(ethers.ZeroHash);
      expect(balanceHandleCarolA.slice(46, 62)).to.eq(this.chains[0].chainId.toString(16).padStart(16, '0'));

      const decryptedA = await instanceA.userDecryptSingleHandle({
        handle: balanceHandleCarolA,
        contractAddress: this.chainA.erc20Address,
        signer: this.signersA.carol,
      });
      expect(decryptedA).to.equal(200n);

      // Chain B: encrypted transfer to carol
      const instanceB = await createInstance(this.chains[1]);
      const encryptedB = await instanceB.encryptUint64({
        contractAddress: this.chainB.erc20Address,
        userAddress: this.deployerB.address,
        value: 300,
      });

      const txB = await (erc20B.connect(this.deployerB) as EncryptedERC20)['transfer(address,bytes32,bytes)'](
        this.signersB.carol.address,
        encryptedB.handles[0],
        encryptedB.inputProof,
        { gasLimit: 10_000_000 },
      );
      const receiptB = await txB.wait();
      expect(receiptB?.status).to.eq(1);

      const balanceHandleCarolB = await erc20B.balanceOf(this.signersB.carol.address);
      expect(balanceHandleCarolB).to.not.eq(ethers.ZeroHash);
      expect(balanceHandleCarolB.slice(46, 62)).to.eq(this.chains[1].chainId.toString(16).padStart(16, '0'));

      const decryptedB = await instanceB.userDecryptSingleHandle({
        handle: balanceHandleCarolB,
        contractAddress: this.chainB.erc20Address,
        signer: this.signersB.carol,
      });
      expect(decryptedB).to.equal(300n);
    });
  });

  describe('Ciphertext Handle Isolation', function () {
    it('ciphertext handle from Chain A cannot be used on Chain B', async function () {
      const erc20A = this.chainA.erc20 as unknown as EncryptedERC20;
      const erc20B = this.chainB.erc20 as unknown as EncryptedERC20;

      expect(this.deployerA.address).to.eq(this.deployerB.address);
      expect(this.chainA.erc20Address).to.eq(this.chainB.erc20Address);

      const instanceA = await createInstance(this.chains[0]);

      // const inputA = instanceA.createEncryptedInput(this.chainA.erc20Address, this.deployerA.address);
      // inputA.add64(125);
      // const encryptedA = await inputA.encrypt();
      const encryptedA = await instanceA.encryptUint64({
        contractAddress: this.chainA.erc20Address,
        userAddress: this.deployerA.address,
        value: 125,
      });

      const txA = await (erc20A.connect(this.deployerA) as EncryptedERC20)['transfer(address,bytes32,bytes)'](
        this.signersA.dave.address,
        encryptedA.handles[0],
        encryptedA.inputProof,
        { gasLimit: 10_000_000 },
      );
      const receiptA = await txA.wait();
      expect(receiptA?.status).to.eq(1);

      const balanceHandleDaveA = await erc20A.balanceOf(this.signersA.dave.address);
      expect(balanceHandleDaveA).to.not.eq(ethers.ZeroHash);
      expect(balanceHandleDaveA.slice(46, 62)).to.eq(this.chains[0].chainId.toString(16).padStart(16, '0'));

      const handleB = await erc20B.balanceOf(this.deployerB.address);
      expect(handleB).to.not.eq(ethers.ZeroHash);
      expect(handleB.slice(46, 62)).to.eq(this.chains[1].chainId.toString(16).padStart(16, '0'));

      let crossChainSucceeded = false;
      try {
        const crossChainTx = await (erc20B.connect(this.deployerB) as EncryptedERC20)[
          'transfer(address,bytes32,bytes)'
        ](this.signersB.bob.address, encryptedA.handles[0], encryptedA.inputProof, { gasLimit: 10_000_000 });
        await crossChainTx.wait();
        crossChainSucceeded = true;
      } catch {
        // Expected: Chain A input should be rejected on Chain B.
      }
      expect(crossChainSucceeded).to.eq(false, 'cross-chain transfer should have reverted');

      expect(await erc20B.balanceOf(this.deployerB.address)).to.eq(handleB);
    });
  });

  describe('ACL Permission Isolation', function () {
    it('ACL allow on Chain A does not grant access on Chain B', async function () {
      const erc20A = this.chainA.erc20 as unknown as EncryptedERC20;

      expect(this.signersA.bob.address).to.eq(this.signersB.bob.address);
      expect(this.chainA.erc20Address).to.eq(this.chainB.erc20Address);

      const instanceDeployerA = await createInstance(this.chains[0]);
      const instanceBobA = await createInstance(this.chains[0]);

      // const inputA = instanceDeployerA.createEncryptedInput(this.chainA.erc20Address, this.deployerA.address);
      // inputA.add64(500);
      // const encryptedAmountA = await inputA.encrypt();
      const encryptedAmountA = await instanceDeployerA.encryptUint64({
        contractAddress: this.chainA.erc20Address,
        userAddress: this.deployerA.address,
        value: 500,
      });

      const transferTx = await (erc20A.connect(this.deployerA) as EncryptedERC20)['transfer(address,bytes32,bytes)'](
        this.signersA.bob.address,
        encryptedAmountA.handles[0],
        encryptedAmountA.inputProof,
        { gasLimit: 10_000_000 },
      );
      const transferReceipt = await transferTx.wait();
      expect(transferReceipt?.status).to.eq(1);

      const balanceHandleBobA = await erc20A.balanceOf(this.signersA.bob.address);
      expect(balanceHandleBobA).to.not.eq(ethers.ZeroHash);

      const balanceBobA = await instanceBobA.userDecryptSingleHandle({
        handle: balanceHandleBobA,
        contractAddress: this.chainA.erc20Address,
        signer: this.hardhatSigners.bob,
      });
      expect(balanceBobA).to.equal(500n);

      const aclAbi = ['function isAllowed(bytes32 handle, address account) view returns (bool)'];
      const aclA = new ethers.Contract(this.chains[0].aclAddress, aclAbi, getProvider(this.chains[0]));
      const aclB = new ethers.Contract(this.chains[1].aclAddress, aclAbi, getProvider(this.chains[1]));
      expect(await aclA.isAllowed(balanceHandleBobA, this.signersA.bob.address)).to.eq(true);
      expect(await aclB.isAllowed(balanceHandleBobA, this.signersB.bob.address)).to.eq(false);

      const instanceBobB = await createInstance(this.chains[1]);
      let crossChainDecryptSucceeded = false;
      try {
        await instanceBobB.userDecryptSingleHandle({
          handle: balanceHandleBobA,
          contractAddress: this.chainB.erc20Address,
          signer: this.signersB.bob,
        });

        crossChainDecryptSucceeded = true;
      } catch {
        // Expected: Chain A ACL permission should not authorize Chain B decryption.
      }
      expect(crossChainDecryptSucceeded).to.eq(false, 'cross-chain user decrypt should have been rejected');
    });
  });

  describe('Block Reorg Isolation', function () {
    it('evm_revert on Chain A does not affect Chain B state', async function () {
      if (isLiveNetwork()) {
        this.skip();
      }

      const providerB = getProvider(this.chains[1]);
      const erc20A = this.chainA.erc20 as unknown as EncryptedERC20;
      const erc20B = this.chainB.erc20 as unknown as EncryptedERC20;

      const supplyABefore = await erc20A.totalSupply();
      const supplyBBefore = await erc20B.totalSupply();
      const chainBBlockBefore = await providerB.getBlockNumber();

      const snapshotId = await evmSnapshot(ethers.provider);

      const mintTx = await (erc20A.connect(this.deployerA) as EncryptedERC20).mint(7000, { gasLimit: 10_000_000 });
      await mintTx.wait();
      expect(await erc20A.totalSupply()).to.eq(supplyABefore + 7000n);

      const transferB = await this.deployerB.sendTransaction({
        to: this.signersB.bob.address,
        value: ethers.parseEther('0.1'),
      });
      const receiptB = await transferB.wait();

      const chainBBlockDuring = receiptB!.blockNumber;
      expect(chainBBlockDuring).to.be.greaterThan(chainBBlockBefore);

      const reverted = await evmRevert(ethers.provider, snapshotId);
      expect(reverted).to.eq(true);

      // After evm_revert, reset NonceManager so it re-fetches the reverted nonce
      if (this.deployerA.reset) this.deployerA.reset();

      expect(await erc20A.totalSupply()).to.eq(supplyABefore);
      expect(await erc20B.totalSupply()).to.eq(supplyBBefore);
      expect(await providerB.getBlockNumber()).to.be.greaterThanOrEqual(chainBBlockDuring);
    });
  });

  describe('Chain Halt Isolation', function () {
    it('Chain A halt does not affect Chain B operations', async function () {
      if (isLiveNetwork()) {
        this.skip();
      }

      const providerB = getProvider(this.chains[1]);
      const chainABlockBefore = await ethers.provider.getBlockNumber();

      await ethers.provider.send('evm_setAutomine', [false]);
      await ethers.provider.send('evm_setIntervalMining', [0]);

      try {
        const chainBBlockBefore = await providerB.getBlockNumber();

        const freshRecipient = ethers.Wallet.createRandom();
        const ethTransferTx = await this.deployerB.sendTransaction({
          to: freshRecipient.address,
          value: ethers.parseEther('0.5'),
        });
        await ethTransferTx.wait();
        expect(await providerB.getBalance(freshRecipient.address)).to.eq(ethers.parseEther('0.5'));

        // Wait to mine a new block
        await new Promise((resolve) => setTimeout(resolve, 3000));
        expect(await ethers.provider.getBlockNumber()).to.eq(chainABlockBefore);
        expect(await providerB.getBlockNumber()).to.be.greaterThan(chainBBlockBefore);
      } finally {
        await ethers.provider.send('evm_setAutomine', [true]);
      }
    });
  });

  describe('Simultaneous Decryption Across Chains', function () {
    it('same user can decrypt on both chains independently', async function () {
      const handleA = await this.chainA.userDecrypt.xUint64();
      const handleB = await this.chainB.userDecrypt.xUint64();
      expect(handleA).to.not.eq(ethers.ZeroHash);
      expect(handleB).to.not.eq(ethers.ZeroHash);
      expect(handleA).to.not.eq(handleB);

      expect(handleA.slice(46, 62)).to.eq(this.chains[0].chainId.toString(16).padStart(16, '0'));
      expect(handleB.slice(46, 62)).to.eq(this.chains[1].chainId.toString(16).padStart(16, '0'));

      const instanceA = await createInstance(this.chains[0]);
      const decryptedA = await instanceA.userDecryptSingleHandle({
        handle: handleA,
        contractAddress: this.chainA.userDecryptAddress,
        signer: this.deployerA,
      });
      expect(decryptedA).to.equal(18446744073709551600n);

      const instanceB = await createInstance(this.chains[1]);
      const decryptedB = await instanceB.userDecryptSingleHandle({
        handle: handleB,
        contractAddress: this.chainB.userDecryptAddress,
        signer: this.deployerB,
      });
      expect(decryptedB).to.equal(18446744073709551600n);
    });
  });

  describe('Random Number Generation Independence', function () {
    it('random numbers generated on Chain A and Chain B are independent', async function () {
      const handlesA: string[] = [];
      for (let i = 0; i < 3; i++) {
        const txn = await this.chainA.rand.connect(this.deployerA).generate64({ gasLimit: 10_000_000 });
        await txn.wait();
        const valueHandle = await this.chainA.rand.value64();
        expect(valueHandle).to.not.eq(ethers.ZeroHash);
        handlesA.push(valueHandle);
      }

      const handlesB: string[] = [];
      for (let i = 0; i < 3; i++) {
        const txn = await this.chainB.rand.connect(this.deployerB).generate64({ gasLimit: 10_000_000 });
        await txn.wait();
        const valueHandle = await this.chainB.rand.value64();
        expect(valueHandle).to.not.eq(ethers.ZeroHash);
        handlesB.push(valueHandle);
      }

      for (const handleA of handlesA) {
        for (const handleB of handlesB) {
          expect(handleA).to.not.eq(handleB);
        }
      }

      for (const handle of handlesA) {
        expect(handle.slice(46, 62)).to.eq(this.chains[0].chainId.toString(16).padStart(16, '0'));
      }
      for (const handle of handlesB) {
        expect(handle.slice(46, 62)).to.eq(this.chains[1].chainId.toString(16).padStart(16, '0'));
      }

      expect(new Set(handlesA).size).to.be.greaterThanOrEqual(2);
      expect(new Set(handlesB).size).to.be.greaterThanOrEqual(2);

      const instanceA = await createHardhatInstance();
      const valuesA: bigint[] = [];
      for (const handle of handlesA) {
        const res = await instanceA.publicDecrypt([handle]);
        const value = res.clearValues[handle as `0x${string}`] as bigint;
        expect(typeof value).to.eq('bigint');
        valuesA.push(value);
      }
      expect(new Set(valuesA).size).to.be.greaterThanOrEqual(2);
    });
  });
});
