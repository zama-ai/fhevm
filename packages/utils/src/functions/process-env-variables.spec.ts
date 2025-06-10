import { afterEach, describe, expect, test, vi } from 'vitest'
import { processEnvVariables } from './process-env-variables.js'

describe('processEnvVariables', () => {
  afterEach(() => {
    vi.unstubAllEnvs()
  })

  test('should process env variables', () => {
    vi.stubEnv('APP__FIRST__SECOND', '123')
    expect(processEnvVariables()).toEqual({ first: { second: '123' } })
  })

  test('should handle empty env variables', () => {
    vi.stubEnv('APP__', 'foo')
    expect(processEnvVariables()).toEqual({})
  })

  test('should handle undefined env variables', () => {
    expect(processEnvVariables()).toEqual({})
  })

  test('should handle array env variables with primitive values', () => {
    vi.stubEnv('APP__ARR__0', '123')
    vi.stubEnv('APP__ARR__1', '456')
    vi.stubEnv('APP__ARR__2', 'John')
    expect(processEnvVariables()).toEqual({
      arr: ['123', '456', 'John'],
    })
  })

  test('should handle array env variables with object values', () => {
    vi.stubEnv('APP__ARR__0__ID', '123')
    vi.stubEnv('APP__ARR__0__NAME', 'Lucy')
    vi.stubEnv('APP__ARR__1__ID', '456')
    vi.stubEnv('APP__ARR__1__NAME', 'John')
    expect(processEnvVariables()).toEqual({
      arr: [
        { id: '123', name: 'Lucy' },
        { id: '456', name: 'John' },
      ],
    })
  })
})
