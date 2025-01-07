import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import { ApolloProvider } from '@apollo/client'

import { App } from './application.js'
import { UiProvider } from '#providers/ui.js'
import { apolloClient } from '#providers/apollo.js'

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <ApolloProvider client={apolloClient}>
      <UiProvider>
        <App />
      </UiProvider>
    </ApolloProvider>
  </StrictMode>,
)
