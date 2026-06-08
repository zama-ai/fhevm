// One-off, no-reinitializer upgrade of the deployed Decryption proxy to the implementation that
// adds `userDecryptionRequestSolana` (+ the UserDecryptionRequestSolana event). The change adds no
// storage, so the layout is identical to the deployed impl — a clean storage-compatible swap; the
// new function lives in the new implementation bytecode. `forceImport` rebuilds the OZ upgrades
// manifest from the deployed proxy (the local worktree has none). `_authorizeUpgrade` is
// `onlyGatewayOwner`, so DEPLOYER_PRIVATE_KEY must be the GatewayConfig owner.
//
//   RPC_URL=http://127.0.0.1:8546 npx hardhat run --network staging scripts/upgradeDecryptionSolana.ts
import { ethers, upgrades } from "hardhat";
import { Wallet } from "ethers";

async function main() {
  const proxy = process.env.DECRYPTION_PROXY ?? "0xDE409109E0fCCAaE7B87De518F61d617A3fda094";
  const deployerKey = process.env.DEPLOYER_PRIVATE_KEY;
  if (!deployerKey) throw new Error("DEPLOYER_PRIVATE_KEY not set");
  const deployer = new Wallet(deployerKey).connect(ethers.provider);
  console.log(`Deployer: ${deployer.address}`);
  console.log(`Decryption proxy: ${proxy}`);

  const factory = await ethers.getContractFactory("contracts/Decryption.sol:Decryption", deployer);

  console.log("forceImport (rebuild OZ manifest from the deployed proxy)...");
  await upgrades.forceImport(proxy, factory, { kind: "uups" });

  const before = await upgrades.erc1967.getImplementationAddress(proxy);
  console.log(`Current implementation: ${before}`);

  console.log("upgradeProxy (no reinitializer; storage-identical)...");
  await upgrades.upgradeProxy(proxy, factory);

  const after = await upgrades.erc1967.getImplementationAddress(proxy);
  console.log(`New implementation:     ${after}`);
  console.log(after.toLowerCase() === before.toLowerCase() ? "WARN: impl unchanged" : "Upgrade OK");
}

main().catch((e) => {
  console.error(e);
  process.exit(1);
});
