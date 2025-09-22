import { useAuth0 } from "@auth0/auth0-react";
import PropTypes from "prop-types";

export const SignupButton = ({ isLink, isPriceAction }) => {
  const { loginWithRedirect } = useAuth0();

  const handleSignUp = async () => {
    window.moesif?.track("clicked-sign-up", {
      provider: "Auth0",
    });

    await loginWithRedirect({
      appState: {
        returnTo: "/plans",
      },
      authorizationParams: {
        prompt: "login",
        screen_hint: "signup",
      },
      scope: "openid profile email offline_access",
    });
  };

  const className = isLink
    ? " button__link"
    : ` button__${isPriceAction ? "price-action" : "sign-up"}`;

  return (
    <button className={className} onClick={handleSignUp}>
      Sign Up
    </button>
  );
};

SignupButton.propTypes = {
  isLink: PropTypes.bool,
  isPriceAction: PropTypes.bool,
};
