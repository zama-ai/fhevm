import { task, types } from 'hardhat/config';

// Verify a mock ERC20 contract
// Example usage:
// npx hardhat task:verifyMockERC20 --contract-address 0x1234567890123456789012345678901234567890 --network testnet
task('task:verifyMockERC20')
  .addParam('contractAddress', 'The address of the mock ERC20 contract to verify', '', types.string)
  .addParam('name', 'The name of the mock ERC20 contract to verify', '', types.string)
  .addParam('symbol', 'The symbol of the mock ERC20 contract to verify', '', types.string)
  .setAction(async function ({ contractAddress, name, symbol }, hre) {
    const { run } = hre;

    console.log(`Verifying mock ERC20 contract at ${contractAddress}...\n`);
    await run('verify:verify', {
      address: contractAddress,
      constructorArguments: [name, symbol],
    });
    console.log(
      `Mock ERC20 contract verification complete\n`,
    );
  });
