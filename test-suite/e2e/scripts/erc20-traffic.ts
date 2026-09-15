// Organic ERC-20 traffic for Blue/Green QA rounds: ONE EncryptedERC20 per chain for the whole
// round, mints and encrypted transfers between three holders, and user decryptions of every
// balance checked against the tracked expectation. Driven by env, run with
//   TRAFFIC_CMD=setup|loop|verify npx hardhat run --no-compile scripts/erc20-traffic.ts --network <net>
//
// setup   deploy once (alice = owner) and mint the initial supply; no-op if the state file exists
// loop    one step every TRAFFIC_INTERVAL_SECS until TRAFFIC_STOP_FILE exists: a transfer with an
//         encrypted amount (input proof + FHE ops), every TRAFFIC_MINT_EVERY steps a mint, every
//         TRAFFIC_DECRYPT_EVERY steps a user decryption of the next holder's balance compared to
//         the expected value. Failures are retried, then counted; the loop never stops on its own.
// verify  decrypt every holder's balance and read totalSupply; exit 1 on any mismatch
//
// State (contract, expected balances, counters) lives in TRAFFIC_STATE_FILE, so the same token is
// used across coprocessor resets and the expectation survives the whole round.
import fs from 'node:fs/promises';
import path from 'node:path';
import { ethers, network } from 'hardhat';

import type { EncryptedERC20 } from '../types';
import { createInstances } from '../test/instance';
import { getSigners, initSigners, type Signers } from '../test/signers';
import type { FhevmInstances } from '../test/types';
import { waitForTransactionReceipt } from '../test/utils';

type Holder = 'alice' | 'bob' | 'carol';
const HOLDERS: Holder[] = ['alice', 'bob', 'carol'];

type State = {
  network: string;
  chainId: number;
  contractAddress: string;
  owner: string;
  holders: Record<Holder, string>;
  expected: Record<Holder, string>; // bigint as decimal string
  mintedTotal: string;
  counters: {
    iterations: number;
    transfers: number;
    mints: number;
    decrypts: number;
    decryptMismatches: number;
    retries: number;
    failures: number;
  };
  running: boolean;
  startedAt?: string;
  lastIterationAt?: string;
  lastError?: string;
  lastErrorAt?: string;
};

const cmd = process.env.TRAFFIC_CMD ?? '';
const dir = process.env.TRAFFIC_DIR ?? '/data/erc20-traffic';
const stateFile = process.env.TRAFFIC_STATE_FILE ?? path.join(dir, `${network.name}.json`);
const stopFile = process.env.TRAFFIC_STOP_FILE ?? path.join(dir, `${network.name}.stop`);
const intervalSecs = Number(process.env.TRAFFIC_INTERVAL_SECS ?? '60');
const maxIterations = Number(process.env.TRAFFIC_MAX_ITERATIONS ?? '0'); // 0 = until stop file
const initialMint = BigInt(process.env.TRAFFIC_INITIAL_MINT ?? '1000000');
const mintAmount = BigInt(process.env.TRAFFIC_MINT_AMOUNT ?? '100000');
const maxTransfer = BigInt(process.env.TRAFFIC_MAX_TRANSFER ?? '1000');
const mintEvery = Number(process.env.TRAFFIC_MINT_EVERY ?? '10');
const decryptEvery = Number(process.env.TRAFFIC_DECRYPT_EVERY ?? '2');
const maxRetries = Number(process.env.TRAFFIC_MAX_RETRIES ?? '3');
const retryBackoffMs = Number(process.env.TRAFFIC_RETRY_BACKOFF_MS ?? '5000');
const receiptTimeoutMs = Number(process.env.TRAFFIC_RECEIPT_TIMEOUT_SECS ?? '180') * 1000;

const now = () => new Date().toISOString();
const log = (msg: string) => console.log(`${now()} [traffic ${network.name}] ${msg}`);
const sleep = (ms: number) => new Promise((r) => setTimeout(r, ms));
const fileExists = async (p: string) => fs.access(p).then(() => true, () => false);

const loadState = async (): Promise<State | undefined> => {
  if (!(await fileExists(stateFile))) return undefined;
  return JSON.parse(await fs.readFile(stateFile, 'utf8')) as State;
};
// Write via a temp file so a reader never sees a half-written state.
const saveState = async (state: State): Promise<void> => {
  await fs.mkdir(path.dirname(stateFile), { recursive: true });
  const tmp = `${stateFile}.tmp`;
  await fs.writeFile(tmp, JSON.stringify(state, null, 2));
  await fs.rename(tmp, stateFile);
};
const recordError = (state: State, err: unknown) => {
  state.lastError = err instanceof Error ? err.message : String(err);
  state.lastErrorAt = now();
};

// Retry `fn` up to maxRetries times; count retries; rethrow after the last attempt.
const withRetries = async <T>(state: State, label: string, fn: () => Promise<T>): Promise<T> => {
  let lastErr: unknown;
  for (let attempt = 1; attempt <= maxRetries; attempt += 1) {
    try {
      return await fn();
    } catch (err) {
      lastErr = err;
      log(`${label}: attempt ${attempt}/${maxRetries} failed: ${err instanceof Error ? err.message : String(err)}`);
      if (attempt < maxRetries) {
        state.counters.retries += 1;
        await sleep(retryBackoffMs);
      }
    }
  }
  throw lastErr;
};

type Ctx = { signers: Signers; instances: FhevmInstances; token: EncryptedERC20; state: State };

const attach = async (address: string): Promise<EncryptedERC20> => {
  const factory = await ethers.getContractFactory('EncryptedERC20');
  return factory.attach(address) as EncryptedERC20;
};

const sendAndWait = async (txPromise: Promise<{ hash: string }>) => {
  const tx = await txPromise;
  const receipt = await waitForTransactionReceipt(tx.hash, receiptTimeoutMs);
  if (receipt.status !== 1) throw new Error(`tx ${tx.hash} reverted`);
  return receipt;
};

const decryptBalance = async (ctx: Ctx, holder: Holder): Promise<bigint> => {
  const handle = await ctx.token.balanceOf(ctx.signers[holder].address);
  return ctx.instances[holder].userDecryptSingleHandle({
    handle,
    contractAddress: ctx.state.contractAddress,
    signer: ctx.signers[holder],
  });
};

const transfer = async (ctx: Ctx, from: Holder, to: Holder, amount: bigint) => {
  const enc = await ctx.instances[from].encryptUint64({
    value: amount,
    contractAddress: ctx.state.contractAddress,
    userAddress: ctx.signers[from].address,
  });
  await sendAndWait(
    ctx.token
      .connect(ctx.signers[from])
      ['transfer(address,bytes32,bytes)'](ctx.signers[to].address, enc.handles[0], enc.inputProof),
  );
  ctx.state.expected[from] = (BigInt(ctx.state.expected[from]) - amount).toString();
  ctx.state.expected[to] = (BigInt(ctx.state.expected[to]) + amount).toString();
  ctx.state.counters.transfers += 1;
};

const mint = async (ctx: Ctx, amount: bigint) => {
  await sendAndWait(ctx.token.connect(ctx.signers.alice).mint(amount));
  ctx.state.expected.alice = (BigInt(ctx.state.expected.alice) + amount).toString();
  ctx.state.mintedTotal = (BigInt(ctx.state.mintedTotal) + amount).toString();
  ctx.state.counters.mints += 1;
};

// Deterministic-but-varied pick: the sender rotates over the holders that have a balance, the
// recipient rotates over the others, the amount is a pseudo-random slice of the sender's balance
// capped at maxTransfer (never above the balance, so the FHE transfer is never a silent no-op).
const pickTransfer = (state: State, iteration: number): { from: Holder; to: Holder; amount: bigint } | undefined => {
  const funded = HOLDERS.filter((h) => BigInt(state.expected[h]) > 0n);
  if (funded.length === 0) return undefined;
  const from = funded[iteration % funded.length]!;
  const others = HOLDERS.filter((h) => h !== from);
  const to = others[Math.floor(iteration / funded.length) % others.length]!;
  const balance = BigInt(state.expected[from]);
  const cap = balance < maxTransfer ? balance : maxTransfer;
  const amount = 1n + (BigInt(iteration * 7919 + 13) % cap);
  return { from, to, amount };
};

const buildCtx = async (state: State): Promise<Ctx> => {
  await initSigners(3);
  const signers = await getSigners();
  const instances = await createInstances(signers);
  const token = await attach(state.contractAddress);
  return { signers, instances, token, state };
};

const setup = async () => {
  const existing = await loadState();
  if (existing) {
    log(`setup: state exists, keeping token ${existing.contractAddress} (minted ${existing.mintedTotal})`);
    return;
  }
  await initSigners(3);
  const signers = await getSigners();
  const chainId = Number((await ethers.provider.getNetwork()).chainId);
  const factory = await ethers.getContractFactory('EncryptedERC20');
  const deployTx = await factory.getDeployTransaction('Naraggara', 'NARA');
  const tx = await signers.alice.sendTransaction({ ...deployTx, gasLimit: 10_000_000 });
  const receipt = await waitForTransactionReceipt(tx.hash, receiptTimeoutMs);
  if (!receipt.contractAddress || receipt.status !== 1) throw new Error(`deploy failed: ${tx.hash}`);
  const state: State = {
    network: network.name,
    chainId,
    contractAddress: receipt.contractAddress,
    owner: signers.alice.address,
    holders: { alice: signers.alice.address, bob: signers.bob.address, carol: signers.carol.address },
    expected: { alice: '0', bob: '0', carol: '0' },
    mintedTotal: '0',
    counters: { iterations: 0, transfers: 0, mints: 0, decrypts: 0, decryptMismatches: 0, retries: 0, failures: 0 },
    running: false,
  };
  const ctx: Ctx = { signers, instances: await createInstances(signers), token: await attach(state.contractAddress), state };
  await mint(ctx, initialMint);
  const balance = await decryptBalance(ctx, 'alice');
  if (balance !== initialMint) throw new Error(`setup: alice decrypts to ${balance}, expected ${initialMint}`);
  ctx.state.counters.decrypts += 1;
  await saveState(state);
  log(`setup: token ${state.contractAddress} deployed by ${state.owner}, minted ${initialMint}, alice balance decrypts OK`);
};

const loop = async () => {
  const state = await loadState();
  if (!state) throw new Error(`loop: no state at ${stateFile}; run setup first`);
  if (await fileExists(stopFile)) await fs.rm(stopFile);
  const ctx = await buildCtx(state);
  state.running = true;
  state.startedAt = now();
  await saveState(state);
  log(`loop: token ${state.contractAddress}, interval ${intervalSecs}s, mint every ${mintEvery}, decrypt every ${decryptEvery}`);

  let decryptCursor = 0;
  while (!(await fileExists(stopFile)) && (maxIterations === 0 || state.counters.iterations < maxIterations)) {
    const i = state.counters.iterations + 1;
    try {
      if (i % mintEvery === 0) {
        await withRetries(state, `#${i} mint`, () => mint(ctx, mintAmount));
        log(`#${i} mint ${mintAmount} -> alice=${state.expected.alice} total=${state.mintedTotal}`);
      } else {
        const pick = pickTransfer(state, i);
        if (!pick) throw new Error('no holder has a balance to transfer');
        await withRetries(state, `#${i} transfer`, () => transfer(ctx, pick.from, pick.to, pick.amount));
        log(`#${i} transfer ${pick.from}->${pick.to} ${pick.amount} -> ${pick.from}=${state.expected[pick.from]} ${pick.to}=${state.expected[pick.to]}`);
      }
      if (i % decryptEvery === 0) {
        // Rotate through every holder so each balance is decrypted regularly.
        const holder = HOLDERS[decryptCursor % HOLDERS.length]!;
        decryptCursor += 1;
        const value = await withRetries(state, `#${i} decrypt ${holder}`, () => decryptBalance(ctx, holder));
        state.counters.decrypts += 1;
        const expected = BigInt(state.expected[holder]);
        if (value === expected) {
          log(`#${i} decrypt ${holder}=${value} OK`);
        } else {
          state.counters.decryptMismatches += 1;
          recordError(state, `decrypt ${holder}: got ${value}, expected ${expected}`);
          log(`#${i} decrypt ${holder} MISMATCH got ${value} expected ${expected}`);
        }
      }
    } catch (err) {
      state.counters.failures += 1;
      recordError(state, err);
      log(`#${i} FAILED after retries: ${state.lastError}`);
    }
    state.counters.iterations = i;
    state.lastIterationAt = now();
    await saveState(state);
    // Sleep in short slices so a stop request is honored within a few seconds.
    for (let waited = 0; waited < intervalSecs && !(await fileExists(stopFile)); waited += 2) {
      await sleep(2000);
    }
  }
  state.running = false;
  await saveState(state);
  const c = state.counters;
  log(
    `loop: stopped after ${c.iterations} iterations: ${c.transfers} transfers, ${c.mints} mints, ${c.decrypts} decrypts ` +
      `(${c.decryptMismatches} mismatches), ${c.retries} retries, ${c.failures} failures`,
  );
};

const verify = async () => {
  const state = await loadState();
  if (!state) throw new Error(`verify: no state at ${stateFile}`);
  const ctx = await buildCtx(state);
  let ok = true;
  for (const holder of HOLDERS) {
    const expected = BigInt(state.expected[holder]);
    const value = await decryptBalance(ctx, holder);
    const match = value === expected;
    ok = ok && match;
    log(`verify ${holder} (${state.holders[holder]}): decrypted=${value} expected=${expected} ${match ? 'OK' : 'MISMATCH'}`);
  }
  const supply = await ctx.token.totalSupply();
  const supplyOk = BigInt(supply) === BigInt(state.mintedTotal);
  ok = ok && supplyOk;
  log(`verify totalSupply: on-chain=${supply} expected=${state.mintedTotal} ${supplyOk ? 'OK' : 'MISMATCH'}`);
  const c = state.counters;
  log(`verify counters: ${c.iterations} iterations, ${c.transfers} transfers, ${c.mints} mints, ${c.decrypts} decrypts, ${c.decryptMismatches} mismatches, ${c.failures} failures`);
  if (!ok) throw new Error('verify: balances do not match the tracked expectation');
};

const main = async () => {
  switch (cmd) {
    case 'setup':
      return setup();
    case 'loop':
      return loop();
    case 'verify':
      return verify();
    default:
      throw new Error(`TRAFFIC_CMD must be setup, loop or verify (got '${cmd}')`);
  }
};

// Exit explicitly: the SDK instances and providers keep the event loop alive otherwise.
main()
  .then(() => process.exit(0))
  .catch((err) => {
    console.error(`${now()} [traffic ${network.name}] ${err instanceof Error ? err.stack ?? err.message : String(err)}`);
    process.exit(1);
  });
