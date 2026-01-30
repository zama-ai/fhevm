import type { HardhatEthersSigner } from '@nomicfoundation/hardhat-ethers/signers';
import type { Provider, TransactionReceipt, TransactionResponse } from 'ethers';
import { ethers, network } from 'hardhat';
import assert from 'node:assert/strict';

import { aclAddress, coprocessorAddress, createInstance, kmsVerifierAddress } from '../test/instance';
import { userDecryptSingleHandle } from '../test/utils';
import type { SmokeTestInput } from '../types';

type FeeData = {
  maxFeePerGas: bigint;
  maxPriorityFeePerGas: bigint;
};

type TxOverrides = {
  gasLimit?: bigint;
  maxFeePerGas: bigint;
  maxPriorityFeePerGas: bigint;
  nonce: number;
};

type SignerState = {
  signer: HardhatEthersSigner;
  index: number;
  address: string;
  latest: number;
  pending: number;
  balance: bigint;
};

const DEFAULT_SIGNER_INDICES = '0,1,2';
const DEFAULT_TX_MAX_RETRIES = 2;
const DEFAULT_MAX_BACKLOG = 3;
const TIME_PER_BLOCK = 12;
const NUMBER_OF_BLOCKS = 4;
const DEFAULT_TX_TIMEOUT_SECS = TIME_PER_BLOCK * NUMBER_OF_BLOCKS;
const DEFAULT_FEE_BUMP = 1.125 ** NUMBER_OF_BLOCKS;
const DEFAULT_DECRYPT_TIMEOUT_SECS = 120;

const MIN_PRIORITY_FEE = ethers.parseUnits('2', 'gwei');
const CANCEL_GAS_LIMIT = 21_000n;
const LOW_BALANCE_THRESHOLD = ethers.parseEther('0.1');
const SMOKE_GAS_ESTIMATE = 1_000_000n; // ~1M gas covers deploy + call with buffer
const RECEIPT_POLL_MS = 4_000;

const withTimeout = <T>(promise: Promise<T>, timeoutMs: number, label: string): Promise<T> => {
  let timer: NodeJS.Timeout;
  const timeoutPromise = new Promise<never>((_, reject) => {
    timer = setTimeout(() => reject(new Error(`Timeout after ${timeoutMs}ms: ${label}`)), timeoutMs);
  });
  return Promise.race([
    promise.then((result) => {
      clearTimeout(timer);
      return result;
    }),
    timeoutPromise,
  ]);
};

const parseIndices = (value: string): number[] => {
  const indices = value
    .split(',')
    .map((entry) => Number.parseInt(entry.trim(), 10))
    .filter((entry) => Number.isFinite(entry) && entry >= 0);
  if (indices.length === 0) {
    throw new Error(`SMOKE_SIGNER_INDICES resolved to no valid indices (value: "${value}")`);
  }
  return indices;
};

const parseBooleanEnv = (name: string, defaultValue: boolean): boolean => {
  const value = process.env[name];
  if (value === undefined) return defaultValue;
  if (value === '1') return true;
  if (value === '0') return false;
  throw new Error(`${name} must be '0' or '1' (got: "${value}")`);
};

const formatSignerState = (state: SignerState): string => {
  const balanceStr = ethers.formatEther(state.balance);
  const lowBalanceWarning = state.balance < LOW_BALANCE_THRESHOLD ? ' ⚠️ LOW_BALANCE' : '';
  return `index=${state.index} address=${state.address} latest=${state.latest} pending=${state.pending} balance=${balanceStr} ETH${lowBalanceWarning}`;
};

const bumpFees = (base: FeeData, multiplier: number): FeeData => {
  const scaled = BigInt(Math.round(multiplier * 100));
  let maxFeePerGas = (base.maxFeePerGas * scaled) / 100n;
  let maxPriorityFeePerGas = (base.maxPriorityFeePerGas * scaled) / 100n;
  if (maxFeePerGas < maxPriorityFeePerGas) {
    maxFeePerGas = maxPriorityFeePerGas;
  }
  return { maxFeePerGas, maxPriorityFeePerGas };
};

const getBaseFees = async (provider: Provider): Promise<FeeData> => {
  const feeData = await provider.getFeeData();
  const priority = feeData.maxPriorityFeePerGas ?? MIN_PRIORITY_FEE;
  const maxPriorityFeePerGas = priority > MIN_PRIORITY_FEE ? priority : MIN_PRIORITY_FEE;
  let maxFeePerGas = feeData.maxFeePerGas ?? null;

  if (maxFeePerGas == null) {
    const pendingBlock = await provider.getBlock('pending');
    const baseFee = pendingBlock?.baseFeePerGas ?? feeData.gasPrice ?? MIN_PRIORITY_FEE;
    maxFeePerGas = baseFee * 2n + maxPriorityFeePerGas;
  }

  if (maxFeePerGas < maxPriorityFeePerGas) {
    maxFeePerGas = maxPriorityFeePerGas;
  }

  return { maxFeePerGas, maxPriorityFeePerGas };
};

const waitForReceipt = async (
  provider: Provider,
  txHash: string,
  timeoutMs: number,
): Promise<TransactionReceipt | null> => {
  const deadline = Date.now() + timeoutMs;
  while (Date.now() < deadline) {
    const receipt = await provider.getTransactionReceipt(txHash);
    if (receipt) return receipt;
    await new Promise((resolve) => setTimeout(resolve, RECEIPT_POLL_MS));
  }
  return null;
};

const sendWithRetries = async (params: {
  signer: HardhatEthersSigner;
  label: string;
  nonce: number;
  timeoutMs: number;
  maxRetries: number;
  feeBump: number;
  send: (overrides: TxOverrides) => Promise<TransactionResponse>;
}): Promise<TransactionReceipt> => {
  const { signer, label, nonce, timeoutMs, maxRetries, feeBump, send } = params;
  const provider = signer.provider;
  if (!provider) throw new Error('Signer has no provider');

  const baseFees = await getBaseFees(provider);
  let lastError: Error | undefined;
  const sentTxHashes: string[] = [];

  // Helper to check if any previously sent tx got mined
  const checkPreviousTxs = async (): Promise<TransactionReceipt | null> => {
    for (const hash of sentTxHashes) {
      const receipt = await provider.getTransactionReceipt(hash);
      if (receipt) {
        console.log(`SMOKE_TX_LATE_RECEIPT label=${label} hash=${hash}`);
        return receipt;
      }
    }
    return null;
  };

  for (let attempt = 0; attempt <= maxRetries; attempt += 1) {
    const fees = bumpFees(baseFees, Math.pow(feeBump, attempt));
    let tx: TransactionResponse;

    try {
      tx = await send({ nonce, ...fees });
    } catch (error) {
      // Send failed - maybe because a previous tx just got mined (nonce used)
      const lateReceipt = await checkPreviousTxs();
      if (lateReceipt) {
        if (lateReceipt.status === 1) return lateReceipt;
        throw new Error(`Transaction reverted (label=${label}, hash=${lateReceipt.hash})`);
      }
      lastError = error instanceof Error ? error : new Error(String(error));
      console.warn(`SMOKE_TX_SEND_FAILED label=${label} nonce=${nonce} attempt=${attempt}`);
      continue;
    }

    sentTxHashes.push(tx.hash);
    console.log(`SMOKE_TX_SENT label=${label} nonce=${nonce} hash=${tx.hash} attempt=${attempt}`);
    const receipt = await waitForReceipt(provider, tx.hash, timeoutMs);
    if (receipt) {
      if (receipt.status === 1) return receipt;
      throw new Error(`Transaction reverted (label=${label}, hash=${tx.hash})`);
    }

    console.warn(`SMOKE_TX_TIMEOUT label=${label} nonce=${nonce} attempt=${attempt} hash=${tx.hash}`);
    lastError = new Error(`Timeout waiting for tx ${tx.hash} (label=${label})`);
  }

  // Final check before giving up - a tx might have been mined during the last attempt
  const finalReceipt = await checkPreviousTxs();
  if (finalReceipt) {
    if (finalReceipt.status === 1) return finalReceipt;
    throw new Error(`Transaction reverted (label=${label}, hash=${finalReceipt.hash})`);
  }

  throw lastError ?? new Error(`Failed to send transaction (label=${label})`);
};

const getSignerStates = async (
  provider: Provider,
  signers: HardhatEthersSigner[],
  indices: number[],
): Promise<SignerState[]> => {
  return Promise.all(
    indices.map(async (index) => {
      const signer = signers[index];
      if (!signer) {
        throw new Error(`Signer index ${index} is unavailable; only ${signers.length} signers loaded.`);
      }
      const address = signer.address;
      const [latest, pending, balance] = await Promise.all([
        provider.getTransactionCount(address, 'latest'),
        provider.getTransactionCount(address, 'pending'),
        provider.getBalance(address),
      ]);
      return { signer, index, address, latest, pending, balance };
    }),
  );
};

const cancelBacklog = async (params: {
  signer: HardhatEthersSigner;
  latest: number;
  pending: number;
  timeoutMs: number;
  maxRetries: number;
  feeBump: number;
}): Promise<void> => {
  const { signer, latest, pending, timeoutMs, maxRetries, feeBump } = params;

  for (let nonce = latest; nonce < pending; nonce += 1) {
    await sendWithRetries({
      signer,
      label: `cancel-nonce-${nonce}`,
      nonce,
      timeoutMs,
      maxRetries,
      feeBump,
      send: (overrides) =>
        signer.sendTransaction({
          to: signer.address,
          value: 0n,
          gasLimit: CANCEL_GAS_LIMIT,
          ...overrides,
        }),
    });
  }
};

async function runSmoke(): Promise<void> {
  const provider = ethers.provider;
  const signerIndices = parseIndices(process.env.SMOKE_SIGNER_INDICES ?? DEFAULT_SIGNER_INDICES);
  const timeoutMs = (Number(process.env.SMOKE_TX_TIMEOUT_SECS) || DEFAULT_TX_TIMEOUT_SECS) * 1000;
  const maxRetries = Number(process.env.SMOKE_TX_MAX_RETRIES) || DEFAULT_TX_MAX_RETRIES;
  const feeBump = Number(process.env.SMOKE_FEE_BUMP) || DEFAULT_FEE_BUMP;
  const maxBacklog = Number(process.env.SMOKE_MAX_BACKLOG) || DEFAULT_MAX_BACKLOG;
  const decryptTimeoutMs = (Number(process.env.SMOKE_DECRYPT_TIMEOUT_SECS) || DEFAULT_DECRYPT_TIMEOUT_SECS) * 1000;
  const allowCancel = parseBooleanEnv('SMOKE_CANCEL_BACKLOG', true);
  const deployContract = parseBooleanEnv('SMOKE_DEPLOY_CONTRACT', true);
  const runTests = parseBooleanEnv('SMOKE_RUN_TESTS', true);
  const existingContractAddress = process.env.TEST_INPUT_CONTRACT_ADDRESS;

  const allSigners = await ethers.getSigners();
  const states = await getSignerStates(provider, allSigners, signerIndices);

  // Calculate minimum usable balance based on current gas prices
  const baseFees = await getBaseFees(provider);
  const minUsableBalance = baseFees.maxFeePerGas * SMOKE_GAS_ESTIMATE;

  console.log(`SMOKE_START network=${network.name} chainId=${(await provider.getNetwork()).chainId}`);
  console.log(`SMOKE_SIGNERS_AVAILABLE count=${states.length} minBalance=${ethers.formatEther(minUsableBalance)} ETH`);
  states.forEach((state) => {
    console.log(`  SMOKE_SIGNER ${formatSignerState(state)}`);
  });

  let selected = states.find((state) => state.pending === state.latest && state.balance >= minUsableBalance);

  if (!selected) {
    if (!allowCancel) {
      throw new Error('All signers have pending backlogs and auto-cancel is disabled.');
    }
    const primary = states[0];
    if (!primary) {
      throw new Error('No signer candidates available.');
    }
    const backlog = primary.pending - primary.latest;
    if (backlog <= 0) {
      throw new Error('No clean signer available; primary has no backlog to cancel.');
    }
    if (backlog > maxBacklog) {
      throw new Error(`Signer backlog too large (backlog=${backlog}, max=${maxBacklog}).`);
    }
    const minCancelBalance = baseFees.maxFeePerGas * CANCEL_GAS_LIMIT * BigInt(backlog);
    if (primary.balance < minCancelBalance) {
      throw new Error(
        `Signer ${primary.address} has insufficient balance for cancel (need ${ethers.formatEther(minCancelBalance)} ETH, have ${ethers.formatEther(primary.balance)} ETH)`,
      );
    }

    console.log(`SMOKE_CANCEL backlog=${backlog} signer=${primary.address}`);
    await cancelBacklog({
      signer: primary.signer,
      latest: primary.latest,
      pending: primary.pending,
      timeoutMs,
      maxRetries,
      feeBump,
    });

    const [latest, pending] = await Promise.all([
      provider.getTransactionCount(primary.address, 'latest'),
      provider.getTransactionCount(primary.address, 'pending'),
    ]);
    if (pending !== latest) {
      throw new Error('Backlog remains after auto-cancel.');
    }
    selected = { ...primary, latest, pending };
  }

  console.log(`SMOKE_SIGNER_SELECTED ${formatSignerState(selected)}`);

  const signer = selected.signer;
  const signerAddress = signer.address;
  const contractFactory = await ethers.getContractFactory('SmokeTestInput', signer);

  let contractAddress: string;
  let contract: SmokeTestInput;
  let deployMs = 0;

  if (deployContract) {
    if (existingContractAddress) {
      console.log('SMOKE_DEPLOY_CONTRACT ignoring TEST_INPUT_CONTRACT_ADDRESS');
    }
    console.log(
      `SMOKE_COPROCESSOR_CONFIG acl=${aclAddress} coprocessor=${coprocessorAddress} kms=${kmsVerifierAddress}`,
    );
    const deployStart = Date.now();
    const deployTx = await contractFactory.getDeployTransaction(aclAddress, coprocessorAddress, kmsVerifierAddress);
    const deployNonce = await provider.getTransactionCount(signerAddress, 'pending');
    let gasEstimate: bigint;
    try {
      gasEstimate = await provider.estimateGas({ ...deployTx, from: signerAddress });
    } catch (err) {
      console.error('SMOKE_GAS_ESTIMATE_FAILED operation=deploy', err);
      throw new Error(`Gas estimation failed for deploy: ${err instanceof Error ? err.message : String(err)}`);
    }
    const gasLimit = (gasEstimate * 120n) / 100n;

    const receipt = await sendWithRetries({
      signer,
      label: 'deploy-SmokeTestInput',
      nonce: deployNonce,
      timeoutMs,
      maxRetries,
      feeBump,
      send: (overrides) =>
        signer.sendTransaction({
          ...deployTx,
          ...overrides,
          gasLimit,
        }),
    });
    if (receipt.status !== 1 || !receipt.contractAddress) {
      throw new Error('Deployment failed or no contract address in receipt');
    }
    deployMs = Date.now() - deployStart;

    contractAddress = receipt.contractAddress;
    contract = contractFactory.attach(contractAddress) as SmokeTestInput;
    console.log(`SMOKE_DEPLOYED contractAddress=${contractAddress}`);
  } else if (existingContractAddress) {
    contractAddress = existingContractAddress;
    contract = contractFactory.attach(contractAddress) as SmokeTestInput;
    console.log(`SMOKE_ATTACH contractAddress=${contractAddress}`);
  } else {
    throw new Error('TEST_INPUT_CONTRACT_ADDRESS is required when SMOKE_DEPLOY_CONTRACT=0.');
  }

  let timingReport = deployMs > 0 ? `deploy=${deployMs}ms ` : '';

  if (!runTests) {
    console.log(`SMOKE_DEPLOY_ONLY contract=${contractAddress} ${timingReport.trim()}`);
  } else {
    const encryptStart = Date.now();
    const instance = await createInstance();
    const input = instance.createEncryptedInput(contractAddress, signerAddress);
    input.add64(7n);
    const encryptedInput = await input.encrypt();
    const encryptMs = Date.now() - encryptStart;

    const callNonce = await provider.getTransactionCount(signerAddress, 'pending');
    let callGasEstimate: bigint;
    try {
      callGasEstimate = await contract.add42ToInput64.estimateGas(encryptedInput.handles[0], encryptedInput.inputProof);
    } catch (err) {
      console.error('SMOKE_GAS_ESTIMATE_FAILED operation=add42ToInput64', err);
      throw new Error(`Gas estimation failed for add42ToInput64: ${err instanceof Error ? err.message : String(err)}`);
    }
    const gasLimit = (callGasEstimate * 120n) / 100n;

    const txStart = Date.now();
    const receipt = await sendWithRetries({
      signer,
      label: 'add42ToInput64',
      nonce: callNonce,
      timeoutMs,
      maxRetries,
      feeBump,
      send: (overrides) =>
        contract.add42ToInput64(encryptedInput.handles[0], encryptedInput.inputProof, {
          ...overrides,
          gasLimit,
        }),
    });
    const txMs = Date.now() - txStart;

    assert.equal(receipt.status, 1, 'on-chain call failed');

    const handle = await contract.resUint64();
    const { publicKey, privateKey } = instance.generateKeypair();

    const decryptStart = Date.now();
    const decryptedValue = await withTimeout(
      userDecryptSingleHandle(handle, contractAddress, instance, signer, privateKey, publicKey),
      decryptTimeoutMs,
      'userDecryptSingleHandle',
    );
    assert.equal(decryptedValue, 49n);

    const res = await withTimeout(instance.publicDecrypt([handle]), decryptTimeoutMs, 'publicDecrypt');
    assert.deepEqual(res.clearValues, { [handle]: 49n });
    const decryptMs = Date.now() - decryptStart;

    timingReport += `encrypt=${encryptMs}ms tx=${txMs}ms decrypt=${decryptMs}ms`;
    console.log(`SMOKE_SUCCESS signer=${signerAddress} contract=${contractAddress} ${timingReport.trim()}`);
  }

  // Heartbeat ping on success
  const heartbeatUrl = process.env.BETTERSTACK_HEARTBEAT_URL;
  if (heartbeatUrl) {
    await fetch(heartbeatUrl)
      .then(() => console.log('SMOKE_HEARTBEAT_SENT'))
      .catch((err) => console.warn(`SMOKE_HEARTBEAT_FAILED ${err.message}`));
  }

  // Post-success cleanup: clear backlogs on any unclean signers
  if (allowCancel) {
    for (const state of states) {
      const backlog = state.pending - state.latest;
      if (backlog <= 0) continue;

      const minBalanceForCancel = baseFees.maxFeePerGas * CANCEL_GAS_LIMIT * BigInt(backlog);
      if (state.balance < minBalanceForCancel) {
        console.warn(
          `SMOKE_CLEANUP_SKIPPED signer=${state.address} reason=low_balance need=${ethers.formatEther(minBalanceForCancel)} have=${ethers.formatEther(state.balance)}`,
        );
        continue;
      }

      if (backlog > maxBacklog) {
        console.warn(
          `SMOKE_CLEANUP_SKIPPED signer=${state.address} reason=backlog_too_large backlog=${backlog} max=${maxBacklog}`,
        );
        continue;
      }

      console.log(`SMOKE_CLEANUP signer=${state.address} backlog=${backlog}`);
      await cancelBacklog({
        signer: state.signer,
        latest: state.latest,
        pending: state.pending,
        timeoutMs,
        maxRetries,
        feeBump,
      });
    }
  }
}

runSmoke().catch(async (error) => {
  const errorMessage = String(error);
  console.error(`SMOKE_FAILED ${errorMessage}`);
  process.exitCode = 1;

  const heartbeatUrl = process.env.BETTERSTACK_HEARTBEAT_URL;
  if (heartbeatUrl) {
    await fetch(`${heartbeatUrl}/1`, {
      method: 'POST',
      body: errorMessage.slice(0, 10000),
    }).catch((err) => console.warn(`SMOKE_HEARTBEAT_FAILED ${err.message}`));
  }
});
