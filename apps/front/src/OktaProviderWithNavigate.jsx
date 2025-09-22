import React from "react";
import { useNavigate } from "react-router-dom";
import { Security } from "@okta/okta-react";
import { OktaAuth } from "@okta/okta-auth-js";

export const OktaProviderWithNavigate = ({ children }) => {
  const navigate = useNavigate();

  const oktaAuth = new OktaAuth({
    issuer: import.meta.env.REACT_APP_OKTA_ORG_URL,
    clientId: import.meta.env.REACT_APP_OKTA_CLIENT_ID,
    redirectUri: window.location.origin + "/login/callback",
  });

  // Assign onAuthRequired after creation since useNavigate is not available outside of a component
  oktaAuth.options.onAuthRequired = () => navigate("/login");

  const restoreOriginalUri = async (_oktaAuth, originalUri) => {
    navigate(originalUri);
  };

  return (
    <Security oktaAuth={oktaAuth} restoreOriginalUri={restoreOriginalUri}>
      {children}
    </Security>
  );
};
