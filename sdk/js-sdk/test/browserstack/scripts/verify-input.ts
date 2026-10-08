import { setFhevmRuntimeConfig, createFhevmEncryptClient } from '../../../src/ethers/index.js';
import { sepolia } from '../../../src/core/chains/index.js';
import { ethers } from 'ethers';
import { createLogger, createPageHarness, fetchFheTestAddress } from './common.js';
import { FHETestABI } from '../../fheTest/FheTest-abi-v2.js';

const { log, done, elapsed } = createPageHarness();

async function run() {
  const mnemonic = import.meta.env.MNEMONIC;

  if (!mnemonic) {
    log('[SKIP] MNEMONIC not set — cannot submit on-chain tx');
    done('fail');
    return;
  }

  try {
    log('Setting runtime config...');
    setFhevmRuntimeConfig({
      logger: createLogger(log),
    });
    log('[PASS] Runtime config set');

    //
    // 1. Resolve the Sepolia FHETest contract address
    //
    log('Resolving FHETest contract address...');
    const FHE_TEST_ADDRESS = await fetchFheTestAddress('sepolia');
    log(`[PASS] FHETest address: ${FHE_TEST_ADDRESS}`);

    //
    // 2. Derive wallet from mnemonic
    //
    log('Deriving wallet from mnemonic...');
    const provider = new ethers.JsonRpcProvider('https://ethereum-sepolia-rpc.publicnode.com');
    const wallet = ethers.HDNodeWallet.fromMnemonic(ethers.Mnemonic.fromPhrase(mnemonic)).connect(provider);
    log(`[PASS] Wallet address: ${wallet.address}`);

    //
    // 3. Create encrypt client
    //
    log('Creating encrypt client...');
    const client = createFhevmEncryptClient({
      chain: sepolia,
      provider,
    });
    log('[PASS] Encrypt client created');

    //
    // 4. Init (loads TFHE WASM + fetches global FHE key)
    //
    log('Initializing...');
    await client.init();
    log('[PASS] Client initialized');

    //
    // 5. Encrypt a uint8 value
    //
    const testValue = 42;
    log(`Encrypting uint8 value (${testValue})...`);
    const result = await client.encryptValue({
      contractAddress: FHE_TEST_ADDRESS,
      userAddress: wallet.address,
      value: { type: 'uint8', value: testValue },
    });

    log(`  encryptedValue: ${result.encryptedValue.slice(0, 20)}...`);
    log(`  inputProof: ${result.inputProof.slice(0, 40)}... (${result.inputProof.length} chars)`);
    log('[PASS] Encryption complete');

    //
    // 6. Submit encrypted input to FHETest.addEuint8() on-chain
    //    This internally calls TFHE.asEuint8(inputHandle, inputProof)
    //    which verifies the ZK proof on-chain
    //
    log('Submitting input to FHETest.addEuint8() on-chain...');
    const fheTestContract = new ethers.Contract(FHE_TEST_ADDRESS, FHETestABI, wallet);
    const tx = await fheTestContract.getFunction('addEuint8')(
      result.encryptedValue,
      result.inputProof,
      testValue, // clearValue: the contract tracks the expected cleartext alongside the handle
      false, // makePublic
    );

    log(`  tx hash: ${tx.hash}`);
    log('Waiting for confirmation...');
    const receipt = await tx.wait();

    if (!receipt || receipt.status !== 1) {
      throw new Error(`Transaction failed: status=${receipt?.status}`);
    }

    log(`  block: ${receipt.blockNumber}, gasUsed: ${receipt.gasUsed.toString()}`);
    log('[PASS] On-chain input verification succeeded');

    log(`\nAll verify-input checks passed in ${elapsed()}ms`);
    done('pass');
  } catch (err) {
    log(`[FAIL] ${err}`);
    done('fail');
  }
}

run();
