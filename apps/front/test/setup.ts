import { afterAll, afterEach, beforeAll, beforeEach } from 'vitest'
import { cleanup } from '@testing-library/react'
import '@testing-library/jest-dom/vitest'
import { setupServer } from 'msw/node'
import { handlers } from './handlers'
import { createMatchMedia } from './matchMedia'

// Declare global types
declare global {
  interface Window {
    env: {
      VITE_BACK_HTTP_URL: string
      VITE_BACK_WS_URL: string
    }
  }
}

window.env = {
  VITE_BACK_HTTP_URL: 'http://mocked',
  VITE_BACK_WS_URL: 'ws://mocked',
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
