import { useOktaAuth } from "@okta/okta-react";
import { useAuth0 } from "@auth0/auth0-react";
import React from "react";

const LoginButtonWithOkta = ({ isLink }) => {
  const { oktaAuth } = useOktaAuth();

  const handleLogin = async () => {
    window.moesif?.track("clicked-login", {
      provider: "Okta",
    });
    await oktaAuth.signInWithRedirect({ originalUri: "/dashboard" });
  };

  const className = isLink ? " button__link" : "button__login";

  return (
    <button className={className} onClick={handleLogin}>
      Log In
    </button>
  );
};

const LoginButtonWithAuth0 = ({ isLink }) => {
  const { loginWithRedirect } = useAuth0();

  const handleLogin = async () => {
    window.moesif?.track("clicked-login", {
      provider: "Auth0",
    });
    await loginWithRedirect({
      appState: {
        returnTo: "/dashboard",
      },
      authorizationParams: {
        prompt: "login",
      },
      scope: "openid profile email offline_access",
    });
  };
  const className = isLink ? " button__link" : "button__login";

  return (
    <button className={className} onClick={handleLogin}>
      Log In
    </button>
  );
};

export const LoginButton = ({ isLink }) => {
  if (import.meta.env.REACT_APP_AUTH_PROVIDER === "Okta") {
    return <LoginButtonWithOkta isLink={isLink} />;
  } else if (import.meta.env.REACT_APP_AUTH_PROVIDER === "Auth0") {
    return <LoginButtonWithAuth0 isLink={isLink} />;
  } else {
    return null; // or some error message
  }
};
