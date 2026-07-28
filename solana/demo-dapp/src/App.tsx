import { AppHeader } from './AppHeader';
import { DepositJourney } from './DepositJourney';
import { DeveloperEvidence } from './DeveloperEvidence';
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
          <h1>Earn yield privately</h1>
          <p className="hero-copy">Shield USDC. Keep your position private.</p>
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
        <DeveloperEvidence key={controller.state.generation} controller={controller} />
      </main>
    </div>
  );
}
