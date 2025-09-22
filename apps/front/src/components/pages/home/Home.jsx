import React from "react";

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
          <h1>My Dev Portal</h1>
          <p>
            Welcome to your custom developer portal. <br></br>
            Prompt customers to the links below.
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
