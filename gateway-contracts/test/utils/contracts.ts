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
async function initTestingWallets(nKmsNodes: number, nCoprocessors: number, nCustodians: number) {
  // Get signers
  // - the owner owns the contracts and can initialize the protocol, update FHE params
  // - the pauser can pause the protocol
  const owner = new Wallet(getRequiredEnvVar("DEPLOYER_PRIVATE_KEY"), hre.ethers.provider);
  await fund(owner.address);
  const pauser = await hre.ethers.getSigner(getRequiredEnvVar("PAUSER_ADDRESS"));
  await checkIsHardhatSigner(pauser);

  // Load the KMS transaction senders
  const kmsTxSenders = await Promise.all(
    Array.from({ length: nKmsNodes }, async (_, idx) => {
      const kmsTxSender = await hre.ethers.getSigner(getRequiredEnvVar(`KMS_TX_SENDER_ADDRESS_${idx}`));
      await checkIsHardhatSigner(kmsTxSender);
      return kmsTxSender;
    })
  );

  // Load the KMS signers
  const kmsSigners = await Promise.all(
    Array.from({ length: nKmsNodes }, async (_, idx) => {
      const kmsSigner = await hre.ethers.getSigner(getRequiredEnvVar(`KMS_SIGNER_ADDRESS_${idx}`));
      await checkIsHardhatSigner(kmsSigner);
      return kmsSigner;
    })
  );

  // Load the KMS node IPs
  const kmsNodeIps = Array.from({ length: nKmsNodes }, (_, idx) => 
    getRequiredEnvVar(`KMS_NODE_IP_ADDRESS_${idx}`)
  );

  // Load the coprocessor transaction senders
  const coprocessorTxSenders = await Promise.all(
    Array.from({ length: nCoprocessors }, async (_, idx) => {
      const coprocessorTxSender = await hre.ethers.getSigner(getRequiredEnvVar(`COPROCESSOR_TX_SENDER_ADDRESS_${idx}`));
      await checkIsHardhatSigner(coprocessorTxSender);
      return coprocessorTxSender;
    })
  );

  // Load the coprocessor signers
  const coprocessorSigners = await Promise.all(
    Array.from({ length: nCoprocessors }, async (_, idx) => {
      const coprocessorSigner = await hre.ethers.getSigner(getRequiredEnvVar(`COPROCESSOR_SIGNER_ADDRESS_${idx}`));
      await checkIsHardhatSigner(coprocessorSigner);
      return coprocessorSigner;
    })
  );

  // Load the coprocessor S3 buckets
  const coprocessorS3Buckets = Array.from({ length: nCoprocessors }, (_, idx) => 
    getRequiredEnvVar(`COPROCESSOR_S3_BUCKET_URL_${idx}`)
  );

  // Load the custodian transaction senders
  const custodianTxSenders = await Promise.all(
    Array.from({ length: nCustodians }, async (_, idx) => {
      const custodianTxSender = await hre.ethers.getSigner(getRequiredEnvVar(`CUSTODIAN_TX_SENDER_ADDRESS_${idx}`));
      await checkIsHardhatSigner(custodianTxSender);
      return custodianTxSender;
    })
  );

  // Load the custodian signers
  const custodianSigners = await Promise.all(
    Array.from({ length: nCustodians }, async (_, idx) => {
      const custodianSigner = await hre.ethers.getSigner(getRequiredEnvVar(`CUSTODIAN_SIGNER_ADDRESS_${idx}`));
      await checkIsHardhatSigner(custodianSigner);
      return custodianSigner;
    })
  );

  // Load the custodian encryption keys
  const custodianEncryptionKeys = Array.from({ length: nCustodians }, (_, idx) => 
    getRequiredEnvVar(`CUSTODIAN_ENCRYPTION_KEY_${idx}`)
  );

  return {
    owner,
    pauser,
    kmsTxSenders,
    kmsSigners,
    kmsNodeIps,
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

  // Load the KmsManagement contract
  const kmsManagement = await hre.ethers.getContractAt("KmsManagement", getRequiredEnvVar("KMS_MANAGEMENT_ADDRESS"));

  // Load the CiphertextCommits contract
  const ciphertextCommits = await hre.ethers.getContractAt(
    "CiphertextCommits",
    getRequiredEnvVar("CIPHERTEXT_COMMITS_ADDRESS"),
  );

  // Load the MultichainAcl contract
  const multichainAcl = await hre.ethers.getContractAt("MultichainAcl", getRequiredEnvVar("MULTICHAIN_ACL_ADDRESS"));

  // Load the Decryption contract
  const decryption = await hre.ethers.getContractAt("Decryption", getRequiredEnvVar("DECRYPTION_ADDRESS"));

  // Load the FHE parameters
  const fheParamsName = getRequiredEnvVar("FHE_PARAMS_NAME");
  const fheParamsDigest = getRequiredEnvVar("FHE_PARAMS_DIGEST");

  return {
    ...fixtureData,
    gatewayConfig,
    kmsManagement,
    ciphertextCommits,
    multichainAcl,
    decryption,
    inputVerification,
    chainIds,
    nKmsNodes,
    nCoprocessors,
    nCustodians,
    fheParamsName,
    fheParamsDigest,
  };
}
