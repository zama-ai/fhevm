import React from "react";
import { useOktaAuth } from "@okta/okta-react";
import { useNavigate } from "react-router-dom";

import { PageLayout } from "../../page-layout";
import { PageLoader } from "../../page-loader";

import wfImage from "../../../images/assets/dev-portal-architecture-diagram.svg";
import { SignupButton } from "../../buttons/signup-button";
import { LoginButton } from "../../buttons/login-button";

function OktaLogin() {
  const { authState } = useOktaAuth();
  const navigate = useNavigate();

  if (authState?.isAuthenticated) {
    navigate("/dashboard");
  }
  if (authState?.isPending) {
    return <PageLoader />;
  }

  return (
    <PageLayout>
      <div className="login-page">
        <h1>Welcome to Your Custom Dev Portal!</h1>
        <h2>
          <SignupButton isLink /> or <LoginButton isLink /> to get started.
        </h2>
        <div className="page-layout__focus">
          <img src={wfImage} width="100%" alt="flow-diagram" />
        </div>
      </div>
    </PageLayout>
  );
}

export default OktaLogin;
