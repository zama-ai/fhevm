import { useAuth0 } from "@auth0/auth0-react";

export const LogoutButton = () => {
  const { logout } = useAuth0();

  const handleLogout = async () => {
    window.moesif?.track("clicked-logout", {
      provider: "Auth0",
    });
    logout({
      logoutParams: {
        returnTo: window.location.origin,
      },
    });
    window.moesif?.reset();
  };

  return (
    <button className="button__logout" onClick={handleLogout}>
      Log Out
    </button>
  );
};
