import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import { ApolloProvider } from '@apollo/client'

import { App } from './application'
import { UiProvider } from '@/providers/ui'
import { apolloClient } from '@/providers/apollo'

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <ApolloProvider client={apolloClient}>
      <UiProvider>
        <App />
      </UiProvider>
    </ApolloProvider>
  </StrictMode>,
)
