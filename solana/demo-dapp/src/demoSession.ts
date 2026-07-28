import {
  createKeyPairSignerFromBytes,
  createSignableMessage,
  createSolanaRpc,
  type Address,
  type TransactionSigner,
} from "@solana/kit";
import {
  createMessageSignerFromWalletAccount,
  createTransactionSignerFromWalletAccount,
} from "@solana/wallet-account-signer";
import type { UiWalletAccount } from "@wallet-standard/react";

import { demoApiFetch, demoFaucetFetch } from "./demoAuthorization";
import { parseDemoConfig, parseDemoConfigResponse, type DemoConfig } from "./demoConfig";

export { parseDemoConfigResponse, type DemoConfig } from "./demoConfig";

export type DemoSession = {
  readonly config: DemoConfig;
  readonly signer: TransactionSigner;
  readonly signMessageExact: (message: Uint8Array) => Promise<Uint8Array>;
  readonly wallet:
    | { readonly kind: "burner"; readonly name: "Demo wallet" }
    | { readonly kind: "wallet-standard"; readonly name: string; readonly accountKey: string };
  readonly isActive: () => boolean;
  readonly assertActive: () => void;
};

export type DemoSessionResponse = {
  readonly config: DemoConfig;
  readonly aliceKeypair: number[];
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

export const describeWalletError = (
  error: unknown,
  context: "connect" | "transaction" | "reveal",
): string => {
  const candidate = error as { readonly code?: unknown; readonly message?: unknown };
  const rejected =
    candidate?.code === 4001 ||
    candidate?.code === 4_001_000 ||
    (typeof candidate?.message === "string" && /user rejected|request rejected|cancelled by user/i.test(candidate.message));
  if (!rejected) return error instanceof Error ? error.message : String(error);
  if (context === "connect") return "Wallet connection cancelled";
  if (context === "reveal") return "Signature cancelled — your confidential balance remains hidden";
  return "Signature cancelled — nothing new was sent; any confirmed step is saved";
};

export const planDemoFunding = (
  solLamports: bigint,
  usdcBaseUnits: bigint,
  requiredUsdcBaseUnits: bigint = MIN_USDC_BALANCE,
): FundingPlan => {
  const targetUsdcBalance = requiredUsdcBaseUnits > TARGET_USDC_BALANCE
    ? requiredUsdcBaseUnits
    : TARGET_USDC_BALANCE;
  return {
  ...(solLamports < MIN_SOL_BALANCE
    ? { sol: Number(TARGET_SOL_BALANCE - solLamports) / Number(LAMPORTS_PER_SOL) }
    : {}),
  ...(usdcBaseUnits < requiredUsdcBaseUnits
    ? { usdc: Number(targetUsdcBalance - usdcBaseUnits) / Number(USDC_BASE_UNITS) }
    : {}),
  };
};

const object = (value: unknown, name: string): Record<string, unknown> => {
  if (typeof value !== "object" || value === null) throw new Error(`${name} must be an object`);
  return value as Record<string, unknown>;
};

export const parseDemoSessionResponse = (value: unknown): DemoSessionResponse => {
  const root = object(value, "demo session");
  const candidate = root.aliceKeypair;
  if (
    !Array.isArray(candidate) ||
    candidate.length !== 64 ||
    candidate.some((byte) => !Number.isInteger(byte) || byte < 0 || byte > 255)
  ) {
    throw new Error("demo session aliceKeypair must contain exactly 64 bytes");
  }
  return {
    config: parseDemoConfig(root.config),
    aliceKeypair: candidate as number[],
  };
};

const postFaucet = async (
  path: "/airdrop-sol" | "/mint-usdc",
  recipient: Address,
  amount: Record<string, number>,
): Promise<void> => {
  const response = await demoFaucetFetch(path, {
    method: "POST",
    headers: { "content-type": "application/json" },
    body: JSON.stringify({ address: recipient, ...amount }),
  });
  if (!response.ok) {
    const body = (await response.json().catch(() => null)) as { error?: string } | null;
    throw new Error(body?.error ?? `faucet ${path} failed with HTTP ${response.status}`);
  }
};

const readBalances = async (config: DemoConfig, owner: Address): Promise<readonly [bigint, bigint]> => {
  const rpc = createSolanaRpc(config.rpcUrl);
  const [sol, tokenAccounts] = await Promise.all([
    rpc.getBalance(owner, { commitment: "confirmed" }).send(),
    rpc
      .getTokenAccountsByOwner(
        owner,
        { mint: config.mints.joinUnderlying },
        { commitment: "confirmed", encoding: "jsonParsed" },
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
  const [solLamports, usdcBaseUnits] = await readBalances(config, owner);
  const funding = planDemoFunding(solLamports, usdcBaseUnits, requiredUsdcBaseUnits);
  await Promise.all([
    ...(funding.sol === undefined ? [] : [postFaucet("/airdrop-sol", owner, { sol: funding.sol })]),
    ...(funding.usdc === undefined ? [] : [postFaucet("/mint-usdc", owner, { amount: funding.usdc })]),
  ]);
  for (let attempt = 0; attempt < 40; attempt += 1) {
    const [fundedSolLamports, fundedUsdcBaseUnits] = await readBalances(config, owner);
    const missing = planDemoFunding(fundedSolLamports, fundedUsdcBaseUnits, requiredUsdcBaseUnits);
    if (missing.sol === undefined && missing.usdc === undefined) return;
    await new Promise((resolve) => setTimeout(resolve, 250));
  }
  throw new Error("Demo funding was not confirmed within 10 seconds");
};

const responseJson = async (response: Response, name: string): Promise<unknown> => {
  if (!response.ok) {
    const body = (await response.json().catch(() => null)) as { error?: string } | null;
    throw new Error(body?.error ?? `${name} failed with HTTP ${response.status}`);
  }
  return response.json();
};

export const loadDemoConfig = async (): Promise<DemoConfig> =>
  parseDemoConfigResponse(await responseJson(await fetch("/api/demo-config"), "demo config"));

export const readExactMessageSignature = (
  message: Uint8Array,
  signed: {
    readonly content: Uint8Array;
    readonly signatures: Readonly<Record<string, Uint8Array | null | undefined>>;
  } | undefined,
  signerAddress: string,
): Uint8Array => {
  if (signed === undefined || signed.content.length !== message.length) {
    throw new Error("Wallet did not return the exact decrypt authorization message");
  }
  for (let index = 0; index < message.length; index += 1) {
    if (signed.content[index] !== message[index]) {
      throw new Error("Wallet modified the decrypt authorization message");
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

export const assertWalletAccountCapabilities = (account: UiWalletAccount, walletName: string): void => {
  if (!account.chains.includes("solana:localnet")) {
    throw new Error(
      `${walletName} has not enabled Solana localnet. Enable http://127.0.0.1:8899 in the wallet, then reconnect.`,
    );
  }
  if (!account.features.includes("solana:signTransaction")) {
    throw new Error(`${walletName} does not support transaction signing`);
  }
  if (!account.features.includes("solana:signMessage")) {
    throw new Error(`${walletName} does not support message signing required for private balance reveals`);
  }
};

export const connectWalletSession = async (
  account: UiWalletAccount,
  walletName: string,
  accountKey: string,
  isActive: () => boolean,
): Promise<DemoSession> => {
  const assertActive = (): void => {
    if (!isActive()) throw new Error("Wallet account changed while the action was running");
  };
  assertWalletAccountCapabilities(account, walletName);
  const config = await loadDemoConfig();
  assertActive();
  const signer = createTransactionSignerFromWalletAccount(account, "solana:localnet");
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
    wallet: { kind: "wallet-standard", name: walletName, accountKey },
    isActive,
    assertActive,
  };
};

export const connectDemoSession = async (isActive: () => boolean = () => true): Promise<DemoSession> => {
  const assertActive = (): void => {
    if (!isActive()) throw new Error("Demo wallet session is no longer active");
  };
  const response = await demoApiFetch("/api/demo-session");
  const { config, aliceKeypair } = parseDemoSessionResponse(await responseJson(response, "demo session"));
  const signer = await createKeyPairSignerFromBytes(Uint8Array.from(aliceKeypair));
  if (signer.address !== config.personas.alice) {
    throw new Error(`burner signer ${signer.address} does not match seeded Alice ${config.personas.alice}`);
  }
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
    wallet: { kind: "burner", name: "Demo wallet" },
    isActive,
    assertActive,
  };
};
