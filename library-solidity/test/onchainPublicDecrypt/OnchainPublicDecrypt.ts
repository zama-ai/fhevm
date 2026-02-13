import { expect } from 'chai';
import dotenv from 'dotenv';
import * as fs from 'fs';
import { ethers } from 'hardhat';

import { awaitCoprocessor, getClearText } from '../coprocessorUtils';
import { createInstances } from '../instance';
import { getSigners, initSigners } from '../signers';

describe.only('OnchainPublicDecrypt', function () {
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

//   it('original owner adds one signer, then adds two more signers, then removes one signer', async function () {
//     if (process.env.HARDHAT_PARALLEL !== '1') {
//       // to avoid messing up other tests if used on the real node, in parallel testing

//       const origKMSAdd = dotenv.parse(fs.readFileSync('addresses/.env.host')).KMS_VERIFIER_CONTRACT_ADDRESS;
//       const deployer = new ethers.Wallet(process.env.DEPLOYER_PRIVATE_KEY!).connect(ethers.provider);
//       const kmsVerifier = await this.kmsFactory.attach(origKMSAdd);
//       expect(await kmsVerifier.getVersion()).to.equal('KMSVerifier v0.1.0');

//       const addressSigner = process.env['KMS_SIGNER_ADDRESS_1']!;
//       let setSigners = await kmsVerifier.getKmsSigners();
//       setSigners = [...setSigners, addressSigner];
//       const tx1 = await kmsVerifier.connect(deployer).defineNewContext(setSigners, 1);
//       await tx1.wait();

//       expect((await kmsVerifier.getKmsSigners()).length).to.equal(2); // one signer has been added

//       const contractFactory = await ethers.getContractFactory('TestAsyncDecrypt');
//       const contract = (await contractFactory.connect(this.signers.alice).deploy()) as TestAsyncDecrypt;
//       const tx2 = await contract.requestBool();
//       await tx2.wait();
//       await awaitAllDecryptionResults();
//       expect(await contract.yBool()).to.equal(true); // in this case, one signature still suffices to pass the decrypt (threshold is still 1)

//       setSigners = [...setSigners, addressSigner];
//       await expect(kmsVerifier.connect(deployer).defineNewContext(setSigners, 1)).to.revertedWithCustomError(
//         kmsVerifier,
//         'KMSAlreadySigner',
//       ); // cannot add duplicated signer
//       expect((await kmsVerifier.getKmsSigners()).length).to.equal(2);

//       const kmsSigner2Address = process.env['KMS_SIGNER_ADDRESS_2']!;
//       const kmsSigner3Address = process.env['KMS_SIGNER_ADDRESS_3']!;
//       let setSigners2 = await kmsVerifier.getKmsSigners();
//       setSigners2 = [...setSigners2, kmsSigner2Address, kmsSigner3Address];
//       const tx3 = await kmsVerifier.connect(deployer).defineNewContext(setSigners2, 1);
//       await tx3.wait();
//       expect((await kmsVerifier.getKmsSigners()).length).to.equal(4); // 3rd and 4th signer has been added successfully

//       const tx4 = await kmsVerifier.connect(deployer).setThreshold(2n);
//       await tx4.wait();
//       expect(await kmsVerifier.getThreshold()).to.equal(2);

//       const tx5 = await contract.requestUint16();
//       await tx5.wait();

//       await expect(awaitAllDecryptionResults())
//         .to.revertedWithCustomError(kmsVerifier, 'KMSSignatureThresholdNotReached')
//         .withArgs(1n); // should revert because now we are below the threshold! (we receive only 1 signature but threshold is 2)

//       process.env.NUM_KMS_NODES = '4';

//       const tx6 = await contract.requestUint8();
//       await tx6.wait();
//       await awaitAllDecryptionResults();
//       expect(await contract.yUint8()).to.equal(42); // even with more than 2 signatures decryption should still succeed

//       process.env.NUM_KMS_NODES = '2';
//       process.env.KMS_SIGNER_ADDRESS_1 = process.env.KMS_SIGNER_ADDRESS_0;
//       const tx8 = await contract.requestUint16();
//       await tx8.wait();
//       await expect(awaitAllDecryptionResults()).to.revertedWithCustomError(contract, 'InvalidKMSSignatures'); // cannot use duplicated signatures if threshold is 2
//       expect(await contract.yUint16()).to.equal(0);

//       process.env.NUM_KMS_NODES = '1';
//       let setSigners3 = [...(await kmsVerifier.getKmsSigners())];
//       setSigners3.pop();

//       const tx9 = await kmsVerifier.connect(deployer).defineNewContext(setSigners3, 1);
//       await tx9.wait();
//       expect(await kmsVerifier.getThreshold()).to.equal(1);

//       const tx10 = await contract.requestUint16();
//       await tx10.wait();
//       await awaitAllDecryptionResults();
//       expect(await contract.yUint16()).to.equal(16); // after removing one of the 4 signers, one signature is enough for decryption
//     }
//   });

//   it('cannot add/remove signers if not the owner', async function () {
//     const origKMSAdd = dotenv.parse(fs.readFileSync('addresses/.env.host')).KMS_VERIFIER_CONTRACT_ADDRESS;
//     const kmsVerifier = await this.kmsFactory.attach(origKMSAdd);
//     let setSigners = await kmsVerifier.getKmsSigners();
//     const randomAccount = this.signers.carol;
//     setSigners = [...setSigners, randomAccount];
//     await expect(kmsVerifier.connect(randomAccount).defineNewContext(setSigners, 2)).to.be.revertedWithCustomError(
//       kmsVerifier,
//       'NotHostOwner',
//     );
//   });
// });
