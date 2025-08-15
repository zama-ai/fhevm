import { createInstance as createFhevmInstance, SepoliaConfig, FhevmInstanceConfig } from '@zama-fhe/relayer-sdk/node';
import { Network } from 'hardhat/types';

export function sleep(ms: number) {
  console.debug(`Waiting ${ms}ms`)
  return new Promise(resolve => setTimeout(resolve, ms));
}

export const timeout = (ms: number): Promise<null> =>
  new Promise((_, reject) =>
    setTimeout(() => reject(new Error('Operation timed out')), ms)
  );


// https://gist.github.com/chrismilson/e6549023bdca1fa9c263973b8f7a713b
export type Iterableify<T> = { [K in keyof T]: Iterable<T[K]> }
export function* zip<T extends Array<any>>(
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

export async function waitAllPromises(promises: Promise<any>[], ms: number): Promise<any[]> {
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

export async function createInstance(network: Network, debugLogs: boolean = false) {
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

  
  if (debugLogs) {
    console.debug(`Using configuration: ${JSON.stringify(fhevmConfig)}`);
  }
  const instance = await createFhevmInstance(fhevmConfig);
  return instance;
}


export interface KeyPair {
  publicKey: string,
  privateKey: string,
}

/**
 * Checks if a contract function reverts when called with the given inputs.
 * @param contractAddress The address of the contract.
 * @param functionSignature The function signature, e.g. "myFunction(uint256,address)".
 * @param inputs The array of input values to encode and pass to the function.
 * @param provider The ethers provider.
 * @param ethers The ethers object.
 * @returns true if the function call reverts, false otherwise.
 */
export async function hasFunctionReverted(
  contractAddress: string,
  functionSignature: string,
  inputs: any[],
  provider: any,
  ethers: any
): Promise<boolean> {
  try {
    // Get the ABI fragment for encoding
    const functionInterface = [`function ${functionSignature}`];
    const contractPartialInterface = new ethers.Interface(functionInterface);

    // Encode the function data with inputs
    const data = contractPartialInterface.encodeFunctionData(functionSignature.split('(')[0], inputs);

    // Call the function and check if it reverts
    await provider.call({
      to: contractAddress,
      data: data
    });
    return false;
  } catch (error) {
    return true;
  }
}