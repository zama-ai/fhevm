import { expect } from 'chai';
import dotenv from 'dotenv';
import * as fs from 'fs';
import { ethers } from 'hardhat';
import { awaitCoprocessor, coprocDbReadCleartext } from '../coprocessorUtils';
import { getSigners, initSigners } from '../signers';
import { getDecryptionEip712Domain, HOST_ADDRESSES_ENV_PATH } from '../utils';
import { getRequiredEnvVar } from '../../tasks/utils/loadVariables';
import { activateDefinedKmsContext, defineNewKmsContext } from './protocolConfigEip712';
import { ProtocolConfig__factory } from '../../typechain-types';
import type {
  KmsNodeParamsStruct,
  KmsThresholdsStruct,
} from '../../typechain-types/fhevmTemp/contracts/ProtocolConfig';
import type { OnchainPublicDecrypt, ProtocolConfig } from '../../typechain-types';
import type { Signers } from '../signers';
import type { ContractTransactionReceipt, Signer } from 'ethers';

describe('OnchainPublicDecrypt', function () {
  let contract: OnchainPublicDecrypt;
  let signers: Signers;

  beforeEach(async function () {
    await initSigners(2);
    signers = await getSigners();
    const contractFactory = await ethers.getContractFactory('OnchainPublicDecrypt');

    contract = await contractFactory.connect(signers.alice).deploy();
    await contract.waitForDeployment();
  });

  it('One KMS Signer: isPublicDecryptionResultValid (View) and checkSignatures (Non-View)', async function () {
    const tx = await contract.requestDecryption();
    const receipt = await tx.wait();
    if (receipt === null) {
      throw new Error('requestDecryption transaction receipt is null');
    }
    const { decryptedResult, decryptionSignatures } = await getPublicDecryptionFromReceipt(receipt);

    const decryptionProof = convertSignaturesToDecryptionProof([decryptionSignatures[0]]); /// KMS_SIGNER_ADDRESS_0 is the only signer by default, so next calls should pass
    expect(await contract.isPublicDecryptionResultValid(decryptedResult, decryptionProof)).to.equal(true);
    const tx2 = await contract.callbackDecryption(decryptedResult, decryptionProof);
    await tx2.wait();
    expect(await contract.yUint64()).to.equal(42);

    await expect(
      contract.isPublicDecryptionResultValid(decryptedResult, decryptionProof.slice(0, 40)),
    ).to.be.revertedWithCustomError(
      { interface: new ethers.Interface(['error DeserializingDecryptionProofFail()']) },
      'DeserializingDecryptionProofFail',
    ); // reverts with selector of DeserializingDecryptionProofFail()
    await expect(
      contract.callbackDecryption(decryptedResult, decryptionProof.slice(0, 40)),
    ).to.be.revertedWithCustomError(
      { interface: new ethers.Interface(['error DeserializingDecryptionProofFail()']) },
      'DeserializingDecryptionProofFail',
    );

    const decryptionProof2 = convertSignaturesToDecryptionProof([decryptionSignatures[1]]); ///  KMS_SIGNER_ADDRESS_1 is not a signer, so next calls should not pass
    await expect(
      contract.isPublicDecryptionResultValid(decryptedResult, decryptionProof2),
    ).to.be.revertedWithCustomError(
      { interface: new ethers.Interface(['error KMSInvalidSigner(address)']) },
      'KMSInvalidSigner',
    ); // reverts with selector of KMSInvalidSigner(address)
    await expect(contract.callbackDecryption(decryptedResult, decryptionProof2)).to.be.revertedWithCustomError(
      { interface: new ethers.Interface(['error KMSInvalidSigner(address)']) },
      'KMSInvalidSigner',
    );

    await expect(contract.isPublicDecryptionResultValid(decryptedResult, '0x')).to.be.revertedWithCustomError(
      { interface: new ethers.Interface(['error EmptyDecryptionProof()']) },
      'EmptyDecryptionProof',
    ); // reverts with selector of EmptyDecryptionProof()
    await expect(contract.callbackDecryption(decryptedResult, '0x')).to.be.revertedWithCustomError(
      { interface: new ethers.Interface(['error EmptyDecryptionProof()']) },
      'EmptyDecryptionProof',
    );
  });

  it('3 KMS Signers - threshold of 2: isPublicDecryptionResultValid (View) and checkSignatures (Non-View)', async function () {
    const parsedEnv = dotenv.parse(fs.readFileSync(HOST_ADDRESSES_ENV_PATH));
    const kmsAdd = parsedEnv.KMS_VERIFIER_CONTRACT_ADDRESS;
    const protocolConfigAdd = parsedEnv.PROTOCOL_CONFIG_CONTRACT_ADDRESS;
    const deployer = new ethers.Wallet(getRequiredEnvVar('DEPLOYER_PRIVATE_KEY')).connect(ethers.provider);
    const accounts = await ethers.getSigners();
    const newSigners = [accounts[7], accounts[8], accounts[9]];
    const signerAddresses = newSigners.map((s) => s.address);
    const kmsVerifier = await ethers.getContractAt('KMSVerifier', kmsAdd);
    // Same default runner as `ethers.getContractAt`: the first hardhat signer.
    const protocolConfig = ProtocolConfig__factory.connect(protocolConfigAdd, accounts[0]);
    const newTxSenders = [accounts[2], accounts[3], accounts[4]];
    const newNodes: KmsNodeParamsStruct[] = signerAddresses.map((address, index) => ({
      txSenderAddress: newTxSenders[index].address,
      signerAddress: address,
      ipAddress: `127.0.0.${index + 1}`,
      storageUrl: `https://kms-${index + 1}.example.com`,
      partyId: index,
      mpcIdentity: `127.0.0.${index + 1}`,
      caCert: '0x',
      storagePrefix: '',
    }));
    const newThresholds = { publicDecryption: 2, userDecryption: 2, kmsGen: 2, mpc: 2 };
    // accounts[7] signs for the default context too, so its confirmation also covers the previous side's
    // n - t target (one node, so the target is 1).
    await activateNewKmsContext(protocolConfig, deployer, newNodes, newThresholds, newSigners);
    expect(await protocolConfig.getPublicDecryptionThreshold()).to.equal(2);
    expect(await kmsVerifier.getKmsSigners()).to.deep.equal(signerAddresses); /// Now KMS_SIGNER_ADDRESS_0, KMS_SIGNER_ADDRESS_1 and KMS_SIGNER_ADDRESS_2 are all signers, threshold is 2

    const tx = await contract.requestDecryption();
    const receipt = await tx.wait();
    if (receipt === null) {
      throw new Error('requestDecryption transaction receipt is null');
    }

    const { decryptedResult, decryptionSignatures } = await getPublicDecryptionFromReceipt(receipt);

    const decryptionProof = convertSignaturesToDecryptionProof([decryptionSignatures[0]]); /// a single signer is not enough here
    await expect(
      contract.isPublicDecryptionResultValid(decryptedResult, decryptionProof),
    ).to.be.revertedWithCustomError(
      { interface: new ethers.Interface(['error KMSSignatureThresholdNotReached(uint256)']) },
      'KMSSignatureThresholdNotReached',
    ); // reverts with selector of KMSSignatureThresholdNotReached()
    await expect(contract.callbackDecryption(decryptedResult, decryptionProof)).to.be.revertedWithCustomError(
      { interface: new ethers.Interface(['error KMSSignatureThresholdNotReached(uint256)']) },
      'KMSSignatureThresholdNotReached',
    );
    const decryptionProof2 = convertSignaturesToDecryptionProof([decryptionSignatures[1], decryptionSignatures[2]]); /// 2 of 3, should be good
    expect(await contract.isPublicDecryptionResultValid(decryptedResult, decryptionProof2)).to.equal(true);
    const decryptionProof3 = convertSignaturesToDecryptionProof([decryptionSignatures[0], decryptionSignatures[2]]); /// 2 of 3, should be also good
    expect(await contract.isPublicDecryptionResultValid(decryptedResult, decryptionProof3)).to.equal(true);
    const decryptionProof4 = convertSignaturesToDecryptionProof([
      decryptionSignatures[0],
      decryptionSignatures[1],
      decryptionSignatures[2],
    ]); /// 3 of 3, more than enough
    expect(await contract.isPublicDecryptionResultValid(decryptedResult, decryptionProof4)).to.equal(true);
    const decryptionProof5 = convertSignaturesToDecryptionProof([decryptionSignatures[1], decryptionSignatures[1]]); /// 1 duplicate, here the view function should return false, but the checkSignatures should still revert!
    expect(await contract.isPublicDecryptionResultValid(decryptedResult, decryptionProof5)).to.equal(false);
    await expect(contract.callbackDecryption(decryptedResult, decryptionProof5)).to.be.revertedWithCustomError(
      { interface: new ethers.Interface(['error InvalidKMSSignatures()']) },
      'InvalidKMSSignatures',
    );

    const decryptionProof6 = convertSignaturesToDecryptionProof([decryptionSignatures[0], decryptionSignatures[2]]); /// 2 of 3, should be also good, now we do the tx
    expect(await contract.isPublicDecryptionResultValid(decryptedResult, decryptionProof6)).to.equal(true);
    await contract.callbackDecryption(decryptedResult, decryptionProof6);
    expect(await contract.yUint64()).to.equal(42);

    const resetNodes = [
      {
        txSenderAddress: accounts[2].address,
        signerAddress: signerAddresses[0],
        ipAddress: '127.0.0.1',
        storageUrl: 'https://kms-1.example.com',
        partyId: 0,
        mpcIdentity: '127.0.0.1',
        caCert: '0x',
        storagePrefix: '',
      },
    ];
    const resetThresholds = { publicDecryption: 1, userDecryption: 1, kmsGen: 1, mpc: 1 };
    // accounts[7] alone completes the creation quorum. It is the only new signer and its
    // confirmation also covers the previous side's n - t = 1 target (3 nodes, mpc = 2).
    await activateNewKmsContext(protocolConfig, deployer, resetNodes, resetThresholds, [accounts[7]]);
    expect(await protocolConfig.getPublicDecryptionThreshold()).to.equal(1);
    expect(await kmsVerifier.getKmsSigners()).to.deep.equal([signerAddresses[0]]);
  });
});

async function activateNewKmsContext(
  protocolConfig: ProtocolConfig,
  deployer: Signer,
  nodes: KmsNodeParamsStruct[],
  thresholds: KmsThresholdsStruct,
  signerAccounts: Signer[],
): Promise<void> {
  const asOwner = protocolConfig.connect(deployer);
  const context = await defineNewKmsContext(asOwner, nodes, thresholds);
  await activateDefinedKmsContext(asOwner, signerAccounts, context);
}

function isStringArray(value: unknown): value is string[] {
  return Array.isArray(value) && value.every((v) => typeof v === 'string');
}

// Returns the handles listed in the ACL's AllowedForDecryption event emitted in `receipt`.
function getAllowedForDecryptionHandles(receipt: ContractTransactionReceipt): string[] {
  const aclIface = new ethers.Interface(['event AllowedForDecryption(address indexed caller, bytes32[] handlesList)']);
  const event = aclIface.getEvent('AllowedForDecryption');
  if (event === null) {
    throw new Error('AllowedForDecryption event is missing from the ACL interface');
  }
  const log = receipt.logs.find((l) => l.topics[0] === event.topicHash);
  if (log === undefined) {
    throw new Error('No AllowedForDecryption event in the transaction receipt');
  }
  const parsed = aclIface.parseLog(log);
  if (parsed === null) {
    throw new Error('Failed to parse the AllowedForDecryption event');
  }
  const handlesList: unknown = parsed.args.getValue('handlesList');
  if (!isStringArray(handlesList)) {
    throw new Error('AllowedForDecryption.handlesList is not an array of handles');
  }
  return handlesList;
}

async function getPublicDecryptionFromReceipt(
  receipt: ContractTransactionReceipt,
): Promise<{ handles: string[]; decryptedResult: string; decryptionSignatures: string[] }> {
  /// this will scan the tx receipt for all handles which have been made publicly decryptable and attempt to decrypt + sign them
  /// decryptionSignatures always contains the 4 signatures from all the KMS_SIGNER_ADDRESS_i
  await awaitCoprocessor();

  const handles = getAllowedForDecryptionHandles(receipt);
  const clearTexts = await Promise.all(handles.map((h) => coprocDbReadCleartext(h)));
  const types = handles.map(() => 'uint256');
  const decryptedResult = ethers.AbiCoder.defaultAbiCoder().encode(types, clearTexts.map(BigInt));

  const accounts = await ethers.getSigners();
  const signers = [accounts[7], accounts[8], accounts[9], accounts[10]]; /// those are the KMS_SIGNER_ADDRESS_{i} from the default `.env` (with i between 0 and 3)
  const domain = getDecryptionEip712Domain();
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
