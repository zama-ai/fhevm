import { ethers } from 'hardhat';

async function addRelayerOracle(ownerPrivateKey: string, oracleAddress: string, relayerAddress: string) {
  const codeAtAddress = await ethers.provider.getCode(oracleAddress);
  if (codeAtAddress === '0x') {
    throw Error(`${oracleAddress} is not a smart contract`);
  }
  const owner = new ethers.Wallet(ownerPrivateKey).connect(ethers.provider);
  const oracle = await ethers.getContractAt('OraclePredeploy', oracleAddress, owner);
  const tx = await oracle.addRelayer(relayerAddress);
  const rcpt = await tx.wait();
  if (rcpt.status === 1) {
    console.log(`Account ${oracleAddress} was succesfully added as an oracle relayer`);
  } else {
    console.log('Adding relayer failed');
  }
}

async function main() {
  const args = process.argv.slice(2);
  if (args.length !== 3) {
    console.error(
      'Please provide exactly 3 arguments. First one should be the privte key of the owner, second one should be the address of OraclePredeploy, and third one the address of the relayer.',
    );
    process.exit(1);
  }
  await addRelayerOracle(args[0], args[1], args[2]);
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
