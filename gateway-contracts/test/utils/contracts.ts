import { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";
import dotenv from "dotenv";
import { Wallet } from "ethers";
import fs from "fs";
import hre from "hardhat";

import { getRequiredEnvVar } from "../../tasks/utils/loadVariables";
import { CoprocessorStruct } from "../../typechain-types/contracts/interfaces/ICoprocessorContexts";
import { fund } from "./wallets";

// Loads the host chains' chain IDs
export function loadHostChainIds() {
  const nHostChain = parseInt(getRequiredEnvVar("NUM_HOST_CHAINS"));
  return [...Array(nHostChain)].map((_, i) => {
    return parseInt(getRequiredEnvVar(`HOST_CHAIN_CHAIN_ID_${i}`));
  });
}

// Check if the given signer is a valid hardhat signer
// This is needed because `hre.ethers.getSigner` does not throw an error if it used on a random address
async function checkIsHardhatSigner(signer: HardhatEthersSigner) {
  const signers = await hre.ethers.getSigners();
  if (signers.findIndex((s) => s.address === signer.address) === -1) {
    throw new Error(
      `The provided address (${signer.address}) is not the address of a valid hardhat signer.
      Please use addresses listed via the 'npx hardhat get-accounts' command.`,
    );
  }
}

// Creates the wallets used for the tests from the private keys in the .env file.
// Adds some funds to these wallets.
async function initTestingWallets(nKmsNodes: number, nCustodians: number) {
  // Get signers
  // - the owner owns the contracts and can initialize the protocol, update FHE params
  // - the pauser can pause the protocol
  const owner = new Wallet(getRequiredEnvVar("DEPLOYER_PRIVATE_KEY"), hre.ethers.provider);
  await fund(owner.address);
  const pauser = await hre.ethers.getSigner(getRequiredEnvVar("PAUSER_ADDRESS"));
  await checkIsHardhatSigner(pauser);

  // Load the KMS transaction senders
  const kmsTxSenders = [];
  for (let idx = 0; idx < nKmsNodes; idx++) {
    const kmsTxSender = await hre.ethers.getSigner(getRequiredEnvVar(`KMS_TX_SENDER_ADDRESS_${idx}`));
    await checkIsHardhatSigner(kmsTxSender);
    kmsTxSenders.push(kmsTxSender);
  }

  // Load the KMS signers
  const kmsSigners = [];
  for (let idx = 0; idx < nKmsNodes; idx++) {
    const kmsSigner = await hre.ethers.getSigner(getRequiredEnvVar(`KMS_SIGNER_ADDRESS_${idx}`));
    await checkIsHardhatSigner(kmsSigner);
    kmsSigners.push(kmsSigner);
  }

  // Load the number of coprocessors
  const nCoprocessors = parseInt(getRequiredEnvVar("NUM_COPROCESSORS"));

  // Load the coprocessors, and their transaction senders and signers
  const coprocessorSigners = [];
  const coprocessorTxSenders = [];
  const coprocessors: CoprocessorStruct[] = [];
  for (let idx = 0; idx < nCoprocessors; idx++) {
    // Load the coprocessor transaction sender
    const txSenderAddress = getRequiredEnvVar(`COPROCESSOR_TX_SENDER_ADDRESS_${idx}`);
    const coprocessorTxSender = await hre.ethers.getSigner(txSenderAddress);
    await checkIsHardhatSigner(coprocessorTxSender);
    coprocessorTxSenders.push(coprocessorTxSender);

    // Load the coprocessor signer
    const signerAddress = getRequiredEnvVar(`COPROCESSOR_SIGNER_ADDRESS_${idx}`);
    const coprocessorSigner = await hre.ethers.getSigner(signerAddress);
    await checkIsHardhatSigner(coprocessorSigner);
    coprocessorSigners.push(coprocessorSigner);

    // Load the coprocessor
    coprocessors.push({
      name: getRequiredEnvVar(`COPROCESSOR_NAME_${idx}`),
      txSenderAddress,
      signerAddress,
      s3BucketUrl: getRequiredEnvVar(`COPROCESSOR_S3_BUCKET_URL_${idx}`),
    });
  }

  // Load the custodian transaction senders
  const custodianTxSenders = [];
  for (let idx = 0; idx < nCustodians; idx++) {
    const custodianTxSender = await hre.ethers.getSigner(getRequiredEnvVar(`CUSTODIAN_TX_SENDER_ADDRESS_${idx}`));
    await checkIsHardhatSigner(custodianTxSender);
    custodianTxSenders.push(custodianTxSender);
  }

  // Load the custodian signers
  const custodianSigners = [];
  for (let idx = 0; idx < nCustodians; idx++) {
    const custodianSigner = await hre.ethers.getSigner(getRequiredEnvVar(`CUSTODIAN_SIGNER_ADDRESS_${idx}`));
    await checkIsHardhatSigner(custodianSigner);
    custodianSigners.push(custodianSigner);
  }

  // Load the custodian encryption keys
  const custodianEncryptionKeys = [];
  for (let idx = 0; idx < nCustodians; idx++) {
    const custodianEncryptionKey = getRequiredEnvVar(`CUSTODIAN_ENCRYPTION_KEY_${idx}`);
    custodianEncryptionKeys.push(custodianEncryptionKey);
  }

  return {
    owner,
    pauser,
    kmsTxSenders,
    kmsSigners,
    coprocessors,
    coprocessorTxSenders,
    coprocessorSigners,
    custodianTxSenders,
    custodianSigners,
    custodianEncryptionKeys,
  };
}

// Loads the addresses of the deployed contracts, and the values required for the tests.
export async function loadTestVariablesFixture() {
  // Load the number of KMS nodes
  const nKmsNodes = parseInt(getRequiredEnvVar("NUM_KMS_NODES"));
  const nCustodians = parseInt(getRequiredEnvVar("NUM_CUSTODIANS"));

  // Load the host chains' chain IDs
  const chainIds = loadHostChainIds();

  // Load the transaction senders and signers
  const fixtureData = await initTestingWallets(nKmsNodes, nCustodians);

  // Load the GatewayConfig contract
  const parsedEnvGatewayConfig = dotenv.parse(fs.readFileSync("addresses/.env.gateway_config"));
  const gatewayConfig = await hre.ethers.getContractAt("GatewayConfig", parsedEnvGatewayConfig.GATEWAY_CONFIG_ADDRESS);

  // Load the CoprocessorContexts contract
  const parsedEnvCoprocessorContexts = dotenv.parse(fs.readFileSync("addresses/.env.coprocessor_contexts"));
  const coprocessorContexts = await hre.ethers.getContractAt(
    "CoprocessorContexts",
    parsedEnvCoprocessorContexts.COPROCESSOR_CONTEXTS_ADDRESS,
  );

  // Load the InputVerification contract
  const parsedEnvInputVerification = dotenv.parse(fs.readFileSync("addresses/.env.input_verification"));
  const inputVerification = await hre.ethers.getContractAt(
    "InputVerification",
    parsedEnvInputVerification.INPUT_VERIFICATION_ADDRESS,
  );

  // Load the KmsManagement contract
  const parsedEnvKmsManagement = dotenv.parse(fs.readFileSync("addresses/.env.kms_management"));
  const kmsManagement = await hre.ethers.getContractAt("KmsManagement", parsedEnvKmsManagement.KMS_MANAGEMENT_ADDRESS);

  // Load the CiphertextCommits contract
  const parsedEnvCiphertextCommits = dotenv.parse(fs.readFileSync("addresses/.env.ciphertext_commits"));
  const ciphertextCommits = await hre.ethers.getContractAt(
    "CiphertextCommits",
    parsedEnvCiphertextCommits.CIPHERTEXT_COMMITS_ADDRESS,
  );

  // Load the MultichainAcl contract
  const parsedEnvMultichainAcl = dotenv.parse(fs.readFileSync("addresses/.env.multichain_acl"));
  const multichainAcl = await hre.ethers.getContractAt("MultichainAcl", parsedEnvMultichainAcl.MULTICHAIN_ACL_ADDRESS);

  // Load the Decryption contract
  const parsedEnvDecryption = dotenv.parse(fs.readFileSync("addresses/.env.decryption"));
  const decryption = await hre.ethers.getContractAt("Decryption", parsedEnvDecryption.DECRYPTION_ADDRESS);

  // Load the FHE parameters
  const fheParamsName = getRequiredEnvVar("FHE_PARAMS_NAME");
  const fheParamsDigest = getRequiredEnvVar("FHE_PARAMS_DIGEST");

  return {
    ...fixtureData,
    gatewayConfig,
    coprocessorContexts,
    kmsManagement,
    ciphertextCommits,
    multichainAcl,
    decryption,
    inputVerification,
    chainIds,
    nKmsNodes,
    nCustodians,
    fheParamsName,
    fheParamsDigest,
  };
}
