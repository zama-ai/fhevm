import { CheckEnv } from './lib/check-env'
import { Router } from './router.tsx'

export function App() {
  return (
    <>
      <CheckEnv />
      <Router />
    </>
  )
}
