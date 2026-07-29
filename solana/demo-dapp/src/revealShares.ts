import { createSolanaRpc, getAddressEncoder, type Address } from '@solana/kit';
import {
  createFhevmDecryptClient,
  defineFhevmSolanaChain,
  setFhevmRuntimeConfig,
  type SolanaUserDecryptSigner,
} from '@fhevm/sdk/solana';
import {
  confidentialBalanceValueAccount,
  decryptPosition,
  getEncryptedValueState,
  tokenAccountAddress,
} from '@fhevm/sdk/solana/vault';

import type { DemoSession } from './demoSession';
import { recordDecryptionEvidence } from './evidenceStore';

type Bytes32Hex = Parameters<typeof defineFhevmSolanaChain>[0]['fhevm']['acl']['domainKeys'][number];

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

const userDecryptSigner = (session: DemoSession): SolanaUserDecryptSigner => ({
  publicKey: new Uint8Array(getAddressEncoder().encode(session.signer.address)),
  sign: (message) => session.signMessageExact(message),
});

/** One-shot exact-handle reveal; the clear balance is never persisted. */
const revealConfidentialBalance = async (
  session: DemoSession,
  mint: DemoSession['config']['mints']['joinConfidential'],
  label: 'cShares' | 'cUSDC',
): Promise<RevealedBalance> => {
  const tokenAccount = await tokenAccountAddress(mint, session.signer.address);
  const balance = await confidentialBalanceValueAccount(mint, tokenAccount);
  const state = await getEncryptedValueState(createSolanaRpc(session.config.rpcUrl), balance.encryptedValueAddress);
  const encodedDomain = `0x${Array.from(getAddressEncoder().encode(mint), (byte) =>
    byte.toString(16).padStart(2, '0'),
  ).join('')}` as Bytes32Hex;
  const chain = defineFhevmSolanaChain({
    id: BigInt(session.config.chainId),
    fhevm: {
      relayerUrl: session.config.relayerUrl,
      acl: { domainKeys: [encodedDomain] },
    },
  });
  const supportsThreads = globalThis.crossOriginIsolated === true && typeof SharedArrayBuffer !== 'undefined';
  setFhevmRuntimeConfig({
    auth: { type: 'ApiKeyHeader', value: 'local' },
    singleThread: !supportsThreads,
  });
  const signer = userDecryptSigner(session);
  const client = createFhevmDecryptClient({ chain, signer });
  await client.ready;
  const startedAt = performance.now();
  let jobId: string | null = null;
  let queuedAt: number | null = null;
  let responseAt: number | null = null;
  const [clearValue] = await decryptPosition(
    { chain, runtime: client.runtime, options: { batchRpcCalls: false } },
    signer,
    {
      handles: [state.currentHandle],
      aclValueKey: balance.aclValueKey,
      contextId: asBytes32BigEndian(session.config.userDecryptContextId),
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
    },
  );
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
  const balance = await confidentialBalanceValueAccount(mint, tokenAccount);
  const state = await getEncryptedValueState(createSolanaRpc(session.config.rpcUrl), balance.encryptedValueAddress);
  return handleHex(state.currentHandle);
};

export const readConfidentialBalanceEvidence = async (
  session: DemoSession,
  mint: DemoSession['config']['mints']['joinConfidential'],
): Promise<ConfidentialBalanceEvidence> => {
  const tokenAccount = await tokenAccountAddress(mint, session.signer.address);
  const balance = await confidentialBalanceValueAccount(mint, tokenAccount);
  const state = await getEncryptedValueState(createSolanaRpc(session.config.rpcUrl), balance.encryptedValueAddress);
  return {
    encryptedValue: balance.encryptedValueAddress,
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
