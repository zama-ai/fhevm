import React from "react";
import { NavLink } from "react-router-dom";

export const MobileNavBarTab = ({ path, label, handleClick }) => {
  return (
    <NavLink
      onClick={handleClick}
      to={path}
      end
      className={({ isActive }) =>
        "mobile-nav-bar__tab " + (isActive ? "mobile-nav-bar__tab--active" : "")
      }
    >
      {label}
    </NavLink>
  );
};
