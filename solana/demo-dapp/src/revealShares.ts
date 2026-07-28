import { createSolanaRpc, getAddressEncoder } from '@solana/kit';
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
  const [clearValue] = await decryptPosition(
    { chain, runtime: client.runtime, options: { batchRpcCalls: false } },
    signer,
    {
      handles: [state.currentHandle],
      aclValueKey: balance.aclValueKey,
      contextId: asBytes32BigEndian(session.config.userDecryptContextId),
      options: { timeout: 60_000 },
    },
  );
  if (clearValue === undefined || typeof clearValue.value !== 'bigint') {
    throw new Error('decrypted cShares balance is not an integer');
  }
  return { handle: handleHex(state.currentHandle), value: clearValue.value };
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

export const revealClaimedShares = (session: DemoSession): Promise<RevealedBalance> =>
  revealConfidentialBalance(session, session.config.mints.payoutConfidential);

export const revealClaimedUsdc = (session: DemoSession): Promise<RevealedBalance> =>
  revealConfidentialBalance(session, session.config.mints.joinConfidential);
