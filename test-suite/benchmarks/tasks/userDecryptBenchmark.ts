import { task, types } from "hardhat/config";
import { DecryptedResults } from '@zama-fhe/relayer-sdk/node';
import { HardhatRuntimeEnvironment } from 'hardhat/types';
import { UserDecryptBenchmark } from '../types';
import { createInstance, KeyPair, sleep, hasFunctionReverted, timeout } from './shared';
import { resolve, join } from 'path';
import fs from 'fs';

const cacheDir = resolve(__dirname, '../decryption_cache');
if (!fs.existsSync(cacheDir)) {
  fs.mkdirSync(cacheDir, { recursive: true });
}

interface TaskArgs {
  decryptionsPerBatch: number,
  nBatch: number,
  sleepBetweenBatches: number,
  sleepBetweenDecryptions: number,
  deployNewContract: boolean,
  generateNewHandles: boolean,
  debugLogs: boolean,
}

task("user-decrypt-benchmark", `User-Decryption Benchmark.

This script does the following:
- Deploy contract if cache is not used or does not exist yet
- Load Decryptions x BatchNumber random euint64 from cache or generate new ones on-chain
- Wait until all values are ready to be decrypted (ACL propagation and Ciphertext Add)
- User decrypt all values
`)
  .addParam("decryptionsPerBatch", "Number of decryptions to do per batch", 10, types.int, true)
  .addParam("nBatch", "Number of batches to do", 1, types.int, true)
  .addParam("sleepBetweenDecryptions", "Sleep (milliseconds) between decryptions", 0, types.int, true)
  .addParam("sleepBetweenBatches", "Sleep (milliseconds) between batches", 0, types.int, true)
  .addParam("deployNewContract", "Deploy a new contract. Use cache if false", false, types.boolean, true)
  .addParam("generateNewHandles", "Generate new handles. Use cache if false", true, types.boolean, true)
  .addParam("debugLogs", "Display debug logs", false, types.boolean, true)
  .setAction(async (taskArgs: TaskArgs, hre: HardhatRuntimeEnvironment) => {
    const { ethers, network } = hre;
    const signers = await ethers.getSigners();
    const signer = signers[0];
    const userAddress = signer.address;
    console.log("Network:", network.name);
    console.log("Signer:", signer.address);
    console.log("User address:", userAddress);

    const totalDecryptions = taskArgs.decryptionsPerBatch * taskArgs.nBatch;
    console.log(`Total number of decryptions: ${totalDecryptions} (${taskArgs.decryptionsPerBatch} x ${taskArgs.nBatch})`);
  
    // Currently, a single user decryption takes ~ 1.3s, so we set a timeout a little higher:
    // - 60s for taking into account the latency for small number of decryptions
    // - 2s per decryption
    const timeoutMs = 60000 + totalDecryptions * 2000; 
    console.log(`Total decryption timeout: ${timeoutMs}ms`);

    const instance = await createInstance(network, taskArgs.debugLogs);

    let contract: UserDecryptBenchmark;
    let contractAddress: string;
    let ctHandleContractPairsPerBatch: { handle: string, contractAddress: string }[][];

    // User contract address cache file path
    const contractCacheFile = join(cacheDir, `user_contract.json`);
    
    const contractFactory = await ethers.getContractFactory('UserDecryptBenchmark');

    // Load contract address
    if (!taskArgs.deployNewContract && fs.existsSync(contractCacheFile)) {
      const userContractCache = JSON.parse(fs.readFileSync(contractCacheFile, 'utf-8'));

      console.info("Using pre-deployed contract");
      contractAddress = userContractCache.contractAddress;
      contract = contractFactory.attach(contractAddress).connect(signer) as UserDecryptBenchmark;
      
      console.log(`Loaded contract address from ${contractCacheFile}`);
    }
    else {
      console.info("Deploying new contract");
      contract = await contractFactory.connect(signer).deploy();
      await contract.waitForDeployment();
      contractAddress = await contract.getAddress();

      fs.writeFileSync(contractCacheFile, JSON.stringify({ contractAddress }, null, 2), 'utf-8');
      console.log(`Saved contract address to ${contractCacheFile}`);
    }
    
    console.info(`Using contract deployed at ${contractAddress}`);

    // Cache file path
    const handlesCacheFile = join(cacheDir, `user_${taskArgs.decryptionsPerBatch}_${taskArgs.nBatch}.json`);
    
    // Load handles from cache if it exists
    if (!taskArgs.generateNewHandles && fs.existsSync(handlesCacheFile)) {
      const handlesCache = JSON.parse(fs.readFileSync(handlesCacheFile, 'utf-8'));

      // Load ctHandleContractPairsPerBatch
      ctHandleContractPairsPerBatch = handlesCache.ctHandleContractPairsPerBatch;

      console.log(`Loaded cache from ${handlesCacheFile}`);
    } else {
      const decryptionContractAddress = process.env.DECRYPTION_CONTRACT_ADDRESS;
      if (!decryptionContractAddress) {
        throw new Error("DECRYPTION_CONTRACT_ADDRESS is not set");
      }
      const rpcUrl = process.env.RPC_URL;
      if (!rpcUrl) {
        throw new Error("RPC_URL is not set");
      }
      const provider = new ethers.JsonRpcProvider(rpcUrl);

      // Initialize fresh random values on-chain
      console.log(`Creating ciphertexts:`);
      for (let i = 0; i < taskArgs.nBatch; i++) {
        console.log(`Starting batch ${i}`);
        const tx = await contract.refresh(taskArgs.decryptionsPerBatch, i);
        await tx.wait();
        console.log(`Batch ${i} completed`);
      }

      // Get handles from all batches and create (handle, contractAddress) pairs
      const allHandles: string[] = [];
      ctHandleContractPairsPerBatch = [];
      for (let i = 0; i < taskArgs.nBatch; i++) {
        const values = await contract.getValuesFromBatch(i);
        allHandles.push(...values);
        ctHandleContractPairsPerBatch.push(values.map((handle) => ({ handle: handle, contractAddress: contractAddress })));
      }

      // Validate number of handles
      console.assert(allHandles.length == totalDecryptions, `on-chain-handles=${allHandles.length} != ${totalDecryptions}=decryptions-requested`);

      console.log("All handles submitted");

      if (taskArgs.debugLogs) {
        console.debug("ctHandleContractPairs:", ctHandleContractPairsPerBatch);
      }

      // Make a list of 2-tuples [handle, contractAddress] for the function call
      const allCtHandleContractPairs: [string, string][] = allHandles.map(handle => [handle, contractAddress] as [string, string]);

      // Wait for all handles to be ready for user decryption
      while (await hasFunctionReverted(decryptionContractAddress, `checkUserDecryptionReady(address,(bytes32,address)[])`, [userAddress, allCtHandleContractPairs], provider, ethers)) {
        console.log("Waiting for handles to be ready for user decryption");
        await sleep(1000);
      }
      console.log("All handles are ready for user decryption");

      fs.writeFileSync(handlesCacheFile, JSON.stringify({ ctHandleContractPairsPerBatch }, null, 2), 'utf-8');
      console.log(`Saved cache to ${handlesCacheFile}`);
    }

    // Create the EIP-712 signature
    console.debug("Generating keypair");
    const keypair: KeyPair = instance.generateKeypair();
    const publicKey = keypair.publicKey;
    const privateKey = keypair.privateKey;
    const durationDays = '100'; // String for consistency
    const currentTimestamp = Math.floor(Date.now() / 1000).toString();
    const contractAddresses = [contractAddress];
    const eip712 = instance.createEIP712(publicKey, contractAddresses, currentTimestamp, durationDays);
    const eip712Signature = await signer.signTypedData(
      eip712.domain,
      {
        UserDecryptRequestVerification: eip712.types.UserDecryptRequestVerification,
      },
      eip712.message,
    );

    // Leverage cache of relayer to retry until all promises resolve
    console.log("User decrypting ...");
    console.time("decryptions");
    let decrypted_values: DecryptedResults[] | null;
    let counter = 0;
    while (true) {
      console.time("decryptions-inner");
      let decrypted_values_promises: Promise<DecryptedResults>[] = [];
      for (let i = 0; i < taskArgs.nBatch; i++) {
        for (let j = 0; j < taskArgs.decryptionsPerBatch; j++) {

          // Request a user decryption for the handle and contract address
          decrypted_values_promises.push(instance.userDecrypt([ctHandleContractPairsPerBatch[i][j]], privateKey, publicKey, eip712Signature, [contractAddress], signer.address, currentTimestamp, durationDays));
          
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
        console.log(`Waiting for all user decryption promises to resolve within ${timeoutMs}ms`)
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
        console.log(`Retrying user decryptions attempt: ${counter}`);
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
      console.log("All decryption promises resolved")

      if (taskArgs.debugLogs) {
        console.debug(decrypted_values);
      }
    }
  });
