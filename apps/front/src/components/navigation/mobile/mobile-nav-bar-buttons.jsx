import React from "react";
import { useOktaAuth } from "@okta/okta-react";
import { useAuth0 } from "@auth0/auth0-react";
import { LoginButton } from "../../buttons/login-button";
import { LogoutButton } from "../../buttons/logout-button";
import { SignupButton } from "../../buttons/signup-button";

export const MobileNavBarButtons = () => {
  const oktaAuth = useOktaAuth();
  const oktaIsAuthenticated = oktaAuth?.authState?.isAuthenticated;

  const auth0Auth = useAuth0();
  const auth0IsAuthenticated = auth0Auth?.isAuthenticated;

  let isAuthenticated;

  if (import.meta.env.REACT_APP_AUTH_PROVIDER === "Okta") {
    isAuthenticated = oktaIsAuthenticated;
  } else if (import.meta.env.REACT_APP_AUTH_PROVIDER === "Auth0") {
    isAuthenticated = auth0IsAuthenticated;
  }

  return (
    <div className="mobile-nav-bar__buttons">
      {!isAuthenticated && import.meta.env.REACT_APP_AUTH_PROVIDER === "Okta" && (
        <>
          <LoginButton />
        </>
      )}
      {!isAuthenticated && import.meta.env.REACT_APP_AUTH_PROVIDER === "Auth0" && (
        <>
          <SignupButton />
          <LoginButton />
        </>
      )}
      {isAuthenticated && (
        <>
          <LogoutButton />
        </>
      )}
    </div>
  );
};
