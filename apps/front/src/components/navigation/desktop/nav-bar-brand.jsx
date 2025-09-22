import React from "react";
import { NavLink } from "react-router-dom";
import yourLogo from '../../../images/icons/your-logo.svg';

export const NavBarBrand = () => {
  return (
    <div className="nav-bar__brand">
      <NavLink to="/">
        <img
          className="nav-bar__logo"
          src={yourLogo}
          alt="Your Logo"
          height="36"
        />
      </NavLink>
    </div>
  );
};
