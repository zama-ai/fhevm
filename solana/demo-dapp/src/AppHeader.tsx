import type { DemoController } from './useDemoController';
import { WalletControl } from './WalletControl';

export function AppHeader({ controller }: { readonly controller: DemoController }) {
  const { connection, deposit } = controller.state;
  const { actions } = controller;
  return (
    <header className="topbar">
      <a className="brand" href="/" aria-label="Confidential Vault home">
        <span className="brand-mark">Z</span>
        <span>Confidential Vault</span>
      </a>
      <div className="network-pill">
        <span className="network-dot" />
        Solana localnet
      </div>
      <WalletControl
        connection={connection}
        disabled={deposit.kind === 'running'}
        onBurnerConnect={actions.connectBurner}
        onConnect={(createSession) => void actions.connect(createSession)}
        onDisconnect={actions.disconnect}
      />
    </header>
  );
}
