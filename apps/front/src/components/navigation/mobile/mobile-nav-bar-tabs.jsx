import PropTypes from "prop-types";
import { useAuth0 } from "@auth0/auth0-react";
import { MobileNavBarTab } from "./mobile-nav-bar-tab";

export function MobileNavBarTabs({ handleClick }) {
  const auth0Auth = useAuth0();
  const isAuthenticated = auth0Auth?.isAuthenticated;

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
}

MobileNavBarTabs.propTypes = {
  handleClick: PropTypes.func.isRequired,
};
