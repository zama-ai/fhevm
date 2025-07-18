import { task, types } from "hardhat/config";
import { createInstance as createFhevmInstance, SepoliaConfig, FhevmInstanceConfig, DecryptedResults } from '@zama-fhe/relayer-sdk/node';
import { HardhatRuntimeEnvironment } from 'hardhat/types';
import { EncryptedERC20 } from '../types';
import fs from 'fs';
import path from 'path';

function sleep(ms: number) {
  console.debug(`Waiting ${ms}ms`)
  return new Promise(resolve => setTimeout(resolve, ms));
}

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

    let contract: EncryptedERC20;
    let contractAddress: string;
    if (taskArgs.cerc20Address == null) {
      console.info("Deploying cERC-20 contract");
      const contractFactory = await ethers.getContractFactory('EncryptedERC20');
      contract = await contractFactory.connect(signer).deploy('Naraggara', 'NARA'); // City of Zama's battle
      await contract.waitForDeployment();
      contractAddress = await contract.getAddress();
    } else {
      console.info(`Using pre-deployed cERC-20 contract at: ${taskArgs.cerc20Address}`);
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
    }
    // Validate mint was succesful
    const encrypted_balance = await contract.balanceOf(signer);

    console.info(`Deciphering cERC-20 balance: ${encrypted_balance}`);
    const balanceAliceResults = await instance.userDecrypt([{ handle: encrypted_balance, contractAddress: contractAddress }], privateKey, publicKey, aliceEIPSignature, [contractAddress], signer.address, taskArgs.eip712Timestamp, durationDays);
    const balanceAlice = balanceAliceResults[encrypted_balance];
    console.log(`Alice's cERC-20 balance: ${balanceAlice}`);

    // Create encrypted input euint64(1)
    console.info("Creating encrypted input");
    let input = instance.createEncryptedInput(contractAddress, signer.address);
    input.add64(1);
    const encryptedTransferAmount = await input.encrypt();

    // Transfer 1 token to each derived wallet
    if (taskArgs.transfer) {
      console.info("Transfering cERC-20 tokens");
      let txs = [];
      for (let index = 1; index < taskArgs.mintAmount + 1; index++) {
        const tx = await contract['transfer(address,bytes32,bytes)'](
          signers[index].address,
          encryptedTransferAmount.handles[0],
          encryptedTransferAmount.inputProof,
        );
        txs.push(tx.wait());
      }
      const transfers = await Promise.all(txs);
      console.log(`Done: ${JSON.stringify(transfers)}`);
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

      // Leverage cache of relayer to retry until all promises resolve
      console.time("decrytions");
      let decrypted_balances: DecryptedResults[];
      let counter = 0;
      while (true) {
        let decrypted_balances_promises = [];
        for (let index = 0; index < taskArgs.mintAmount + 1; index++) {

          decrypted_balances_promises.push(instance.userDecrypt([{ handle: balancesHandles[index], contractAddress: contractAddress }], privateKey, publicKey, eip712Signatures[index], [contractAddress], signers[index].address, taskArgs.eip712Timestamp, durationDays));
          await sleep(taskArgs.sleepBetweenDecryptions);
        }
        try {
          decrypted_balances = await Promise.all(decrypted_balances_promises);
          break;
        } catch (error) {
          console.error(error);
          console.log(`Retrying user decryptions attempt: ${counter}`);
          counter += 1;
        }

      }
      console.timeEnd("decrytions");
      for (const [result, handle] of zip(decrypted_balances, balancesHandles)) {
        console.log(result[handle]);
      }
      console.log(decrypted_balances);
    };
  });
