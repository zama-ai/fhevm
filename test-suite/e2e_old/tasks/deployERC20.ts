import { task } from "hardhat/config";
import type { TaskArguments } from "hardhat/types";

task("task:deployERC20").setAction(async function (taskArguments: TaskArguments, { ethers }) {
  const signers = await ethers.getSigners();
  const erc20Factory = await ethers.getContractFactory("EncryptedERC20");
  const encryptedERC20 = await erc20Factory.connect(signers[0]).deploy("Naraggara", "NARA");
  await encryptedERC20.waitForDeployment();
  console.log("ERC20 deployed to: ", await encryptedERC20.getAddress());
});
