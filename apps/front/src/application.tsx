import { CheckEnv } from './lib/check-env.tsx'
import { Router } from './router.tsx'

export function App() {
  return (
    <>
      <CheckEnv />
      <Router />
    </>
  )
}
