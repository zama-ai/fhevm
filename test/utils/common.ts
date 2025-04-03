import { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";
import dotenv from "dotenv";
import { Wallet } from "ethers";
import fs from "fs";
import hre from "hardhat";

const DEFAULT_BALANCE = "0x1000000000000000000000000000000000000000";

// Get the required environment variable, throw an error if it's not set
// We only check if the variable is set, not if it's empty
function getRequiredEnvVar(name: string): string {
  if (!(name in process.env)) {
    throw new Error(`"${name}" env variable is not set`);
  }
  return process.env[name]!;
}

// Add fund to the given address
async function fund(address: string, balance: string) {
  await hre.ethers.provider.send("hardhat_setBalance", [address, balance]);
}

// Create a new random user with some funds
export async function createAndFundRandomUser() {
  const user = hre.ethers.Wallet.createRandom().connect(hre.ethers.provider);
  await fund(user.address, DEFAULT_BALANCE);
  return user;
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
async function initTestingWallets(nKmsNodes: number, nCoprocessors: number) {
  // Get signers
  // - the owner owns the contracts and can initialize the protocol, update FHE params
  // - the pauser can pause the protocol
  // - the user has no particular rights and is mostly used to check roles are properly set
  const owner = new Wallet(getRequiredEnvVar("DEPLOYER_PRIVATE_KEY"), hre.ethers.provider);
  await fund(owner.address, DEFAULT_BALANCE);
  const pauser = await hre.ethers.getSigner(getRequiredEnvVar("PAUSER_ADDRESS"));
  await checkIsHardhatSigner(pauser);
  const user = await createAndFundRandomUser();

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

  return { owner, pauser, user, kmsTxSenders, kmsSigners, coprocessorTxSenders, coprocessorSigners };
}

// Loads the addresses of the deployed contracts, and the values required for the tests.
export async function loadTestVariablesFixture() {
  // Define the number of KMS nodes and coprocessors
  const nKmsNodes = parseInt(getRequiredEnvVar("NUM_KMS_NODES"));
  const nCoprocessors = parseInt(getRequiredEnvVar("NUM_COPROCESSORS"));
  const nNetwork = parseInt(getRequiredEnvVar("NUM_NETWORKS"));

  // Load the transaction senders and signers
  const fixtureData = await initTestingWallets(nKmsNodes, nCoprocessors);

  // Load the networks' chain IDs
  const chainIds = [...Array(nNetwork)].map((_, i) => {
    return parseInt(getRequiredEnvVar(`NETWORK_CHAIN_ID_${i}`));
  });

  // Load the HTTPZ contract
  const parsedEnvHttpz = dotenv.parse(fs.readFileSync("addresses/.env.httpz"));
  const httpz = await hre.ethers.getContractAt("HTTPZ", parsedEnvHttpz.HTTPZ_ADDRESS);

  // Load the ZKPoKManager contract
  const parsedEnvZkpokManager = dotenv.parse(fs.readFileSync("addresses/.env.zkpok_manager"));
  const zkpokManager = await hre.ethers.getContractAt("ZKPoKManager", parsedEnvZkpokManager.ZKPOK_MANAGER_ADDRESS);

  // Load the KeyManager contract
  const parsedEnvKeyManager = dotenv.parse(fs.readFileSync("addresses/.env.key_manager"));
  const keyManager = await hre.ethers.getContractAt("KeyManager", parsedEnvKeyManager.KEY_MANAGER_ADDRESS);

  // Load the CiphertextManager contract
  const parsedEnvCiphertextManager = dotenv.parse(fs.readFileSync("addresses/.env.ciphertext_manager"));
  const ciphertextManager = await hre.ethers.getContractAt(
    "CiphertextManager",
    parsedEnvCiphertextManager.CIPHERTEXT_MANAGER_ADDRESS,
  );

  // Load the ACLManager contract
  const parsedEnvAclManager = dotenv.parse(fs.readFileSync("addresses/.env.acl_manager"));
  const aclManager = await hre.ethers.getContractAt("ACLManager", parsedEnvAclManager.ACL_MANAGER_ADDRESS);

  // Load the DecryptionManager contract
  const parsedEnvDecryptionManager = dotenv.parse(fs.readFileSync("addresses/.env.decryption_manager"));
  const decryptionManager = await hre.ethers.getContractAt(
    "DecryptionManager",
    parsedEnvDecryptionManager.DECRYPTION_MANAGER_ADDRESS,
  );

  // Load the FHE parameters
  const fheParamsName = getRequiredEnvVar("FHE_PARAMS_NAME");
  const fheParamsDigest = getRequiredEnvVar("FHE_PARAMS_DIGEST");

  return {
    ...fixtureData,
    httpz,
    keyManager,
    ciphertextManager,
    aclManager,
    decryptionManager,
    zkpokManager,
    chainIds,
    fheParamsName,
    fheParamsDigest,
  };
}
