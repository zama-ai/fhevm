import { CheckEnv } from './lib/check-env.tsx'
import { Router } from './router.tsx'
import { TitleContextWrapper } from './components/title-context/title-context-wrapper.tsx'

export function App() {
  return (
    <TitleContextWrapper>
      <CheckEnv />
      <Router />
    </TitleContextWrapper>
  )
}
