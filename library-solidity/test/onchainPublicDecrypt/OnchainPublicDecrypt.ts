import { expect } from 'chai';
import dotenv from 'dotenv';
import * as fs from 'fs';
import { ethers } from 'hardhat';

import { awaitCoprocessor, getClearText } from '../coprocessorUtils';
import { createInstances } from '../instance';
import { getSigners, initSigners } from '../signers';

describe('OnchainPublicDecrypt', function () {
  beforeEach(async function () {
    await initSigners(2);
    this.signers = await getSigners();
    this.instances = await createInstances(this.signers);
    const contractFactory = await ethers.getContractFactory('OnchainPublicDecrypt');

    this.contract = await contractFactory.connect(this.signers.alice).deploy();
    await this.contract.waitForDeployment();
    this.contractAddress = await this.contract.getAddress();
    this.instances = await createInstances(this.signers);
  });

  it('One KMS Signer: isPublicDecryptionResultValid (View) and checkSignatures (Non-View)', async function () {
    const tx = await this.contract.requestDecryption();
    const receipt = await tx.wait();
    const { decryptedResult, decryptionSignatures } = await getPublicDecryptionFromReceipt(receipt);

    const decryptionProof = convertSignaturesToDecryptionProof([decryptionSignatures[0]]); /// KMS_SIGNER_ADDRESS_0 is the only signer by default, so next calls should pass
    expect(await this.contract.isPublicDecryptionResultValid(decryptedResult, decryptionProof)).to.be.true;
    const tx2 = await this.contract.callbackDecryption(decryptedResult, decryptionProof);
    await tx2.wait();
    expect(await this.contract.yUint64()).to.equal(42);

    await expect(
      this.contract.isPublicDecryptionResultValid(decryptedResult, decryptionProof.slice(0, 40)),
    ).to.be.revertedWithCustomError(
      { interface: new ethers.Interface(['error DeserializingDecryptionProofFail()']) },
      'DeserializingDecryptionProofFail',
    ); // reverts with selector of DeserializingDecryptionProofFail()
    await expect(
      this.contract.callbackDecryption(decryptedResult, decryptionProof.slice(0, 40)),
    ).to.be.revertedWithCustomError(
      { interface: new ethers.Interface(['error DeserializingDecryptionProofFail()']) },
      'DeserializingDecryptionProofFail',
    );

    const decryptionProof2 = convertSignaturesToDecryptionProof([decryptionSignatures[1]]); ///  KMS_SIGNER_ADDRESS_1 is not a signer, so next calls should not pass
    await expect(
      this.contract.isPublicDecryptionResultValid(decryptedResult, decryptionProof2),
    ).to.be.revertedWithCustomError(
      { interface: new ethers.Interface(['error KMSInvalidSigner(address)']) },
      'KMSInvalidSigner',
    ); // reverts with selector of KMSInvalidSigner(address)
    await expect(this.contract.callbackDecryption(decryptedResult, decryptionProof2)).to.be.revertedWithCustomError(
      { interface: new ethers.Interface(['error KMSInvalidSigner(address)']) },
      'KMSInvalidSigner',
    );

    await expect(this.contract.isPublicDecryptionResultValid(decryptedResult, '0x')).to.be.revertedWithCustomError(
      { interface: new ethers.Interface(['error EmptyDecryptionProof()']) },
      'EmptyDecryptionProof',
    ); // reverts with selector of EmptyDecryptionProof()
    await expect(this.contract.callbackDecryption(decryptedResult, '0x')).to.be.revertedWithCustomError(
      { interface: new ethers.Interface(['error EmptyDecryptionProof()']) },
      'EmptyDecryptionProof',
    );
  });

  it('3 KMS Signers - threshold of 2: isPublicDecryptionResultValid (View) and checkSignatures (Non-View)', async function () {
    const parsedEnv = dotenv.parse(fs.readFileSync('./fhevmTemp/addresses/.env.host'));
    const kmsAdd = parsedEnv.KMS_VERIFIER_CONTRACT_ADDRESS;
    const deployer = new ethers.Wallet(process.env.DEPLOYER_PRIVATE_KEY!).connect(ethers.provider);
    const accounts = await ethers.getSigners();
    const signerAddresses = [accounts[7], accounts[8], accounts[9]].map((s) => s.address);
    const kmsVerifier = await ethers.getContractAt('KMSVerifier', kmsAdd);
    const txNewConfig = await kmsVerifier.connect(deployer).defineNewContext(signerAddresses, 2);
    await txNewConfig.wait();
    expect(await kmsVerifier.getThreshold()).to.equal(2);
    expect(await kmsVerifier.getKmsSigners()).to.deep.equal(signerAddresses); /// Now KMS_SIGNER_ADDRESS_0, KMS_SIGNER_ADDRESS_1 and KMS_SIGNER_ADDRESS_2 are all signers, threshold is 2

    const tx = await this.contract.requestDecryption();
    const receipt = await tx.wait();
    const { decryptedResult, decryptionSignatures } = await getPublicDecryptionFromReceipt(receipt);

    const decryptionProof = convertSignaturesToDecryptionProof([decryptionSignatures[0]]); /// a single signer is not enough here
    await expect(
      this.contract.isPublicDecryptionResultValid(decryptedResult, decryptionProof),
    ).to.be.revertedWithCustomError(
      { interface: new ethers.Interface(['error KMSSignatureThresholdNotReached(uint256)']) },
      'KMSSignatureThresholdNotReached',
    ); // reverts with selector of KMSSignatureThresholdNotReached()
    await expect(this.contract.callbackDecryption(decryptedResult, decryptionProof)).to.be.revertedWithCustomError(
      { interface: new ethers.Interface(['error KMSSignatureThresholdNotReached(uint256)']) },
      'KMSSignatureThresholdNotReached',
    );
    const decryptionProof2 = convertSignaturesToDecryptionProof([decryptionSignatures[1], decryptionSignatures[2]]); /// 2 of 3, should be good
    expect(await this.contract.isPublicDecryptionResultValid(decryptedResult, decryptionProof2)).to.be.true;
    const decryptionProof3 = convertSignaturesToDecryptionProof([decryptionSignatures[0], decryptionSignatures[2]]); /// 2 of 3, should be also good
    expect(await this.contract.isPublicDecryptionResultValid(decryptedResult, decryptionProof3)).to.be.true;
    const decryptionProof4 = convertSignaturesToDecryptionProof([
      decryptionSignatures[0],
      decryptionSignatures[1],
      decryptionSignatures[2],
    ]); /// 3 of 3, more than enough
    expect(await this.contract.isPublicDecryptionResultValid(decryptedResult, decryptionProof4)).to.be.true;
    const decryptionProof5 = convertSignaturesToDecryptionProof([decryptionSignatures[1], decryptionSignatures[1]]); /// 1 duplicate, here the view function should return false, but the checkSignatures should still revert!
    expect(await this.contract.isPublicDecryptionResultValid(decryptedResult, decryptionProof5)).to.be.false;
    await expect(this.contract.callbackDecryption(decryptedResult, decryptionProof5)).to.be.revertedWithCustomError(
      { interface: new ethers.Interface(['error InvalidKMSSignatures()']) },
      'InvalidKMSSignatures',
    );

    const decryptionProof6 = convertSignaturesToDecryptionProof([decryptionSignatures[0], decryptionSignatures[2]]); /// 2 of 3, should be also good, now we do the tx
    expect(await this.contract.isPublicDecryptionResultValid(decryptedResult, decryptionProof6)).to.be.true;
    await this.contract.callbackDecryption(decryptedResult, decryptionProof6);
    expect(await this.contract.yUint64()).to.equal(42);

    const txResetConfig = await kmsVerifier.connect(deployer).defineNewContext([signerAddresses[0]], 1);
    await txResetConfig.wait();
    expect(await kmsVerifier.getThreshold()).to.equal(1);
    expect(await kmsVerifier.getKmsSigners()).to.deep.equal([signerAddresses[0]]);
  });
});

async function getPublicDecryptionFromReceipt(
  receipt: any,
): Promise<{ handles: string[]; decryptedResult: string; decryptionSignatures: string[] }> {
  /// this will scan the tx receipt for all handles which have been made publicly decryptable and attempt to decrypt + sign them
  /// decryptionSignatures always contains the 4 signatures from all the KMS_SIGNER_ADDRESS_i
  await awaitCoprocessor();
  const aclIface = new ethers.Interface(['event AllowedForDecryption(address indexed caller, bytes32[] handlesList)']);
  const topic = aclIface.getEvent('AllowedForDecryption')!.topicHash;
  const log = receipt!.logs.find((l: any) => l.topics[0] === topic);
  const parsed = aclIface.parseLog({ data: log!.data, topics: [...log!.topics] })!;
  const handles: string[] = parsed.args.handlesList;
  const clearTexts = await Promise.all(handles.map((h) => getClearText(h)));
  const types = handles.map(() => 'uint256');
  const decryptedResult = ethers.AbiCoder.defaultAbiCoder().encode(types, clearTexts.map(BigInt));

  const accounts = await ethers.getSigners();
  const signers = [accounts[7], accounts[8], accounts[9], accounts[10]]; /// those are the KMS_SIGNER_ADDRESS_{i} from the default `.env` (with i between 0 and 3)
  const domain = {
    name: 'Decryption',
    version: '1',
    chainId: process.env['CHAIN_ID_GATEWAY']!,
    verifyingContract: process.env['DECRYPTION_ADDRESS']!,
  };
  const typesEIP712 = {
    PublicDecryptVerification: [
      { name: 'ctHandles', type: 'bytes32[]' },
      { name: 'decryptedResult', type: 'bytes' },
      { name: 'extraData', type: 'bytes' },
    ],
  };
  const message = {
    ctHandles: handles,
    decryptedResult,
    extraData: '0x00',
  };
  const decryptionSignatures = await Promise.all(signers.map((s) => s.signTypedData(domain, typesEIP712, message)));

  return { handles, decryptedResult, decryptionSignatures };
}

function convertSignaturesToDecryptionProof(decryptionSignatures: string[]): string {
  const numSigs = ethers.toBeHex(decryptionSignatures.length, 1);
  const sigs = decryptionSignatures.map((s) => s.slice(2)).join('');
  return numSigs + sigs + '00';
}
