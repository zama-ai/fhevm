import { task } from 'hardhat/config';
import { HardhatRuntimeEnvironment } from 'hardhat/types';

// Deploy an ERC20Mock contract
async function deployERC20MockAndMint(
  tokenName: string,
  symbol: string,
  decimals: number,
  hre: HardhatRuntimeEnvironment,
) {
  const { getNamedAccounts, ethers, deployments, network } = hre;
  const { save, getArtifact } = deployments;

  // Get the deployer account
  const { deployer } = await getNamedAccounts();
  const deployerSigner = await ethers.getSigner(deployer);

  const contractName = 'ERC20Mock';

  // Get the contract factory and deploy the ERC20Mock contract
  const erc20MockFactory = await ethers.getContractFactory(contractName, deployerSigner);
  const erc20Mock = await erc20MockFactory.deploy(tokenName, symbol, decimals);
  await erc20Mock.waitForDeployment();

  // Get the ERC20Mock address
  const erc20MockAddress = await erc20Mock.getAddress();

  console.log(
    [
      `✅ Deployed ${tokenName} ERC20Mock:`,
      `  - ERC20Mock address: ${erc20MockAddress}`,
      `  - Token name: ${tokenName}`,
      `  - Token symbol: ${symbol}`,
      `  - Decimals: ${decimals}`,
      `  - Deployed by deployer account: ${deployer}`,
      `  - Network: ${network.name}`,
      '',
    ].join('\n'),
  );

  // Mint the deployer account with 1 billion tokens
  const amount = BigInt(10 ** 9) * BigInt(10 ** decimals);
  await erc20Mock.connect(deployerSigner).mint(deployer, amount);

  console.log(`👉 Minted ${amount} tokens to deployer account ${deployer}`);

  // Save the ERC20Mock contract artifact
  const artifact = await getArtifact(contractName);
  await save(contractName, { address: erc20MockAddress, abi: artifact.abi });

  return erc20MockAddress;
}

// Deploy an ERC20Mock contract
// Example usage:
// npx hardhat task:deployERC20MockAndMintDeployer --network testnet
task('task:deployERC20MockAndMintDeployer').setAction(async function (_, hre) {
  const name = 'ZamaMock';
  const symbol = 'ZAMAMock';
  const decimals = 18;

  console.log(`Deploying mocked ERC20 Zama token: ${name} (${symbol})...\n`);

  await deployERC20MockAndMint(name, symbol, decimals, hre);

  console.log('Mocked ERC20 Zama token deployment complete\n');
});
