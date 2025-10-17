import { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";
import dotenv from "dotenv";
import { Wallet } from "ethers";
import hre from "hardhat";
import path from "path";

import { ADDRESSES_DIR } from "../../hardhat.config";
import { getRequiredEnvVar } from "../../tasks/utils/loadVariables";
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
async function checkIsHardhatSigner(signer: HardhatEthersSigner | Wallet) {
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
async function initTestingWallets(nKmsNodes: number, nCoprocessors: number, nCustodians: number) {
  // The owner owns the contracts and can initialize the protocol
  const owner = new Wallet(getRequiredEnvVar("DEPLOYER_PRIVATE_KEY"), hre.ethers.provider);
  await fund(owner.address);

  // A pauser can pause the protocol by pausing some of the contracts
  const pauser = new Wallet(getRequiredEnvVar("PAUSER_PRIVATE_KEY"), hre.ethers.provider);
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

  // Load the KMS node IPs
  const kmsNodeIps = [];
  for (let idx = 0; idx < nKmsNodes; idx++) {
    const kmsNodeIp = getRequiredEnvVar(`KMS_NODE_IP_ADDRESS_${idx}`);
    kmsNodeIps.push(kmsNodeIp);
  }

  // Load the KMS node storage URLs
  const kmsNodeStorageUrls = [];
  for (let idx = 0; idx < nKmsNodes; idx++) {
    const kmsNodeStorageUrl = getRequiredEnvVar(`KMS_NODE_STORAGE_URL_${idx}`);
    kmsNodeStorageUrls.push(kmsNodeStorageUrl);
  }

  // Load the coprocessor transaction senders
  const coprocessorTxSenders = [];
  for (let idx = 0; idx < nCoprocessors; idx++) {
    const coprocessorTxSender = await hre.ethers.getSigner(getRequiredEnvVar(`COPROCESSOR_TX_SENDER_ADDRESS_${idx}`));
    await checkIsHardhatSigner(coprocessorTxSender);
    coprocessorTxSenders.push(coprocessorTxSender);
  }

  // Load the coprocessor signers
  const coprocessorSigners = [];
  for (let idx = 0; idx < nCoprocessors; idx++) {
    const coprocessorSigner = await hre.ethers.getSigner(getRequiredEnvVar(`COPROCESSOR_SIGNER_ADDRESS_${idx}`));
    await checkIsHardhatSigner(coprocessorSigner);
    coprocessorSigners.push(coprocessorSigner);
  }

  // Load the coprocessor S3 buckets
  const coprocessorS3Buckets = [];
  for (let idx = 0; idx < nCoprocessors; idx++) {
    const coprocessorS3Bucket = getRequiredEnvVar(`COPROCESSOR_S3_BUCKET_URL_${idx}`);
    coprocessorS3Buckets.push(coprocessorS3Bucket);
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
    kmsNodeIps,
    kmsNodeStorageUrls,
    coprocessorTxSenders,
    coprocessorSigners,
    coprocessorS3Buckets,
    custodianTxSenders,
    custodianSigners,
    custodianEncryptionKeys,
  };
}

// Loads the addresses of the deployed contracts, and the values required for the tests.
export async function loadTestVariablesFixture() {
  // Load the number of KMS nodes and coprocessors
  const nKmsNodes = parseInt(getRequiredEnvVar("NUM_KMS_NODES"));
  const nCoprocessors = parseInt(getRequiredEnvVar("NUM_COPROCESSORS"));
  const nCustodians = parseInt(getRequiredEnvVar("NUM_CUSTODIANS"));

  // Load the host chains' chain IDs
  const chainIds = loadHostChainIds();

  // Load the transaction senders and signers
  const fixtureData = await initTestingWallets(nKmsNodes, nCoprocessors, nCustodians);

  // Load the environment variables for the /addresses directory
  dotenv.config({ path: path.join(ADDRESSES_DIR, ".env.gateway"), override: true });

  // Load the GatewayConfig contract
  const gatewayConfig = await hre.ethers.getContractAt("GatewayConfig", getRequiredEnvVar("GATEWAY_CONFIG_ADDRESS"));

  // Load the InputVerification contract
  const inputVerification = await hre.ethers.getContractAt(
    "InputVerification",
    getRequiredEnvVar("INPUT_VERIFICATION_ADDRESS"),
  );

  // Load the KMSGeneration contract
  const kmsGeneration = await hre.ethers.getContractAt("KMSGeneration", getRequiredEnvVar("KMS_GENERATION_ADDRESS"));

  // Load the CiphertextCommits contract
  const ciphertextCommits = await hre.ethers.getContractAt(
    "CiphertextCommits",
    getRequiredEnvVar("CIPHERTEXT_COMMITS_ADDRESS"),
  );

  // Load the MultichainACL contract
  const multichainACL = await hre.ethers.getContractAt("MultichainACL", getRequiredEnvVar("MULTICHAIN_ACL_ADDRESS"));

  // Load the Decryption contract
  const decryption = await hre.ethers.getContractAt("Decryption", getRequiredEnvVar("DECRYPTION_ADDRESS"));

  // Load the PauserSet contract
  const pauserSet = await hre.ethers.getContractAt("PauserSet", getRequiredEnvVar("PAUSER_SET_ADDRESS"));

  return {
    ...fixtureData,
    gatewayConfig,
    kmsGeneration,
    ciphertextCommits,
    multichainACL,
    decryption,
    inputVerification,
    chainIds,
    nKmsNodes,
    nCoprocessors,
    nCustodians,
    pauserSet,
  };
}
