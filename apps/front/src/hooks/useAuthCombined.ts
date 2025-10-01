import { useState, useEffect } from "react";
import { useAuth0 } from "@auth0/auth0-react";
import { moesifIdentifyUserFrontEndIfPossible } from "../common/utils";

// purpose to consolidate the different hooks from auth provider.
// and have a consistent interface to get idToken, accessToken, and userObject

export default function useAuth() {
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

  const [idToken, setIdToken] = useState<string>();
  const [accessToken, setAccessToken] = useState<string>();

  const handleSignUp = async ({ returnTo }: { returnTo?: string }) => {
    await loginWithRedirect({
      appState: {
        returnTo: returnTo || "/product-select",
      },
      authorizationParams: {
        prompt: "login",
        screen_hint: "signup",
      },
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
          const idToken = result?.__raw;
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
