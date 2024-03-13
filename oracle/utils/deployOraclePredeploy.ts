import { ethers } from 'hardhat';

async function deployOraclePredeploy(deployerPrivateKey: string, ownerAddress: string) {
  const deployer = new ethers.Wallet(deployerPrivateKey).connect(ethers.provider);
  const oracleFactory = await ethers.getContractFactory('OraclePredeploy');
  const oracle = await oracleFactory.connect(deployer).deploy(ownerAddress);
  await oracle.waitForDeployment();
  const oraclePredeployAddress = await oracle.getAddress();
  console.log('oracle was deployed at address: ', oraclePredeployAddress);
}

async function main() {
  const args = process.argv.slice(2);
  if (args.length !== 2) {
    console.error(
      'Please provide exactly 2 arguments. First one should be the privte key of the deployer and second one should be the address of the owner.',
    );
    process.exit(1);
  }
  await deployOraclePredeploy(args[0], args[1]);
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
