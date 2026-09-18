import { ERC20_MOCK_TOKEN_NAME, ERC20_MOCK_TOKEN_SYMBOL, ERC20_MOCK_TOKEN_DECIMALS } from './ERC20Mock';
import { task, types } from 'hardhat/config';

// Verify a mock ERC20 contract
// Example usage:
// npx hardhat task:verifyERC20Mock --contract-address 0x1234567890123456789012345678901234567890 --network testnet
task('task:verifyERC20Mock')
  .addParam('contractAddress', 'The address of the ERC20Mock contract to verify', '', types.string)
  .setAction(async function ({ contractAddress }, hre) {
    const { run } = hre;

    console.log(`Verifying ERC20Mock contract at ${contractAddress}...\n`);
    await run('verify:verify', {
      address: contractAddress,
      constructorArguments: [ERC20_MOCK_TOKEN_NAME, ERC20_MOCK_TOKEN_SYMBOL, ERC20_MOCK_TOKEN_DECIMALS],
    });
    console.log(`ERC20Mock contract verification complete\n`);
  });
