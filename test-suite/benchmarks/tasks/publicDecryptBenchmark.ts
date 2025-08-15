import { task, types } from "hardhat/config";
import { DecryptedResults } from '@zama-fhe/relayer-sdk/node';
import { HardhatRuntimeEnvironment } from 'hardhat/types';
import { PublicDecryptBenchmark } from '../types';
import { createInstance, sleep, timeout, hasFunctionReverted } from './shared';
import { resolve, join } from 'path';
import fs from 'fs';

const cacheDir = resolve(__dirname, '../decryption_cache');
if (!fs.existsSync(cacheDir)) {
  fs.mkdirSync(cacheDir, { recursive: true });
}

interface TaskArgs {
  decryptionsPerBatch: number,
  nBatch: number,
  sleepBetweenDecryptions: number,
  sleepBetweenBatches: number,
  deployNewContract: boolean,
  debugLogs: boolean,
}

task("public-decrypt-benchmark", `Public-Decryption Benchmark.

This script does the following:
- Deploy contract if cache is not used or does not exist yet
- Create decryptionsPerBatch x nBatch random euint64 on-chain
- Wait until all values are ready to be decrypted (ACL propagation and Ciphertext Add)
- Public decrypt all values
`)
  .addParam("decryptionsPerBatch", "Number of decryptions to do per batch", 10, types.int, true)
  .addParam("nBatch", "Number of batches to do", 1, types.int, true)
  .addParam("sleepBetweenDecryptions", "Sleep (milliseconds) between decryptions", 0, types.int, true)
  .addParam("sleepBetweenBatches", "Sleep (milliseconds) between batches", 0, types.int, true)
  .addParam("deployNewContract", "Deploy a new contract. Use cache if false", false, types.boolean, true)
  .addParam("debugLogs", "Display debug logs", false, types.boolean, true)
  .setAction(async (taskArgs: TaskArgs, hre: HardhatRuntimeEnvironment) => {
    const { ethers, network } = hre;
    const decryptionContractAddress = process.env.DECRYPTION_CONTRACT_ADDRESS;
    if (!decryptionContractAddress) {
      throw new Error("DECRYPTION_CONTRACT_ADDRESS is not set");
    }
    const rpcUrl = process.env.RPC_URL;
    if (!rpcUrl) {
      throw new Error("RPC_URL is not set");
    }
    const provider = new ethers.JsonRpcProvider(rpcUrl);

    const signers = await ethers.getSigners();
    const signer = signers[0];
    console.log("Network:", network.name);
    console.log("Signer:", signer.address);

    const totalDecryptions = taskArgs.decryptionsPerBatch * taskArgs.nBatch;
    console.log(`Total number of decryptions: ${totalDecryptions} (${taskArgs.decryptionsPerBatch} x ${taskArgs.nBatch})`);
  
    // Currently, a single public decryption takes ~ 0.3s, so we set a timeout a little higher:
    // - 60s for taking into account the latency for small number of decryptions
    // - 0.5s per decryption
    const timeoutMs = 60000 + totalDecryptions * 500; 
    console.log(`Total decryption timeout: ${timeoutMs}ms`);

    const instance = await createInstance(network, taskArgs.debugLogs);

    // Deploy benchmark contract (or not)
    let contract: PublicDecryptBenchmark;
    let contractAddress: string;

    // Public contract address cache file path
    const contractCacheFile = join(cacheDir, `public_contract.json`);
    
    const contractFactory = await ethers.getContractFactory('PublicDecryptBenchmark');

    // Load contract address from cache if it exists
    if (!taskArgs.deployNewContract && fs.existsSync(contractCacheFile)) {
      const contractCache = JSON.parse(fs.readFileSync(contractCacheFile, 'utf-8'));

      console.info("Using pre-deployed contract");
      contractAddress = contractCache.contractAddress;
      contract = contractFactory.attach(contractAddress).connect(signer) as PublicDecryptBenchmark;
      console.info(`Using contract deployed at ${contractAddress}`);

      console.log(`Loaded cache from ${contractCacheFile}`);
    } else {
  
      console.info("Deploying contract");
      contract = await contractFactory.connect(signer).deploy();
      await contract.waitForDeployment();
      contractAddress = await contract.getAddress();

      console.log(`Deployed contract at ${contractAddress}`);

      fs.writeFileSync(contractCacheFile, JSON.stringify({ contractAddress }, null, 2), 'utf-8');
      console.log(`Saved cache to ${contractCacheFile}`);
    }
  
    // Initialize fresh random values on-chain
    console.log(`Creating ciphertexts:`);
    for (let i = 0; i < taskArgs.nBatch; i++) {
      console.log(`Starting batch ${i}`);
      const tx = await contract.refresh(taskArgs.decryptionsPerBatch, i);
      await tx.wait();
      console.log(`Batch ${i} completed`);
    }

    // Get handles from all batches
    const handles: string[][] = [];
    for (let i = 0; i < taskArgs.nBatch; i++) {
      const values = await contract.getValuesFromBatch(i);
      handles[i] = values;
    }

    // Validate number of handles
    const allHandles = handles.flat();
    console.assert(allHandles.length == totalDecryptions, `on-chain-handles=${allHandles.length} != ${totalDecryptions}=decryptions-requested`);

    console.log("All handles submitted");

    if (taskArgs.debugLogs) {
      console.debug("All handles:", allHandles);
    }

    // Wait for all handles to be ready for public decryption
    while (await hasFunctionReverted(decryptionContractAddress, `checkPublicDecryptionReady(bytes32[])`, [allHandles], provider, ethers)) {
      console.log("Waiting for handles to be ready for public decryption");
      await sleep(1000);
    }
    console.log("All handles are ready for public decryption");

    // Leverage cache of relayer to retry until all promises resolve
    console.log("Public decrypting ...");
    console.time("decryptions");
    let decrypted_values: DecryptedResults[] | null;
    let counter = 0;
    while (true) {
      console.time("decryptions-inner");
      let decrypted_values_promises: Promise<DecryptedResults>[] = [];
      for (let i = 0; i < taskArgs.nBatch; i++) {
        for (let j = 0; j < taskArgs.decryptionsPerBatch; j++) {

          // Request a public decryption for the handle
          decrypted_values_promises.push(instance.publicDecrypt([handles[i][j]]));
          
          if (taskArgs.sleepBetweenDecryptions > 0 && taskArgs.decryptionsPerBatch > 1) {
            console.log(`Sleeping ${taskArgs.sleepBetweenDecryptions}ms between decryptions`);
            await sleep(taskArgs.sleepBetweenDecryptions);
          }
        }
        if (taskArgs.sleepBetweenBatches > 0 && taskArgs.nBatch > 1) {
          console.log(`Sleeping ${taskArgs.sleepBetweenBatches}ms between batches`);
          await sleep(taskArgs.sleepBetweenBatches);
        }
      }

      // monitor promises states
      const promiseStates = new Array(decrypted_values_promises.length).fill('pending');
      decrypted_values_promises.forEach((p, i) => {
        p.then(() => { promiseStates[i] = 'fulfilled' })
          .catch(() => { promiseStates[i] = 'rejected' });
      });


      try {
        console.log(`Waiting for all public decryption promises to resolve within ${timeoutMs}ms`)
        try {
          decrypted_values = await Promise.race([Promise.all(decrypted_values_promises), timeout(timeoutMs)]);
          console.timeEnd("decryptions-inner");
        } catch (timeoutError) {
          console.log("Counting fulfilled decryptions.");
          const fulfilled = promiseStates.filter(s => s === 'fulfilled').length;
          console.log(`Timeout after ${timeoutMs}ms: ${fulfilled}/${decrypted_values_promises.length} promises already resolved`);
          throw timeoutError;
        }
        break;
      } catch (error) {
        counter += 1;
        console.error(error);
        console.log(`Retrying public decryptions attempt: ${counter}`);
        console.timeEnd("decryptions-inner");

        if (taskArgs.debugLogs) {
          console.debug("Waiting 1s before retrying");
        }
        
        // To avoid spamming too much the relayer
        await sleep(1000);
      }
    }
    console.timeEnd("decryptions");

    if (decrypted_values) {
      console.log("All decryption promises resolved");

      if (taskArgs.debugLogs) {
        console.debug(decrypted_values);
      }
    }
  });
