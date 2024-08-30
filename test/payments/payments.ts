import { expect } from 'chai';
import { ethers } from 'hardhat';

import { createInstances } from '../instance';
import { FHE_GASPRICE_NATIVE_RATIO, MIN_FHE_GASPRICE, initializeFHEPayment } from '../paymentUtils';
import { getSigners, initSigners } from '../signers';

describe('TestFHEPayment', function () {
  before(async function () {
    await initSigners(2);
    this.signers = await getSigners();
    this.fhePayment = await initializeFHEPayment();
  });

  beforeEach(async function () {
    this.instances = await createInstances(this.signers);
  });

  it('contract which is not funded cannot be used by non-depositor', async function () {
    const contractFactory = await ethers.getContractFactory('EncryptedERC20');
    const contract = await contractFactory.connect(this.signers.alice).deploy('Naraggara', 'NARA', {
      value: ethers.parseEther('0'), // don't fund contract
    });
    await contract.waitForDeployment();
    await expect(contract.mint(1000)).to.be.revertedWithCustomError(this.fhePayment, 'AccountNotEnoughFunded');
  });

  it('contract with enough deposits burns the correct amount of fheGas', async function () {
    const contractFactory = await ethers.getContractFactory('EncryptedERC20');
    const contract = await contractFactory.connect(this.signers.alice).deploy('Naraggara', 'NARA', {
      value: ethers.parseEther('0.001'),
    });
    const initialDeposit = await this.fhePayment.getAvailableDepositsETH(await contract.getAddress());
    await contract.waitForDeployment();
    const tx = await contract.mint(1000n);
    const rcpt = await tx.wait();
    const ratioGas = (rcpt!.gasPrice * FHE_GASPRICE_NATIVE_RATIO) / 1_000_000n;
    const effectiveFheGasPrice = ratioGas > MIN_FHE_GASPRICE ? ratioGas : MIN_FHE_GASPRICE;
    const remainingDeposit = await this.fhePayment.getAvailableDepositsETH(await contract.getAddress());
    const consumedFheGas = (initialDeposit - remainingDeposit) / effectiveFheGasPrice;
    expect(consumedFheGas).to.equal(188000n + 600n); // scalarFheAdd(euint64) + trivialEncrypt(euint64)
  });

  it('contract which is not funded can be used by depositor if he whitelisted dApp contract', async function () {
    const contractFactory = await ethers.getContractFactory('EncryptedERC20');
    const contract = await contractFactory.connect(this.signers.alice).deploy('Naraggara', 'NARA', {
      value: ethers.parseEther('0'), // don't fund contract
    });
    await contract.waitForDeployment();
    const tx = await this.fhePayment.depositETH(this.signers.alice, { value: ethers.parseEther('0.001') });
    await tx.wait();
    const initialDeposit = await this.fhePayment.getAvailableDepositsETH(this.signers.alice);
    const txbis = await this.fhePayment.whitelistContract(contract);
    await txbis.wait();
    const tx2 = await contract.mint(1000);
    const rcpt = await tx2.wait();
    const ratioGas = (rcpt!.gasPrice * FHE_GASPRICE_NATIVE_RATIO) / 1_000_000n;
    const effectiveFheGasPrice = ratioGas > MIN_FHE_GASPRICE ? ratioGas : MIN_FHE_GASPRICE;
    const remainingDeposit = await this.fhePayment.getAvailableDepositsETH(this.signers.alice);
    const consumedFheGas = (initialDeposit - remainingDeposit) / effectiveFheGasPrice;
    expect(consumedFheGas).to.equal(188000n + 600n); // scalarFheAdd(euint64) + trivialEncrypt(euint64)
  });

  it('contract which is not funded can be used by depositor if he authorized all contracts', async function () {
    const contractFactory = await ethers.getContractFactory('EncryptedERC20');
    const contract = await contractFactory.connect(this.signers.alice).deploy('Naraggara', 'NARA', {
      value: ethers.parseEther('0'), // don't fund contract
    });
    await contract.waitForDeployment();
    const tx = await this.fhePayment.depositETH(this.signers.alice, { value: ethers.parseEther('0.001') });
    await tx.wait();
    const initialDeposit = await this.fhePayment.getAvailableDepositsETH(this.signers.alice);
    const txbis = await this.fhePayment.authorizeAllContracts(contract);
    await txbis.wait();
    const tx2 = await contract.mint(1000);
    const rcpt = await tx2.wait();
    const ratioGas = (rcpt!.gasPrice * FHE_GASPRICE_NATIVE_RATIO) / 1_000_000n;
    const effectiveFheGasPrice = ratioGas > MIN_FHE_GASPRICE ? ratioGas : MIN_FHE_GASPRICE;
    const remainingDeposit = await this.fhePayment.getAvailableDepositsETH(this.signers.alice);
    const consumedFheGas = (initialDeposit - remainingDeposit) / effectiveFheGasPrice;
    expect(consumedFheGas).to.equal(188000n + 600n); // scalarFheAdd(euint64) + trivialEncrypt(euint64)
  });

  it('contract which is not funded cannot be used by depositor if he did not authorize all contracts nor whitelisted dapp contract', async function () {
    const contractFactory = await ethers.getContractFactory('EncryptedERC20');
    const contract = await contractFactory.connect(this.signers.alice).deploy('Naraggara', 'NARA', {
      value: ethers.parseEther('0'), // don't fund contract
    });
    await contract.waitForDeployment();
    const tx = await this.fhePayment.depositETH(this.signers.alice, { value: ethers.parseEther('0.001') });
    await tx.wait();
    const initialDeposit = await this.fhePayment.getAvailableDepositsETH(this.signers.alice);
    const txbis = await this.fhePayment.removeAuthorizationAllContracts();
    await txbis.wait();
    await expect(contract.mint(1000)).to.be.revertedWithCustomError(this.fhePayment, 'AccountNotEnoughFunded');
    const remainingDeposit = await this.fhePayment.getAvailableDepositsETH(this.signers.alice);
    expect(remainingDeposit).to.equal(initialDeposit);
  });

  it('tx succeeds if under block fheGas limit', async function () {
    const contractFactory = await ethers.getContractFactory('PaymentLimit');
    const contract = await contractFactory.connect(this.signers.alice).deploy();
    await contract.waitForDeployment();
    const initialDeposit = await this.fhePayment.getAvailableDepositsETH(this.signers.alice);
    const txbis = await this.fhePayment.authorizeAllContracts();
    await txbis.wait();
    const tx2 = await contract.underBlockFHEGasLimit();
    const rcpt = await tx2.wait();
    const ratioGas = (rcpt!.gasPrice * FHE_GASPRICE_NATIVE_RATIO) / 1_000_000n;
    const effectiveFheGasPrice = ratioGas > MIN_FHE_GASPRICE ? ratioGas : MIN_FHE_GASPRICE;
    const remainingDeposit = await this.fhePayment.getAvailableDepositsETH(this.signers.alice);
    const consumedFheGas = (initialDeposit - remainingDeposit) / effectiveFheGasPrice;
    expect(consumedFheGas).to.equal(15n * 641000n + 2n * 600n); // 15*FheMul(euint64) + 2*trivialEncrypt(euint64)
  });

  it('tx reverts if above block fheGas limit', async function () {
    const contractFactory = await ethers.getContractFactory('PaymentLimit');
    const contract = await contractFactory.connect(this.signers.alice).deploy();
    await contract.waitForDeployment();
    await expect(contract.aboveBlockFHEGasLimit()).revertedWithCustomError(this.fhePayment, 'FHEGasBlockLimitExceeded');
  });

  it('a smart account becomes spender by calling becomeTransientSpender', async function () {
    const contractFactory = await ethers.getContractFactory('SmartAccount');
    const smartAccount = await contractFactory.connect(this.signers.bob).deploy();
    await smartAccount.waitForDeployment();
    const tx = await this.fhePayment
      .connect(this.signers.bob)
      .depositETH(await smartAccount.getAddress(), { value: ethers.parseEther('0.001') });
    await tx.wait();

    const initialDeposit = await this.fhePayment.getAvailableDepositsETH(await smartAccount.getAddress());

    const contractFactory2 = await ethers.getContractFactory('PaymentLimit');
    const contract = await contractFactory2.connect(this.signers.alice).deploy();
    await contract.waitForDeployment();

    const allowTx = [
      {
        target: await this.fhePayment.getAddress(),
        data: this.fhePayment.interface.encodeFunctionData('authorizeAllContracts'),
        value: 0,
      },
    ];

    const txSmartAllow = await smartAccount.connect(this.signers.bob).executeBatch(allowTx);
    await txSmartAllow.wait();

    const FHETx = [
      {
        target: await this.fhePayment.getAddress(),
        data: this.fhePayment.interface.encodeFunctionData('becomeTransientSpender'),
        value: 0,
      },
      {
        target: await contract.getAddress(),
        data: contract.interface.encodeFunctionData('underBlockFHEGasLimit'),
        value: 0,
      },
    ];

    // Execute the batched transaction
    const txSmartFHE = await smartAccount.connect(this.signers.bob).executeBatch(FHETx);

    const rcpt = await txSmartFHE.wait();
    const ratioGas = (rcpt!.gasPrice * FHE_GASPRICE_NATIVE_RATIO) / 1_000_000n;
    const effectiveFheGasPrice = ratioGas > MIN_FHE_GASPRICE ? ratioGas : MIN_FHE_GASPRICE;
    const remainingDeposit = await this.fhePayment.getAvailableDepositsETH(await smartAccount.getAddress());
    const consumedFheGas = (initialDeposit - remainingDeposit) / effectiveFheGasPrice;
    expect(consumedFheGas).to.equal(15n * 641000n + 2n * 600n); // 15*FheMul(euint64) + 2*trivialEncrypt(euint64)
  });

  it('batching a user-paid tx with a dapp-sponsored tx via a smart account', async function () {
    const contractFactory = await ethers.getContractFactory('SmartAccount');
    const smartAccount = await contractFactory.connect(this.signers.bob).deploy();
    await smartAccount.waitForDeployment();
    const tx = await this.fhePayment
      .connect(this.signers.bob)
      .depositETH(await smartAccount.getAddress(), { value: ethers.parseEther('0.001') });
    await tx.wait();

    const contractFactory2 = await ethers.getContractFactory('PaymentLimit');
    const contract = await contractFactory2.connect(this.signers.alice).deploy(); // non-sponsored dApp
    await contract.waitForDeployment();

    const contract2 = await contractFactory2.connect(this.signers.alice).deploy({ value: ethers.parseEther('0.001') }); // sponsored dApp
    await contract2.waitForDeployment();

    const initialDepositSmartAccount = await this.fhePayment.getAvailableDepositsETH(await smartAccount.getAddress());
    const initialDepositSponsoredDapp = await this.fhePayment.getAvailableDepositsETH(await contract2.getAddress());

    const allowTx = [
      {
        target: await this.fhePayment.getAddress(),
        data: this.fhePayment.interface.encodeFunctionData('authorizeAllContracts'),
        value: 0,
      },
    ];

    const txSmartAllow = await smartAccount.connect(this.signers.bob).executeBatch(allowTx);
    await txSmartAllow.wait();

    const FHETx = [
      {
        target: await this.fhePayment.getAddress(),
        data: this.fhePayment.interface.encodeFunctionData('becomeTransientSpender'),
        value: 0,
      },
      {
        target: await contract.getAddress(),
        data: contract.interface.encodeFunctionData('wayunderBlockFHEGasLimit'),
        value: 0,
      },
      {
        target: await this.fhePayment.getAddress(),
        data: this.fhePayment.interface.encodeFunctionData('stopBeingTransientSpender'),
        value: 0,
      },
      {
        target: await contract2.getAddress(),
        data: contract2.interface.encodeFunctionData('wayunderBlockFHEGasLimit'),
        value: 0,
      },
    ];

    // Execute the batched transaction
    const txSmartFHE = await smartAccount.connect(this.signers.bob).executeBatch(FHETx);

    const rcpt = await txSmartFHE.wait();
    const ratioGas = (rcpt!.gasPrice * FHE_GASPRICE_NATIVE_RATIO) / 1_000_000n;
    const effectiveFheGasPrice = ratioGas > MIN_FHE_GASPRICE ? ratioGas : MIN_FHE_GASPRICE;
    const remainingDepositSmartAccount = await this.fhePayment.getAvailableDepositsETH(await smartAccount.getAddress());
    const remainingDepositSponsoredDapp = await this.fhePayment.getAvailableDepositsETH(await contract2.getAddress());

    const consumedFheGasSmartAccount =
      (initialDepositSmartAccount - remainingDepositSmartAccount) / effectiveFheGasPrice;
    expect(consumedFheGasSmartAccount).to.equal(3n * 641000n + 2n * 600n); // 3*FheMul(euint64) + 2*trivialEncrypt(euint64)
    const consumedFheGasSponsoredDapp =
      (initialDepositSponsoredDapp - remainingDepositSponsoredDapp) / effectiveFheGasPrice;
    expect(consumedFheGasSponsoredDapp).to.equal(3n * 641000n + 2n * 600n); // 3*FheMul(euint64) + 2*trivialEncrypt(euint64)
  });

  it('user can withdraw his unburnt deposited funds', async function () {
    const depositValue = await this.fhePayment.getAvailableDepositsETH(this.signers.alice);
    expect(depositValue).to.be.greaterThan(0);
    const balBobBefore = await ethers.provider.getBalance(this.signers.bob);
    const tx = await this.fhePayment.withdrawETH(depositValue, this.signers.bob);
    await tx.wait();
    const balBobAfter = await ethers.provider.getBalance(this.signers.bob);
    expect(balBobAfter - balBobBefore).to.equal(depositValue);
  });
});
