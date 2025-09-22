import React from "react";
import OktaLogin from "./OktaLogin";
import Auth0Login from "./Auth0Login";

export default function Login() {
  if (import.meta.env.REACT_APP_AUTH_PROVIDER === "Okta") {
    return <OktaLogin />;
  } else if (import.meta.env.REACT_APP_AUTH_PROVIDER === "Auth0") {
    return <Auth0Login />;
  }
}
