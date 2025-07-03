import { afterAll, afterEach, beforeAll, beforeEach } from 'vitest'
import { cleanup } from '@testing-library/react'
import '@testing-library/jest-dom/vitest'
import { setupServer } from 'msw/node'
import { handlers } from './handlers'
import { createMatchMedia } from './matchMedia'
import { EnvVariable } from '../src/lib/check-env'

// Declare global types
declare global {
  interface Window {
    env: Record<EnvVariable, string> & {
      [K in `VITE_FLAG_${string}`]: boolean
    }
  }
}

window.env = {
  VITE_BACK_HTTP_URL: 'http://mocked',
  VITE_BACK_WS_URL: 'ws://mocked',
  VITE_FLAG_INVITATIONS: true,
}

export const server = setupServer(...handlers)

beforeAll(() => {
  server.listen()
})

beforeEach(() => {
  // Fix: TypeError: window.matchMedia is not a function
  createMatchMedia(1024)
})

afterEach(() => {
  server.resetHandlers()
  // runs a clean after each test case (e.g. clearing jsdom)
  cleanup()
})
afterAll(() => server.close())
