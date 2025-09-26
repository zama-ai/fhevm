import { PageLayout } from "../../page-layout";
import MoesifPlans from "../plans/MoesifPlans";
import { SignupButton } from "../../buttons/signup-button";
import { LoginButton } from "../../buttons/login-button";

import heroImage from "../../../images/assets/dev-portal-hero.svg";

function Home() {
  return (
    <PageLayout isHome>
      <section className="hero">
        <div className="hero-content">
          <h1>Zama Developer Platform</h1>
          <p>
            The Zama Developer Platform is your gateway to confidential-by-default blockchain apps<br/>Get started in minutes with secure APIs, usage analytics, and simple key management.
          </p>

          <div className="buttons">
            <LoginButton isLink />
            <SignupButton />
          </div>
        </div>
        <div className="hero-image">
          <img src={heroImage} alt="flow-diagram" />
        </div>
      </section>
      <MoesifPlans />
      <section style={{ paddingTop: "2em" }} />
    </PageLayout>
  );
}

export default Home;
