import React from "react";
import { NavLink } from "react-router-dom";
import yourLogo from '../../../images/icons/your-logo.svg';

export const MobileNavBarBrand = ({ handleClick }) => {
  return (
    <div onClick={handleClick} className="mobile-nav-bar__brand">
      <NavLink to="/">
        <img
          className="mobile-nav-bar__logo"
          src={yourLogo}
          alt="Your logo"
          height="24"
        />
      </NavLink>
    </div>
  );
};
