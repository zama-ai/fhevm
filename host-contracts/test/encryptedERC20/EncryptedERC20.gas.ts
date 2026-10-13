import { ethers } from 'hardhat';

import { createInstances } from '../instance';
import { getSigners, initSigners } from '../signers';
import type { ContractTransactionResponse, ContractTransactionReceipt } from 'ethers';

describe('EncryptedERC20:Gas', function () {
  before(async function () {
    await initSigners(2);
    this.signers = await getSigners();
    this.instances = await createInstances(this.signers);
  });

  it('EncryptedERC20 gas - deployment and usage', async function () {
    const contractFactory = await ethers.getContractFactory('EncryptedERC20');
    const contract = await contractFactory.connect(this.signers.alice).deploy('Naraggara', 'NARA');
    const contractAddress = await contract.getAddress();
    const deployTx: ContractTransactionResponse | null = contract.deploymentTransaction();
    if (deployTx === null) {
      throw new Error('EncryptedERC20 deployment transaction is missing');
    }
    const receipt: ContractTransactionReceipt | null = await deployTx.wait(1);
    if (receipt === null) {
      throw new Error('EncryptedERC20 deployment transaction has no receipt');
    }
    console.log('Gas consumed in EncryptedERC20 deployment:', '\x1b[1m' + receipt.gasUsed.toString() + '\x1b[0m');

    const transaction = await contract.mint(1000);
    const rcptMint: ContractTransactionReceipt | null = await transaction.wait();
    if (rcptMint === null) {
      throw new Error('EncryptedERC20 mint transaction has no receipt');
    }

    console.log('Gas consumed in EncryptedERC20 Mint tx:', '\x1b[1m' + rcptMint.gasUsed.toString() + '\x1b[0m');

    const input = this.instances.alice.createEncryptedInput(contractAddress, this.signers.alice.address);
    input.add64(1337);
    const encryptedTransferAmount = await input.encrypt();
    const tx = await contract['transfer(address,bytes32,bytes)'](
      this.signers.bob.address,
      encryptedTransferAmount.handles[0],
      encryptedTransferAmount.inputProof,
    );
    const rcptTransfer: ContractTransactionReceipt | null = await tx.wait();
    if (rcptTransfer === null) {
      throw new Error('EncryptedERC20 transfer transaction has no receipt');
    }
    console.log('Gas consumed in EncryptedERC20 Transfer tx:', '\x1b[1m' + rcptTransfer.gasUsed.toString() + '\x1b[0m');

    const inputAlice = this.instances.alice.createEncryptedInput(contractAddress, this.signers.alice.address);
    inputAlice.add64(1337);
    const encryptedAllowanceAmount = await inputAlice.encrypt();
    const txbis = await contract['approve(address,bytes32,bytes)'](
      this.signers.bob.address,
      encryptedAllowanceAmount.handles[0],
      encryptedAllowanceAmount.inputProof,
    );
    const rcptApprove: ContractTransactionReceipt | null = await txbis.wait();
    if (rcptApprove === null) {
      throw new Error('EncryptedERC20 approve transaction has no receipt');
    }
    console.log('Gas consumed in EncryptedERC20 Approve tx:', '\x1b[1m' + rcptApprove.gasUsed.toString() + '\x1b[0m');

    const bobErc20 = contract.connect(this.signers.bob);
    const inputBob1 = this.instances.bob.createEncryptedInput(contractAddress, this.signers.bob.address);
    inputBob1.add64(1338); // above allowance so next tx should actually not send any token
    const encryptedTransferAmount2 = await inputBob1.encrypt();
    const tx2 = await bobErc20['transferFrom(address,address,bytes32,bytes)'](
      this.signers.alice.address,
      this.signers.bob.address,
      encryptedTransferAmount2.handles[0],
      encryptedTransferAmount2.inputProof,
    );
    const rcptTransferFrom: ContractTransactionReceipt | null = await tx2.wait();
    if (rcptTransferFrom === null) {
      throw new Error('EncryptedERC20 transferFrom transaction has no receipt');
    }
    console.log(
      'Gas consumed in EncryptedERC20 TransferFrom tx:',
      '\x1b[1m' + rcptTransferFrom.gasUsed.toString() + '\x1b[0m',
    );
  });
});
