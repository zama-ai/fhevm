import type { HardhatEthersSigner } from '@nomicfoundation/hardhat-ethers/signers';
import type { Provider, TransactionReceipt, TransactionResponse } from 'ethers';
import { ethers, network } from 'hardhat';
import assert from 'node:assert/strict';

import { createInstance } from '../test/instance';
import { userDecryptSingleHandle } from '../test/utils';

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

const DEFAULT_SIGNER_INDICES = '0';
const DEFAULT_TX_MAX_RETRIES = 2;
const DEFAULT_MAX_BACKLOG = 3;
const TIME_PER_BLOCK = 12;
const NUMBER_OF_BLOCKS = 4;
const DEFAULT_TX_TIMEOUT_SECS = TIME_PER_BLOCK * NUMBER_OF_BLOCKS;
const DEFAULT_FEE_BUMP = 1.125 ** NUMBER_OF_BLOCKS;

const MIN_PRIORITY_FEE = ethers.parseUnits('2', 'gwei');
const CANCEL_GAS_LIMIT = 21_000n;
const RECEIPT_POLL_MS = 4_000;

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

const formatSignerState = (state: SignerState): string =>
  `index=${state.index} address=${state.address} latest=${state.latest} pending=${state.pending} balance=${ethers.formatEther(
    state.balance,
  )} ETH`;

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
    const baseFee = pendingBlock?.baseFeePerGas ?? feeData.gasPrice ?? (await provider.getGasPrice());
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

  for (let attempt = 0; attempt <= maxRetries; attempt += 1) {
    const fees = bumpFees(baseFees, Math.pow(feeBump, attempt));
    let tx: TransactionResponse;

    try {
      tx = await send({ nonce, ...fees });
    } catch (error) {
      lastError = error instanceof Error ? error : new Error(String(error));
      console.warn(`SMOKE_TX_SEND_FAILED label=${label} nonce=${nonce} attempt=${attempt}`);
      continue;
    }

    console.log(`SMOKE_TX_SENT label=${label} nonce=${nonce} hash=${tx.hash} attempt=${attempt}`);
    const receipt = await waitForReceipt(provider, tx.hash, timeoutMs);
    if (receipt) {
      if (receipt.status === 1) return receipt;
      throw new Error(`Transaction reverted (label=${label}, hash=${tx.hash})`);
    }

    console.warn(`SMOKE_TX_TIMEOUT label=${label} nonce=${nonce} attempt=${attempt} hash=${tx.hash}`);
    lastError = new Error(`Timeout waiting for tx ${tx.hash} (label=${label})`);
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
  const timeoutMs = Math.trunc(Number(process.env.SMOKE_TX_TIMEOUT_SECS) || DEFAULT_TX_TIMEOUT_SECS) * 1000;
  const maxRetries = Math.trunc(Number(process.env.SMOKE_TX_MAX_RETRIES) || DEFAULT_TX_MAX_RETRIES);
  const feeBump = Number(process.env.SMOKE_FEE_BUMP) || DEFAULT_FEE_BUMP;
  const maxBacklog = Math.trunc(Number(process.env.SMOKE_MAX_BACKLOG) || DEFAULT_MAX_BACKLOG);
  const cancelRaw = process.env.SMOKE_CANCEL_BACKLOG;
  const allowCancel = cancelRaw === undefined || cancelRaw === '' ? true : cancelRaw.trim().toLowerCase() !== 'false';
  const forceRaw = process.env.SMOKE_FORCE_DEPLOY;
  const forceDeploy = forceRaw !== undefined && forceRaw !== '' && forceRaw.trim().toLowerCase() !== 'false';
  const existingContractAddress = process.env.TEST_INPUT_CONTRACT_ADDRESS;

  const allSigners = await ethers.getSigners();
  const states = await getSignerStates(provider, allSigners, signerIndices);

  console.log(`SMOKE_START network=${network.name} chainId=${(await provider.getNetwork()).chainId}`);
  states.forEach((state) => {
    console.log(`SMOKE_SIGNER_STATE ${formatSignerState(state)}`);
  });

  let selected = states.find((state) => state.pending === state.latest && state.balance > 0n);

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
    if (primary.balance === 0n) {
      throw new Error(`Signer ${primary.address} has zero balance; cannot auto-cancel.`);
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
  const contractFactory = await ethers.getContractFactory('TestInput', signer);

  let contractAddress: string;
  let contract: ReturnType<typeof contractFactory.attach>;

  if (forceDeploy) {
    if (existingContractAddress) {
      console.log('SMOKE_FORCE_DEPLOY ignoring TEST_INPUT_CONTRACT_ADDRESS');
    }
    const deployTx = await contractFactory.getDeployTransaction();
    const deployNonce = await provider.getTransactionCount(signerAddress, 'pending');
    const gasEstimate = await provider.estimateGas({ ...deployTx, from: signerAddress });
    const gasLimit = (gasEstimate * 120n) / 100n;

    const receipt = await sendWithRetries({
      signer,
      label: 'deploy-TestInput',
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
    if (receipt.status !== 1) {
      throw new Error('Deployment failed');
    }

    contractAddress = ethers.getCreateAddress({ from: signerAddress, nonce: deployNonce });
    contract = contractFactory.attach(contractAddress);
    console.log(`SMOKE_DEPLOYED contractAddress=${contractAddress}`);
  } else if (existingContractAddress) {
    contractAddress = existingContractAddress;
    contract = contractFactory.attach(contractAddress);
    console.log(`SMOKE_ATTACH contractAddress=${contractAddress}`);
  } else {
    throw new Error(
      'TEST_INPUT_CONTRACT_ADDRESS is required for smoke runs. Set SMOKE_FORCE_DEPLOY=1 to deploy explicitly.',
    );
  }

  const instance = await createInstance();
  const input = instance.createEncryptedInput(contractAddress, signerAddress);
  input.add64(7n);
  const encryptedInput = await input.encrypt();

  const callNonce = await provider.getTransactionCount(signerAddress, 'pending');
  const gasEstimate = await contract.add42ToInput64.estimateGas(encryptedInput.handles[0], encryptedInput.inputProof);
  const gasLimit = (gasEstimate * 120n) / 100n;

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

  assert.equal(receipt.status, 1, 'on-chain call failed');

  const handle = await contract.resUint64();
  const { publicKey, privateKey } = instance.generateKeypair();
  const decryptedValue = await userDecryptSingleHandle(
    handle,
    contractAddress,
    instance,
    signer,
    privateKey,
    publicKey,
  );
  assert.equal(decryptedValue, 49n);

  const res = await instance.publicDecrypt([handle]);
  assert.deepEqual(res.clearValues, { [handle]: 49n });

  console.log(`SMOKE_SUCCESS signer=${signerAddress} contract=${contractAddress}`);

  // Post-success cleanup: clear backlogs on any unclean signers
  if (allowCancel) {
    const minBalanceForCancel = CANCEL_GAS_LIMIT * MIN_PRIORITY_FEE * 2n;

    for (const state of states) {
      const backlog = state.pending - state.latest;
      if (backlog <= 0) continue;

      if (state.balance < minBalanceForCancel) {
        console.warn(
          `SMOKE_CLEANUP_SKIPPED signer=${state.address} reason=low_balance balance=${ethers.formatEther(state.balance)}`,
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

runSmoke().catch((error) => {
  console.error(`SMOKE_FAILED ${String(error)}`);
  process.exitCode = 1;
});
