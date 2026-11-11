// Step 1.2 of the Hardhat 3 migration: a trivial task that checks the HH3 task and network connection model with
// hardhat-ethers.
//
// `network.getOrCreate()` returns the connection already opened for this network in this Hardhat process, if any:
// every task and test using it shares the same in-process chain (`network.create()` would always start a new one;
// `network.connect()` is deprecated).
import type { HardhatRuntimeEnvironment } from 'hardhat/types/hre';

export default async function networkInfo(_taskArguments: object, hre: HardhatRuntimeEnvironment): Promise<void> {
  const connection = await hre.network.getOrCreate();
  const { ethers } = connection;

  const network = await ethers.provider.getNetwork();
  const signers = await ethers.getSigners();
  const firstSigner = signers[0];
  const balance = await ethers.provider.getBalance(firstSigner.address);

  console.log(`network:      ${connection.networkName}`);
  console.log(`chain id:     ${network.chainId}`);
  console.log(`signers:      ${signers.length}`);
  console.log(`first signer: ${firstSigner.address} (${ethers.formatEther(balance)} ETH)`);
}
