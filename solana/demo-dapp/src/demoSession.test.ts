import { ZAMA_HOST_PROGRAM_ADDRESS } from '@fhevm/sdk/solana/host';
import { describe, expect, test } from 'vitest';
import type { UiWalletAccount } from '@wallet-standard/react';
import type { Wallet, WalletAccount } from '@wallet-standard/base';
import { SolanaSignOffchainMessage } from '@solana/wallet-standard-features';
import { getOrCreateUiWalletAccountForStandardWalletAccount_DO_NOT_USE_OR_YOU_WILL_BE_FIRED } from '@wallet-standard/ui-registry';
import { solanaPermitWalletFromSecretKey } from '@fhevm/sdk/solana';
import { ZAMA_HOST_PROGRAM_ADDRESS } from '@fhevm/sdk/solana/host';
import { CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS } from '@fhevm/confidential-token';

import {
  assertWalletAccountCapabilities,
  describeWalletError,
  parseDemoConfigResponse,
  permitWalletFromWalletAccount,
  planDemoFunding,
  readExactMessageSignature,
} from './demoSession';
import { parseRuntimeDemoConfig } from './demoConfig';

const validResponse = {
  config: {
    source: 'demo-config',
    network: 'localnet',
    demoBootId: 'test-boot',
    chainId: '72057594037940281',
    rpcUrl: 'http://127.0.0.1:8899',
    wsUrl: 'ws://127.0.0.1:8900',
    relayerUrl: 'http://127.0.0.1:3000',
    aclProgram: '0xb825643b79bf4499ff31ccfe8a297aa0eb30a8a0cc1f2319e82176c1b4d65e71',
    userDecryptContextId: '123',
    kmsSigners: [`0x${'01'.repeat(20)}`],
    kmsEpochId: `0x${'00'.repeat(32)}`,
    fheParameter: 'test',
    gatewayChainId: '31337',
    gatewayDecryptionContract: `0x${'aa'.repeat(20)}`,
    authorityFundingLamports: '1000000',
    hostConfig: '11111111111111111111111111111111',
    kmsContext: '11111111111111111111111111111111',
    vault: '11111111111111111111111111111111',
    programs: {
      batcher: '11111111111111111111111111111111',
      token: '11111111111111111111111111111111',
      vault: '11111111111111111111111111111111',
      host: ZAMA_HOST_PROGRAM_ADDRESS,
    },
    mints: {
      joinUnderlying: '11111111111111111111111111111111',
      payoutUnderlying: '11111111111111111111111111111111',
      joinConfidential: '11111111111111111111111111111111',
      payoutConfidential: '11111111111111111111111111111111',
    },
    batchers: {
      deposit: {
        batcher: '11111111111111111111111111111111',
        lookupTable: '11111111111111111111111111111111',
      },
      redeem: {
        batcher: '11111111111111111111111111111111',
        lookupTable: '11111111111111111111111111111111',
      },
    },
    personas: {
      keeper: '11111111111111111111111111111111',
      alice: '11111111111111111111111111111111',
    },
  },
};

describe('parseDemoConfigResponse', () => {
  test('rejects a non-local RPC on localnet', () => {
    expect(() =>
      parseDemoConfigResponse({
        config: { ...validResponse.config, rpcUrl: 'https://api.mainnet-beta.solana.com' },
      }),
    ).toThrow('must use http://127.0.0.1');
  });

  test('parses the public configuration', () => {
    expect(parseDemoConfigResponse({ config: validResponse.config })).toEqual(validResponse.config);
  });

  test('rejects a host program spelled differently as bytes32 and base58', () => {
    expect(() =>
      parseDemoConfigResponse({
        config: {
          ...validResponse.config,
          programs: { ...validResponse.config.programs, host: '11111111111111111111111111111111' },
        },
      }),
    ).toThrow('aclProgram and demo config.programs.host name different programs');
  });

  test('binds the lifecycle boot to a seeded runtime config', () => {
    const { demoBootId: _omitted, ...runtimeConfig } = validResponse.config;
    expect(parseRuntimeDemoConfig(runtimeConfig, 'current-boot')).toEqual({
      ...validResponse.config,
      demoBootId: 'current-boot',
    });
  });
});

describe('planDemoFunding', () => {
  test('does not fund a healthy reconnect', () => {
    expect(planDemoFunding(4_900_000_000n, 900_000_000n)).toEqual({});
  });

  test('tops each missing asset up to its demo target', () => {
    expect(planDemoFunding(25_000_000n, 50_000_000n)).toEqual({
      sol: 0.2,
      usdc: 950,
    });
  });

  test('retries only the asset that is still below its safety threshold', () => {
    expect(planDemoFunding(5_000_000_000n, 0n)).toEqual({ usdc: 1_000 });
    expect(planDemoFunding(0n, 1_000_000_000n)).toEqual({ sol: 0.2 });
  });

  test('funds the requested deposit when it is above the default target', () => {
    expect(planDemoFunding(5_000_000_000n, 900_000_000n, 1_000_000_000n)).toEqual({ usdc: 100 });
    expect(planDemoFunding(5_000_000_000n, 900_000_000n, 800_000_000n)).toEqual({});
  });
});

describe('the permit adapter', () => {
  // The SDK's own headless wallet doubles as the standard wallet under test: its account and
  // feature object are exactly what a conforming browser wallet registers, so the adapter's output
  // is checked against real objects rather than shapes invented here.
  const headless = solanaPermitWalletFromSecretKey(new Uint8Array(32).fill(9));
  const feature = headless.features[SolanaSignOffchainMessage];
  const standardAccount: WalletAccount = headless.account;
  const standardWallet: Wallet = {
    version: '1.0.0',
    name: 'Fake Phantom',
    icon: 'data:image/svg+xml;base64,',
    chains: ['solana:localnet'],
    features: { [SolanaSignOffchainMessage]: feature },
    accounts: [standardAccount],
  };

  test("wires the wallet's registered account and feature object through, untouched", () => {
    const uiAccount = getOrCreateUiWalletAccountForStandardWalletAccount_DO_NOT_USE_OR_YOU_WILL_BE_FIRED(
      standardWallet,
      standardAccount,
    );
    const permitWallet = permitWalletFromWalletAccount(uiAccount);
    if (permitWallet === undefined) throw new Error('expected a permit wallet from a conforming account');
    // Identity, not equality: wallets recognize the accounts they registered, and the SDK hands
    // the account object back to the feature verbatim.
    expect(permitWallet.account).toBe(standardAccount);
    expect(permitWallet.features[SolanaSignOffchainMessage]).toBe(feature);
  });

  test('yields undefined for an account that does not list the feature — reveals then refuse clearly', () => {
    const bareAccount: WalletAccount = { ...standardAccount, features: ['solana:signMessage'] };
    const bareWallet: Wallet = { ...standardWallet, features: {}, accounts: [bareAccount] };
    const uiAccount = getOrCreateUiWalletAccountForStandardWalletAccount_DO_NOT_USE_OR_YOU_WILL_BE_FIRED(
      bareWallet,
      bareAccount,
    );
    expect(permitWalletFromWalletAccount(uiAccount)).toBeUndefined();
  });
});

describe('Wallet Standard boundary', () => {
  const walletAccount = (overrides: Partial<UiWalletAccount> = {}): UiWalletAccount =>
    ({
      address: '11111111111111111111111111111111',
      chains: ['solana:localnet'],
      features: ['solana:signTransaction', 'solana:signMessage'],
      ...overrides,
    }) as UiWalletAccount;

  test('requires localnet transaction and exact-message capabilities before funding', () => {
    expect(() => assertWalletAccountCapabilities(walletAccount(), 'Phantom')).not.toThrow();
    expect(() => assertWalletAccountCapabilities(walletAccount({ chains: ['solana:devnet'] }), 'Phantom')).toThrow(
      'has not enabled Solana localnet',
    );
    expect(() =>
      assertWalletAccountCapabilities(walletAccount({ features: ['solana:signMessage'] }), 'Phantom'),
    ).toThrow('does not support transaction signing');
    expect(() =>
      assertWalletAccountCapabilities(walletAccount({ features: ['solana:signTransaction'] }), 'Phantom'),
    ).toThrow('does not support message signing');
  });

  test('accepts an unchanged decrypt preimage and copies its signature', () => {
    const signature = new Uint8Array([7, 8, 9]);
    expect(
      readExactMessageSignature(
        new Uint8Array([1, 2, 3]),
        {
          content: new Uint8Array([1, 2, 3]),
          signatures: { '11111111111111111111111111111111': signature },
        },
        '11111111111111111111111111111111',
      ),
    ).toEqual(signature);
  });

  test('rejects modified messages and missing signatures', () => {
    expect(() =>
      readExactMessageSignature(
        new Uint8Array([1, 2, 3]),
        { content: new Uint8Array([1, 4, 3]), signatures: {} },
        '11111111111111111111111111111111',
      ),
    ).toThrow('modified');
    expect(() =>
      readExactMessageSignature(
        new Uint8Array([1, 2, 3]),
        { content: new Uint8Array([1, 2, 3]), signatures: {} },
        '11111111111111111111111111111111',
      ),
    ).toThrow('did not sign');
  });

  test('turns wallet rejection codes into actionable, stage-specific copy', () => {
    expect(describeWalletError({ code: 4_001_000 }, 'connect')).toBe('Wallet connection cancelled');
    expect(describeWalletError({ code: 4001 }, 'transaction')).toContain('any confirmed step is saved');
    expect(describeWalletError(new Error('User rejected the request'), 'reveal')).toContain('balance remains hidden');
  });

  test('decodes a missing journal when the host program identity is in the logs', () => {
    expect(
      describeWalletError(
        new Error(
          [
            `Program ${ZAMA_HOST_PROGRAM_ADDRESS} invoke [1]`,
            'Program log: AnchorError caused by account: transient_store. Error Code: TransientStoreNotOpened. Error Number: 6080. Error Message: transient store must be opened for this transaction and closed last.',
            `Program ${ZAMA_HOST_PROGRAM_ADDRESS} failed: custom program error: 0x17c0`,
          ].join('\n'),
        ),
        'transaction',
      ),
    ).toBe('TransientStoreNotOpened: transient store must be opened for this transaction and closed last');
  });

  test('leaves a token OwnerMismatch as the original diagnostic', () => {
    const original = [
      `Program ${CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS} invoke [1]`,
      'Program log: AnchorError caused by account: authority. Error Code: OwnerMismatch. Error Number: 6000. Error Message: owner mismatch.',
      `Program ${CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS} failed: custom program error: 0x1770`,
    ].join('\n');
    expect(describeWalletError(new Error(original), 'transaction')).toBe(original);
  });

  test('leaves a numeric 6000 without program identity unclassified', () => {
    const original = 'InstructionError: [1, {"Custom":6000}]';
    expect(describeWalletError(new Error(original), 'transaction')).toBe(original);
  });

  test('leaves a network failure unchanged', () => {
    expect(describeWalletError(new Error('fetch failed'), 'transaction')).toBe('fetch failed');
  });
});
