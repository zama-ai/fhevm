import React from "react";
import { useAuth0 } from "@auth0/auth0-react";

import { PageLayout } from "../../page-layout";
import { PageLoader } from "../../page-loader";

import wfImage from "../../../images/assets/dev-portal-architecture-diagram.svg";
import { SignupButton } from "../../buttons/signup-button";
import { LoginButton } from "../../buttons/login-button";

function Auth0Login() {
  const { isLoading } = useAuth0();

  if (isLoading) {
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

export default Auth0Login;
