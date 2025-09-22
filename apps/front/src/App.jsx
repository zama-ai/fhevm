import { BrowserRouter, Routes, Route, Navigate } from "react-router-dom";
import { LoginCallback } from "@okta/okta-react";
import { useAuth0 } from "@auth0/auth0-react";

import Dashboard from "./components/pages/dashboard/Dashboard";
import Settings from "./components/pages/settings/Settings";
import { OktaProviderWithNavigate } from "./OktaProviderWithNavigate";
import { Auth0ProviderWithNavigate } from "./Auth0ProviderWithNavigate";
import Keys from "./components/pages/keys/Keys";
import SecureRoute from "./components/okta/SecureRoute";
import { AuthenticationGuard } from "./components/authentication-guard";
import SignUp from "./components/pages/signup/SignUp";
import RedirectToSignIn from "./components/pages/signup/OktaPostCreate";
import Return from "./components/pages/checkout/Return";
import Setup from "./components/pages/setup/Setup";
import Plans from "./components/pages/plans/Plans";
import Home from "./components/pages/home/Home";
import Checkout from "./components/pages/checkout/Checkout";
import Subscription from "./components/pages/subscription/Subscription";
import { PageFooter } from "./components/page-footer";

function App() {
  const { isAuthenticated } = useAuth0();

  if (import.meta.env.REACT_APP_AUTH_PROVIDER === "Okta") {
    return (
      <div>
        <div>
          <BrowserRouter>
            <OktaProviderWithNavigate>
              <Routes>
                <Route path="/" element={<Home />} />
                <Route path="/signup" element={<SignUp />} />
                <Route path="login/callback" element={<LoginCallback />} />
                <Route path="/return" element={<Return />} />
                <Route path="/setup" element={<Setup />} />
                <Route path="/plans" element={<Plans />} />
                <Route
                  path="login/oktapostcreate"
                  element={<RedirectToSignIn />}
                />
                <Route
                  path="/checkout"
                  element={
                    <SecureRoute>
                      <Checkout />
                    </SecureRoute>
                  }
                />
                <Route
                  path="/return"
                  element={
                    <SecureRoute>
                      <Return />
                    </SecureRoute>
                  }
                />
                <Route
                  path="dashboard"
                  element={
                    <SecureRoute>
                      <Dashboard />
                    </SecureRoute>
                  }
                />
                <Route
                  path="settings"
                  element={
                    <SecureRoute>
                      <Settings />
                    </SecureRoute>
                  }
                />
                <Route
                  path="keys"
                  element={
                    <SecureRoute>
                      <Keys />
                    </SecureRoute>
                  }
                />
                <Route
                  path="subscriptions"
                  element={
                    <SecureRoute>
                      <Subscription />
                    </SecureRoute>
                  }
                />
              </Routes>
            </OktaProviderWithNavigate>
          </BrowserRouter>
        </div>
        <PageFooter />
      </div>
    );
  } else if (import.meta.env.REACT_APP_AUTH_PROVIDER === "Auth0") {
    return (
      <div>
        <div>
          <BrowserRouter>
            <Auth0ProviderWithNavigate>
              <Routes>
                <Route
                  path="/"
                  element={
                    !isAuthenticated ? (
                      <Home />
                    ) : (
                      <Navigate replace to={"dashboard"} />
                    )
                  }
                />
                <Route path="/return" element={<Return />} />
                <Route path="/setup" element={<Setup />} />
                <Route path="/plans" element={<Plans />} />
                <Route
                  path="/checkout"
                  element={<AuthenticationGuard component={Checkout} />}
                />
                <Route
                  path="/return"
                  element={<AuthenticationGuard component={Return} />}
                />
                <Route
                  path="dashboard"
                  element={<AuthenticationGuard component={Dashboard} />}
                />
                <Route
                  path="settings"
                  element={<AuthenticationGuard component={Settings} />}
                />
                <Route
                  path="keys"
                  element={<AuthenticationGuard component={Keys} />}
                />
                <Route
                  path="subscription"
                  element={<AuthenticationGuard component={Subscription} />}
                />
              </Routes>
            </Auth0ProviderWithNavigate>
          </BrowserRouter>
        </div>
        <PageFooter />
      </div>
    );
  } else {
    return (
      <div className="App">
        Please check your env files, the REACT_APP_AUTH_PROVIDER variable must
        be provided.
      </div>
    );
  }
}

export default App;
