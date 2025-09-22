import { BrowserRouter, Routes, Route, Navigate } from "react-router-dom";
import { useAuth0 } from "@auth0/auth0-react";

import Dashboard from "./components/pages/dashboard/Dashboard";
import Settings from "./components/pages/settings/Settings";
import { Auth0ProviderWithNavigate } from "./Auth0ProviderWithNavigate";
import Keys from "./components/pages/keys/Keys";
import { AuthenticationGuard } from "./components/authentication-guard";
import Return from "./components/pages/checkout/Return";
import Setup from "./components/pages/setup/Setup";
import Plans from "./components/pages/plans/Plans";
import Home from "./components/pages/home/Home";
import Checkout from "./components/pages/checkout/Checkout";
import { PageFooter } from "./components/page-footer";

function App() {
  const { isAuthenticated } = useAuth0();

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
            </Routes>
          </Auth0ProviderWithNavigate>
        </BrowserRouter>
      </div>
      <PageFooter />
    </div>
  );
}

export default App;
