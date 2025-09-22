import { useAuth0 } from "@auth0/auth0-react";
import { LoginButton } from "../../buttons/login-button";
import { LogoutButton } from "../../buttons/logout-button";
import { SignupButton } from "../../buttons/signup-button";

export const MobileNavBarButtons = () => {
  const auth0Auth = useAuth0();
  const isAuthenticated = auth0Auth?.isAuthenticated;

  return (
    <div className="mobile-nav-bar__buttons">
      {isAuthenticated ? (
        <>
          <LogoutButton />
        </>
      ) : (
        <>
          <SignupButton />
          <LoginButton />
        </>
      )}
    </div>
  );
};
