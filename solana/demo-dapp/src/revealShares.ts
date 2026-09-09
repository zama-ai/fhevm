import { createSolanaRpc, getAddressEncoder, type Address } from '@solana/kit';
import {
  createFhevmDecryptClient,
  defineFhevmSolanaChain,
  setFhevmRuntimeConfig,
  type SolanaDecryptTrust,
} from '@fhevm/sdk/solana';
import { balanceValueAddress, decryptPosition, getEncryptedValueState, tokenAccountAddress } from './vault/index.js';

import type { DemoSession } from './demoSession';
import { permitSessionFor } from './permitCache';
import { recordDecryptionEvidence } from './evidenceStore';

type Bytes32Hex = NonNullable<Parameters<typeof defineFhevmSolanaChain>[0]['fhevm']['verifyingProgramId']>;

export type RevealedBalance = {
  readonly handle: string;
  readonly value: bigint;
};

export type ConfidentialBalanceEvidence = {
  readonly encryptedValue: string;
  readonly handle: string;
  readonly tokenAccount: string;
};

const handleHex = (handle: Uint8Array): string =>
  `0x${Array.from(handle, (byte) => byte.toString(16).padStart(2, '0')).join('')}`;

const asBytes32BigEndian = (decimal: string): Uint8Array => {
  const bytes = new Uint8Array(32);
  let value = BigInt(decimal);
  for (let index = 31; index >= 0 && value > 0n; index -= 1) {
    bytes[index] = Number(value & 0xffn);
    value >>= 8n;
  }
  if (value > 0n) throw new Error(`${decimal} does not fit in 32 bytes`);
  return bytes;
};

/** The trust configuration the seeded demo deployment pins; party ids follow the registry order. */
const demoTrust = (config: DemoSession['config']): SolanaDecryptTrust => ({
  kmsSigners: config.kmsSigners.map((address, index) => ({ partyId: index + 1, address })),
  kmsContextId: handleHex(asBytes32BigEndian(config.userDecryptContextId)) as Bytes32Hex,
  kmsEpochId: config.kmsEpochId as Bytes32Hex,
  fheParameter: config.fheParameter,
  gatewayEip712Domain: {
    name: 'Decryption',
    version: '1',
    chainId: BigInt(config.gatewayChainId),
    verifyingContract: config.gatewayDecryptionContract,
  },
});

/** One-shot exact-handle reveal; the clear balance is never persisted. */
const revealConfidentialBalance = async (
  session: DemoSession,
  mint: DemoSession['config']['mints']['joinConfidential'],
  label: 'cShares' | 'cUSDC',
): Promise<RevealedBalance> => {
  const tokenAccount = await tokenAccountAddress(mint, session.signer.address);
  const encryptedValueAccount = await balanceValueAddress(mint, tokenAccount);
  const state = await getEncryptedValueState(createSolanaRpc(session.config.rpcUrl), encryptedValueAccount, {
    commitment: 'confirmed',
  });
  const encodeAddress = (value: Address): Uint8Array => new Uint8Array(getAddressEncoder().encode(value));
  // The permit covers this token program's values under this mint, and nothing else.
  const permitScope = {
    program: handleHex(encodeAddress(session.config.programs.token)) as Bytes32Hex,
    scope: handleHex(encodeAddress(mint)) as Bytes32Hex,
  };
  const chain = defineFhevmSolanaChain({
    id: BigInt(session.config.chainId),
    fhevm: {
      relayerUrl: session.config.relayerUrl,
      verifyingProgramId: session.config.aclProgram as Bytes32Hex,
    },
  });
  const supportsThreads = globalThis.crossOriginIsolated === true && typeof SharedArrayBuffer !== 'undefined';
  setFhevmRuntimeConfig({
    auth: { type: 'ApiKeyHeader', value: 'local' },
    singleThread: !supportsThreads,
  });
  // The permit channel is the one way to sign: a session whose wallet does not back the sRFC-38
  // feature cannot reveal, and says so instead of falling back to raw message signing.
  const permitWallet = session.permitWallet;
  if (permitWallet === undefined) {
    throw new Error(
      `${session.wallet.name} does not support solana:signOffchainMessage, the only channel a reveal is signed through; connect a wallet that does, or use the demo wallet`,
    );
  }
  const trust = demoTrust(session.config);
  const client = createFhevmDecryptClient({ chain, trust });
  await client.ready;
  const startedAt = performance.now();
  // One confirmation per (wallet, permit scope, KMS route) and validity window: repeated views of
  // the same private balance reuse the signed permit instead of prompting the wallet again.
  const permit = await permitSessionFor(
    {
      walletAddress: permitWallet.account.address,
      chainId: session.config.chainId,
      permitScope: `${permitScope.program}/${permitScope.scope}`,
      kmsContextId: trust.kmsContextId,
      kmsEpochId: trust.kmsEpochId,
    },
    () => client.signPermit({ wallet: permitWallet, durationSeconds: 3_600n, allowedScopes: [permitScope] }),
  );
  let jobId: string | null = null;
  let queuedAt: number | null = null;
  let responseAt: number | null = null;
  const [clearValue] = await decryptPosition(client, {
    session: permit,
    entries: [{ handle: state.currentHandle, encryptedValueAccount: encodeAddress(encryptedValueAccount) }],
    options: {
      timeout: 60_000,
      onProgress: (progress) => {
        if (progress.type === 'queued' && progress.method === 'POST') {
          jobId = progress.jobId;
          queuedAt = performance.now();
        } else if (progress.type === 'succeeded') {
          jobId ??= progress.jobId;
          responseAt = performance.now();
        }
      },
    },
  });
  if (clearValue === undefined || typeof clearValue.value !== 'bigint') {
    throw new Error(`decrypted ${label} balance is not an integer`);
  }
  const handle = handleHex(state.currentHandle);
  const totalElapsedMs = Math.round(performance.now() - startedAt);
  if (jobId !== null && queuedAt !== null && responseAt !== null) {
    recordDecryptionEvidence(session, {
      label,
      handle,
      jobId,
      queueToResponseMs: Math.round(responseAt - queuedAt),
      totalElapsedMs,
      completedAt: Date.now(),
    });
  }
  return { handle, value: clearValue.value };
};

export const readClaimedSharesHandle = async (session: DemoSession): Promise<string> => {
  const mint = session.config.mints.payoutConfidential;
  const tokenAccount = await tokenAccountAddress(mint, session.signer.address);
  const state = await getEncryptedValueState(
    createSolanaRpc(session.config.rpcUrl),
    await balanceValueAddress(mint, tokenAccount),
    { commitment: 'confirmed' },
  );
  return handleHex(state.currentHandle);
};

export const readClaimedUsdcHandle = async (session: DemoSession): Promise<string> => {
  const mint = session.config.mints.joinConfidential;
  const tokenAccount = await tokenAccountAddress(mint, session.signer.address);
  const state = await getEncryptedValueState(
    createSolanaRpc(session.config.rpcUrl),
    await balanceValueAddress(mint, tokenAccount),
    { commitment: 'confirmed' },
  );
  return handleHex(state.currentHandle);
};

export const readConfidentialBalanceEvidence = async (
  session: DemoSession,
  mint: DemoSession['config']['mints']['joinConfidential'],
): Promise<ConfidentialBalanceEvidence> => {
  const tokenAccount = await tokenAccountAddress(mint, session.signer.address);
  const encryptedValue = await balanceValueAddress(mint, tokenAccount);
  const state = await getEncryptedValueState(createSolanaRpc(session.config.rpcUrl), encryptedValue, {
    commitment: 'confirmed',
  });
  return {
    encryptedValue,
    handle: handleHex(state.currentHandle),
    tokenAccount,
  };
};

export const hasConfidentialBalanceAccount = async (
  session: DemoSession,
  mint: Address,
): Promise<boolean> => {
  const tokenAccount = await tokenAccountAddress(mint, session.signer.address);
  const account = await createSolanaRpc(session.config.rpcUrl)
    .getAccountInfo(tokenAccount, { commitment: 'confirmed', encoding: 'base64' })
    .send();
  if (account.value === null || account.value.owner === '11111111111111111111111111111111') return false;
  if (account.value.owner !== session.config.programs.token) {
    throw new Error('The canonical confidential token account is owned by an unexpected program');
  }
  return true;
};

export const revealClaimedShares = (session: DemoSession): Promise<RevealedBalance> =>
  revealConfidentialBalance(session, session.config.mints.payoutConfidential, 'cShares');

export const revealClaimedUsdc = (session: DemoSession): Promise<RevealedBalance> =>
  revealConfidentialBalance(session, session.config.mints.joinConfidential, 'cUSDC');
