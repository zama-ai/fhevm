import { task } from "hardhat/config";
import type { TaskArguments } from "hardhat/types";

import { getInstance } from "../test/instance";

task("task:mint")
  .addParam("mint", "Tokens to mint")
  .addParam("account", "Specify which account [0, 9]")
  .setAction(async function (taskArguments: TaskArguments, hre) {
    const { ethers, deployments } = hre;
    const EncryptedERC20 = await deployments.get("EncryptedERC20");

    const instance = await getInstance(EncryptedERC20.address, ethers);

    const signers = await ethers.getSigners();

    const encryptedERC20 = await ethers.getContractAt("EncryptedERC20", EncryptedERC20.address);

    await encryptedERC20.connect(signers[taskArguments.account]).mint(instance.encrypt32(taskArguments.mint));

    console.log("Mint done: ", taskArguments.mint);
  });
