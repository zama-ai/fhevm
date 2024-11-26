import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import { ApolloProvider } from '@apollo/client'

import { App } from './application'
import { Provider } from '@/components/ui/provider'
import { apolloClient } from './apolloClient'

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <ApolloProvider client={apolloClient}>
      <Provider>
        <App />
      </Provider>
    </ApolloProvider>
  </StrictMode>
)
