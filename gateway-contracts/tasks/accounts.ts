import { task, types } from "hardhat/config";
import { HardhatNetworkHDAccountsConfig } from "hardhat/types";

import { NUM_ACCOUNTS } from "../hardhat.config";

// Use this task to get the list of accounts (addresses, private keys, public keys)
task("get-accounts", "Prints the list of accounts").setAction(async (_, hre) => {
  // Get signers from hardhat
  const signers = await hre.ethers.getSigners();
  const accounts = [];
  const { mnemonic } = hre.network.config.accounts as HardhatNetworkHDAccountsConfig;

  // Get details for specified number of accounts
  for (let i = 0; i < NUM_ACCOUNTS && i < signers.length; i++) {
    const signer = signers[i];
    const address = await signer.getAddress();
    const phrase = hre.ethers.Mnemonic.fromPhrase(mnemonic);
    const pathDeployer = "m/44'/60'/0'/0/" + i;
    const privateKey = hre.ethers.HDNodeWallet.fromMnemonic(phrase, pathDeployer).privateKey;
    const publicKey = hre.ethers.HDNodeWallet.fromMnemonic(phrase, pathDeployer).publicKey;

    accounts.push({
      index: i,
      privateKey: privateKey,
      publicKey: publicKey,
      address: address,
    });
  }
  console.info("\nAccount Details:");
  console.info("================");
  accounts.forEach(({ index, privateKey, address, publicKey }) => {
    console.info(`\nAccount ${index}:`);
    console.info(`Address:     ${address}`);
    console.info(`Private Key: ${privateKey}`);
    console.info(`Public Key:  ${publicKey}`);
  });
});
