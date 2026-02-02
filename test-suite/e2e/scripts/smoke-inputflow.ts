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
  baseFeePerGas?: bigint | null;
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
const DEFAULT_DECRYPT_TIMEOUT_SECS = 300;

const FALLBACK_PRIORITY_FEE = ethers.parseUnits('0.1', 'gwei');
const CANCEL_GAS_LIMIT = 21_000n;
const LOW_BALANCE_THRESHOLD = ethers.parseEther('0.005');
const SMOKE_GAS_ESTIMATE = 1_000_000n; // ~1M gas covers deploy + call with buffer

class SmokeTimeoutError extends Error {
  label: string;
  constructor(label: string, timeoutMs: number) {
    super(`Timeout after ${timeoutMs}ms: ${label}`);
    this.name = 'SmokeTimeoutError';
    this.label = label;
  }
}

const formatGwei = (value: bigint): string => `${ethers.formatUnits(value, 'gwei')} gwei`;
const formatEth = (value: bigint): string => `${ethers.formatEther(value)} ETH`;

const parseOptionalGweiEnv = (name: string): bigint | null => {
  const value = process.env[name];
  if (value === undefined || value === '') return null;
  if (!/^\d+(\.\d+)?$/.test(value)) {
    throw new Error(`${name} must be a positive number of gwei (got: "${value}")`);
  }
  const parsed = Number(value);
  if (!Number.isFinite(parsed) || parsed <= 0) {
    throw new Error(`${name} must be a positive number of gwei (got: "${value}")`);
  }
  return ethers.parseUnits(value, 'gwei');
};

const describeError = (error: unknown): string => {
  if (error instanceof Error) {
    const err = error as Error & {
      code?: string;
      reason?: string;
      shortMessage?: string;
      data?: unknown;
    };
    const details: string[] = [];
    if (err.code) details.push(`code=${err.code}`);
    if (err.reason) details.push(`reason=${err.reason}`);
    if (err.shortMessage) details.push(`shortMessage=${err.shortMessage}`);
    if (err.data !== undefined) details.push(`data=${String(err.data)}`);
    return details.length > 0 ? `${err.message} (${details.join(' ')})` : err.message;
  }
  return String(error);
};

const logTxReceipt = (params: {
  label: string;
  receipt: TransactionReceipt;
  nonce?: number;
  note?: string;
}): void => {
  const { label, receipt, nonce, note } = params;
  const effectiveGasPrice = receipt.effectiveGasPrice ?? null;
  const feePaid = effectiveGasPrice ? receipt.gasUsed * effectiveGasPrice : null;
  const parts = [
    `SMOKE_TX_MINED label=${label}`,
    nonce === undefined ? null : `nonce=${nonce}`,
    `hash=${receipt.hash}`,
    `block=${receipt.blockNumber}`,
    `status=${receipt.status}`,
    `gasUsed=${receipt.gasUsed}`,
    effectiveGasPrice ? `effectiveGasPrice=${formatGwei(effectiveGasPrice)}` : null,
    feePaid ? `fee=${formatEth(feePaid)}` : null,
    note ? `note=${note}` : null,
  ]
    .filter(Boolean)
    .join(' ');
  console.log(parts);
};

const getFailureClass = (error: unknown): string => {
  if (error instanceof SmokeTimeoutError) {
    if (error.label === 'userDecryptSingleHandle' || error.label === 'publicDecrypt') {
      return 'decrypt_timeout';
    }
    return 'timeout';
  }
  return 'error';
};

const withTimeout = <T>(promise: Promise<T>, timeoutMs: number, label: string): Promise<T> => {
  let timer: NodeJS.Timeout;
  const timeoutPromise = new Promise<never>((_, reject) => {
    timer = setTimeout(() => reject(new SmokeTimeoutError(label, timeoutMs)), timeoutMs);
  });
  return Promise.race([
    promise.finally(() => clearTimeout(timer)),
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
  const pendingBlock = await provider.getBlock('pending');
  const baseFeePerGas = pendingBlock?.baseFeePerGas ?? null;
  const maxPriorityFeePerGas = feeData.maxPriorityFeePerGas ?? FALLBACK_PRIORITY_FEE;
  let maxFeePerGas = feeData.maxFeePerGas ?? null;

  if (maxFeePerGas == null) {
    const baseFee = baseFeePerGas ?? feeData.gasPrice ?? FALLBACK_PRIORITY_FEE;
    maxFeePerGas = baseFee * 2n + maxPriorityFeePerGas;
  }

  if (maxFeePerGas < maxPriorityFeePerGas) {
    maxFeePerGas = maxPriorityFeePerGas;
  }

  return { maxFeePerGas, maxPriorityFeePerGas, baseFeePerGas };
};

const sendWithRetries = async (params: {
  signer: HardhatEthersSigner;
  label: string;
  nonce: number;
  timeoutMs: number;
  maxRetries: number;
  feeBump: number;
  maxFeePerGasCap?: bigint | null;
  maxPriorityFeePerGasCap?: bigint | null;
  send: (overrides: TxOverrides) => Promise<TransactionResponse>;
}): Promise<TransactionReceipt> => {
  const { signer, label, nonce, timeoutMs, maxRetries, feeBump, maxFeePerGasCap, maxPriorityFeePerGasCap, send } =
    params;
  const provider = signer.provider;
  if (!provider) throw new Error('Signer has no provider');

  const baseFees = await getBaseFees(provider);
  const sentTxHashes: string[] = [];
  let lastError: string | null = null;

  const logFeeCaps = (fees: FeeData, attempt: number): void => {
    if (maxFeePerGasCap && fees.maxFeePerGas > maxFeePerGasCap) {
      const message = `SMOKE_GAS_CAP_EXCEEDED label=${label} attempt=${attempt} maxFee=${formatGwei(
        fees.maxFeePerGas,
      )} cap=${formatGwei(maxFeePerGasCap)}`;
      lastError = message;
      console.error(message);
      throw new Error(message);
    }
    if (maxPriorityFeePerGasCap && fees.maxPriorityFeePerGas > maxPriorityFeePerGasCap) {
      const message = `SMOKE_PRIORITY_FEE_CAP_EXCEEDED label=${label} attempt=${attempt} maxPriority=${formatGwei(
        fees.maxPriorityFeePerGas,
      )} cap=${formatGwei(maxPriorityFeePerGasCap)}`;
      lastError = message;
      console.error(message);
      throw new Error(message);
    }
  };

  // Check if any previously sent tx was mined (used after send failure and as final fallback)
  const findMinedReceipt = async (): Promise<TransactionReceipt | null> => {
    for (const hash of sentTxHashes) {
      let receipt: TransactionReceipt | null;
      try {
        receipt = await provider.getTransactionReceipt(hash);
      } catch (error) {
        const msg = describeError(error);
        console.warn(`SMOKE_TX_RECEIPT_CHECK_FAILED label=${label} hash=${hash} error=${msg}`);
        continue;
      }
      if (receipt) {
        console.log(`SMOKE_TX_LATE_RECEIPT label=${label} hash=${hash}`);
        logTxReceipt({ label, receipt, nonce, note: 'late-receipt' });
        if (receipt.status !== 1) throw new Error(`Transaction reverted (label=${label}, hash=${hash})`);
        return receipt;
      }
    }
    return null;
  };

  for (let attempt = 0; attempt <= maxRetries; attempt += 1) {
    const fees = bumpFees(baseFees, Math.pow(feeBump, attempt));
    logFeeCaps(fees, attempt);
    const feeParts = [
      `SMOKE_TX_FEES label=${label}`,
      `nonce=${nonce}`,
      `attempt=${attempt}`,
      `maxFee=${formatGwei(fees.maxFeePerGas)}`,
      `maxPriority=${formatGwei(fees.maxPriorityFeePerGas)}`,
      baseFees.baseFeePerGas ? `baseFee=${formatGwei(baseFees.baseFeePerGas)}` : null,
    ]
      .filter(Boolean)
      .join(' ');
    console.log(feeParts);

    // 1. Try to send (may fail if nonce was consumed by a previous tx)
    let tx: TransactionResponse;
    try {
      tx = await send({ nonce, ...fees });
      sentTxHashes.push(tx.hash);
      console.log(`SMOKE_TX_SENT label=${label} nonce=${nonce} hash=${tx.hash} attempt=${attempt}`);
    } catch (error) {
      const mined = await findMinedReceipt();
      if (mined) return mined;
      const msg = describeError(error);
      lastError = `send_failed: ${msg}`;
      console.warn(`SMOKE_TX_SEND_FAILED label=${label} nonce=${nonce} attempt=${attempt} error=${msg}`);
      continue;
    }

    // 2. Wait for confirmation
    // ethers v6 tx.wait() behavior:
    //   - Success: returns receipt with status === 1
    //   - Revert: throws CALL_EXCEPTION (receipt available on error)
    //   - Timeout: throws TIMEOUT
    //   - Replacement mined: throws TRANSACTION_REPLACED
    try {
      const receipt = await tx.wait(1, timeoutMs);
      // Success - tx mined and not reverted
      if (receipt?.status === 1) {
        logTxReceipt({ label, receipt, nonce });
        return receipt;
      }
      // Defensive: if somehow we get a receipt with status !== 1, treat as revert
      throw new Error(`Transaction reverted (label=${label}, hash=${tx.hash})`);
    } catch (error: unknown) {
      const err = error as { code?: string; reason?: string; receipt?: TransactionReceipt };

      // CALL_EXCEPTION: tx was mined but reverted - terminal error, don't retry
      if (err.code === 'CALL_EXCEPTION') {
        const receiptHash = err.receipt?.hash ?? tx.hash;
        if (err.receipt) {
          logTxReceipt({ label, receipt: err.receipt, nonce, note: 'revert' });
        }
        throw new Error(`Transaction reverted (label=${label}, hash=${receiptHash})`);
      }

      // TIMEOUT: tx.wait() timed out - retry with bumped fees
      if (err.code === 'TIMEOUT') {
        console.warn(`SMOKE_TX_TIMEOUT label=${label} nonce=${nonce} attempt=${attempt} hash=${tx.hash}`);
        lastError = `timeout: ${tx.hash}`;
        continue;
      }

      // TRANSACTION_REPLACED: another tx with same nonce was mined
      // The `reason` field indicates what happened:
      //   - "repriced": same tx data, higher fees (our retry got mined) → SUCCESS
      //   - "cancelled": 0-value self-transfer (nonce was cancelled) → NOT our tx
      //   - "replaced": completely different tx (another process used nonce) → NOT our tx
      // In our controlled smoke test, only "repriced" should occur. We defensively
      // reject other reasons to avoid misreporting success for unrelated transactions.
      if (err.code === 'TRANSACTION_REPLACED') {
        const reason = err.reason;
        const receiptHash = err.receipt?.hash ?? 'unknown';
        if (reason === 'repriced' && err.receipt) {
          console.log(`SMOKE_TX_REPRICED label=${label} original=${tx.hash} mined=${receiptHash}`);
          logTxReceipt({ label, receipt: err.receipt, nonce, note: 'repriced' });
          if (err.receipt.status === 1) return err.receipt;
          throw new Error(`Repriced transaction reverted (label=${label}, hash=${receiptHash})`);
        }
        // Cancelled or replaced by unrelated tx - log warning and retry
        console.warn(
          `SMOKE_TX_REPLACED_UNEXPECTED label=${label} original=${tx.hash} reason=${reason} mined=${receiptHash}`,
        );
        lastError = `replaced_unexpected: reason=${reason ?? 'unknown'} hash=${receiptHash}`;
        continue;
      }

      // Unknown error - log and retry (network issues, etc.)
      const msg = describeError(error);
      lastError = `wait_error: ${msg}`;
      console.warn(`SMOKE_TX_WAIT_ERROR label=${label} nonce=${nonce} attempt=${attempt} error=${msg}`);
      continue;
    }
  }

  // 3. Final fallback: a tx may have been mined after our last wait timed out
  const mined = await findMinedReceipt();
  if (mined) return mined;

  throw new Error(`All ${maxRetries + 1} attempts failed (label=${label})${lastError ? ` lastError=${lastError}` : ''}`);
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
  maxFeePerGasCap?: bigint | null;
  maxPriorityFeePerGasCap?: bigint | null;
}): Promise<void> => {
  const { signer, latest, pending, timeoutMs, maxRetries, feeBump, maxFeePerGasCap, maxPriorityFeePerGasCap } = params;

  for (let nonce = latest; nonce < pending; nonce += 1) {
    await sendWithRetries({
      signer,
      label: `cancel-nonce-${nonce}`,
      nonce,
      timeoutMs,
      maxRetries,
      feeBump,
      maxFeePerGasCap,
      maxPriorityFeePerGasCap,
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
  const maxFeePerGasCap = parseOptionalGweiEnv('SMOKE_MAX_FEE_GWEI');
  const maxPriorityFeePerGasCap = parseOptionalGweiEnv('SMOKE_MAX_PRIORITY_FEE_GWEI');

  const allSigners = await ethers.getSigners();
  const states = await getSignerStates(provider, allSigners, signerIndices);

  // Calculate minimum usable balance based on current gas prices
  const baseFees = await getBaseFees(provider);
  const minUsableBalance = baseFees.maxFeePerGas * SMOKE_GAS_ESTIMATE;

  console.log(`SMOKE_START network=${network.name} chainId=${(await provider.getNetwork()).chainId}`);
  console.log(`SMOKE_SIGNERS_AVAILABLE count=${states.length} minBalance=${ethers.formatEther(minUsableBalance)} ETH`);
  console.log(
    `SMOKE_BASE_FEES maxFee=${formatGwei(baseFees.maxFeePerGas)} maxPriority=${formatGwei(
      baseFees.maxPriorityFeePerGas,
    )}${baseFees.baseFeePerGas ? ` baseFee=${formatGwei(baseFees.baseFeePerGas)}` : ''}`,
  );
  if (maxFeePerGasCap || maxPriorityFeePerGasCap) {
    console.log(
      `SMOKE_GAS_CAPS maxFee=${maxFeePerGasCap ? formatGwei(maxFeePerGasCap) : 'none'} maxPriority=${
        maxPriorityFeePerGasCap ? formatGwei(maxPriorityFeePerGasCap) : 'none'
      }`,
    );
  }
  states.forEach((state) => {
    console.log(`  SMOKE_SIGNER ${formatSignerState(state)}`);
  });

  if (maxFeePerGasCap && baseFees.maxFeePerGas > maxFeePerGasCap) {
    const message = `SMOKE_GAS_CAP_EXCEEDED_PRECHECK maxFee=${formatGwei(
      baseFees.maxFeePerGas,
    )} cap=${formatGwei(maxFeePerGasCap)}`;
    console.error(message);
    throw new Error(message);
  }
  if (maxPriorityFeePerGasCap && baseFees.maxPriorityFeePerGas > maxPriorityFeePerGasCap) {
    const message = `SMOKE_PRIORITY_FEE_CAP_EXCEEDED_PRECHECK maxPriority=${formatGwei(
      baseFees.maxPriorityFeePerGas,
    )} cap=${formatGwei(maxPriorityFeePerGasCap)}`;
    console.error(message);
    throw new Error(message);
  }

  const cleanStates = states.filter((state) => state.pending === state.latest);
  const fundedCleanStates = cleanStates.filter((state) => state.balance >= minUsableBalance);

  let selected = fundedCleanStates[0];

  if (!selected) {
    if (cleanStates.length > 0) {
      console.error(
        `SMOKE_NO_SIGNER_WITH_BALANCE minBalance=${formatEth(minUsableBalance)} maxFee=${formatGwei(
          baseFees.maxFeePerGas,
        )}`,
      );
      cleanStates.forEach((state) => {
        const deficit = minUsableBalance - state.balance;
        console.error(
          `SMOKE_SIGNER_INSUFFICIENT ${formatSignerState(state)} need=${formatEth(
            minUsableBalance,
          )} deficit=${formatEth(deficit)}`,
        );
      });
      throw new Error('No clean signer has sufficient balance for smoke test.');
    }
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
      maxFeePerGasCap,
      maxPriorityFeePerGasCap,
    });

    const [latest, pending, balance] = await Promise.all([
      provider.getTransactionCount(primary.address, 'latest'),
      provider.getTransactionCount(primary.address, 'pending'),
      provider.getBalance(primary.address),
    ]);
    if (pending !== latest) {
      throw new Error('Backlog remains after auto-cancel.');
    }
    selected = { ...primary, latest, pending, balance };
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
      maxFeePerGasCap,
      maxPriorityFeePerGasCap,
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
      maxFeePerGasCap,
      maxPriorityFeePerGasCap,
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
    console.log(`SMOKE_DECRYPT_START handle=${handle} timeoutMs=${decryptTimeoutMs}`);
    let decryptMs = 0;
    try {
      const decryptedValue = await withTimeout(
        userDecryptSingleHandle(handle, contractAddress, instance, signer, privateKey, publicKey),
        decryptTimeoutMs,
        'userDecryptSingleHandle',
      );
      console.log(`SMOKE_DECRYPT_USER_DONE ms=${Date.now() - decryptStart}`);
      assert.equal(decryptedValue, 49n);

      const res = await withTimeout(instance.publicDecrypt([handle]), decryptTimeoutMs, 'publicDecrypt');
      console.log(`SMOKE_DECRYPT_PUBLIC_DONE ms=${Date.now() - decryptStart}`);
      assert.deepEqual(res.clearValues, { [handle]: 49n });
      decryptMs = Date.now() - decryptStart;
    } catch (error) {
      if (error instanceof SmokeTimeoutError) {
        console.error(`SMOKE_DECRYPT_TIMEOUT step=${error.label} timeoutMs=${decryptTimeoutMs}`);
      }
      throw error;
    }

    timingReport += `encrypt=${encryptMs}ms tx=${txMs}ms decrypt=${decryptMs}ms`;
    console.log(`SMOKE_SUCCESS signer=${signerAddress} contract=${contractAddress} ${timingReport.trim()}`);
  }

  // Heartbeat ping on success
  const heartbeatUrl = process.env.BETTERSTACK_HEARTBEAT_URL;
  if (heartbeatUrl) {
    await fetch(heartbeatUrl)
      .then(() => console.log('SMOKE_HEARTBEAT_SENT'))
      .catch((err) => console.error(`SMOKE_HEARTBEAT_FAILED ${err.message}`));
  }

  // Post-success cleanup: clear backlogs on any unclean signers
  // Re-fetch state since nonces may have changed during the test
  if (allowCancel) {
    const freshStates = await getSignerStates(provider, allSigners, signerIndices);
    for (const state of freshStates) {
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
      try {
        await cancelBacklog({
          signer: state.signer,
          latest: state.latest,
          pending: state.pending,
          timeoutMs,
          maxRetries,
          feeBump,
          maxFeePerGasCap,
          maxPriorityFeePerGasCap,
        });
      } catch (error) {
        console.error(`SMOKE_CLEANUP_FAILED signer=${state.address} error=${describeError(error)}`);
      }
    }
  }
}

runSmoke().catch(async (error) => {
  const errorMessage = describeError(error);
  const failureClass = getFailureClass(error);
  console.error(`SMOKE_FAILED class=${failureClass} ${errorMessage}`);
  process.exitCode = 1;

  const heartbeatUrl = process.env.BETTERSTACK_HEARTBEAT_URL;
  if (heartbeatUrl) {
    const failurePayload = `class=${failureClass}\n${errorMessage}`;
    await fetch(`${heartbeatUrl}/1`, {
      method: 'POST',
      body: failurePayload.slice(0, 10000),
    }).catch((err) => console.error(`SMOKE_HEARTBEAT_FAILED ${err.message}`));
  }
});
