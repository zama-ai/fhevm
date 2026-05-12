import { task } from "hardhat/config";
import { HardhatNetworkHDAccountsConfig } from "hardhat/types";

import { NUM_ACCOUNTS } from "../hardhat.config";

// Use this task to get the list of accounts.
task("get-accounts", "Prints the list of accounts")
  .addFlag("includePrivateKeys", "Print private keys as well as addresses")
  .setAction(async ({ includePrivateKeys }, hre) => {
    // Get signers from hardhat
    const signers = await hre.ethers.getSigners();
    const accounts = [];
    const { mnemonic } = hre.network.config.accounts as HardhatNetworkHDAccountsConfig;

    // Get details for specified number of accounts
    for (let i = 0; i < NUM_ACCOUNTS && i < signers.length; i++) {
      const signer = signers[i];
      const address = await signer.getAddress();
      const wallet = includePrivateKeys
        ? hre.ethers.HDNodeWallet.fromMnemonic(hre.ethers.Mnemonic.fromPhrase(mnemonic), "m/44'/60'/0'/0/" + i)
        : undefined;

      accounts.push({
        index: i,
        privateKey: wallet?.privateKey,
        publicKey: wallet?.publicKey,
        address,
      });
    }
    console.info("\nAccount Details:");
    console.info("================");
    accounts.forEach(({ index, privateKey, address, publicKey }) => {
      console.info(`\nAccount ${index}:`);
      console.info(`Address:     ${address}`);
      if (includePrivateKeys) {
        console.info(`Private Key: ${privateKey}`);
        console.info(`Public Key:  ${publicKey}`);
      }
    });
  });
