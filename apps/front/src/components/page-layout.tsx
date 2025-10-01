import type { ReactNode } from "react";

import { NavBar } from "./navigation/desktop/nav-bar";
import { MobileNavBar } from "./navigation/mobile/mobile-nav-bar";

export const PageLayout = ({
  children,
  isHome,
}: {
  children: ReactNode;
  isHome?: boolean;
}) => {
  return (
    <div className={`page-layout ${isHome ? "page-layout--home" : ""}`}>
      <NavBar />
      <MobileNavBar />
      <div className="page-layout__content">{children}</div>
    </div>
  );
};
