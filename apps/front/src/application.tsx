import { CheckEnv } from './lib/check-env.tsx.js'
import { Router } from './router.tsx.js'
import { TitleContextWrapper } from './components/title-context/title-context-wrapper.tsx.js'

export function App() {
  return (
    <TitleContextWrapper>
      <CheckEnv />
      <Router />
    </TitleContextWrapper>
  )
}
