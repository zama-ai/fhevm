import React from "react";
import { NavBarBrand } from "./nav-bar-brand";
import { NavBarButtons } from "./nav-bar-buttons";
import { NavBarTabs } from "./nav-bar-tabs";

export const NavBar = ({ hideNavTabs }) => {
  return (
    <div className="nav-bar__container">
      <nav className="nav-bar">
        <NavBarBrand />
        <div className="nav-bar__actions">
          <NavBarTabs />
          <NavBarButtons />
        </div>
      </nav>
    </div>
  );
};
