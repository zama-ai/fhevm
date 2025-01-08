import { CheckEnv } from './lib/check-env.js'
import { Router } from './router.js'
import { TitleContextWrapper } from './components/title-context/title-context-wrapper.js'

export function App() {
  return (
    <TitleContextWrapper>
      <CheckEnv />
      <Router />
    </TitleContextWrapper>
  )
}
