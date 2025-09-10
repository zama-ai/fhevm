import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'

import { App } from './application.js'
import { UiProvider } from './providers/ui.js'

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <UiProvider>
      <App />
    </UiProvider>
  </StrictMode>,
)
