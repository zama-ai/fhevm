import { getUiWalletAccountStorageKey, useConnect, useDisconnect, useWallets, type UiWallet } from '@wallet-standard/react';
import { useEffect } from 'react';

import { connectWalletSession, type DemoSession } from './demoSession';

type Props = {
  readonly connection:
    | { readonly kind: 'disconnected' | 'connecting' | 'error' }
    | { readonly kind: 'ready'; readonly session: DemoSession };
  readonly disabled: boolean;
  readonly onBurnerConnect: () => void;
  readonly onConnect: (connect: (isActive: () => boolean) => Promise<DemoSession>) => void;
  readonly onDisconnect: () => void;
};

const walletNamed = (wallets: readonly UiWallet[], name: string): UiWallet | undefined =>
  wallets.find((wallet) => wallet.name === name);

function PhantomConnect({
  wallet,
  disabled,
  onConnect,
}: {
  readonly wallet: UiWallet;
  readonly disabled: boolean;
  readonly onConnect: Props['onConnect'];
}) {
  const [isConnecting, connect] = useConnect(wallet);
  return (
    <button
      className="wallet-button"
      type="button"
      disabled={disabled || isConnecting}
      onClick={() =>
        onConnect(async (isActive) => {
          const accounts = await connect();
          const account = accounts[0];
          if (account === undefined) throw new Error(`${wallet.name} did not authorize a Solana account`);
          return connectWalletSession(
            account,
            wallet.name,
            getUiWalletAccountStorageKey(account),
            isActive,
          );
        })
      }
    >
      {isConnecting ? 'Opening Phantom…' : 'Connect Phantom'}
    </button>
  );
}

function DisconnectAuthorizedWallet({
  wallet,
  disabled,
  onDisconnect,
}: {
  readonly wallet: UiWallet;
  readonly disabled: boolean;
  readonly onDisconnect: () => void;
}) {
  const [isDisconnecting, disconnect] = useDisconnect(wallet);
  return (
    <button
      className="wallet-demo-button"
      type="button"
      disabled={disabled || isDisconnecting}
      onClick={() => {
        void disconnect().finally(onDisconnect);
      }}
    >
      {isDisconnecting ? 'Disconnecting…' : 'Disconnect'}
    </button>
  );
}

function ConnectedWallet({
  wallet,
  session,
  disabled,
  onDisconnect,
}: {
  readonly wallet: UiWallet;
  readonly session: DemoSession;
  readonly disabled: boolean;
  readonly onDisconnect: () => void;
}) {
  const [isDisconnecting, disconnect] = useDisconnect(wallet);
  const walletMetadata = session.wallet;
  const selectedAccount =
    walletMetadata.kind === 'wallet-standard'
      ? wallet.accounts.find((account) => getUiWalletAccountStorageKey(account) === walletMetadata.accountKey)
      : undefined;

  useEffect(() => {
    if (selectedAccount === undefined || selectedAccount.address !== session.signer.address) onDisconnect();
  }, [onDisconnect, selectedAccount, session.signer.address]);

  return (
    <button
      className="wallet-button"
      type="button"
      disabled={disabled || isDisconnecting}
      title={`Disconnect ${wallet.name}`}
      onClick={() => {
        void disconnect().finally(onDisconnect);
      }}
    >
      {isDisconnecting ? 'Disconnecting…' : `${session.signer.address.slice(0, 4)}…${session.signer.address.slice(-4)}`}
    </button>
  );
}

function MissingConnectedWallet({ onDisconnect }: { readonly onDisconnect: () => void }) {
  useEffect(onDisconnect, [onDisconnect]);
  return (
    <button className="wallet-button" type="button" disabled>
      Wallet disconnected
    </button>
  );
}

export function WalletControl({
  connection,
  disabled,
  onBurnerConnect,
  onConnect,
  onDisconnect,
}: Props) {
  const wallets = useWallets();
  const phantom = wallets.find((wallet) => wallet.name.toLowerCase() === 'phantom');

  if (connection.kind === 'ready') {
    if (connection.session.wallet.kind === 'wallet-standard') {
      const wallet = walletNamed(wallets, connection.session.wallet.name);
      if (wallet !== undefined) {
        return (
          <ConnectedWallet
            wallet={wallet}
            session={connection.session}
            disabled={disabled}
            onDisconnect={onDisconnect}
          />
        );
      }
      return <MissingConnectedWallet onDisconnect={onDisconnect} />;
    }
    return (
      <button className="wallet-button" type="button" disabled={disabled} onClick={onDisconnect}>
        {`${connection.session.signer.address.slice(0, 4)}…${connection.session.signer.address.slice(-4)}`}
      </button>
    );
  }

  if (phantom === undefined) {
    return (
      <button
        className="wallet-button"
        type="button"
        disabled={disabled || connection.kind === 'connecting'}
        onClick={onBurnerConnect}
      >
        {connection.kind === 'connecting' ? 'Funding wallet…' : 'Start demo'}
      </button>
    );
  }

  return (
    <div className="wallet-connect-options">
      <PhantomConnect wallet={phantom} disabled={disabled} onConnect={onConnect} />
      {connection.kind === 'error' && phantom.accounts.length > 0 ? (
        <DisconnectAuthorizedWallet wallet={phantom} disabled={disabled} onDisconnect={onDisconnect} />
      ) : (
        <button
          className="wallet-demo-button"
          type="button"
          disabled={disabled || connection.kind === 'connecting'}
          onClick={onBurnerConnect}
        >
          Demo wallet
        </button>
      )}
    </div>
  );
}
