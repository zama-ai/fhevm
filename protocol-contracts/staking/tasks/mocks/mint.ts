import { ERC20_MOCK_CONTRACT_NAME } from '../deployment';
import { getRequiredEnvVar } from '../utils/loadVariables';
import { task, types } from 'hardhat/config';
import { HardhatRuntimeEnvironment } from 'hardhat/types';

// Amount to mint per call: 1 million tokens with 18 decimals
const MINT_AMOUNT = BigInt(10 ** 6) * BigInt(10 ** 18);

// Mint 1 million tokens to the deployer account
// Note: The ERC20Mock contract has a max mint amount of 1 million tokens per call
// Example usage:
// npx hardhat task:mintToDeployer --count 5 --network testnet
task('task:mintToDeployer')
  .addParam('count', 'The number of times to mint 1 million tokens', 1, types.int)
  .setAction(async function ({ count }, hre: HardhatRuntimeEnvironment) {
    const { ethers, network, getNamedAccounts } = hre;

    console.log(`Minting ${count} million tokens to the deployer account...\n`);

    // Get the deployer account
    const { deployer } = await getNamedAccounts();
    const deployerSigner = await ethers.getSigner(deployer);

    // Get the Zama token contract as an ERC20Mock interface
    const zamaToken = await ethers.getContractAt(
      ERC20_MOCK_CONTRACT_NAME,
      getRequiredEnvVar('ZAMA_TOKEN_ADDRESS'),
      deployerSigner,
    );

    // Get the initial balance
    const initialBalance = await zamaToken.balanceOf(deployer);

    // Mint tokens `count` times
    for (let i = 0; i < count; i++) {
      const tx = await zamaToken.mint(deployer, MINT_AMOUNT);
      await tx.wait();
      console.log(`👉 Minted ${MINT_AMOUNT} tokens (${i + 1}/${count})`);

      // Small delay to ensure nonce is updated on the node
      if (i < count - 1) {
        await new Promise(resolve => setTimeout(resolve, 1000));
      }
    }

    // Get the final balance
    const finalBalance = await zamaToken.balanceOf(deployer);
    const totalMinted = finalBalance - initialBalance;

    console.log(
      [
        '',
        `✅ Minting complete:`,
        `  - Token address: ${getRequiredEnvVar('ZAMA_TOKEN_ADDRESS')}`,
        `  - Receiver (deployer): ${deployer}`,
        `  - Total minted: ${totalMinted}`,
        `  - Final balance: ${finalBalance}`,
        `  - Network: ${network.name}`,
        '',
      ].join('\n'),
    );
  });
