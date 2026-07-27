import { AppHeader } from './AppHeader';
import { DepositJourney } from './DepositJourney';
import { PortfolioOverview } from './PortfolioOverview';
import { RedeemJourney } from './RedeemJourney';
import { useDemoController } from './useDemoController';

export function App() {
  const controller = useDemoController();
  const { connection } = controller.state;

  return (
    <div className="app-shell">
      <AppHeader controller={controller} />
      <main>
        <section className="hero">
          <p className="eyebrow">Private yield, familiar Solana flow</p>
          <h1>Your confidential portfolio</h1>
          <p className="hero-copy">
            Deposit USDC into an encrypted vault. Your position stays private; settlement remains verifiable on Solana.
          </p>
        </section>

        {connection.kind === 'error' && (
          <div className="error-banner" role="alert">
            <span>{connection.message}</span>
            <button type="button" onClick={controller.actions.disconnect}>
              Dismiss
            </button>
          </div>
        )}

        <PortfolioOverview controller={controller} />
        <DepositJourney controller={controller} />
        <RedeemJourney controller={controller} />
      </main>
    </div>
  );
}
