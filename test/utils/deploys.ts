import { loadFixture } from "@nomicfoundation/hardhat-toolbox/network-helpers";
import dotenv from "dotenv";
import fs from "fs";
import hre from "hardhat";
import { config } from "hardhat";
import { HDAccountsUserConfig } from "hardhat/types";

/// @dev Deploy the empty proxies
export async function deployEmptyProxiesFixture() {
  const accounts = config.networks.hardhat.accounts as HDAccountsUserConfig;
  const owner = hre.ethers.Wallet.fromPhrase(accounts.mnemonic, hre.ethers.provider);
  await hre.run("task:deployEmptyUUPSProxies", {
    deployerPrivateKey: owner.privateKey,
  });
  return owner;
}

/// @dev Deploy the HTTPZ contract, initialize the protocol, add KMS nodes and coprocessors
export async function deployHTTPZFixture() {
  // Define the number of KMS nodes and coprocessors
  const kmsThreshold = 1;
  const nKmsNodes = 4;
  const nCoprocessors = 3;
  const chainIds = [2025, 2026, 2027, 2028];

  // Check that the KMS threshold is valid
  if (3 * kmsThreshold >= nKmsNodes) {
    throw new Error("Invalid KMS threshold: 3 * kmsThreshold must be less than the number of KMS nodes");
  }

  // Get signers
  // - the owner owns the HTTPZ contract and can initialize the protocol, update FHE params
  // - the admin can add KMS nodes, coprocessors, networks, trigger key/CRS/KSK generation
  // - the user has no particular rights and is mostly used to check roles are properly set
  const signers = await hre.ethers.getSigners();
  const owner = await loadFixture(deployEmptyProxiesFixture);
  const [admin, user] = signers.splice(1, 3);
  const admins = [admin];

  const kmsSigners = signers.splice(0, nKmsNodes);
  const coprocessorSigners = signers.splice(0, nCoprocessors);

  // Setting up env variables for deployHttpz task
  process.env["PROTOCOL_NAME"] = "Protocol";
  process.env["PROTOCOL_WEBSITE"] = "https://protocol.com";
  process.env["KMS_THRESHOLD"] = kmsThreshold.toString();
  process.env[`ADMIN_ADDRESS_0`] = admin.address;

  // Create dummy KMS nodes with the signers' addresses
  const kmsNodes = kmsSigners.forEach((kmsNode, idx) => {
    process.env[`KMS_NODE_ADDRESS_${idx}`] = kmsNode.address;
    process.env[`KMS_NODE_IDENTITY_${idx}`] = toHexString(hre.ethers.randomBytes(32));
    process.env[`KMS_NODE_IP_ADDRESS_${idx}`] = "127.0.0.1";
    process.env[`KMS_NODE_DA_URL_${idx}`] = "https://da.com";
  });

  // Create dummy Coprocessors with the signers' addresses
  const coprocessors = coprocessorSigners.forEach((coprocessorSigner, idx) => {
    process.env[`COPROCESSOR_ADDRESS_${idx}`] = coprocessorSigner.address;
    process.env[`COPROCESSOR_IDENTITY_${idx}`] = toHexString(hre.ethers.randomBytes(32));
    process.env[`COPROCESSOR_DA_URL_${idx}`] = "https://da.com";
    process.env[`COPROCESSOR_S3_BUCKET_URL_${idx}`] = "s3://bucket";
  });

  // Create dummy Networks with the chain IDs
  process.env.NUM_NETWORKS = chainIds.length.toString();
  const networks = chainIds.map((chainId, idx) => {
    process.env[`NETWORK_CHAIN_ID_${idx}`] = chainId.toString();
    process.env[`NETWORK_HTTPZ_EXECUTOR_${idx}`] = "0x1234567890abcdef1234567890abcdef12345678";
    process.env[`NETWORK_ACL_ADDRESS_${idx}`] = "0xabcdef1234567890abcdef1234567890abcdef12";
    process.env[`NETWORK_NAME_${idx}`] = "Network";
    process.env[`NETWORK_WEBSITE_${idx}`] = "https://network.com";
  });

  await hre.run("task:deployHttpz", {
    deployerPrivateKey: owner.privateKey,
    numAdmins: admins.length.toString(),
    numKmsNodes: nKmsNodes.toString(),
    numCoprocessors: nCoprocessors.toString(),
  });

  await hre.run("task:addNetworksToHttpz");

  const parsedEnvHttpz = dotenv.parse(fs.readFileSync("addresses/.env.httpz"));
  const httpz = await hre.ethers.getContractAt("HTTPZ", parsedEnvHttpz.HTTPZ_ADDRESS);

  return {
    httpz,
    owner,
    admins,
    user,
    kmsSigners,
    coprocessorSigners,
    signers,
    kmsNodes,
    coprocessors,
    networks,
    chainIds,
  };
}

/// @dev Deploy the ZKPoKManager contract
export async function deployZKPoKManagerFixture() {
  const {
    httpz,
    owner,
    admins,
    user,
    kmsSigners,
    coprocessorSigners,
    signers,
    kmsNodes,
    coprocessors,
    networks,
    chainIds,
  } = await loadFixture(deployHTTPZFixture);

  await hre.run("task:deployZkpokManager", {
    deployerPrivateKey: owner.privateKey,
  });

  const parsedEnvZkpokManager = dotenv.parse(fs.readFileSync("addresses/.env.zkpok_manager"));
  const zkpokManager = await hre.ethers.getContractAt("ZKPoKManager", parsedEnvZkpokManager.ZKPOK_MANAGER_ADDRESS);

  return {
    httpz,
    zkpokManager,
    owner,
    admins,
    user,
    kmsSigners,
    coprocessorSigners,
    signers,
    kmsNodes,
    coprocessors,
    networks,
    chainIds,
  };
}

/// @dev Deploy the KeyManager contract
export async function deployKeyManagerFixture() {
  const {
    httpz,
    owner,
    admins,
    user,
    kmsSigners,
    coprocessorSigners,
    signers,
    kmsNodes,
    coprocessors,
    networks,
    chainIds,
  } = await loadFixture(deployHTTPZFixture);

  // Create dummy FHE params and set up env variables for deployKeyManager task
  const fheParamsName = "TEST";
  const fheParamsDigest = hre.ethers.randomBytes(32);
  process.env["FHE_PARAMS_NAME"] = fheParamsName;
  process.env["FHE_PARAMS_DIGEST"] = toHexString(fheParamsDigest);

  await hre.run("task:deployKeyManager", {
    deployerPrivateKey: owner.privateKey,
  });

  const parsedEnvKeyManager = dotenv.parse(fs.readFileSync("addresses/.env.key_manager"));
  const keyManager = await hre.ethers.getContractAt("KeyManager", parsedEnvKeyManager.KEY_MANAGER_ADDRESS);

  return {
    httpz,
    keyManager,
    owner,
    admins,
    user,
    kmsSigners,
    coprocessorSigners,
    signers,
    kmsNodes,
    coprocessors,
    networks,
    chainIds,
    fheParamsName,
    fheParamsDigest,
  };
}

/// @dev Deploy the CiphertextManager contract
export async function deployCiphertextManagerFixture() {
  const {
    httpz,
    keyManager,
    owner,
    admins,
    user,
    kmsSigners,
    coprocessorSigners,
    signers,
    kmsNodes,
    coprocessors,
    networks,
    chainIds,
    fheParamsName,
    fheParamsDigest,
  } = await loadFixture(deployKeyManagerFixture);

  await hre.run("task:deployCiphertextManager", {
    deployerPrivateKey: owner.privateKey,
  });

  const parsedEnvCiphertextManager = dotenv.parse(fs.readFileSync("addresses/.env.ciphertext_manager"));
  const ciphertextManager = await hre.ethers.getContractAt(
    "CiphertextManager",
    parsedEnvCiphertextManager.CIPHERTEXT_MANAGER_ADDRESS,
  );

  return {
    httpz,
    keyManager,
    ciphertextManager,
    owner,
    admins,
    user,
    kmsSigners,
    coprocessorSigners,
    signers,
    kmsNodes,
    coprocessors,
    networks,
    chainIds,
    fheParamsName,
    fheParamsDigest,
  };
}

/// @dev Deploy the ACLManager contract
export async function deployACLManagerFixture() {
  const {
    httpz,
    keyManager,
    ciphertextManager,
    owner,
    admins,
    user,
    kmsSigners,
    coprocessorSigners,
    signers,
    kmsNodes,
    coprocessors,
    networks,
    chainIds,
    fheParamsName,
    fheParamsDigest,
  } = await loadFixture(deployCiphertextManagerFixture);

  await hre.run("task:deployAclManager", {
    deployerPrivateKey: owner.privateKey,
  });

  const parsedEnvAclManager = dotenv.parse(fs.readFileSync("addresses/.env.acl_manager"));
  const aclManager = await hre.ethers.getContractAt("ACLManager", parsedEnvAclManager.ACL_MANAGER_ADDRESS);

  return {
    httpz,
    keyManager,
    ciphertextManager,
    aclManager,
    owner,
    admins,
    user,
    kmsSigners,
    coprocessorSigners,
    signers,
    kmsNodes,
    coprocessors,
    networks,
    chainIds,
    fheParamsName,
    fheParamsDigest,
  };
}

/// @dev Deploy the DecryptionManager contract
export async function deployDecryptionManagerFixture() {
  const {
    httpz,
    keyManager,
    ciphertextManager,
    aclManager,
    owner,
    admins,
    user,
    kmsSigners,
    coprocessorSigners,
    signers,
    kmsNodes,
    coprocessors,
    networks,
    chainIds,
    fheParamsName,
    fheParamsDigest,
  } = await loadFixture(deployACLManagerFixture);

  await hre.run("task:deployDecryptionManager", {
    deployerPrivateKey: owner.privateKey,
  });

  const parsedEnvDecryptionManager = dotenv.parse(fs.readFileSync("addresses/.env.decryption_manager"));
  const decryptionManager = await hre.ethers.getContractAt(
    "DecryptionManager",
    parsedEnvDecryptionManager.DECRYPTION_MANAGER_ADDRESS,
  );

  return {
    httpz,
    keyManager,
    ciphertextManager,
    aclManager,
    decryptionManager,
    owner,
    admins,
    user,
    kmsSigners,
    coprocessorSigners,
    signers,
    kmsNodes,
    coprocessors,
    networks,
    chainIds,
    fheParamsName,
    fheParamsDigest,
  };
}

/// @dev A helper function to convert a bytearray into an hex string (prefixed with '0x')
function toHexString(byteArray: Uint8Array) {
  return (
    "0x" +
    Array.from(byteArray, function (byte) {
      return ("0" + (byte & 0xff).toString(16)).slice(-2);
    }).join("")
  );
}
