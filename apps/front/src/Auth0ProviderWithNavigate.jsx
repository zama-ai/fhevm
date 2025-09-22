import { Auth0Provider } from "@auth0/auth0-react";
import React from "react";
import { useNavigate } from "react-router-dom";

export const Auth0ProviderWithNavigate = ({ children }) => {
  const navigate = useNavigate();

  const onRedirectCallback = (appState) => {
    navigate(appState?.returnTo || window.location.pathname);
  };

  return (
    <Auth0Provider
      domain={import.meta.env.REACT_APP_AUTH0_DOMAIN}
      clientId={import.meta.env.REACT_APP_AUTH0_CLIENT_ID}
      useRefreshTokens={true} // Enables Refresh Tokens
      cacheLocation="localstorage" // Required when using Refresh Tokens
      onRedirectCallback={onRedirectCallback}
      authorizationParams={{
        redirect_uri: window.location.origin,
        // optional, up to you to set an audience.
        // audience: import.meta.env.REACT_APP_DEV_PORTAL_API_SERVER,
        // if you set an audience, be sure to register the app in auth0.
        scope: "openid profile email offline_access", // Request offline_access scope
      }}
    >
      {children}
    </Auth0Provider>
  );
};
