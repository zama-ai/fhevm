import { task } from "hardhat/config";
import { HardhatNetworkHDAccountsConfig } from "hardhat/types";

const NUM_ACCOUNTS = 10;

// Use this task to get the list of accounts (addresses, private keys, public keys)
// Example usage:
// npx hardhat get-accounts
task("get-accounts", "Prints the list of accounts").setAction(
  async (_, { ethers, network }) => {
    // Get signers from hardhat
    const signers = await ethers.getSigners();
    const accounts = [];
    const { mnemonic } = network.config
      .accounts as HardhatNetworkHDAccountsConfig;

    // Get details for specified number of accounts
    for (let i = 0; i < NUM_ACCOUNTS && i < signers.length; i++) {
      const signer = signers[i];
      const address = await signer.getAddress();
      const phrase = ethers.Mnemonic.fromPhrase(mnemonic);
      const pathDeployer = "m/44'/60'/0'/0/" + i;
      const privateKey = ethers.HDNodeWallet.fromMnemonic(
        phrase,
        pathDeployer,
      ).privateKey;
      const publicKey = ethers.HDNodeWallet.fromMnemonic(
        phrase,
        pathDeployer,
      ).publicKey;

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
  },
);
