import { useOktaAuth } from "@okta/okta-react";
import { useLocation } from "react-router-dom";
import { toRelativeUrl } from "@okta/okta-auth-js";
import OktaError from "./OktaError";
import React, { useEffect, useRef, useState } from "react";

const SecureRoute = ({ children, onAuthRequired, errorComponent }) => {
  const { oktaAuth, authState, _onAuthRequired } = useOktaAuth();
  const location = useLocation();
  const pendingLogin = useRef(false);
  const [handleLoginError, setHandleLoginError] = useState(null);
  const ErrorReporter = errorComponent || OktaError;

  useEffect(() => {
    const handleLogin = async () => {
      if (pendingLogin.current) {
        return;
      }

      pendingLogin.current = true;

      const originalUri = toRelativeUrl(
        window.location.href,
        window.location.origin
      );
      oktaAuth.setOriginalUri(originalUri);
      const onAuthRequiredFn = onAuthRequired || _onAuthRequired;
      if (onAuthRequiredFn) {
        await onAuthRequiredFn(oktaAuth);
      } else {
        await oktaAuth.signInWithRedirect();
      }
    };

    if (!location) {
      return;
    }

    if (!authState) {
      return;
    }

    if (authState?.isAuthenticated) {
      pendingLogin.current = false;
      return;
    }

    if (!authState?.isAuthenticated) {
      handleLogin().catch((err) => {
        setHandleLoginError(err);
      });
    }
  }, [authState, oktaAuth, location, onAuthRequired, _onAuthRequired]);

  if (handleLoginError) {
    return <ErrorReporter error={handleLoginError} />;
  }

  if (!authState || !authState?.isAuthenticated) {
    return null;
  }

  return children;
};

export default SecureRoute;
