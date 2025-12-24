import { task, types } from 'hardhat/config';
import { HardhatRuntimeEnvironment } from 'hardhat/types';

const ERC20_MOCK_CONTRACT_NAME = 'ERC20Mock';

// Deploy the ERC20Mock contract
async function deployERC20Mock(hre: HardhatRuntimeEnvironment, name: string, symbol: string, decimals: number) {
  const { getNamedAccounts, ethers, deployments, network } = hre;
  const { save, getArtifact } = deployments;

  const { deployer } = await getNamedAccounts();
  const deployerSigner = await ethers.getSigner(deployer);

  const factory = await ethers.getContractFactory(ERC20_MOCK_CONTRACT_NAME, deployerSigner);
  const contract = await factory.deploy(name, symbol, decimals);
  await contract.waitForDeployment();

  const contractAddress = await contract.getAddress();

  console.log(
    [
      `✅ Deployed ERC20Mock:`,
      `  - Address: ${contractAddress}`,
      `  - Name: ${name}`,
      `  - Symbol: ${symbol}`,
      `  - Deployed by deployer account: ${deployer}`,
      `  - Network: ${network.name}`,
      '',
    ].join('\n'),
  );

  const artifact = await getArtifact(ERC20_MOCK_CONTRACT_NAME);
  await save(ERC20_MOCK_CONTRACT_NAME, {
    address: contractAddress,
    abi: artifact.abi,
  });

  return contractAddress;
}

// Deploy the ERC20Mock contract
// Example usage:
// npx hardhat task:deployERC20Mock --name "Mock Token" --symbol "MTK" --decimals 18 --network testnet
task('task:deployERC20Mock')
  .addParam('name', 'The name of the ERC20 token')
  .addParam('symbol', 'The symbol of the ERC20 token')
  .addParam('decimals', 'The decimals of the ERC20 token', 18, types.int)
  .setAction(async function ({ name, symbol, decimals }, hre) {
    console.log('Deploying ERC20Mock contract...\n');

    await deployERC20Mock(hre, name, symbol, parseInt(decimals));

    console.log('✅ ERC20Mock contract deployed\n');
  });
