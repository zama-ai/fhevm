import dotenv from "dotenv";
import fs from "fs";
import { task } from "hardhat/config";
import type { HardhatEthersHelpers, TaskArguments } from "hardhat/types";

import { HTTPZ } from "../typechain-types";

async function loadHttpzContract(customHttpzAddress: string | undefined, ethers: HardhatEthersHelpers): Promise<HTTPZ> {
  const httpzFactory = await ethers.getContractFactory("./contracts/HTTPZ.sol:HTTPZ");
  const httpzAddress = customHttpzAddress
    ? customHttpzAddress
    : dotenv.parse(fs.readFileSync("addresses/.env.httpz")).HTTPZ_ADDRESS;
  return httpzFactory.attach(httpzAddress).connect(ethers.provider) as HTTPZ;
}

task("task:getKmsSigners")
  .addOptionalParam(
    "customHttpzAddress",
    "Use a custom address for the HTTPZ contract instead of the default one - ie stored inside .env.httpz",
  )
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    const httpz = await loadHttpzContract(taskArguments.customHttpzAddress, ethers);
    const listCurrentKMSSigners = await httpz.getKmsSigners();
    console.log("The list of current KMS Signers stored inside HTTPZ contract is: ", listCurrentKMSSigners);
  });

task("task:getCoprocessorSigners")
  .addOptionalParam(
    "customHttpzAddress",
    "Use a custom address for the HTTPZ contract instead of the default one - ie stored inside .env.httpz",
  )
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    const httpz = await loadHttpzContract(taskArguments.customHttpzAddress, ethers);
    const listCurrentCoprocessorSigners = await httpz.getCoprocessorSigners();
    console.log(
      "The list of current Coprocessor Signers stored inside HTTPZ contract is: ",
      listCurrentCoprocessorSigners,
    );
  });

task("task:getNetworks")
  .addOptionalParam(
    "customHttpzAddress",
    "Use a custom address for the HTTPZ contract instead of the default one - ie stored inside .env.httpz",
  )
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    const httpz = await loadHttpzContract(taskArguments.customHttpzAddress, ethers);
    const listCurrentNetworks = await httpz.getNetworks();
    console.log("The list of current Networks stored inside HTTPZ contract is: ", listCurrentNetworks);
  });
