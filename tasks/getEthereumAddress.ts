import dotenv from "dotenv";
import { ethers } from "ethers";
import { task } from "hardhat/config";

dotenv.config();

task(
  "task:getEthereumAddress",
  "Gets the first address derived from a mnemonic phrase defined in .env",
  async (_taskArgs, _) => {
    const words = process.env.MNEMONIC;
    const mnemonic = ethers.Mnemonic.fromPhrase(words);
    if (!mnemonic) {
      throw new Error("No MNEMONIC in .env file");
    }
    const wallet = ethers.HDNodeWallet.fromMnemonic(mnemonic, `m/44'/60'/0'/0/0`);
    console.log(wallet.address);
  },
);
