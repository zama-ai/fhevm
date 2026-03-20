import { expect } from 'chai';
import dotenv from 'dotenv';
import * as fs from 'fs';
import { ethers } from 'hardhat';

import { awaitCoprocessor, getClearText } from '../coprocessorUtils';
import { createInstances } from '../instance';
import { getSigners, initSigners } from '../signers';

const DEFAULT_MNEMONIC = 'adapt mosquito move limb mobile illegal tree voyage juice mosquito burger raise father hope layer';

/**
 * Derives the private key for a hardhat account at the given index.
 */
function getPrivateKeyForAccount(index: number): string {
  const hdNode = ethers.HDNodeWallet.fromMnemonic(
    ethers.Mnemonic.fromPhrase(DEFAULT_MNEMONIC),
    "m/44'/60'/0'/0",
  );
  return hdNode.deriveChild(index).privateKey;
}

/**
 * Builds MpcNode structs from hardhat account indices.
 * The verificationKey is the uncompressed secp256k1 public key (64 bytes, no 04 prefix),
 * so keccak256(verificationKey) produces the correct Ethereum address.
 */
function buildMpcNodes(accountIndices: number[]) {
  return accountIndices.map((acctIdx, i) => {
    const pk = getPrivateKeyForAccount(acctIdx);
    const signingKey = new ethers.SigningKey(pk);
    // signingKey.publicKey is 0x04 + X(32) + Y(32). Strip the 04 prefix.
    const verificationKey = '0x' + signingKey.publicKey.slice(4);
    return {
      mpcIdentity: `node-${i}`,
      partyId: i,
      verificationKey,
      externalUrl: 'https://example.com',
      caCert: '0xdeadbeef',
      publicStorageUrl: 'https://storage.example.com',
      publicStoragePrefix: 'prefix',
      extraVerificationKeys: [],
    };
  });
}

/**
 * Defines a new context+epoch via the epoch lifecycle and activates it
 * by having all signers confirm context creation and epoch result.
 */
async function defineAndActivateContext(
  kmsVerifier: any,
  deployer: any,
  signerAccounts: any[],
  accountIndices: number[],
  threshold: number,
) {
  // Compute next IDs from current active state (counters are sequential)
  const [currentContextId, currentEpochId] = await kmsVerifier.getCurrentKmsContext();
  const newContextId = currentContextId + 1n;
  const newEpochId = currentEpochId + 1n;

  const mpcNodes = buildMpcNodes(accountIndices);

  const tx = await kmsVerifier.connect(deployer).defineNewContextAndEpoch(mpcNodes, threshold, '1.0.0', []);
  await tx.wait();

  // EIP-712 domain for KMSVerifier native signing
  const chainId = (await ethers.provider.getNetwork()).chainId;
  const kmsVerifierAddress = await kmsVerifier.getAddress();
  const domain = {
    name: 'KMSVerifier',
    version: '1',
    chainId,
    verifyingContract: kmsVerifierAddress,
  };

  // --- Confirm context creation ---
  const contextCreationTypes = {
    ContextCreationConfirmation: [{ name: 'contextId', type: 'uint256' }],
  };
  for (const signer of signerAccounts) {
    const sig = await signer.signTypedData(domain, contextCreationTypes, { contextId: newContextId });
    await kmsVerifier.confirmContextCreation(newContextId, sig);
  }

  // --- Confirm epoch result ---
  const extraData = ethers.concat([
    '0x01',
    ethers.zeroPadValue(ethers.toBeHex(newContextId), 32),
    ethers.zeroPadValue(ethers.toBeHex(newEpochId), 32),
  ]);
  const keyDigests = [{ keyType: 1, digest: '0xaabbccdd' }];
  const keygenTypes = {
    KeygenVerification: [
      { name: 'prepKeygenId', type: 'uint256' },
      { name: 'keyId', type: 'uint256' },
      { name: 'keyDigests', type: 'KeyDigest[]' },
      { name: 'extraData', type: 'bytes' },
    ],
    KeyDigest: [
      { name: 'keyType', type: 'uint8' },
      { name: 'digest', type: 'bytes' },
    ],
  };
  const keygenMessage = { prepKeygenId: 1, keyId: 2, keyDigests, extraData };
  for (const signer of signerAccounts) {
    const sig = await signer.signTypedData(domain, keygenTypes, keygenMessage);
    await kmsVerifier.confirmEpochResult(newEpochId, 1, 2, keyDigests, extraData, sig);
  }
}

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
    const signerAccounts = [accounts[7], accounts[8], accounts[9]];
    const signerAddresses = signerAccounts.map((s) => s.address);
    const kmsVerifier = await ethers.getContractAt('KMSVerifier', kmsAdd);

    // Activate a new context with 3 signers, threshold 2 (full epoch lifecycle)
    await defineAndActivateContext(kmsVerifier, deployer, signerAccounts, [7, 8, 9], 2);
    expect(await kmsVerifier.getThreshold()).to.equal(2);
    expect(await kmsVerifier.getKmsSigners()).to.deep.equal(signerAddresses);

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

    // Reset to 1 signer with threshold 1 (full epoch lifecycle)
    await defineAndActivateContext(kmsVerifier, deployer, [accounts[7]], [7], 1);
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
