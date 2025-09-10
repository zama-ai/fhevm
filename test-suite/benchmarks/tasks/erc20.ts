import { task, types } from "hardhat/config";
import { createInstance as createFhevmInstance, SepoliaConfig, FhevmInstanceConfig, DecryptedResults } from '@zama-fhe/relayer-sdk/node';
import { HardhatRuntimeEnvironment, Network } from 'hardhat/types';
import { EncryptedERC20 } from '../types';
import fs from 'fs';
import path from 'path';

function sleep(ms: number) {
  console.debug(`Waiting ${ms}ms`)
  return new Promise(resolve => setTimeout(resolve, ms));
}

const timeout = (ms: number): Promise<null> =>
  new Promise((_, reject) =>
    setTimeout(() => reject(new Error('Operation timed out')), ms)
  );

const TIMEOUT = 30_000;

// https://gist.github.com/chrismilson/e6549023bdca1fa9c263973b8f7a713b
type Iterableify<T> = { [K in keyof T]: Iterable<T[K]> }
function* zip<T extends Array<any>>(
  ...toZip: Iterableify<T>
): Generator<T> {
  // Get iterators for all of the iterables.
  const iterators = toZip.map(i => i[Symbol.iterator]())

  while (true) {
    // Advance all of the iterators.
    const results = iterators.map(i => i.next())

    // If any of the iterators are done, we should stop.
    if (results.some(({ done }) => done)) {
      break
    }

    // We can assert the yield type, since we know none
    // of the iterators are done.
    yield results.map(({ value }) => value) as T
  }
}

interface TaskArgs {
  cerc20Address?: string;
  mint: boolean,
  transfer: boolean,
  decrypt: boolean,
  eip712Timestamp: string,
  mintAmount: number,
  sleepBetweenDecryptions: number,
  reuseKeypair: boolean,
}

interface KeyPair {
  publicKey: string,
  privateKey: string,
}

async function waitAllPromises(promises: Promise<any>[], ms: number): Promise<any[]> {
  console.log("Waiting for all promises to resolve")
  const promiseStates = new Array(promises.length).fill('pending');
  promises.forEach((p, i) => {
    p.then(() => { promiseStates[i] = 'fulfilled' })
      .catch(() => { promiseStates[i] = 'rejected' });
  });

  try {
    while (true) {
      try {
        let res: any = await Promise.race([Promise.all(promises), timeout(ms)]);
        return res;
      } catch (timeoutError) {
        const fulfilled = promiseStates.filter(s => s === 'fulfilled').length;
        const pendingIndexes = promiseStates
          .map((state, index) => state === 'pending' ? index : -1)
          .filter(index => index !== -1);
        console.log(`Timeout after ${ms}ms: ${fulfilled}/${promises.length} promises already resolved`);
        console.log(`Pending promise indexes: ${pendingIndexes.join(', ')}`);
      }
    }
  } catch (error) {
    const fulfilled = promiseStates.filter(s => s === 'fulfilled').length;
    const pendingIndexes = promiseStates
      .map((state, index) => state === 'pending' ? index : -1)
      .filter(index => index !== -1);
    console.log(`Error: ${fulfilled}/${promises.length} promises already resolved`);
    console.log(`Pending promise indexes: ${pendingIndexes.join(', ')}`);
    throw error;
  }
}

async function create_instance(network: Network) {
  // Default to Sepolia configuration
  const fhevmConfig: FhevmInstanceConfig = { ...SepoliaConfig, network: network.provider, chainId: network.config.chainId };

  // Conditionally set properties only if env vars exist
  if (process.env.CHAIN_ID_GATEWAY) {
    fhevmConfig.gatewayChainId = parseInt(process.env.CHAIN_ID_GATEWAY);
  }
  if (process.env.RELAYER_URL) {
    fhevmConfig.relayerUrl = process.env.RELAYER_URL;
  }
  if (process.env.DECRYPTION_ADDRESS) {
    fhevmConfig.verifyingContractAddressDecryption = process.env.DECRYPTION_ADDRESS;
  }
  if (process.env.INPUT_VERIFICATION_ADDRESS) {
    fhevmConfig.verifyingContractAddressInputVerification = process.env.INPUT_VERIFICATION_ADDRESS;
  }
  if (process.env.KMS_VERIFIER_CONTRACT_ADDRESS) {
    fhevmConfig.kmsContractAddress = process.env.KMS_VERIFIER_CONTRACT_ADDRESS;
  }
  if (process.env.INPUT_VERIFIER_CONTRACT_ADDRESS) {
    fhevmConfig.inputVerifierContractAddress = process.env.INPUT_VERIFIER_CONTRACT_ADDRESS;
  }
  if (process.env.ACL_CONTRACT_ADDRESS) {
    fhevmConfig.aclContractAddress = process.env.ACL_CONTRACT_ADDRESS;
  }

  console.log(`Using configuration: ${JSON.stringify(fhevmConfig)}`);

  const instance = await createFhevmInstance(fhevmConfig);
  console.debug(JSON.stringify(fhevmConfig));
  return instance;
}

task("cerc-20-multi-transfer-decrypt", `cERC-20 multi-transfer and decryptions script.

This script does the following:
- Mint X tokens from cERC-20 contract
- Transfer 1 token to X derived wallets
- Decrypt the balance of all wallets used (i.e. X+1 decryptions)

This script makes the following optional:
- Deploying the cERC-20 contract (if one wants to use an already deployed contract)
- Minting the cERC-20 tokens (if one already minted the tokens)
- Transfering funds (if one already transferred funds)
- Generating Keypair (if a \`keypair.json\` file is already present)
- Decrypting the balances

NOTE: to benefit from request-caching in the Relayer, one must use *both* the same timestamp and keypair.

NOTE: one constraint is that the transfers should fit within the HCU limit
as batched transfers is not yet implemented in this script.
`)
  .addParam("cerc20Address", "Already deployed cERC-20 address", undefined, types.string, true)
  .addParam("mint", "Whether to mint cERC-20 tokens (required if newly deployed contract)", true, types.boolean, true)
  .addParam("transfer", "Whether to transfer funds to derived wallets (required if newly deployed contract)", true, types.boolean, true)
  .addParam("decrypt", "Whether to decrypt encrypted balances of all derived wallets", true, types.boolean, true)
  .addParam("eip712Timestamp", "EIP-712 Timestamp", Math.floor(Date.now() / 1000).toString(), types.string, true)
  .addParam("mintAmount", "Mint amount and number of transfers to do", 10, types.int, true)
  .addParam("sleepBetweenDecryptions", "Sleep (milliseconds) between decryptions", 0, types.int, true)
  .addParam("reuseKeypair", "Whether to re-use the keypair between scripts launches", false, types.boolean, true)
  .setAction(async (taskArgs: TaskArgs, hre: HardhatRuntimeEnvironment) => {
    const { ethers, network } = hre;
    const signers = await ethers.getSigners();
    const signer = signers[0];
    console.log("Network:", network.name);
    console.log("Signer:", signer.address);

    const instance = await create_instance(network);

    // Deploy benchmark cERC-20 contract (or not)
    let contract: EncryptedERC20;
    let contractAddress: string;
    if (taskArgs.cerc20Address == null) {
      console.info("Deploying cERC-20 contract");
      const contractFactory = await ethers.getContractFactory('EncryptedERC20');
      contract = await contractFactory.connect(signer).deploy('Naraggara', 'NARA'); // City of Zama's battle
      await contract.waitForDeployment();
      contractAddress = await contract.getAddress();
    } else {
      console.info("Using pre-deployed cERC-20");
      const contractFactory = await ethers.getContractFactory('EncryptedERC20');
      contract = contractFactory.attach(taskArgs.cerc20Address).connect(signer) as EncryptedERC20;
      contractAddress = taskArgs.cerc20Address;
    }
    console.info(`Using cERC-20 deployed at ${contractAddress}`);
    console.info(`cERC-20 contract name: ${await contract.name()}`);
    console.info(`cERC-20 contract owner: ${await contract.owner()}`);

    // Boiler-plate for user-decryption
    let publicKey: string;
    let privateKey: string;
    const filePath = path.join(__dirname, 'keypair.json');
    // We might want to re-use an already generated keypair
    if (fs.existsSync(filePath) && taskArgs.reuseKeypair) {
      console.debug("Loading serialized keypair");
      const fileContent = fs.readFileSync(filePath, 'utf-8');
      const keypair = JSON.parse(fileContent) as KeyPair;
      publicKey = keypair.publicKey;
      privateKey = keypair.privateKey;
    } else {
      console.debug("Generating and saving keypair");
      const keypair = instance.generateKeypair();
      publicKey = keypair.publicKey;
      privateKey = keypair.privateKey;
      fs.writeFileSync(filePath, JSON.stringify({ publicKey: publicKey, privateKey: privateKey }, null, 2));
    }

    // EIP-712
    const durationDays = '100'; // String for consistency
    const contractAddresses = [contractAddress];
    const eip712 = instance.createEIP712(publicKey, contractAddresses, taskArgs.eip712Timestamp, durationDays);
    const aliceEIPSignature = await signer.signTypedData(
      eip712.domain,
      {
        UserDecryptRequestVerification: eip712.types.UserDecryptRequestVerification,
      },
      eip712.message,
    );

    // Mint
    if (taskArgs.mint) {
      console.info("Minting cERC-20 contract");
      const transaction = await contract.mint(taskArgs.mintAmount);
      await transaction.wait();

      // Validate mint was succesful
      const encrypted_balance = await contract.balanceOf(signer);
      console.info(`Deciphering cERC-20 balance: ${encrypted_balance}`);
      const balanceAliceResults = await instance.userDecrypt([{ handle: encrypted_balance, contractAddress: contractAddress }], privateKey, publicKey, aliceEIPSignature, [contractAddress], signer.address, taskArgs.eip712Timestamp, durationDays);
      const balanceAlice = balanceAliceResults[encrypted_balance];
      console.log(`Alice's cERC-20 balance: ${balanceAlice}`);
    }

    // Transfer 1 token to each derived wallet
    if (taskArgs.transfer) {
      // Create encrypted input euint64(1)
      console.info("Creating encrypted input");
      let input = instance.createEncryptedInput(contractAddress, signer.address);
      input.add64(1);
      const encryptedTransferAmount = await input.encrypt();

      // Create an array of async transfer tasks
      const transferTasks = [];

      for (let index = 1; index < taskArgs.mintAmount + 1; index++) {

        // Create an async function for each transfer
        const transferTask = async () => {
          console.log(`Starting processing for signer: ${index}`);
          let internalCounter = 0;
          while (true) {
            try {

              console.log(`Broadcasting transaction (${index}/${taskArgs.mintAmount}, retry:${internalCounter})`);
              const tx = await Promise.race([contract['transfer(address,bytes32,bytes)'](
                signers[index].address,
                encryptedTransferAmount.handles[0],
                encryptedTransferAmount.inputProof,
              ), timeout(TIMEOUT)]);
              if (tx == null) {
                continue;
              }
              console.log(`Waiting transaction (${index}/${taskArgs.mintAmount}, retry:${internalCounter}): ${tx.hash}`);
              await tx.wait(1, 20_000); // Wait 1 block for confirmation, 20s for timeout
              console.log(`Completed transaction (${index}/${taskArgs.mintAmount}, retry:${internalCounter}): ${tx.hash}`);
              return tx; // Return the transaction for potential later use
            } catch (error) {
              internalCounter += 1;
              console.error(`Caught error: ${error}, retrying transfer (${index}/${taskArgs.mintAmount}), retry number: ${internalCounter}`);
              // Continue will restart the while loop to retry
              continue;
            }
          }
        };

        // Start the task immediately and add the promise to the array
        transferTasks.push(transferTask());
        await sleep(1_000);
      }

      // Wait for all transfers to complete
      const completedTransactions: any[] = await waitAllPromises(transferTasks, 5_000);
      console.info(`All ${completedTransactions.length} transfers completed`);
    };

    // Run multiple concurrent decryptions
    if (taskArgs.decrypt) {
      console.info("Decrypting cERC-20 balances");
      let balancesHandles: string[] = [];
      let eip712Signatures: string[] = []

      for (let index = 0; index < taskArgs.mintAmount + 1; index++) {
        let encrypted_balance = await contract.balanceOf(signers[index].address);
        balancesHandles.push(encrypted_balance);
        const signature = await signers[index].signTypedData(
          eip712.domain,
          {
            UserDecryptRequestVerification: eip712.types.UserDecryptRequestVerification,
          },
          eip712.message,
        );
        eip712Signatures.push(signature);
      };
      console.debug(`handles: ${JSON.stringify(balancesHandles, null, "\t")}`);

      // Leverage cache of relayer to retry until all promises resolve
      console.time("decrytions");
      let decrypted_balances: DecryptedResults[] | null;
      let counter = 0;
      while (true) {
        console.time("decrytions-inner");
        let decrypted_balances_promises: Promise<DecryptedResults>[] = [];
        for (let index = 0; index < taskArgs.mintAmount + 1; index++) {

          decrypted_balances_promises.push(instance.userDecrypt([{ handle: balancesHandles[index], contractAddress: contractAddress }], privateKey, publicKey, eip712Signatures[index], [contractAddress], signers[index].address, taskArgs.eip712Timestamp, durationDays));
          await sleep(taskArgs.sleepBetweenDecryptions);
        }

        // monitor promises states
        const promiseStates = new Array(decrypted_balances_promises.length).fill('pending');
        decrypted_balances_promises.forEach((p, i) => {
          p.then(() => { promiseStates[i] = 'fulfilled' })
            .catch(() => { promiseStates[i] = 'rejected' });
        });


        try {
          console.log(`Waiting for all decryption promises to resolve within ${TIMEOUT}ms`)
          try {
            decrypted_balances = await Promise.race([Promise.all(decrypted_balances_promises), timeout(TIMEOUT)]);
            console.timeEnd("decrytions-inner");
          } catch (timeoutError) {
            console.log("Counting fulfilled decryptions.");
            const fulfilled = promiseStates.filter(s => s === 'fulfilled').length;
            console.log(`Timeout after ${TIMEOUT}ms: ${fulfilled}/${decrypted_balances_promises.length} promises already resolved`);
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

      if (decrypted_balances) {
        console.log("All decryption promises resolved")
        for (const [result, handle] of zip(decrypted_balances, balancesHandles)) {
          console.log(result[handle]);
        }
        console.log(decrypted_balances);
      }
    };
  });
