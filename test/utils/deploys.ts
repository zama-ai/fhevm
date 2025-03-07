import { loadFixture } from "@nomicfoundation/hardhat-toolbox/network-helpers";
import hre from "hardhat";

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
  const [owner, admin, user] = signers.splice(0, 3);
  const admins = [admin];

  const kmsSigners = signers.splice(0, nKmsNodes);
  const coprocessorSigners = signers.splice(0, nCoprocessors);

  const protocolMetadata = { name: "Protocol", website: "https://protocol.com" };

  // Create dummy KMS nodes with the signers' addresses
  const kmsNodes = kmsSigners.map((kmsNode) => ({
    connectorAddress: kmsNode.address,
    identity: hre.ethers.randomBytes(32),
    ipAddress: "127.0.0.1",
    daAddress: "https://da.com",
    tlsCertificate: hre.ethers.randomBytes(32),
  }));

  // Create dummy Coprocessors with the signers' addresses
  const coprocessors = coprocessorSigners.map((coprocessorSigner) => ({
    transactionSenderAddress: coprocessorSigner.address,
    identity: hre.ethers.randomBytes(32),
    daAddress: "https://da.com",
  }));

  // Create dummy Networks with the chain IDs
  const networks = chainIds.map((chainId) => ({
    chainId: chainId,
    httpzExecutor: hre.ethers.getAddress("0x1234567890abcdef1234567890abcdef12345678"),
    aclAddress: hre.ethers.getAddress("0xabcdef1234567890abcdef1234567890abcdef12"),
    name: "Network",
    website: "https://network.com",
  }));

  const HTTPZ = await hre.ethers.getContractFactory("HTTPZ", owner);

  // Deploy the HTTPZ contract
  const httpz = await HTTPZ.connect(owner).deploy(
    protocolMetadata,
    admins,
    kmsThreshold,
    kmsNodes,
    coprocessors,
    networks,
  );

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

  const dummyPaymentManager = "0x1234567890abcdef1234567890abcdef12345678";

  const ZKPoKManager = await hre.ethers.getContractFactory("ZKPoKManager", owner);
  const zkpokManager = await ZKPoKManager.deploy(httpz, dummyPaymentManager);

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

  // Create dummy FHE params
  const fheParamsName = "TEST";
  const fheParamsDigest = hre.ethers.randomBytes(32);

  const KeyManager = await hre.ethers.getContractFactory("KeyManager", owner);
  const keyManager = await KeyManager.deploy(httpz, fheParamsName, fheParamsDigest);

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

/// @dev Deploy the CiphertextStorage contract
export async function deployCiphertextStorageFixture() {
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
    ciphertextStorage,
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
    ciphertextStorage,
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
    kmsNodes,
    coprocessors,
    networks,
    chainIds,
    fheParamsName,
    fheParamsDigest,
  };
}
