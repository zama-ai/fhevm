import { loadFixture } from "@nomicfoundation/hardhat-toolbox/network-helpers";
import hre from "hardhat";

/// @dev Deploy the HTTPZ contract, initialize the protocol, add KMS nodes and coprocessors
export async function deployHTTPZFixture() {
  // Define the number of KMS nodes and coprocessors
  const kmsThreshold = 1;
  const nKmsNodes = 4;
  const nCoprocessors = 3;

  // Check that the KMS threshold is valid
  if (3 * kmsThreshold >= nKmsNodes) {
    throw new Error("Invalid KMS threshold: 3 * kmsThreshold must be less than the number of KMS nodes");
  }

  // Get signers
  // - the owner owns the HTTPZ contract and can initialize the protocol, update FHE params
  // - the admin can add KMS nodes, coprocessors, networks, trigger key/CRS/KSK generation
  // - the user has no particular rights and is mostly used to check roles are properly set
  const signers = await hre.ethers.getSigners();
  const [owner, admin, user] = signers.splice(0, 3);
  const admins = [admin];

  const kmsSigners = signers.splice(0, nKmsNodes);
  const coprocessorSigners = signers.splice(0, nCoprocessors);

  // Create dummy KMS nodes with the signers' addresses
  const kmsNodes = kmsSigners.map((kmsNode) => ({
    connectorAddress: kmsNode.address,
    identity: hre.ethers.randomBytes(32),
    ipAddress: "127.0.0.1",
    signedNodes: [hre.ethers.randomBytes(32)],
    daAddress: "0x1234567890abcdef1234567890abcdef12345678",
  }));

  // Create dummy Coprocessors with the signers' addresses
  const coprocessors = coprocessorSigners.map((coprocessorSigner) => ({
    connectorAddress: coprocessorSigner.address,
    identity: hre.ethers.randomBytes(32),
    daAddress: "0x1234567890abcdef1234567890abcdef12345678",
  }));

  const HTTPZ = await hre.ethers.getContractFactory("HTTPZ", owner);
  const httpz = await HTTPZ.connect(owner).deploy();

  // Initialize a dummy protocol
  const protocolMetadata = { name: "Protocol", website: "https://protocol.com" };
  await httpz.connect(owner).initialize(protocolMetadata, admins, kmsThreshold, kmsNodes, coprocessors);

  const network = {
    chainId: 2025,
    httpzLibrary: "0x1234567890abcdef1234567890abcdef12345678",
    acl: "0xabcdef1234567890abcdef1234567890abcdef12",
    name: "Network",
    website: "https://network.com",
  };

  // Add network
  await httpz.connect(admin).addNetwork(network);

  return { httpz, owner, admins, user, kmsSigners, coprocessorSigners, signers, kmsNodes, coprocessors, network };
}

/// @dev Deploy the KeyManager contract
export async function deployKeyManagerFixture() {
  const { httpz, owner, admins, user, kmsSigners, coprocessorSigners, signers } = await loadFixture(deployHTTPZFixture);

  const KeyManager = await hre.ethers.getContractFactory("KeyManager", owner);
  const keyManager = await KeyManager.deploy(httpz);

  // Set the FHE params
  const fheParams = { dummy: "dummy" };
  await keyManager.connect(owner).setFheParams(fheParams);

  return { httpz, keyManager, owner, admins, user, kmsSigners, coprocessorSigners, signers, fheParams };
}

/// @dev Deploy the CiphertextStorage contract
export async function deployCiphertextStorageFixture() {
  const { httpz, keyManager, owner, admins, user, kmsSigners, coprocessorSigners, signers, fheParams } =
    await loadFixture(deployKeyManagerFixture);

  const CiphertextStorage = await hre.ethers.getContractFactory("CiphertextStorage", owner);
  const ciphertextStorage = await CiphertextStorage.deploy(httpz, keyManager);

  return {
    httpz,
    keyManager,
    ciphertextStorage,
    owner,
    admins,
    user,
    kmsSigners,
    coprocessorSigners,
    signers,
    fheParams,
  };
}

/// @dev Deploy the ACLManager contract
export async function deployACLManagerFixture() {
  const {
    httpz,
    keyManager,
    ciphertextStorage,
    owner,
    admins,
    user,
    kmsSigners,
    coprocessorSigners,
    signers,
    fheParams,
  } = await loadFixture(deployCiphertextStorageFixture);

  const ACLManager = await hre.ethers.getContractFactory("ACLManager", owner);
  const aclManager = await ACLManager.deploy(httpz, ciphertextStorage);

  return {
    httpz,
    keyManager,
    ciphertextStorage,
    aclManager,
    owner,
    admins,
    user,
    kmsSigners,
    coprocessorSigners,
    signers,
    fheParams,
  };
}

/// @dev Deploy the DecryptionManager contract
export async function deployDecryptionManagerFixture() {
  const {
    httpz,
    keyManager,
    ciphertextStorage,
    aclManager,
    owner,
    admins,
    user,
    kmsSigners,
    coprocessorSigners,
    signers,
    fheParams,
  } = await loadFixture(deployACLManagerFixture);

  const dummyPaymentManager = "0x1234567890abcdef1234567890abcdef12345678";

  const DecryptionManager = await hre.ethers.getContractFactory("DecryptionManager", owner);
  const decryptionManager = await DecryptionManager.deploy(httpz, aclManager, ciphertextStorage, dummyPaymentManager);

  return {
    httpz,
    keyManager,
    ciphertextStorage,
    aclManager,
    decryptionManager,
    owner,
    admins,
    user,
    kmsSigners,
    coprocessorSigners,
    signers,
    fheParams,
  };
}
