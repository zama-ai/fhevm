import hre from "hardhat";

/// @dev Deploy the HTTPZ contract, initialize the protocol and add KMS nodes
export async function deployInitKmsHTTPZFixture() {
  // Define the number of KMS nodes
  const n = 4;

  // Get signers
  const signers = await hre.ethers.getSigners();
  const [owner, admin] = signers.splice(0, 2);
  const kmsSigners = signers.splice(0, n);
  const coprocessorSigners = signers.splice(0, n);

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

  // Add the Coprocessor
  await httpz.connect(admin).addCoprocessors(coprocessors);

  // Mark all KMS nodes as ready
  for (let i = 0; i < n; i++) {
    await httpz
      .connect(kmsSigners[i])
      .kmsNodeReady(hre.ethers.randomBytes(32), "0x1234567890abcdef1234567890abcdef12345678");

    await httpz.connect(coprocessorSigners[i]).coprocessorReady("0x1234567890abcdef1234567890abcdef12345678");
  }

  return { httpz, owner, admin, kmsSigners, coprocessorSigners, signers };
}
