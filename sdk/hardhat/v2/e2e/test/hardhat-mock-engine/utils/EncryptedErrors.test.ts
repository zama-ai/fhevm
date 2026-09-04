import { FhevmType } from '@fhevm/hardhat-plugin';
import { expect } from 'chai';
import * as hre from 'hardhat';

import type { TestEncryptedErrors } from '../../../typechain-types';
import { type Signers, getSigners, initSigners } from '../signers';
import { deployEncryptedErrors } from './EncryptedErrors.fixture';

describe('EncryptedErrors', function () {
  const NO_ERROR_CODE = 0n;

  let signers: Signers;
  let encryptedErrors: TestEncryptedErrors;
  let encryptedErrorsAddress: string;

  before(async function () {
    await initSigners();
    signers = await getSigners();
  });

  beforeEach(async function () {
    const numberErrors = 3;
    const contract = await deployEncryptedErrors(signers.alice, numberErrors);
    encryptedErrorsAddress = await contract.getAddress();
    encryptedErrors = contract;
  });

  it('post-deployment', async function () {
    expect(await encryptedErrors.errorGetCounter()).to.be.eq(BigInt('0'));
    expect(await encryptedErrors.errorGetNumCodesDefined()).to.be.eq(BigInt('3'));

    for (let i = 0; i < 3; i++) {
      const handle = await encryptedErrors.connect(signers.alice).errorGetCodeDefinition(i);
      expect(
        await hre.fhevm.userDecryptEuint(FhevmType.euint8, handle, encryptedErrorsAddress, signers.alice),
      ).to.be.eq(BigInt(i));
    }
  });

  it('errorDefineIf --> true', async function () {
    // True --> errorId=0 has errorCode=2
    const condition = true;
    const targetErrorCode = 2;

    const input = hre.fhevm.createEncryptedInput(encryptedErrorsAddress, signers.alice.address);
    const encryptedData = await input.addBool(condition).encrypt();

    await encryptedErrors
      .connect(signers.alice)
      .errorDefineIf(encryptedData.handles[0], encryptedData.inputProof, targetErrorCode);

    const handle = await encryptedErrors.connect(signers.alice).errorGetCodeEmitted(0);
    expect(await hre.fhevm.userDecryptEuint(FhevmType.euint8, handle, encryptedErrorsAddress, signers.alice)).to.be.eq(
      BigInt(targetErrorCode),
    );
    expect(await encryptedErrors.errorGetCounter()).to.be.eq(1n);
  });

  it('errorDefineIf --> false', async function () {
    // False --> errorId=1 has errorCode=0
    const condition = false;
    const targetErrorCode = 2;

    const input = hre.fhevm.createEncryptedInput(encryptedErrorsAddress, signers.alice.address);
    const encryptedData = await input.addBool(condition).encrypt();

    await encryptedErrors
      .connect(signers.alice)
      .errorDefineIf(encryptedData.handles[0], encryptedData.inputProof, targetErrorCode);

    const handle = await encryptedErrors.connect(signers.alice).errorGetCodeEmitted(0);
    expect(await hre.fhevm.userDecryptEuint(FhevmType.euint8, handle, encryptedErrorsAddress, signers.alice)).to.be.eq(
      NO_ERROR_CODE,
    );
    expect(await encryptedErrors.errorGetCounter()).to.be.eq(BigInt('1'));
  });

  it('errorDefineIfNot --> true', async function () {
    // True --> errorId=0 has errorCode=0
    const condition = true;
    const targetErrorCode = 2;

    const input = hre.fhevm.createEncryptedInput(encryptedErrorsAddress, signers.alice.address);
    const encryptedData = await input.addBool(condition).encrypt();

    await encryptedErrors
      .connect(signers.alice)
      .errorDefineIfNot(encryptedData.handles[0], encryptedData.inputProof, targetErrorCode);

    const handle = await encryptedErrors.connect(signers.alice).errorGetCodeEmitted(0);
    expect(await hre.fhevm.userDecryptEuint(FhevmType.euint8, handle, encryptedErrorsAddress, signers.alice)).to.be.eq(
      NO_ERROR_CODE,
    );
    expect(await encryptedErrors.errorGetCounter()).to.be.eq(BigInt('1'));
  });

  it('errorDefineIf --> false', async function () {
    // False --> errorId=1 has errorCode=2
    const condition = false;
    const targetErrorCode = 2;

    const input = hre.fhevm.createEncryptedInput(encryptedErrorsAddress, signers.alice.address);
    const encryptedData = await input.addBool(condition).encrypt();

    await encryptedErrors
      .connect(signers.alice)
      .errorDefineIfNot(encryptedData.handles[0], encryptedData.inputProof, targetErrorCode);

    const handle = await encryptedErrors.connect(signers.alice).errorGetCodeEmitted(0);
    expect(await hre.fhevm.userDecryptEuint(FhevmType.euint8, handle, encryptedErrorsAddress, signers.alice)).to.be.eq(
      BigInt(targetErrorCode),
    );
    expect(await encryptedErrors.errorGetCounter()).to.be.eq(1n);
  });

  it('errorChangeIf --> true --> change error code', async function () {
    // True --> change errorCode
    const condition = true;
    const errorCode = 1;
    const targetErrorCode = 2;

    const input = hre.fhevm.createEncryptedInput(encryptedErrorsAddress, signers.alice.address);
    const encryptedData = await input.addBool(condition).add8(errorCode).encrypt();

    await encryptedErrors
      .connect(signers.alice)
      .errorChangeIf(encryptedData.handles[0], encryptedData.handles[1], encryptedData.inputProof, targetErrorCode);

    const handle = await encryptedErrors.connect(signers.alice).errorGetCodeEmitted(0);
    expect(await hre.fhevm.userDecryptEuint(FhevmType.euint8, handle, encryptedErrorsAddress, signers.alice)).to.be.eq(
      BigInt(targetErrorCode),
    );
    expect(await encryptedErrors.errorGetCounter()).to.be.eq(1n);
  });

  it('errorChangeIf --> false --> no change for error code', async function () {
    // False --> no change in errorCode
    const condition = false;
    const errorCode = 1;
    const targetErrorCode = 2;

    const input = hre.fhevm.createEncryptedInput(encryptedErrorsAddress, signers.alice.address);
    const encryptedData = await input.addBool(condition).add8(errorCode).encrypt();

    await encryptedErrors
      .connect(signers.alice)
      .errorChangeIf(encryptedData.handles[0], encryptedData.handles[1], encryptedData.inputProof, targetErrorCode);

    const handle = await encryptedErrors.connect(signers.alice).errorGetCodeEmitted(0);
    expect(await hre.fhevm.userDecryptEuint(FhevmType.euint8, handle, encryptedErrorsAddress, signers.alice)).to.be.eq(
      BigInt(errorCode),
    );
    expect(await encryptedErrors.errorGetCounter()).to.be.eq(1n);
  });

  it('errorChangeIfNot --> true --> no change for error code', async function () {
    // True --> no change errorCode
    const condition = true;
    const errorCode = 1;
    const targetErrorCode = 2;

    const input = hre.fhevm.createEncryptedInput(encryptedErrorsAddress, signers.alice.address);
    const encryptedData = await input.addBool(condition).add8(errorCode).encrypt();

    await encryptedErrors
      .connect(signers.alice)
      .errorChangeIfNot(encryptedData.handles[0], encryptedData.handles[1], encryptedData.inputProof, targetErrorCode);

    const handle = await encryptedErrors.connect(signers.alice).errorGetCodeEmitted(0);
    expect(await hre.fhevm.userDecryptEuint(FhevmType.euint8, handle, encryptedErrorsAddress, signers.alice)).to.be.eq(
      BigInt(errorCode),
    );
    expect(await encryptedErrors.errorGetCounter()).to.be.eq(1n);
  });

  it('errorChangeIfNot --> false --> change error code', async function () {
    // False --> change in errorCode
    const condition = false;
    const errorCode = 1;
    const targetErrorCode = 2;

    const input = hre.fhevm.createEncryptedInput(encryptedErrorsAddress, signers.alice.address);
    const encryptedData = await input.addBool(condition).add8(errorCode).encrypt();

    await encryptedErrors
      .connect(signers.alice)
      .errorChangeIfNot(encryptedData.handles[0], encryptedData.handles[1], encryptedData.inputProof, targetErrorCode);

    const handle = await encryptedErrors.connect(signers.alice).errorGetCodeEmitted(0);

    expect(await hre.fhevm.userDecryptEuint(FhevmType.euint8, handle, encryptedErrorsAddress, signers.alice)).to.be.eq(
      BigInt(targetErrorCode),
    );
    expect(await encryptedErrors.errorGetCounter()).to.be.eq(BigInt('1'));
  });

  it('cannot deploy if totalNumberErrorCodes_ == 0', async function () {
    const numberErrors = 0;
    const contractFactory = await hre.ethers.getContractFactory('TestEncryptedErrors');
    await expect(contractFactory.connect(signers.alice).deploy(numberErrors)).to.be.revertedWithCustomError(
      encryptedErrors,
      'TotalNumberErrorCodesEqualToZero',
    );
  });

  it('cannot define errors if indexCode is greater or equal than totalNumberErrorCodes', async function () {
    const condition = true;
    const targetErrorCode = (await encryptedErrors.errorGetNumCodesDefined()) + 1n;

    const input = hre.fhevm.createEncryptedInput(encryptedErrorsAddress, signers.alice.address);
    const encryptedData = await input.addBool(condition).encrypt();

    await expect(
      encryptedErrors
        .connect(signers.alice)
        .errorDefineIf(encryptedData.handles[0], encryptedData.inputProof, targetErrorCode),
    ).to.be.revertedWithCustomError(encryptedErrors, 'ErrorIndexInvalid');

    await expect(
      encryptedErrors
        .connect(signers.alice)
        .errorDefineIfNot(encryptedData.handles[0], encryptedData.inputProof, targetErrorCode),
    ).to.be.revertedWithCustomError(encryptedErrors, 'ErrorIndexInvalid');
  });

  it('cannot define errors if indexCode is 0 or equal', async function () {
    const condition = true;
    const targetErrorCode = 0;

    const input = hre.fhevm.createEncryptedInput(encryptedErrorsAddress, signers.alice.address);
    const encryptedData = await input.addBool(condition).encrypt();

    await expect(
      encryptedErrors
        .connect(signers.alice)
        .errorDefineIf(encryptedData.handles[0], encryptedData.inputProof, targetErrorCode),
    ).to.be.revertedWithCustomError(encryptedErrors, 'ErrorIndexIsNull');

    await expect(
      encryptedErrors
        .connect(signers.alice)
        .errorDefineIfNot(encryptedData.handles[0], encryptedData.inputProof, targetErrorCode),
    ).to.be.revertedWithCustomError(encryptedErrors, 'ErrorIndexIsNull');
  });

  it('cannot change errors if indexCode is greater or equal than totalNumberErrorCodes', async function () {
    const condition = true;
    const errorCode = 1;
    const targetErrorCode = (await encryptedErrors.errorGetNumCodesDefined()) + 1n;

    const input = hre.fhevm.createEncryptedInput(encryptedErrorsAddress, signers.alice.address);
    const encryptedData = await input.addBool(condition).add8(errorCode).encrypt();

    await expect(
      encryptedErrors
        .connect(signers.alice)
        .errorChangeIf(encryptedData.handles[0], encryptedData.handles[1], encryptedData.inputProof, targetErrorCode),
    ).to.be.revertedWithCustomError(encryptedErrors, 'ErrorIndexInvalid');

    await expect(
      encryptedErrors
        .connect(signers.alice)
        .errorChangeIfNot(
          encryptedData.handles[0],
          encryptedData.handles[1],
          encryptedData.inputProof,
          targetErrorCode,
        ),
    ).to.be.revertedWithCustomError(encryptedErrors, 'ErrorIndexInvalid');
  });

  it('cannot call _errorGetCodeDefinition if indexCode is greater or equal than totalNumberErrorCodes', async function () {
    const indexCodeDefinition = await encryptedErrors.errorGetNumCodesDefined();

    await expect(
      encryptedErrors.connect(signers.alice).errorGetCodeDefinition(indexCodeDefinition),
    ).to.be.revertedWithCustomError(encryptedErrors, 'ErrorIndexInvalid');
  });

  it('cannot call _errorGetCodeEmitted if errorId is greater than errorCounter', async function () {
    const errorCounter = await encryptedErrors.errorGetCounter();

    await expect(
      encryptedErrors.connect(signers.alice).errorGetCodeEmitted(errorCounter),
    ).to.be.revertedWithCustomError(encryptedErrors, 'ErrorIndexInvalid');
  });
});
