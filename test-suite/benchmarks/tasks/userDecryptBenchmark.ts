import { task, types } from "hardhat/config";
import { DecryptedResults } from '@zama-fhe/relayer-sdk/node';
import { HardhatRuntimeEnvironment } from 'hardhat/types';
import { UserDecryptBenchmark } from '../types';
import { createInstance, KeyPair, sleep, zip, timeout } from './shared';
const TIMEOUT = 30_000;


interface TaskArgs {
  benchmarkContractAddress?: string;
  decryptions: number,
  sleepBeforeDecryptions: number,
  sleepBetweenDecryptions: number,
}

// TODO: instead of waiting before decryptions we should have a utility functions to wait for Ciphertext Add and ACL propagation of ciphertexts.
// TODO: improve contract to be able to add chunks of values to avoid HCL limit

task("user-decrypt-benchmark", `User-Decryption Benchmark.

This script does the following:
- Deploy contract if benchmark contract address is not provided.
- Create X random euint64 on-chain
- TODO: Wait for ACL propagation and Ciphertext Add to happen on Gateway chain.
- Decrypt X first values held by contract
`)
  .addParam("benchmarkContractAddress", "Already deployed benchmark contract address", undefined, types.string, true)
  .addParam("decryptions", "Number of decryptions to do", 10, types.int, true)
  .addParam("sleepBetweenDecryptions", "Sleep (milliseconds) between decryptions", 0, types.int, true)
  .addParam("sleepBeforeDecryptions", "Sleep (milliseconds) before decryptions", 0, types.int, true)
  .setAction(async (taskArgs: TaskArgs, hre: HardhatRuntimeEnvironment) => {
    const { ethers, network } = hre;
    const signers = await ethers.getSigners();
    const signer = signers[0];
    console.log("Network:", network.name);
    console.log("Signer:", signer.address);

    const instance = await createInstance(network);

    // Deploy benchmark contract (or not)
    let contract: UserDecryptBenchmark;
    let contractAddress: string;
    if (taskArgs.benchmarkContractAddress == null) {
      console.info("Deploying contract");
      const contractFactory = await ethers.getContractFactory('UserDecryptBenchmark');
      contract = await contractFactory.connect(signer).deploy();
      await contract.waitForDeployment();
      contractAddress = await contract.getAddress();
    } else {
      console.info("Using pre-deployed contract");
      const contractFactory = await ethers.getContractFactory('UserDecryptBenchmark');
      contract = contractFactory.attach(taskArgs.benchmarkContractAddress).connect(signer) as UserDecryptBenchmark;
      contractAddress = taskArgs.benchmarkContractAddress;
    }
    console.info(`Using contract deployed at ${contractAddress}`);
    console.info(`Contract owner: ${await contract.owner()}`);

    // Boiler-plate for user-decryption
    let publicKey: string;
    let privateKey: string;
    console.debug("Generating keypair");
    const keypair: KeyPair = instance.generateKeypair();
    publicKey = keypair.publicKey;
    privateKey = keypair.privateKey;

    // EIP-712
    const durationDays = '100'; // String for consistency
    const contractAddresses = [contractAddress];
    const currentTimestamp = Math.floor(Date.now() / 1000).toString();
    const eip712 = instance.createEIP712(publicKey, contractAddresses, currentTimestamp, durationDays);
    const eip712Signature = await signer.signTypedData(
      eip712.domain,
      {
        UserDecryptRequestVerification: eip712.types.UserDecryptRequestVerification,
      },
      eip712.message,
    );

    // Initialize fresh random values on-chain
    console.log("Refreshing ciphertexts");
    const transaction = await contract.refresh(taskArgs.decryptions);
    await transaction.wait();

    // Validate  was succesful
    const valuesCount: bigint = await contract.getValuesCount();
    console.assert(valuesCount >= taskArgs.decryptions, `on-chain-values.length=${valuesCount} < ${taskArgs.decryptions}=decryptions-requested`);

    const handles: string[] = await contract.getValuesRange(0, taskArgs.decryptions);
    console.assert(handles.length == taskArgs.decryptions, `on-chain-handles=${handles.length} != ${taskArgs.decryptions}=decryptions-requested`);

    await sleep(taskArgs.sleepBeforeDecryptions);

    // Leverage cache of relayer to retry until all promises resolve
    console.log("Decrypting ...");
    console.time("decrytions");
    let decrypted_values: DecryptedResults[] | null;
    let counter = 0;
    while (true) {
      console.time("decrytions-inner");
      let decrypted_values_promises: Promise<DecryptedResults>[] = [];
      for (let index = 0; index < taskArgs.decryptions; index++) {

        decrypted_values_promises.push(instance.userDecrypt([{ handle: handles[index], contractAddress: contractAddress }], privateKey, publicKey, eip712Signature, [contractAddress], signer.address, currentTimestamp, durationDays));
        await sleep(taskArgs.sleepBetweenDecryptions);
      }

      // monitor promises states
      const promiseStates = new Array(decrypted_values_promises.length).fill('pending');
      decrypted_values_promises.forEach((p, i) => {
        p.then(() => { promiseStates[i] = 'fulfilled' })
          .catch(() => { promiseStates[i] = 'rejected' });
      });


      try {
        console.log(`Waiting for all decryption promises to resolve within ${TIMEOUT}ms`)
        try {
          decrypted_values = await Promise.race([Promise.all(decrypted_values_promises), timeout(TIMEOUT)]);
          console.timeEnd("decrytions-inner");
        } catch (timeoutError) {
          console.log("Counting fulfilled decryptions.");
          const fulfilled = promiseStates.filter(s => s === 'fulfilled').length;
          console.log(`Timeout after ${TIMEOUT}ms: ${fulfilled}/${decrypted_values_promises.length} promises already resolved`);
          throw timeoutError;
        }
        break;
      } catch (error) {
        counter += 1;
        console.error(error);
        console.log(`Retrying user decryptions attempt: ${counter}`);
        console.timeEnd("decrytions-inner");
        // To avoid spamming too much the relayer
        console.debug("Waiting 1s before retrying");
        await sleep(1000);
      }
    }
    console.timeEnd("decrytions");

    if (decrypted_values) {
      console.log("All decryption promises resolved")
      for (const [result, handle] of zip(decrypted_values, handles)) {
        console.log(result[handle]);
      }
      console.log(decrypted_values);
    }
  });
