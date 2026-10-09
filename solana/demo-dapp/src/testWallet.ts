import { SolanaSignTransaction, type SolanaTransactionVersion } from '@solana/wallet-standard-features';
import type { IdentifierString, Wallet, WalletAccount } from '@wallet-standard/base';
import type { UiWallet, UiWalletAccount } from '@wallet-standard/react';
import {
  getOrCreateUiWalletAccountForStandardWalletAccount_DO_NOT_USE_OR_YOU_WILL_BE_FIRED,
  getOrCreateUiWalletForStandardWallet_DO_NOT_USE_OR_YOU_WILL_BE_FIRED,
} from '@wallet-standard/ui-registry';

type TestWalletOptions = {
  readonly name?: string;
  readonly supportedTransactionVersions?: readonly SolanaTransactionVersion[];
  readonly chains?: readonly IdentifierString[];
  readonly accountFeatures?: readonly IdentifierString[];
};

/** A registered Wallet Standard wallet, so the registry's feature lookups see what a browser wallet registers. */
const standardTestWallet = ({
  name = 'Test wallet',
  supportedTransactionVersions = ['legacy', 0, 1],
  chains = ['solana:localnet'],
  accountFeatures = [SolanaSignTransaction, 'solana:signMessage'],
}: TestWalletOptions): Wallet => {
  const account: WalletAccount = {
    address: '11111111111111111111111111111111',
    publicKey: new Uint8Array(32),
    chains,
    features: accountFeatures,
  };
  return {
    version: '1.0.0',
    name,
    icon: 'data:image/svg+xml;base64,',
    chains,
    features: {
      'standard:connect': { version: '1.0.0', connect: async () => ({ accounts: [account] }) },
      [SolanaSignTransaction]: { version: '1.0.0', supportedTransactionVersions, signTransaction: async () => [] },
    },
    accounts: [account],
  };
};

export const testUiWallet = (options: TestWalletOptions = {}): UiWallet =>
  getOrCreateUiWalletForStandardWallet_DO_NOT_USE_OR_YOU_WILL_BE_FIRED(standardTestWallet(options));

export const testUiWalletAccount = (options: TestWalletOptions = {}): UiWalletAccount => {
  const wallet = standardTestWallet(options);
  return getOrCreateUiWalletAccountForStandardWalletAccount_DO_NOT_USE_OR_YOU_WILL_BE_FIRED(wallet, wallet.accounts[0]!);
};
