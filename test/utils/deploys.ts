import { loadFixture } from "@nomicfoundation/hardhat-toolbox/network-helpers";
import hre from "hardhat";

/// @dev Deploy the HTTPZ contract, initialize the protocol, add KMS nodes and coprocessors
export async function deployHTTPZFixture() {
  // Define the number of KMS nodes and coprocessors
  const nKmsNodes = 4;
  const nCoprocessors = 3;

  // Get signers
  // - the owner owns the HTTPZ contract and can initialize the protocol, update FHE params
  // - the admin can add KMS nodes, coprocessors, networks, trigger key/CRS/KSK generation
  // - the user has no particular rights and is mostly used to check roles are properly set
  const signers = await hre.ethers.getSigners();
  const [owner, admin, user] = signers.splice(0, 3);

  const kmsSigners = signers.splice(0, nKmsNodes);
  const coprocessorSigners = signers.splice(0, nCoprocessors);

  // Create dummy KMS nodes with the signers' addresses
  const kmsNodes = kmsSigners.map((kmsNode) => ({
    connectorAddress: kmsNode.address,
    identity: hre.ethers.randomBytes(32),
    ipAddress: "127.0.0.1",
  }));

  // Create dummy Coprocessors with the signers' addresses
  const coprocessors = coprocessorSigners.map((coprocessorSigner) => ({
    connectorAddress: coprocessorSigner.address,
    identity: hre.ethers.randomBytes(32),
  }));

  const HTTPZ = await hre.ethers.getContractFactory("HTTPZ", owner);
  const httpz = await HTTPZ.connect(owner).deploy();

  // Initialize a dummy protocol
  const protocolMetadata = { name: "Protocol", website: "https://protocol.com" };
  await httpz.connect(owner).initialize(protocolMetadata, [admin.address]);

  // Add the KMS nodes
  await httpz.connect(admin).addKmsNodes(kmsNodes);

  // Mark all KMS nodes as ready, and provide a dummy keychain DA address for each
  for (let i = 0; i < nKmsNodes; i++) {
    await httpz
      .connect(kmsSigners[i])
      .kmsNodeReady(hre.ethers.randomBytes(32), "0x1234567890abcdef1234567890abcdef12345678");
  }

  // Add the coprocessors
  await httpz.connect(admin).addCoprocessors(coprocessors);

  // Mark all coprocessors as ready, and provide a dummy coprocessor DA address for each
  for (let i = 0; i < nCoprocessors; i++) {
    await httpz.connect(coprocessorSigners[i]).coprocessorReady("0x1234567890abcdef1234567890abcdef12345678");
  }

  return { httpz, owner, admin, user, kmsSigners, coprocessorSigners, signers };
}

/// @dev Deploy the KeyManager contract
export async function deployKeyManagerFixture() {
  const { httpz, owner, admin, user, kmsSigners, coprocessorSigners, signers } = await loadFixture(deployHTTPZFixture);

  const KeyManager = await hre.ethers.getContractFactory("KeyManager", owner);
  const keyManager = await KeyManager.deploy(httpz);

  // Set the FHE params
  const fheParams = { dummy: "dummy" };
  await keyManager.connect(owner).setFheParams(fheParams);

  return { httpz, keyManager, owner, admin, user, kmsSigners, coprocessorSigners, signers, fheParams };
}

/// @dev Deploy the CiphertextStorage contract
export async function deployCiphertextStorageFixture() {
  const { httpz, keyManager, owner, admin, user, kmsSigners, coprocessorSigners, signers, fheParams } =
    await loadFixture(deployKeyManagerFixture);

  const CiphertextStorage = await hre.ethers.getContractFactory("CiphertextStorage", owner);
  const ciphertextStorage = await CiphertextStorage.deploy(httpz, keyManager);

  return {
    httpz,
    keyManager,
    ciphertextStorage,
    owner,
    admin,
    user,
    kmsSigners,
    coprocessorSigners,
    signers,
    fheParams,
  };
}
