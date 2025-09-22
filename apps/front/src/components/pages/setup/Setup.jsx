import React from "react";
import { PageLayout } from "../../page-layout";
import wfImage from "../../../images/assets/dev-portal-architecture-diagram.svg";
import { LoginButton } from '../../buttons/login-button';
import { SignupButton } from '../../buttons/signup-button';

function Setup(props) {
  return (
    <PageLayout>
      <div className="heading-centered">
        <h1>Dev Portal Setup</h1>
        <p className="description">
          Below is an architectural diagram of how is Developer Portal ties
          together various system to delivery a complete solution. For detailed
          set up instruction please see{" "}
          <a
            className="button__link"
            target="_blank"
            href="https://github.com/Moesif/moesif-developer-portal"
          >
            README and our github repo.
          </a>
        </p>
        <div className="buttons">
          <LoginButton isLink />
          <SignupButton />
        </div>
      </div>
      <div className="page-layout__focus">
        <img src={wfImage} style={{ padding: '30px'}}width="100%" alt="flow-diagram" />
      </div>
      <div style={{ height: "50" }} />
    </PageLayout>
  );
}

export default Setup;
