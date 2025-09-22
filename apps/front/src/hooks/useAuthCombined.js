import { useState, useEffect } from "react";
import { useAuth0 } from "@auth0/auth0-react";
import { useOktaAuth } from "@okta/okta-react";
import { useNavigate } from "react-router-dom";
import { moesifIdentifyUserFrontEndIfPossible } from "../common/utils";

// purpose to consolidate the different hooks from auth provider.
// and have a consistent interface to get idToken, accessToken, and userObject

function useAuthOktaVersion() {
  const navigate = useNavigate();
  const { authState } = useOktaAuth();

  const isAuthenticated = authState?.isAuthenticated;

  let isLoading = !authState || authState?.isPending;
  let user = authState?.idToken?.claims;

  const userEmail = user?.email || authState?.accessToken?.claims?.sub;
  const idToken = authState?.idToken;
  useEffect(() => {
    if (isAuthenticated && idToken) {
      moesifIdentifyUserFrontEndIfPossible(idToken);
    }
  }, [isAuthenticated, idToken]);

  const handleSignUp = async ({ returnTo }) => {
    navigate(`/signup?return_to=${encodeURIComponent(returnTo)}`);
  };

  return {
    isAuthenticated,
    isLoading,
    user,
    idToken: authState?.idToken,
    accessToken: authState?.accessToken,
    oktaAuthState: authState,
    userEmail,
    handleSignUp,
  };
}

function useAuthAuth0Version() {
  const {
    user: auth0User,
    isLoading: auth0IsLoading,
    isAuthenticated,
    loginWithRedirect,
    getAccessTokenSilently,
    getIdTokenClaims,
    ...rest
  } = useAuth0();

  let isLoading = auth0IsLoading;
  let user = auth0User;

  const [idToken, setIdToken] = useState();
  const [accessToken, setAccessToken] = useState();

  const handleSignUp = async ({ returnTo }) => {
    await loginWithRedirect({
      appState: {
        returnTo: returnTo || "/product-select",
      },
      authorizationParams: {
        prompt: "login",
        screen_hint: "signup",
      },
      scope: "openid profile email offline_access",
    });
  };

  useEffect(() => {
    if (isAuthenticated) {
      getAccessTokenSilently()
        .then((result) => {
          setAccessToken(result);
        })
        .catch((err) => {
          console.error("failed to load access token", err);
        });

      getIdTokenClaims()
        .then((result) => {
          const idToken = result.__raw;
          // https://github.com/auth0/auth0-react/issues/262
          setIdToken(idToken);
          moesifIdentifyUserFrontEndIfPossible(idToken);
        })
        .catch((err) => {
          console.error("failed to load id token", err);
        });
    }
  }, [isAuthenticated, getAccessTokenSilently, getIdTokenClaims]);

  return {
    isAuthenticated,
    user,
    isLoading,
    handleSignUp,
    idToken,
    accessToken,
    ...rest,
  };
}

const useAuthCombined =
  import.meta.env.REACT_APP_AUTH_PROVIDER === "Okta"
    ? useAuthOktaVersion
    : useAuthAuth0Version;

export default useAuthCombined;
