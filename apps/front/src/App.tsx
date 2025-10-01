import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { BrowserRouter } from "react-router-dom";

import { Auth0ProviderWithNavigate } from "./Auth0ProviderWithNavigate";
import { PageFooter } from "./components/page-footer";
import { AppRoutes } from "./app-routes";

const queryClient = new QueryClient()

function App() {
  return (
    <div>
      <div>
        <BrowserRouter>
          <QueryClientProvider client={queryClient}>
            <Auth0ProviderWithNavigate>
              <AppRoutes />
            </Auth0ProviderWithNavigate>
          </QueryClientProvider>
        </BrowserRouter>
      </div>
      <PageFooter />
    </div>
  );
}

export default App;
