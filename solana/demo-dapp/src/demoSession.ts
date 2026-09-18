import {
  createKeyPairSignerFromBytes,
  createSignableMessage,
  createSolanaRpc,
  type Address,
  type TransactionSigner,
} from '@solana/kit';
import {
  createMessageSignerFromWalletAccount,
  createTransactionSignerFromWalletAccount,
} from '@solana/wallet-account-signer';
import type { UiWalletAccount } from '@wallet-standard/react';
import { SolanaSignOffchainMessage, type SolanaSignOffchainMessageFeature } from '@solana/wallet-standard-features';
import { getWalletAccountFeature } from '@wallet-standard/ui';
import { getWalletAccountForUiWalletAccount_DO_NOT_USE_OR_YOU_WILL_BE_FIRED } from '@wallet-standard/ui-registry';
import { solanaPermitWalletFromSecretKey, type SolanaPermitWallet } from '@fhevm/sdk/solana';
import {
  ZAMA_HOST_PROGRAM_ADDRESS,
  getZamaHostErrorMessage,
  type ZamaHostError,
} from '@fhevm/sdk/solana/host';

import { loadOrCreateBurnerSecretKey } from './burnerWallet';
import { demoFaucetFetch } from './demoAuthorization';
import { parseDemoConfigResponse, type DemoConfig } from './demoConfig';

export { parseDemoConfigResponse, type DemoConfig } from './demoConfig';

export type DemoSession = {
  readonly config: DemoConfig;
  readonly signer: TransactionSigner;
  readonly signMessageExact: (message: Uint8Array) => Promise<Uint8Array>;
  /**
   * The wallet a user-decrypt permit is signed through, when this session kind can provide one.
   * The burner session always can; a wallet-standard session can when the connected wallet
   * exposes `solana:signOffchainMessage` — absent that feature this is `undefined`, and reveals
   * refuse with a clear message instead of falling back to raw message signing.
   */
  readonly permitWallet: SolanaPermitWallet | undefined;
  readonly wallet:
    | { readonly kind: 'burner'; readonly name: 'Demo wallet' }
    | { readonly kind: 'wallet-standard'; readonly name: string; readonly accountKey: string };
  readonly isActive: () => boolean;
  readonly assertActive: () => void;
};

const LAMPORTS_PER_SOL = 1_000_000_000n;
const USDC_BASE_UNITS = 1_000_000n;
const MIN_SOL_BALANCE = 2n * LAMPORTS_PER_SOL;
const TARGET_SOL_BALANCE = 5n * LAMPORTS_PER_SOL;
const MIN_USDC_BALANCE = 100n * USDC_BASE_UNITS;
const TARGET_USDC_BALANCE = 1_000n * USDC_BASE_UNITS;

export type FundingPlan = {
  readonly sol?: number;
  readonly usdc?: number;
};

export const describeWalletError = (error: unknown, context: 'connect' | 'transaction' | 'reveal'): string => {
  const candidate = error as { readonly code?: unknown; readonly message?: unknown };
  const rejected =
    candidate?.code === 4001 ||
    candidate?.code === 4_001_000 ||
    (typeof candidate?.message === 'string' &&
      /user rejected|request rejected|cancelled by user/i.test(candidate.message));
  if (rejected) {
    if (context === 'connect') return 'Wallet connection cancelled';
    if (context === 'reveal') return 'Signature cancelled — your confidential balance remains hidden';
    return 'Signature cancelled — nothing new was sent; any confirmed step is saved';
  }
  const host = identifiedZamaHostErrorCopy(error);
  if (host !== undefined) return host;
  return error instanceof Error ? error.message : String(error);
};

const HOST_FAILED = new RegExp(
  `Program ${ZAMA_HOST_PROGRAM_ADDRESS} failed(?:[^\\n]*custom program error: 0x([0-9a-f]+))?`,
  'i',
);

function identifiedZamaHostErrorCopy(error: unknown): string | undefined {
  const text = diagnosticText(error);
  const failed = text.match(HOST_FAILED);
  const hex = failed?.[1];
  if (hex === undefined) return undefined;
  const code = Number.parseInt(hex, 16);
  if (!Number.isInteger(code)) return undefined;
  const message = getZamaHostErrorMessage(code as ZamaHostError);
  if (typeof message !== 'string' || message.length === 0) return undefined;
  if (message === 'Error message not available in production bundles.') return undefined;
  const name = text.match(/Error Code: (\w+)\./)?.[1];
  return name === undefined ? message : `${name}: ${message}`;
}

function diagnosticText(error: unknown): string {
  if (typeof error === 'string') return error;
  if (error == null || typeof error !== 'object') return '';
  const record = error as { readonly message?: unknown; readonly logs?: unknown; readonly context?: unknown };
  const parts: string[] = [];
  if (typeof record.message === 'string') parts.push(record.message);
  if (Array.isArray(record.logs)) {
    parts.push(record.logs.filter((line): line is string => typeof line === 'string').join('\n'));
  }
  if (record.context !== null && typeof record.context === 'object') {
    const logs = (record.context as { readonly logs?: unknown }).logs;
    if (Array.isArray(logs)) {
      parts.push(logs.filter((line): line is string => typeof line === 'string').join('\n'));
    }
  }
  return parts.join('\n');
}

export const planDemoFunding = (
  solLamports: bigint,
  usdcBaseUnits: bigint,
  requiredUsdcBaseUnits: bigint = MIN_USDC_BALANCE,
): FundingPlan => {
  const targetUsdcBalance = requiredUsdcBaseUnits > TARGET_USDC_BALANCE ? requiredUsdcBaseUnits : TARGET_USDC_BALANCE;
  return {
    ...(solLamports < MIN_SOL_BALANCE
      ? { sol: Number(TARGET_SOL_BALANCE - solLamports) / Number(LAMPORTS_PER_SOL) }
      : {}),
    ...(usdcBaseUnits < requiredUsdcBaseUnits
      ? { usdc: Number(targetUsdcBalance - usdcBaseUnits) / Number(USDC_BASE_UNITS) }
      : {}),
  };
};

const postFaucet = async (
  path: '/airdrop-sol' | '/mint-usdc',
  recipient: Address,
  amount: Record<string, number>,
): Promise<void> => {
  const response = await demoFaucetFetch(path, {
    method: 'POST',
    headers: { 'content-type': 'application/json' },
    body: JSON.stringify({ address: recipient, ...amount }),
  });
  if (!response.ok) {
    const body = (await response.json().catch(() => null)) as { error?: string } | null;
    throw new Error(body?.error ?? `faucet ${path} failed with HTTP ${response.status}`);
  }
};

export const readDemoWalletBalances = async (
  config: DemoConfig,
  owner: Address,
): Promise<readonly [solLamports: bigint, usdcBaseUnits: bigint]> => {
  const rpc = createSolanaRpc(config.rpcUrl);
  const [sol, tokenAccounts] = await Promise.all([
    rpc.getBalance(owner, { commitment: 'confirmed' }).send(),
    rpc
      .getTokenAccountsByOwner(
        owner,
        { mint: config.mints.joinUnderlying },
        { commitment: 'confirmed', encoding: 'jsonParsed' },
      )
      .send(),
  ]);
  const usdc = tokenAccounts.value.reduce((sum, tokenAccount) => {
    const data = tokenAccount.account.data as {
      readonly parsed?: { readonly info?: { readonly tokenAmount?: { readonly amount?: string } } };
    };
    const amount = data.parsed?.info?.tokenAmount?.amount;
    return sum + (amount === undefined ? 0n : BigInt(amount));
  }, 0n);
  return [sol.value, usdc];
};

export const ensureDemoFunding = async (
  config: DemoConfig,
  owner: Address,
  requiredUsdcBaseUnits: bigint = MIN_USDC_BALANCE,
): Promise<void> => {
  const [solLamports, usdcBaseUnits] = await readDemoWalletBalances(config, owner);
  const funding = planDemoFunding(solLamports, usdcBaseUnits, requiredUsdcBaseUnits);
  await Promise.all([
    ...(funding.sol === undefined ? [] : [postFaucet('/airdrop-sol', owner, { sol: funding.sol })]),
    ...(funding.usdc === undefined ? [] : [postFaucet('/mint-usdc', owner, { amount: funding.usdc })]),
  ]);
  for (let attempt = 0; attempt < 40; attempt += 1) {
    const [fundedSolLamports, fundedUsdcBaseUnits] = await readDemoWalletBalances(config, owner);
    const missing = planDemoFunding(fundedSolLamports, fundedUsdcBaseUnits, requiredUsdcBaseUnits);
    if (missing.sol === undefined && missing.usdc === undefined) return;
    await new Promise((resolve) => setTimeout(resolve, 250));
  }
  throw new Error('Demo funding was not confirmed within 10 seconds');
};

const responseJson = async (response: Response, name: string): Promise<unknown> => {
  if (!response.ok) {
    const body = (await response.json().catch(() => null)) as { error?: string } | null;
    throw new Error(body?.error ?? `${name} failed with HTTP ${response.status}`);
  }
  return response.json();
};

export const loadDemoConfig = async (): Promise<DemoConfig> =>
  parseDemoConfigResponse(await responseJson(await fetch('/api/demo-config'), 'demo config'));

export const readExactMessageSignature = (
  message: Uint8Array,
  signed:
    | {
        readonly content: Uint8Array;
        readonly signatures: Readonly<Record<string, Uint8Array | null | undefined>>;
      }
    | undefined,
  signerAddress: string,
): Uint8Array => {
  if (signed === undefined || signed.content.length !== message.length) {
    throw new Error('Wallet did not return the exact decrypt authorization message');
  }
  for (let index = 0; index < message.length; index += 1) {
    if (signed.content[index] !== message[index]) {
      throw new Error('Wallet modified the decrypt authorization message');
    }
  }
  const signature = signed.signatures[signerAddress];
  if (signature === undefined || signature === null) {
    throw new Error(`Wallet did not sign the decrypt request for ${signerAddress}`);
  }
  return new Uint8Array(signature);
};

const signatureForExactMessage = async (
  signer: ReturnType<typeof createMessageSignerFromWalletAccount>,
  message: Uint8Array,
): Promise<Uint8Array> => {
  const [signed] = await signer.modifyAndSignMessages([createSignableMessage(message)]);
  return readExactMessageSignature(message, signed, signer.address);
};

/**
 * The permit adapter: the SDK's `SolanaPermitWallet` from the selected `UiWalletAccount`, when the
 * wallet backs the one permit channel.
 *
 * The account object handed to the SDK is the wallet's own registered `WalletAccount` — reached
 * through the same registry accessor the official `@solana/wallet-account-signer` bridges through —
 * never a rebuilt lookalike, because wallets recognize their accounts by identity. A wallet whose
 * account does not list `solana:signOffchainMessage` yields `undefined`: reveals then refuse with
 * a clear message instead of falling back to raw message signing.
 */
export const permitWalletFromWalletAccount = (account: UiWalletAccount): SolanaPermitWallet | undefined => {
  if (!account.features.includes(SolanaSignOffchainMessage)) return undefined;
  const feature = getWalletAccountFeature(
    account,
    SolanaSignOffchainMessage,
  ) as SolanaSignOffchainMessageFeature[typeof SolanaSignOffchainMessage];
  const walletAccount = getWalletAccountForUiWalletAccount_DO_NOT_USE_OR_YOU_WILL_BE_FIRED(account);
  return { account: walletAccount, features: { [SolanaSignOffchainMessage]: feature } };
};

export const assertWalletAccountCapabilities = (account: UiWalletAccount, walletName: string): void => {
  if (!account.chains.includes('solana:localnet')) {
    throw new Error(
      `${walletName} has not enabled Solana localnet. Enable http://127.0.0.1:8899 in the wallet, then reconnect.`,
    );
  }
  if (!account.features.includes('solana:signTransaction')) {
    throw new Error(`${walletName} does not support transaction signing`);
  }
  if (!account.features.includes('solana:signMessage')) {
    throw new Error(`${walletName} does not support message signing`);
  }
};

export const connectWalletSession = async (
  account: UiWalletAccount,
  walletName: string,
  accountKey: string,
  isActive: () => boolean,
): Promise<DemoSession> => {
  const assertActive = (): void => {
    if (!isActive()) throw new Error('Wallet account changed while the action was running');
  };
  assertWalletAccountCapabilities(account, walletName);
  const config = await loadDemoConfig();
  assertActive();
  const signer = createTransactionSignerFromWalletAccount(account, 'solana:localnet');
  const messageSigner = createMessageSignerFromWalletAccount(account);
  await ensureDemoFunding(config, signer.address);
  assertActive();
  return {
    config,
    signer,
    signMessageExact: async (message) => {
      assertActive();
      const signature = await signatureForExactMessage(messageSigner, message);
      assertActive();
      return signature;
    },
    wallet: { kind: 'wallet-standard', name: walletName, accountKey },
    // The permit channel is exclusively `solana:signOffchainMessage`: a wallet that backs it signs
    // permits through the adapter above; one that does not gets a clear refusal at reveal time.
    permitWallet: permitWalletFromWalletAccount(account),
    isActive,
    assertActive,
  };
};

export const connectDemoSession = async (isActive: () => boolean = () => true): Promise<DemoSession> => {
  const assertActive = (): void => {
    if (!isActive()) throw new Error('Demo wallet session is no longer active');
  };
  // This browser's own burner: generated here, kept in local storage, funded through the faucet.
  const [config, secretKey] = await Promise.all([loadDemoConfig(), loadOrCreateBurnerSecretKey(window.localStorage)]);
  const signer = await createKeyPairSignerFromBytes(secretKey);
  await ensureDemoFunding(config, signer.address);
  assertActive();
  return {
    config,
    signer,
    signMessageExact: async (message) => {
      assertActive();
      const [signatures] = await signer.signMessages([createSignableMessage(message)]);
      const signature = signatures?.[signer.address];
      if (signature === undefined || signature === null) {
        throw new Error(`Demo wallet did not sign the decrypt request for ${signer.address}`);
      }
      assertActive();
      return new Uint8Array(signature);
    },
    wallet: { kind: 'burner', name: 'Demo wallet' },
    // The burner key doubles as a conforming sRFC-38 wallet: the permit path's one channel.
    permitWallet: solanaPermitWalletFromSecretKey(secretKey),
    isActive,
    assertActive,
  };
};
