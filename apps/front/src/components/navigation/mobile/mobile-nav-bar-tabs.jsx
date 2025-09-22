import React from "react";
import { useOktaAuth } from "@okta/okta-react";
import { useAuth0 } from "@auth0/auth0-react";
import { MobileNavBarTab } from "./mobile-nav-bar-tab";

export const MobileNavBarTabs = ({ handleClick }) => {
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
    <div className="mobile-nav-bar__tabs">
      {isAuthenticated && (
        <>
          <MobileNavBarTab
            path="/settings"
            label="Settings"
            handleClick={handleClick}
          />
          <MobileNavBarTab
            path="/dashboard"
            label="Dashboard"
            handleClick={handleClick}
          />
          <MobileNavBarTab
            path="/keys"
            label="Keys"
            handleClick={handleClick}
          />
        </>
      )}
    </div>
  );
};
