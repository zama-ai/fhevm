import { NavBarTab } from "./nav-bar-tab";
import preLoginMenu from "./pre-login-menu.json";
import postLoginMenu from "./post-login-menu.json";
import useAuth from "../../../hooks/useAuth";
import { useMemo } from "react";

export function NavBarTabs() {
  const { isAuthenticated } = useAuth();

  const menus = useMemo(
    () => (isAuthenticated ? postLoginMenu : preLoginMenu),
    [isAuthenticated]
  );

  return (
    <div className="nav-bar__tabs">
      {menus.map((item) => (
        <NavBarTab key={item.path} path={item.path} label={item.label} />
      ))}
    </div>
  );
}
