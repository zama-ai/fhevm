import React from "react";
import { useOktaAuth } from "@okta/okta-react";
import { NavBarTab } from "./nav-bar-tab";
import preLoginMenu from "./pre-login-menu.json";
import postLoginMenu from "./post-login-menu.json";

const OktaNavBarTabs = () => {
  const { authState } = useOktaAuth();
  const isAuthenticated = authState?.isAuthenticated;

  const menus = isAuthenticated ? postLoginMenu : preLoginMenu;

  return (
    <div className="nav-bar__tabs">
      {menus.map((item) => (
        <NavBarTab key={item.path} path={item.path} label={item.label} />
      ))}
    </div>
  );
};

export default OktaNavBarTabs;
