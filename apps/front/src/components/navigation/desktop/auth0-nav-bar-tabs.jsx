import React from "react";
import { useAuth0 } from "@auth0/auth0-react";
import { NavBarTab } from "./nav-bar-tab";
import preLoginMenu from "./pre-login-menu.json";
import postLoginMenu from "./post-login-menu.json";

function Auth0NavBarTabs() {
  const { isAuthenticated } = useAuth0();

  const menus = isAuthenticated ? postLoginMenu : preLoginMenu;

  return (
    <div className="nav-bar__tabs">
      {menus.map((item) => (
        <NavBarTab key={item.path} path={item.path} label={item.label} />
      ))}
    </div>
  );
}

export default Auth0NavBarTabs;
