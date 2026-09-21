import { OPERATOR_STAKING_CONTRACT_NAME } from '../deployment';
import { getAllOperatorStakingCoproAddresses, getAllOperatorStakingKMSAddresses } from '../utils/getAddresses';
import { getRequiredEnvVar } from '../utils/loadVariables';
import { task, types } from 'hardhat/config';
import { HardhatRuntimeEnvironment } from 'hardhat/types';

// Deposit assets into the operator staking contract from the deployer account
// Note: The assets are in the smallest unit of $ZAMA, using 10^18 decimals
// Example usage:
// npx hardhat task:depositOperatorStakingFromDeployer \
// --assets 1000000000000000000 \
// --receiver 0x1234567890123456789012345678901234567890 \
// --operator-staking-address 0x1234567890123456789012345678901234567890 \
// --network testnet
task('task:depositOperatorStakingFromDeployer')
  .addParam('assets', 'The amount of assets to deposit into the operator staking contract', 0n, types.bigint)
  .addParam('receiver', 'The address to receive the minted shares', '', types.string)
  .addParam(
    'operatorStakingAddress',
    'The address of the operator staking contract to deposit assets into',
    '',
    types.string,
  )
  .setAction(async function ({ assets, receiver, operatorStakingAddress }, hre: HardhatRuntimeEnvironment) {
    const { ethers, network, getNamedAccounts } = hre;

    console.log('Depositing assets into the operator staking contract from the deployer account...');

    // Get the deployer account
    const { deployer } = await getNamedAccounts();
    const deployerSigner = await ethers.getSigner(deployer);

    // Get the Zama token contract as an ERC20 interface
    const zamaToken = await ethers.getContractAt('IERC20', getRequiredEnvVar('ZAMA_TOKEN_ADDRESS'));

    // Approve the operator staking contract with the assets amount
    const txApprove = await zamaToken.connect(deployerSigner).approve(operatorStakingAddress, assets);
    const receiptApprove = await txApprove.wait();

    if (receiptApprove?.status !== 1) {
      throw new Error(
        `Approval failed for contract ${operatorStakingAddress} with assets ${assets} for 
        sender (deployer) ${deployerSigner.address}\n`,
      );
    }

    console.log(`👉 Approval transaction successful\n`);

    // Load the operator staking contract
    const operatorStaking = await ethers.getContractAt(
      OPERATOR_STAKING_CONTRACT_NAME,
      operatorStakingAddress,
      deployerSigner,
    );

    // Deposit assets into the operator staking contract
    const txDeposit = await operatorStaking.deposit(assets, receiver);
    await txDeposit.wait();

    console.log(
      [
        `💰 Deposited assets into operator staking contract:`,
        `  - Operator staking address: ${operatorStakingAddress}`,
        `  - Assets: ${assets}`,
        `  - Receiver: ${receiver}`,
        `  - Sender (deployer): ${deployer}`,
        `  - Network: ${network.name}`,
        '',
      ].join('\n'),
    );
  });

// Deposit assets into all coprocessor operator staking contracts from the deployer account
// Example usage:
// npx hardhat task:depositAllCoproOperatorStakingFromDeployer --network testnet
task('task:depositAllCoproOperatorStakingFromDeployer').setAction(async function (_, hre: HardhatRuntimeEnvironment) {
  console.log('Depositing assets into all coprocessor operator staking contracts from the deployer account...\n');

  // Get the addresses of all coprocessor operator staking contracts
  const operatorStakingAddresses = await getAllOperatorStakingCoproAddresses(hre);

  for (let i = 0; i < operatorStakingAddresses.length; i++) {
    const assets = BigInt(getRequiredEnvVar(`OPERATOR_STAKING_COPRO_INITIAL_DEPOSIT_ASSETS_${i}`));
    const receiver = getRequiredEnvVar(`OPERATOR_STAKING_COPRO_INITIAL_DEPOSIT_RECEIVER_${i}`);

    await hre.run('task:depositOperatorStakingFromDeployer', {
      assets,
      receiver,
      operatorStakingAddress: operatorStakingAddresses[i],
    });
  }
});

// Deposit assets into all KMS operator staking contracts from the deployer account
// Example usage:
// npx hardhat task:depositAllKMSOperatorStakingFromDeployer --network testnet
task('task:depositAllKMSOperatorStakingFromDeployer').setAction(async function (_, hre: HardhatRuntimeEnvironment) {
  console.log('Depositing assets into all KMS operator staking contracts from the deployer account...\n');

  // Get the addresses of all KMS operator staking contracts
  const operatorStakingAddresses = await getAllOperatorStakingKMSAddresses(hre);

  for (let i = 0; i < operatorStakingAddresses.length; i++) {
    const assets = BigInt(getRequiredEnvVar(`OPERATOR_STAKING_KMS_INITIAL_DEPOSIT_ASSETS_${i}`));
    const receiver = getRequiredEnvVar(`OPERATOR_STAKING_KMS_INITIAL_DEPOSIT_RECEIVER_${i}`);

    await hre.run('task:depositOperatorStakingFromDeployer', {
      assets,
      receiver,
      operatorStakingAddress: operatorStakingAddresses[i],
    });
  }
});

// Deposit assets into all operator staking contracts from the deployer account
// Example usage:
// npx hardhat task:depositAllOperatorStakingFromDeployer --network testnet
task('task:depositAllOperatorStakingFromDeployer').setAction(async function (_, hre: HardhatRuntimeEnvironment) {
  console.log('Depositing assets into all operator staking contracts from the deployer account...\n');

  await hre.run('task:depositAllCoproOperatorStakingFromDeployer');
  await hre.run('task:depositAllKMSOperatorStakingFromDeployer');

  console.log('✅ All operator staking contracts deposited from the deployer account\n');
});
