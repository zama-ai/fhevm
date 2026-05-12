import { execFile as oldExecFile } from 'child_process';
import { task } from 'hardhat/config';
import { promisify } from 'util';

const execFile = promisify(oldExecFile);

const validateContainerName = (containerName: string) => {
  if (!/^[a-zA-Z0-9_.-]+$/.test(containerName)) {
    throw new Error(`Invalid TEST_CONTAINER_NAME: ${containerName}`);
  }
};

const getCoin = async (address: string) => {
  const containerName = process.env['TEST_CONTAINER_NAME'] || 'fhevm';
  validateContainerName(containerName);
  const response = await execFile('docker', ['exec', '-i', containerName, 'faucet', address]);
  const stdout = String(response.stdout);
  const line = stdout
    .split('\n')
    .find((entry) => entry.includes('height'));
  const res = JSON.parse(line ?? stdout);
  if (res.raw_log.match('account sequence mismatch')) await getCoin(address);
};

////////////////////////////////////////////////////////////////////////////////
// Balances
////////////////////////////////////////////////////////////////////////////////

task('task:getBalances').setAction(async function (taskArgs, hre) {
  const privKeyDeployer = process.env.DEPLOYER_PRIVATE_KEY;
  const deployerAddress = new hre.ethers.Wallet(privKeyDeployer!).address;
  console.log(await hre.ethers.provider.getBalance(deployerAddress));
});

////////////////////////////////////////////////////////////////////////////////
// Faucet
////////////////////////////////////////////////////////////////////////////////

task('task:faucetToPrivate')
  .addParam('privateKey', 'The receiver private key')
  .setAction(async function (taskArgs, hre) {
    const receiverAddress = new hre.ethers.Wallet(taskArgs.privateKey).address;
    if (hre.network.name === 'hardhat') {
      const bal = '0x1000000000000000000000000000000000000000';
      await hre.network.provider.send('hardhat_setBalance', [receiverAddress, bal]);
    } else {
      await getCoin(receiverAddress);
      await new Promise((res) => setTimeout(res, 5000)); // wait 5 seconds
    }
  });

task('task:faucetToAddress')
  .addParam('address', 'The receiver address')
  .setAction(async function (taskArgs, hre) {
    const receiverAddress = hre.ethers.getAddress(taskArgs.address);
    if (hre.network.name === 'hardhat') {
      const bal = '0x1000000000000000000000000000000000000000';
      await hre.network.provider.send('hardhat_setBalance', [receiverAddress, bal]);
    } else {
      await getCoin(receiverAddress);
      await new Promise((res) => setTimeout(res, 5000)); // wait 5 seconds
    }
  });
