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
    const protocolConfigAdd = parsedEnv.PROTOCOL_CONFIG_CONTRACT_ADDRESS;
    const deployer = new ethers.Wallet(process.env.DEPLOYER_PRIVATE_KEY!).connect(ethers.provider);
    const accounts = await ethers.getSigners();
    const newSigners = [accounts[7], accounts[8], accounts[9]];
    const signerAddresses = newSigners.map((s) => s.address);
    const kmsVerifier = await ethers.getContractAt('KMSVerifier', kmsAdd);
    const protocolConfig = await ethers.getContractAt('ProtocolConfig', protocolConfigAdd);
    const newTxSenders = [accounts[2], accounts[3], accounts[4]];
    const newNodes = signerAddresses.map((address, index) => ({
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
    // accounts[7] alone completes the creation quorum: it is the only new signer and its
    // confirmation also covers the previous side's n - t = 1 target (3 nodes, mpc = 2).
    await activateNewKmsContext(protocolConfig, deployer, resetNodes, resetThresholds, [accounts[7]]);
    expect(await protocolConfig.getPublicDecryptionThreshold()).to.equal(1);
    expect(await kmsVerifier.getKmsSigners()).to.deep.equal([signerAddresses[0]]);
  });
});

async function activateNewKmsContext(
  protocolConfig: any,
  deployer: any,
  nodes: any[],
  thresholds: { publicDecryption: number; userDecryption: number; kmsGen: number; mpc: number },
  signerAccounts: any[],
) {
  const previousContextId = await protocolConfig.getCurrentKmsContextId();
  const [, previousEpochId] = await protocolConfig.getCurrentKmsContextAndEpoch();
  const txNewConfig = await protocolConfig.connect(deployer).defineNewKmsContextAndEpoch(nodes, thresholds, '', []);
  const newConfigReceipt = await txNewConfig.wait();
  const contextId = findEventArgs(protocolConfig, newConfigReceipt, 'NewKmsContext').contextId;
  const domain = await protocolConfigDomain(protocolConfig);

  // nodeConfigHash = keccak256(abi.encode(KmsNode[] nodes, KmsThresholds thresholds)) over the stored node fields.
  const nodeConfigHash = ethers.keccak256(
    ethers.AbiCoder.defaultAbiCoder().encode(
      ['tuple(address,address,string,string)[]', 'tuple(uint256,uint256,uint256,uint256)'],
      [
        nodes.map((n) => [n.txSenderAddress, n.signerAddress, n.ipAddress, n.storageUrl]),
        [thresholds.publicDecryption, thresholds.userDecryption, thresholds.kmsGen, thresholds.mpc],
      ],
    ),
  );
  const contextCreationTypes = {
    ContextCreationConfirmation: [
      { name: 'previousContextId', type: 'uint256' },
      { name: 'newContextId', type: 'uint256' },
      { name: 'nodeConfigHash', type: 'bytes32' },
      { name: 'extraData', type: 'bytes' },
    ],
  };

  // Anyone may submit a signer's confirmation, so each signer account submits its own.
  let epochId;
  for (const signer of signerAccounts) {
    const signature = await signer.signTypedData(domain, contextCreationTypes, {
      previousContextId,
      newContextId: contextId,
      nodeConfigHash,
      extraData: '0x',
    });
    const txConfirmContext = await protocolConfig.connect(signer).confirmKmsContextCreation(contextId, signature, '0x');
    const confirmReceipt = await txConfirmContext.wait();
    // The pending epoch ID is only known once enough confirmations reach the context-creation quorum,
    // which emits NewKmsEpoch carrying that epoch ID.
    const createdEvent = findEventArgs(protocolConfig, confirmReceipt, 'NewKmsEpoch');
    if (createdEvent !== undefined) {
      epochId = createdEvent.epochId;
    }
  }
  for (const signer of signerAccounts) {
    const { keys, crsList, signature } = await buildEpochAttestations(
      protocolConfig,
      signer,
      contextId,
      previousEpochId,
      epochId,
    );
    const txConfirmEpoch = await protocolConfig
      .connect(signer)
      .confirmEpochActivation(epochId, keys, crsList, signature, '0x');
    await txConfirmEpoch.wait();
  }
}

async function protocolConfigDomain(protocolConfig: any) {
  return {
    name: 'ProtocolConfig',
    version: '1',
    chainId: (await ethers.provider.getNetwork()).chainId,
    verifyingContract: await protocolConfig.getAddress(),
  };
}

// An empty `keys` or `crsList` reverts with EmptyEpochActivationAttestation, so supply one self-signed
// attestation of each. confirmEpochActivation checks only that the signatures recover to the node signer, so
// the constant ids need not exist in KMSGeneration, and every signer produces the same epochMaterialHash for
// quorum. The returned `signature` is the signer's EpochActivationConfirmation with empty extraData.
async function buildEpochAttestations(
  protocolConfig: any,
  signerAccount: any,
  contextId: bigint,
  previousEpochId: bigint,
  epochId: bigint,
) {
  const domain = await protocolConfigDomain(protocolConfig);
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
  const crsgenTypes = {
    CrsgenVerification: [
      { name: 'crsId', type: 'uint256' },
      { name: 'maxBitLength', type: 'uint256' },
      { name: 'crsDigest', type: 'bytes' },
      { name: 'extraData', type: 'bytes' },
    ],
  };
  const keyId = (4n << 248n) + 1n; // KEY_COUNTER_BASE + 1
  const prepKeygenId = (3n << 248n) + 1n; // PREP_KEYGEN_COUNTER_BASE + 1
  const keyDigests = [{ keyType: 0, digest: '0x01020304' }];
  const crsId = (5n << 248n) + 1n; // CRS_COUNTER_BASE + 1
  const maxBitLength = 4096n;
  const crsDigest = '0x01020304';
  // extraData mirrors abi.encodePacked(EXTRA_DATA_V2, contextId, epochId) with EXTRA_DATA_V2 = 0x02.
  const extraData = ethers.solidityPacked(['uint8', 'uint256', 'uint256'], [2, contextId, epochId]);
  const signature = await signerAccount.signTypedData(domain, keygenTypes, {
    prepKeygenId,
    keyId,
    keyDigests,
    extraData,
  });
  const crsSignature = await signerAccount.signTypedData(domain, crsgenTypes, {
    crsId,
    maxBitLength,
    crsDigest,
    extraData,
  });
  // epochMaterialHash = keccak256(abi.encode(keyHashes, crsHashes)). keyHashes hash the EIP-712 KeyDigest[]
  // array hash, crsHashes ABI-encode the raw crsDigest.
  const abi = ethers.AbiCoder.defaultAbiCoder();
  const keyDigestsHash = ethers.keccak256(
    ethers.concat(
      keyDigests.map((d) => ethers.TypedDataEncoder.hashStruct('KeyDigest', { KeyDigest: keygenTypes.KeyDigest }, d)),
    ),
  );
  const epochMaterialHash = ethers.keccak256(
    abi.encode(
      ['bytes32[]', 'bytes32[]'],
      [
        [ethers.keccak256(abi.encode(['uint256', 'uint256', 'bytes32'], [prepKeygenId, keyId, keyDigestsHash]))],
        [ethers.keccak256(abi.encode(['uint256', 'uint256', 'bytes'], [crsId, maxBitLength, crsDigest]))],
      ],
    ),
  );
  const activationSignature = await signerAccount.signTypedData(
    domain,
    {
      EpochActivationConfirmation: [
        { name: 'contextId', type: 'uint256' },
        { name: 'previousEpochId', type: 'uint256' },
        { name: 'epochId', type: 'uint256' },
        { name: 'epochMaterialHash', type: 'bytes32' },
        { name: 'extraData', type: 'bytes' },
      ],
    },
    { contextId, previousEpochId, epochId, epochMaterialHash, extraData: '0x' },
  );
  return {
    keys: [{ prepKeygenId, keyId, keyDigests, signature }],
    crsList: [{ crsId, maxBitLength, crsDigest, signature: crsSignature }],
    signature: activationSignature,
  };
}

function findEventArgs(protocolConfig: any, receipt: any, eventName: string): any {
  for (const log of receipt.logs) {
    let parsed;
    try {
      parsed = protocolConfig.interface.parseLog(log);
    } catch {
      continue;
    }
    if (parsed?.name === eventName) {
      return parsed.args;
    }
  }
  return undefined;
}

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
