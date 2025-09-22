import React, { useEffect } from "react";
import { useOktaAuth } from "@okta/okta-react";

const RedirectToSignIn = () => {
  const { oktaAuth } = useOktaAuth();

  useEffect(() => {
    const redirect = async () => {
      await oktaAuth.signInWithRedirect({ originalUri: "/dashboard" });
    };

    redirect();
  }, [oktaAuth]);

  return <div>Redirecting to sign-in...</div>;
};

export default RedirectToSignIn;
